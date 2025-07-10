mod advance_and_serialize;
mod input;
mod logger;

use class_groups::dkg::Secp256k1Party;
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{
    MPCMessage, MPCPrivateInput, MPCPrivateOutput, MPCSessionPublicOutput, MPCSessionStatus,
    SerializedWrappedMPCPublicOutput, VersionedDWalletImportedKeyVerificationOutput,
    VersionedDecryptionKeyReconfigurationOutput, VersionedDwalletDKGFirstRoundPublicOutput,
    VersionedDwalletDKGSecondRoundPublicOutput, VersionedPresignOutput, VersionedSignOutput,
};
use group::helpers::DeduplicateAndSort;
use group::PartyID;
use ika_types::crypto::AuthorityPublicKeyBytes;
use itertools::Itertools;
use mpc::{AsynchronousRoundResult, WeightedThresholdAccessStructure};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::runtime::Handle;
use tracing::{debug, error, info, warn};
use twopc_mpc::sign::Protocol;

use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::dwallet_dkg::{
    DWalletDKGFirstParty, DWalletDKGSecondParty, DWalletImportedKeyVerificationParty,
};
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::encrypt_user_share::verify_encrypted_share;
use crate::dwallet_mpc::make_dwallet_user_secret_key_shares_public::verify_secret_share;
use crate::dwallet_mpc::network_dkg::{advance_network_dkg, DwalletMPCNetworkKeys};
use crate::dwallet_mpc::presign::PresignParty;
use crate::dwallet_mpc::reconfiguration::ReconfigurationSecp256k1Party;
use crate::dwallet_mpc::sign::{verify_partial_signature, SignFirstParty};
use crate::dwallet_mpc::{message_digest, party_ids_to_authority_names};
use crate::stake_aggregator::StakeAggregator;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_consensus::ConsensusTransaction;
use ika_types::messages_dwallet_mpc::{
    AsyncProtocol, DWalletMPCEvent, DWalletMPCMessage, EncryptedShareVerificationRequestEvent,
    MPCRequestInput, MPCSessionRequest, MaliciousReport, SessionIdentifier, SessionType,
    ThresholdNotReachedReport, NETWORK_ENCRYPTION_KEY_DKG_STR_KEY,
    NETWORK_ENCRYPTION_KEY_RECONFIGURATION_STR_KEY,
};
use sui_types::base_types::{EpochId, ObjectID};

pub(crate) use advance_and_serialize::advance_and_serialize;
use dwallet_rng::RootSeed;
use ika_types::committee::{ClassGroupsEncryptionKeyAndProof, Committee};
pub(crate) use input::session_input_from_event;
pub(crate) use logger::MPCSessionLogger;

#[derive(Clone)]
pub enum PublicInput {
    DWalletImportedKeyVerificationRequest(
        <DWalletImportedKeyVerificationParty as mpc::Party>::PublicInput,
    ),
    DKGFirst(<DWalletDKGFirstParty as mpc::Party>::PublicInput),
    DKGSecond(<DWalletDKGSecondParty as mpc::Party>::PublicInput),
    Presign(<PresignParty as mpc::Party>::PublicInput),
    Sign(<SignFirstParty as mpc::Party>::PublicInput),
    NetworkEncryptionKeyDkg(<Secp256k1Party as mpc::Party>::PublicInput),
    EncryptedShareVerification(twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters),
    PartialSignatureVerification(twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters),
    NetworkEncryptionKeyReconfiguration(<ReconfigurationSecp256k1Party as mpc::Party>::PublicInput),
    MakeDWalletUserSecretKeySharesPublicPublicInput(
        twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
    ),
}

/// The DWallet MPC session data that is based on the event that initiated the session.
#[derive(Clone)]
pub struct MPCEventData {
    pub private_input: MPCPrivateInput,
    pub request_input: MPCRequestInput,
    pub(crate) decryption_key_shares:
        Option<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>,
    pub(crate) session_type: SessionType,
    pub(crate) session_sequence_number: u64,
    pub(crate) public_input: PublicInput,
    pub(crate) requires_network_key_data: bool,
    pub(crate) requires_next_active_committee: bool,
}

impl MPCEventData {
    pub(crate) fn try_new(
        event: DWalletMPCEvent,
        access_structure: &WeightedThresholdAccessStructure,
        committee: &Committee,
        network_keys: &DwalletMPCNetworkKeys,
        next_active_committee: Option<Committee>,
        validators_class_groups_public_keys_and_proofs: HashMap<
            PartyID,
            ClassGroupsEncryptionKeyAndProof,
        >,
    ) -> Result<Self, DwalletMPCError> {
        let (public_input, private_input) = session_input_from_event(
            event.clone(),
            access_structure,
            committee,
            network_keys,
            next_active_committee,
            validators_class_groups_public_keys_and_proofs,
        )?;

        let needs_decryption_key_shares = matches!(
            event.session_request.request_input.clone(),
            MPCRequestInput::Sign(_) | MPCRequestInput::NetworkEncryptionKeyReconfiguration(_)
        );

        let decryption_key_shares = if needs_decryption_key_shares {
            if let Some(network_encryption_key_id) = event
                .session_request
                .request_input
                .get_network_encryption_key_id()
            {
                Some(network_keys.get_decryption_key_shares(&network_encryption_key_id)?)
            } else {
                error!(
                    should_never_happen =? true,
                    session_id=?event.session_request.session_identifier,
                    "failed to get network encryption key ID for a session that requires decryption key shares",
                );

                None
            }
        } else {
            None
        };

        let mpc_event_data = Self {
            session_type: event.session_request.session_type,
            session_sequence_number: event.session_request.session_sequence_number,
            request_input: event.session_request.request_input,
            private_input,
            decryption_key_shares,
            public_input,
            requires_network_key_data: event.session_request.requires_network_key_data,
            requires_next_active_committee: event.session_request.requires_next_active_committee,
        };

        Ok(mpc_event_data)
    }
}

