//! This module contains the DWalletMPCService struct.
//! It is responsible to read DWallet MPC messages from the
//! local DB every [`READ_INTERVAL_MS`] seconds
//! and forward them to the [`DWalletMPCManager`].

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::mpc_manager::{DWalletMPCDBMessage, DWalletMPCManager};
use crate::dwallet_mpc::session_info_from_event;
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, MPCSessionStatus};
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::error::IkaResult;
use ika_types::messages_dwallet_mpc::DBSuiEvent;
use ika_types::messages_dwallet_mpc::DWalletMPCEvent;
use std::collections::HashMap;
use std::sync::Arc;
use sui_json_rpc_types::SuiEvent;
use sui_types::base_types::EpochId;
use sui_types::event::EventID;
use sui_types::messages_consensus::Round;
use tokio::sync::watch::Receiver;
use tokio::sync::{watch, Notify};
use tokio::task::yield_now;
use tracing::{error, info, warn};
use typed_store::Map;

const READ_INTERVAL_MS: u64 = 100;

pub struct DWalletMPCService {
    last_read_consensus_round: Round,
    read_messages: usize,
    epoch_store: Arc<AuthorityPerEpochStore>,
    epoch_id: EpochId,
    notify: Arc<Notify>,
    pub exit: Receiver<()>,
}

impl DWalletMPCService {
    pub fn new(epoch_store: Arc<AuthorityPerEpochStore>, exit: Receiver<()>) -> Self {
        Self {
            last_read_consensus_round: 0,
            read_messages: 0,
            epoch_store: epoch_store.clone(),
            epoch_id: epoch_store.epoch(),
            notify: Arc::new(Notify::new()),
            exit,
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
        loop {
            match self.exit.has_changed() {
                Ok(true) | Err(_) => {
                    break;
                }
                Ok(false) => (),
            };
            tokio::time::sleep(tokio::time::Duration::from_millis(READ_INTERVAL_MS)).await;
            if let Err(e) = self.read_events().await {
                error!("failed to handle dWallet MPC events: {}", e);
            }
            let mut manager = self.epoch_store.get_dwallet_mpc_manager().await;
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
                manager.mpc_sessions.get_mut(&session_id).map(|session| {
                    session.status = MPCSessionStatus::Finished;
                });
            }
            let Ok(events) = self
                .epoch_store
                .load_dwallet_mpc_events_from_round(self.last_read_consensus_round + 1)
                .await
            else {
                error!("Failed to load DWallet MPC events from the local DB");
                continue;
            };
            for event in events {
                manager.handle_dwallet_db_event(event);
            }
            let new_dwallet_messages_iter = tables
                .dwallet_mpc_messages
                .iter_with_bounds(Some(self.last_read_consensus_round + 1), None);
            let mut new_messages = vec![];
            for (round, messages) in new_dwallet_messages_iter {
                self.last_read_consensus_round = round;
                new_messages.extend(messages);
            }
            for message in new_messages {
                manager.handle_dwallet_db_message(message).await;
            }
            manager
                .handle_dwallet_db_message(DWalletMPCDBMessage::PerformCryptographicComputations)
                .await;
            drop(manager);
        }
    }

    async fn read_events(&mut self) -> IkaResult<()> {
        let pending_events = self.epoch_store.perpetual_tables.get_all_pending_events();
        let events: Vec<DWalletMPCEvent> = pending_events
            .iter()
            .filter_map(|(id, event)| {
                let Ok(event) = bcs::from_bytes::<DBSuiEvent>(event) else {
                    return None;
                };
                let Ok(Some(session_info)) =
                    session_info_from_event(event.clone(), &self.epoch_store.packages_config)
                else {
                    return None;
                };
                info!(mpc_protocol=?session_info.mpc_round,
                    session_id=?session_info.session_id
                    "Received start event for session");
                let event = DWalletMPCEvent {
                    event,
                    session_info,
                };
                Some(event)
            })
            .collect();

        let mut round_events = self.epoch_store.dwallet_mpc_round_events.lock().await;
        round_events.extend(events.clone());
        self.epoch_store
            .perpetual_tables
            .remove_pending_events(&pending_events.keys().cloned().collect::<Vec<EventID>>())?;
        Ok(())
    }
}
