use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::error::PeraResult;

use crate::dwallet_mpc::mpc_events::StartBatchedSignEvent;
use crate::dwallet_mpc::mpc_instance::DWalletMPCInstance;
use crate::dwallet_mpc::mpc_party::MPCParty;
use crate::dwallet_mpc::sign::BatchedSignSession;
use crate::dwallet_mpc::{authority_name_to_party_id, DWalletMPCMessage};
use group::PartyID;
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use mpc::WeightedThresholdAccessStructure;
use pera_config::NodeConfig;
use pera_mpc_types::dwallet_mpc::MPCSessionStatus;
use pera_types::committee::{EpochId, StakeUnit};
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::event::Event;
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Weak};
use tracing::log::warn;
use tracing::{error, info};
use twopc_mpc::secp256k1::class_groups::DecryptionKeyShare;

/// The [`DWalletMPCManager`] manages MPC instances:
/// — Keeping track of all MPC instances,
/// — Executing all active instances, and
/// — (De)activating instances.
pub struct DWalletMPCManager {
    party_id: PartyID,
    batched_sign_sessions: HashMap<ObjectID, BatchedSignSession>,
    /// Holds the active MPC instances, cleaned every epoch switch.
    mpc_instances: HashMap<ObjectID, DWalletMPCInstance>,
    /// Used to keep track of the order in which pending instances are received,
    /// so they are activated in order of arrival.
    pending_instances_queue: VecDeque<DWalletMPCInstance>,
    // TODO (#257): Make sure the counter is always in sync with the number of active instances.
    /// Keep track of the active instances to avoid exceeding the limit.
    /// We can't use the length of `mpc_instances` since it is never cleaned.
    active_instances_counter: usize,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    pub(crate) node_config: NodeConfig,
    epoch_store: Weak<AuthorityPerEpochStore>,
    max_active_mpc_sessions: usize,
    epoch_id: EpochId,
    /// A set of all the authorities that behaved maliciously at least once during the epoch.
    /// Any message/output from these authorities will be ignored.
    pub(crate) malicious_actors: HashSet<AuthorityName>,
    weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    // todo(zeev): DOC and why is it PartyID twice?
    weighted_parties: HashMap<PartyID, PartyID>,
}

/// The result of verifying an incoming output for an MPC session.
/// We need to differentiate between a duplicate and a malicious output,
/// as the output can be sent twice by honest parties.
pub enum OutputVerificationResult {
    // todo(zeev): why do we even need this special case?
    /// When working on a batch, e.g., signing on a batch of messages,
    /// we write the output to the chain only once — when the entire batch is ready.
    ValidWithNewOutput(Vec<u8>),
    // todo(zeev): maybe replace this with just Valid?
    /// When the output is correct but not all the MPC flows in
    /// the batch have been completed.
    ValidWithoutOutput,
    Valid,
    Duplicate,
    Malicious,
}

// todo(zeev): doc this.
impl DWalletMPCManager {
    pub fn try_new(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        epoch_id: EpochId,
        node_config: NodeConfig,
    ) -> DwalletMPCResult<Self> {
        let weighted_parties: HashMap<PartyID, PartyID> = epoch_store
            .committee()
            .voting_rights
            .iter()
            .map(|(name, weight)| {
                Ok((
                    authority_name_to_party_id(&name, &epoch_store)?,
                    // todo(zeev): StakeUnit?
                    *weight as PartyID,
                ))
            })
            .collect::<DwalletMPCResult<HashMap<PartyID, PartyID>>>()?;
        let weighted_threshold_access_structure = WeightedThresholdAccessStructure::new(
            // todo(zeev): StakeUnit?
            epoch_store.committee().quorum_threshold() as PartyID,
            weighted_parties.clone(),
        )
        .map_err(|e| DwalletMPCError::MPCManagerError(format!("{}", e)))?;
        Ok(Self {
            mpc_instances: HashMap::new(),
            pending_instances_queue: VecDeque::new(),
            active_instances_counter: 0,
            consensus_adapter,
            party_id: authority_name_to_party_id(&epoch_store.name, &epoch_store)?,
            epoch_store: Arc::downgrade(&epoch_store),
            epoch_id,
            max_active_mpc_sessions: node_config.max_active_dwallet_mpc_sessions,
            node_config,
            malicious_actors: HashSet::new(),
            weighted_threshold_access_structure,
            weighted_parties,
            batched_sign_sessions: HashMap::new(),
        })
    }