/// A dWallet MPC session.
/// It keeps track of the session, the channel to send messages to the session,
/// and the messages that are pending to be sent to the session.
// TODO (#539): Simplify struct to only contain session related data.
#[derive(Clone)]
pub(crate) struct DWalletMPCSession {
    /// The status of the MPC session.
    pub(super) status: MPCSessionStatus,
    /// All the messages that have been received for this session.
    /// We need to accumulate a threshold of those before advancing the session.
    /// TODO(Scaly): By consensus round, not mpc round
    /// HashMap{R1: Map{Validator1->Message, Validator2->Message}, R2: Map{Validator1->Message} ...}
    pub(super) messages_by_consensus_round: HashMap<u64, HashMap<PartyID, MPCMessage>>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    epoch_id: EpochId,
    pub(super) session_identifier: SessionIdentifier,
    // TODO(Scaly): delete this too?
    /// The current MPC round number of the session.
    /// Starts at `1` and increments after each advance of the session.
    /// In round `1` We start the flow, without messages, from the event trigger.
    /// Decremented only upon an `TWOPCMPCThresholdNotReached` Error.
    pub(super) current_round: usize,
    pub(crate) party_id: PartyID,
    // TODO (#539): Simplify struct to only contain session related data - remove this field.
    weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    pub(crate) mpc_event_data: Option<MPCEventData>,
    /// The *total* number of attempts to advance that failed in the session.
    /// Used to make `ThresholdNotReachedReport` unique.
    // TODO(Scaly): delete?
    pub(crate) attempts_count: usize,
    /// A mapping between the MPC protocol of this session to the authorities that voted for it.
    mpc_protocol_to_voting_authorities: HashMap<String, StakeAggregator<(), true>>,

    network_dkg_third_round_delay: usize,
    decryption_key_reconfiguration_third_round_delay: usize,

    /// The number of consensus rounds since the last time a quorum was reached for the session.
    consensus_rounds_since_quorum_reached: usize,
    validator_name: AuthorityPublicKeyBytes,
    committee: Arc<Committee>,

    dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,

    /// The root seed of this validator, used for deriving the per-round seed for advancing this session.
    /// SECURITY NOTICE: *MUST KEEP PRIVATE*.
    root_seed: RootSeed,
}

impl DWalletMPCSession {
    /// The round number where NetworkDkg protocol applies consensus delay.
    const NETWORK_DKG_DELAY_ROUND: usize = 3;

    /// The round number where DecryptionKey protocol applies consensus delay.
    const DECRYPTION_KEY_RECONFIGURATION_DELAY_ROUND: usize = 3;

    pub(crate) fn new(
        validator_name: AuthorityPublicKeyBytes,
        committee: Arc<Committee>,
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch: EpochId,
        status: MPCSessionStatus,
        session_identifier: SessionIdentifier,
        party_id: PartyID,
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        mpc_event_data: Option<MPCEventData>,
        network_dkg_third_round_delay: usize,
        decryption_key_reconfiguration_third_round_delay: usize,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
        root_seed: RootSeed,
    ) -> Self {
        Self {
            status,
            messages_by_consensus_round: HashMap::new(),
            consensus_adapter,
            epoch_id: epoch,
            session_identifier,
            current_round: 1,
            party_id,
            weighted_threshold_access_structure,
            mpc_event_data,
            received_more_messages_since_last_advance: false,
            attempts_count: 0,
            mpc_protocol_to_voting_authorities: HashMap::new(),
            agreed_mpc_protocol: None,
            network_dkg_third_round_delay,
            decryption_key_reconfiguration_third_round_delay,
            consensus_rounds_since_quorum_reached: 0,
            validator_name,
            committee,
            dwallet_mpc_metrics,
            root_seed,
        }
    }

    pub(crate) fn clear_data(&mut self) {
        self.mpc_event_data = None;
        self.messages_by_consensus_round = HashMap::new();
    }

