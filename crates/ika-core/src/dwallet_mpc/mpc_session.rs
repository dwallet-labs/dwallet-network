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
use tracing::{error, warn};
use twopc_mpc::sign::Protocol;

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::dkg::{DKGFirstParty, DKGSecondParty};
use crate::dwallet_mpc::encrypt_user_share::{verify_encrypted_share, verify_encryption_key};
use crate::dwallet_mpc::network_dkg::advance_network_dkg;
use crate::dwallet_mpc::presign::PresignParty;
use crate::dwallet_mpc::sign::{verify_partial_signature, SignFirstParty};
use crate::dwallet_mpc::{
    authority_name_to_party_id, party_id_to_authority_name, party_ids_to_authority_names, presign,
};
use ika_types::committee::StakeUnit;
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_consensus::ConsensusTransaction;
use ika_types::messages_dwallet_mpc::{
    AdvanceResult, DWalletMPCMessage, MPCProtocolInitData, MPCSessionSpecificState,
    MaliciousReport, PresignSessionState, SessionInfo, SignIASessionState,
    StartEncryptedShareVerificationEvent, StartPresignFirstRoundEvent,
};
use sui_types::base_types::{EpochId, ObjectID};
use sui_types::id::ID;

pub(crate) type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;

/// The result of the check if the session is ready to advance.
///
/// Returns whether the session is ready to advance or not, and a list of the malicious parties that were detected
/// while performing the check.
pub(crate) struct ReadyToAdvanceCheckResult {
    pub(crate) is_ready: bool,
    pub(crate) malicious_parties: Vec<PartyID>,
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
    pub(super) serialized_messages: Vec<HashMap<PartyID, MPCMessage>>,
    epoch_store: Weak<AuthorityPerEpochStore>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    epoch_id: EpochId,
    pub(super) session_info: SessionInfo,
    pub(super) public_input: MPCPublicInput,
    /// The current MPC round number of the session.
    /// Starts at 0 and increments by one each time we advance the session.
    pub(super) pending_quorum_for_highest_round_number: usize,
    /// Contains state that is specific to the session's protocol, i.e. presign specific state in a presign session,
    /// or sign specific state in a sign session.
    pub(super) session_specific_state: Option<MPCSessionSpecificState>,
    party_id: PartyID,
    // TODO (#539): Simplify struct to only contain session related data - remove this field.
    weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    // TODO (#539): Simplify struct to only contain session related data - remove this field.
    decryption_share: HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>,
    // TODO (#539): Simplify struct to only contain session related data - remove this field.
    private_input: MPCPrivateInput,
}

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
    ) -> Self {
        Self {
            status,
            serialized_messages: vec![HashMap::new()],
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
            session_specific_state: None,
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
                let consensus_adapter = self.consensus_adapter.clone();
                let epoch_store = self.epoch_store()?.clone();
                if !malicious_parties.is_empty() || self.is_verifying_sign_ia_report() {
                    self.report_malicious_actors(
                        tokio_runtime_handle,
                        malicious_parties,
                        AdvanceResult::Success,
                    )?;
                }
                let consensus_message =
                    self.new_dwallet_mpc_output_message(public_output.clone())?;
                tokio_runtime_handle.spawn(async move {
                    if let Err(err) = consensus_adapter
                        .submit_to_consensus(&vec![consensus_message], &epoch_store)
                        .await
                    {
                        error!("failed to submit an MPC message to consensus: {:?}", err);
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
                error!("failed to advance the MPC session: {:?}", e);
                // TODO (#524): Handle failed MPC sessions
                Err(e)
            }
        }
    }

    /// Returns true if the session is still verifying that a Start Sign Identifiable Report
    /// message is valid; false otherwise.
    /// The Sign Identifiable Abort protocol differs from other protocols as,
    /// besides verifying that the output is valid, we must also verify that the malicious report,
    /// which caused all other validators to spend extra resources, was honest.
    pub(crate) fn is_verifying_sign_ia_report(&self) -> bool {
        let Some(MPCSessionSpecificState::Sign(sign_state)) = &self.session_specific_state else {
            return false;
        };
        sign_state.verified_malicious_report.is_none()
    }

    /// Starts the Sign Identifiable Abort protocol if needed.
    ///
    /// In the aggregated signing protocol, a single malicious report is enough
    /// to trigger the Sign-Identifiable Abort protocol.
    /// In the Sign-Identifiable Abort protocol, each validator runs the final step,
    /// agreeing on the malicious parties in the session and
    /// removing their messages before the signing session continues as usual.
    pub(crate) fn check_for_sign_ia_start(
        &mut self,
        reporting_authority: AuthorityName,
        report: MaliciousReport,
    ) {
        if matches!(self.session_info.mpc_round, MPCProtocolInitData::Sign(..))
            && self.status == MPCSessionStatus::Active
            && self.session_specific_state.is_none()
        {
            self.session_specific_state = Some(MPCSessionSpecificState::Sign(SignIASessionState {
                start_ia_flow_malicious_report: report,
                initiating_ia_authority: reporting_authority,
                verified_malicious_report: None,
            }))
        }
    }

    /// In the Sign Identifiable Abort protocol, each validator sends a malicious report, even
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
            self.session_info.session_id.clone(),
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

    fn advance_specific_party(
        &self,
    ) -> DwalletMPCResult<AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>> {
        let session_id =
            CommitmentSizedNumber::from_le_slice(self.session_info.session_id.to_vec().as_slice());
        match &self.session_info.mpc_round {
            MPCProtocolInitData::DKGFirst(..) => {
                let public_input = bcs::from_bytes(&self.public_input)?;
                crate::dwallet_mpc::advance_and_serialize::<DKGFirstParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_messages.clone(),
                    public_input,
                    (),
                )
            }
            MPCProtocolInitData::DKGSecond(event_data, _) => {
                let public_input = bcs::from_bytes(&self.public_input)?;
                let result = crate::dwallet_mpc::advance_and_serialize::<DKGSecondParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_messages.clone(),
                    public_input,
                    (),
                )?;
                if let AsynchronousRoundResult::Finalize { public_output, .. } = &result {
                    verify_encrypted_share(&StartEncryptedShareVerificationEvent {
                        decentralized_public_output: public_output.clone(),
                        encrypted_centralized_secret_share_and_proof: event_data
                            .event_data
                            .encrypted_centralized_secret_share_and_proof
                            .clone(),
                        encryption_key: event_data.event_data.encryption_key.clone(),
                        encryption_key_id: event_data.event_data.encryption_key_id.clone(),

                        // Fields not relevant for verification; passing empty values.
                        dwallet_id: ObjectID::new([0; 32]),
                        session_id: ObjectID::new([0; 32]),
                    })?;
                }
                Ok(result)
            }
            MPCProtocolInitData::Presign(..) => {
                let public_input = bcs::from_bytes(&self.public_input)?;
                crate::dwallet_mpc::advance_and_serialize::<PresignParty>(
                    session_id,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_messages.clone(),
                    public_input,
                    (),
                )
            }
            MPCProtocolInitData::Sign(init_data) => {
                let public_input = bcs::from_bytes(&self.public_input)?;
                let yes = CommitmentSizedNumber::from_le_slice(
                    init_data.presign_session_id.to_vec().as_slice(),
                );
                crate::dwallet_mpc::advance_and_serialize::<SignFirstParty>(
                    yes,
                    self.party_id,
                    &self.weighted_threshold_access_structure,
                    self.serialized_messages.clone(),
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
                self.serialized_messages.clone(),
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
            MPCProtocolInitData::PartialSignatureVerification(event_data) => {
                for (signature_data, hashed_message) in event_data
                    .signature_data
                    .iter()
                    .zip(event_data.hashed_messages.iter())
                {
                    verify_partial_signature(
                        hashed_message,
                        &event_data.dwallet_decentralized_public_output,
                        &signature_data.presign_output,
                        &signature_data.message_centralized_signature,
                        &bcs::from_bytes(&self.public_input)?,
                        &signature_data.presign_id,
                    )?;
                }
                Ok(AsynchronousRoundResult::Finalize {
                    public_output: vec![],
                    private_output: vec![],
                    malicious_parties: vec![],
                })
            }
            MPCProtocolInitData::BatchedPresign(..) | MPCProtocolInitData::BatchedSign(..) => {
                // This case is unreachable because the batched session is handled separately.
                // The bathed session is only an indicator to expect a batch of messages.
                unreachable!("advance should never be called on a batched session")
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
            self.session_info.session_id.clone(),
            self.pending_quorum_for_highest_round_number,
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

    /// Stores a message in the serialized messages map.
    /// Every new message received for a session is stored.
    /// When a threshold of messages is reached, the session advances.
    pub(crate) fn store_message(&mut self, message: &DWalletMPCMessage) -> DwalletMPCResult<()> {
        let source_party_id =
            authority_name_to_party_id(&message.authority, &*self.epoch_store()?)?;

        let current_round = self.serialized_messages.len();
        match self.serialized_messages.get_mut(message.round_number) {
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
                self.serialized_messages.push(map);
            }
            None => {
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
                                .serialized_messages
                                .get(self.pending_quorum_for_highest_round_number)
                                .unwrap_or(&HashMap::new())
                                .keys()
                                .cloned()
                                .collect::<HashSet<PartyID>>(),
                        )
                        .ok()
                        .is_some()
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

    fn get_protocol_public_parameters(
        &self,
        key_scheme: DWalletMPCNetworkKeyScheme,
        key_version: u8,
    ) -> DwalletMPCResult<Vec<u8>> {
        if let Some(self_decryption_share) = self.epoch_store()?.dwallet_mpc_network_keys.get() {
            return self_decryption_share.get_protocol_public_parameters(key_scheme, key_version);
        }
        Err(DwalletMPCError::TwoPCMPCError(
            "Decryption share not found".to_string(),
        ))
    }
}
