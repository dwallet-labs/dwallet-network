//! This module contains the DWalletMPCService struct.
//! It is responsible to read DWallet MPC messages from the
//! local DB every [`READ_INTERVAL_MS`] seconds
//! and forward them to the [`DWalletMPCManager`].

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::crytographic_computation::ComputationId;
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::mpc_manager::DWalletMPCManager;
use crate::dwallet_mpc::mpc_session::MPCEventData;
use crate::dwallet_mpc::party_ids_to_authority_names;
use dwallet_mpc_types::dwallet_mpc::{
    MPCMessage, MPCPrivateOutput, MPCSessionPublicOutput, MPCSessionStatus,
    SerializedWrappedMPCPublicOutput,
};
use ika_config::NodeConfig;
use ika_sui_client::SuiConnectorClient;
use ika_types::committee::Committee;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_consensus::ConsensusTransaction;
use ika_types::messages_dwallet_mpc::{
    DWalletNetworkEncryptionKeyData, MPCSessionRequest, SessionIdentifier,
};
use ika_types::sui::DWalletCoordinatorInner;
use mpc::AsynchronousRoundResult;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use sui_json_rpc_types::SuiEvent;
use sui_types::base_types::ObjectID;
use sui_types::messages_consensus::Round;
use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tokio::sync::watch::Receiver;
use tracing::{debug, error, info, warn};
use typed_store::Map;

const READ_INTERVAL_MS: u64 = 100;

pub struct DWalletMPCService {
    last_read_consensus_round: Round,
    pub(crate) epoch_store: Arc<AuthorityPerEpochStore>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
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
        let validator_name = epoch_store.name;
        let committee = epoch_store.committee().clone();
        let epoch_id = epoch_store.epoch();

        let network_dkg_third_round_delay = epoch_store
            .protocol_config()
            .network_dkg_third_round_delay() as usize;

        let decryption_key_reconfiguration_third_round_delay = epoch_store
            .protocol_config()
            .decryption_key_reconfiguration_third_round_delay()
            as usize;

        let dwallet_mpc_manager = DWalletMPCManager::must_create_dwallet_mpc_manager(
            validator_name,
            committee,
            epoch_id,
            network_keys_receiver,
            next_epoch_committee_receiver,
            node_config,
            network_dkg_third_round_delay,
            decryption_key_reconfiguration_third_round_delay,
            dwallet_mpc_metrics,
        );

        Self {
            last_read_consensus_round: 0,
            epoch_store: epoch_store.clone(),
            consensus_adapter,
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

            for (consensus_round, messages) in mpc_messages {
                // Since we sorted, this assures this variable will be the last read in this batch when we are done iterating.
                self.last_read_consensus_round = consensus_round;

                self.dwallet_mpc_manager
                    .handle_consensus_round_messages(consensus_round, messages);
            }

            let completed_computation_results = self
                .dwallet_mpc_manager
                .perform_cryptographic_computation()
                .await;

            self.handle_and_broadcast_computation_results(completed_computation_results)
                .await;

            tokio::time::sleep(Duration::from_millis(READ_INTERVAL_MS)).await;
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

    async fn handle_and_broadcast_computation_results(
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
            let mpc_event_data = self.dwallet_mpc_manager.mpc_sessions.get(&session_identifier).and_then(|session| session.mpc_event_data.clone()).expect("mpc_event_data must be set for a session for which we got a completed computation update");

            match computation_result {
                Ok(AsynchronousRoundResult::Advance {
                    malicious_parties: _, // TODO(Scaly): this will no longer be
                    message,
                }) => {
                    // TODO(Scaly): actually, there is still some ugly link here between native and mpc computations.
                    // I don't think we even need a Session for native computations, probably we do need the identifier tho. Maybe rename it `ComputationId`?

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
                        let malicious_authorities =
                            party_ids_to_authority_names(&malicious_parties, &committee);

                        error!(
                            ?session_identifier,
                                validator=?validator_name,
                                ?malicious_parties,
                                ?malicious_authorities,
                            "Malicious parties detected upon MPC session finalize",
                        );

                        self.dwallet_mpc_manager
                            .record_malicious_actors(&malicious_authorities);
                    }
                    let consensus_message = self.new_dwallet_mpc_output_message(
                        session_identifier,
                        MPCSessionPublicOutput::CompletedSuccessfully(public_output.clone()),
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
                    let consensus_message = self.new_dwallet_mpc_output_message(
                        session_identifier,
                        MPCSessionPublicOutput::SessionFailed,
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
            }
        }
    }

    /// Create a new consensus transaction with the message to be sent to the other MPC parties.
    /// Returns Error only if the epoch switched in the middle and was not available.
    fn new_dwallet_mpc_message(
        &self,
        session_identifier: SessionIdentifier,
        mpc_round: usize,
        message: MPCMessage,
        mpc_event_data: MPCEventData,
    ) -> ConsensusTransaction {
        let epoch = self.epoch_store.epoch();

        let session_request = MPCSessionRequest {
            session_type: mpc_event_data.session_type.clone(),
            request_input: mpc_event_data.request_input.clone(),
            epoch,
            session_identifier,
            session_sequence_number: mpc_event_data.session_sequence_number,
            requires_network_key_data: mpc_event_data.requires_network_key_data,
            requires_next_active_committee: mpc_event_data.requires_next_active_committee,
        };

        ConsensusTransaction::new_dwallet_mpc_message(
            self.epoch_store.name.clone(),
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
        output: MPCSessionPublicOutput,
        mpc_event_data: MPCEventData,
    ) -> ConsensusTransaction {
        let epoch = self.epoch_store.epoch();

        // TODO(Scaly): what to do with serialization error?
        let output = bcs::to_bytes(&output).expect("serialization error");
        ConsensusTransaction::new_dwallet_mpc_output(
            self.epoch_store.name.clone(),
            output,
            MPCSessionRequest {
                session_type: mpc_event_data.session_type.clone(),
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
