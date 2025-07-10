//! This module contains the DWalletMPCService struct.
//! It is responsible to read DWallet MPC messages from the
//! local DB every [`READ_INTERVAL_MS`] seconds
//! and forward them to the [`DWalletMPCManager`].

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_checkpoints::{
    DWalletCheckpointServiceNotify, PendingDWalletCheckpoint, PendingDWalletCheckpointInfo,
    PendingDWalletCheckpointV1,
};
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::mpc_manager::{DWalletMPCDBMessage, DWalletMPCManager};
use crate::dwallet_mpc::mpc_outputs_verifier::OutputVerificationStatus;
use dwallet_mpc_types::dwallet_mpc::MPCSessionStatus;
use ika_config::NodeConfig;
use ika_sui_client::SuiConnectorClient;
use ika_types::committee::Committee;
use ika_types::crypto::keccak256_digest;
use ika_types::message::DWalletCheckpointMessageKind;
use ika_types::messages_dwallet_mpc::{DWalletNetworkEncryptionKeyData, SessionIdentifier};
use ika_types::sui::DWalletCoordinatorInner;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use sui_json_rpc_types::SuiEvent;
use sui_types::base_types::ObjectID;
use sui_types::messages_consensus::Round;
use tokio::sync::watch::Receiver;
use tracing::{debug, error, info, warn};
use typed_store::Map;

const READ_INTERVAL_MS: u64 = 100;

pub struct DWalletMPCService {
    last_read_consensus_round: Round,
    pub(crate) epoch_store: Arc<AuthorityPerEpochStore>,
    pub(crate) sui_client: Arc<SuiConnectorClient>,
    dwallet_checkpoint_service: Arc<dyn DWalletCheckpointServiceNotify + Send + Sync>,
    dwallet_mpc_manager: DWalletMPCManager,
    pub exit: Receiver<()>,
    pub new_events_receiver: tokio::sync::broadcast::Receiver<Vec<SuiEvent>>,
    end_of_publish: bool,
}