    // todo(zeev): doc this.
    pub fn get_decryption_share(&self) -> DwalletMPCResult<DecryptionKeyShare> {
        let epoch_store = self.epoch_store()?;
        let party_id = authority_name_to_party_id(&epoch_store.name, &epoch_store)?;
        let shares = self
            .node_config
            .dwallet_mpc_class_groups_decryption_shares
            .as_ref()
            .ok_or(DwalletMPCError::MissingDwalletMPCClassGroupsDecryptionShares)?;

        let share_value = shares
            .get(&party_id)
            .ok_or(DwalletMPCError::DwalletMPCClassGroupsDecryptionShareMissing(party_id))?
            .clone();

        let public_parameters = self
            .node_config
            .dwallet_mpc_decryption_shares_public_parameters
            .as_ref()
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionSharesPublicParameters)?;

        DecryptionKeyShare::new(party_id, share_value, public_parameters)
            .map_err(|e| DwalletMPCError::TwoPCMPCError(e.to_string()))
    }

    /// Tries to verify that the received output for the MPC session matches the
    /// one generated locally.
    /// Returns true if the output is correct, false otherwise.
    // TODO (#311): Make validator don't mark other validators
    // TODO (#311): as malicious or take any active action while syncing
    pub fn try_verify_output(
        &mut self,
        output: &Vec<u8>,
        session_info: &SessionInfo,
    ) -> DwalletMPCResult<OutputVerificationResult> {
        // Check if the instance exists for the given session ID.
        let Some(instance) = self.mpc_instances.get(&session_info.session_id) else {
            return Ok(OutputVerificationResult::Malicious);
        };

        // Ensure the instance is in the "Finalizing" status.
        let MPCSessionStatus::Finalizing(stored_output) = &instance.status else {
            return Ok(OutputVerificationResult::Duplicate);
        };

        // Validate the output and session information.
        if stored_output != output
            || session_info.initiating_user_address != instance.session_info.initiating_user_address
            || session_info.dwallet_cap_id != instance.session_info.dwallet_cap_id
        {
            return Ok(OutputVerificationResult::Malicious);
        }
        // clone to enable mutable borrow later.
        let mpc_round = instance.session_info.mpc_round.clone();

        // Finalize the MPC instance.
        // todo(zeev): why is it here? maybe move to the end.
        self.finalize_mpc_instance(&session_info.session_id)?;

        // Handle batched signature cases.
        if let MPCRound::Sign(_, batch_session_id, hashed_message) = &mpc_round {
            let batched_sign_session = self.batched_sign_sessions.get_mut(batch_session_id).ok_or(
                DwalletMPCError::MPCSessionNotFound {
                    session_id: *batch_session_id,
                },
            )?;

            batched_sign_session
                .hashed_msg_to_signature
                .insert(hashed_message.clone(), output.clone());

            // Check if all messages in the batch are processed.
            if batched_sign_session.hashed_msg_to_signature.len()
                == batched_sign_session.ordered_messages.len()
            {
                // Collect the ordered output messages.
                let new_output: Vec<Vec<u8>> = batched_sign_session
                    .ordered_messages
                    .iter()
                    .map(|msg| {
                        batched_sign_session
                            .hashed_msg_to_signature
                            .get(msg)
                            .cloned()
                            .ok_or_else(|| DwalletMPCError::MissingMessageInBatch(msg.clone()))
                    })
                    .collect::<DwalletMPCResult<Vec<Vec<u8>>>>()?;

                return Ok(OutputVerificationResult::ValidWithNewOutput(
                    // todo(zeev): check if the From is working automatically.
                    bcs::to_bytes(&new_output)?,
                ));
            }
            return Ok(OutputVerificationResult::ValidWithoutOutput);
        }
        Ok(OutputVerificationResult::Valid)
    }

    /// Checks and handles a batched sign event if the current event is a batched sign event.
    /// Returns true if the event is a batched sign event, false otherwise.
    pub fn check_for_batched_sign_event(&mut self, event: &Event) -> DwalletMPCResult<bool> {
        if event.type_ != StartBatchedSignEvent::type_() {
            return Ok(false);
        }

        let deserialized_event: StartBatchedSignEvent = bcs::from_bytes(&event.contents)?;

        // Remove duplicates from the hashed messages.
        let messages_without_duplicates: Vec<_> = {
            let mut seen = HashSet::new();
            deserialized_event
                .hashed_messages
                .into_iter()
                .filter(|msg| seen.insert(msg.clone()))
                .collect()
        };

        self.batched_sign_sessions.insert(
            deserialized_event.session_id.bytes,
            BatchedSignSession {
                hashed_msg_to_signature: HashMap::new(),
                ordered_messages: messages_without_duplicates,
            },
        );

        Ok(true)
    }

    /// Advance all the MPC instances that either received enough messages
    /// or perform the first step of the flow.
    /// We parallelize the advances with `Rayon` to speed up the process.
    pub async fn handle_end_of_delivery(&mut self) -> PeraResult {
        let threshold = self.epoch_store()?.committee().quorum_threshold();
        let mut malicious_parties = vec![];
        let mut messages = vec![];

        let mut ready_to_advance = self
            .mpc_instances
            .iter_mut()
            .filter_map(|(_, instance)| match instance.status {
                MPCSessionStatus::Active(round) => {
                    let received_weight: StakeUnit = instance.pending_messages[round]
                        .keys()
                        .map(|authority_index| {
                            *self.weighted_parties.get(authority_index).unwrap_or(&0) as StakeUnit
                        })
                        .sum();
                    if received_weight >= threshold {
                        return Some(instance);
                    }
                    return None;
                }
                MPCSessionStatus::FirstExecution => Some(instance),
                _ => None,
            })
            .collect::<Vec<&mut DWalletMPCInstance>>();

        ready_to_advance
            .par_iter_mut()
            .map(|instance| {
                instance.advance(&self.weighted_threshold_access_structure, self.party_id)
            })
            .collect::<Vec<_>>()
            // Convert back to an iterator for processing.
            .into_iter()
            .try_for_each(|result| match result {
                Ok((message, malicious)) => {
                    messages.push(message.clone());
                    malicious_parties.extend(malicious);
                    Ok(())
                }
                Err(DwalletMPCError::MaliciousParties(malicious)) => {
                    malicious_parties.extend(malicious);
                    Ok(())
                }
                // todo(zeev): if there is a fatal error, should we abort?
                Err(e) => Err(e),
            })?;

        self.flag_parties_as_malicious(&malicious_parties)?;

        // Need to send the messages' one by one, so the consensus adapter won't think they
        // are a [soft bundle](https://github.com/sui-foundation/sips/pull/19).
        for message in messages {
            self.consensus_adapter
                .submit_to_consensus(&vec![message], &self.epoch_store()?)
                .await?;
        }
        Ok(())
    }

    fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }

    /// Handles a message by forwarding it to the relevant MPC instance.
    /// If the instance does not exist, punishes the sender.
    pub(crate) fn handle_message(
        &mut self,
        message: &[u8],
        authority_name: AuthorityName,
        session_id: ObjectID,
    ) -> DwalletMPCResult<()> {
        // Ignore messages from known malicious actors.
        if self.malicious_actors.contains(&authority_name) {
            return Ok(());
        }

        let instance = match self.mpc_instances.get_mut(&session_id) {
            Some(instance) => instance,
            None => {
                warn!(
                    "received a message for instance {:?} which does not exist",
                    session_id
                );
                self.malicious_actors.insert(authority_name);
                return Ok(());
            }
        };

        match instance.handle_message(&DWalletMPCMessage {
            message: message.to_vec(),
            authority: authority_name,
        }) {
            Err(DwalletMPCError::MaliciousParties(malicious_parties)) => {
                self.flag_parties_as_malicious(&malicious_parties)?;
                Ok(())
            }
            other => other,
        }
    }

    /// Convert the indices of the malicious parties to their addresses and store them
    /// in the malicious actors set.
    /// New messages from these parties will be ignored.
    /// todo(zeev): clarify if it's restarted on epoch change.
    fn flag_parties_as_malicious(&mut self, malicious_parties: &[PartyID]) -> DwalletMPCResult<()> {
        let malicious_parties_names = malicious_parties
            .iter()
            .map(|party_id| {
                self.epoch_store()?
                    .committee()
                    .authority_by_index(*party_id as u32)
                    .cloned()
                    .ok_or(DwalletMPCError::AuthorityIndexNotFound(*party_id))
            })
            .collect::<DwalletMPCResult<Vec<AuthorityName>>>()?;
        warn!(
            "[dWallet MPC] Flagged the following parties as malicious: {:?}",
            malicious_parties_names
        );
        self.malicious_actors.extend(malicious_parties_names);
        Ok(())
    }

    /// Spawns a new MPC instance if the number of active instances is below the limit.
    /// Otherwise, add the instance to the pending queue.
    pub(crate) fn push_new_mpc_instance(
        &mut self,
        auxiliary_input: Vec<u8>,
        party: MPCParty,
        session_info: SessionInfo,
    ) -> DwalletMPCResult<()> {
        // let session_id = session_info.session_id;
        if self.mpc_instances.contains_key(&session_info.session_id) {
            // This should never happen, as the session ID is a Move UniqueID.
            error!(
                "received start flow event for session ID {:?} that already exists",
                &session_info.session_id
            );
            return Ok(());
        }
        info!(
            "Received start MPC flow event for session ID {:?}",
            session_info.session_id
        );
        let mut new_instance = DWalletMPCInstance::new(
            self.epoch_store.clone(),
            self.epoch_id,
            party,
            MPCSessionStatus::Pending,
            auxiliary_input,
            session_info.clone(),
            self.get_decryption_share()?,
        );
        // TODO (#311): Make sure validator don't mark other validators
        // TODO (#311): as malicious or take any active action while syncing
        // todo(zeev): remvoed             || !self.pending_instances_queue.is_empty()
        if self.active_instances_counter > self.max_active_mpc_sessions {
            self.pending_instances_queue.push_back(new_instance);
            info!(
                "Added MPCInstance to pending queue for session_id {:?}",
                &session_info.session_id
            );
            return Ok(());
        }
        new_instance.status = MPCSessionStatus::FirstExecution;
        self.mpc_instances
            .insert(session_info.session_id, new_instance);
        self.active_instances_counter += 1;
        info!(
            "Added MPCInstance to MPC manager for session_id {:?}",
            session_info.session_id
        );
        Ok(())
    }

    // todo(zeev): doc
    fn finalize_mpc_instance(&mut self, session_id: &ObjectID) -> DwalletMPCResult<()> {
        let instance = self
            .mpc_instances
            .get_mut(session_id)
            .ok_or_else(|| DwalletMPCError::FinalizeEventSessionNotFound(*session_id))?;

        if let MPCSessionStatus::Finalizing(output) = &instance.status {
            instance.status = MPCSessionStatus::Finished(output.clone());
            let pending_instance = self.pending_instances_queue.pop_front();
            if let Some(mut instance) = pending_instance {
                instance.status = MPCSessionStatus::FirstExecution;
                // todo(zeev): why don't we increment the counter here?
                self.mpc_instances
                    .insert(instance.session_info.session_id, instance);
            } else {
                self.active_instances_counter -= 1;
            }
            info!("Finalized MPCInstance for session_id {:?}", session_id);
            return Ok(());
        }
        Err(DwalletMPCError::InvalidFinalizeState {
            session_id: *session_id,
            status: instance.status.clone(),
        })
    }
}
