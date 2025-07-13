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
use crate::dwallet_mpc::mpc_session::MPCEventData;
use crate::dwallet_mpc::party_ids_to_authority_names;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCMessage, MPCPrivateOutput, MPCSessionStatus,
    SerializedWrappedMPCPublicOutput,
};
use ika_config::NodeConfig;
use ika_sui_client::SuiConnectorClient;
use ika_types::committee::Committee;
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::message::{
    DKGFirstRoundOutput, DKGSecondRoundOutput, DWalletCheckpointMessageKind,
    DWalletImportedKeyVerificationOutput, EncryptedUserShareOutput, MPCNetworkDKGOutput,
    MPCNetworkReconfigurationOutput, MakeDWalletUserSecretKeySharesPublicOutput,
    PartialSignatureVerificationOutput, PresignOutput, SignOutput,
};
use ika_types::messages_consensus::ConsensusTransaction;
use ika_types::messages_dwallet_mpc::{
    DWalletNetworkEncryptionKeyData, MPCRequestInput, SessionIdentifier,
};
use ika_types::sui::DWalletCoordinatorInner;
use itertools::{izip, Itertools};
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
const FIVE_KILO_BYTES: usize = 5 * 1024;

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
        self.bootstrap_computation_completed_sessions();

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

    /// Bootstrap all computation completed MPC sessions from the local DB for current epoch.
    /// This is used to avoid re-running computations for sessions.
    /// Panics if the DB tables cannot be loaded.
    fn bootstrap_computation_completed_sessions(&mut self) {
        let Ok(tables) = self.epoch_store.tables() else {
            error!("failed to load DB tables from the epoch store");
            panic!("failed to load DB tables from the epoch store");
        };
        let bootstrapping_computation_completed_sessions =
            tables.get_dwallet_mpc_computation_completed_sessions_iter();

        let mut last_bootstrapped_consensus_round = None;
        for bootstrapping_computation_completed_session in
            bootstrapping_computation_completed_sessions
        {
            match bootstrapping_computation_completed_session {
                Ok((round, completed_sessions)) => {
                    for session_identifier in completed_sessions {
                        self.dwallet_mpc_manager
                            .complete_computation_mpc_session_and_create_if_not_exists(
                                &session_identifier,
                            );
                    }
                    last_bootstrapped_consensus_round = Some(round);
                }
                Err(e) => {
                    error!(
                        err=?e,
                        last_bootstrapped_consensus_round,
                        "failed to load all completed MPC sessions from the local DB during bootstrapping"
                    );
                    panic!("failed to load all completed MPC sessions from the local DB during bootstrapping");
                }
            }
        }
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
                    should_never_happen=true,
                    consensus_round,
                    last_read_consensus_round=?self.last_read_consensus_round,
                    "consensus round must be in a ascending order"
                );

                return false;
            }

            // Let's start processing the MPC messages for the current round.
            self.dwallet_mpc_manager
                .handle_consensus_round_messages(consensus_round, mpc_messages);

            // Now we have the MPC messages for the current round, we can
            // process the MPC outputs for the current round.
            let (mut checkpoint_messages, completed_sessions) = self
                .dwallet_mpc_manager
                .handle_consensus_round_outputs(consensus_round, mpc_outputs);

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
                .insert_dwallet_mpc_computation_completed_sessions(
                    &consensus_round,
                    &completed_sessions,
                )
            {
                error!(
                    err=?e,
                    ?consensus_round,
                    ?completed_sessions,
                    "failed to insert computation completed MPC sessions into the local DB"
                );
            }
            self.last_read_consensus_round = Some(consensus_round);
        }

        true
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
                                let malicious_authorities = if !malicious_parties.is_empty() {
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
                                    malicious_authorities
                                } else {
                                    vec![]
                                };

                                self.dwallet_mpc_manager
                                    .record_malicious_actors(&malicious_authorities);

                                let consensus_message = self.new_dwallet_mpc_output_message(
                                    session_identifier,
                                    &mpc_event_data,
                                    public_output,
                                    malicious_authorities,
                                    false,
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
                                    &mpc_event_data,
                                    vec![],
                                    vec![],
                                    true,
                                );

                                if let Err(err) = consensus_adapter
                                    .submit_to_consensus(&[consensus_message], &epoch_store)
                                    .await
                                {
                                    error!(
                                        ?session_identifier,
                                        validator=?validator_name,
                                        error=?err,
                                        "failed to submit an MPC SessionFailed message to consensus"
                                    );
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
    ) -> ConsensusTransaction {
        ConsensusTransaction::new_dwallet_mpc_message(
            self.epoch_store.name.clone(),
            session_identifier,
            message,
            mpc_round,
        )
    }

    /// Create a new consensus transaction with the flow result (output) to be
    /// sent to the other MPC parties.
    /// Errors if the epoch was switched in the middle and was not available.
    fn new_dwallet_mpc_output_message(
        &self,
        session_identifier: SessionIdentifier,
        mpc_event_data: &MPCEventData,
        output: Vec<u8>,
        malicious_authorities: Vec<AuthorityName>,
        rejected: bool,
    ) -> ConsensusTransaction {
        let output = Self::build_dwallet_checkpoint_message_kinds_from_output(
            &session_identifier,
            &mpc_event_data.request_input,
            output,
            rejected,
        );
        ConsensusTransaction::new_dwallet_mpc_output(
            self.epoch_store.name,
            session_identifier,
            output,
            malicious_authorities,
        )
    }

    fn build_dwallet_checkpoint_message_kinds_from_output(
        session_identifier: &SessionIdentifier,
        request_input: &MPCRequestInput,
        output: Vec<u8>,
        rejected: bool,
    ) -> Vec<DWalletCheckpointMessageKind> {
        info!(
            mpc_protocol=?request_input,
            session_identifier=?session_identifier,
            "Creating session output message for checkpoint"
        );
        match &request_input {
            MPCRequestInput::DKGFirst(request_input) => {
                let tx = DWalletCheckpointMessageKind::RespondDWalletDKGFirstRoundOutput(
                    DKGFirstRoundOutput {
                        dwallet_id: request_input.event_data.dwallet_id.to_vec(),
                        output,
                        session_sequence_number: request_input.session_sequence_number,
                        rejected,
                    },
                );
                vec![tx]
            }
            MPCRequestInput::DKGSecond(request_input) => {
                let tx = DWalletCheckpointMessageKind::RespondDWalletDKGSecondRoundOutput(
                    DKGSecondRoundOutput {
                        output,
                        dwallet_id: request_input.event_data.dwallet_id.to_vec(),
                        encrypted_secret_share_id: request_input
                            .event_data
                            .encrypted_user_secret_key_share_id
                            .to_vec(),
                        rejected,
                        session_sequence_number: request_input.session_sequence_number,
                    },
                );
                vec![tx]
            }
            MPCRequestInput::Presign(request_input) => {
                let tx = DWalletCheckpointMessageKind::RespondDWalletPresign(PresignOutput {
                    presign: output,
                    dwallet_id: request_input.event_data.dwallet_id.map(|id| id.to_vec()),
                    presign_id: request_input.event_data.presign_id.to_vec(),
                    rejected,
                    session_sequence_number: request_input.session_sequence_number,
                });
                vec![tx]
            }
            MPCRequestInput::Sign(request_input) => {
                let tx = DWalletCheckpointMessageKind::RespondDWalletSign(SignOutput {
                    signature: output,
                    dwallet_id: request_input.event_data.dwallet_id.to_vec(),
                    is_future_sign: request_input.event_data.is_future_sign,
                    sign_id: request_input.event_data.sign_id.to_vec(),
                    rejected,
                    session_sequence_number: request_input.session_sequence_number,
                });
                vec![tx]
            }
            MPCRequestInput::EncryptedShareVerification(request_input) => {
                let tx = DWalletCheckpointMessageKind::RespondDWalletEncryptedUserShare(
                    EncryptedUserShareOutput {
                        dwallet_id: request_input.event_data.dwallet_id.to_vec(),
                        encrypted_user_secret_key_share_id: request_input
                            .event_data
                            .encrypted_user_secret_key_share_id
                            .to_vec(),
                        rejected,
                        session_sequence_number: request_input.session_sequence_number,
                    },
                );
                vec![tx]
            }
            MPCRequestInput::PartialSignatureVerification(request_input) => {
                let tx =
                    DWalletCheckpointMessageKind::RespondDWalletPartialSignatureVerificationOutput(
                        PartialSignatureVerificationOutput {
                            dwallet_id: request_input.event_data.dwallet_id.to_vec(),
                            partial_centralized_signed_message_id: request_input
                                .event_data
                                .partial_centralized_signed_message_id
                                .to_vec(),
                            rejected,
                            session_sequence_number: request_input.session_sequence_number,
                        },
                    );
                vec![tx]
            }
            MPCRequestInput::NetworkEncryptionKeyDkg(_key_scheme, request_input) => {
                let slices = if rejected {
                    vec![MPCNetworkDKGOutput {
                        dwallet_network_encryption_key_id: request_input
                            .event_data
                            .dwallet_network_encryption_key_id
                            .clone()
                            .to_vec(),
                        public_output: vec![],
                        supported_curves: vec![DWalletMPCNetworkKeyScheme::Secp256k1 as u32],
                        is_last: true,
                        rejected: true,
                        session_sequence_number: request_input.session_sequence_number,
                    }]
                } else {
                    Self::slice_public_output_into_messages(
                        output,
                        |public_output_chunk, is_last| MPCNetworkDKGOutput {
                            dwallet_network_encryption_key_id: request_input
                                .event_data
                                .dwallet_network_encryption_key_id
                                .clone()
                                .to_vec(),
                            public_output: public_output_chunk,
                            supported_curves: vec![DWalletMPCNetworkKeyScheme::Secp256k1 as u32],
                            is_last,
                            rejected: false,
                            session_sequence_number: request_input.session_sequence_number,
                        },
                    )
                };

                let messages: Vec<_> = slices
                    .into_iter()
                    .map(DWalletCheckpointMessageKind::RespondDWalletMPCNetworkDKGOutput)
                    .collect();
                messages
            }
            MPCRequestInput::NetworkEncryptionKeyReconfiguration(request_input) => {
                let slices = if rejected {
                    vec![MPCNetworkReconfigurationOutput {
                        dwallet_network_encryption_key_id: request_input
                            .event_data
                            .dwallet_network_encryption_key_id
                            .clone()
                            .to_vec(),
                        public_output: vec![],
                        supported_curves: vec![DWalletMPCNetworkKeyScheme::Secp256k1 as u32],
                        is_last: true,
                        rejected: true,
                        session_sequence_number: request_input.session_sequence_number,
                    }]
                } else {
                    Self::slice_public_output_into_messages(
                        output,
                        |public_output_chunk, is_last| MPCNetworkReconfigurationOutput {
                            dwallet_network_encryption_key_id: request_input
                                .event_data
                                .dwallet_network_encryption_key_id
                                .clone()
                                .to_vec(),
                            public_output: public_output_chunk,
                            supported_curves: vec![DWalletMPCNetworkKeyScheme::Secp256k1 as u32],
                            is_last,
                            rejected: false,
                            session_sequence_number: request_input.session_sequence_number,
                        },
                    )
                };

                let messages: Vec<_> = slices
                    .into_iter()
                    .map(
                        DWalletCheckpointMessageKind::RespondDWalletMPCNetworkReconfigurationOutput,
                    )
                    .collect();
                messages
            }
            MPCRequestInput::MakeDWalletUserSecretKeySharesPublicRequest(request_input) => {
                let tx = DWalletCheckpointMessageKind::RespondMakeDWalletUserSecretKeySharesPublic(
                    MakeDWalletUserSecretKeySharesPublicOutput {
                        dwallet_id: request_input.event_data.dwallet_id.to_vec(),
                        public_user_secret_key_shares: request_input
                            .event_data
                            .public_user_secret_key_shares
                            .clone(),
                        rejected,
                        session_sequence_number: request_input.session_sequence_number,
                    },
                );
                vec![tx]
            }
            MPCRequestInput::DWalletImportedKeyVerificationRequest(request_input) => {
                let tx = DWalletCheckpointMessageKind::RespondDWalletImportedKeyVerificationOutput(
                    DWalletImportedKeyVerificationOutput {
                        dwallet_id: request_input.event_data.dwallet_id.to_vec().clone(),
                        public_output: output,
                        encrypted_user_secret_key_share_id: request_input
                            .event_data
                            .encrypted_user_secret_key_share_id
                            .to_vec()
                            .clone(),
                        rejected,
                        session_sequence_number: request_input.session_sequence_number,
                    },
                );
                vec![tx]
            }
        }
    }

    /// Break down the key to slices because of chain transaction size limits.
    /// Limit 16 KB per Tx `pure` argument.
    fn slice_public_output_into_messages<T>(
        public_output: Vec<u8>,
        func: impl Fn(Vec<u8>, bool) -> T,
    ) -> Vec<T> {
        let mut slices = Vec::new();
        // We set a total of 5 KB since we need 6 KB buffer for other params.

        let public_chunks = public_output.chunks(FIVE_KILO_BYTES).collect_vec();
        let empty: &[u8] = &[];
        // Take the max of the two lengths to ensure we have enough slices.
        for i in 0..public_chunks.len() {
            // If the chunk is missing, use an empty slice, as the size of the slices can be different.
            let public_chunk = public_chunks.get(i).unwrap_or(&empty);
            slices.push(func(public_chunk.to_vec(), i == public_chunks.len() - 1));
        }
        slices
    }
}
