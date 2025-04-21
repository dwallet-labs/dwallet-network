// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! The SuiSyncer module handles synchronizing Events emitted
//! on the Sui blockchain from concerned modules of `ika_system` package.
use crate::authority::authority_perpetual_tables::AuthorityPerpetualTables;
use crate::dwallet_mpc::network_dkg::{
    instantiate_dwallet_mpc_network_decryption_key_shares_from_public_output, DwalletMPCNetworkKeys,
};
use crate::sui_connector::metrics::SuiConnectorMetrics;
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, NetworkDecryptionKeyPublicData};
use ika_sui_client::{retry_with_max_elapsed_time, SuiClient, SuiClientInner};
use ika_types::committee::Committee;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::error::IkaResult;
use ika_types::messages_dwallet_mpc::DWalletNetworkDecryptionKey;
use ika_types::sui::SystemInnerTrait;
use itertools::Itertools;
use mpc::WeightedThresholdAccessStructure;
use mysten_metrics::spawn_logged_monitored_task;
use std::{collections::HashMap, sync::Arc};
use sui_json_rpc_types::SuiEvent;
use sui_types::base_types::ObjectID;
use sui_types::BRIDGE_PACKAGE_ID;
use sui_types::{event::EventID, Identifier};
use tokio::sync::RwLock;
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
        next_epoch_committee: Arc<RwLock<Option<Committee>>>,
    ) -> IkaResult<Vec<JoinHandle<()>>> {
        info!("Starting SuiSyncer");
        let mut task_handles = vec![];
        let sui_client_clone = self.sui_client.clone();
        tokio::spawn(Self::sync_next_committee(
            sui_client_clone.clone(),
            next_epoch_committee,
        ));
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

    async fn sync_next_committee(
        sui_client: Arc<SuiClient<C>>,
        next_epoch_committee: Arc<RwLock<Option<Committee>>>,
    ) {
        loop {
            time::sleep(Duration::from_secs(2)).await;
            let system_inner = sui_client.get_system_inner_until_success().await;
            let system_inner = system_inner.into_init_version_for_tooling();

            let Some(new_next_committee) = system_inner.get_ika_next_epoch_committee() else {
                let mut committee_lock = next_epoch_committee.write().await;
                *committee_lock = None;
                debug!("ika next epoch active committee not found, retrying...");
                continue;
            };

            let validator_ids: Vec<_> = new_next_committee.keys().cloned().collect();

            let validators = match sui_client
                .get_validators_info_by_ids(&system_inner, validator_ids)
                .await
            {
                Ok(v) => v,
                Err(e) => {
                    error!("failed to fetch validators info: {e}");
                    continue;
                }
            };

            let class_group_encryption_keys_and_proofs = match sui_client
                .get_class_groups_public_keys_and_proofs(&validators)
                .await
            {
                Ok(data) => data,
                Err(e) => {
                    error!("can't get_class_groups_public_keys_and_proofs: {e}");
                    continue;
                }
            };

            let class_group_encryption_keys_and_proofs = class_group_encryption_keys_and_proofs
                .into_iter()
                .filter_map(|(id, class_groups)| {
                    let authority_name = match new_next_committee.get(&id) {
                        Some((authority_name, _)) => *authority_name,
                        None => {
                            error!("missing validator authority name for id: {id}");
                            return None;
                        }
                    };

                    match bcs::to_bytes(&class_groups) {
                        Ok(bytes) => Some((authority_name, bytes)),
                        Err(e) => {
                            error!("failed to serialize class group for id {id}: {e}");
                            None
                        }
                    }
                })
                .collect();

            let committee = Committee::new(
                system_inner.epoch + 1,
                new_next_committee.values().cloned().collect(),
                class_group_encryption_keys_and_proofs,
            );

            let mut committee_lock = next_epoch_committee.write().await;
            *committee_lock = Some(committee);
        }
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

            for (key_id, network_dec_key_shares) in network_decryption_keys.into_iter() {
                match Self::sync_network_decryption_key_inner(
                    &sui_client,
                    dwallet_mpc_network_keys.clone(),
                    &weighted_threshold_access_structure,
                    &key_id,
                    &network_dec_key_shares,
                )
                .await
                {
                    Ok(_) => {
                        info!(
                            "Successfully synced network decryption key for key_id: {:?}",
                            key_id
                        );
                    }
                    Err(e) => {
                        error!(
                            "Failed to sync network decryption key for key_id: {:?}, error: {:?}",
                            key_id, e
                        );
                    }
                }
            }
        }
    }

    async fn sync_network_decryption_key_inner(
        sui_client: &Arc<SuiClient<C>>,
        dwallet_mpc_network_keys: Arc<DwalletMPCNetworkKeys>,
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
        key_id: &ObjectID,
        network_dec_key_shares: &DWalletNetworkDecryptionKey,
    ) -> DwalletMPCResult<()> {
        let local_network_decryption_keys = dwallet_mpc_network_keys.network_decryption_keys();

        let should_update = match local_network_decryption_keys.get(key_id) {
            Some(local_key) => local_key.epoch != network_dec_key_shares.current_epoch,
            None => true,
        };

        if !should_update {
            info!(
                "Network decryption key for key_id: {:?} is up to date",
                key_id
            );
            return Ok(());
        }

        let key = Self::fetch_and_create_network_key(
            &sui_client,
            &network_dec_key_shares,
            &weighted_threshold_access_structure,
        )
        .await?;

        if local_network_decryption_keys.contains_key(&key_id) {
            info!("Updating network key for key_id: {:?}", key_id);
            dwallet_mpc_network_keys.update_network_key(
                *key_id,
                key,
                &weighted_threshold_access_structure,
            )
        } else {
            info!("Adding new network key for key_id: {:?}", key_id);
            dwallet_mpc_network_keys.add_new_network_key(
                *key_id,
                key,
                &weighted_threshold_access_structure,
            )
        }
    }

    async fn fetch_and_create_network_key(
        sui_client: &SuiClient<C>,
        network_dec_key_shares: &DWalletNetworkDecryptionKey,
        access_structure: &WeightedThresholdAccessStructure,
    ) -> DwalletMPCResult<NetworkDecryptionKeyPublicData> {
        let output = sui_client
            .get_network_decryption_key_with_full_data(network_dec_key_shares)
            .await
            .map_err(|e| DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?;

        instantiate_dwallet_mpc_network_decryption_key_shares_from_public_output(
            output.current_epoch,
            DWalletMPCNetworkKeyScheme::Secp256k1,
            access_structure,
            output,
        )
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