    /// Advances the MPC session and sends the advancement result to the other validators.
    /// The consensus submission logic is being spawned as a separate tokio task, as it's an IO
    /// heavy task.
    /// Rayon, which is good for CPU heavy tasks, is used to perform the cryptographic
    /// computation, and Tokio, which is good for IO heavy tasks, is used to submit the result to
    /// the consensus.
    pub(super) fn advance(&mut self, tokio_runtime_handle: &Handle) -> DwalletMPCResult<()> {
        // TODO: no handle I think.

        // Make sure we transfer only the messages up to the current round
        // (exclude messages that might be received from future rounds)
        self.messages_by_consensus_round
            .retain(|round, _| round < &self.current_round);
        // Safe to unwrap as advance can only be called after the event is received.
        let mpc_protocol = self.mpc_event_data.clone().unwrap().request_input;
        match self.advance_specific_party() {
            Ok(AsynchronousRoundResult::Advance {
                malicious_parties,
                message,
            }) => {
                let session_id = self.session_identifier;
                let round_number = self.messages_by_consensus_round.len();
                info!(
                    mpc_protocol=?mpc_protocol,
                    session_id=?session_id,
                    validator=?self.validator_name,
                    round=?round_number,
                    "Advanced MPC session"
                );
                let consensus_adapter = self.consensus_adapter.clone();
                if !malicious_parties.is_empty() {
                    self.report_malicious_actors(malicious_parties, &self.committee)?;
                }
                let message = self.new_dwallet_mpc_message(message, &mpc_protocol.to_string())?;

                // TODO: not from here! send in channel
                // tokio_runtime_handle.spawn(async move {
                //     if let Err(err) = consensus_adapter
                //         .submit_to_consensus(&[message], &epoch_store)
                //         .await
                //     {
                //         error!(
                //             mpc_protocol=?mpc_protocol,
                //             session_id=?session_id,
                //             validator=?validator_name,
                //             round=?round_number,
                //             err=?err,
                //             "failed to submit an MPC message to consensus"
                //         );
                //     }
                // });

                Ok(())
            }
            Ok(AsynchronousRoundResult::Finalize {
                malicious_parties,
                private_output: _,
                public_output,
            }) => {
                info!(
                    mpc_protocol=?&mpc_protocol,
                    session_identifier=?self.session_identifier,
                    validator=?&self.validator_name,
                    "Reached public output (Finalize) for session"
                );
                let consensus_adapter = self.consensus_adapter.clone();
                if !malicious_parties.is_empty() {
                    warn!(
                        mpc_protocol=?&mpc_protocol,
                        session_identifier=?self.session_identifier,
                        validator=?&self.validator_name,
                        ?malicious_parties,
                        "Malicious Parties detected on MPC session Finalize",
                    );
                    self.report_malicious_actors(malicious_parties, &self.committee)?;
                }
                let consensus_message = self.new_dwallet_mpc_output_message(
                    MPCSessionPublicOutput::CompletedSuccessfully(public_output.clone()),
                )?;
                let session_id_clone = self.session_identifier;

                // TODO: not from here!
                // tokio_runtime_handle.spawn(async move {
                //     if let Err(err) = consensus_adapter
                //         .submit_to_consensus(&[consensus_message], &epoch_store)
                //         .await
                //     {
                //         error!(
                //             mpc_protocol=?mpc_protocol,
                //             session_id=?session_id_clone,
                //             validator=?validator_name,
                //             err=?err,
                //             "failed to submit an MPC output message to consensus",
                //         );
                //     }
                // });

                Ok(())
            }
            Err(DwalletMPCError::TWOPCMPCThresholdNotReached) => {
                error!(
                    err=?DwalletMPCError::TWOPCMPCThresholdNotReached,
                    session_identifier=?self.session_identifier,
                    validator=?self.validator_name,
                    crypto_round=?self.current_round,
                    party_id=?self.party_id,
                    mpc_protocol=?&mpc_protocol,
                    "MPC session failed"
                );
                self.report_threshold_not_reached(tokio_runtime_handle)
            }
            Err(err) => {
                error!(
                    session_identifier=?self.session_identifier,
                    validator=?self.validator_name,
                    crypto_round=?self.current_round,
                    party_id=?self.party_id,
                    error=?err,
                    mpc_protocol=?mpc_protocol,
                    epoch=?self.epoch_id,
                    "failed to advance the MPC session"
                );

                let consensus_adapter = self.consensus_adapter.clone();
                let consensus_message =
                    self.new_dwallet_mpc_output_message(MPCSessionPublicOutput::SessionFailed)?;
                let session_id_clone = self.session_identifier;
                let epoch_id_clone = self.epoch_id;

                // TODO(Scaly): what is this code
                // tokio_runtime_handle.spawn(async move {
                //     if let Err(err) = consensus_adapter
                //         .submit_to_consensus(&[consensus_message], &epoch_store)
                //         .await
                //     {
                //         error!(
                //             mpc_protocol=?&mpc_protocol,
                //             session_id=?session_id_clone,
                //             validator=?validator_name,
                //             epoch=?epoch_id_clone,
                //             error=?err,
                //             "failed to submit an MPC SessionFailed message to consensus");
                //     }
                // });

                Err(err)
            }
        }
    }

    /// Create a new consensus transaction with the flow result (output) to be
    /// sent to the other MPC parties.
    /// Errors if the epoch was switched in the middle and was not available.
    fn new_dwallet_mpc_output_message(
        &self,
        output: MPCSessionPublicOutput,
    ) -> DwalletMPCResult<ConsensusTransaction> {
        let output = bcs::to_bytes(&output)?;
        let Some(mpc_event_data) = &self.mpc_event_data else {
            return Err(DwalletMPCError::MissingEventDrivenData);
        };
        Ok(ConsensusTransaction::new_dwallet_mpc_output(
            self.validator_name,
            output,
            MPCSessionRequest {
                session_type: mpc_event_data.session_type.clone(),
                session_identifier: self.session_identifier,
                session_sequence_number: mpc_event_data.session_sequence_number,
                request_input: mpc_event_data.request_input.clone(),
                epoch: self.epoch_id,
                requires_network_key_data: mpc_event_data.requires_network_key_data,
                requires_next_active_committee: mpc_event_data.requires_next_active_committee,
            },
        ))
    }

