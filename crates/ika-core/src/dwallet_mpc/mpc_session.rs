use class_groups::dkg::Secp256k1Party;
use commitment::CommitmentSizedNumber;
use crypto_bigint::Uint;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCMessage, MPCPrivateInput, MPCPublicInput, MPCSessionStatus,
};
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
use crate::dwallet_mpc::dkg::{DKGFirstParty, DKGSecondParty};
use crate::dwallet_mpc::encrypt_user_share::verify_encrypted_share;
use crate::dwallet_mpc::network_dkg::{advance_network_dkg, DwalletMPCNetworkKeys};
use crate::dwallet_mpc::presign::PresignParty;
use crate::dwallet_mpc::reshare::ReshareSecp256k1Party;
use crate::dwallet_mpc::sign::{verify_partial_signature, SignFirstParty};
use crate::dwallet_mpc::{
    message_digest, party_id_to_authority_name, party_ids_to_authority_names, presign,
};
use ika_types::committee::StakeUnit;
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_consensus::ConsensusTransaction;
use ika_types::messages_dwallet_mpc::{
    AdvanceResult, DWalletMPCMessage, MPCProtocolInitData, MaliciousReport, PresignSessionState,
    SessionInfo, StartEncryptedShareVerificationEvent, StartPresignFirstRoundEvent,
};
use sui_types::base_types::{EpochId, ObjectID};
use sui_types::id::ID;

pub(crate) type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;

pub const FAILED_SESSION_OUTPUT: [u8; 1] = [1];

/// The result of the check if the session is ready to advance.
///
/// Returns whether the session is ready to advance or not, and a list of the malicious parties that were detected
/// while performing the check.
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
    pub(crate) decryption_share: HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>,
    pub(crate) is_immediate: bool,
}

