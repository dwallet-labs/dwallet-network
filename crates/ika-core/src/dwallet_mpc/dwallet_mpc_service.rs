//! This module contains the DWalletMPCService struct.
//! It is responsible to read DWallet MPC messages from the
//! local DB every [`READ_INTERVAL_MS`] seconds
//! and forward them to the [`DWalletMPCManager`].

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::generate_access_structure_from_committee;
use crate::dwallet_mpc::mpc_manager::{DWalletMPCDBMessage, DWalletMPCManager};
use crate::dwallet_mpc::mpc_session::session_info_from_event;
use crate::dwallet_mpc::network_dkg::instantiate_dwallet_mpc_network_decryption_key_shares_from_public_output;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCSessionStatus, NetworkDecryptionKeyPublicData,
};
use ika_config::NodeConfig;
use ika_sui_client::SuiConnectorClient;
use ika_types::committee::{Committee, StakeUnit};
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::error::{IkaError, IkaResult};
use ika_types::messages_dwallet_mpc::{DBSuiEvent, DWalletMPCEvent, DWalletNetworkDecryptionKey};
use ika_types::sui::{DWalletCoordinatorInner, SystemInner, SystemInnerInit, SystemInnerTrait};
use mpc::WeightedThresholdAccessStructure;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use sui_json_rpc_types::SuiEvent;
use sui_types::base_types::{EpochId, ObjectID};
use sui_types::event::EventID;
use sui_types::messages_consensus::Round;
use tokio::sync::broadcast::error::TryRecvError;
use tokio::sync::watch::{Receiver, Sender};
use tokio::sync::Notify;
use tokio::time;
use tracing::{debug, error, info, warn};
use typed_store::Map;

const READ_INTERVAL_MS: u64 = 100;

pub struct DWalletMPCService {
    last_read_consensus_round: Round,
    #[allow(dead_code)]
    read_messages: usize,
    epoch_store: Arc<AuthorityPerEpochStore>,
    epoch_id: EpochId,
    #[allow(dead_code)]
    notify: Arc<Notify>,
    sui_client: Arc<SuiConnectorClient>,
    dwallet_mpc_manager: DWalletMPCManager,
    pub exit: Receiver<()>,
    pub network_keys_receiver: Receiver<Arc<HashMap<ObjectID, NetworkDecryptionKeyPublicData>>>,
    pub new_events_receiver: tokio::sync::broadcast::Receiver<Vec<SuiEvent>>,
}

impl DWalletMPCService {
    pub async fn new(
        epoch_store: Arc<AuthorityPerEpochStore>,
        exit: Receiver<()>,
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        node_config: NodeConfig,
        sui_client: Arc<SuiConnectorClient>,
        network_keys_receiver: Receiver<Arc<HashMap<ObjectID, NetworkDecryptionKeyPublicData>>>,
        new_events_receiver: tokio::sync::broadcast::Receiver<Vec<SuiEvent>>,
        next_epoch_committee_receiver: Receiver<Committee>,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    ) -> Self {
        let dwallet_mpc_manager = DWalletMPCManager::must_create_dwallet_mpc_manager(
            consensus_adapter.clone(),
            epoch_store.clone(),
            next_epoch_committee_receiver,
            node_config,
            dwallet_mpc_metrics,
        )
        .await;
        Self {
            last_read_consensus_round: 0,
            read_messages: 0,
            epoch_store: epoch_store.clone(),
            epoch_id: epoch_store.epoch(),
            notify: Arc::new(Notify::new()),
            sui_client: sui_client.clone(),
            dwallet_mpc_manager,
            network_keys_receiver,
            new_events_receiver,
            exit,
        }
    }

    async fn update_last_session_to_complete_in_current_epoch(&mut self) {
        let system_inner = self.sui_client.must_get_system_inner_object().await;
        let SystemInner::V1(system_inner) = system_inner;
        if let Some(dwallet_coordinator_id) = system_inner.dwallet_2pc_mpc_coordinator_id {
            let coordinator_state = self
                .sui_client
                .must_get_dwallet_coordinator_inner(dwallet_coordinator_id)
                .await;
            let DWalletCoordinatorInner::V1(inner_state) = coordinator_state;
            self.dwallet_mpc_manager
                .update_last_session_to_complete_in_current_epoch(
                    inner_state
                        .session_management
                        .last_session_to_complete_in_current_epoch,
                );
        }
    }

