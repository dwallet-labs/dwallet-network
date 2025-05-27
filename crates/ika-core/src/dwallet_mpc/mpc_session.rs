use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{
    MPCMessage, MPCPrivateInput, MPCPrivateOutput, MPCPublicInput, MPCSessionPublicOutput,
    MPCSessionStatus, SerializedWrappedMPCPublicOutput,
    VersionedDWalletImportedKeyVerificationOutput, VersionedDecryptionKeyReshareOutput,
    VersionedDwalletDKGFirstRoundPublicOutput, VersionedDwalletDKGSecondRoundPublicOutput,
    VersionedPresignOutput, VersionedSignOutput,
};
use group::helpers::DeduplicateAndSort;
use group::PartyID;
use mpc::{AsynchronousRoundResult, WeightedThresholdAccessStructure};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Weak};
use tokio::runtime::Handle;
use tracing::{error, info, warn};
use twopc_mpc::sign::Protocol;

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::dkg::{DKGFirstParty, DKGSecondParty, DWalletImportedKeyVerificationParty};
use crate::dwallet_mpc::encrypt_user_share::verify_encrypted_share;
use crate::dwallet_mpc::make_dwallet_user_secret_key_shares_public::verify_secret_share;
use crate::dwallet_mpc::network_dkg::advance_network_dkg;
use crate::dwallet_mpc::presign::PresignParty;
use crate::dwallet_mpc::reshare::ReshareSecp256k1Party;
use crate::dwallet_mpc::sign::{verify_partial_signature, SignFirstParty};
use crate::dwallet_mpc::{message_digest, party_ids_to_authority_names};
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_consensus::ConsensusTransaction;
use ika_types::messages_dwallet_mpc::{
    DWalletMPCMessage, EncryptedShareVerificationRequestEvent, MPCProtocolInitData,
    MaliciousReport, SessionInfo, SessionType, ThresholdNotReachedReport,
};
use sui_types::base_types::{EpochId, ObjectID};

pub(crate) type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;

/// Represents the result of checking whether the session is ready to advance.
///
/// This structure contains a flag indicating if the session is ready to advance,
/// and a list of malicious parties detected during the check.
pub(crate) struct ReadyToAdvanceCheckResult {
    pub(crate) is_ready: bool,
    pub(crate) malicious_parties: Vec<PartyID>,
}

/// The DWallet MPC session data that is based on the event that initiated the session.
#[derive(Clone)]
pub struct MPCEventData {
    pub private_input: MPCPrivateInput,
    pub(super) public_input: MPCPublicInput,
    pub init_protocol_data: MPCProtocolInitData,
    pub(crate) decryption_shares: HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>,
    pub(crate) session_type: SessionType,
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
    pub(super) session_id: ObjectID,
    /// The current MPC round number of the session.
    /// Starts at `1` and increments after each advance of the session.
    /// In round `1` We start the flow, without messages, from the event trigger.
    /// Decremented only upon an `TWOPCMPCThresholdNotReached` Error.
    pub(super) current_round: usize,
    party_id: PartyID,
    // TODO (#539): Simplify struct to only contain session related data - remove this field.
    weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    pub(crate) mpc_event_data: Option<MPCEventData>,
    pub(crate) received_more_messages_since_last_advance: bool,
    // The *total* number of attempts to advance that failed in the session.
    // Used to make `ThresholdNotReachedReport` unique.
    pub(crate) attempts_count: usize,
}

