// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! The SuiSyncer module handles synchronizing Events emitted
//! on the Sui blockchain from concerned modules of `ika_system` package.
use crate::dwallet_mpc::generate_access_structure_from_committee;
use crate::dwallet_mpc::network_dkg::instantiate_dwallet_mpc_network_decryption_key_shares_from_public_output;
use crate::sui_connector::metrics::SuiConnectorMetrics;
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, NetworkDecryptionKeyPublicData};
use ika_sui_client::{retry_with_max_elapsed_time, SuiClient, SuiClientInner};
use ika_types::committee::{Committee, StakeUnit};
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::error::IkaResult;
use ika_types::messages_dwallet_mpc::DWalletNetworkDecryptionKey;
use ika_types::sui::{SystemInner, SystemInnerInit, SystemInnerTrait};
use im::HashSet;
use mpc::WeightedThresholdAccessStructure;
use mysten_metrics::spawn_logged_monitored_task;
use std::{collections::HashMap, sync::Arc};
use sui_json_rpc_types::SuiEvent;
use sui_types::base_types::ObjectID;
use sui_types::{event::EventID, Identifier};
use tokio::sync::watch::Sender;
use tokio::{
    sync::Notify,
    task::JoinHandle,
    time::{self, Duration},
};
use tracing::{debug, error, info, warn};

pub struct SuiSyncer<C> {
    sui_client: Arc<SuiClient<C>>,
    // The last transaction that the syncer has fully processed.
    // Syncer will resume posting this transaction (i.e., exclusive) when it starts.
    modules: Vec<Identifier>,
    metrics: Arc<SuiConnectorMetrics>,
}

