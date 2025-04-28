//! This module contains the DWalletMPCService struct.
//! It is responsible to read DWallet MPC messages from the
//! local DB every [`READ_INTERVAL_MS`] seconds
//! and forward them to the [`DWalletMPCManager`].

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::{ConsensusAdapter, SubmitToConsensus};
use crate::dwallet_mpc::mpc_manager::{DWalletMPCDBMessage, DWalletMPCManager};
use crate::dwallet_mpc::session_info_from_event;
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, MPCSessionStatus};
use ika_config::NodeConfig;
use ika_sui_client::{SuiBridgeClient, SuiClient};
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::error::IkaResult;
use ika_types::messages_consensus::ConsensusTransaction;
use ika_types::messages_dwallet_mpc::DBSuiEvent;
use ika_types::messages_dwallet_mpc::DWalletMPCEvent;
use ika_types::sui::epoch_start_system::EpochStartSystemTrait;
use ika_types::sui::DWalletCoordinatorInner;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use sui_json_rpc_types::SuiEvent;
use sui_types::base_types::EpochId;
use sui_types::event::EventID;
use sui_types::messages_consensus::Round;
use tokio::sync::watch::error::RecvError;
use tokio::sync::watch::Receiver;
use tokio::sync::{watch, Notify};
use tokio::task::yield_now;
use tokio::time;
use tracing::{error, info, warn};
use typed_store::Map;

const READ_INTERVAL_MS: u64 = 100;

pub struct DWalletMPCService {
    last_read_consensus_round: Round,
    read_messages: usize,
    epoch_store: Arc<AuthorityPerEpochStore>,
    epoch_id: EpochId,
    notify: Arc<Notify>,
    sui_client: Arc<SuiBridgeClient>,
    dwallet_mpc_manager: DWalletMPCManager,
    pub exit: Receiver<()>,
}

impl DWalletMPCService {
    pub async fn new(
        epoch_store: Arc<AuthorityPerEpochStore>,
        exit: Receiver<()>,
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        node_config: NodeConfig,
        sui_client: Arc<SuiBridgeClient>,
    ) -> Self {
        let dwallet_mpc_manager = DWalletMPCManager::must_create_dwallet_mpc_manager(
            consensus_adapter.clone(),
            epoch_store.clone(),
            node_config,
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
            exit,
        }
    }

    async fn update_last_session_to_complete_in_current_epoch(&mut self) {
        let system_inner = self.sui_client.must_get_system_inner_object().await;
        if let Some(dwallet_coordinator_id) = system_inner
            .into_init_version_for_tooling()
            .dwallet_2pc_mpc_secp256k1_id
        {
            let coordinator_state = self
                .sui_client
                .must_get_dwallet_coordinator_inner(dwallet_coordinator_id)
                .await;
            match coordinator_state {
                DWalletCoordinatorInner::V1(inner_state) => {
                    self.dwallet_mpc_manager
                        .update_last_session_to_complete_in_current_epoch(
                            inner_state.last_session_to_complete_in_current_epoch,
                        );
                }
            }
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
                            session_id=?session_info.session_id,
                            sequence_number=?session_info.sequence_number,
                            is_immediate=?session_info.is_immediate,
                            mpc_round=?session_info.mpc_round,
                            "Successfully processed missed event from Sui"
                        );
                    }
                    Ok(None) => {
                        error!("Failed to extract session info from missed event");
                    }
                    Err(e) => {
                        error!("Error processing a missed event: {}", e);
                    }
                }
            }
            return;
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
                    error!("DWalletMPCService exit signal received");
                    break;
                }
                Err(err) => {
                    error!("Failed to check DWalletMPCService exit signal: {:?}", err);
                    break;
                }
                Ok(false) => (),
            };
            tokio::time::sleep(Duration::from_millis(READ_INTERVAL_MS)).await;

            if self.dwallet_mpc_manager.recognized_self_as_malicious {
                error!(
                    authority=?self.epoch_store.name,
                    "node has identified itself as malicious and is no longer participating in MPC protocols"
                );
                tokio::time::sleep(Duration::from_secs(120)).await;
                continue;
            }

            info!("Running DWalletMPCService loop");
            self.dwallet_mpc_manager
                .cryptographic_computations_orchestrator
                .check_for_completed_computations();
            self.update_last_session_to_complete_in_current_epoch()
                .await;
            let Ok(tables) = self.epoch_store.tables() else {
                error!("Failed to load DB tables from epoch store");
                continue;
            };
            let Ok(completed_sessions) = self
                .epoch_store
                .load_dwallet_mpc_completed_sessions_from_round(self.last_read_consensus_round + 1)
                .await
            else {
                error!("Failed to load DWallet MPC events from the local DB");
                continue;
            };
            for session_id in completed_sessions {
                self.dwallet_mpc_manager
                    .mpc_sessions
                    .get_mut(&session_id)
                    .map(|session| {
                        session.clear_data();
                        session.status = MPCSessionStatus::Finished;
                    });
            }
            let Ok(events_from_sui) = self
                .epoch_store
                .load_dwallet_mpc_events_from_round(self.last_read_consensus_round + 1)
                .await
            else {
                error!("Failed to load DWallet MPC events from the local DB");
                continue;
            };
            for event in events_from_sui {
                self.dwallet_mpc_manager
                    .handle_dwallet_db_event(event)
                    .await;
            }
            let mpc_msgs_iter = tables
                .dwallet_mpc_messages
                .iter_with_bounds(Some(self.last_read_consensus_round + 1), None);
            let mut new_messages = vec![];
            for (round, messages) in mpc_msgs_iter {
                self.last_read_consensus_round = round;
                new_messages.extend(messages);
            }
            for message in new_messages {
                self.dwallet_mpc_manager
                    .handle_dwallet_db_message(message)
                    .await;
            }
            self.dwallet_mpc_manager
                .handle_dwallet_db_message(DWalletMPCDBMessage::PerformCryptographicComputations)
                .await;
        }
    }
}
