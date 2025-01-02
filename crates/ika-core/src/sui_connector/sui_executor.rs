// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! The SuiExecutor module is responsible for synchronizing Events emitted
//! on Sui blockchain from concerned modules of bridge package 0x9.

use ika_types::sui::{
    error::BridgeResult,
};
use ika_sui_client::{SuiClient, SuiClientInner, retry_with_max_elapsed_time};
use crate::sui_connector::metrics::SuiConnectorMetrics;
use mysten_metrics::spawn_logged_monitored_task;
use std::{collections::HashMap, sync::Arc};
use fastcrypto::traits::ToFromBytes;
use sui_json_rpc_types::SuiEvent;
use sui_macros::fail_point_async;
use sui_types::BRIDGE_PACKAGE_ID;
use sui_types::{event::EventID, Identifier};
use sui_types::base_types::ObjectID;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{CallArg, ObjectArg, Transaction, TransactionKind};
use tokio::{
    sync::Notify,
    task::JoinHandle,
    time::{self, Duration},
};
use tracing::{debug, error, info};
use ika_config::node::RunWithRange;
use ika_types::committee::EpochId;
use ika_types::governance::{MIN_VALIDATOR_JOINING_STAKE_NIKA, VALIDATOR_LOW_STAKE_GRACE_PERIOD, VALIDATOR_LOW_STAKE_THRESHOLD_NIKA, VALIDATOR_VERY_LOW_STAKE_THRESHOLD_NIKA};
use ika_types::messages_checkpoint::CheckpointMessage;
use crate::checkpoints::CheckpointStore;
use crate::sui_connector::SuiNotifier;
use ika_types::sui::ika_system_state::{IkaSystemState, IkaSystemStateTrait};

#[derive(PartialEq, Eq, Debug)]
pub enum StopReason {
    EpochComplete(IkaSystemState),
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
where C: SuiClientInner + 'static {
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
        tracing::info!("Starting sui connector SuiExecutor run_epoch for epoch {}", epoch);
        // check if we want to run this epoch based on RunWithRange condition value
        // we want to be inclusive of the defined RunWithRangeEpoch::Epoch
        // i.e Epoch(N) means we will execute epoch N and stop when reaching N+1
        if run_with_range.map_or(false, |rwr| rwr.is_epoch_gt(epoch)) {
            info!(
                "RunWithRange condition satisfied at {:?}, run_epoch={:?}",
                run_with_range,
                epoch
            );
            return StopReason::RunWithRangeCondition;
        };


        let mut interval = time::interval(Duration::from_millis(120));

        loop {
            interval.tick().await;
            let ika_system_state = self.sui_client.get_ika_system_state_until_success().await;
            let epoch_on_sui: u64 = ika_system_state.epoch();
            if epoch_on_sui > epoch {
                fail_point_async!("crash");
                debug!(epoch, "finished epoch");
                return StopReason::EpochComplete(ika_system_state);
            }
            if epoch_on_sui < epoch {
                error!("epoch_on_sui cannot be less than epoch");
            }
            let last_processed_checkpoint_sequence_number: Option<u64> = ika_system_state.last_processed_checkpoint_sequence_number();
            let next_checkpoint_sequence_number = last_processed_checkpoint_sequence_number.map(|s| s + 1).unwrap_or(0);

            if let Some(sui_notifier) = self.sui_notifier.as_ref() {
                if let Ok(Some(checkpoint_message)) = self.checkpoint_store.get_checkpoint_by_sequence_number(next_checkpoint_sequence_number) {
                    
                    let auth_sig = checkpoint_message.auth_sig();
                    let signature = auth_sig.signature.as_bytes().to_vec();
                    let signers = auth_sig
                        .signers_map
                        .iter()
                        .map(|s| s as u16)
                        .collect::<Vec<_>>();
                    let message =
                        bcs::to_bytes::<CheckpointMessage>(&checkpoint_message.into_message()).expect("Serializing checkpoint message cannot fail");
                    let message_test: CheckpointMessage =
                        bcs::from_bytes(&message).expect("Serializing checkpoint message cannot fail");
                    
                    let task = Self::handle_execution_task(self.ika_system_package_id, signature, signers, message, &sui_notifier, &self.sui_client, &self.metrics).await;
                    match task {
                        Ok(_) => {
                            tracing::info!("Sui transaction successfully built");
                        }
                        Err(err) => {
                            tracing::error!("Sui transaction build failed: {}", err);
                        }
                    };
                }
            }
        }
    }

    async fn handle_execution_task(
        ika_system_package_id: ObjectID,
        signature: Vec<u8>,
        signers: Vec<u16>,
        message: Vec<u8>,
        sui_notifier: &SuiNotifier,
        sui_client: &Arc<SuiClient<C>>,
        metrics: &Arc<SuiConnectorMetrics>,
    ) -> anyhow::Result<()> {
        let (gas_coin, gas_obj_ref, owner) = sui_client
            .get_gas_data_panic_if_not_gas(sui_notifier.gas_object_ref.0)
            .await;
        
        let mut ptb = ProgrammableTransactionBuilder::new();

        let ika_system_state_arg = sui_client.get_mutable_ika_system_state_arg_must_succeed().await;
        
        ptb.move_call(
            ika_system_package_id,
            Identifier::new("ika_system")?,
            Identifier::new("process_checkpoint_message")?,
            vec![],
            vec![
                CallArg::Object(ika_system_state_arg),
                CallArg::Pure(bcs::to_bytes(&signature)?),
                CallArg::Pure(bcs::to_bytes(&signers)?),
                CallArg::Pure(bcs::to_bytes(&message)?),
            ]
        )?;
        
        let transaction = super::build_sui_transaction(sui_notifier.sui_address, ptb.finish(), sui_client, vec![gas_obj_ref], &sui_notifier.sui_key).await?;

        sui_client.execute_transaction_block_with_effects(transaction).await.map_err(|e| anyhow::format_err!("{:#?}", e))?;
        
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
