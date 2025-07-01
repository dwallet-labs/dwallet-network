mod advance_and_serialize;
mod input;
mod logger;
mod session_info;

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
use itertools::Itertools;
use mpc::{AsynchronousRoundResult, WeightedThresholdAccessStructure};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Weak};
use tokio::runtime::Handle;
use tracing::{error, info, warn};
use twopc_mpc::sign::Protocol;

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::dwallet_dkg::{
    DWalletDKGFirstParty, DWalletDKGSecondParty, DWalletImportedKeyVerificationParty,
};
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::encrypt_user_share::verify_encrypted_share;
use crate::dwallet_mpc::make_dwallet_user_secret_key_shares_public::verify_secret_share;
use crate::dwallet_mpc::network_dkg::advance_network_dkg;
use crate::dwallet_mpc::presign::PresignParty;
use crate::dwallet_mpc::reconfiguration::ReconfigurationSecp256k1Party;
use crate::dwallet_mpc::sign::{verify_partial_signature, SignFirstParty};
use crate::dwallet_mpc::{message_digest, party_ids_to_authority_names};
use crate::stake_aggregator::StakeAggregator;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_consensus::ConsensusTransaction;
use ika_types::messages_dwallet_mpc::{
    AsyncProtocol, DWalletMPCMessage, EncryptedShareVerificationRequestEvent, MPCProtocolInitData,
    MaliciousReport, SessionIdentifier, SessionInfo, SessionType, ThresholdNotReachedReport,
    NETWORK_ENCRYPTION_KEY_DKG_STR_KEY, NETWORK_ENCRYPTION_KEY_RECONFIGURATION_STR_KEY,
};
use sui_types::base_types::{EpochId, ObjectID};

pub(crate) use advance_and_serialize::advance_and_serialize;
use dwallet_rng::RootSeed;
pub(crate) use input::session_input_from_event;
pub(crate) use logger::MPCSessionLogger;
pub(crate) use session_info::session_info_from_event;

