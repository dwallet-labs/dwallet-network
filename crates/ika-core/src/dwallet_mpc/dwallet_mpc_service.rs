//! This module contains the DWalletMPCService struct.
//! It is responsible to read DWallet MPC messages from the
//! local DB every [`READ_INTERVAL_MS`] seconds
//! and forward them to the [`DWalletMPCManager`].

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::mpc_manager::{DWalletMPCDBMessage, DWalletMPCManager};
use crate::dwallet_mpc::mpc_session::session_info_from_event;
use dwallet_mpc_types::dwallet_mpc::{MPCSessionStatus, NetworkDecryptionKeyPublicData};
use ika_config::NodeConfig;
use ika_sui_client::SuiConnectorClient;
use ika_types::committee::Committee;
use ika_types::error::{IkaError, IkaResult};
use ika_types::messages_dwallet_mpc::{DBSuiEvent, DWalletMPCEvent, SessionIdentifier};
use ika_types::sui::{DWalletCoordinatorInner, SystemInner};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use sui_json_rpc_types::SuiEvent;
use sui_types::base_types::ObjectID;
use sui_types::messages_consensus::Round;
use tokio::sync::broadcast::error::TryRecvError;
use tokio::sync::watch::Receiver;
use tokio::sync::{mpsc, Notify};
use tracing::{debug, error, info, warn};
use typed_store::Map;

const READ_INTERVAL_MS: u64 = 100;

pub struct DWalletMPCService {
    last_read_consensus_round: Round,
    #[allow(dead_code)]
    read_messages: usize,
    epoch_store: Arc<AuthorityPerEpochStore>,
    #[allow(dead_code)]
    notify: Arc<Notify>,
    sui_client: Arc<SuiConnectorClient>,
    dwallet_mpc_manager: DWalletMPCManager,
    pub exit: Receiver<()>,
    pub network_keys_receiver: Receiver<Arc<HashMap<ObjectID, NetworkDecryptionKeyPublicData>>>,
    consensus_round_completed_sessions_receiver: mpsc::UnboundedReceiver<SessionIdentifier>,
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
        consensus_round_completed_sessions_receiver: mpsc::UnboundedReceiver<SessionIdentifier>,
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
            notify: Arc::new(Notify::new()),
            sui_client: sui_client.clone(),
            dwallet_mpc_manager,
            network_keys_receiver,
            new_events_receiver,
            consensus_round_completed_sessions_receiver,
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

    /// Proactively pull uncompleted events from the Sui network.
    /// We do that to assure we don't miss any events.
    /// These events might be from a different Epoch, not necessarily the current one.
    async fn fetch_uncompleted_events(&mut self) -> Vec<DWalletMPCEvent> {
        let epoch_store = self.epoch_store.clone();
        loop {
            let Ok(events) = self
                .sui_client
                .pull_dwallet_mpc_uncompleted_events(epoch_store.epoch())
                .await
            else {
                error!("failed to fetch missed dWallet MPC events from Sui");
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            };

            let events = events
                .into_iter()
                .flat_map(|event| {
                    match session_info_from_event(event.clone(), &epoch_store.packages_config) {
                        Ok(Some(session_info)) => {
                            let event = DWalletMPCEvent {
                                event,
                                session_info: session_info.clone(),
                                override_epoch_check: true,
                            };

                            debug!(
                                session_identifier=?session_info.session_identifier,
                                session_type=?session_info.session_type,
                                mpc_round=?session_info.mpc_round,
                                "Fetched uncompleted event from Sui"
                            );

                            Some(event)
                        }
                        Ok(None) => {
                            warn!(
                                "Received an event that does not trigger the start of an MPC flow"
                            );

                            None
                        }
                        Err(e) => {
                            error!(
                                erorr=?e,
                                "error while processing a missed event"
                            );

                            None
                        }
                    }
                })
                .collect();

            return events;
        }
    }

    async fn update_network_keys(&mut self) {
        match self.network_keys_receiver.has_changed() {
            Ok(has_changed) => {
                if has_changed {
                    let new_keys = self.network_keys_receiver.borrow_and_update();
                    for (key_id, key_data) in new_keys.iter() {
                        info!(
                            "Updating (decrypting new shares) network key for key_id: {:?}",
                            key_id
                        );
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
        let mut loop_index = 0;
        loop {
            let mut events = vec![];

            // Load events from Sui every 5 minutes (3000*100ms = 300,000ms = 300s = 5m).
            // Note: when we spawn, `loop_index == 0` so we fetch uncompleted events on spawn.
            if loop_index % 3_000 == 0 {
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
                        // There might be more completed sessions to report, so report this one and continue receiving (don't break).
                        completed_sessions.insert(completed_session_identifier);
                    }
                }
            }

            // self.last_read_consensus_round is the current reading consesnus round, also add self.last_db_consesnus_round that before it we don't compute.
            // maybe we can get this last_db_consesnus_round from the dag from the db. highest_known_commit_at_startup maybe its not the consensus round tho
            // read if the sui consensus syncs these values somehow
            for session_identifier in completed_sessions {
                // If no session with SID `session_identifier` exist, create a new one.
                if !self.dwallet_mpc_manager.mpc_sessions.contains_key(&session_identifier) {
                    self.dwallet_mpc_manager.new_mpc_session(&session_identifier, None)
                }

                // Now this session is guaranteed to exist, so safe to `unwrap()`.
                let session = self.dwallet_mpc_manager.mpc_sessions.get_mut(&session_identifier).unwrap();

                // Mark the session as completed, but *don't remove it from the map* (important!)
                session.clear_data();
                session.status = MPCSessionStatus::Finished;
            }

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
                // TODO(Scaly): why is there a message `EndOfDelivery` and `PerformCryptographicComputations` ? why not call function?
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

        let events: Vec<DWalletMPCEvent> = pending_events
            .iter()
            .filter_map(|event| {
                let id = event.id;
                let event = DBSuiEvent {
                    type_: event.type_.clone(),
                    contents: event.bcs.clone().into_bytes(),
                };

                match session_info_from_event(event.clone(), &self.epoch_store.packages_config) {
                    Ok(Some(session_info)) => {
                        info!(
                            mpc_protocol=?session_info.mpc_round,
                            session_identifier=?session_info.session_identifier,
                            validator=?self.epoch_store.name,
                            id=format!("{:?}", id),
                            "Received start event for session"
                        );
                        let event = DWalletMPCEvent {
                            event,
                            session_info,
                            override_epoch_check: false,
                        };
                        Some(event)
                    }
                    Ok(None) => {
                        warn!(
                            event=?event,
                            id=format!("{:?}", id),
                            "Received an event that does not trigger the start of an MPC flow"
                        );
                        None
                    }
                    Err(e) => {
                        error!("error getting session info from event: {}", e);
                        None
                    }
                }
            })
            .collect();

        Ok(events)
    }
}
