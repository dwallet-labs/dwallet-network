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
use itertools::izip;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use sui_json_rpc_types::SuiEvent;
use sui_types::base_types::ObjectID;
use sui_types::messages_consensus::Round;
use tokio::sync::watch::Receiver;
use tracing::{debug, error, info, warn};

const READ_INTERVAL_MS: u64 = 100;

pub struct DWalletMPCService {
    last_read_consensus_round: Option<Round>,
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
        let dwallet_mpc_manager = DWalletMPCManager::new(
            consensus_adapter.clone(),
            epoch_store.clone(),
            network_keys_receiver,
            next_epoch_committee_receiver,
            node_config,
            dwallet_mpc_metrics,
        );

        Self {
            last_read_consensus_round: None,
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
        // Process all MPC completed session outputs we bootstrapped from storage
        // before starting execution, to avoid their computation.
        if !self.bootstrap_completed_sessions() {
            return;
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

            if !self.process_consensus_rounds_from_storage() {
                // If we failed to process consensus rounds from storage
                // we should try again in the next iteration.
                info!(
                    last_read_consensus_round=?self.last_read_consensus_round,
                    "Retrying in the next iteration to process consensus rounds from storage"
                );
                continue;
            }

            self.dwallet_mpc_manager
                .handle_dwallet_db_message(&DWalletMPCDBMessage::PerformCryptographicComputations);
        }
    }

    /// Bootstrap all completed MPC sessions from the local DB for current epoch.
    /// Return `true` if bootstrapping was successful, `false` otherwise.
    fn bootstrap_completed_sessions(&mut self) -> bool {
        let Ok(tables) = self.epoch_store.tables() else {
            warn!("failed to load DB tables from the epoch store");
            return false;
        };
        let bootstrapping_completed_sessions = tables.get_dwallet_mpc_completed_sessions_iter();
        let mut last_bootstrapped_consensus_round = None;
        for bootstrapping_completed_session in bootstrapping_completed_sessions {
            match bootstrapping_completed_session {
                Ok((round, completed_sessions)) => {
                    self.process_completed_mpc_session_identifiers(&completed_sessions);
                    last_bootstrapped_consensus_round = Some(round);
                }
                Err(e) => {
                    error!(
                        err=?e,
                        last_bootstrapped_consensus_round,
                        "failed to load all completed MPC sessions from the local DB during bootstrapping"
                    );
                    return false;
                }
            }
        }
        true
    }

