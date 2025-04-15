// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! The SuiExecutor module is responsible for executing process_checkpoint_message
//! on Sui blockchain on ika_system package.

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
use std::{collections::HashMap, sync::Arc};
use sui_json_rpc_types::SuiEvent;
use sui_macros::fail_point_async;
use sui_types::base_types::ObjectID;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{CallArg, ObjectArg, Transaction, TransactionKind};
use sui_types::BRIDGE_PACKAGE_ID;
use sui_types::{event::EventID, Identifier};
use tokio::{
    sync::Notify,
    task::JoinHandle,
    time::{self, Duration},
};
use tracing::{debug, error, info};

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

    pub async fn run_epoch_switch(
        &self,
        sui_notifier: &SuiNotifier,
        ika_system_state_inner: &SystemInner,
    ) {
        let Ok(clock) = self.sui_client.get_clock().await else {
            error!("Failed to get clock when running epoch switch");
            return;
        };
        let Some(dwallet_2pc_mpc_secp256k1_id) =
            ika_system_state_inner.dwallet_2pc_mpc_secp256k1_id()
        else {
            error!("Failed to get dwallet_2pc_mpc_secp256k1_id when running epoch switch");
            return;
        };
        let SystemInner::V1(system_inner_v1) = &ika_system_state_inner else {
            error!("Failed to get system inner when running epoch switch");
            return;
        };

        if clock.timestamp_ms
            > ika_system_state_inner.epoch_start_timestamp_ms()
                + (ika_system_state_inner.epoch_duration_ms() / 2)
            && system_inner_v1
                .validators
                .next_epoch_active_committee
                .is_none()
        {
            info!("calling process mid epoch");
            if let Err(e) =
                Self::process_mid_epoch(self.ika_system_package_id, &sui_notifier, &self.sui_client)
                    .await
            {
                error!("Failed to process mid epoch: {:?}", e);
            } else {
                info!("Successfully processed mid epoch");
            }
        }

        let Ok(DWalletCoordinatorInner::V1(coordinator)) = self
            .sui_client
            .get_dwallet_coordinator_inner(dwallet_2pc_mpc_secp256k1_id)
            .await
        else {
            error!("Failed to get dwallet coordinator inner when running epoch switch");
            return;
        };

        if clock.timestamp_ms
            > ika_system_state_inner.epoch_start_timestamp_ms()
                + ika_system_state_inner.epoch_duration_ms()
            && !coordinator.locked_last_session_to_complete_in_current_epoch
        {
            info!("calling lock last active session sequence number");
            if let Err(e) = Self::lock_last_active_session_sequence_number(
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

        if coordinator.locked_last_session_to_complete_in_current_epoch
            && coordinator.number_of_completed_sessions
                == coordinator.last_session_to_complete_in_current_epoch
            && system_inner_v1
                .validators
                .next_epoch_active_committee
                .is_some()
        {
            info!("calling process request advance epoch");
            if let Err(e) = Self::process_request_advance_epoch(
                self.ika_system_package_id,
                dwallet_2pc_mpc_secp256k1_id,
                &sui_notifier,
                &self.sui_client,
            )
            .await
            {
                error!("Failed to process request advance epoch: {:?}", e);
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
        tracing::info!(
            "Starting sui connector SuiExecutor run_epoch for epoch {}",
            epoch
        );
        // check if we want to run this epoch based on RunWithRange condition value
        // we want to be inclusive of the defined RunWithRangeEpoch::Epoch
        // i.e Epoch(N) means we will execute epoch N and stop when reaching N+1
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
                debug!(epoch, "finished epoch");
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
                        let signers_bitmap = Self::calculate_signers_bitmap(auth_sig);
                        let message =
                            bcs::to_bytes::<CheckpointMessage>(&checkpoint_message.into_message())
                                .expect("Serializing checkpoint message cannot fail");

                        info!("signers_bitmap: {:?}", signers_bitmap);

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

    fn calculate_signers_bitmap(auth_sig: &AuthorityStrongQuorumSignInfo) -> Vec<u8> {
        let mut signers_bitmap = vec![0u8; auth_sig.signers_map.len().div_ceil(8) as usize];
        for i in auth_sig.signers_map.iter() {
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
        // max_checkpoint_size_bytes is 50KB, so we split the message into 4 slices
        for i in 0..4 {
            // If the chunk is missing, use an empty slice, as the transaction must receive all arguments.
            let message = messages.get(i).unwrap_or(&empty).clone();
            slices.push(CallArg::Pure(bcs::to_bytes(message).unwrap()));
        }
        slices
    }

    async fn process_mid_epoch(
        ika_system_package_id: ObjectID,
        sui_notifier: &SuiNotifier,
        sui_client: &Arc<SuiClient<C>>,
    ) -> IkaResult<()> {
        info!("process_mid_epoch");
        let (gas_coin, gas_obj_ref, owner) = sui_client
            .get_gas_data_panic_if_not_gas(sui_notifier.gas_object_ref.0)
            .await;

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
                "Can't ProgrammableTransactionBuilder::move_call: {e}"
            ))
        })?;

        let transaction = super::build_sui_transaction(
            sui_notifier.sui_address,
            ptb.finish(),
            sui_client,
            vec![gas_obj_ref],
            &sui_notifier.sui_key,
        )
        .await;

        sui_client
            .execute_transaction_block_with_effects(transaction)
            .await?;

        Ok(())
    }

    async fn lock_last_active_session_sequence_number(
        ika_system_package_id: ObjectID,
        dwallet_2pc_mpc_secp256k1_id: ObjectID,
        sui_notifier: &SuiNotifier,
        sui_client: &Arc<SuiClient<C>>,
    ) -> IkaResult<()> {
        info!("process_lock_last_active_session_sequence_number");
        let (gas_coin, gas_obj_ref, owner) = sui_client
            .get_gas_data_panic_if_not_gas(sui_notifier.gas_object_ref.0)
            .await;

        let mut ptb = ProgrammableTransactionBuilder::new();

        let ika_system_state_arg = sui_client.get_mutable_system_arg_must_succeed().await;
        let clock_arg = sui_client.get_clock_arg_must_succeed().await;

        let dwallet_2pc_mpc_secp256k1_arg = sui_client
            .get_mutable_dwallet_2pc_mpc_secp256k1_arg_must_succeed(dwallet_2pc_mpc_secp256k1_id)
            .await;

        let mut args = vec![
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
                "Can't ProgrammableTransactionBuilder::move_call: {e}"
            ))
        })?;

        let transaction = super::build_sui_transaction(
            sui_notifier.sui_address,
            ptb.finish(),
            sui_client,
            vec![gas_obj_ref],
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
        info!("process_request_advance_epoch");
        let (gas_coin, gas_obj_ref, owner) = sui_client
            .get_gas_data_panic_if_not_gas(sui_notifier.gas_object_ref.0)
            .await;

        let mut ptb = ProgrammableTransactionBuilder::new();

        let ika_system_state_arg = sui_client.get_mutable_system_arg_must_succeed().await;
        let clock_arg = sui_client.get_clock_arg_must_succeed().await;

        let dwallet_2pc_mpc_secp256k1_arg = sui_client
            .get_mutable_dwallet_2pc_mpc_secp256k1_arg_must_succeed(dwallet_2pc_mpc_secp256k1_id)
            .await;

        let mut args = vec![
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
                "Can't ProgrammableTransactionBuilder::move_call: {e}"
            ))
        })?;

        let transaction = super::build_sui_transaction(
            sui_notifier.sui_address,
            ptb.finish(),
            sui_client,
            vec![gas_obj_ref],
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
        metrics: &Arc<SuiConnectorMetrics>,
    ) -> IkaResult<()> {
        let (gas_coin, gas_obj_ref, owner) = sui_client
            .get_gas_data_panic_if_not_gas(sui_notifier.gas_object_ref.0)
            .await;

        let mut ptb = ProgrammableTransactionBuilder::new();

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
            vec![gas_obj_ref],
            &sui_notifier.sui_key,
        )
        .await;

        sui_client
            .execute_transaction_block_with_effects(transaction)
            .await?;

        Ok(())
    }
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     use crate::{sui_client::SuiClient, sui_mock_client::SuiMockClient};
//     use prometheus::Registry;
//     use sui_json_rpc_types::EventPage;
//     use sui_types::{digests::TransactionDigest, event::EventID, Identifier};
//     use tokio::time::timeout;
//
//     #[tokio::test]
//     async fn test_sui_syncer_basic() -> anyhow::Result<()> {
//         telemetry_subscribers::init_for_testing();
//         let registry = Registry::new();
//         mysten_metrics::init_metrics(&registry);
//         let metrics = Arc::new(SuiHandlerMetrics::new(&registry));
//         let mock = SuiMockClient::default();
//         let client = Arc::new(SuiClient::new_for_testing(mock.clone()));
//         let module_foo = Identifier::new("Foo").unwrap();
//         let module_bar = Identifier::new("Bar").unwrap();
//         let empty_events = EventPage::empty();
//         let cursor = EventID {
//             tx_digest: TransactionDigest::random(),
//             event_seq: 0,
//         };
//         add_event_response(&mock, module_foo.clone(), cursor, empty_events.clone());
//         add_event_response(&mock, module_bar.clone(), cursor, empty_events.clone());
//
//         let target_modules = HashMap::from_iter(vec![
//             (module_foo.clone(), Some(cursor)),
//             (module_bar.clone(), Some(cursor)),
//         ]);
//         let interval = Duration::from_millis(200);
//         let (_handles, mut events_rx) = SuiExecutor::new(client, target_modules, metrics.clone())
//             .run(interval)
//             .await
//             .unwrap();
//
//         // Initially there are no events
//         assert_no_more_events(interval, &mut events_rx).await;
//
//         mock.set_latest_checkpoint_sequence_number(999);
//         // Module Foo has new events
//         let mut event_1: SuiEvent = SuiEvent::random_for_testing();
//         let package_id = BRIDGE_PACKAGE_ID;
//         event_1.type_.address = package_id.into();
//         event_1.type_.module = module_foo.clone();
//         let module_foo_events_1: sui_json_rpc_types::Page<SuiEvent, EventID> = EventPage {
//             data: vec![event_1.clone(), event_1.clone()],
//             next_cursor: Some(event_1.id),
//             has_next_page: false,
//         };
//         add_event_response(&mock, module_foo.clone(), event_1.id, empty_events.clone());
//         add_event_response(
//             &mock,
//             module_foo.clone(),
//             cursor,
//             module_foo_events_1.clone(),
//         );
//
//         let (identifier, received_events) = events_rx.recv().await.unwrap();
//         assert_eq!(identifier, module_foo);
//         assert_eq!(received_events.len(), 2);
//         assert_eq!(received_events[0].id, event_1.id);
//         assert_eq!(received_events[1].id, event_1.id);
//         // No more
//         assert_no_more_events(interval, &mut events_rx).await;
//         assert_eq!(
//             metrics
//                 .last_synced_sui_checkpoints
//                 .get_metric_with_label_values(&["Foo"])
//                 .unwrap()
//                 .get(),
//             999
//         );
//
//         // Module Bar has new events
//         let mut event_2: SuiEvent = SuiEvent::random_for_testing();
//         event_2.type_.address = package_id.into();
//         event_2.type_.module = module_bar.clone();
//         let module_bar_events_1 = EventPage {
//             data: vec![event_2.clone()],
//             next_cursor: Some(event_2.id),
//             has_next_page: true, // Set to true so that the syncer will not update the last synced checkpoint
//         };
//         add_event_response(&mock, module_bar.clone(), event_2.id, empty_events.clone());
//
//         add_event_response(&mock, module_bar.clone(), cursor, module_bar_events_1);
//
//         let (identifier, received_events) = events_rx.recv().await.unwrap();
//         assert_eq!(identifier, module_bar);
//         assert_eq!(received_events.len(), 1);
//         assert_eq!(received_events[0].id, event_2.id);
//         // No more
//         assert_no_more_events(interval, &mut events_rx).await;
//         assert_eq!(
//             metrics
//                 .last_synced_sui_checkpoints
//                 .get_metric_with_label_values(&["Bar"])
//                 .unwrap()
//                 .get(),
//             0, // Not updated
//         );
//
//         Ok(())
//     }
//
//     async fn assert_no_more_events(
//         interval: Duration,
//         events_rx: &mut mysten_metrics::metered_channel::Receiver<(Identifier, Vec<SuiEvent>)>,
//     ) {
//         match timeout(interval * 2, events_rx.recv()).await {
//             Err(_e) => (),
//             other => panic!("Should have timed out, but got: {:?}", other),
//         };
//     }
//
//     fn add_event_response(
//         mock: &SuiMockClient,
//         module: Identifier,
//         cursor: EventID,
//         events: EventPage,
//     ) {
//         mock.add_event_response(BRIDGE_PACKAGE_ID, module.clone(), cursor, events.clone());
//     }
// }