impl<C> SuiSyncer<C>
where
    C: SuiClientInner + 'static,
{
    pub fn new(
        sui_client: Arc<SuiClient<C>>,
        modules: Vec<Identifier>,
        metrics: Arc<SuiConnectorMetrics>,
    ) -> Self {
        Self {
            sui_client,
            modules,
            metrics,
        }
    }

    pub async fn run(
        self,
        query_interval: Duration,
        next_epoch_committee_sender: Sender<Committee>,
        network_keys_sender: Sender<Arc<HashMap<ObjectID, NetworkDecryptionKeyPublicData>>>,
        new_events_sender: tokio::sync::broadcast::Sender<Vec<SuiEvent>>,
    ) -> IkaResult<Vec<JoinHandle<()>>> {
        info!("Starting SuiSyncer");
        let mut task_handles = vec![];
        let sui_client_clone = self.sui_client.clone();
        tokio::spawn(Self::sync_next_committee(
            sui_client_clone.clone(),
            next_epoch_committee_sender,
        ));
        // Todo (#810): Check the usage adding the task handle to the task_handles vector.
        tokio::spawn(Self::sync_dwallet_network_keys(
            sui_client_clone,
            network_keys_sender,
        ));
        for module in self.modules {
            let metrics = self.metrics.clone();
            let sui_client_clone = self.sui_client.clone();
            let new_events_sender_clone = new_events_sender.clone();
            task_handles.push(spawn_logged_monitored_task!(
                Self::run_event_listening_task(
                    module,
                    sui_client_clone,
                    query_interval,
                    metrics,
                    new_events_sender_clone,
                )
            ));
        }
        Ok(task_handles)
    }

    async fn sync_next_committee(
        sui_client: Arc<SuiClient<C>>,
        next_epoch_committee_sender: Sender<Committee>,
    ) {
        loop {
            time::sleep(Duration::from_secs(10)).await;
            let system_inner = sui_client.must_get_system_inner_object().await;
            let SystemInner::V1(system_inner) = system_inner;
            let Some(new_next_bls_committee) = system_inner.get_ika_next_epoch_committee() else {
                debug!("ika next epoch active committee not found, retrying...");
                continue;
            };

            let new_next_committee = system_inner.read_bls_committee(&new_next_bls_committee);

            let committee = match Self::new_committee(
                sui_client.clone(),
                &system_inner,
                new_next_committee.clone(),
                system_inner.epoch() + 1,
                new_next_bls_committee.quorum_threshold,
                new_next_bls_committee.validity_threshold,
            )
            .await
            {
                Ok(committee) => committee,
                Err(e) => {
                    error!("failed to initiate the next committee: {e}");
                    continue;
                }
            };
            let committee_epoch = committee.epoch();
            if let Err(err) = next_epoch_committee_sender.send(committee) {
                error!(?err, committee_epoch=?committee_epoch, "failed to send the next epoch committee to the channel");
            } else {
                info!(committee_epoch=?committee_epoch, "The next epoch committee was sent successfully");
            }
        }
    }

    async fn new_committee(
        sui_client: Arc<SuiClient<C>>,
        system_inner: &SystemInnerInit,
        committee: Vec<(ObjectID, (AuthorityName, StakeUnit))>,
        epoch: u64,
        quorum_threshold: u64,
        validity_threshold: u64,
    ) -> DwalletMPCResult<Committee> {
        let validator_ids: Vec<_> = committee.iter().map(|(id, _)| *id).collect();

        let validators = sui_client
            .get_validators_info_by_ids(system_inner, validator_ids)
            .await
            .map_err(DwalletMPCError::IkaError)?;

        let class_group_encryption_keys_and_proofs = sui_client
            .get_class_groups_public_keys_and_proofs(&validators)
            .await
            .map_err(DwalletMPCError::IkaError)?;

        let class_group_encryption_keys_and_proofs = committee
            .iter()
            .map(|(id, (name, _))| {
                let validator_class_groups_public_key_and_proof =
                    class_group_encryption_keys_and_proofs
                        .get(id)
                        .ok_or(DwalletMPCError::ValidatorIDNotFound(*id))?;

                let validator_class_groups_public_key_and_proof =
                    bcs::to_bytes(&validator_class_groups_public_key_and_proof)?;
                Ok((*name, validator_class_groups_public_key_and_proof))
            })
            .collect::<DwalletMPCResult<HashMap<_, _>>>()?;

        Ok(Committee::new(
            epoch,
            committee
                .iter()
                .map(|(_, (name, stake))| (*name, *stake))
                .collect(),
            class_group_encryption_keys_and_proofs,
            quorum_threshold,
            validity_threshold,
        ))
    }

    /// Sync the DwalletMPC network keys from the Sui client to the local store.
    async fn sync_dwallet_network_keys(
        sui_client: Arc<SuiClient<C>>,
        network_keys_sender: Sender<Arc<HashMap<ObjectID, NetworkDecryptionKeyPublicData>>>,
    ) {
        // (Key Obj ID, Epoch)
        let mut network_keys_cache: HashSet<(ObjectID, u64)> = HashSet::new();
        'sync_network_keys: loop {
            time::sleep(Duration::from_secs(5)).await;

            let system_inner = sui_client.must_get_system_inner_object().await;
            let SystemInner::V1(system_inner) = system_inner;
            let network_encryption_keys = sui_client
                .get_dwallet_mpc_network_keys()
                .await
                .unwrap_or_else(|e| {
                    warn!("failed to fetch dwallet MPC network keys: {e}");
                    HashMap::new()
                });
            if network_encryption_keys
                .iter()
                .any(|(_, key)| key.current_epoch != system_inner.epoch())
            {
                // Gather all the (ObjectID, current epoch) pairs that are out of date.
                let mismatches: Vec<(ObjectID, u64)> = network_encryption_keys
                    .iter()
                    .filter(|(_, key)| key.current_epoch != system_inner.epoch())
                    .map(|(id, key)| (*id, key.current_epoch))
                    .collect();
                warn!(
                    keys_current_epoch=?mismatches,
                    system_inner_epoch=?system_inner.epoch(),
                    "Network encryption keys are out-of-date for this authority"
                );
                continue;
            }
            let active_bls_committee = system_inner.get_ika_active_committee();
            let active_committee = system_inner.read_bls_committee(&active_bls_committee);
            let current_keys = system_inner.dwallet_2pc_mpc_coordinator_network_encryption_keys();
            let should_fetch_keys = current_keys.iter().any(|key| {
                !network_keys_cache
                    .contains(&(key.dwallet_network_decryption_key_id, system_inner.epoch()))
            });
            if !should_fetch_keys {
                info!("No new network keys to fetch");
                continue;
            }
            let active_committee = match Self::new_committee(
                sui_client.clone(),
                &system_inner,
                active_committee,
                system_inner.epoch(),
                active_bls_committee.quorum_threshold,
                active_bls_committee.validity_threshold,
            )
            .await
            {
                Ok(committee) => committee,
                Err(e) => {
                    error!("failed to initiate committee: {e}");
                    continue;
                }
            };
            let weighted_threshold_access_structure =
                match generate_access_structure_from_committee(&active_committee) {
                    Ok(access_structure) => access_structure,
                    Err(e) => {
                        error!("failed to generate access structure: {e}");
                        continue;
                    }
                };
            let mut all_network_keys_data = HashMap::new();
            for (key_id, network_dec_key_shares) in network_encryption_keys.into_iter() {
                match Self::fetch_and_init_network_key(
                    &sui_client,
                    &network_dec_key_shares,
                    &weighted_threshold_access_structure,
                )
                .await
                {
                    Ok(key) => {
                        all_network_keys_data.insert(key_id, key.clone());
                        network_keys_cache.insert((key_id, key.epoch));
                        info!(
                            key_id=?key_id,
                            "Successfully synced the network decryption key for `key_id`",
                        );
                    }
                    Err(DwalletMPCError::WaitingForNetworkKey(key_id)) => {
                        // This is expected if the key is not yet available.
                        // We can skip this key and continue to the next one.
                        info!(key=?key_id, "Waiting for network decryption key data");
                        continue 'sync_network_keys;
                    }
                    Err(err) => {
                        // DO NOT CHANGE THIS TO WARNING!
                        // THIS IS A CRITICAL ERROR IN SOME CASES!
                        error!(
                            key=?key_id,
                            err=?err,
                            "failed to get network decryption key data, retrying...",
                        );
                        continue 'sync_network_keys;
                    }
                }
            }
            if let Err(err) = network_keys_sender.send(Arc::new(all_network_keys_data)) {
                error!(?err, "failed to send network keys data to the channel",);
            }
        }
    }

    async fn fetch_and_init_network_key(
        sui_client: &SuiClient<C>,
        network_dec_key_shares: &DWalletNetworkDecryptionKey,
        access_structure: &WeightedThresholdAccessStructure,
    ) -> DwalletMPCResult<NetworkDecryptionKeyPublicData> {
        let output = sui_client
            .get_network_decryption_key_with_full_data(network_dec_key_shares)
            .await
            .map_err(|e| DwalletMPCError::MissingDwalletMPCDecryptionKeyShares(e.to_string()))?;

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
        sui_client: Arc<SuiClient<C>>,
        query_interval: Duration,
        metrics: Arc<SuiConnectorMetrics>,
        new_events_sender: tokio::sync::broadcast::Sender<Vec<SuiEvent>>,
    ) {
        info!(?module, "Starting sui events listening task");
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
        let mut cursor: Option<EventID> = None;
        let mut start_epoch_cursor: Option<EventID> = None;
        let mut loop_index: usize = 0;
        loop {
            // Fetching the epoch start TX digest less frequently
            // as it is unexpected to change often.
            if loop_index % 10 == 0 {
                debug!("Querying epoch start cursor from Sui");
                let SystemInner::V1(system_inner) = sui_client.must_get_system_inner_object().await;
                let Ok(epoch_start_tx_digest) = system_inner.epoch_start_tx_digest.try_into()
                else {
                    // This should not happen, but if it does, we need to know about it.
                    error!("cloud not parse `epoch_start_tx_digest` - wrong length");
                    continue;
                };
                let start_epoch_event = EventID::from((epoch_start_tx_digest, 0));
                if start_epoch_cursor != Some(start_epoch_event) {
                    start_epoch_cursor = Some(start_epoch_event);
                    cursor = start_epoch_cursor;
                }
            }
            loop_index += 1;

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
                if let Err(e) = new_events_sender.send(events.data) {
                    error!(error=?e, ?module, "failed to send new events to the channel");
                }

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
