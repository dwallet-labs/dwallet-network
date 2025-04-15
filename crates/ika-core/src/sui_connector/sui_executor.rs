// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! The SuiExecutor module handles executing process_checkpoint_message
//! on Sui blockchain on `ika_system` package.

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
    SystemInner, SystemInnerTrait, PROCESS_CHECKPOINT_MESSAGE_BY_QUORUM_FUNCTION_NAME,
    SYSTEM_MODULE_NAME,
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
        for i in signers_map.iter() {
            // Set the i-th bit to 1,
            signers_bitmap[(i / 8) as usize] |= 1u8 << (i % 8);
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
        let gas_coins = sui_client.get_gas_objects(sui_notifier.sui_address).await;
        let mut ptb = ProgrammableTransactionBuilder::new();

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
            .ok_or_else(|| IkaError::SuiConnectorInternalError("No gas coin found".to_string()))?;

        let ika_system_state_arg = sui_client.get_mutable_system_arg_must_succeed().await;

        let dwallet_2pc_mpc_secp256k1_arg = sui_client
            .get_mutable_dwallet_2pc_mpc_secp256k1_arg_must_succeed(dwallet_2pc_mpc_secp256k1_id)
            .await;

        let messages = Self::break_down_checkpoint_message(message);
        let mut args = vec![
            CallArg::Object(ika_system_state_arg),
            CallArg::Object(dwallet_2pc_mpc_secp256k1_arg),
            CallArg::Pure(bcs::to_bytes(&signature).map_err(|e| {
                IkaError::SuiConnectorSerializationError(format!("Can't bcs::to_bytes: {e}"))
            })?),
            CallArg::Pure(bcs::to_bytes(&signers_bitmap).map_err(|e| {
                IkaError::SuiConnectorSerializationError(format!("Can't bcs::to_bytes: {e}"))
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
    }

    #[test]
    fn test_calculate_signers_bitmap_various_sizes() {
        let test_cases = vec![4, 8, 12, 50, 115, 200, 300];

        for &num_validators in &test_cases {
            let mut signers = RoaringBitmap::new();
            for i in 0..num_validators {
                signers.insert(i);
            }

            let bitmap = SuiExecutor::<SuiSdkClient>::calculate_signers_bitmap(&signers);

            // Ensure the bitmap is large enough.
            let expected_size = (num_validators / 8 + 1) as usize;
            assert!(
                bitmap.len() >= expected_size,
                "Bitmap too small for {} validators: got {}, expected at least {}",
                num_validators,
                bitmap.len(),
                expected_size
            );

            // Validate that all expected bits are set
            let indices: Vec<u32> = (0..num_validators).collect();
            assert_bitmap_has_indices(&bitmap, &indices);
        }
    }
}
