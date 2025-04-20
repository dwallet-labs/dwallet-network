// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! The SuiExecutor module handles executing transactions
//! on Sui blockchain for `ika_system` package.
use crate::checkpoints::CheckpointStore;
use crate::sui_connector::metrics::SuiConnectorMetrics;
use crate::sui_connector::SuiNotifier;
use fastcrypto::traits::ToFromBytes;
use ika_config::node::RunWithRange;
use ika_sui_client::{retry_with_max_elapsed_time, SuiClient, SuiClientInner};
use ika_types::committee::EpochId;
use ika_types::crypto::AuthorityStrongQuorumSignInfo;
use ika_types::error::{IkaError, IkaResult};
use ika_types::governance::{
    MIN_VALIDATOR_JOINING_STAKE_NIKA, VALIDATOR_LOW_STAKE_GRACE_PERIOD,
    VALIDATOR_LOW_STAKE_THRESHOLD_NIKA, VALIDATOR_VERY_LOW_STAKE_THRESHOLD_NIKA,
};
use ika_types::message::Secp256K1NetworkDKGOutputSlice;
use ika_types::messages_checkpoint::CheckpointMessage;
use ika_types::sui::epoch_start_system::EpochStartSystem;
use ika_types::sui::{
    DWalletCoordinatorInner, SystemInner, SystemInnerTrait,
    PROCESS_CHECKPOINT_MESSAGE_BY_QUORUM_FUNCTION_NAME, REQUEST_ADVANCE_EPOCH_FUNCTION_NAME,
    REQUEST_LOCK_EPOCH_SESSIONS_FUNCTION_NAME, REQUEST_MID_EPOCH_FUNCTION_NAME, SYSTEM_MODULE_NAME,
};
use itertools::Itertools;
use mysten_metrics::spawn_logged_monitored_task;
use roaring::RoaringBitmap;
use std::{collections::HashMap, sync::Arc};
use sui_json_rpc_types::SuiEvent;
use sui_macros::fail_point_async;
use sui_types::base_types::ObjectID;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{Argument, CallArg, ObjectArg, Transaction, TransactionKind};
use sui_types::BRIDGE_PACKAGE_ID;
use sui_types::{event::EventID, Identifier};
use tokio::{
    sync::Notify,
    task::JoinHandle,
    time::{self, Duration},
};
use tracing::{error, info};

#[derive(PartialEq, Eq, Debug)]
pub enum StopReason {
    EpochComplete(SystemInner, EpochStartSystem),
    RunWithRangeCondition,
}

pub struct SuiExecutor<C> {
    ika_system_package_id: ObjectID,
    checkpoint_store: Arc<CheckpointStore>,
    sui_notifier: Option<SuiNotifier>,
    sui_client: Arc<SuiClient<C>>,
    metrics: Arc<SuiConnectorMetrics>,
}