    fn process_consensus_rounds_from_storage(&mut self) -> bool {
        let Ok(tables) = self.epoch_store.tables() else {
            warn!("failed to load DB tables from the epoch store");
            return false;
        };
        let next_consensus_round = self
            .last_read_consensus_round
            .map(|round| round + 1)
            .unwrap_or_default();
        let mpc_messages_iter = tables.get_dwallet_mpc_messages_iter(next_consensus_round);
        let mpc_outputs_iter = tables.get_dwallet_mpc_outputs_iter(next_consensus_round);
        let verified_dwallet_checkpoint_messages_iter =
            tables.get_verified_dwallet_checkpoint_messages_iter(next_consensus_round);

        let zipped_consensus_rounds_iter = izip!(
            mpc_messages_iter,
            mpc_outputs_iter,
            verified_dwallet_checkpoint_messages_iter
        );

        for (mpc_messages, mpc_outputs, verified_dwallet_checkpoint_messages) in
            zipped_consensus_rounds_iter
        {
            let Ok((mpc_messages_round, mpc_messages)) = mpc_messages else {
                error!("Failed to load DWallet MPC messages from the local DB");
                return false;
            };
            let Ok((mpc_outputs_round, mpc_outputs)) = mpc_outputs else {
                error!("Failed to load DWallet MPC outputs from the local DB");
                return false;
            };
            let Ok((
                verified_dwallet_checkpoint_messages_round,
                verified_dwallet_checkpoint_messages,
            )) = verified_dwallet_checkpoint_messages
            else {
                error!("Failed to load verified DWallet checkpoint messages from the local DB");
                return false;
            };
            if mpc_messages_round != mpc_outputs_round
                || mpc_messages_round != verified_dwallet_checkpoint_messages_round
            {
                error!(
                        ?mpc_messages_round,
                        ?mpc_outputs_round,
                        ?verified_dwallet_checkpoint_messages_round,
                        "The consensus rounds of MPC messages, MPC outputs and checkpoint messages do not match"
                    );
                return false;
            }

            let consensus_round = mpc_messages_round;

            if self.last_read_consensus_round >= Some(consensus_round) {
                error!(
                    consensus_round,
                    last_read_consensus_round=?self.last_read_consensus_round,
                    "Consensus round must be in a ascending order, should never happen"
                );
                return false;
            }

            // Let's start processing the MPC messages for the current round.

            // Since we sorted, this assures this variable will be the
            // last read in this batch when we are done iterating.
            for message in &mpc_messages {
                self.dwallet_mpc_manager.handle_dwallet_db_message(message);
            }
            self.dwallet_mpc_manager
                .handle_dwallet_db_message(&DWalletMPCDBMessage::EndOfDelivery);

            // Not let's move to process MPC outputs for the current round.

            let mut checkpoint_messages = vec![];
            let mut completed_sessions = vec![];
            for output in &mpc_outputs {
                let output_result = self.dwallet_mpc_manager.handle_dwallet_db_output(output);
                let session_identifier = output.session_request.session_identifier;
                match output_result {
                    Ok(output_result) => match output_result.result {
                        OutputVerificationStatus::FirstQuorumReached(m) => {
                            checkpoint_messages.extend(m);
                            completed_sessions.push(session_identifier);
                            let output_digest = keccak256_digest(&output.output);
                            info!(
                                ?output_digest,
                                consensus_round,
                                ?session_identifier,
                                "MPC output is verified and reached quorum"
                            );
                        }
                        OutputVerificationStatus::Malicious => {
                            debug!(
                                ?output,
                                consensus_round,
                                ?session_identifier,
                                "MPC output is marked as malicious, skipping it"
                            );
                        }
                        OutputVerificationStatus::NotEnoughVotes => {
                            debug!(
                                ?output,
                                consensus_round,
                                ?session_identifier,
                                "MPC output does not have enough votes, skipping it"
                            );
                        }
                        OutputVerificationStatus::AlreadyCommitted => {
                            debug!(
                                ?output,
                                consensus_round,
                                ?session_identifier,
                                "MPC output is already committed, skipping it"
                            );
                        }
                    },
                    Err(e) => {
                        error!(err=?e, ?output,"failed to load verify MPC output from the local DB");
                        return false;
                    }
                };
            }
            self.process_completed_mpc_session_identifiers(&completed_sessions);

            // Now we have the MPC outputs for the current round, we can
            // add messages from the consensus output such as EndOfPublish.
            checkpoint_messages.extend(verified_dwallet_checkpoint_messages);

            if !self.end_of_publish {
                let final_round = checkpoint_messages
                    .iter()
                    .last()
                    .is_some_and(|msg| matches!(msg, DWalletCheckpointMessageKind::EndOfPublish));
                if final_round {
                    self.end_of_publish = true;
                    info!(
                        epoch=?self.epoch_store.epoch(),
                        consensus_round,
                        "End of publish reached, no more dwallet checkpoints will be processed for this epoch"
                    );
                }
                let pending_checkpoint = PendingDWalletCheckpoint::V1(PendingDWalletCheckpointV1 {
                    messages: checkpoint_messages.clone(),
                    details: PendingDWalletCheckpointInfo {
                        checkpoint_height: consensus_round,
                    },
                });
                if let Err(e) = self
                    .epoch_store
                    .insert_pending_dwallet_checkpoint(pending_checkpoint)
                {
                    error!(
                            err=?e,
                            ?consensus_round,
                            ?checkpoint_messages,
                            "failed to insert pending checkpoint into the local DB"
                    );
                    return false;
                };
                debug!(
                    ?consensus_round,
                    "Notifying checkpoint service about new pending checkpoint(s)",
                );
                // Only after batch is written, notify checkpoint service to start building any new
                // pending checkpoints.
                if let Err(e) = self.dwallet_checkpoint_service.notify_checkpoint() {
                    error!(
                        err=?e,
                        ?consensus_round,
                        "failed to notify checkpoint service about new pending checkpoint(s)"
                    );
                    return false;
                }
            }

            if let Err(e) = self
                .epoch_store
                .insert_dwallet_mpc_completed_sessions(&consensus_round, &completed_sessions)
            {
                error!(
                    err=?e,
                    ?consensus_round,
                    ?completed_sessions,
                    "failed to insert completed MPC sessions into the local DB"
                );
            }
            self.last_read_consensus_round = Some(consensus_round);
        }
        true
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