/// A dWallet MPC session.
/// It keeps track of the session, the channel to send messages to the session,
/// and the messages that are pending to be sent to the session.
// TODO (#539): Simplify struct to only contain session related data.
#[derive(Clone)]
pub(super) struct DWalletMPCSession {
    /// The status of the MPC session.
    pub(super) status: MPCSessionStatus,
    /// All the messages that have been received for this session.
    /// We need to accumulate a threshold of those before advancing the session.
    /// Vec[Round1: Map{Validator1->Message, Validator2->Message}, Round2: Map{Validator1->Message} ...]
    pub(super) serialized_full_messages: Vec<HashMap<PartyID, MPCMessage>>,
    epoch_store: Weak<AuthorityPerEpochStore>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    epoch_id: EpochId,
    pub(super) session_id: ObjectID,
    /// The current MPC round number of the session.
    /// Starts at 0 and increments by one each time we advance the session.
    pub(super) pending_quorum_for_highest_round_number: usize,
    party_id: PartyID,
    // TODO (#539): Simplify struct to only contain session related data - remove this field.
    weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    pub(crate) mpc_event_data: Option<MPCEventData>,
    pub(crate) sequence_number: u64,
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
        sequence_number: u64,
    ) -> Self {
        Self {
            status,
            serialized_full_messages: vec![HashMap::new()],
            consensus_adapter,
            epoch_store: epoch_store.clone(),
            epoch_id: epoch,
            session_id,
            pending_quorum_for_highest_round_number: 0,
            party_id,
            weighted_threshold_access_structure,
            mpc_event_data,
            sequence_number,
        }
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
                info!(
                    // Safe to unwrap as advance can only be called after the event is received.
                    mpc_protocol=?self.mpc_event_data.clone().unwrap().init_protocol_data,
                    session_id=?self.session_id,
                    validator=?self.epoch_store()?.name,
                    round=?self.serialized_full_messages.len(),
                    "Advanced MPC session"
                );
                let consensus_adapter = self.consensus_adapter.clone();
                let epoch_store = self.epoch_store()?.clone();
                if !malicious_parties.is_empty() {
                    self.report_malicious_actors(
                        tokio_runtime_handle,
                        malicious_parties,
                        AdvanceResult::Success,
                    )?;
                }
                let message = self.new_dwallet_mpc_message(message)?;
                tokio_runtime_handle.spawn(async move {
                    if let Err(err) = consensus_adapter
                        .submit_to_consensus(&vec![message], &epoch_store)
                        .await
                    {
                        error!("failed to submit an MPC message to consensus: {:?}", err);
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
                    self.report_malicious_actors(
                        tokio_runtime_handle,
                        malicious_parties,
                        AdvanceResult::Success,
                    )?;
                }
                let consensus_message = self
                    .new_dwallet_mpc_output_message(public_output.clone(), self.sequence_number)?;
                let session_id_clone = self.session_id;
                tokio_runtime_handle.spawn(async move {
                    if let Err(err) = consensus_adapter
                        .submit_to_consensus(&vec![consensus_message], &epoch_store)
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
            Err(DwalletMPCError::SessionFailedWithMaliciousParties(malicious_parties)) => {
                error!(
                    "session failed with malicious parties: {:?}",
                    malicious_parties
                );
                self.report_malicious_actors(
                    tokio_runtime_handle,
                    malicious_parties,
                    AdvanceResult::Failure,
                )
            }
            Err(e) => {
                let validator_name = self.epoch_store()?.name;
                // Safe to unwrap as advance can only be called after the event is received.
                let mpc_protocol = self.mpc_event_data.clone().unwrap().init_protocol_data;
                error!(
                    mpc_protocol=?&mpc_protocol,
                    session_id=?self.session_id,
                    validator=?validator_name,
                    epoch=?self.epoch_id,
                    error=?e,
                    "failed to advance the MPC session"
                );
                let consensus_adapter = self.consensus_adapter.clone();
                let epoch_store = self.epoch_store()?.clone();
                let consensus_message = self.new_dwallet_mpc_output_message(
                    FAILED_SESSION_OUTPUT.to_vec(),
                    self.sequence_number,
                )?;
                let session_id_clone = self.session_id;
                let epoch_id_clone = self.epoch_id;

                tokio_runtime_handle.spawn(async move {
                    if let Err(err) = consensus_adapter
                        .submit_to_consensus(&vec![consensus_message], &epoch_store)
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
                Err(e)
            }
        }
    }

    /// Create a new consensus transaction with the flow result (output) to be
    /// sent to the other MPC parties.
    /// Errors if the epoch was switched in the middle and was not available.
    fn new_dwallet_mpc_output_message(
        &self,
        output: Vec<u8>,
        sequence_number: u64,
    ) -> DwalletMPCResult<ConsensusTransaction> {
        let Some(mpc_event_data) = &self.mpc_event_data else {
            return Err(DwalletMPCError::MissingEventDrivenData);
        };
        Ok(ConsensusTransaction::new_dwallet_mpc_output(
            self.epoch_store()?.name,
            output,
            SessionInfo {
                sequence_number,
                session_id: self.session_id.clone(),
                mpc_round: mpc_event_data.init_protocol_data.clone(),
                is_immediate: false,
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
        advance_result: AdvanceResult,
    ) -> DwalletMPCResult<()> {
        let report = MaliciousReport::new(
            party_ids_to_authority_names(&malicious_parties_ids, &*self.epoch_store()?)?,
            self.session_id.clone(),
            advance_result,
        );
        let report_tx = self.new_dwallet_report_failed_session_with_malicious_actors(report)?;
        let epoch_store = self.epoch_store()?.clone();
        let consensus_adapter = self.consensus_adapter.clone();
        tokio_runtime_handle.spawn(async move {
            if let Err(err) = consensus_adapter
                .submit_to_consensus(&vec![report_tx], &epoch_store)
                .await
            {
                error!("failed to submit an MPC message to consensus: {:?}", err);
            }
        });
        Ok(())
    }

    fn dwallet_mpc_network_keys(&self) -> DwalletMPCResult<Arc<DwalletMPCNetworkKeys>> {
        Ok(self
            .epoch_store()?
            .dwallet_mpc_network_keys
            .get()
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .clone())
    }

    fn advance_specific_party(
        &self,
    ) -> DwalletMPCResult<AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>> {
        let Some(mpc_event_data) = &self.mpc_event_data else {
            return Err(DwalletMPCError::MissingEventDrivenData);
        };
        info!(
            mpc_protocol=?mpc_event_data.init_protocol_data,
            validator=?self.epoch_store()?.name,
            session_id=?self.session_id,
            crypto_round=?self.pending_quorum_for_highest_round_number,
            "Advancing MPC session"
        );
        let session_id = CommitmentSizedNumber::from_le_slice(self.session_id.to_vec().as_slice());
        let public_input = &mpc_event_data.public_input;
        match &mpc_event_data.init_protocol_data {
            MPCProtocolInitData::DKGFirst(..) => {
                info!(
                    mpc_protocol=?mpc_event_data.init_protocol_data,
                    validator=?self.epoch_store()?.name,
                    session_id=?self.session_id,
                    crypto_round=?self.pending_quorum_for_highest_round_number,
                    "Advancing DKG first party",
                );
                let public_input = bcs::from_bytes(public_input)?;
                crate::dwallet_mpc::advance_and_serialize::<DKGFirstParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_full_messages.clone(),
                    public_input,
                    (),
                )
            }
            MPCProtocolInitData::DKGSecond(event_data) => {
                let public_input: <DKGSecondParty as mpc::Party>::PublicInput =
                    bcs::from_bytes(public_input)?;
                let result = crate::dwallet_mpc::advance_and_serialize::<DKGSecondParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_full_messages.clone(),
                    public_input.clone(),
                    (),
                )?;
                if let AsynchronousRoundResult::Finalize { public_output, .. } = &result {
                    verify_encrypted_share(
                        &StartEncryptedShareVerificationEvent {
                            decentralized_public_output: public_output.clone(),
                            encrypted_centralized_secret_share_and_proof: event_data
                                .event_data
                                .encrypted_centralized_secret_share_and_proof
                                .clone(),
                            encryption_key: event_data.event_data.encryption_key.clone(),
                            encryption_key_id: event_data.event_data.encryption_key_id.clone(),
                            dwallet_mpc_network_key_id: event_data
                                .event_data
                                .dwallet_mpc_network_key_id
                                .clone(),

                            // Fields not relevant for verification; passing empty values.
                            dwallet_id: ObjectID::new([0; 32]),
                            source_encrypted_user_secret_key_share_id: ObjectID::new([0; 32]),
                            encrypted_user_secret_key_share_id: ObjectID::new([0; 32]),
                        },
                        &bcs::to_bytes(&public_input.protocol_public_parameters)?,
                    )?;
                }
                Ok(result)
            }
            MPCProtocolInitData::Presign(..) => {
                let public_input = bcs::from_bytes(public_input)?;
                crate::dwallet_mpc::advance_and_serialize::<PresignParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_full_messages.clone(),
                    public_input,
                    (),
                )
            }
            MPCProtocolInitData::Sign(..) => {
                let public_input = bcs::from_bytes(public_input)?;
                crate::dwallet_mpc::advance_and_serialize::<SignFirstParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_full_messages.clone(),
                    public_input,
                    mpc_event_data.decryption_share.clone(),
                )
            }
            MPCProtocolInitData::NetworkDkg(key_scheme, _init_event) => advance_network_dkg(
                session_id,
                &self.weighted_threshold_access_structure,
                self.party_id,
                public_input,
                key_scheme,
                self.serialized_full_messages.clone(),
                bcs::from_bytes(
                    &mpc_event_data
                        .private_input
                        .clone()
                        .ok_or(DwalletMPCError::MissingMPCPrivateInput)?,
                )?,
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
                    &bcs::from_bytes(public_input)?,
                )?;

                Ok(AsynchronousRoundResult::Finalize {
                    public_output: vec![],
                    private_output: vec![],
                    malicious_parties: vec![],
                })
            }
            MPCProtocolInitData::DecryptionKeyReshare(_) => {
                let public_input = bcs::from_bytes(public_input)?;
                let decryption_key_shares = mpc_event_data
                    .decryption_share
                    .iter()
                    .map(|(party_id, share)| (*party_id, share.decryption_key_share))
                    .collect::<HashMap<_, _>>();
                crate::dwallet_mpc::advance_and_serialize::<ReshareSecp256k1Party>(
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
                        decryption_key_shares,
                    ),
                )
            }
            _ => {
                unreachable!("Unsupported MPC protocol type")
            }
        }
    }

    /// Create a new consensus transaction with the message to be sent to the other MPC parties.
    /// Returns Error only if the epoch switched in the middle and was not available.
    fn new_dwallet_mpc_message(
        &self,
        message: MPCMessage,
    ) -> DwalletMPCResult<ConsensusTransaction> {
        Ok(ConsensusTransaction::new_dwallet_mpc_message(
            self.epoch_store()?.name,
            message,
            self.session_id.clone(),
            self.pending_quorum_for_highest_round_number,
            self.sequence_number,
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

    /// Stores a message in the serialized messages map.
    /// Every new message received for a session is stored.
    /// When a threshold of messages is reached, the session advances.
    pub(crate) fn store_message(&mut self, message: &DWalletMPCMessage) -> DwalletMPCResult<()> {
        // This happens because we clear the session when it is finished and change the status,
        // so we might receive a message with delay, and it's irrelevant.
        if self.status != MPCSessionStatus::Active {
            warn!(
                session_id=?message.session_id,
                from_authority=?message.authority,
                receiving_authority=?self.epoch_store()?.name,
                crypto_round_number=?message.round_number,
                "Received a message for a session that is not active",
            );
            return Ok(());
        }
        // TODO (#876): Set the maximum message size to the smallest size possible.
        // todo(zeev): add protocol
        info!(
            session_id=?message.session_id,
            from_authority=?message.authority,
            receiving_authority=?self.epoch_store()?.name,
            crypto_round_number=?message.round_number,
            message_size_bytes=?message.message.len(),
            "Received a dWallet MPC message",
        );
        if message.round_number == 0 {
            error!(
                session_id=?message.session_id,
                from_authority=?message.authority,
                receiving_authority=?self.epoch_store()?.name,
                crypto_round_number=?message.round_number,
                "Received a message for round zero",
            );
            return Err(DwalletMPCError::MessageForFirstMPCStep);
        }
        let source_party_id = self
            .epoch_store()?
            .authority_name_to_party_id(&message.authority)?;
        let current_round = self.serialized_full_messages.len();

        let authority_name = self.epoch_store()?.name;

        match self.serialized_full_messages.get_mut(message.round_number) {
            Some(party_to_msg) => {
                if party_to_msg.contains_key(&source_party_id) {
                    error!(
                        session_id=?message.session_id,
                        from_authority=?message.authority,
                        receiving_authority=?authority_name,
                        crypto_round_number=?message.round_number,
                        "Received a duplicate message from authority",
                    );
                    // Duplicate.
                    // This should never happen, as the consensus uniqueness key contains only the origin authority,
                    // session ID and MPC round.
                    return Ok(());
                }
                info!(
                    session_id=?message.session_id,
                    from_authority=?message.authority,
                    receiving_authority=?authority_name,
                    crypto_round_number=?message.round_number,
                    "Inserting an MPC message into the party to message maps",
                );
                party_to_msg.insert(source_party_id, message.message.clone());
            }
            None if message.round_number == current_round => {
                info!(
                    session_id=?message.session_id,
                    from_authority=?message.authority,
                    receiving_authority=?authority_name,
                    crypto_round_number=?message.round_number,
                    "Storing a message for the current round",
                );
                let mut map = HashMap::new();
                map.insert(source_party_id, message.message.clone());
                self.serialized_full_messages.push(map);
            }
            // Received a message for a future round (above the current round).
            // If it happens, there is an issue with the consensus.
            None => {
                error!(
                    session_id=?message.session_id,
                    from_authority=?message.authority,
                    receiving_authority=?authority_name,
                    crypto_round_number=?message.round_number,
                    "Received a message for two or more rounds above known round for session",
                );
                // Unexpected round number; rounds should grow sequentially.
                return Err(DwalletMPCError::MaliciousParties(vec![source_party_id]));
            }
        }
        Ok(())
    }

    pub(crate) fn check_quorum_for_next_crypto_round(&self) -> ReadyToAdvanceCheckResult {
        match self.status {
            MPCSessionStatus::Active => {
                if self.pending_quorum_for_highest_round_number == 0
                    || self
                        .weighted_threshold_access_structure
                        .is_authorized_subset(
                            &self
                                .serialized_full_messages
                                .get(self.pending_quorum_for_highest_round_number)
                                .unwrap_or(&HashMap::new())
                                .keys()
                                .cloned()
                                .collect::<HashSet<PartyID>>(),
                        )
                        .is_ok()
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

    /// Helper function to spawn a task for submitting messages to consensus.
    fn spawn_submit_to_consensus(
        tokio_runtime_handle: &Handle,
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        messages: Vec<ConsensusTransaction>,
    ) {
        tokio_runtime_handle.spawn(async move {
            for msg in messages {
                if let Err(err) = consensus_adapter
                    .submit_to_consensus(&vec![msg], &epoch_store)
                    .await
                {
                    error!("failed to submit an MPC message to consensus: {:?}", err);
                }
            }
        });
    }
}
