use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::sign::BatchedSignSession;
use anyhow::anyhow;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::committee::StakeUnit;
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use std::collections::{HashMap, HashSet};

/// A struct to manage the DWallet MPC outputs.
/// It stores all the outputs received for each instance, and decides whether an output is valid
/// by checking if a validators with quorum of stake voted for it.
#[derive(Clone)]
pub struct DWalletMPCOutputsManager {
    /// The batched sign sessions that are currently being processed.
    pub batched_sign_sessions: HashMap<ObjectID, BatchedSignSession>,
    /// The outputs received for each instance.
    pub mpc_instances_outputs: HashMap<ObjectID, InstanceOutputsData>,
    /// A mapping between an authority name to its stake.
    pub weighted_parties: HashMap<AuthorityName, StakeUnit>,
    /// The quorum threshold of the chain.
    pub quorum_threshold: StakeUnit,
    pub completed_locking_next_committee: bool,
    voted_to_lock_committee: HashSet<AuthorityName>,
}

/// The data needed to manage the outputs of an MPC instance.
#[derive(Clone)]
pub struct InstanceOutputsData {
    /// needed to easily check if any of the outputs received a quorum of votes and should be written to the chain
    pub output_to_voting_authorities: HashMap<(Vec<u8>, SessionInfo), HashSet<AuthorityName>>,
    /// Needed to make sure an authority does not send two outputs for the same session
    pub authorities_that_sent_output: HashSet<AuthorityName>,
}

impl DWalletMPCOutputsManager {
    pub fn new(epoch_store: &AuthorityPerEpochStore) -> Self {
        DWalletMPCOutputsManager {
            batched_sign_sessions: HashMap::new(),
            quorum_threshold: epoch_store.committee().quorum_threshold(),
            mpc_instances_outputs: HashMap::new(),
            weighted_parties: epoch_store
                .committee()
                .voting_rights
                .clone()
                .into_iter()
                .collect(),
            completed_locking_next_committee: false,
            voted_to_lock_committee: HashSet::new(),
        }
    }

    pub fn should_lock_committee(&mut self, authority_name: AuthorityName) -> bool {
        self.voted_to_lock_committee.insert(authority_name);
        self.voted_to_lock_committee
            .iter()
            .map(|voter_name| self.weighted_parties.get(voter_name).unwrap_or(&0))
            .sum::<StakeUnit>()
            >= self.quorum_threshold
    }

    /// Stores the given MPC output, and checks if any of the received outputs already received a quorum of votes.
    /// If so, the output is returned along with a vector of malicious actors, i.e. parties that voted for other outputs.
    // TODO (#311): Make validator don't mark other validators as malicious or take any active action while syncing
    pub fn try_verify_output(
        &mut self,
        output: &Vec<u8>,
        session_info: &SessionInfo,
        origin_authority: AuthorityName,
    ) -> anyhow::Result<OutputVerificationResult> {
        let Some(ref mut session) = self.mpc_instances_outputs.get_mut(&session_info.session_id)
        else {
            return Ok(OutputVerificationResult::Malicious);
        };
        if session
            .authorities_that_sent_output
            .contains(&origin_authority)
        {
            return Ok(OutputVerificationResult::Malicious);
        }
        session
            .authorities_that_sent_output
            .insert(origin_authority.clone());
        session
            .output_to_voting_authorities
            .entry((output.clone(), session_info.clone()))
            .or_default()
            .insert(origin_authority);
        if let Some(agreed_output) =
            session
                .output_to_voting_authorities
                .iter()
                .find(|(output, voters)| {
                    voters
                        .iter()
                        .map(|voter_name| self.weighted_parties.get(voter_name).unwrap_or(&0))
                        .sum::<StakeUnit>()
                        >= self.quorum_threshold
                })
        {
            let voted_for_other_outputs: Vec<AuthorityName> = session
                .output_to_voting_authorities
                .iter()
                .filter(|(output, _)| *output != agreed_output.0)
                .flat_map(|(_, voters)| voters)
                .cloned()
                .collect();
            if let MPCRound::Sign(batch_session_id, hashed_message) = session_info.mpc_round.clone()
            {
                let batched_sign_session = self
                    .batched_sign_sessions
                    .get_mut(&batch_session_id)
                    .ok_or(anyhow!(
                        "failed to find batch for session id {}",
                        batch_session_id
                    ))?;
                batched_sign_session
                    .hashed_msg_to_signature
                    .insert(hashed_message.clone(), output.clone());
                if batched_sign_session.hashed_msg_to_signature.values().len()
                    == batched_sign_session.ordered_messages.len()
                {
                    let new_output: Vec<Vec<u8>> = batched_sign_session
                        .ordered_messages
                        .iter()
                        .map(|msg| {
                            Ok(batched_sign_session
                                .hashed_msg_to_signature
                                .get(msg)
                                .ok_or(anyhow!("failed to find message in batch {:?}", msg))?
                                .clone())
                        })
                        .collect::<anyhow::Result<Vec<Vec<u8>>>>()?;
                    return Ok(OutputVerificationResult::ValidWithNewOutput(
                        bcs::to_bytes(&new_output)?,
                        voted_for_other_outputs,
                    ));
                } else {
                    return Ok(OutputVerificationResult::ValidWithoutOutput(
                        voted_for_other_outputs,
                    ));
                }
            }
            return Ok(OutputVerificationResult::Valid(voted_for_other_outputs));
        }
        Ok(OutputVerificationResult::ValidWithoutOutput(vec![]))
    }

    pub fn handle_new_event(&mut self, session_info: &SessionInfo) {
        if let MPCRound::BatchedSign(hashed_messages) = &session_info.mpc_round {
            let mut seen = HashSet::new();
            let messages_without_duplicates = hashed_messages
                .clone()
                .into_iter()
                .filter(|x| seen.insert(x.clone()))
                .collect();
            self.batched_sign_sessions.insert(
                session_info.session_id,
                BatchedSignSession {
                    hashed_msg_to_signature: HashMap::new(),
                    ordered_messages: messages_without_duplicates,
                },
            );
        } else {
            self.insert_new_output_instance(&session_info.session_id);
        }
    }

    pub fn insert_new_output_instance(&mut self, session_id: &ObjectID) {
        self.mpc_instances_outputs.insert(
            session_id.clone(),
            InstanceOutputsData {
                output_to_voting_authorities: HashMap::new(),
                authorities_that_sent_output: HashSet::new(),
            },
        );
    }
}

/// The possible results of verifying an incoming output for an MPC session.
/// We need to differentiate between a duplicate & a malicious output, as the output can be sent twice by honest parties.
#[derive(PartialOrd, PartialEq)]
pub enum OutputVerificationResult {
    /// When working on a batch, e.g. signing on a batch of messages, we write the output to the chain only once - when the entire batch is ready.
    /// The returned value contains the new output, and the list of the malicious parties that voted for other outputs.
    ValidWithNewOutput(Vec<u8>, Vec<AuthorityName>),
    /// When the output is correct but not all the MPC flows in the batch have been completed.
    ValidWithoutOutput(Vec<AuthorityName>),
    Valid(Vec<AuthorityName>),
    Duplicate,
    Malicious,
}
