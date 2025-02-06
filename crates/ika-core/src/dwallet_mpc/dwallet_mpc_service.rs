//! This module contains the DWalletMPCService struct.
//! It is responsible to read DWallet MPC messages from the
//! local DB every [`READ_INTERVAL_MS`] seconds
//! and forward them to the [`crate::dwallet_mpc::mpc_manager::DWalletMPCManager`].

use std::collections::HashMap;
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::mpc_manager::DWalletMPCDBMessage;
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, MPCSessionStatus};
use std::sync::Arc;
use sui_types::base_types::EpochId;
use sui_types::event::EventID;
use sui_types::messages_consensus::Round;
use tokio::sync::watch::Receiver;
use tokio::sync::{watch, Notify};
use tracing::error;
use typed_store::Map;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::DWalletMPCEventMessage;
use crate::dwallet_mpc::{authority_name_to_party_id, session_info_from_event};

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
    /// [`crate::dwallet_mpc::mpc_manager::DWalletMPCManager`] for processing.
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

            let Ok(events) = self.read_events()
            else {
                error!("Failed to load DWallet MPC events from the local DB");
                continue;
            };
            for (id, event) in events {
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
        }
    }

    fn read_events(&self) -> DwalletMPCResult<HashMap<EventID, DWalletMPCEventMessage>> {
        let key_version = self.epoch_store
            .dwallet_mpc_network_keys
            .get()
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .key_version(DWalletMPCNetworkKeyScheme::Secp256k1)
            .unwrap_or_default();
        let pending_events = self.epoch_store.perpetual_tables.get_all_pending_events();
        let party_id = authority_name_to_party_id(&self.epoch_store.name, &self.epoch_store)?;
        let dwallet_mpc_new_events = pending_events
            .iter()
            .map(|(id, event)| {
                let session_info =
                    session_info_from_event(event.clone(), party_id, Some(key_version))
                        .map_err(|e| DwalletMPCError::NonMPCEvent(e.to_string()))?
                        .ok_or(DwalletMPCError::NonMPCEvent(
                            "Failed to craete session info from event".to_string(),
                        ))?;
                Ok((*id, DWalletMPCEventMessage {
                    event: event.clone(),
                    session_info,
                }))
            })
            .collect::<DwalletMPCResult<_>>()?;
        Ok(dwallet_mpc_new_events)

        // output.set_dwallet_mpc_round_events(dwallet_mpc_new_events);
        // let pending_event_ids = pending_events.keys().cloned().collect::<Vec<_>>();
        // self.perpetual_tables
        //     .remove_pending_events(&pending_event_ids)?;
    }
}
