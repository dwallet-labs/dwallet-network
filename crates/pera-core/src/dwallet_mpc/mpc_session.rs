use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{
    MPCMessage, MPCPrivateInput, MPCPublicInput, MPCSessionStatus,
};
use group::PartyID;
use mpc::{AsynchronousRoundResult, WeightedThresholdAccessStructure};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Weak};
use tokio::runtime::Handle;
use tracing::error;
use twopc_mpc::sign::Protocol;

use pera_types::base_types::{EpochId, ObjectID};
use pera_types::committee::StakeUnit;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::id::ID;
use pera_types::messages_consensus::{ConsensusTransaction, DWalletMPCMessage};
use pera_types::messages_dwallet_mpc::{
    MPCProtocolInitData, MaliciousReport, SessionInfo, StartEncryptedShareVerificationEvent,
};

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::dkg::{DKGFirstParty, DKGSecondParty};
use crate::dwallet_mpc::encrypt_user_share::{verify_encrypted_share, verify_encryption_key};
use crate::dwallet_mpc::network_dkg::advance_network_dkg;
use crate::dwallet_mpc::presign::{PresignFirstParty, PresignSecondParty};
use crate::dwallet_mpc::sign::SignFirstParty;
use crate::dwallet_mpc::{authority_name_to_party_id, party_id_to_authority_name};

pub(crate) type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;

/// A dWallet MPC session.
/// It keeps track of the session, the channel to send messages to the session,
/// and the messages that are pending to be sent to the session.
// TODO (#539): Simplify struct to only contain session related data.
#[derive(Clone)]
pub(super) struct DWalletMPCSession {
    /// The status of the MPC session.
    pub(super) status: MPCSessionStatus,
    /// The messages that are pending to be executed while advancing the session
    /// We need to accumulate a threshold of those before advancing the session.
    /// Vec[Round1: Map{Validator1->Message, Validator2->Message}, Round2: Map{Validator1->Message} ...]
    pub(super) pending_messages: Vec<HashMap<PartyID, MPCMessage>>,
    epoch_store: Weak<AuthorityPerEpochStore>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    epoch_id: EpochId,
    /// The total number of parties in the chain
    /// We can calculate the threshold and parties IDs (indexes) from it.
    /// To calculate the party's ID, all we need to know is the number of parties,
    /// as the IDs are just the indexes of those parties.
    /// If there are three parties, the IDs are [0, 1, 2].
    pub(super) session_info: SessionInfo,
    pub(super) public_input: MPCPublicInput,
    /// The current MPC round number of the session.
    /// Starts at 0 and increments by one each time we advance the session.
    pub(super) pending_quorum_for_highest_round_number: usize,
    party_id: PartyID,
    // TODO (#539): Simplify struct to only contain session related data - remove this field.
    weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    // TODO (#539): Simplify struct to only contain session related data - remove this field.
    decryption_share: HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>,
    // TODO (#539): Simplify struct to only contain session related data - remove this field.
    private_input: MPCPrivateInput,
}

// todo remove
/// Needed to be able to iterate over a vector of generic DWalletMPCSession with Rayon.
unsafe impl Send for DWalletMPCSession {}

impl DWalletMPCSession {
    pub(crate) fn new(
        epoch_store: Weak<AuthorityPerEpochStore>,
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch: EpochId,
        status: MPCSessionStatus,
        public_input: MPCPublicInput,
        session_info: SessionInfo,
        party_id: PartyID,
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        decryption_share: HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>,
        private_input: MPCPrivateInput,
        pending_messages: Vec<HashMap<PartyID, MPCMessage>>,
    ) -> Self {
        Self {
            status,
            pending_messages,
            consensus_adapter,
            epoch_store: epoch_store.clone(),
            epoch_id: epoch,
            public_input,
            session_info,
            pending_quorum_for_highest_round_number: 0,
            party_id,
            weighted_threshold_access_structure,
            decryption_share,
            private_input,
        }
    }

    fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }

    /// Advances the MPC session and sends the advancement result to the other validators.
    /// The consensus submission logic is being spawned as a separate tokio task, as it's an IO
    /// heavy task. Rayon, which is good for CPU heavy tasks, is used to perform the cryptographic
    /// computation, and Tokio, which is good for IO heavy tasks, is used to submit the result to
    /// the consensus.
    pub(super) fn advance(&self, tokio_runtime_handle: &Handle) -> DwalletMPCResult<()> {
        match self.advance_specific_party() {
            Ok(AsynchronousRoundResult::Advance {
                malicious_parties: _malicious_parties,
                message,
            }) => {
                let message = self.new_dwallet_mpc_message(message).map_err(|e| {
                    DwalletMPCError::MPCSessionError {
                        session_id: self.session_info.session_id,
                        error: format!("failed to create a new MPC message on advance(): {:?}", e),
                    }
                })?;
                let consensus_adapter = self.consensus_adapter.clone();
                let epoch_store = self.epoch_store()?.clone();
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
                let output = self.new_dwallet_mpc_output_message(public_output)?;
                let consensus_adapter = self.consensus_adapter.clone();
                let epoch_store = self.epoch_store()?.clone();
                tokio_runtime_handle.spawn(async move {
                    if let Err(err) = consensus_adapter
                        .submit_to_consensus(&vec![output], &epoch_store)
                        .await
                    {
                        error!("failed to submit MPC message to consensus: {:?}", err);
                    }
                });
                Ok(())
            }
            Err(DwalletMPCError::SessionFailedWithMaliciousParties(malicious_parties)) => {
                error!(
                    "Session {:?} failed with malicious parties: {:?}",
                    self.session_info.mpc_round , malicious_parties
                );
                let malicious_parties = malicious_parties
                    .into_iter()
                    .map(|party_id| {
                        Ok(party_id_to_authority_name(party_id, &*self.epoch_store()?)?)
                    })
                    .collect::<DwalletMPCResult<Vec<_>>>()?;
                let report = MaliciousReport::new(
                    malicious_parties,
                    self.session_info.session_id.clone(),
                    self.pending_messages[self.pending_quorum_for_highest_round_number]
                        .keys()
                        .map(|party_id| *party_id)
                        .collect(),
                );
                let output =
                    self.new_dwallet_report_failed_session_with_malicious_actors(report)?;
                let consensus_adapter = self.consensus_adapter.clone();
                let epoch_store = self.epoch_store()?.clone();
                tokio_runtime_handle.spawn(async move {
                    if let Err(err) = consensus_adapter
                        .submit_to_consensus(&vec![output], &epoch_store)
                        .await
                    {
                        error!("failed to submit MPC message to consensus: {:?}", err);
                    }
                });
                Ok(())
            }
            Err(DwalletMPCError::SessionFailedWithMaliciousParties(malicious_parties)) => {
                error!(
                    "session failed with malicious parties: {:?}",
                    malicious_parties
                );
                let malicious_parties = malicious_parties
                    .into_iter()
                    .map(|party_id| {
                        Ok(party_id_to_authority_name(party_id, &*self.epoch_store()?)?)
                    })
                    .collect::<DwalletMPCResult<Vec<_>>>()?;
                let report =
                    MaliciousReport::new(malicious_parties, self.session_info.session_id.clone());
                let output =
                    self.new_dwallet_report_failed_session_with_malicious_actors(report)?;
                let consensus_adapter = self.consensus_adapter.clone();
                let epoch_store = self.epoch_store()?.clone();
                tokio_runtime_handle.spawn(async move {
                    if let Err(err) = consensus_adapter
                        .submit_to_consensus(&vec![output], &epoch_store)
                        .await
                    {
                        error!("failed to submit an MPC message to consensus: {:?}", err);
                    }
                });
                Ok(())
            }
            Err(e) => {
                error!("failed to advance the MPC session: {:?}", e);
                // TODO (#524): Handle failed MPC sessions
                Err(e)
            }
        }
    }

    fn advance_specific_party(
        &self,
    ) -> DwalletMPCResult<AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>> {
        let session_id = CommitmentSizedNumber::from_le_slice(
            self.session_info.flow_session_id.to_vec().as_slice(),
        );
        match &self.session_info.mpc_round {
            MPCProtocolInitData::SignIdentifiableAbort(sign_ia_session_data) => {
                let public_input = bcs::from_bytes(&self.public_input)?;
                crate::dwallet_mpc::advance::<SignFirstParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.pending_messages.clone(),
                    public_input,
                    self.decryption_share.clone(),
                )
            }
            MPCProtocolInitData::DKGFirst => {
                let public_input = bcs::from_bytes(&self.public_input)?;
                crate::dwallet_mpc::advance::<DKGFirstParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.pending_messages.clone(),
                    public_input,
                    (),
                )
            }
            MPCProtocolInitData::DKGSecond(event_data, _) => {
                let public_input = bcs::from_bytes(&self.public_input)?;
                let result = crate::dwallet_mpc::advance::<DKGSecondParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.pending_messages.clone(),
                    public_input,
                    (),
                )?;
                if let AsynchronousRoundResult::Finalize { public_output, .. } = &result {
                    verify_encrypted_share(&StartEncryptedShareVerificationEvent {
                        dwallet_centralized_public_output: event_data
                            .dkg_centralized_public_output
                            .clone(),
                        encrypted_secret_share_and_proof: event_data
                            .encrypted_secret_share_and_proof
                            .clone(),
                        encryption_key: event_data.encryption_key.clone(),
                        encryption_key_id: event_data.encryption_key_id.clone(),
                        initiator: event_data.initiator.clone(),
                        signed_public_share: event_data.signed_public_share.clone(),
                        encryptor_ed25519_pubkey: event_data.encryptor_ed25519_pubkey.clone(),

                        // Fields not relevant for verification; passing empty values.
                        dwallet_id: ID::new(ObjectID::new([0; 32])),
                        session_id: ID::new(ObjectID::new([0; 32])),
                    })?;
                }
                Ok(result)
            }
            MPCProtocolInitData::PresignFirst(..) => {
                let public_input = bcs::from_bytes(&self.public_input)?;
                crate::dwallet_mpc::advance::<PresignFirstParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.pending_messages.clone(),
                    public_input,
                    (),
                )
            }
            MPCProtocolInitData::PresignSecond(..) => {
                let public_input = bcs::from_bytes(&self.public_input)?;
                crate::dwallet_mpc::advance::<PresignSecondParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.pending_messages.clone(),
                    public_input,
                    (),
                )
            }
            MPCProtocolInitData::Sign(..) => {
                let public_input = bcs::from_bytes(&self.public_input)?;
                crate::dwallet_mpc::advance::<SignFirstParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.pending_messages.clone(),
                    public_input,
                    self.decryption_share.clone(),
                )
            }
            MPCProtocolInitData::NetworkDkg(key_scheme, _) => advance_network_dkg(
                session_id,
                &self.weighted_threshold_access_structure,
                self.party_id,
                &self.public_input,
                key_scheme,
                self.pending_messages.clone(),
                bcs::from_bytes(
                    &self
                        .private_input
                        .clone()
                        .ok_or(DwalletMPCError::MissingMPCPrivateInput)?,
                )?,
                self.epoch_store()?,
            ),
            MPCProtocolInitData::EncryptedShareVerification(verification_data) => {
                match verify_encrypted_share(verification_data) {
                    Ok(_) => Ok(AsynchronousRoundResult::Finalize {
                        public_output: vec![],
                        private_output: vec![],
                        malicious_parties: vec![],
                    }),
                    Err(err) => Err(err),
                }
            }
            MPCProtocolInitData::EncryptionKeyVerification(verification_data) => {
                verify_encryption_key(verification_data)
                    .map(|_| AsynchronousRoundResult::Finalize {
                        public_output: vec![],
                        private_output: vec![],
                        malicious_parties: vec![],
                    })
                    .map_err(|err| err)
            }
            MPCProtocolInitData::BatchedPresign(..) | MPCProtocolInitData::BatchedSign(..) => {
                unreachable!("advance should never be called on a batched session")
            }
        }
    }

    /// A function to restart an MPC session.
    /// Being called when session advancement has failed due to malicious parties.
    /// Those parties will be flagged as malicious and ignored,
    /// the session will be restarted.
    pub(crate) fn restart(&mut self) {
        self.status = MPCSessionStatus::Active;
        self.pending_messages = vec![HashMap::new()];
    }

    /// Create a new consensus transaction with the message to be sent to the other MPC parties.
    /// Returns None only if the epoch switched in the middle and was not available.
    fn new_dwallet_mpc_message(
        &self,
        message: MPCMessage,
    ) -> DwalletMPCResult<ConsensusTransaction> {
        Ok(ConsensusTransaction::new_dwallet_mpc_message(
            self.epoch_store()?.name,
            message,
            self.session_info.session_id.clone(),
            self.pending_quorum_for_highest_round_number + 1,
        ))
    }

    /// Create a new consensus transaction with the flow result (output) to be
    /// sent to the other MPC parties.
    /// Errors if the epoch was switched in the middle and was not available.
    fn new_dwallet_mpc_output_message(
        &self,
        output: Vec<u8>,
    ) -> DwalletMPCResult<ConsensusTransaction> {
        Ok(ConsensusTransaction::new_dwallet_mpc_output(
            self.epoch_store()?.name,
            output,
            self.session_info.clone(),
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

    /// Stores a message in the pending messages map.
    /// Every new message received for a session is stored.
    /// When a threshold of messages is reached, the session advances.
    fn store_message(&mut self, message: &DWalletMPCMessage) -> DwalletMPCResult<()> {
        let source_party_id =
            authority_name_to_party_id(&message.authority, &*self.epoch_store()?)?;

        let current_round = self.pending_messages.len();
        match self.pending_messages.get_mut(message.round_number) {
            Some(party_to_msg) => {
                if party_to_msg.contains_key(&source_party_id) {
                    // Duplicate.
                    return Err(DwalletMPCError::MaliciousParties(vec![source_party_id]));
                }
                party_to_msg.insert(source_party_id, message.message.clone());
            }
            // If next round.
            None if message.round_number == current_round => {
                let mut map = HashMap::new();
                map.insert(source_party_id, message.message.clone());
                self.pending_messages.push(map);
            }
            _ => {
                // Unexpected round number; rounds should grow sequentially.
                return Err(DwalletMPCError::MaliciousParties(vec![source_party_id]));
            }
        }
        Ok(())
    }

    /// Handles a message by either forwarding it to the session
    /// or ignoring it if the session is not active.
    pub(crate) fn handle_message(&mut self, message: &DWalletMPCMessage) -> DwalletMPCResult<()> {
        if matches!(
            self.session_info.mpc_round,
            MPCProtocolInitData::SignIdentifiableAbort(..)
        ) {
            return Ok(());
        }
        self.store_message(message)?;
        Ok(())
    }

    pub(crate) fn rerun_last_round_without_malicious_parties(
        &mut self,
        malicious_parties: &HashSet<PartyID>,
    ) -> Result<(), DwalletMPCError> {
        // For every advance we increase the round number by 1,
        // so to re-run the same round we decrease it by 1.
        self.pending_quorum_for_highest_round_number -= 1;
        // Remove malicious parties from the session messages.
        let round_messages = self
            .pending_messages
            .get_mut(self.pending_quorum_for_highest_round_number)
            .ok_or(DwalletMPCError::MPCSessionNotFound {
                session_id: self.session_info.session_id,
            })?;

        malicious_parties.iter().for_each(|malicious_actor| {
            round_messages.remove(malicious_actor);
        });
        Ok(())
    }
}