/// Represents the result of checking whether the session is ready to advance.
///
/// This structure contains a flag indicating if the session is ready to advance,
/// and a list of malicious parties detected during the check.
pub(crate) struct ReadyToAdvanceCheckResult {
    pub(crate) is_ready: bool,
    pub(crate) malicious_parties: Vec<PartyID>,
}

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
    pub init_protocol_data: MPCProtocolInitData,
    pub(crate) decryption_shares: HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>,
    pub(crate) session_type: SessionType,
    pub(crate) public_input: PublicInput,
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
    /// HashMap{R1: Map{Validator1->Message, Validator2->Message}, R2: Map{Validator1->Message} ...}
    pub(super) serialized_full_messages: HashMap<usize, HashMap<PartyID, MPCMessage>>,
    epoch_store: Weak<AuthorityPerEpochStore>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    epoch_id: EpochId,
    pub(super) session_identifier: SessionIdentifier,
    /// The current MPC round number of the session.
    /// Starts at `1` and increments after each advance of the session.
    /// In round `1` We start the flow, without messages, from the event trigger.
    /// Decremented only upon an `TWOPCMPCThresholdNotReached` Error.
    pub(super) current_round: usize,
    pub(crate) party_id: PartyID,
    // TODO (#539): Simplify struct to only contain session related data - remove this field.
    weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    pub(crate) mpc_event_data: Option<MPCEventData>,
    /// Indicates whether more messages have been received since the last advance.
    /// The reason we need it is to know when to retry and advance call that failed.
    /// For example, quorum was not reached because some Authorities were malicious.
    pub(crate) received_more_messages_since_last_advance: bool,
    /// The *total* number of attempts to advance that failed in the session.
    /// Used to make `ThresholdNotReachedReport` unique.
    pub(crate) attempts_count: usize,
    /// A mapping between the MPC protocol of this session to the authorities that voted for it.
    mpc_protocol_to_voting_authorities: HashMap<String, StakeAggregator<(), true>>,
    /// The MPC protocol that was agreed upon by a quorum of the authorities.
    agreed_mpc_protocol: Option<String>,
    /// The number of consensus rounds since the last time a quorum was reached for the session.
    consensus_rounds_since_quorum_reached: usize,
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
        epoch_store: Weak<AuthorityPerEpochStore>,
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch: EpochId,
        status: MPCSessionStatus,
        session_identifier: SessionIdentifier,
        party_id: PartyID,
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        mpc_event_data: Option<MPCEventData>,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
        root_seed: RootSeed,
    ) -> Self {
        Self {
            status,
            serialized_full_messages: HashMap::new(),
            consensus_adapter,
            epoch_store: epoch_store.clone(),
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
            consensus_rounds_since_quorum_reached: 0,
            dwallet_mpc_metrics,
            root_seed,
        }
    }

    pub(crate) fn clear_data(&mut self) {
        self.mpc_event_data = None;
        self.serialized_full_messages = HashMap::new();
    }

    /// Returns the epoch store.
    /// Errors if the epoch was switched in the middle.
    fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }

    /// Advances the MPC session and sends the advancement result to the other validators.
    /// The consensus submission logic is being spawned as a separate tokio task, as it's an IO
    /// heavy task.
    /// Rayon, which is good for CPU heavy tasks, is used to perform the cryptographic
    /// computation, and Tokio, which is good for IO heavy tasks, is used to submit the result to
    /// the consensus.
    pub(super) fn advance(&mut self, tokio_runtime_handle: &Handle) -> DwalletMPCResult<()> {
        // Make sure we transfer only the messages up to the current round
        // (exclude messages that might be received from future rounds)
        self.serialized_full_messages
            .retain(|round, _| round < &self.current_round);
        // Safe to unwrap as advance can only be called after the event is received.
        let mpc_protocol = self.mpc_event_data.clone().unwrap().init_protocol_data;
        match self.advance_specific_party() {
            Ok(AsynchronousRoundResult::Advance {
                malicious_parties,
                message,
            }) => {
                let session_id = self.session_identifier;
                let validator_name = self.epoch_store()?.name;
                let round_number = self.serialized_full_messages.len();
                info!(
                    mpc_protocol=?mpc_protocol,
                    session_id=?session_id,
                    validator=?validator_name,
                    round=?round_number,
                    "Advanced MPC session"
                );
                let consensus_adapter = self.consensus_adapter.clone();
                let epoch_store = self.epoch_store()?.clone();
                if !malicious_parties.is_empty() {
                    self.report_malicious_actors(tokio_runtime_handle, malicious_parties)?;
                }
                let message = self.new_dwallet_mpc_message(message, &mpc_protocol.to_string())?;
                tokio_runtime_handle.spawn(async move {
                    if let Err(err) = consensus_adapter
                        .submit_to_consensus(&[message], &epoch_store)
                        .await
                    {
                        error!(
                            mpc_protocol=?mpc_protocol,
                            session_id=?session_id,
                            validator=?validator_name,
                            round=?round_number,
                            err=?err,
                            "failed to submit an MPC message to consensus"
                        );
                    }
                });
                Ok(())
            }
            Ok(AsynchronousRoundResult::Finalize {
                malicious_parties,
                private_output: _,
                public_output,
            }) => {
                let validator_name = self.epoch_store()?.name;
                info!(
                    mpc_protocol=?&mpc_protocol,
                    session_identifier=?self.session_identifier,
                    validator=?&validator_name,
                    "Reached public output (Finalize) for session"
                );
                let consensus_adapter = self.consensus_adapter.clone();
                let epoch_store = self.epoch_store()?.clone();
                if !malicious_parties.is_empty() {
                    warn!(
                        mpc_protocol=?&mpc_protocol,
                        session_identifier=?self.session_identifier,
                        validator=?&validator_name,
                        ?malicious_parties,
                        "Malicious Parties detected on MPC session Finalize",
                    );
                    self.report_malicious_actors(tokio_runtime_handle, malicious_parties)?;
                }
                let consensus_message = self.new_dwallet_mpc_output_message(
                    MPCSessionPublicOutput::CompletedSuccessfully(public_output.clone()),
                )?;
                let session_id_clone = self.session_identifier;
                tokio_runtime_handle.spawn(async move {
                    if let Err(err) = consensus_adapter
                        .submit_to_consensus(&[consensus_message], &epoch_store)
                        .await
                    {
                        error!(
                            mpc_protocol=?mpc_protocol,
                            session_id=?session_id_clone,
                            validator=?validator_name,
                            err=?err,
                            "failed to submit an MPC output message to consensus",
                        );
                    }
                });
                Ok(())
            }
            Err(DwalletMPCError::TWOPCMPCThresholdNotReached) => {
                error!(
                    err=?DwalletMPCError::TWOPCMPCThresholdNotReached,
                    session_identifier=?self.session_identifier,
                    validator=?self.epoch_store()?.name,
                    crypto_round=?self.current_round,
                    party_id=?self.party_id,
                    mpc_protocol=?&mpc_protocol,
                    "MPC session failed"
                );
                self.report_threshold_not_reached(tokio_runtime_handle)
            }
            Err(err) => {
                let validator_name = self.epoch_store()?.name;

                error!(
                    session_identifier=?self.session_identifier,
                    validator=?validator_name,
                    crypto_round=?self.current_round,
                    party_id=?self.party_id,
                    error=?err,
                    mpc_protocol=?mpc_protocol,
                    epoch=?self.epoch_id,
                    "failed to advance the MPC session"
                );

                let consensus_adapter = self.consensus_adapter.clone();
                let epoch_store = self.epoch_store()?.clone();
                let consensus_message =
                    self.new_dwallet_mpc_output_message(MPCSessionPublicOutput::SessionFailed)?;
                let session_id_clone = self.session_identifier;
                let epoch_id_clone = self.epoch_id;
                tokio_runtime_handle.spawn(async move {
                    if let Err(err) = consensus_adapter
                        .submit_to_consensus(&[consensus_message], &epoch_store)
                        .await
                    {
                        error!(
                            mpc_protocol=?&mpc_protocol,
                            session_id=?session_id_clone,
                            validator=?validator_name,
                            epoch=?epoch_id_clone,
                            error=?err,
                            "failed to submit an MPC SessionFailed message to consensus");
                    }
                });
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
            self.epoch_store()?.name,
            output,
            SessionInfo {
                session_type: mpc_event_data.session_type.clone(),
                session_identifier: self.session_identifier,
                mpc_round: mpc_event_data.init_protocol_data.clone(),
                epoch: self.epoch_id,
            },
        ))
    }

    /// In the Sign-Identifiable Abort protocol, each validator sends a malicious report, even
    /// if no malicious actors are found. This is necessary to reach agreement on a malicious report
    /// and to punish the validator who started the Sign IA report if they sent a faulty report.
    fn report_malicious_actors(
        &self,
        tokio_runtime_handle: &Handle,
        malicious_parties_ids: Vec<PartyID>,
    ) -> DwalletMPCResult<()> {
        // Makes sure all the validators report on the malicious
        // actors in the same order without duplicates.
        let malicious_parties_ids = malicious_parties_ids.deduplicate_and_sort();
        let report = MaliciousReport::new(
            party_ids_to_authority_names(&malicious_parties_ids, &*self.epoch_store()?)?,
            self.session_identifier,
        );
        let report_tx = self.new_dwallet_report_failed_session_with_malicious_actors(report)?;
        let epoch_store = self.epoch_store()?.clone();
        let consensus_adapter = self.consensus_adapter.clone();
        tokio_runtime_handle.spawn(async move {
            if let Err(err) = consensus_adapter
                .submit_to_consensus(&[report_tx], &epoch_store)
                .await
            {
                error!("failed to submit an MPC message to consensus: {:?}", err);
            }
        });
        Ok(())
    }

    /// Report that the session failed because the threshold was not reached.
    /// This is submitted to the consensus,
    /// in order to make sure that all the Validators agree that this session needs more messages.
    fn report_threshold_not_reached(&self, tokio_runtime_handle: &Handle) -> DwalletMPCResult<()> {
        let report = ThresholdNotReachedReport {
            session_identifier: self.session_identifier,
            attempt: self.attempts_count,
        };
        let report_tx = self.new_dwallet_report_threshold_not_reached(report)?;
        let epoch_store = self.epoch_store()?.clone();
        let consensus_adapter = self.consensus_adapter.clone();
        tokio_runtime_handle.spawn(async move {
            if let Err(err) = consensus_adapter
                .submit_to_consensus(&[report_tx], &epoch_store)
                .await
            {
                error!(
                    ?err,
                    "failed to submit `threshold not reached` report to consensus"
                );
            }
        });
        Ok(())
    }

    fn advance_specific_party(
        &self,
    ) -> DwalletMPCResult<
        AsynchronousRoundResult<MPCMessage, MPCPrivateOutput, SerializedWrappedMPCPublicOutput>,
    > {
        let Some(mpc_event_data) = &self.mpc_event_data else {
            return Err(DwalletMPCError::MissingEventDrivenData);
        };
        let serialized_messages_skeleton = self
            .serialized_full_messages
            .iter()
            .map(|(round, messages_map)| {
                (
                    *round,
                    messages_map.keys().copied().sorted().collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<_, _>>();
        info!(
            mpc_protocol=?mpc_event_data.init_protocol_data,
            validator=?self.epoch_store()?.name,
            session_identifier=?self.session_identifier,
            crypto_round=?self.current_round,
            weighted_parties=?self.weighted_threshold_access_structure,
            ?serialized_messages_skeleton,
            "Advancing MPC session"
        );
        let session_identifier =
            CommitmentSizedNumber::from_le_slice(&self.session_identifier.into_bytes());
        let party_to_authority_map = self.epoch_store()?.committee().party_to_authority_map();
        let mpc_protocol_name = mpc_event_data.init_protocol_data.to_string();

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
            self.party_id as u64,
            self.current_round as u64,
            self.attempts_count as u64,
            self.epoch_id as u64,
        );

        match &mpc_event_data.init_protocol_data {
            MPCProtocolInitData::DWalletImportedKeyVerificationRequest(event_data) => {
                let PublicInput::DWalletImportedKeyVerificationRequest(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.init_protocol_data,
                        validator=?self.epoch_store()?.name,
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
                    self.serialized_full_messages.clone(),
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
                                dwallet_network_decryption_key_id: event_data
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
            MPCProtocolInitData::DKGFirst(..) => {
                info!(
                    mpc_protocol=?mpc_event_data.init_protocol_data,
                    validator=?self.epoch_store()?.name,
                    session_identifier=?self.session_identifier,
                    crypto_round=?self.current_round,
                    "Advancing DKG first party",
                );
                let PublicInput::DKGFirst(public_input) = &mpc_event_data.public_input else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?mpc_event_data.init_protocol_data,
                        validator=?self.epoch_store()?.name,
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
                    self.serialized_full_messages.clone(),
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
            MPCProtocolInitData::DKGSecond(event_data) => {
                let PublicInput::DKGSecond(public_input) = &mpc_event_data.public_input else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.init_protocol_data,
                        validator=?self.epoch_store()?.name,
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
                    self.serialized_full_messages.clone(),
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
                            dwallet_network_decryption_key_id: event_data
                                .event_data
                                .dwallet_network_decryption_key_id,
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
            MPCProtocolInitData::Presign(..) => {
                let PublicInput::Presign(public_input) = &mpc_event_data.public_input else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?mpc_event_data.init_protocol_data,
                        validator=?self.epoch_store()?.name,
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
                    self.serialized_full_messages.clone(),
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
            MPCProtocolInitData::Sign(..) => {
                let decryption_key_shares = mpc_event_data
                    .decryption_shares
                    .iter()
                    .map(|(party_id, share)| (*party_id, share.decryption_key_share))
                    .collect::<HashMap<_, _>>();

                // Extend base logger with decryption key shares for Sign protocol
                let logger = base_logger.with_decryption_key_shares(decryption_key_shares.clone());
                let PublicInput::Sign(public_input) = &mpc_event_data.public_input else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.init_protocol_data,
                        validator=?self.epoch_store()?.name,
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
                    self.serialized_full_messages.clone(),
                    public_input,
                    mpc_event_data.decryption_shares.clone(),
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
                        let public_output = bcs::to_bytes(&VersionedSignOutput::V1(public_output))?;
                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        })
                    }
                    _ => result,
                }
            }
            MPCProtocolInitData::NetworkEncryptionKeyDkg(key_scheme, _init_event) => {
                advance_network_dkg(
                    session_identifier,
                    &self.weighted_threshold_access_structure,
                    &self.mpc_event_data.clone().unwrap(),
                    self.party_id,
                    key_scheme,
                    self.serialized_full_messages.clone(),
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
            MPCProtocolInitData::EncryptedShareVerification(verification_data) => {
                let PublicInput::EncryptedShareVerification(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.init_protocol_data,
                        validator=?self.epoch_store()?.name,
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
            MPCProtocolInitData::PartialSignatureVerification(event_data) => {
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
                        mpc_protocol=?mpc_event_data.init_protocol_data,
                        validator=?self.epoch_store()?.name,
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
            MPCProtocolInitData::NetworkEncryptionKeyReconfiguration(_) => {
                let PublicInput::NetworkEncryptionKeyReconfiguration(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.init_protocol_data,
                        validator=?self.epoch_store()?.name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_round,
                        weighted_parties=?self.weighted_threshold_access_structure,
                        ?serialized_messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };
                let decryption_key_shares = mpc_event_data
                    .decryption_shares
                    .iter()
                    .map(|(party_id, share)| (*party_id, share.decryption_key_share))
                    .collect::<HashMap<_, _>>();

                // Extend base logger with decryption key shares for Reconfiguration protocol
                let logger = base_logger.with_decryption_key_shares(decryption_key_shares.clone());

                let result = advance_and_serialize::<ReconfigurationSecp256k1Party>(
                    session_identifier,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_full_messages.clone(),
                    public_input,
                    decryption_key_shares.clone(),
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
            }
            MPCProtocolInitData::MakeDWalletUserSecretKeySharesPublicRequest(init_event) => {
                let PublicInput::MakeDWalletUserSecretKeySharesPublicPublicInput(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.init_protocol_data,
                        validator=?self.epoch_store()?.name,
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
                            validator=?self.epoch_store()?.name,
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
        let session_info = SessionInfo {
            session_type: mpc_event_data.session_type.clone(),
            mpc_round: mpc_event_data.init_protocol_data.clone(),
            epoch: self.epoch_id,
            session_identifier: self.session_identifier,
        };
        Ok(ConsensusTransaction::new_dwallet_mpc_message(
            self.epoch_store()?.name,
            message,
            self.session_identifier,
            self.current_round,
            mpc_protocol.to_string(),
            session_info,
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
                self.epoch_store()?.name,
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
                self.epoch_store()?.name,
                report,
            ),
        )
    }

    /// Stores a message in the serialized messages map.
    /// Every new message received for a session is stored.
    /// When a threshold of messages is reached, the session advances.
    pub(crate) fn store_message(&mut self, message: &DWalletMPCMessage) -> DwalletMPCResult<()> {
        // This happens because we clear the session when it is finished and change the status,
        // so we might receive a message with delay, and it's irrelevant.
        if self.status != MPCSessionStatus::Active {
            warn!(
                session_id=?message.session_identifier,
                from_authority=?message.authority,
                receiving_authority=?self.epoch_store()?.name,
                crypto_round_number=?message.round_number,
                mpc_protocol=%message.mpc_protocol,
                "Received a message for a session that is not active",
            );
            return Ok(());
        }
        let committee = self.epoch_store()?.committee().clone();
        if self.agreed_mpc_protocol.is_none() {
            let mpc_protocol = message.mpc_protocol.clone();
            if self
                .mpc_protocol_to_voting_authorities
                .entry(mpc_protocol.clone())
                .or_insert(StakeAggregator::new(committee))
                .insert_generic(message.authority, ())
                .is_quorum_reached()
            {
                self.agreed_mpc_protocol = Some(mpc_protocol);
            }
        }
        info!(
            session_id=?message.session_identifier,
            from_authority=?message.authority,
            receiving_authority=?self.epoch_store()?.name,
            crypto_round_number=?message.round_number,
            message_size_bytes=?message.message.len(),
            mpc_protocol=message.mpc_protocol,
            messages_count_for_current_round=?self.serialized_full_messages.get(&(self.current_round - 1)).unwrap_or(&HashMap::new()).len(),
            "Received a dWallet MPC message",
        );
        if message.round_number == 0 {
            error!(
                session_id=?message.session_identifier,
                from_authority=?message.authority,
                receiving_authority=?self.epoch_store()?.name,
                crypto_round_number=?message.round_number,
                mpc_protocol=?message.mpc_protocol,
                "Received a message for round zero",
            );
            return Err(DwalletMPCError::MessageForFirstMPCStep);
        }
        let source_party_id = self
            .epoch_store()?
            .authority_name_to_party_id(&message.authority)?;
        // We should only receive outputs of previous rounds.
        if message.round_number > self.current_round {
            warn!(
                session_id=?message.session_identifier,
                from_authority=?message.authority,
                receiving_authority=?self.epoch_store()?.name,
                recieved_message_round_number=?message.round_number,
                "Received a message for a future round",
            );
            return Err(DwalletMPCError::MaliciousParties(vec![source_party_id]));
        }
        let round_messages_map = self
            .serialized_full_messages
            .entry(message.round_number)
            .or_default();
        if round_messages_map.contains_key(&source_party_id) {
            return Ok(());
        }
        round_messages_map.insert(source_party_id, message.message.clone());
        self.received_more_messages_since_last_advance = true;
        Ok(())
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
    fn wait_consensus_rounds_delay(&mut self) -> DwalletMPCResult<ReadyToAdvanceCheckResult> {
        match self.agreed_mpc_protocol.as_deref() {
            None => Ok(ReadyToAdvanceCheckResult {
                is_ready: false,
                malicious_parties: vec![],
            }),
            Some(protocol) => match protocol {
                NETWORK_ENCRYPTION_KEY_DKG_STR_KEY => {
                    let delay = self
                        .epoch_store()?
                        .protocol_config()
                        .network_dkg_third_round_delay() as usize;
                    self.check_round_delay(Self::NETWORK_DKG_DELAY_ROUND, delay)
                }
                NETWORK_ENCRYPTION_KEY_RECONFIGURATION_STR_KEY => {
                    let delay = self
                        .epoch_store()?
                        .protocol_config()
                        .decryption_key_reconfiguration_third_round_delay()
                        as usize;
                    self.check_round_delay(Self::DECRYPTION_KEY_RECONFIGURATION_DELAY_ROUND, delay)
                }
                _ => Ok(ReadyToAdvanceCheckResult {
                    is_ready: true,
                    malicious_parties: vec![],
                }),
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
    ) -> DwalletMPCResult<ReadyToAdvanceCheckResult> {
        if self.current_round != target_round {
            return Ok(ReadyToAdvanceCheckResult {
                is_ready: true,
                malicious_parties: vec![],
            });
        }
        if self.consensus_rounds_since_quorum_reached >= required_consensus_rounds_delay {
            info!(
                ?self.consensus_rounds_since_quorum_reached,
                ?self.current_round,
                ?self.agreed_mpc_protocol,
                ?self.session_identifier,
                messages_count_for_current_round=?self.serialized_full_messages.get(&(self.current_round - 1)).unwrap_or(&HashMap::new()).len(),
                "Quorum reached for MPC session and delay passed, advancing to next round",
            );
            self.consensus_rounds_since_quorum_reached = 0;
            Ok(ReadyToAdvanceCheckResult {
                is_ready: true,
                malicious_parties: vec![],
            })
        } else {
            info!(
                ?self.consensus_rounds_since_quorum_reached,
                ?self.current_round,
                ?self.agreed_mpc_protocol,
                messages_count_for_current_round=?self.serialized_full_messages.get(&(self.current_round - 1)).unwrap_or(&HashMap::new()).len(),
                "Quorum reached for MPC session but delay not passed yet, waiting for another round",
            );
            self.consensus_rounds_since_quorum_reached += 1;
            Ok(ReadyToAdvanceCheckResult {
                is_ready: false,
                malicious_parties: vec![],
            })
        }
    }

    pub(crate) fn check_quorum_for_next_crypto_round(
        &mut self,
    ) -> DwalletMPCResult<ReadyToAdvanceCheckResult> {
        match self.status {
            MPCSessionStatus::Active => {
                // MPC First round doesn't require a threshold of messages to advance.
                // This is the round after the MPC event.
                let is_quorum_reached = self
                    .weighted_threshold_access_structure
                    .is_authorized_subset(
                        &self
                            .serialized_full_messages
                            // Check if we have the threshold of messages for the previous round
                            // to advance to the next round.
                            .get(&(self.current_round - 1))
                            .unwrap_or(&HashMap::new())
                            .keys()
                            .cloned()
                            .collect::<HashSet<PartyID>>(),
                    )
                    .is_ok();
                // Round 1 does not have a delay.
                if self.current_round == 1 {
                    Ok(ReadyToAdvanceCheckResult {
                        is_ready: true,
                        malicious_parties: vec![],
                    })
                } else if is_quorum_reached
                    && self.received_more_messages_since_last_advance
                    && self.agreed_mpc_protocol.is_some()
                {
                    self.wait_consensus_rounds_delay()
                } else {
                    Ok(ReadyToAdvanceCheckResult {
                        is_ready: false,
                        malicious_parties: vec![],
                    })
                }
            }
            _ => Ok(ReadyToAdvanceCheckResult {
                is_ready: false,
                malicious_parties: vec![],
            }),
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
                self.serialized_full_messages
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
