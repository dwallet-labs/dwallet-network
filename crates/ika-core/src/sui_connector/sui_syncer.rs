// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! The SuiSyncer module handles synchronizing Events emitted
//! on the Sui blockchain from concerned modules of `ika_system` package.
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
use tracing::{debug, error, info, warn};

/// Map from contract address to their start cursor (exclusive)
pub type SuiTargetModules = HashMap<Identifier, Option<EventID>>;

pub struct SuiSyncer<C> {
    sui_client: Arc<SuiClient<C>>,
    // The last transaction that the syncer has fully processed.
    // Syncer will resume posting this transaction (i.e., exclusive) when it starts.
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
                    warn!("failed to fetch dwallet MPC network keys: {e}");
                    HashMap::new()
                });
            let mut local_network_decryption_keys =
                dwallet_mpc_network_keys.network_decryption_keys();
            network_decryption_keys
                .into_iter()
                .for_each(|(key_id, network_dec_key_shares)| {
                    if let Some(local_dec_key_shares) = local_network_decryption_keys.get(&key_id) {
                        info!("Updating the network key for `key_id`: {:?}", key_id);
                        if *local_dec_key_shares != network_dec_key_shares {
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
        // Module is always of ika system package.
        module: Identifier,
        mut cursor: Option<EventID>,
        sui_client: Arc<SuiClient<C>>,
        query_interval: Duration,
        metrics: Arc<SuiConnectorMetrics>,
        perpetual_tables: Arc<AuthorityPerpetualTables>,
    ) {
        info!(?module, ?cursor, "Starting sui events listening task");
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
                    error!("failed to query the latest checkpoint sequence number from the sui client after retry");
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
                error!("failed to query events from the sui client — retrying");
                continue;
            };

            let len = events.data.len();
            if len != 0 {
                if !events.has_next_page {
                    // If this is the last page, it means we have processed all
                    // events up to the latest checkpoint
                    // We can then update the latest checkpoint metric.
                    notify.notify_one();
                }
                perpetual_tables
                    .insert_pending_events(module.clone(), &events.data)
                    // todo(zeev): this code can panic, check it.
                    .expect("failed to insert pending events");
                if let Some(next) = events.next_cursor {
                    cursor = Some(next);
                }
                info!(
                    ?module,
                    ?cursor,
                    "Observed {len} new events from Sui network"
                );
            }
        }
    }
}