    fn report_malicious_actors(
        &self,
        malicious_parties_ids: Vec<PartyID>,
        committee: &Committee,
    ) -> DwalletMPCResult<()> {
        // Makes sure all the validators report on the malicious
        // actors in the same order without duplicates.
        let malicious_parties_ids = malicious_parties_ids.deduplicate_and_sort();
        let report = MaliciousReport::new(
            party_ids_to_authority_names(&malicious_parties_ids, committee),
            self.session_identifier,
        );
        let report_tx = self.new_dwallet_report_failed_session_with_malicious_actors(report)?;
        let consensus_adapter = self.consensus_adapter.clone();

        // TODO: this should call the malicious handler. Its all a mess

        // TODO(Scaly): why is this sent from here?
        // tokio_runtime_handle.spawn(async move {
        //     if let Err(err) = consensus_adapter
        //         .submit_to_consensus(&[report_tx], &epoch_store)
        //         .await
        //     {
        //         error!("failed to submit an MPC message to consensus: {:?}", err);
        //     }
        // });

        Ok(())
    }

    /// Report that the session failed because the threshold was not reached.
    /// This is submitted to the consensus,
    /// in order to make sure that all the Validators agree that this session needs more messages.
    fn report_threshold_not_reached(&self, tokio_runtime_handle: &Handle) -> DwalletMPCResult<()> {
        // TODO: just save the report...

        // let report = ThresholdNotReachedReport {
        //     session_identifier: self.session_identifier,
        //     attempt: self.attempts_count,
        // };
        // let report_tx = self.new_dwallet_report_threshold_not_reached(report)?;
        // let epoch_store = self.epoch_store()?.clone();
        // let consensus_adapter = self.consensus_adapter.clone();
        // tokio_runtime_handle.spawn(async move {
        //     if let Err(err) = consensus_adapter
        //         .submit_to_consensus(&[report_tx], &epoch_store)
        //         .await
        //     {
        //         error!(
        //             ?err,
        //             "failed to submit `threshold not reached` report to consensus"
        //         );
        //     }
        // });
        Ok(())
    }

