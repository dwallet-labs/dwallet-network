//! A module to verify the dWallet MPC outputs.
//! The module handles storing the outputs received for each session,
//! and deciding whether an output is valid
//! by checking if a validators with quorum of stake voted for it.
//! Any validator that voted for a different output is considered malicious.

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::dkg::DKGSecondParty;
use crate::dwallet_mpc::mpc_manager::{AggregatorMessageStatus, DWalletMPCChannelMessage};
use crate::dwallet_mpc::sign::SignFirstParty;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCPublicOutput, MPCUpdateOutputSender,
};
use group::GroupElement;
use mpc::Party;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::committee::StakeUnit;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo, SignMessageData};
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use twopc_mpc::sign::verify_signature;
use twopc_mpc::{secp256k1, ProtocolPublicParameters};

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
    pub fn new(epoch_store: &AuthorityPerEpochStore) -> Self {
        DWalletMPCOutputsVerifier {
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
    pub fn try_verify_output(
        &mut self,
        output: &Vec<u8>,
        session_info: &SessionInfo,
        origin_authority: AuthorityName,
        epoch_store: Arc<AuthorityPerEpochStore>,
    ) -> DwalletMPCResult<OutputVerificationResult> {
        let Some(ref mut session) = self.mpc_sessions_outputs.get_mut(&session_info.session_id)
        else {
            return Ok(OutputVerificationResult {
                result: OutputResult::Malicious,
                malicious_actors: vec![origin_authority],
            });
        };
        if session.current_result == OutputResult::AlreadyCommitted {
            return Ok(OutputVerificationResult {
                result: OutputResult::Duplicate,
                malicious_actors: vec![],
            });
        }
        // Sent more than once.
        if session
            .authorities_that_sent_output
            .contains(&origin_authority)
        {
            return Ok(OutputVerificationResult {
                result: OutputResult::Malicious,
                malicious_actors: vec![origin_authority],
            });
        }
        session
            .authorities_that_sent_output
            .insert(origin_authority.clone());
        session
            .session_output_to_voting_authorities
            .entry((output.clone(), session_info.clone()))
            .or_default()
            .insert(origin_authority);

        if let MPCRound::Sign(s) = &session_info.mpc_round {
            // it could be a problem if the output is sent twice, and then we mark party as malicious
            if let Some(sender) = epoch_store.dwallet_mpc_sender.get() {
                return match Self::verify_signature(s.clone(), epoch_store.clone(), output.clone())
                {
                    Ok(res) => {
                        sender
                            .send(DWalletMPCChannelMessage::SessionWithAggregator(
                                session_info.session_id,
                                AggregatorMessageStatus::ValidMessageReceived {
                                    public_output: output.clone(),
                                    malicious_parties: res.malicious_actors.clone(),
                                },
                            ))
                            .map_err(|_| {
                                DwalletMPCError::DWalletMPCSenderSendFailed(
                                    "failed to send verified output to manager".to_string(),
                                )
                            })?;

                        session.current_result = OutputResult::AlreadyCommitted;
                        Ok(res)
                    }
                    Err(_) => {
                        sender
                            .send(DWalletMPCChannelMessage::SessionWithAggregator(
                                session_info.session_id,
                                AggregatorMessageStatus::InvalidMessageReceived {
                                    aggregator: origin_authority,
                                },
                            ))
                            .map_err(|_| {
                                DwalletMPCError::DWalletMPCSenderSendFailed(
                                    "failed to send verified output to manager".to_string(),
                                )
                            })?;

                        Ok(OutputVerificationResult {
                            result: OutputResult::Malicious,
                            malicious_actors: vec![origin_authority],
                        })
                    }
                };
            }
        }

        let agreed_output =
            session
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
            let voted_for_other_outputs = session
                .session_output_to_voting_authorities
                .iter()
                .filter(|(output, _)| *output != agreed_output)
                .flat_map(|(_, voters)| voters)
                .cloned()
                .collect();
            session.current_result = OutputResult::AlreadyCommitted;
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

    fn verify_signature(
        sign_data: SignMessageData,
        epoch_store: Arc<AuthorityPerEpochStore>,
        output: MPCPublicOutput,
    ) -> DwalletMPCResult<OutputVerificationResult> {
        let sign_output = bcs::from_bytes::<<SignFirstParty as Party>::PublicOutput>(&output)?;
        let dkg_output =
            bcs::from_bytes::<<DKGSecondParty as Party>::PublicOutput>(&sign_data.dkg_output)?
                .public_key;
        let protocol_public_parameters = epoch_store
            .dwallet_mpc_network_keys
            .get()
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .get_protocol_public_parameters(
                DWalletMPCNetworkKeyScheme::Secp256k1,
                sign_data.network_key_version,
            );
        let protocol_public_parameters: secp256k1::class_groups::ProtocolPublicParameters =
            bcs::from_bytes(&protocol_public_parameters?)?;
        let public_key = secp256k1::GroupElement::new(
            dkg_output,
            &protocol_public_parameters.as_ref().group_public_parameters,
        )
        .map_err(|_| {
            DwalletMPCError::ClassGroupsError("Failed to create public key".to_string())
        })?;

        if verify_signature(
            sign_output.0,
            sign_output.1,
            bcs::from_bytes(&sign_data.message)?,
            public_key,
        )
        .is_err()
        {
            return Err(DwalletMPCError::SignatureVerificationFailed);
        }
        Ok(OutputVerificationResult {
            result: OutputResult::Valid,
            malicious_actors: vec![],
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
