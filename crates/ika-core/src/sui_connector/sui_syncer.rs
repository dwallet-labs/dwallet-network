// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! The SuiSyncer module is responsible for synchronizing Events emitted
//! on Sui blockchain from concerned modules of ika_system package.

use crate::authority::authority_perpetual_tables::AuthorityPerpetualTables;
use crate::dwallet_mpc::network_dkg::DwalletMPCNetworkKeys;
use crate::sui_connector::metrics::SuiConnectorMetrics;
use ika_sui_client::{retry_with_max_elapsed_time, SuiClient, SuiClientInner};
use ika_types::error::IkaResult;
use itertools::Itertools;
use mpc::WeightedThresholdAccessStructure;
use mysten_metrics::spawn_logged_monitored_task;
use std::{collections::HashMap, sync::Arc};
use sui_json_rpc_types::SuiEvent;
use sui_types::BRIDGE_PACKAGE_ID;
use sui_types::{event::EventID, Identifier};
use tokio::{
    sync::Notify,
    task::JoinHandle,
    time::{self, Duration},
};
use tracing::{info, warn, error};

/// Map from contract address to their start cursor (exclusive)
pub type SuiTargetModules = HashMap<Identifier, Option<EventID>>;

pub struct SuiSyncer<C> {
    sui_client: Arc<SuiClient<C>>,
    // The last transaction that the syncer has fully processed.
    // Syncer will resume post this transaction (i.e. exclusive), when it starts.
    cursors: SuiTargetModules,
    metrics: Arc<SuiConnectorMetrics>,
    perpetual_tables: Arc<AuthorityPerpetualTables>,
}

impl<C> SuiSyncer<C>
where
    C: SuiClientInner + 'static,
{
    pub fn new(
        sui_client: Arc<SuiClient<C>>,
        cursors: SuiTargetModules,
        metrics: Arc<SuiConnectorMetrics>,
        perpetual_tables: Arc<AuthorityPerpetualTables>,
    ) -> Self {
        Self {
            sui_client,
            cursors,
            metrics,
            perpetual_tables,
        }
    }

    pub async fn run(
        self,
        query_interval: Duration,
        dwallet_mpc_network_keys: Option<Arc<DwalletMPCNetworkKeys>>,
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    ) -> IkaResult<Vec<JoinHandle<()>>> {
        info!("Starting SuiSyncer");
        let mut task_handles = vec![];
        let sui_client_clone = self.sui_client.clone();
        if let Some(dwallet_mpc_network_keys) = dwallet_mpc_network_keys {
            // Todo (#810): Check the usage adding the task handle to the task_handles vector.
            tokio::spawn(Self::sync_dwallet_network_keys(
                sui_client_clone,
                dwallet_mpc_network_keys,
                weighted_threshold_access_structure,
            ));
        }
        for (module, cursor) in self.cursors {
            let metrics = self.metrics.clone();
            let sui_client_clone = self.sui_client.clone();
            let perpetual_tables_clone = self.perpetual_tables.clone();
            task_handles.push(spawn_logged_monitored_task!(
                Self::run_event_listening_task(
                    module,
                    cursor,
                    sui_client_clone,
                    query_interval,
                    metrics,
                    perpetual_tables_clone
                )
            ));
        }
        Ok(task_handles)
    }

    /// Sync the DwalletMPC network keys from the Sui client to the local store.
    async fn sync_dwallet_network_keys(
        sui_client: Arc<SuiClient<C>>,
        dwallet_mpc_network_keys: Arc<DwalletMPCNetworkKeys>,
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    ) {
        loop {
            time::sleep(Duration::from_secs(2)).await;
            let network_decryption_keys = sui_client
                .get_dwallet_mpc_network_keys()
                .await
                .unwrap_or_else(|e| {
                    error!("failed to fetch dwallet MPC network keys: {e}");
                    HashMap::new()
                });
            let mut local_network_decryption_keys =
                dwallet_mpc_network_keys.network_decryption_keys();
            network_decryption_keys
                .into_iter()
                .for_each(|(key_id, network_dec_key_shares)| {
                    if let Some(local_dec_key_shares) = local_network_decryption_keys.get(&key_id) {
                        if *local_dec_key_shares != network_dec_key_shares {
                            info!("Updating the network key for `key_id`: {:?}", key_id);
                            if let Err(e) =
                                dwallet_mpc_network_keys.update_network_key(key_id, network_dec_key_shares, &weighted_threshold_access_structure,)
                            {
                                error!(
                                    "failed to update the key version for key_id: {:?}, error: {:?}",
                                    key_id, e
                                );
                            }
                        }
                    } else {
                        info!("Adding a new network key with ID: {:?}", key_id);
                        if let Err(e) =
                            dwallet_mpc_network_keys.add_new_network_key(key_id, network_dec_key_shares, &weighted_threshold_access_structure,)
                        {
                            error!(
                                "failed to add new key for `key_id`: {:?}, error: {:?}",
                                key_id, e
                            );
                        }
                    }
                });
        }
    }

    async fn run_event_listening_task(
        // The module where interested events are defined.
        // module is always of ika system package.
        module: Identifier,
        mut cursor: Option<EventID>,
        sui_client: Arc<SuiClient<C>>,
        query_interval: Duration,
        metrics: Arc<SuiConnectorMetrics>,
        perpetual_tables: Arc<AuthorityPerpetualTables>,
    ) {
        tracing::info!(?module, ?cursor, "Starting sui events listening task");
        let mut interval = time::interval(query_interval);
        interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

        // Create a task to update metrics
        let notify = Arc::new(Notify::new());
        let notify_clone = notify.clone();
        let sui_client_clone = sui_client.clone();
        let last_synced_sui_checkpoints_metric = metrics
            .last_synced_sui_checkpoints
            .with_label_values(&[&module.to_string()]);
        spawn_logged_monitored_task!(async move {
            loop {
                notify_clone.notified().await;
                let Ok(Ok(latest_checkpoint_sequence_number)) = retry_with_max_elapsed_time!(
                    sui_client_clone.get_latest_checkpoint_sequence_number(),
                    Duration::from_secs(120)
                ) else {
                    tracing::error!("Failed to query latest checkpoint sequence number from sui client after retry");
                    continue;
                };
                last_synced_sui_checkpoints_metric.set(latest_checkpoint_sequence_number as i64);
            }
        });

        loop {
            interval.tick().await;
            let Ok(Ok(events)) = retry_with_max_elapsed_time!(
                sui_client.query_events_by_module(module.clone(), cursor),
                Duration::from_secs(120)
            ) else {
                tracing::error!("Failed to query events from sui client after retry");
                continue;
            };

            let len = events.data.len();
            if len != 0 {
                if !events.has_next_page {
                    // If this is the last page, it means we have processed all events up to the latest checkpoint
                    // We can then update the latest checkpoint metric.
                    notify.notify_one();
                }
                perpetual_tables
                    .insert_pending_events(module.clone(), &events.data)
                    .expect("Failed to insert pending events");
                if let Some(next) = events.next_cursor {
                    cursor = Some(next);
                }
                tracing::info!(?module, ?cursor, "Observed {len} new Sui events");
            }
        }
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
//         let (_handles, mut events_rx) = SuiSyncer::new(client, target_modules, metrics.clone())
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
