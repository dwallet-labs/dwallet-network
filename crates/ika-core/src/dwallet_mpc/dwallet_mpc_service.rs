//! This module contains the DWalletMPCService struct.
//! It is responsible to read DWallet MPC messages from the
//! local DB every [`READ_INTERVAL_MS`] seconds
//! and forward them to the [`DWalletMPCManager`].

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::mpc_manager::{DWalletMPCDBMessage, DWalletMPCManager};
use crate::dwallet_mpc::session_info_from_event;
use dwallet_mpc_types::dwallet_mpc::{MPCSessionStatus, NetworkDecryptionKeyPublicData};
use ika_config::NodeConfig;
use ika_sui_client::SuiConnectorClient;
use ika_types::committee::Committee;
use ika_types::messages_dwallet_mpc::DWalletMPCEvent;
use ika_types::sui::{DWalletCoordinatorInner, SystemInner};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use sui_types::base_types::{EpochId, ObjectID};
use sui_types::messages_consensus::Round;
use tokio::sync::watch::Receiver;
use tokio::sync::Notify;
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
}

impl DWalletMPCService {
    pub async fn new(
        epoch_store: Arc<AuthorityPerEpochStore>,
        exit: Receiver<()>,
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        node_config: NodeConfig,
        sui_client: Arc<SuiConnectorClient>,
        network_keys_receiver: Receiver<Arc<HashMap<ObjectID, NetworkDecryptionKeyPublicData>>>,
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
    pub async fn spawn(&mut self) {
        self.load_missed_events().await;
        loop {
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

            if !completed_sessions_ids.is_empty() {
                // Delete all completed sessions from the local DB.
                if let Ok(tables) = self.epoch_store.tables() {
                    let mut batch = tables
                        .dwallet_mpc_events_for_pending_and_active_sessions
                        .batch();
                    if let Err(e) = batch.delete_batch(
                        &tables.dwallet_mpc_events_for_pending_and_active_sessions,
                        completed_sessions_ids.clone(),
                    ) {
                        error!(error=?e,
                            session_id=?completed_sessions_ids,
                            "failed to delete batch for session");
                    }
                    if let Err(e) = batch.write() {
                        error!(error=?e,
                            session_id=?completed_sessions_ids,
                            "failed to write batch for session");
                    }
                }
            }

            // Read **new** dWallet MPC events from sui, save them to the local DB.
            if let Err(e) = self.epoch_store.read_new_sui_events().await {
                error!(
                    error=?e,
                    "failed to load dWallet MPC events from the local DB");
                continue;
            };

            // Read all dWallet MPC events for uncompleted sessions from the local DB and handle them.
            let dwallet_mpc_events_for_pending_and_active_sessions = match self.epoch_store.tables() {
                Ok(tables) => tables.get_all_dwallet_mpc_events_for_pending_and_active_sessions().unwrap_or_else(|e| {
                    error!(error=?e, "failed to get all dWallet MPC events for uncompleted sessions");
                    vec![]
                }),
                Err(e) => {
                    error!(error=?e, "failed to get tables from epoch store");
                    vec![]
                }
            };

            // If session is already exists with event information, it will be ignored.
            for event in dwallet_mpc_events_for_pending_and_active_sessions {
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
}