impl DWalletMPCService {
    pub fn new(
        epoch_store: Arc<AuthorityPerEpochStore>,
        exit: Receiver<()>,
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        node_config: NodeConfig,
        sui_client: Arc<SuiConnectorClient>,
        dwallet_checkpoint_service: Arc<dyn DWalletCheckpointServiceNotify + Send + Sync>,
        network_keys_receiver: Receiver<Arc<HashMap<ObjectID, DWalletNetworkEncryptionKeyData>>>,
        new_events_receiver: tokio::sync::broadcast::Receiver<Vec<SuiEvent>>,
        next_epoch_committee_receiver: Receiver<Committee>,
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
            dwallet_checkpoint_service,
            dwallet_mpc_manager,
            new_events_receiver,
            exit,
            end_of_publish: false,
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
        let bootstrapping_completed_sessions =
            self.epoch_store.get_all_dwallet_mpc_completed_sessions();
        match bootstrapping_completed_sessions {
            Ok(completed_sessions) => {
                self.process_completed_mpc_session_identifiers(&completed_sessions);
            }
            Err(e) => {
                error!(
                    err=?e,
                    "failed to load all completed MPC sessions from the local DB during bootstrapping"
                );
                return;
            }
        }

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

            let mpc_messages_iter = tables
                .dwallet_mpc_messages
                .safe_iter_with_bounds(Some(self.last_read_consensus_round + 1), None)
                .collect::<Result<Vec<_>, _>>();
            let mut mpc_messages = match mpc_messages_iter {
                Ok(iter) => iter,
                Err(e) => {
                    error!(err=?e, "failed to load DWallet MPC messages from the local DB");
                    continue;
                }
            };

            let mpc_outputs_iter = tables
                .dwallet_mpc_outputs
                .safe_iter_with_bounds(Some(self.last_read_consensus_round + 1), None)
                .collect::<Result<Vec<_>, _>>();
            let mut mpc_outputs = match mpc_outputs_iter {
                Ok(iter) => iter,
                Err(e) => {
                    error!(err=?e, "failed to load DWallet MPC outputs from the local DB");
                    continue;
                }
            };

            // Sort the MPC messages by round in ascending order.
            mpc_messages.sort_by(|(round, _), (other_round, _)| round.cmp(other_round));

            // Sort the MPC outputs by round in ascending order.
            mpc_outputs.sort_by(|(round, _), (other_round, _)| round.cmp(other_round));

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

            for (round, outputs) in mpc_outputs {
                let mut messages = vec![];
                let mut completed_sessions = vec![];
                for output in outputs {
                    let output_result = self
                        .dwallet_mpc_manager
                        .handle_dwallet_db_output(&output)
                        .await;
                    let session_identifier = output.session_request.session_identifier;
                    match output_result {
                        Ok(output_result) => match output_result.result {
                            OutputVerificationStatus::FirstQuorumReached(m) => {
                                messages.extend(m);
                                completed_sessions.push(session_identifier);
                                let output_digest = keccak256_digest(&output.output);
                                info!(
                                    ?output_digest,
                                    round,
                                    ?session_identifier,
                                    "MPC output is verified and reached quorum"
                                );
                            }
                            OutputVerificationStatus::Malicious => {
                                debug!(
                                    ?output,
                                    round,
                                    ?session_identifier,
                                    "MPC output is marked as malicious, skipping it"
                                );
                            }
                            OutputVerificationStatus::NotEnoughVotes => {
                                debug!(
                                    ?output,
                                    round,
                                    ?session_identifier,
                                    "MPC output does not have enough votes, skipping it"
                                );
                            }
                            OutputVerificationStatus::AlreadyCommitted => {
                                debug!(
                                    ?output,
                                    round,
                                    ?session_identifier,
                                    "MPC output is already committed, skipping it"
                                );
                            }
                        },
                        Err(e) => {
                            error!(err=?e, ?output,"failed to load verify MPC output from the local DB");
                        }
                    };
                }
                self.process_completed_mpc_session_identifiers(&completed_sessions);
                // Now we have the MPC outputs for the current round, we can
                // add messages from the consensus output such as EndOfPublish.
                match tables.get_verified_dwallet_checkpoint_messages(round) {
                    Ok(Some(m)) => {
                        messages.extend(m);
                    }
                    Ok(None) => {
                        error!(round, "No verified dwallet checkpoint messages found for round, this is unexpected.");
                    }
                    Err(e) => {
                        error!(err=?e, round, "Failed to load verified dwallet checkpoint messages from the local DB");
                    }
                }

                if !self.end_of_publish {
                    let final_round = messages.iter().last().is_some_and(|msg| {
                        matches!(msg, DWalletCheckpointMessageKind::EndOfPublish)
                    });
                    if final_round {
                        self.end_of_publish = true;
                        info!(
                            epoch=?self.epoch_store.epoch(),
                            round,
                            "End of publish reached, no more dwallet checkpoints will be processed for this epoch"
                        );
                    }
                    let pending_checkpoint =
                        PendingDWalletCheckpoint::V1(PendingDWalletCheckpointV1 {
                            messages: messages.clone(),
                            details: PendingDWalletCheckpointInfo {
                                checkpoint_height: round,
                            },
                        });
                    if let Err(e) = self
                        .epoch_store
                        .insert_pending_dwallet_checkpoint(pending_checkpoint)
                    {
                        error!(
                            err=?e,
                            ?round,
                            ?messages,
                            "failed to insert pending checkpoint into the local DB"
                        );
                    };
                    debug!(
                        ?round,
                        "Notifying checkpoint service about new pending checkpoint(s)",
                    );
                    // Only after batch is written, notify checkpoint service to start building any new
                    // pending checkpoints.
                    if let Err(e) = self.dwallet_checkpoint_service.notify_checkpoint() {
                        error!(
                            err=?e,
                            ?round,
                            "failed to notify checkpoint service about new pending checkpoint(s)"
                        );
                    }
                }

                if let Err(e) = self
                    .epoch_store
                    .insert_dwallet_mpc_completed_sessions(&round, &completed_sessions)
                {
                    error!(
                        err=?e,
                        ?round,
                        ?completed_sessions,
                        "failed to insert completed MPC sessions into the local DB"
                    );
                }
            }

            self.dwallet_mpc_manager
                .handle_dwallet_db_message(DWalletMPCDBMessage::PerformCryptographicComputations)
                .await;
        }
    }

    /// Process completed MPC sessions from the MPC Output Verifier or from the local DB.
    /// If the session exists, mark is as [`MPCSessionStatus::Finished`].
    /// Otherwise, create a new session with that status, to avoid re-running the computation for it.
    fn process_completed_mpc_session_identifiers(
        &mut self,
        completed_sessions: &Vec<SessionIdentifier>,
    ) {
        debug!(
            validator=?self.epoch_store.name,
            completed_sessions=?completed_sessions,
            "Process completed session identifiers"
        );

        for session_identifier in completed_sessions {
            // If no session with SID `session_identifier` exist, create a new one.
            if !self
                .dwallet_mpc_manager
                .mpc_sessions
                .contains_key(session_identifier)
            {
                self.dwallet_mpc_manager
                    .new_mpc_session(session_identifier, None)
            }

            // Now this session is guaranteed to exist, so safe to `unwrap()`.
            let session = self
                .dwallet_mpc_manager
                .mpc_sessions
                .get_mut(session_identifier)
                .unwrap();

            // Mark the session as completed, but *don't remove it from the map* (important!)
            session.clear_data();
            session.status = MPCSessionStatus::Finished;
        }
    }
}