impl DWalletMPCSession {
    pub(crate) fn new(
        epoch_store: Weak<AuthorityPerEpochStore>,
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch: EpochId,
        status: MPCSessionStatus,
        session_id: ObjectID,
        party_id: PartyID,
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        mpc_event_data: Option<MPCEventData>,
    ) -> Self {
        Self {
            status,
            serialized_full_messages: HashMap::new(),
            consensus_adapter,
            epoch_store: epoch_store.clone(),
            epoch_id: epoch,
            session_id,
            current_round: 1,
            party_id,
            weighted_threshold_access_structure,
            mpc_event_data,
            received_more_messages_since_last_advance: false,
            attempts_count: 0,
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
    pub(super) fn advance(&self, tokio_runtime_handle: &Handle) -> DwalletMPCResult<()> {
        match self.advance_specific_party() {
            Ok(AsynchronousRoundResult::Advance {
                malicious_parties,
                message,
            }) => {
                // Safe to unwrap as advance can only be called after the event is received.
                let mpc_protocol = self.mpc_event_data.clone().unwrap().init_protocol_data;
                let session_id = self.session_id;
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
                // Safe to unwrap as advance can only be called after the event is received.
                let mpc_protocol = self.mpc_event_data.clone().unwrap().init_protocol_data;
                info!(
                    mpc_protocol=?&mpc_protocol,
                    session_id=?self.session_id,
                    validator=?&validator_name,
                    "Reached public output (Finalize) for session"
                );
                let consensus_adapter = self.consensus_adapter.clone();
                let epoch_store = self.epoch_store()?.clone();
                if !malicious_parties.is_empty() {
                    warn!(
                        mpc_protocol=?&mpc_protocol,
                        session_id=?self.session_id,
                        validator=?&validator_name,
                        ?malicious_parties,
                        "Malicious Parties detected on MPC session Finalize",
                    );
                    self.report_malicious_actors(tokio_runtime_handle, malicious_parties)?;
                }
                let consensus_message = self.new_dwallet_mpc_output_message(
                    MPCSessionPublicOutput::CompletedSuccessfully(public_output.clone()),
                )?;
                let session_id_clone = self.session_id;
                tokio_runtime_handle.spawn(async move {
                    if let Err(err) = consensus_adapter
                        .submit_to_consensus(&[consensus_message], &epoch_store)
                        .await
                    {
                        error!(
                            mpc_protocol=?mpc_protocol,
                            session_id=?session_id_clone,
                            validator=?validator_name,
                            "failed to submit an MPC message to consensus: {:?}", err
                        );
                    }
                });
                Ok(())
            }
            Err(DwalletMPCError::TWOPCMPCThresholdNotReached) => {
                error!(
                    err=?DwalletMPCError::TWOPCMPCThresholdNotReached,
                    session_id=?self.session_id,
                    validator=?self.epoch_store()?.name,
                    crypto_round=?self.current_round,
                    party_id=?self.party_id,
                    "MPC session failed"
                );
                self.report_threshold_not_reached(tokio_runtime_handle)
            }
            Err(err) => {
                let mpc_protocol = self.mpc_event_data.clone().unwrap().init_protocol_data;
                let validator_name = self.epoch_store()?.name;

                error!(
                    session_id=?self.session_id,
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
                let session_id_clone = self.session_id;
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
                            "failed to submit an MPC message to consensus: {:?}", err);
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
                session_id: self.session_id,
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
            self.session_id,
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
            session_id: self.session_id,
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

        info!(
            mpc_protocol=?mpc_event_data.init_protocol_data,
            validator=?self.epoch_store()?.name,
            session_id=?self.session_id,
            crypto_round=?self.current_round,
            weighted_parties=?self.weighted_threshold_access_structure,
            "Advancing MPC session"
        );
        let session_id = CommitmentSizedNumber::from_le_slice(self.session_id.to_vec().as_slice());
        let encoded_public_input = &mpc_event_data.public_input;
        let party_to_authority_map = self.epoch_store()?.committee().party_to_authority_map();
        let mpc_protocol_name = mpc_event_data.init_protocol_data.to_string();

        match &mpc_event_data.init_protocol_data {
            MPCProtocolInitData::DWalletImportedKeyVerificationRequest(event_data) => {
                let dwallet_id = CommitmentSizedNumber::from_le_slice(
                    event_data.event_data.dwallet_id.to_vec().as_slice(),
                );
                let public_input = (
                    bcs::from_bytes(encoded_public_input)?,
                    dwallet_id,
                    bcs::from_bytes(&event_data.event_data.centralized_party_message)?,
                )
                    .into();
                let result = crate::dwallet_mpc::advance_and_serialize::<
                    DWalletImportedKeyVerificationParty,
                >(
                    // we are using the dWallet ID as a unique session identifier,
                    // as no two dWallets will ever have the same ID
                    // or be used for any other import session.
                    dwallet_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_full_messages.clone(),
                    public_input,
                    (),
                );
                match result.clone() {
                    Ok(AsynchronousRoundResult::Finalize {
                        public_output,
                        malicious_parties,
                        private_output,
                    }) => {
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
                    session_id=?self.session_id,
                    crypto_round=?self.current_round,
                    "Advancing DKG first party",
                );
                let public_input = bcs::from_bytes(encoded_public_input)?;

                let result = crate::dwallet_mpc::advance_and_serialize::<DKGFirstParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_full_messages.clone(),
                    public_input,
                    (),
                    None,
                    encoded_public_input,
                    mpc_protocol_name,
                    party_to_authority_map,
                    None,
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
                let public_input: <DKGSecondParty as mpc::Party>::PublicInput =
                    bcs::from_bytes(encoded_public_input)?;
                let result = crate::dwallet_mpc::advance_and_serialize::<DKGSecondParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_full_messages.clone(),
                    public_input.clone(),
                    (),
                    None,
                    encoded_public_input,
                    mpc_protocol_name,
                    party_to_authority_map,
                    None,
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
                        &bcs::to_bytes(&public_input.protocol_public_parameters)?,
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
                let public_input = bcs::from_bytes(encoded_public_input)?;
                let result = crate::dwallet_mpc::advance_and_serialize::<PresignParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_full_messages.clone(),
                    public_input,
                    (),
                    None,
                    encoded_public_input,
                    mpc_protocol_name,
                    party_to_authority_map,
                    None,
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
                let public_input = bcs::from_bytes(encoded_public_input)?;

                let decryption_key_shares = mpc_event_data
                    .decryption_shares
                    .iter()
                    .map(|(party_id, share)| (*party_id, share.decryption_key_share))
                    .collect::<HashMap<_, _>>();

                let result = crate::dwallet_mpc::advance_and_serialize::<SignFirstParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_full_messages.clone(),
                    public_input,
                    mpc_event_data.decryption_shares.clone(),
                    None,
                    encoded_public_input,
                    mpc_protocol_name,
                    party_to_authority_map,
                    Some(&decryption_key_shares),
                );
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
            MPCProtocolInitData::NetworkDkg(key_scheme, _init_event) => advance_network_dkg(
                session_id,
                &self.weighted_threshold_access_structure,
                self.party_id,
                encoded_public_input,
                key_scheme,
                self.serialized_full_messages.clone(),
                bcs::from_bytes(
                    &mpc_event_data
                        .private_input
                        .clone()
                        .ok_or(DwalletMPCError::MissingMPCPrivateInput)?,
                )?,
                encoded_public_input,
                mpc_protocol_name,
                party_to_authority_map,
            ),
            MPCProtocolInitData::EncryptedShareVerification(verification_data) => {
                let protocol_public_parameters = mpc_event_data.public_input.clone();
                match verify_encrypted_share(
                    &verification_data.event_data,
                    &protocol_public_parameters,
                ) {
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
                verify_partial_signature(
                    &hashed_message,
                    &event_data.event_data.dkg_output,
                    &event_data.event_data.presign,
                    &event_data.event_data.message_centralized_signature,
                    &bcs::from_bytes(encoded_public_input)?,
                )?;

                Ok(AsynchronousRoundResult::Finalize {
                    public_output: vec![],
                    private_output: vec![],
                    malicious_parties: vec![],
                })
            }
            MPCProtocolInitData::DecryptionKeyReshare(_) => {
                let public_input = bcs::from_bytes(encoded_public_input)?;
                let decryption_key_shares = mpc_event_data
                    .decryption_shares
                    .iter()
                    .map(|(party_id, share)| (*party_id, share.decryption_key_share))
                    .collect::<HashMap<_, _>>();
                let result = crate::dwallet_mpc::advance_and_serialize::<ReshareSecp256k1Party>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_full_messages.clone(),
                    public_input,
                    (
                        bcs::from_bytes(
                            &mpc_event_data
                                .private_input
                                .clone()
                                .ok_or(DwalletMPCError::MissingMPCPrivateInput)?,
                        )?,
                        decryption_key_shares.clone(),
                    ),
                    mpc_event_data.private_input.clone(),
                    encoded_public_input,
                    mpc_protocol_name,
                    party_to_authority_map,
                    Some(&decryption_key_shares),
                );
                match result.clone() {
                    Ok(AsynchronousRoundResult::Finalize {
                        public_output,
                        malicious_parties,
                        private_output,
                    }) => {
                        let public_output =
                            bcs::to_bytes(&VersionedDecryptionKeyReshareOutput::V1(public_output))?;
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
                match verify_secret_share(
                    &mpc_event_data.public_input,
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
                            session_id=?self.session_id,
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
        Ok(ConsensusTransaction::new_dwallet_mpc_message(
            self.epoch_store()?.name,
            message,
            self.session_id,
            self.current_round,
            mpc_protocol,
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
        self.received_more_messages_since_last_advance = true;
        // This happens because we clear the session when it is finished and change the status,
        // so we might receive a message with delay, and it's irrelevant.
        if self.status != MPCSessionStatus::Active {
            warn!(
                session_id=?message.session_id,
                from_authority=?message.authority,
                receiving_authority=?self.epoch_store()?.name,
                crypto_round_number=?message.round_number,
                mpc_protocol=%message.mpc_protocol,
                "Received a message for a session that is not active",
            );
            return Ok(());
        }
        // TODO (#876): Set the maximum message size to the smallest size possible.
        info!(
            session_id=?message.session_id,
            from_authority=?message.authority,
            receiving_authority=?self.epoch_store()?.name,
            crypto_round_number=?message.round_number,
            message_size_bytes=?message.message.len(),
            mpc_protocol=message.mpc_protocol,
            "Received a dWallet MPC message",
        );
        if message.round_number == 0 {
            error!(
                session_id=?message.session_id,
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
        if message.round_number >= self.current_round {
            warn!(
                session_id=?message.session_id,
                from_authority=?message.authority,
                receiving_authority=?self.epoch_store()?.name,
                crypto_round_number=?message.round_number,
                "Received a message for a future round",
            );
            return Err(DwalletMPCError::MaliciousParties(vec![source_party_id]));
        }
        self.serialized_full_messages
            .entry(message.round_number)
            .or_default()
            .insert(source_party_id, message.message.clone());
        Ok(())
    }

    pub(crate) fn check_quorum_for_next_crypto_round(&self) -> ReadyToAdvanceCheckResult {
        match self.status {
            MPCSessionStatus::Active => {
                // MPC First round doesn't require a threshold of messages to advance.
                // This is the round after the MPC event.
                if self.current_round == 1
                    || (self
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
                        .is_ok()
                        && self.received_more_messages_since_last_advance)
                {
                    ReadyToAdvanceCheckResult {
                        is_ready: true,
                        malicious_parties: vec![],
                    }
                } else {
                    ReadyToAdvanceCheckResult {
                        is_ready: false,
                        malicious_parties: vec![],
                    }
                }
            }
            _ => ReadyToAdvanceCheckResult {
                is_ready: false,
                malicious_parties: vec![],
            },
        }
    }
}
