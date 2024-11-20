use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use pera_types::base_types::{AuthorityName, ObjectID, PeraAddress};
use pera_types::error::{PeraError, PeraResult};

use crate::dwallet_mpc::bytes_party::MPCParty;
use crate::dwallet_mpc::mpc_instance::{
    authority_name_to_party_id, DWalletMPCInstance, DWalletMPCMessage, MPCSessionStatus,
};
use group::PartyID;
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use mpc::{Error, WeightedThresholdAccessStructure};
use pera_config::NodeConfig;
use pera_types::committee::{EpochId, StakeUnit};
use pera_types::messages_consensus::ConsensusTransaction;
use pera_types::messages_dwallet_mpc::SessionInfo;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Weak};
use tracing::log::warn;
use tracing::{error, info};
use twopc_mpc::secp256k1::class_groups::DecryptionKeyShare;

/// The `MPCService` is responsible for managing MPC instances:
/// - keeping track of all MPC instances,
/// - executing all active instances, and
/// - (de)activating instances.
pub struct DWalletMPCManager {
    mpc_instances: HashMap<ObjectID, DWalletMPCInstance>,
    /// Used to keep track of the order in which pending instances are received so they are activated in order of arrival.
    pending_instances_queue: VecDeque<DWalletMPCInstance>,
    // TODO (#257): Make sure the counter is always in sync with the number of active instances.
    active_instances_counter: usize,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    pub node_config: NodeConfig,
    pub epoch_store: Weak<AuthorityPerEpochStore>,
    pub max_active_mpc_instances: usize,
    pub epoch_id: EpochId,
    /// A set of all the authorities that behaved maliciously at least once during the epoch. Any message/output from these authorities will be ignored.
    pub malicious_actors: HashSet<AuthorityName>,
    pub weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    pub weighted_parties: HashMap<PartyID, PartyID>,
}

/// The possible results of verifying an incoming output for an MPC session.
/// We need to differentiate between a duplicate & a malicious output, as the output can be sent twice by honest parties.
pub enum OutputVerificationResult {
    Valid,
    Duplicate,
    Malicious,
}

impl DWalletMPCManager {
    pub fn try_new(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        epoch_id: EpochId,
        node_config: NodeConfig,
    ) -> PeraResult<Self> {
        let weighted_parties: HashMap<PartyID, PartyID> = epoch_store
            .committee()
            .voting_rights
            .iter()
            .map(|(name, weight)| {
                Ok((
                    authority_name_to_party_id(name.clone(), &epoch_store)?,
                    *weight as PartyID,
                ))
            })
            .collect::<PeraResult<HashMap<PartyID, PartyID>>>()?;
        let weighted_threshold_access_structure = WeightedThresholdAccessStructure::new(
            epoch_store.committee().quorum_threshold() as PartyID,
            weighted_parties.clone(),
        )
        .map_err(|_| PeraError::InternalDWalletMPCError)?;
        Ok(Self {
            mpc_instances: HashMap::new(),
            pending_instances_queue: VecDeque::new(),
            active_instances_counter: 0,
            consensus_adapter,
            epoch_store: Arc::downgrade(&epoch_store),
            epoch_id,
            max_active_mpc_instances: node_config.max_active_dwallet_mpc_instances,
            node_config,
            malicious_actors: HashSet::new(),
            weighted_threshold_access_structure,
            weighted_parties,
        })
    }

    pub fn get_decryption_share(
        &self,
    ) -> PeraResult<twopc_mpc::secp256k1::class_groups::DecryptionKeyShare> {
        let party_id = authority_name_to_party_id(
            self.epoch_store()?.name.clone(),
            &self.epoch_store()?.clone(),
        )?;
        let try_this = self
            .node_config
            .dwallet_mpc_class_groups_decryption_shares
            .clone()
            .ok_or(PeraError::InternalDWalletMPCError)?
            .get(&party_id);
        let share = DecryptionKeyShare::new(
            party_id,
            self.node_config
                .dwallet_mpc_class_groups_decryption_shares
                .clone()
                .ok_or(PeraError::InternalDWalletMPCError)?
                .get(&party_id)
                .ok_or(PeraError::InternalDWalletMPCError)?
                .clone(),
            &self
                .node_config
                .dwallet_mpc_decryption_shares_public_parameters
                .clone()
                .unwrap(),
        )
        .map_err(|e| twopc_error_to_pera_error(e.into()))?;
        Ok(share)
    }

