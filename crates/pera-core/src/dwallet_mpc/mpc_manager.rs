use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use pera_types::base_types::{AuthorityName, ObjectID, PeraAddress};
use pera_types::error::{PeraError, PeraResult};

use crate::dwallet_mpc::bytes_party::{MPCParty, MPCSessionInfo};
use crate::dwallet_mpc::mpc_instance::{DWalletMPCInstance, DWalletMPCMessage, MPCSessionStatus};
use pera_types::committee::EpochId;
use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Weak};
use tracing::{error, info, warn};

/// The [`DWalletMPCManager`] manages MPC instances:
/// — Keeping track of all MPC instances,
/// — Executing all active instances, and
/// — (De)activating instances.
pub struct DWalletMPCManager {
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
    epoch_store: Weak<AuthorityPerEpochStore>,
    max_active_mpc_instances: usize,
    epoch_id: EpochId,
    /// The total number of parties in the chain
    /// We can calculate the threshold and parties IDs (indexes) from it.
    pub number_of_parties: usize,
}

/// The possible results of verifying an incoming output for an MPC session.
/// We need to differentiate between a duplicate and a malicious output,
/// as the output can be sent twice by honest parties.
pub enum OutputVerificationResult {
    Valid,
    Duplicate,
    Malicious,
}

impl DWalletMPCManager {
    pub fn new(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Weak<AuthorityPerEpochStore>,
        epoch_id: EpochId,
        max_active_mpc_instances: usize,
        number_of_parties: usize,
    ) -> Self {
        Self {
            mpc_instances: HashMap::new(),
            pending_instances_queue: VecDeque::new(),
            active_instances_counter: 0,
            consensus_adapter,
            epoch_store,
            epoch_id,
            max_active_mpc_instances,
            // TODO (#268): Take into account the validator's voting power
            number_of_parties,
        }
    }

    /// Tries to verify that the received output for the MPC session matches the
    /// one generated locally.
    /// Returns true if the output is correct, false otherwise.
    // TODO (#311): Make validator don't mark other validators
    // TODO (#311): as malicious or take any active action while syncing
    pub fn try_verify_output(
        &mut self,
        output: &Vec<u8>,
        session_id: &ObjectID,
        sender_address: &PeraAddress,
        dwallet_cap_id: &ObjectID,
    ) -> anyhow::Result<OutputVerificationResult> {
        let Some(instance) = self.mpc_instances.get(session_id) else {
            return Ok(OutputVerificationResult::Malicious);
        };
        let MPCSessionStatus::Finalizing(stored_output) = &instance.status else {
            return Ok(OutputVerificationResult::Duplicate);
        };
        if *stored_output == *output
            && sender_address == &instance.session_info.initiating_user_address
            && dwallet_cap_id == &instance.session_info.dwallet_cap_id
        {
            self.finalize_mpc_instance(session_id)?;
            return Ok(OutputVerificationResult::Valid);
        }
        Ok(OutputVerificationResult::Malicious)
    }

    /// Advance all the MPC instances that either received enough messages to,
    /// or perform the first step of the flow.
    /// We parallelize the advances with `Rayon` to speed up the process.
    /// TODO (#263): Implement logic to mark and punish validators responsible for failed advances.
    pub async fn handle_end_of_delivery(&mut self) -> PeraResult {
        // TODO (#268): Take the voting power into account when dealing with the threshold
        // The math here will be removed soon,
        // it is just a constant from the paper.
        let threshold_number_of_parties = ((self.number_of_parties * 2) + 2) / 3;

        // Process instances in parallel, advancing only those that meet the criteria.
        self.mpc_instances
            .par_iter_mut()
            .try_for_each(|(_, instance)| {
                if instance.status == MPCSessionStatus::FirstExecution
                    || (instance.status == MPCSessionStatus::Active
                        && instance.pending_messages.len() >= threshold_number_of_parties)
                {
                    // TODO (#263): Mark and punish the malicious validators
                    // TODO (#263): that caused some advances to fail
                    instance.advance().map_err(|e| {
                        warn!(?e, "failed to advance MPC instance: {:?}", instance.session_info);
                        e
                    })?;
                }
                Ok(())
            })
    }

    /// Handles a message by forwarding it to the relevant MPC session
    /// If the session does not exist, punish the sender.
    pub fn handle_message(
        &mut self,
        message: &[u8],
        authority_name: AuthorityName,
        session_id: ObjectID,
    ) -> PeraResult<()> {
        let Some(instance) = self.mpc_instances.get_mut(&session_id) else {
            error!(
                "received a message for instance {:?} which does not exist",
                session_id
            );
            // TODO (#261): Punish a validator that sends
            // TODO (#261): a message related to a non-existing mpc instance
            return Ok(());
        };
        instance.handle_message(DWalletMPCMessage {
            message: message.to_vec(),
            authority: authority_name,
        })
    }

    /// Spawns a new MPC instance if the number of active instances is below the limit.
    /// Otherwise, add the instance to the pending queue.
    pub fn push_new_mpc_instance(
        &mut self,
        auxiliary_input: Vec<u8>,
        party: MPCParty,
        session_info: MPCSessionInfo,
    ) {
        let session_id = session_info.session_id.clone();
        if self.mpc_instances.contains_key(&session_id) {
            // This should never happen, as the session ID is a move UniqueID.
            error!(
                "received a start flow MPC event for session ID {:?} that already exists",
                session_id
            );
            return;
        }

        info!("Received start flow MPC event for session ID {:?}", session_id);
        let mut new_instance = DWalletMPCInstance::new(
            Arc::clone(&self.consensus_adapter),
            self.epoch_store.clone(),
            self.epoch_id,
            party,
            MPCSessionStatus::Pending,
            auxiliary_input,
            session_info,
        );
        // TODO (#311): Make sure validator don't mark other validators
        // TODO (#311): as malicious or take any active action while syncing
        if self.active_instances_counter > self.max_active_mpc_instances
        {
            self.pending_instances_queue.push_back(new_instance);
            info!(
                "Added MPCInstance to pending queue for session_id {:?}",
                session_id
            );
            return;
        }
        new_instance.status = MPCSessionStatus::FirstExecution;
        self.mpc_instances.insert(session_id.clone(), new_instance);
        self.active_instances_counter += 1;
        info!(
            "Added MPCInstance to MPC manager for session_id {:?}",
            session_id
        );
    }

    fn finalize_mpc_instance(&mut self, session_id: &ObjectID) -> PeraResult {
        let instance = self.mpc_instances.get_mut(session_id).ok_or_else(|| {
            PeraError::InvalidCommittee(format!(
                "received a `Finalize` event for session ID `{:?}` that does not exist",
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
            "received a `Finalize` event for session ID `{:?}` that is not in the finalizing state; current state: {:?}",
            session_id, instance.status
        )))
    }
}
