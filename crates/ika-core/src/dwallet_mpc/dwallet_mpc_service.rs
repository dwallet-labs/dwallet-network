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
use crate::dwallet_mpc::crytographic_computation::ComputationId;
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::mpc_manager::DWalletMPCManager;
use crate::dwallet_mpc::mpc_outputs_verifier::OutputVerificationStatus;
use crate::dwallet_mpc::mpc_session::MPCEventData;
use crate::dwallet_mpc::party_ids_to_authority_names;
use dwallet_mpc_types::dwallet_mpc::{
    MPCMessage, MPCPrivateOutput, MPCSessionPublicOutput, MPCSessionStatus,
    SerializedWrappedMPCPublicOutput,
};
use ika_config::NodeConfig;
use ika_sui_client::SuiConnectorClient;
use ika_types::committee::Committee;
use ika_types::crypto::keccak256_digest;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::message::DWalletCheckpointMessageKind;
use ika_types::messages_consensus::ConsensusTransaction;
use ika_types::messages_dwallet_mpc::{
    DWalletNetworkEncryptionKeyData, MPCSessionRequest, SessionIdentifier,
};
use ika_types::sui::DWalletCoordinatorInner;
use itertools::izip;
use mpc::AsynchronousRoundResult;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use sui_json_rpc_types::SuiEvent;
use sui_types::base_types::ObjectID;
use sui_types::messages_consensus::Round;
use tokio::sync::watch::Receiver;
use tracing::{debug, error, info, warn};

const READ_INTERVAL_MS: u64 = 20;

pub struct DWalletMPCService {
    last_read_consensus_round: Option<Round>,
    pub(crate) epoch_store: Arc<AuthorityPerEpochStore>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
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
        let validator_name = epoch_store.name;
        let committee = epoch_store.committee().clone();
        let epoch_id = epoch_store.epoch();
        let packages_config = epoch_store.packages_config.clone();

        let network_dkg_third_round_delay = epoch_store
            .protocol_config()
            .network_dkg_third_round_delay();

        let decryption_key_reconfiguration_third_round_delay = epoch_store
            .protocol_config()
            .decryption_key_reconfiguration_third_round_delay();

        let dwallet_mpc_manager = DWalletMPCManager::new(
            validator_name,
            committee,
            epoch_id,
            packages_config,
            network_keys_receiver,
            next_epoch_committee_receiver,
            node_config,
            network_dkg_third_round_delay,
            decryption_key_reconfiguration_third_round_delay,
            dwallet_mpc_metrics,
        );

