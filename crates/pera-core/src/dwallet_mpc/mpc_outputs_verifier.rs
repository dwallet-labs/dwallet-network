//! A module to verify the dWallet MPC outputs.
//! The module handles storing the outputs received for each session,
//! and deciding whether an output is valid
//! by checking if a validators with quorum of stake voted for it.
//! Any validator that voted for a different output is considered malicious.

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::authority_name_to_party_id;
use dwallet_mpc_types::dwallet_mpc::MPCPublicOutput;
use group::PartyID;
use mpc::WeightedThresholdAccessStructure;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::committee::StakeUnit;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::messages_dwallet_mpc::SessionInfo;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

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

/// The possible verification status of an MPC session.
#[derive(Clone, PartialEq)]
enum VerificationStatus {
    /// The session is still active, and we are waiting for more outputs.
    Active,
    /// The session has received enough votes to decide on the output,
    /// and the output has been committed.
    Verified,
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
    pub(crate) status: VerificationStatus,
}

/// The result of verifying an incoming output for an MPC session.
/// We need to differentiate between a duplicate and a malicious output,
/// as the output can be sent twice by honest parties.
#[derive(PartialOrd, PartialEq)]
pub enum OutputResult {
    Valid,
    Malicious,
    /// We need more votes to decide if the output is valid or not.
    NotEnoughVotes,
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
    ) -> anyhow::Result<OutputVerificationResult> {
        let Some(ref mut session) = self.mpc_sessions_outputs.get_mut(&session_info.session_id)
        else {
            return Ok(OutputVerificationResult {
                result: OutputResult::Malicious,
                malicious_actors: vec![origin_authority],
            });
        };
        if session.status == VerificationStatus::Verified {
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
            session.status = VerificationStatus::Verified;
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
                status: VerificationStatus::Active,
            },
        );
    }
}
