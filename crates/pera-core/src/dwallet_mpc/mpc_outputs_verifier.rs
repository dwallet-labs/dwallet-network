//! A module to verify the dWallet MPC outputs.
//! The module handles storing the outputs received for each session,
//! and deciding whether an output is valid
//! by checking if a validators with quorum of stake voted for it.
//! Any validator that voted for a different output is considered malicious.
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use dwallet_mpc_types::dwallet_mpc::DWalletMPCNetworkKey;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::committee::StakeUnit;
use pera_types::messages_dwallet_mpc::SessionInfo;
use std::collections::{HashMap, HashSet};

/// A struct to verify the DWallet MPC outputs.
/// It stores all the outputs received for each session,
/// and decides whether an output is valid
/// by checking if a validators with quorum of stake voted for it.
/// todo(zeev): rename instance to session.
pub struct DWalletMPCOutputsVerifier {
    /// The outputs received for each instance.
    pub mpc_instances_outputs: HashMap<ObjectID, InstanceOutputsData>,
    /// A mapping between an authority name to its stake.
    /// todo(zeev): can we use the data from the manager?
    pub weighted_parties: HashMap<AuthorityName, StakeUnit>,
    /// The quorum threshold of the chain.
    pub quorum_threshold: StakeUnit,
    pub completed_locking_next_committee: bool,
    voted_to_lock_committee: HashSet<AuthorityName>,
    network_key_version: u8,
}

/// The data needed to manage the outputs of an MPC instance.
#[derive(Clone)]
pub struct InstanceOutputsData {
    // todo(zeev): cleanup.
    /// Maps session's output to the authorities that voted for it.
    pub session_output_to_voting_authorities:
        HashMap<(Vec<u8>, SessionInfo), HashSet<AuthorityName>>,
    /// Needed to make sure an authority does not send two outputs for the same session.
    pub authorities_that_sent_output: HashSet<AuthorityName>,
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
}

pub struct OutputVerificationResult {
    pub result: OutputResult,
    pub malicious_actors: Vec<AuthorityName>,
}

impl DWalletMPCOutputsVerifier {
    pub fn new(epoch_store: &AuthorityPerEpochStore) -> Self {
        let quorum_threshold = epoch_store.committee().quorum_threshold();
        let weighted_parties = epoch_store
            .committee()
            .voting_rights
            .clone()
            .into_iter()
            .collect();

        let network_key_version = epoch_store
            .get_encryption_of_decryption_key_shares()
            // Default to an empty HashMap if the key is not found.
            .unwrap_or_default()
            .get(&(DWalletMPCNetworkKey::Secp256k1 as u8))
            .map(|versions| versions.len() as u8)
            .unwrap_or(1);

        DWalletMPCOutputsVerifier {
            quorum_threshold,
            mpc_instances_outputs: HashMap::new(),
            weighted_parties,
            completed_locking_next_committee: false,
            voted_to_lock_committee: HashSet::new(),
            // Todo (#394): Remove hardcoded network key version.
            network_key_version,
        }
    }

    pub fn network_key_version(&self) -> u8 {
        self.network_key_version
    }

    /// Returns true if the `lock_next_epoch_committee` system TX should get called, a.k.a. a quorum of validators voted for it,
    /// and false otherwise.
    pub(crate) fn should_lock_committee(&mut self, authority_name: AuthorityName) -> bool {
        self.voted_to_lock_committee.insert(authority_name);
        self.voted_to_lock_committee
            .iter()
            .map(|voter_name| self.weighted_parties.get(voter_name).unwrap_or(&0))
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
        let Some(ref mut session) = self.mpc_instances_outputs.get_mut(&session_info.session_id)
        else {
            return Ok(OutputVerificationResult {
                result: OutputResult::Malicious,
                malicious_actors: vec![origin_authority],
            });
        };
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
        self.insert_new_output_instance(&session_info.session_id);
    }

    pub fn insert_new_output_instance(&mut self, session_id: &ObjectID) {
        self.mpc_instances_outputs.insert(
            session_id.clone(),
            InstanceOutputsData {
                session_output_to_voting_authorities: HashMap::new(),
                authorities_that_sent_output: HashSet::new(),
            },
        );
    }
}