    /// Load missed events from the Sui network.
    /// These events are from different Epochs, not necessarily the current one.
    ///
    async fn load_missed_events(&mut self) {
        let epoch_store = self.epoch_store.clone();
        loop {
            let Ok(events) = self
                .sui_client
                .get_dwallet_mpc_missed_events(epoch_store.epoch())
                .await
            else {
                error!("failed to fetch missed dWallet MPC events from Sui");
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            };
            for event in events {
                match session_info_from_event(event.clone(), &epoch_store.packages_config) {
                    Ok(Some(mut session_info)) => {
                        // We modify the session info to include the current epoch ID,
                        // or else
                        // this event will be ignored while handled.
                        session_info.epoch = self.epoch_id;
                        self.dwallet_mpc_manager
                            .handle_dwallet_db_event(DWalletMPCEvent {
                                event,
                                session_info: session_info.clone(),
                            })
                            .await;
                        info!(
                            session_identifier=?session_info.session_identifier,
                            session_type=?session_info.session_type,
                            mpc_round=?session_info.mpc_round,
                            "Successfully processed a missed event from Sui"
                        );
                    }
                    Ok(None) => {
                        warn!("Received an event that does not trigger the start of an MPC flow");
                    }
                    Err(e) => {
                        error!(
                            erorr=?e,
                            "error while processing a missed event"
                        );
                    }
                }
            }
            return;
        }
    }

    async fn update_network_keys(&mut self) {
        match self.network_keys_receiver.has_changed() {
            Ok(has_changed) => {
                if has_changed {
                    let new_keys = self.network_keys_receiver.borrow_and_update();
                    for (key_id, key_data) in new_keys.iter() {
                        info!("Updating network key for key_id: {:?}", key_id);
                        self.dwallet_mpc_manager
                            .network_keys
                            .update_network_key(
                                *key_id,
                                key_data,
                                &self.dwallet_mpc_manager.weighted_threshold_access_structure,
                            )
                            .unwrap_or_else(|err| error!(?err, "failed to store network keys"));
                    }
                }
            }
            Err(err) => {
                error!(?err, "failed to check network keys receiver");
            }
        }
    }

    /// Starts the DWallet MPC service.
    ///
    /// This service periodically reads DWallet MPC messages from the local database
    /// at intervals defined by [`READ_INTERVAL_SECS`] seconds.
    /// The messages are then forwarded to the
    /// [`DWalletMPCManager`] for processing.
    ///
    /// The service automatically terminates when an epoch switch occurs.
    pub async fn spawn(
        &mut self,
        next_epoch_committee_sender: Sender<Committee>,
        network_keys_sender: Sender<Arc<HashMap<ObjectID, NetworkDecryptionKeyPublicData>>>,
    ) {
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
        let mut loop_index = 0;
        loop {
            // Load events from Sui every 5 minutes.
            if loop_index % 3_000 == 0 {
                self.load_missed_events().await;
            }
            loop_index += 1;
            match self.exit.has_changed() {
                Ok(true) => {
                    warn!("DWalletMPCService exit signal received");
                    break;
                }
                Err(err) => {
                    warn!(err=?err, "DWalletMPCService exit channel was shutdown incorrectly");
                    break;
                }
                Ok(false) => (),
            };
            tokio::time::sleep(Duration::from_millis(READ_INTERVAL_MS)).await;

            if self.dwallet_mpc_manager.recognized_self_as_malicious {
                error!(
                    authority=?self.epoch_store.name,
                    "the node has identified itself as malicious and is no longer participating in MPC protocols"
                );
                tokio::time::sleep(Duration::from_secs(120)).await;
                continue;
            }
            self.update_network_keys().await;

            debug!("Running DWalletMPCService loop");
            self.dwallet_mpc_manager
                .cryptographic_computations_orchestrator
                .check_for_completed_computations();
            self.update_last_session_to_complete_in_current_epoch()
                .await;
            let Ok(tables) = self.epoch_store.tables() else {
                warn!("failed to load DB tables from the epoch store");
                continue;
            };
            let Ok(completed_sessions) = self
                .epoch_store
                .load_dwallet_mpc_completed_sessions_from_round(self.last_read_consensus_round + 1)
                .await
            else {
                error!("failed to load dWallet MPC completed sessions from the local DB");
                continue;
            };

            let mut completed_sessions_ids = Vec::new();
            for session_id in completed_sessions {
                if let Some(session) = self.dwallet_mpc_manager.mpc_sessions.get_mut(&session_id) {
                    session.clear_data();
                    session.status = MPCSessionStatus::Finished;
                    completed_sessions_ids.push(session.session_identifier);
                }
            }

            // Receive **new** dWallet MPC events and save them in the local DB.
            let events = match self.receive_new_sui_events() {
                Ok(events) => events,
                Err(e) => {
                    error!(
                    error=?e,
                    "failed to receive dWallet MPC events");
                    continue;
                }
            };

            // If session is already exists with event information, it will be ignored.
            for event in events {
                self.dwallet_mpc_manager
                    .handle_dwallet_db_event(event)
                    .await;
            }
            let mpc_msgs_iter = tables
                .dwallet_mpc_messages
                .safe_iter_with_bounds(Some(self.last_read_consensus_round + 1), None)
                .collect::<Result<Vec<_>, _>>();
            let mpc_msgs_iter = match mpc_msgs_iter {
                Ok(iter) => iter,
                Err(e) => {
                    error!(err=?e, "failed to load DWallet MPC messages from the local DB");
                    continue;
                }
            };

            for (round, messages) in mpc_msgs_iter {
                self.last_read_consensus_round = round;
                for message in messages {
                    self.dwallet_mpc_manager
                        .handle_dwallet_db_message(message)
                        .await;
                }
                self.dwallet_mpc_manager
                    .handle_dwallet_db_message(DWalletMPCDBMessage::EndOfDelivery)
                    .await;
            }

            self.dwallet_mpc_manager
                .handle_dwallet_db_message(DWalletMPCDBMessage::PerformCryptographicComputations)
                .await;
        }
    }