        Self {
            last_read_consensus_round: None,
            epoch_store: epoch_store.clone(),
            consensus_adapter,
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

            // Load events from Sui every 30 seconds (1500 * READ_INTERVAL_MS=20ms = 30,000ms = 30s).
            // Note: when we spawn, `loop_index == 0`, so we fetch uncompleted events on spawn.
            if loop_index % 1500 == 0 {
                events = self.fetch_uncompleted_events().await;
            }
            loop_index += 1;
            match self.exit.has_changed() {
                Ok(true) => {
                    warn!(
                        our_epoch_id=self.dwallet_mpc_manager.epoch_id,
                        authority=?self.epoch_store.name,
                        "DWalletMPCService exit signal received"
                    );
                    break;
                }
                Err(err) => {
                    warn!(
                        err=?err,
                        authority=?self.epoch_store.name,
                        our_epoch_id=self.dwallet_mpc_manager.epoch_id,
                        "DWalletMPCService exit channel was shutdown incorrectly"
                    );
                    break;
                }
                Ok(false) => (),
            };

            if self.dwallet_mpc_manager.recognized_self_as_malicious {
                error!(
                    authority=?self.epoch_store.name,
                    "the node has identified itself as malicious, breaking from MPC service loop"
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

            self.dwallet_mpc_manager.handle_sui_db_event_batch(events);

            if !self.process_consensus_rounds_from_storage() {
                // If we failed to process consensus rounds from storage
                // we should try again in the next iteration.
                info!(
                    last_read_consensus_round=?self.last_read_consensus_round,
                    "Retrying in the next iteration to process consensus rounds from storage"
                );

                continue;
            }

            let completed_computation_results = self
                .dwallet_mpc_manager
                .perform_cryptographic_computation()
                .await;

            self.handle_computation_results_and_submit_to_consensus(completed_computation_results)
                .await;

            tokio::time::sleep(Duration::from_millis(READ_INTERVAL_MS)).await;
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

        for (
            mpc_messages_iteration_result,
            mpc_outputs_iteration_result,
            verified_dwallet_checkpoint_messages_iteration_result,
        ) in zipped_consensus_rounds_iter
        {
            let Ok((mpc_messages_consensus_round, mpc_messages)) = mpc_messages_iteration_result
            else {
                error!("failed to load DWallet MPC messages from the local DB");

                return false;
            };
            let Ok((mpc_outputs_consensus_round, mpc_outputs)) = mpc_outputs_iteration_result
            else {
                error!("failed to load DWallet MPC outputs from the local DB");

                return false;
            };
            let Ok((
                verified_dwallet_checkpoint_messages_consensus_round,
                verified_dwallet_checkpoint_messages,
            )) = verified_dwallet_checkpoint_messages_iteration_result
            else {
                error!("Failed to load verified DWallet checkpoint messages from the local DB");

                return false;
            };
            if mpc_messages_consensus_round != mpc_outputs_consensus_round
                || mpc_messages_consensus_round
                    != verified_dwallet_checkpoint_messages_consensus_round
            {
                error!(
                        ?mpc_messages_consensus_round,
                        ?mpc_outputs_consensus_round,
                        ?verified_dwallet_checkpoint_messages_consensus_round,
                        "the consensus rounds of MPC messages, MPC outputs and checkpoint messages do not match"
                    );

                return false;
            }

            let consensus_round = mpc_messages_consensus_round;

            if self.last_read_consensus_round >= Some(consensus_round) {
                error!(
                    consensus_round,
                    last_read_consensus_round=?self.last_read_consensus_round,
                    "consensus round must be in a ascending order, should never happen"
                );

                return false;
            }

            // Let's start processing the MPC messages for the current round.
            self.dwallet_mpc_manager
                .handle_consensus_round_messages(consensus_round, mpc_messages);

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
                                authority=?self.epoch_store.name,
                                ?output_digest,
                                consensus_round,
                                ?session_identifier,
                                "MPC output is verified and reached quorum"
                            );
                        }
                        OutputVerificationStatus::Malicious => {
                            warn!(
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
                        authority=?self.epoch_store.name,
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

    async fn handle_computation_results_and_submit_to_consensus(
        &mut self,
        completed_computation_results: HashMap<
            ComputationId,
            DwalletMPCResult<
                mpc::AsynchronousRoundResult<
                    MPCMessage,
                    MPCPrivateOutput,
                    SerializedWrappedMPCPublicOutput,
                >,
            >,
        >,
    ) {
        let committee = self.epoch_store.committee().clone();
        let validator_name = &self.epoch_store.name;
        let party_id = self.dwallet_mpc_manager.party_id;

        for (computation_id, computation_result) in completed_computation_results {
            let session_identifier = computation_id.session_identifier;
            let mpc_round = computation_id.mpc_round;
            let consensus_adapter = self.consensus_adapter.clone();
            let epoch_store = self.epoch_store.clone();

            if let Some(session) = self
                .dwallet_mpc_manager
                .mpc_sessions
                .get(&session_identifier)
            {
                if session.status == MPCSessionStatus::Active {
                    if let Some(mpc_event_data) = session.mpc_event_data.clone() {
                        match computation_result {
                            Ok(AsynchronousRoundResult::Advance {
                                malicious_parties: _,
                                message,
                            }) => {
                                info!(
                                    ?session_identifier,
                                    validator=?validator_name,
                                    ?mpc_round,
                                    "Advanced MPC session"
                                );
                                let message = self.new_dwallet_mpc_message(
                                    session_identifier,
                                    mpc_round,
                                    message,
                                    mpc_event_data,
                                );

                                if let Err(err) = consensus_adapter
                                    .submit_to_consensus(&[message], &epoch_store)
                                    .await
                                {
                                    error!(
                                        ?session_identifier,
                                        validator=?validator_name,
                                        ?mpc_round,
                                        err=?err,
                                        "failed to submit an MPC message to consensus"
                                    );
                                }
                            }
                            Ok(AsynchronousRoundResult::Finalize {
                                malicious_parties,
                                private_output: _,
                                public_output,
                            }) => {
                                info!(
                                    ?session_identifier,
                                    validator=?validator_name,
                                    "Reached output for session"
                                );
                                let consensus_adapter = self.consensus_adapter.clone();
                                if !malicious_parties.is_empty() {
                                    let malicious_authorities = party_ids_to_authority_names(
                                        &malicious_parties,
                                        &committee,
                                    );

                                    error!(
                                        ?session_identifier,
                                            validator=?validator_name,
                                            ?malicious_parties,
                                            ?malicious_authorities,
                                        "malicious parties detected upon MPC session finalize",
                                    );

                                    self.dwallet_mpc_manager
                                        .record_malicious_actors(&malicious_authorities);
                                }

                                match bcs::to_bytes(&MPCSessionPublicOutput::CompletedSuccessfully(
                                    public_output.clone(),
                                )) {
                                    Ok(public_output) => {
                                        let consensus_message = self
                                            .new_dwallet_mpc_output_message(
                                                session_identifier,
                                                public_output,
                                                mpc_event_data,
                                            );

                                        if let Err(err) = consensus_adapter
                                            .submit_to_consensus(&[consensus_message], &epoch_store)
                                            .await
                                        {
                                            error!(
                                            ?session_identifier,
                                                    validator=?validator_name,
                                                    err=?err,
                                                    "failed to submit an MPC output message to consensus",
                                                );
                                        }
                                    }
                                    Err(e) => {
                                        error!(
                                            ?session_identifier,
                                            validator=?validator_name,
                                            error=?e,
                                            ?public_output,
                                            "failed to serialize an MPC output",
                                        );
                                    }
                                }
                            }
                            Err(DwalletMPCError::TWOPCMPCThresholdNotReached) => {
                                error!(
                                    err=?DwalletMPCError::TWOPCMPCThresholdNotReached,
                                        ?session_identifier,
                                    validator=?validator_name,
                                    mpc_round,
                                    party_id,
                                    "MPC session failed"
                                );

                                let consensus_round = computation_id.consensus_round.expect("consensus round must be set for the computation ID of a computation that got a threshold not reached error");
                                self.dwallet_mpc_manager.record_threshold_not_reached(
                                    consensus_round,
                                    computation_id.session_identifier,
                                )
                            }
                            Err(err) => {
                                error!(
                                        ?session_identifier,
                                    validator=?validator_name,
                                    ?mpc_round,
                                    party_id,
                                        error=?err,
                                    "failed to advance the MPC session, rejecting."
                                );

                                let consensus_adapter = self.consensus_adapter.clone();
                                match bcs::to_bytes(&MPCSessionPublicOutput::SessionFailed) {
                                    Ok(public_output) => {
                                        let consensus_message = self
                                            .new_dwallet_mpc_output_message(
                                                session_identifier,
                                                public_output,
                                                mpc_event_data,
                                            );

                                        if let Err(err) = consensus_adapter
                                            .submit_to_consensus(&[consensus_message], &epoch_store)
                                            .await
                                        {
                                            error!(
                            ?session_identifier,
                            validator=?validator_name,
                            error=?err,
                            "failed to submit an MPC SessionFailed message to consensus");
                                        }
                                    }
                                    Err(e) => {
                                        error!(
                                            ?session_identifier,
                                            validator=?validator_name,
                                            error=?e,
                                            "failed to serialize an MPCSessionPublicOutput::SessionFailed MPC output",
                                        );
                                    }
                                }
                            }
                        }
                    } else {
                        error!(
                            should_never_happen =? true,
                            ?session_identifier,
                            validator=?validator_name,
                            ?mpc_round,
                            "no mpc_event_data for a session for which a computation update was received"
                        );
                    }
                } else {
                    warn!(
                        ?session_identifier,
                        validator=?validator_name,
                        ?mpc_round,
                        "received a computation update for an non-active MPC session"
                    );
                }
            } else {
                error!(
                    should_never_happen =? true,
                    ?session_identifier,
                    validator=?validator_name,
                    ?mpc_round,
                    "failed to retrieve MPC session for which a computation update was received"
                );
            }
        }
    }

    /// Create a new consensus transaction with the message to be sent to the other MPC parties.
    /// Returns Error only if the epoch switched in the middle and was not available.
    fn new_dwallet_mpc_message(
        &self,
        session_identifier: SessionIdentifier,
        mpc_round: u64,
        message: MPCMessage,
        mpc_event_data: MPCEventData,
    ) -> ConsensusTransaction {
        let epoch = self.epoch_store.epoch();

        let session_request = MPCSessionRequest {
            session_type: mpc_event_data.session_type,
            request_input: mpc_event_data.request_input.clone(),
            epoch,
            session_identifier,
            session_sequence_number: mpc_event_data.session_sequence_number,
            requires_network_key_data: mpc_event_data.requires_network_key_data,
            requires_next_active_committee: mpc_event_data.requires_next_active_committee,
        };

        ConsensusTransaction::new_dwallet_mpc_message(
            self.epoch_store.name,
            message,
            session_identifier,
            mpc_round,
            session_request,
        )
    }

    /// Create a new consensus transaction with the flow result (output) to be
    /// sent to the other MPC parties.
    /// Errors if the epoch was switched in the middle and was not available.
    fn new_dwallet_mpc_output_message(
        &self,
        session_identifier: SessionIdentifier,
        output: Vec<u8>,
        mpc_event_data: MPCEventData,
    ) -> ConsensusTransaction {
        let epoch = self.epoch_store.epoch();

        ConsensusTransaction::new_dwallet_mpc_output(
            self.epoch_store.name,
            output,
            MPCSessionRequest {
                session_type: mpc_event_data.session_type,
                session_identifier,
                session_sequence_number: mpc_event_data.session_sequence_number,
                request_input: mpc_event_data.request_input.clone(),
                epoch,
                requires_network_key_data: mpc_event_data.requires_network_key_data,
                requires_next_active_committee: mpc_event_data.requires_next_active_committee,
            },
        )
    }
}
