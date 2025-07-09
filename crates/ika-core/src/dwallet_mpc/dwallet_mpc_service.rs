//! This module contains the DWalletMPCService struct.
//! It is responsible to read DWallet MPC messages from the
//! local DB every [`READ_INTERVAL_MS`] seconds
//! and forward them to the [`DWalletMPCManager`].

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::mpc_manager::{DWalletMPCDBMessage, DWalletMPCManager};
use dwallet_mpc_types::dwallet_mpc::MPCSessionStatus;
use ika_config::NodeConfig;
use ika_sui_client::SuiConnectorClient;
use ika_types::committee::Committee;
use ika_types::messages_dwallet_mpc::{DWalletNetworkEncryptionKeyData, SessionIdentifier};
use ika_types::sui::DWalletCoordinatorInner;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use sui_json_rpc_types::SuiEvent;
use sui_types::base_types::ObjectID;
use sui_types::messages_consensus::Round;
use tokio::sync::mpsc;
use tokio::sync::watch::Receiver;
use tracing::{debug, error, info, warn};
use typed_store::Map;

const READ_INTERVAL_MS: u64 = 100;

pub struct DWalletMPCService {
    last_read_consensus_round: Round,
    pub(crate) epoch_store: Arc<AuthorityPerEpochStore>,
    pub(crate) sui_client: Arc<SuiConnectorClient>,
    dwallet_mpc_manager: DWalletMPCManager,
    pub exit: Receiver<()>,
    consensus_round_completed_sessions_receiver: mpsc::UnboundedReceiver<SessionIdentifier>,
    pub new_events_receiver: tokio::sync::broadcast::Receiver<Vec<SuiEvent>>,
}

impl DWalletMPCService {
    pub fn new(
        epoch_store: Arc<AuthorityPerEpochStore>,
        exit: Receiver<()>,
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        node_config: NodeConfig,
        sui_client: Arc<SuiConnectorClient>,
        network_keys_receiver: Receiver<Arc<HashMap<ObjectID, DWalletNetworkEncryptionKeyData>>>,
        new_events_receiver: tokio::sync::broadcast::Receiver<Vec<SuiEvent>>,
        next_epoch_committee_receiver: Receiver<Committee>,
        consensus_round_completed_sessions_receiver: mpsc::UnboundedReceiver<SessionIdentifier>,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    ) -> Self {
        let dwallet_mpc_manager = DWalletMPCManager::must_create_dwallet_mpc_manager(
            consensus_adapter.clone(),
            epoch_store.clone(),
            network_keys_receiver,
            next_epoch_committee_receiver,
            node_config,
            dwallet_mpc_metrics,
        );

        Self {
            last_read_consensus_round: 0,
            epoch_store: epoch_store.clone(),
            sui_client: sui_client.clone(),
            dwallet_mpc_manager,
            new_events_receiver,
            consensus_round_completed_sessions_receiver,
            exit,
        }
    }

    async fn sync_last_session_to_complete_in_current_epoch(&mut self) {
        let coordinator_state = self.sui_client.must_get_dwallet_coordinator_inner().await;

        let DWalletCoordinatorInner::V1(inner) = coordinator_state;
        self.dwallet_mpc_manager
            .sync_last_session_to_complete_in_current_epoch(
                inner
                    .sessions_manager
                    .last_user_initiated_session_to_complete_in_current_epoch,
            );
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
        // Receive all MPC session outputs we bootstrapped from storage and
        // consensus before starting execution, to avoid their computation.
        self.receive_completed_mpc_session_identifiers(true);
        info!(
            validator=?self.epoch_store.name,
            bootstrapped_sessions=?self.dwallet_mpc_manager.mpc_sessions.keys().copied().collect::<Vec<_>>(),
            "Spawning dWallet MPC Service"
        );
        let mut loop_index = 0;
        loop {
            let mut events = vec![];

            // Load events from Sui every 30 seconds (300 * READ_INTERVAL_MS=100ms = 30,000ms = 30s).
            // Note: when we spawn, `loop_index == 0`, so we fetch uncompleted events on spawn.
            if loop_index % 300 == 0 {
                events = self.fetch_uncompleted_events().await;
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

                // This signifies a bug, we can't proceed before we fix it.
                break;
            }

            debug!("Running DWalletMPCService loop");
            self.sync_last_session_to_complete_in_current_epoch().await;
            let Ok(tables) = self.epoch_store.tables() else {
                warn!("failed to load DB tables from the epoch store");
                continue;
            };

            self.receive_completed_mpc_session_identifiers(false);

            // Receive **new** dWallet MPC events and save them in the local DB.
            match self.receive_new_sui_events() {
                Ok(new_events) => events.extend(new_events),
                Err(e) => {
                    error!(
                    error=?e,
                    "failed to receive dWallet MPC events");
                    continue;
                }
            };

            self.dwallet_mpc_manager
                .handle_sui_db_event_batch(events, &self.epoch_store);

            let mpc_msgs_iter = tables
                .dwallet_mpc_messages
                .safe_iter_with_bounds(Some(self.last_read_consensus_round + 1), None)
                .collect::<Result<Vec<_>, _>>();
            let mut mpc_messages = match mpc_msgs_iter {
                Ok(iter) => iter,
                Err(e) => {
                    error!(err=?e, "failed to load DWallet MPC messages from the local DB");
                    continue;
                }
            };

            // Sort the MPC messages by round in ascending order.
            mpc_messages.sort_by(|(round, _), (other_round, _)| round.cmp(other_round));

            for (round, messages) in mpc_messages {
                // Since we sorted, this assures this variable will be the
                // last read in this batch when we are done iterating.
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

    /// Receive all completed MPC sessions from the MPC Output Verifier over the
    /// `consensus_round_completed_sessions` channel.
    /// If the session exists, mark is as [`MPCSessionStatus::Finished`].
    /// Otherwise, create a new session with that status, to avoid re-running the computation for it.
    fn receive_completed_mpc_session_identifiers(&mut self, bootstrap: bool) {
        let mut completed_sessions = HashSet::new();
        loop {
            match self.consensus_round_completed_sessions_receiver.try_recv() {
                Err(mpsc::error::TryRecvError::Empty) => {
                    // No more completed sessions to report at the moment.
                    break;
                }
                Err(e) => {
                    error!(
                        authority=?self.epoch_store.name,
                        e=?e,
                        "error in reading completed session IDs"
                    );

                    break;
                }
                Ok(completed_session_identifier) => {
                    debug!(
                        validator=?self.epoch_store.name,
                        completed_session_identifier=?completed_session_identifier,
                        bootstrap=bootstrap,
                        "Received completed session identifier"
                    );
                    // There might be more completed sessions to report, so report this one and continue receiving (don't break).
                    completed_sessions.insert(completed_session_identifier);
                }
            }
        }

        debug!(
            validator=?self.epoch_store.name,
            completed_sessions=?completed_sessions,
            bootstrap=bootstrap,
            "Received completed session identifiers"
        );

        for session_identifier in completed_sessions {
            // If no session with SID `session_identifier` exist, create a new one.
            if !self
                .dwallet_mpc_manager
                .mpc_sessions
                .contains_key(&session_identifier)
            {
                self.dwallet_mpc_manager
                    .new_mpc_session(&session_identifier, None)
            }

            // Now this session is guaranteed to exist, so safe to `unwrap()`.
            let session = self
                .dwallet_mpc_manager
                .mpc_sessions
                .get_mut(&session_identifier)
                .unwrap();

            // Mark the session as completed, but *don't remove it from the map* (important!)
            session.clear_data();
            session.status = MPCSessionStatus::Finished;
        }
    }
}