    /// Tries to verify that the received output for the MPC session matches the one generated locally.
    /// Returns true if the output is correct, false otherwise.
    // TODO (#311): Make validator don't mark other validators as malicious or take any active action while syncing
    pub fn try_verify_output(
        &mut self,
        output: &Vec<u8>,
        session_info: &SessionInfo,
    ) -> anyhow::Result<OutputVerificationResult> {
        let Some(instance) = self.mpc_instances.get_mut(&session_info.session_id) else {
            return Ok(OutputVerificationResult::Malicious);
        };
        let MPCSessionStatus::Finalizing(stored_output) = instance.status.clone() else {
            return Ok(OutputVerificationResult::Duplicate);
        };
        if *stored_output == *output
            && session_info.initiating_user_address.to_vec()
                == instance.session_info.initiating_user_address.to_vec()
            && session_info.dwallet_cap_id == instance.session_info.dwallet_cap_id
        {
            self.finalize_mpc_instance(session_info.session_id.clone())?;
            return Ok(OutputVerificationResult::Valid);
        }
        Ok(OutputVerificationResult::Malicious)
    }

    /// Advance all the MPC instances that either received enough messages to, or perform the first step of the flow.
    /// We parallelize the advances with Rayon to speed up the process.
    pub async fn handle_end_of_delivery(&mut self) -> PeraResult {
        let threshold = self.epoch_store()?.committee().quorum_threshold();
        let mut ready_to_advance = self
            .mpc_instances
            .iter_mut()
            .filter_map(|(_, instance)| {
                let received_weight: PartyID = instance
                    .pending_messages
                    .keys()
                    .map(|authority_index| {
                        // should never be "or" as we receive messages only from known authorities
                        self.weighted_parties.get(authority_index).unwrap_or(&0)
                    })
                    .sum();
                if (instance.status == MPCSessionStatus::Active
                    && received_weight as StakeUnit >= threshold)
                    || (instance.status == MPCSessionStatus::FirstExecution)
                {
                    Some(instance)
                } else {
                    None
                }
            })
            .collect::<Vec<&mut DWalletMPCInstance>>();

        let results: Vec<PeraResult<(ConsensusTransaction, Vec<PartyID>)>> = ready_to_advance
            .par_iter_mut()
            .map(|ref mut instance| instance.advance())
            .collect();
        let messages = results
            .into_iter()
            .filter_map(|result| {
                if let Err(PeraError::DWalletMPCMaliciousParties(malicious_parties)) = result {
                    self.flag_parties_as_malicious(malicious_parties).ok()?;
                    return None;
                } else if let Ok((message, malicious_parties)) = result {
                    self.flag_parties_as_malicious(malicious_parties).ok()?;
                    return Some(message);
                }
                None
            })
            .collect::<Vec<ConsensusTransaction>>();
        self.consensus_adapter
            .submit_to_consensus(&messages, &self.epoch_store()?)
            .await?;
        Ok(())
    }

