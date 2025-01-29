//! A module to verify the dWallet MPC outputs.
//! The module handles storing the outputs received for each session,
//! and deciding whether an output is valid
//! by checking if a validators with quorum of stake voted for it.
//! Any validator that voted for a different output is considered malicious.

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::dkg::DKGSecondParty;
use crate::dwallet_mpc::sign::SignFirstParty;
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, MPCPublicOutput};
use group::GroupElement;
use mpc::Party;
use narwhal_types::Round;
use pera_types::base_types::{AuthorityName, EpochId, ObjectID};
use pera_types::committee::StakeUnit;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::messages_dwallet_mpc::{
    MPCProtocolInitData, MPCSessionSpecificState, SessionInfo, SingleSignSessionData,
};
use twopc_mpc::secp256k1::class_groups::{
    ProtocolPublicParameters, FUNDAMENTAL_DISCRIMINANT_LIMBS, NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};

use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Weak};
use tracing::error;
use tracing::log::warn;
use twopc_mpc::secp256k1;
use twopc_mpc::sign::verify_signature;

/// Verify the DWallet MPC outputs.
///
/// Stores all the outputs received for each session,
/// and decides whether an output is valid
/// by checking if a validators with quorum of stake voted for it.
pub struct DWalletMPCOutputsVerifier {
    /// The outputs received for each MPC session.
    pub mpc_sessions_outputs: HashMap<ObjectID, SessionOutputsData>,
    /// A mapping between an authority name to its stake.
    /// This data exists in the MPCManager, but in a different data structure.
    pub weighted_parties: HashMap<AuthorityName, StakeUnit>,
    /// The quorum threshold of the chain.
    pub quorum_threshold: StakeUnit,
    // todo(zeev): why is it here?
    pub completed_locking_next_committee: bool,
    voted_to_lock_committee: HashSet<AuthorityName>,
    /// The latest consensus round that was processed.
    /// Used to check if there's a need to perform a state sync —
    /// if the `latest_processed_dwallet_round` is behind
    /// the currently processed round by more than one,
    /// a state sync should be performed.
    pub(crate) last_processed_consensus_round: Round,
    epoch_store: Weak<AuthorityPerEpochStore>,
    epoch_id: EpochId,
}

/// The data needed to manage the outputs of an MPC session.
#[derive(Clone)]
pub struct SessionOutputsData {
    /// Maps session's output to the authorities that voted for it.
    /// The key must contain the session info, and the output to prevent
    /// malicious behavior, such as sending the correct output, but from a faulty session.
    pub session_output_to_voting_authorities:
        HashMap<(MPCPublicOutput, SessionInfo), HashSet<AuthorityName>>,
    /// Needed to make sure an authority does not send two outputs for the same session.
    pub authorities_that_sent_output: HashSet<AuthorityName>,
    pub(crate) current_result: OutputResult,
}

/// The result of verifying an incoming output for an MPC session.
/// We need to differentiate between a duplicate and a malicious output,
/// as the output can be sent twice by honest parties.
#[derive(PartialOrd, PartialEq, Clone)]
pub enum OutputResult {
    Valid,
    Malicious,
    /// We need more votes to decide if the output is valid or not.
    NotEnoughVotes,
    /// The output has already been verified and committed to the chain
    AlreadyCommitted,
    Duplicate,
}

pub struct OutputVerificationResult {
    pub result: OutputResult,
    pub malicious_actors: Vec<AuthorityName>,
}

impl DWalletMPCOutputsVerifier {
    pub fn new(epoch_store: &Arc<AuthorityPerEpochStore>) -> Self {
        DWalletMPCOutputsVerifier {
            epoch_store: Arc::downgrade(&epoch_store),
            quorum_threshold: epoch_store.committee().quorum_threshold(),
            mpc_sessions_outputs: HashMap::new(),
            weighted_parties: epoch_store
                .committee()
                .voting_rights
                .iter()
                .cloned()
                .collect(),
            completed_locking_next_committee: false,
            voted_to_lock_committee: HashSet::new(),
            last_processed_consensus_round: 0,
            epoch_id: epoch_store.epoch(),
        }
    }

    /// Determines whether the `lock_next_epoch_committee` system transaction should be called.
    ///
    /// This function tracks votes from authorities to decide if a quorum has been reached
    /// to lock the next epoch's committee.
    /// If the total weighted stake of the authorities
    /// that have voted exceeds or equals the quorum threshold, it returns `true`.
    /// Otherwise, it returns `false`.
    pub(crate) fn should_lock_committee(&mut self, authority_name: AuthorityName) -> bool {
        self.voted_to_lock_committee.insert(authority_name);
        self.voted_to_lock_committee
            .iter()
            .map(|voter| self.weighted_parties.get(voter).unwrap_or(&0))
            .sum::<StakeUnit>()
            >= self.quorum_threshold
    }