    fn advance_specific_party(
        &self,
    ) -> DwalletMPCResult<
        AsynchronousRoundResult<MPCMessage, MPCPrivateOutput, SerializedWrappedMPCPublicOutput>,
    > {
        // TODO(Scaly): move all of this logic elsewhere; this function should only call `advance()`, and write the result to the channel.
        let Some(mpc_event_data) = &self.mpc_event_data else {
            return Err(DwalletMPCError::MissingEventDrivenData);
        };
        let serialized_messages_skeleton = self
            .messages_by_consensus_round
            .iter()
            .map(|(round, messages_map)| {
                (
                    *round,
                    messages_map.keys().copied().sorted().collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<_, _>>();
        info!(
            mpc_protocol=?mpc_event_data.request_input,
            validator=?self.validator_name,
            session_identifier=?self.session_identifier,
            crypto_round=?self.current_round,
            weighted_parties=?self.weighted_threshold_access_structure,
            ?serialized_messages_skeleton,
            "Advancing MPC session"
        );
        let session_identifier =
            CommitmentSizedNumber::from_le_slice(&self.session_identifier.into_bytes());
        let party_to_authority_map = self.committee.party_to_authority_map();
        let mpc_protocol_name = mpc_event_data.request_input.to_string();

        // Create a base logger with common parameters.
        let base_logger = MPCSessionLogger::new()
            .with_protocol_name(mpc_protocol_name.clone())
            .with_party_to_authority_map(party_to_authority_map.clone());

        // Derive a one-time use, MPC protocol and round specific, deterministic random generator
        // from the private seed. This should only be used to `advance()` this specific round,
        // and is guaranteed to be deterministic - if we attempt to run the round twice, the same message will be generated.
        // SECURITY NOTICE: don't use for anything else other than (this particular) `advance()`, and keep private!
        let rng = self.root_seed.mpc_round_rng(
            session_identifier,
            self.current_round as u64,
            self.attempts_count as u64,
        );

        match &mpc_event_data.request_input {
            MPCRequestInput::DWalletImportedKeyVerificationRequest(event_data) => {
                let PublicInput::DWalletImportedKeyVerificationRequest(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_round,
                        weighted_parties=?self.weighted_threshold_access_structure,
                        ?serialized_messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                let result = advance_and_serialize::<DWalletImportedKeyVerificationParty>(
                    session_identifier,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.messages_by_consensus_round.clone(),
                    public_input,
                    (),
                    &base_logger,
                    rng,
                );
                match result.clone() {
                    Ok(AsynchronousRoundResult::Finalize {
                        public_output,
                        malicious_parties,
                        private_output,
                    }) => {
                        verify_encrypted_share(
                            &EncryptedShareVerificationRequestEvent {
                                decentralized_public_output: bcs::to_bytes(
                                    &VersionedDwalletDKGSecondRoundPublicOutput::V1(
                                        public_output.clone(),
                                    ),
                                )?,
                                encrypted_centralized_secret_share_and_proof: event_data
                                    .event_data
                                    .encrypted_centralized_secret_share_and_proof
                                    .clone(),
                                encryption_key: event_data.event_data.encryption_key.clone(),
                                encryption_key_id: event_data.event_data.encryption_key_id,
                                dwallet_network_encryption_key_id: event_data
                                    .event_data
                                    .dwallet_network_encryption_key_id,
                                curve: event_data.event_data.curve,

                                // Fields not relevant for verification; passing empty values.
                                dwallet_id: ObjectID::new([0; 32]),
                                source_encrypted_user_secret_key_share_id: ObjectID::new([0; 32]),
                                encrypted_user_secret_key_share_id: ObjectID::new([0; 32]),
                            },
                            public_input.protocol_public_parameters.clone(),
                        )?;
                        let public_output = bcs::to_bytes(
                            &VersionedDWalletImportedKeyVerificationOutput::V1(public_output),
                        )?;
                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        })
                    }
                    _ => result,
                }
            }
            MPCRequestInput::DKGFirst(..) => {
                info!(
                    mpc_protocol=?mpc_event_data.request_input,
                    validator=?self.validator_name,
                    session_identifier=?self.session_identifier,
                    crypto_round=?self.current_round,
                    "Advancing DKG first party",
                );
                let PublicInput::DKGFirst(public_input) = &mpc_event_data.public_input else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_round,
                        weighted_parties=?self.weighted_threshold_access_structure,
                        ?serialized_messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                let result = advance_and_serialize::<DWalletDKGFirstParty>(
                    session_identifier,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.messages_by_consensus_round.clone(),
                    public_input,
                    (),
                    &base_logger,
                    rng,
                );
                match result.clone() {
                    Ok(AsynchronousRoundResult::Finalize {
                        public_output,
                        malicious_parties,
                        private_output,
                    }) => {
                        let public_output = bcs::to_bytes(
                            &VersionedDwalletDKGFirstRoundPublicOutput::V1(public_output),
                        )?;
                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        })
                    }
                    _ => result,
                }
            }
            MPCRequestInput::DKGSecond(event_data) => {
                let PublicInput::DKGSecond(public_input) = &mpc_event_data.public_input else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_round,
                        weighted_parties=?self.weighted_threshold_access_structure,
                        ?serialized_messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                let result = advance_and_serialize::<DWalletDKGSecondParty>(
                    session_identifier,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.messages_by_consensus_round.clone(),
                    public_input,
                    (),
                    &base_logger,
                    rng,
                )?;
                if let AsynchronousRoundResult::Finalize { public_output, .. } = &result {
                    verify_encrypted_share(
                        &EncryptedShareVerificationRequestEvent {
                            decentralized_public_output: bcs::to_bytes(
                                &VersionedDwalletDKGSecondRoundPublicOutput::V1(
                                    public_output.clone(),
                                ),
                            )?,
                            encrypted_centralized_secret_share_and_proof: event_data
                                .event_data
                                .encrypted_centralized_secret_share_and_proof
                                .clone(),
                            encryption_key: event_data.event_data.encryption_key.clone(),
                            encryption_key_id: event_data.event_data.encryption_key_id,
                            dwallet_network_encryption_key_id: event_data
                                .event_data
                                .dwallet_network_encryption_key_id,
                            curve: event_data.event_data.curve,

                            // Fields not relevant for verification; passing empty values.
                            dwallet_id: ObjectID::new([0; 32]),
                            source_encrypted_user_secret_key_share_id: ObjectID::new([0; 32]),
                            encrypted_user_secret_key_share_id: ObjectID::new([0; 32]),
                        },
                        public_input.protocol_public_parameters.clone(),
                    )?;
                }
                match result.clone() {
                    AsynchronousRoundResult::Finalize {
                        public_output,
                        malicious_parties,
                        private_output,
                    } => {
                        let public_output = bcs::to_bytes(
                            &VersionedDwalletDKGSecondRoundPublicOutput::V1(public_output),
                        )?;
                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        })
                    }
                    _ => Ok(result),
                }
            }
            MPCRequestInput::Presign(..) => {
                let PublicInput::Presign(public_input) = &mpc_event_data.public_input else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_round,
                        weighted_parties=?self.weighted_threshold_access_structure,
                        ?serialized_messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                let result = advance_and_serialize::<PresignParty>(
                    session_identifier,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.messages_by_consensus_round.clone(),
                    public_input,
                    (),
                    &base_logger,
                    rng,
                );
                match result.clone() {
                    Ok(AsynchronousRoundResult::Finalize {
                        public_output,
                        malicious_parties,
                        private_output,
                    }) => {
                        let public_output =
                            bcs::to_bytes(&VersionedPresignOutput::V1(public_output))?;
                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        })
                    }
                    _ => result,
                }
            }
            MPCRequestInput::Sign(..) => {
                if let Some(decryption_key_shares) = mpc_event_data.decryption_key_shares.clone() {
                    let raw_decryption_key_shares = decryption_key_shares
                        .iter()
                        .map(|(party_id, share)| (*party_id, share.decryption_key_share))
                        .collect::<HashMap<_, _>>();

                    // Extend base logger with decryption key shares for Sign protocol
                    let logger =
                        base_logger.with_decryption_key_shares(raw_decryption_key_shares.clone());
                    let PublicInput::Sign(public_input) = &mpc_event_data.public_input else {
                        error!(
                            should_never_happen =? true,
                            mpc_protocol=?mpc_event_data.request_input,
                            validator=?self.validator_name,
                            session_identifier=?self.session_identifier,
                            crypto_round=?self.current_round,
                            weighted_parties=?self.weighted_threshold_access_structure,
                            ?serialized_messages_skeleton,
                            "session public input does not match the session type"
                        );
                        return Err(DwalletMPCError::InvalidSessionPublicInput);
                    };
                    let result = advance_and_serialize::<SignFirstParty>(
                        session_identifier,
                        self.party_id,
                        &self.weighted_threshold_access_structure,
                        self.messages_by_consensus_round.clone(),
                        public_input,
                        decryption_key_shares,
                        &logger,
                        rng,
                    );
                    self.update_expected_decrypters_metrics(&public_input.expected_decrypters)?;
                    match result.clone() {
                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        }) => {
                            let public_output =
                                bcs::to_bytes(&VersionedSignOutput::V1(public_output))?;
                            Ok(AsynchronousRoundResult::Finalize {
                                public_output,
                                malicious_parties,
                                private_output,
                            })
                        }
                        _ => result,
                    }
                } else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_round,
                        weighted_parties=?self.weighted_threshold_access_structure,
                        ?serialized_messages_skeleton,
                        "no decryption key shares for a session that requires them (sign)"
                    );

                    Err(DwalletMPCError::InvalidSessionPublicInput)
                }
            }
            MPCRequestInput::NetworkEncryptionKeyDkg(key_scheme, _init_event) => {
                advance_network_dkg(
                    session_identifier,
                    &self.weighted_threshold_access_structure,
                    &self.mpc_event_data.clone().unwrap(),
                    self.party_id,
                    key_scheme,
                    self.messages_by_consensus_round.clone(),
                    bcs::from_bytes(
                        &mpc_event_data
                            .private_input
                            .clone()
                            .ok_or(DwalletMPCError::MissingMPCPrivateInput)?,
                    )?,
                    &base_logger,
                    rng,
                )
            }
            MPCRequestInput::EncryptedShareVerification(verification_data) => {
                let PublicInput::EncryptedShareVerification(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_round,
                        weighted_parties=?self.weighted_threshold_access_structure,
                        ?serialized_messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };
                match verify_encrypted_share(&verification_data.event_data, public_input.clone()) {
                    Ok(_) => Ok(AsynchronousRoundResult::Finalize {
                        public_output: vec![],
                        private_output: vec![],
                        malicious_parties: vec![],
                    }),
                    Err(err) => Err(err),
                }
            }
            MPCRequestInput::PartialSignatureVerification(event_data) => {
                let hashed_message = bcs::to_bytes(
                    &message_digest(
                        &event_data.event_data.message,
                        &event_data.event_data.hash_scheme.try_into().unwrap(),
                    )
                    .map_err(|err| DwalletMPCError::TwoPCMPCError(err.to_string()))?,
                )?;
                let PublicInput::PartialSignatureVerification(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_round,
                        weighted_parties=?self.weighted_threshold_access_structure,
                        ?serialized_messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };
                verify_partial_signature(
                    &hashed_message,
                    &event_data.event_data.dkg_output,
                    &event_data.event_data.presign,
                    &event_data.event_data.message_centralized_signature,
                    public_input,
                )?;

                Ok(AsynchronousRoundResult::Finalize {
                    public_output: vec![],
                    private_output: vec![],
                    malicious_parties: vec![],
                })
            }
            MPCRequestInput::NetworkEncryptionKeyReconfiguration(_) => {
                let PublicInput::NetworkEncryptionKeyReconfiguration(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_round,
                        weighted_parties=?self.weighted_threshold_access_structure,
                        ?serialized_messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                if let Some(decryption_key_shares) = mpc_event_data.decryption_key_shares.clone() {
                    let decryption_key_shares = decryption_key_shares
                        .iter()
                        .map(|(party_id, share)| (*party_id, share.decryption_key_share))
                        .collect::<HashMap<_, _>>();

                    // Extend base logger with decryption key shares for Reconfiguration protocol
                    let logger =
                        base_logger.with_decryption_key_shares(decryption_key_shares.clone());

                    let result = advance_and_serialize::<ReconfigurationSecp256k1Party>(
                        session_identifier,
                        self.party_id,
                        &self.weighted_threshold_access_structure,
                        self.messages_by_consensus_round.clone(),
                        public_input,
                        decryption_key_shares,
                        &logger,
                        rng,
                    );

                    match result.clone() {
                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        }) => {
                            let public_output = bcs::to_bytes(
                                &VersionedDecryptionKeyReconfigurationOutput::V1(public_output),
                            )?;
                            Ok(AsynchronousRoundResult::Finalize {
                                public_output,
                                malicious_parties,
                                private_output,
                            })
                        }
                        _ => result,
                    }
                } else {
                    error!(
                    should_never_happen =? true,
                    mpc_protocol=?mpc_event_data.request_input,
                    validator=?self.validator_name,
                    session_identifier=?self.session_identifier,
                    crypto_round=?self.current_round,
                    weighted_parties=?self.weighted_threshold_access_structure,
                    ?serialized_messages_skeleton,
                    "no decryption key shares for a session that requires them (reconfiguration)"
                    );

                    Err(DwalletMPCError::InvalidSessionPublicInput)
                }
            }
            MPCRequestInput::MakeDWalletUserSecretKeySharesPublicRequest(init_event) => {
                let PublicInput::MakeDWalletUserSecretKeySharesPublicPublicInput(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_round,
                        weighted_parties=?self.weighted_threshold_access_structure,
                        ?serialized_messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };
                match verify_secret_share(
                    public_input.clone(),
                    init_event.event_data.public_user_secret_key_shares.clone(),
                    init_event.event_data.public_output.clone(),
                ) {
                    Ok(..) => Ok(AsynchronousRoundResult::Finalize {
                        public_output: init_event.event_data.public_user_secret_key_shares.clone(),
                        private_output: vec![],
                        malicious_parties: vec![],
                    }),
                    Err(err) => {
                        error!(
                            ?err,
                            session_identifier=?self.session_identifier,
                            validator=?self.validator_name,
                            crypto_round=?self.current_round,
                            "failed to verify secret share"
                        );
                        Err(DwalletMPCError::DWalletSecretNotMatchedDWalletOutput)
                    }
                }
            }
        }
    }

    /// Create a new consensus transaction with the message to be sent to the other MPC parties.
    /// Returns Error only if the epoch switched in the middle and was not available.
    fn new_dwallet_mpc_message(
        &self,
        message: MPCMessage,
        mpc_protocol: &str,
    ) -> DwalletMPCResult<ConsensusTransaction> {
        // MPC event data can not be none, when sending a message.
        let Some(mpc_event_data) = &self.mpc_event_data else {
            return Err(DwalletMPCError::MissingEventDrivenData);
        };
        let session_request = MPCSessionRequest {
            session_type: mpc_event_data.session_type.clone(),
            request_input: mpc_event_data.request_input.clone(),
            epoch: self.epoch_id,
            session_identifier: self.session_identifier,
            session_sequence_number: mpc_event_data.session_sequence_number,
            requires_network_key_data: mpc_event_data.requires_network_key_data,
            requires_next_active_committee: mpc_event_data.requires_next_active_committee,
        };
        Ok(ConsensusTransaction::new_dwallet_mpc_message(
            self.validator_name,
            message,
            self.session_identifier,
            self.current_round,
            mpc_protocol.to_string(),
            session_request,
        ))
    }

    /// Report that the session failed because of malicious actors.
    /// Once a quorum of validators reports the same actor, it is considered malicious.
    /// The session will be continued, and the malicious actors will be ignored.
    fn new_dwallet_report_failed_session_with_malicious_actors(
        &self,
        report: MaliciousReport,
    ) -> DwalletMPCResult<ConsensusTransaction> {
        Ok(
            ConsensusTransaction::new_dwallet_mpc_session_failed_with_malicious(
                self.validator_name,
                report,
            ),
        )
    }

    fn new_dwallet_report_threshold_not_reached(
        &self,
        report: ThresholdNotReachedReport,
    ) -> DwalletMPCResult<ConsensusTransaction> {
        Ok(
            ConsensusTransaction::new_dwallet_mpc_session_threshold_not_reached(
                self.validator_name,
                report,
            ),
        )
    }

    /// Stores an incoming message.
    /// This function performs no checks, it simply stores the message in the map.
    ///
    /// If a party sent a message twice, it will be overridden.
    /// Whilst that is malicious, it has no effect since the messages come in order, so all validators end up seeing the same map.
    /// Other malicious activities like sending a message for a wrong round are also not reported since they have no practical impact for similar reasons.
    pub(crate) fn store_message(
        &mut self,
        consensus_round: u64,
        sender_party_id: PartyID,
        message: DWalletMPCMessage,
    ) {
        debug!(
            session_id=?message.session_identifier,
            from_authority=?message.authority,
            receiving_authority=?self.validator_name,
            crypto_round_number=?message.round_number,
            message_size_bytes=?message.message.len(),
            mpc_protocol=message.mpc_protocol,
            "Received a dWallet MPC message",
        );

        let round_messages_map = self
            .messages_by_consensus_round
            .entry(consensus_round)
            .or_default();

        round_messages_map.insert(sender_party_id, message.message);
    }

    /// Checks if the session should wait for additional consensus rounds before advancing.
    ///
    /// This method implements protocol-specific delays for certain MPC rounds in specific protocols
    /// (Sign, NetworkDkg, DecryptionKeyReconfiguration).
    ///
    /// - **Sign protocol**: Applies delay in round 2 (SIGN_DELAY_ROUND)
    ///   using `sign_second_round_delay` config
    /// - **NetworkDkg protocol**: Applies delay in round 3 (NETWORK_DKG_DELAY_ROUND)
    ///   using `network_dkg_third_round_delay` config
    /// - **DecryptionKeyReconfiguration protocol**: Applies delay in round 3
    ///   (DECRYPTION_KEY_RECONFIGURATION_DELAY_ROUND) using `decryption_key_reconfiguration_third_round_delay` config
    /// - **Other protocols**: No delay applied, always ready to advance
    ///
    /// When a delay is required, the method tracks `consensus_rounds_since_quorum_reached`
    /// and only allows advancement once the configured delay period has passed.
    ///
    /// # Returns
    ///
    /// Returns `ReadyToAdvanceCheckResult` indicating:
    /// - `is_ready`: Whether the session can advance to the next round
    /// - `malicious_parties`: Always empty (no malicious behavior detected in timing)
    ///
    /// # Behavior
    ///
    /// - If delay is required but not yet satisfied: increments the consensus round counter
    ///   and returns `is_ready: false`
    /// - If delay is satisfied: resets the consensus round counter and returns `is_ready: true`
    /// - If no delay is required for the current protocol/round: returns `is_ready: true`
    fn wait_consensus_rounds_delay(&mut self) -> bool {
        match self.agreed_mpc_protocol.as_deref() {
            None => false, // TODO(@scaly): what is this? remove
            Some(protocol) => match protocol {
                NETWORK_ENCRYPTION_KEY_DKG_STR_KEY => self.check_round_delay(
                    Self::NETWORK_DKG_DELAY_ROUND,
                    self.network_dkg_third_round_delay,
                ),
                NETWORK_ENCRYPTION_KEY_RECONFIGURATION_STR_KEY => self.check_round_delay(
                    Self::DECRYPTION_KEY_RECONFIGURATION_DELAY_ROUND,
                    self.decryption_key_reconfiguration_third_round_delay,
                ),
                _ => true,
            },
        }
    }

    /// Helper method to check if a specific round should be delayed.
    ///
    /// # Arguments
    ///
    /// * `target_round` - The round number that requires delay checking
    /// * `required_delay` - The required delay duration.
    ///
    /// # Returns
    ///
    /// Returns `ReadyToAdvanceCheckResult` with the appropriate readiness status.
    fn check_round_delay(
        &mut self,
        target_round: usize,
        required_consensus_rounds_delay: usize,
    ) -> bool {
        if self.current_round != target_round {
            true
        } else if self.consensus_rounds_since_quorum_reached >= required_consensus_rounds_delay {
            info!(
                ?self.consensus_rounds_since_quorum_reached,
                ?self.current_round,
                ?self.agreed_mpc_protocol,
                ?self.session_identifier,
                messages_count_for_current_round=?self.messages_by_consensus_round.get(&(self.current_round - 1)).unwrap_or(&HashMap::new()).len(),
                "Quorum reached for MPC session and delay passed, advancing to next round",
            );
            self.consensus_rounds_since_quorum_reached = 0;

            true
        } else {
            info!(
                ?self.consensus_rounds_since_quorum_reached,
                ?self.current_round,
                ?self.agreed_mpc_protocol,
                messages_count_for_current_round=?self.messages_by_consensus_round.get(&(self.current_round - 1)).unwrap_or(&HashMap::new()).len(),
                "Quorum reached for MPC session but delay not passed yet, waiting for another round",
            );

            self.consensus_rounds_since_quorum_reached += 1;

            false
        }
    }

    pub(crate) fn check_quorum_for_next_crypto_round(&mut self) -> bool {
        match self.status {
            MPCSessionStatus::Active => {
                // Check if we have the threshold of messages for the previous round
                // to advance to the next round.
                let is_quorum_reached = if let Some(previous_round_messages) =
                    self.messages_by_consensus_round.get(&(self.current_round - 1))
                {
                    let previous_round_message_senders: HashSet<PartyID> =
                        previous_round_messages.keys().cloned().collect();

                    self.weighted_threshold_access_structure
                        .is_authorized_subset(&previous_round_message_senders)
                        .is_ok()
                } else {
                    false
                };

                // MPC First round doesn't require a threshold of messages to advance.
                // This is the round after the MPC event.
                // It also doesn't have a delay.
                if self.current_round == 1 {
                    true
                } else if is_quorum_reached
                    && self.received_more_messages_since_last_advance
                    && self.agreed_mpc_protocol.is_some()
                {
                    self.wait_consensus_rounds_delay()
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn update_expected_decrypters_metrics(
        &self,
        expected_decrypters: &HashSet<PartyID>,
    ) -> DwalletMPCResult<()> {
        if self.current_round != 2 {
            return Ok(());
        }
        let participating_expected_decrypters: HashSet<PartyID> = expected_decrypters
            .iter()
            .filter(|party_id| {
                self.messages_by_consensus_round
                    .get(&(self.current_round - 1))
                    .is_some_and(|messages| messages.contains_key(*party_id))
            })
            .copied()
            .collect();
        if self
            .weighted_threshold_access_structure
            .is_authorized_subset(&participating_expected_decrypters)
            .is_ok()
        {
            self.dwallet_mpc_metrics
                .number_of_expected_sign_sessions
                .inc();
        } else {
            self.dwallet_mpc_metrics
                .number_of_unexpected_sign_sessions
                .inc();
        }
        Ok(())
    }
}