    /// Read events from perpetual tables, remove them, and store in the current epoch tables.
    fn receive_new_sui_events(&mut self) -> IkaResult<Vec<DWalletMPCEvent>> {
        let pending_events = match self.new_events_receiver.try_recv() {
            Ok(events) => events,
            Err(TryRecvError::Empty) => {
                debug!("No new Sui events to process");
                return Ok(vec![]);
            }
            Err(e) => {
                return Err(IkaError::ReveiverError(e.to_string()));
            }
        };

        let pending_events = pending_events
            .iter()
            .map(|e| {
                let serialized_event = bcs::to_bytes(&DBSuiEvent {
                    type_: e.type_.clone(),
                    contents: e.bcs.clone().into_bytes(),
                })
                .map_err(|e| IkaError::BCSError(e.to_string()))?;
                Ok((e.id, serialized_event))
            })
            .collect::<IkaResult<Vec<(EventID, Vec<u8>)>>>()?;
        let events: Vec<DWalletMPCEvent> = pending_events
            .iter()
            .filter_map(|(_id, event)| match bcs::from_bytes::<DBSuiEvent>(event) {
                Ok(event) => {
                    match session_info_from_event(event.clone(), &self.epoch_store.packages_config)
                    {
                        Ok(Some(session_info)) => {
                            info!(
                                mpc_protocol=?session_info.mpc_round,
                                session_identifier=?session_info.session_identifier,
                                validator=?self.epoch_store.name,
                                "Received start event for session"
                            );
                            let event = DWalletMPCEvent {
                                event,
                                session_info,
                            };
                            Some(event)
                        }
                        Ok(None) => {
                            warn!(
                                event=?event,
                                "Received an event that does not trigger the start of an MPC flow"
                            );
                            None
                        }
                        Err(e) => {
                            error!("error getting session info from event: {}", e);
                            None
                        }
                    }
                }
                Err(e) => {
                    error!("failed to deserialize event: {}", e);
                    None
                }
            })
            .collect();

        Ok(events)
    }

    async fn sync_next_committee(
        sui_client: Arc<SuiConnectorClient>,
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
        sui_client: Arc<SuiConnectorClient>,
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
        sui_client: Arc<SuiConnectorClient>,
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
                        warn!(
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
        sui_client: &Arc<SuiConnectorClient>,
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
}