    /// Stores the given MPC output, and checks if any of the received
    /// outputs already received a quorum of votes.
    /// If so, the output is returned along with a vector of malicious actors,
    /// i.e., parties that voted for other outputs.
    // TODO (#311): Make sure validator don't mark other validators as malicious
    // TODO (#311): or take any active action while syncing
    pub async fn try_verify_output(
        &mut self,
        output: &Vec<u8>,
        session_info: &SessionInfo,
        origin_authority: AuthorityName,
    ) -> DwalletMPCResult<OutputVerificationResult> {
        let epoch_store = self.epoch_store()?;
        let Some(ref mut session_output_data) =
            self.mpc_sessions_outputs.get_mut(&session_info.session_id)
        else {
            return Ok(OutputVerificationResult {
                result: OutputResult::Malicious,
                malicious_actors: vec![origin_authority],
            });
        };
        if session_output_data.current_result == OutputResult::AlreadyCommitted {
            return Ok(OutputVerificationResult {
                result: OutputResult::Duplicate,
                malicious_actors: vec![],
            });
        }
        if let MPCProtocolInitData::Sign(sign_session_data) = &session_info.mpc_round {
            return match Self::verify_signature(&epoch_store, sign_session_data, output) {
                Ok(res) => {
                    session_output_data.current_result = OutputResult::AlreadyCommitted;
                    let mpc_manager = epoch_store.get_dwallet_mpc_manager().await;
                    let session = mpc_manager
                        .mpc_sessions
                        .get(&session_info.session_id)
                        .ok_or(DwalletMPCError::MPCSessionNotFound {
                            session_id: session_info.session_id,
                        })?;
                    let mut session_malicious_actors = res.malicious_actors;
                    if let Some(MPCSessionSpecificState::Sign(sign_state)) =
                        &session.session_specific_state
                    {
                        // If one of the validators in the sign session sent a malicious report,
                        // every validator need to make sure the reported validator actually marked
                        // as malicious before the sign session got completed.
                        // If the reported validator was not actually malicious, the reporting
                        // validator should be marked as malicious.
                        for reported_malicious_actor in
                            &sign_state.malicious_report.malicious_actors
                        {
                            if !mpc_manager
                                .malicious_handler
                                .get_malicious_actors_names()
                                .contains(&reported_malicious_actor)
                            {
                                warn!("a sign session got completed successfully while the reported malicious actor {:?} was not actually malicious, marking the reporting authority as malicious", reported_malicious_actor);
                                session_malicious_actors.push(sign_state.initiating_ia_authority);
                                break;
                            }
                        }
                    }
                    Ok(OutputVerificationResult {
                        result: OutputResult::Valid,
                        malicious_actors: session_malicious_actors,
                    })
                }
                Err(err) => {
                    // TODO (#549): Handle malicious behavior in aggregated sign flow
                    error!(
                        "received an invalid signature: {:?} for session: {:?}",
                        err, session_info.session_id
                    );
                    Ok(OutputVerificationResult {
                        result: OutputResult::Malicious,
                        malicious_actors: vec![origin_authority],
                    })
                }
            };
        }
        // Sent more than once.
        if session_output_data
            .authorities_that_sent_output
            .contains(&origin_authority)
        {
            return Ok(OutputVerificationResult {
                result: OutputResult::Malicious,
                malicious_actors: vec![origin_authority],
            });
        }
        session_output_data
            .authorities_that_sent_output
            .insert(origin_authority.clone());
        session_output_data
            .session_output_to_voting_authorities
            .entry((output.clone(), session_info.clone()))
            .or_default()
            .insert(origin_authority);

        let agreed_output = session_output_data
            .session_output_to_voting_authorities
            .iter()
            .find(|(_, voters)| {
                voters
                    .iter()
                    .map(|voter| self.weighted_parties.get(voter).unwrap_or(&0))
                    .sum::<StakeUnit>()
                    >= self.quorum_threshold
            });

        if let Some((agreed_output, _)) = agreed_output {
            let voted_for_other_outputs = session_output_data
                .session_output_to_voting_authorities
                .iter()
                .filter(|(output, _)| *output != agreed_output)
                .flat_map(|(_, voters)| voters)
                .cloned()
                .collect();
            session_output_data.current_result = OutputResult::AlreadyCommitted;
            return Ok(OutputVerificationResult {
                result: OutputResult::Valid,
                malicious_actors: voted_for_other_outputs,
            });
        }

        Ok(OutputVerificationResult {
            result: OutputResult::NotEnoughVotes,
            malicious_actors: vec![],
        })
    }

    fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }

    fn verify_signature(
        epoch_store: &Arc<AuthorityPerEpochStore>,
        sign_session_data: &SingleSignSessionData,
        signature: &MPCPublicOutput,
    ) -> DwalletMPCResult<OutputVerificationResult> {
        let sign_output = bcs::from_bytes::<<SignFirstParty as Party>::PublicOutput>(&signature)?;
        let dkg_output = bcs::from_bytes::<<DKGSecondParty as Party>::PublicOutput>(
            &sign_session_data.dkg_output,
        )?
        .public_key;
        let protocol_public_parameters = epoch_store
            .dwallet_mpc_network_keys
            .get()
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .get_protocol_public_parameters(
                DWalletMPCNetworkKeyScheme::Secp256k1,
                sign_session_data.network_key_version,
            )?;
        let protocol_public_parameters: secp256k1::class_groups::ProtocolPublicParameters =
            bcs::from_bytes(&protocol_public_parameters)?;
        let dwallet_public_key = secp256k1::GroupElement::new(
            dkg_output,
            &protocol_public_parameters.as_ref().group_public_parameters,
        )
        .map_err(|e| {
            DwalletMPCError::ClassGroupsError(format!(
                "{}{}",
                "Failed to create public key: ".to_string(),
                e.to_string()
            ))
        })?;

        if let Err(err) = verify_signature(
            sign_output.0,
            sign_output.1,
            bcs::from_bytes(&sign_session_data.message)?,
            dwallet_public_key,
        ) {
            return Err(DwalletMPCError::SignatureVerificationFailed(
                err.to_string(),
            ));
        }
        Ok(OutputVerificationResult {
            result: OutputResult::Valid,
            malicious_actors: vec![],
        })
    }

    // message: GroupElement::Scalar,
    // dkg_output: DKGDecentralizedPartyOutput<
    // SCALAR_LIMBS,
    // FUNDAMENTAL_DISCRIMINANT_LIMBS,
    // NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    // GroupElement,
    // >,
    // presign: Presign<
    // SCALAR_LIMBS,
    // FUNDAMENTAL_DISCRIMINANT_LIMBS,
    // NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    // GroupElement,
    // >,
    // sign_message: Message<
    // SCALAR_LIMBS,
    // FUNDAMENTAL_DISCRIMINANT_LIMBS,
    // NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    // MESSAGE_LIMBS,
    // GroupElement,
    // >,
    // protocol_public_parameters: &crate::class_groups::ProtocolPublicParameters<
    // SCALAR_LIMBS,
    // FUNDAMENTAL_DISCRIMINANT_LIMBS,
    // NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    // GroupElement,
    // >,
    // session_id: CommitmentSizedNumber,

    fn verify_partial_signature(
        hashed_message: &[u8],
        dkg_output: &[u8],
        presign: &[u8],
        partially_signed_message: &[u8],
        protocol_public_parameters: &ProtocolPublicParameters,
        session_id: ObjectID,
    ) -> DwalletMPCResult<()> {
        twopc_mpc::sign::decentralized_party::signature_partial_decryption_round::Party::verify_encryption_of_signature_parts_prehash_class_groups(
            bcs::from_bytes(hashed_message)?,
            bcs::from_bytes(dkg_output)?,
            bcs::from_bytes(presign)?,
            bcs::from_bytes(partially_signed_message)?,
            protocol_public_parameters,
            CommitmentSizedNumber::from_le_slice(
                session_id.to_vec().as_slice(),
            ),
        ).map_err(|err| {
            DwalletMPCError::TwoPCMPCError(format!("{:?}", err))
        })
    }

    /// Stores the session ID of the new MPC session,
    /// and initializes the output data for it.
    /// Needed, so we'll know when we receive a malicious output
    /// that related to a non-existing session.
    pub fn handle_new_event(&mut self, session_info: &SessionInfo) {
        self.mpc_sessions_outputs.insert(
            session_info.session_id,
            SessionOutputsData {
                session_output_to_voting_authorities: HashMap::new(),
                authorities_that_sent_output: HashSet::new(),
                current_result: OutputResult::NotEnoughVotes,
            },
        );
    }
}