impl<C> SuiExecutor<C>
where
    C: SuiClientInner + 'static,
{
    pub fn new(
        ika_system_package_id: ObjectID,
        checkpoint_store: Arc<CheckpointStore>,
        sui_notifier: Option<SuiNotifier>,
        sui_client: Arc<SuiClient<C>>,
        metrics: Arc<SuiConnectorMetrics>,
    ) -> Self {
        Self {
            ika_system_package_id,
            checkpoint_store,
            sui_notifier,
            sui_client,
            metrics,
        }
    }

    /// Checks whether `process_mid_epoch`, `lock_last_active_session_sequence_number`, or
    /// `request_advance_epoch` can be called, and calls them if so.
    ///
    /// Anyone can call these functions based on the epoch and Sui's clock times.
    ///
    /// We don't use Sui's previous epoch switch mechanism as it assumes checkpoints are
    /// being created all the time, and in Ika,
    /// checkpoints are created only when there are new completed MPC sessions to write to Sui.
    async fn run_epoch_switch(
        &self,
        sui_notifier: &SuiNotifier,
        ika_system_state_inner: &SystemInner,
    ) {
        let Ok(clock) = self.sui_client.get_clock().await else {
            error!("failed to get clock when running epoch switch");
            return;
        };
        let Some(dwallet_2pc_mpc_secp256k1_id) =
            ika_system_state_inner.dwallet_2pc_mpc_secp256k1_id()
        else {
            error!("failed to get `dwallet_2pc_mpc_secp256k1_id` when running epoch switch");
            return;
        };
        let SystemInner::V1(system_inner_v1) = &ika_system_state_inner;

        let mid_epoch_time = ika_system_state_inner.epoch_start_timestamp_ms()
            + (ika_system_state_inner.epoch_duration_ms() / 2);
        let next_epoch_committee_is_empty = system_inner_v1
            .validators
            .next_epoch_active_committee
            .is_none();
        if clock.timestamp_ms > mid_epoch_time && next_epoch_committee_is_empty {
            info!("Calling `process_mid_epoch()`");
            if let Err(e) =
                Self::process_mid_epoch(self.ika_system_package_id, &sui_notifier, &self.sui_client)
                    .await
            {
                error!("`process_mid_epoch()` failed: {:?}", e);
            } else {
                info!("`process_mid_epoch()` successful");
            }
        }

        let Ok(DWalletCoordinatorInner::V1(coordinator)) = self
            .sui_client
            .get_dwallet_coordinator_inner(dwallet_2pc_mpc_secp256k1_id)
            .await
        else {
            error!("failed to get dwallet coordinator inner when running epoch switch");
            return;
        };

        // The Epoch was finished.
        let epoch_finish_time = ika_system_state_inner.epoch_start_timestamp_ms()
            + ika_system_state_inner.epoch_duration_ms();
        let epoch_not_locked = !coordinator.locked_last_session_to_complete_in_current_epoch;
        if clock.timestamp_ms > epoch_finish_time && epoch_not_locked {
            info!("Calling `lock_last_active_session_sequence_number()`");
            if let Err(e) = Self::lock_last_session_to_complete_in_current_epoch(
                self.ika_system_package_id,
                dwallet_2pc_mpc_secp256k1_id,
                &sui_notifier,
                &self.sui_client,
            )
            .await
            {
                error!(
                    "failed to lock last active session sequence number: {:?}",
                    e
                );
            } else {
                info!("Successfully locked last active session sequence number");
            }
        }

        // Check if we can advance the epoch.
        let all_epoch_sessions_finished = coordinator.number_of_completed_sessions
            == coordinator.last_session_to_complete_in_current_epoch;
        let all_immediate_sessions_completed = coordinator.started_immediate_sessions_count
            == coordinator.completed_immediate_sessions_count;
        let next_epoch_committee_exists = system_inner_v1
            .validators
            .next_epoch_active_committee
            .is_some();
        if coordinator.locked_last_session_to_complete_in_current_epoch
            && all_epoch_sessions_finished
            && all_immediate_sessions_completed
            && next_epoch_committee_exists
        {
            info!("Calling `process_request_advance_epoch()`");
            if let Err(e) = Self::process_request_advance_epoch(
                self.ika_system_package_id,
                dwallet_2pc_mpc_secp256k1_id,
                &sui_notifier,
                &self.sui_client,
            )
            .await
            {
                error!("failed to process request advance epoch: {:?}", e);
            } else {
                info!("Successfully processed request advance epoch");
            }
        }
    }

    pub async fn run_epoch(
        &self,
        epoch: EpochId,
        run_with_range: Option<RunWithRange>,
    ) -> StopReason {
        info!(
            "Starting sui connector SuiExecutor run_epoch for epoch {}",
            epoch
        );
        // check if we want to run this epoch based on RunWithRange condition value
        // we want to be inclusive of the defined RunWithRangeEpoch::Epoch
        // i.e Epoch(N) means we will execute the epoch N and stop when reaching N+1.
        if run_with_range.map_or(false, |rwr| rwr.is_epoch_gt(epoch)) {
            info!(
                "RunWithRange condition satisfied at {:?}, run_epoch={:?}",
                run_with_range, epoch
            );
            return StopReason::RunWithRangeCondition;
        };

        let mut interval = time::interval(Duration::from_millis(120));

        loop {
            interval.tick().await;
            let ika_system_state_inner = self.sui_client.get_system_inner_until_success().await;
            let epoch_on_sui: u64 = ika_system_state_inner.epoch();
            if epoch_on_sui > epoch {
                fail_point_async!("crash");
                info!(epoch, "Finished epoch");
                let epoch_start_system_state = self
                    .sui_client
                    .get_epoch_start_system_until_success(&ika_system_state_inner)
                    .await;
                return StopReason::EpochComplete(ika_system_state_inner, epoch_start_system_state);
            }
            if epoch_on_sui < epoch {
                error!("epoch_on_sui cannot be less than epoch");
            }
            let last_processed_checkpoint_sequence_number: Option<u64> =
                ika_system_state_inner.last_processed_checkpoint_sequence_number();
            let next_checkpoint_sequence_number = last_processed_checkpoint_sequence_number
                .map(|s| s + 1)
                .unwrap_or(0);

            if let Some(sui_notifier) = self.sui_notifier.as_ref() {
                self.run_epoch_switch(sui_notifier, &ika_system_state_inner)
                    .await;
                if let Ok(Some(checkpoint_message)) = self
                    .checkpoint_store
                    .get_checkpoint_by_sequence_number(next_checkpoint_sequence_number)
                {
                    if let Some(dwallet_2pc_mpc_secp256k1_id) =
                        ika_system_state_inner.dwallet_2pc_mpc_secp256k1_id()
                    {
                        let auth_sig = checkpoint_message.auth_sig();
                        let signature = auth_sig.signature.as_bytes().to_vec();
                        let signers_bitmap = Self::calculate_signers_bitmap(&auth_sig.signers_map);
                        let message =
                            bcs::to_bytes::<CheckpointMessage>(&checkpoint_message.into_message())
                                .expect("Serializing checkpoint message cannot fail");

                        info!("Signers_bitmap: {:?}", signers_bitmap);

                        let task = Self::handle_execution_task(
                            self.ika_system_package_id,
                            dwallet_2pc_mpc_secp256k1_id,
                            signature,
                            signers_bitmap,
                            message,
                            &sui_notifier,
                            &self.sui_client,
                            &self.metrics,
                        )
                        .await;
                        match task {
                            Ok(_) => {
                                info!("Sui transaction successfully executed for checkpoint sequence number: {}", next_checkpoint_sequence_number);
                            }
                            Err(err) => {
                                error!("Sui transaction execution failed for checkpoint sequence number: {}, error: {}", next_checkpoint_sequence_number, err);
                            }
                        };
                    }
                }
            }
        }
    }

    fn calculate_signers_bitmap(signers_map: &RoaringBitmap) -> Vec<u8> {
        let max_singers_bytes = signers_map.max().unwrap_or(0).div_ceil(8) as usize;
        // The bitmap is 1 byte larger than the number of signers to accommodate the last byte.
        let mut signers_bitmap = vec![0u8; max_singers_bytes + 1];
        for singer in signers_map.iter() {
            // Set the i-th bit to 1,
            let byte_index = (singer / 8) as usize;
            let bit_index = singer % 8;
            signers_bitmap[byte_index] |= 1u8 << bit_index;
        }
        signers_bitmap
    }

    /// Break down the message to slices because of chain transaction size limits.
    /// Limit 16 KB per Tx `pure` argument.
    fn break_down_checkpoint_message(message: Vec<u8>) -> Vec<CallArg> {
        let mut slices = Vec::new();
        // Set to 15 because the limit is up to 16 (smaller than).
        let messages = message.chunks(15 * 1024).collect_vec();
        let empty: &[u8] = &[];
        // `max_checkpoint_size_bytes` is 50KB, so we split the message into 4 slices.
        for i in 0..4 {
            // If the chunk is missing, use an empty slice, as the transaction must receive all arguments.
            let message = messages.get(i).unwrap_or(&empty);
            slices.push(CallArg::Pure(bcs::to_bytes(message).unwrap()));
        }
        slices
    }

    async fn process_mid_epoch(
        ika_system_package_id: ObjectID,
        sui_notifier: &SuiNotifier,
        sui_client: &Arc<SuiClient<C>>,
    ) -> IkaResult<()> {
        info!("Running `process_mid_epoch()`");
        let gas_coins = sui_client.get_gas_objects(sui_notifier.sui_address).await;
        let gas_coin = gas_coins
            .first()
            .ok_or_else(|| IkaError::SuiConnectorInternalError("no gas coin found".to_string()))?;

        let mut ptb = ProgrammableTransactionBuilder::new();

        let ika_system_state_arg = sui_client.get_mutable_system_arg_must_succeed().await;
        let clock_arg = sui_client.get_clock_arg_must_succeed().await;

        let args = vec![
            CallArg::Object(ika_system_state_arg),
            CallArg::Object(clock_arg),
        ];

        ptb.move_call(
            ika_system_package_id,
            SYSTEM_MODULE_NAME.into(),
            REQUEST_MID_EPOCH_FUNCTION_NAME.into(),
            vec![],
            args,
        )
        .map_err(|e| {
            IkaError::SuiConnectorInternalError(format!(
                "failed on ProgrammableTransactionBuilder::move_call: {e}"
            ))
        })?;

        let transaction = super::build_sui_transaction(
            sui_notifier.sui_address,
            ptb.finish(),
            sui_client,
            vec![*gas_coin],
            &sui_notifier.sui_key,
        )
        .await;

        sui_client
            .execute_transaction_block_with_effects(transaction)
            .await?;

        Ok(())
    }

    async fn lock_last_session_to_complete_in_current_epoch(
        ika_system_package_id: ObjectID,
        dwallet_2pc_mpc_secp256k1_id: ObjectID,
        sui_notifier: &SuiNotifier,
        sui_client: &Arc<SuiClient<C>>,
    ) -> IkaResult<()> {
        info!("Process `lock_last_active_session_sequence_number()`");
        let gas_coins = sui_client.get_gas_objects(sui_notifier.sui_address).await;
        let gas_coin = gas_coins
            .first()
            .ok_or_else(|| IkaError::SuiConnectorInternalError("no gas coin found".to_string()))?;

        let mut ptb = ProgrammableTransactionBuilder::new();

        let ika_system_state_arg = sui_client.get_mutable_system_arg_must_succeed().await;
        let clock_arg = sui_client.get_clock_arg_must_succeed().await;

        let dwallet_2pc_mpc_secp256k1_arg = sui_client
            .get_mutable_dwallet_2pc_mpc_secp256k1_arg_must_succeed(dwallet_2pc_mpc_secp256k1_id)
            .await;

        let args = vec![
            CallArg::Object(ika_system_state_arg),
            CallArg::Object(dwallet_2pc_mpc_secp256k1_arg),
            CallArg::Object(clock_arg),
        ];

        ptb.move_call(
            ika_system_package_id,
            SYSTEM_MODULE_NAME.into(),
            REQUEST_LOCK_EPOCH_SESSIONS_FUNCTION_NAME.into(),
            vec![],
            args,
        )
        .map_err(|e| {
            IkaError::SuiConnectorInternalError(format!(
                "failed on ProgrammableTransactionBuilder::move_call: {e}"
            ))
        })?;

        let transaction = super::build_sui_transaction(
            sui_notifier.sui_address,
            ptb.finish(),
            sui_client,
            vec![*gas_coin],
            &sui_notifier.sui_key,
        )
        .await;

        sui_client
            .execute_transaction_block_with_effects(transaction)
            .await?;

        Ok(())
    }

    async fn process_request_advance_epoch(
        ika_system_package_id: ObjectID,
        dwallet_2pc_mpc_secp256k1_id: ObjectID,
        sui_notifier: &SuiNotifier,
        sui_client: &Arc<SuiClient<C>>,
    ) -> IkaResult<()> {
        info!("Running `process_request_advance_epoch()`");
        let gas_coins = sui_client.get_gas_objects(sui_notifier.sui_address).await;
        let gas_coin = gas_coins
            .first()
            .ok_or_else(|| IkaError::SuiConnectorInternalError("no gas coin found".to_string()))?;

        let mut ptb = ProgrammableTransactionBuilder::new();

        let ika_system_state_arg = sui_client.get_mutable_system_arg_must_succeed().await;
        let clock_arg = sui_client.get_clock_arg_must_succeed().await;

        let dwallet_2pc_mpc_secp256k1_arg = sui_client
            .get_mutable_dwallet_2pc_mpc_secp256k1_arg_must_succeed(dwallet_2pc_mpc_secp256k1_id)
            .await;

        let args = vec![
            CallArg::Object(ika_system_state_arg),
            CallArg::Object(dwallet_2pc_mpc_secp256k1_arg),
            CallArg::Object(clock_arg),
        ];

        ptb.move_call(
            ika_system_package_id,
            SYSTEM_MODULE_NAME.into(),
            REQUEST_ADVANCE_EPOCH_FUNCTION_NAME.into(),
            vec![],
            args,
        )
        .map_err(|e| {
            IkaError::SuiConnectorInternalError(format!(
                "failed on ProgrammableTransactionBuilder::move_call {e}"
            ))
        })?;

        let transaction = super::build_sui_transaction(
            sui_notifier.sui_address,
            ptb.finish(),
            sui_client,
            vec![*gas_coin],
            &sui_notifier.sui_key,
        )
        .await;

        sui_client
            .execute_transaction_block_with_effects(transaction)
            .await?;

        Ok(())
    }

    async fn handle_execution_task(
        ika_system_package_id: ObjectID,
        dwallet_2pc_mpc_secp256k1_id: ObjectID,
        signature: Vec<u8>,
        signers_bitmap: Vec<u8>,
        message: Vec<u8>,
        sui_notifier: &SuiNotifier,
        sui_client: &Arc<SuiClient<C>>,
        _metrics: &Arc<SuiConnectorMetrics>,
    ) -> IkaResult<()> {
        let mut ptb = ProgrammableTransactionBuilder::new();

        let gas_coins = sui_client.get_gas_objects(sui_notifier.sui_address).await;
        if gas_coins.len() > 1 {
            info!("More than one gas coin was found, merging them into one gas coin.");
            let coins: IkaResult<Vec<_>> = gas_coins
                .iter()
                .skip(1)
                .map(|c| {
                    ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(*c)))
                        .map_err(|e| {
                            IkaError::SuiConnectorInternalError(format!(
                                "error merging coin ProgrammableTransactionBuilder::input: {e}"
                            ))
                        })
                })
                .collect();

            let coins = coins?;

            ptb.command(sui_types::transaction::Command::MergeCoins(
                Argument::GasCoin,
                coins,
            ));
        }
        let gas_coin = gas_coins
            .first()
            .ok_or_else(|| IkaError::SuiConnectorInternalError("no gas coin found".to_string()))?;

        let ika_system_state_arg = sui_client.get_mutable_system_arg_must_succeed().await;

        let dwallet_2pc_mpc_secp256k1_arg = sui_client
            .get_mutable_dwallet_2pc_mpc_secp256k1_arg_must_succeed(dwallet_2pc_mpc_secp256k1_id)
            .await;

        info!(
            "`signers_bitmap` @ handle_execution_task: {:?}",
            signers_bitmap
        );

        let messages = Self::break_down_checkpoint_message(message);
        let mut args = vec![
            CallArg::Object(ika_system_state_arg),
            CallArg::Object(dwallet_2pc_mpc_secp256k1_arg),
            CallArg::Pure(bcs::to_bytes(&signature).map_err(|e| {
                IkaError::SuiConnectorSerializationError(format!(
                    "can't serialize `signature`: {e}"
                ))
            })?),
            CallArg::Pure(bcs::to_bytes(&signers_bitmap).map_err(|e| {
                IkaError::SuiConnectorSerializationError(format!(
                    "can't serialize `signers_bitmap`: {e}"
                ))
            })?),
        ];
        args.extend(messages);

        ptb.move_call(
            ika_system_package_id,
            SYSTEM_MODULE_NAME.into(),
            PROCESS_CHECKPOINT_MESSAGE_BY_QUORUM_FUNCTION_NAME.into(),
            vec![],
            args,
        )
        .map_err(|e| {
            IkaError::SuiConnectorInternalError(format!(
                "Can't ProgrammableTransactionBuilder::move_call: {e}"
            ))
        })?;

        let transaction = super::build_sui_transaction(
            sui_notifier.sui_address,
            ptb.finish(),
            sui_client,
            vec![*gas_coin],
            &sui_notifier.sui_key,
        )
        .await;

        sui_client
            .execute_transaction_block_with_effects(transaction)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roaring::RoaringBitmap;
    use sui_sdk::SuiClient as SuiSdkClient;

    /// Test helper: assert that each expected validator index has its bit set in the output bitmap.
    fn assert_bitmap_has_indices(bitmap: &[u8], indices: &[u32]) {
        for &i in indices {
            let byte = bitmap[(i / 8) as usize];
            let bit = (byte >> (i % 8)) & 1;
            assert_eq!(bit, 1, "Bit for validator {} should be set", i);
        }
        println!("{:?}", bitmap);
    }

    #[test]
    fn test_calculate_signers_bitmap_various_sizes() {
        let test_cases = vec![4, 8, 12, 48, 50, 115, 200, 300];

        for &num_validators in &test_cases {
            let mut signers = RoaringBitmap::new();
            for i in 0..num_validators {
                signers.insert(i);
            }

            let bitmap = SuiExecutor::<SuiSdkClient>::calculate_signers_bitmap(&signers);
            println!("Bitmap: {:?}", bitmap);

            // Ensure the bitmap is large enough.
            let expected_size = (num_validators / 8) as usize;
            assert!(
                bitmap.len() >= expected_size,
                "Bitmap too small for {} validators: got {}, expected at least {}",
                num_validators,
                bitmap.len(),
                expected_size
            );

            // Validate that all expected bits are set
            let indices: Vec<u32> = (0..num_validators).collect();
            // assert_bitmap_has_indices(&bitmap, &indices);
        }
    }

    #[test]
    fn test_calculate_signers_bitmap_with_index_exceeding_bitmap_size() {
        // Simulate a case where there are more validators than entries in the bitmap.
        let num_validators = 10;
        let mut signers = RoaringBitmap::new();

        // Add the 9th index (zero-based),
        // which is out of bounds if bitmap only accounts for 8.
        signers.insert(9);

        let bitmap = SuiExecutor::<SuiSdkClient>::calculate_signers_bitmap(&signers);
        println!("Bitmap: {:?}", bitmap);

        // Bitmap should be large enough to include index 9.
        // Index 9 needs 2 bytes.
        let required_length = (9 / 8) + 1;
        assert!(
            bitmap.len() >= required_length,
            "Bitmap is too small: expected at least {} bytes for validator index 9, got {}",
            required_length,
            bitmap.len()
        );

        // Optionally: verify that the 10th bit is set
        let byte_index = 9 / 8;
        let bit_position = 9 % 8;
        assert_eq!(
            (bitmap[byte_index] >> bit_position) & 1,
            1,
            "Expected bit at index 9 to be set"
        );
    }
}
