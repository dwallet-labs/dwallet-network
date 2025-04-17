//! A module to verify the dWallet MPC outputs.
//! The module handles storing the outputs received for each session,
//! and deciding whether an output is valid
//! by checking if an authorized validator set voted for it.
//! Any validator that voted for a different output is considered malicious.

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::stake_aggregator::StakeAggregator;
use dwallet_mpc_types::dwallet_mpc::MPCPublicOutput;
use group::{GroupElement, PartyID};
use ika_types::committee::StakeUnit;
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{DWalletMPCMessage, SessionInfo};
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, Weak};
use sui_types::base_types::{EpochId, ObjectID};
use sui_types::messages_consensus::Round;

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
    pub completed_locking_next_committee: bool,
    voted_to_lock_committee: HashSet<PartyID>,
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
pub struct SessionOutputsData {
    /// Maps session's output to the authorities that voted for it.
    /// The key must contain the session info, and the output to prevent
    /// malicious behavior, such as sending the correct output, but from a faulty session.
    pub session_output_to_voting_authorities:
        HashMap<(MPCPublicOutput, SessionInfo), StakeAggregator<(), true>>,
    /// Needed to make sure an authority does not send two outputs for the same session.
    pub authorities_that_sent_output: HashSet<AuthorityName>,
    pub(crate) current_result: OutputVerificationStatus,
}

/// The result of verifying an incoming output for an MPC session.
/// We need to differentiate between a duplicate and a malicious output,
/// as the output can be sent twice by honest parties.
#[derive(PartialOrd, PartialEq, Clone)]
pub enum OutputVerificationStatus {
    FirstQuorumReached(MPCPublicOutput),
    Malicious,
    /// We need more votes to decide if the output is valid or not.
    NotEnoughVotes,
    /// The output has already been verified and committed to the chain.
    /// This happens every time since all honest parties send the same output.
    AlreadyCommitted,
    BuildingOutput,
}

pub struct OutputVerificationResult {
    pub result: OutputVerificationStatus,
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
    pub(crate) fn append_vote_and_check_committee_lock(
        &mut self,
        authority_name: AuthorityName,
    ) -> DwalletMPCResult<bool> {
        let epoch_store = self.epoch_store()?;
        self.voted_to_lock_committee
            .insert(epoch_store.authority_name_to_party_id(&authority_name)?);
        Ok(epoch_store
            .get_weighted_threshold_access_structure()?
            .is_authorized_subset(&self.voted_to_lock_committee)
            .is_ok())
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
        let committee = epoch_store.committee().clone();

        let ref mut session_output_data = self
            .mpc_sessions_outputs
            .entry(session_info.session_id)
            .or_insert(SessionOutputsData {
                session_output_to_voting_authorities: HashMap::new(),
                authorities_that_sent_output: HashSet::new(),
                current_result: OutputVerificationStatus::NotEnoughVotes,
            });
        if session_output_data.current_result == OutputVerificationStatus::AlreadyCommitted {
            return Ok(OutputVerificationResult {
                result: OutputVerificationStatus::AlreadyCommitted,
                malicious_actors: vec![],
            });
        }
        // Sent more than once.
        if session_output_data
            .authorities_that_sent_output
            .contains(&origin_authority)
        {
            // Duplicate.
            return Ok(OutputVerificationResult {
                result: OutputVerificationStatus::AlreadyCommitted,
                malicious_actors: vec![],
            });
        }
        session_output_data
            .authorities_that_sent_output
            .insert(origin_authority.clone());

        if session_output_data
            .session_output_to_voting_authorities
            .entry((output.clone(), session_info.clone()))
            .or_insert(StakeAggregator::new(committee))
            .insert_generic(origin_authority, ())
            .is_quorum_reached()
        {
            session_output_data.current_result = OutputVerificationStatus::AlreadyCommitted;
            return Ok(OutputVerificationResult {
                result: OutputVerificationStatus::FirstQuorumReached(output.clone()),
                malicious_actors: vec![],
            });
        }
        Ok(OutputVerificationResult {
            result: OutputVerificationStatus::NotEnoughVotes,
            malicious_actors: vec![],
        })
    }

    fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }

    /// Stores the session ID of the new MPC session,
    /// and initializes the output data for it.
    /// Needed, so we'll know when we receive a malicious output
    /// that related to a non-existing session.
    pub fn monitor_new_session_outputs(&mut self, session_info: &SessionInfo) {
        self.mpc_sessions_outputs.insert(
            session_info.session_id,
            SessionOutputsData {
                session_output_to_voting_authorities: HashMap::new(),
                authorities_that_sent_output: HashSet::new(),
                current_result: OutputVerificationStatus::NotEnoughVotes,
            },
        );
    }
}