    fn epoch_store(&self) -> PeraResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(PeraError::EpochEnded(self.epoch_id))
    }

    /// Handles a message by forwarding it to the relevant MPC instance
    /// If the instance does not exist, punish the sender
    pub fn handle_message(
        &mut self,
        message: &[u8],
        authority_name: AuthorityName,
        session_id: ObjectID,
    ) -> PeraResult<()> {
        if self.malicious_actors.contains(&authority_name) {
            return Ok(());
        }
        let Some(instance) = self.mpc_instances.get_mut(&session_id) else {
            warn!(
                "received a message for instance {:?} which does not exist",
                session_id
            );
            self.malicious_actors.insert(authority_name);
            return Ok(());
        };
        let handle_message_response = instance.handle_message(DWalletMPCMessage {
            message: message.to_vec(),
            authority: authority_name,
        });
        if let Err(PeraError::DWalletMPCMaliciousParties(malicious_parties)) =
            handle_message_response
        {
            self.flag_parties_as_malicious(malicious_parties)?;
            return Ok(());
        };
        handle_message_response
    }

    /// Convert the indices of the malicious parties to their addresses and store them
    /// in the malicious actors set
    /// New messages from these parties will be ignored
    fn flag_parties_as_malicious(&mut self, malicious_parties: Vec<PartyID>) -> PeraResult {
        let malicious_parties_names = malicious_parties
            .into_iter()
            .map(|party_id| {
                Ok(*self
                    .epoch_store()?
                    .committee()
                    .authority_by_index(party_id as u32)
                    .ok_or(PeraError::InvalidCommittee("".to_string()))?)
            })
            .collect::<PeraResult<Vec<AuthorityName>>>()?;
        warn!(
            "flagged the following parties as malicious: {:?}",
            malicious_parties_names
        );
        self.malicious_actors.extend(malicious_parties_names);
        Ok(())
    }

    /// Spawns a new MPC instance if the number of active instances is below the limit
    /// and the pending instances queue is empty. Otherwise, adds the instance to the pending queue
    pub fn push_new_mpc_instance(
        &mut self,
        auxiliary_input: Vec<u8>,
        party: MPCParty,
        session_info: SessionInfo,
    ) -> PeraResult {
        let session_id = session_info.session_id.clone();
        if self.mpc_instances.contains_key(&session_id) {
            // This should never happen, as the session ID is a move UniqueID
            error!(
                "Received start flow event for session ID {:?} that already exists",
                session_id
            );
            return Ok(());
        }

        info!("Received start flow event for session ID {:?}", session_id);
        let party_id = self
            .epoch_store()?
            .committee()
            .authority_index(&self.epoch_store()?.name)
            .unwrap();
        let mut new_instance = DWalletMPCInstance::new(
            Arc::clone(&self.consensus_adapter),
            self.epoch_store.clone(),
            self.epoch_id,
            party,
            MPCSessionStatus::Pending,
            auxiliary_input,
            session_info,
            self.get_decryption_share()?,
        );
        // TODO (#311): Make validator don't mark other validators as malicious or take any active action while syncing
        if self.active_instances_counter > self.max_active_mpc_instances
            || !self.pending_instances_queue.is_empty()
        {
            self.pending_instances_queue.push_back(new_instance);
            info!(
                "Added MPCInstance to pending queue for session_id {:?}",
                session_id
            );
            return Ok(());
        }
        new_instance.status = MPCSessionStatus::FirstExecution;
        self.mpc_instances.insert(session_id.clone(), new_instance);
        self.active_instances_counter += 1;
        info!(
            "Added MPCInstance to MPC manager for session_id {:?}",
            session_id
        );
        Ok(())
    }

    pub fn finalize_mpc_instance(&mut self, session_id: ObjectID) -> PeraResult {
        let instance = self.mpc_instances.get_mut(&session_id).ok_or_else(|| {
            PeraError::InvalidCommittee(format!(
                "Received a finalize event for session ID {:?} that does not exist",
                session_id
            ))
        })?;
        if let MPCSessionStatus::Finalizing(output) = &instance.status {
            instance.status = MPCSessionStatus::Finished(output.clone());
            let pending_instance = self.pending_instances_queue.pop_front();
            if let Some(mut instance) = pending_instance {
                instance.status = MPCSessionStatus::FirstExecution;
                self.mpc_instances
                    .insert(instance.session_info.session_id, instance);
            } else {
                self.active_instances_counter -= 1;
            }
            info!("Finalized MPCInstance for session_id {:?}", session_id);
            return Ok(());
        }
        Err(PeraError::Unknown(format!(
            "Received a finalize event for session ID {:?} that is not in the finalizing state; current state: {:?}",
            session_id, instance.status
        )))
    }
}

/// Convert a `twopc_mpc::Error` to a `PeraError`.
/// Needed this function and not a `From` implementation because when including the `twopc_mpc` crate
/// as a dependency in the `pera-types` crate there are many conflicting implementations.
pub fn twopc_error_to_pera_error(error: twopc_mpc::Error) -> PeraError {
    let Ok(error): Result<Error, _> = error.try_into() else {
        return PeraError::InternalDWalletMPCError;
    };
    return match error {
        Error::UnresponsiveParties(parties)
        | Error::InvalidMessage(parties)
        | Error::MaliciousMessage(parties) => PeraError::DWalletMPCMaliciousParties(parties),
        _ => PeraError::InternalDWalletMPCError,
    };
}
