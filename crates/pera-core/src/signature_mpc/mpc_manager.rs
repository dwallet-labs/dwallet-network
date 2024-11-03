use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::{ConsensusAdapter, SubmitToConsensus};
use anyhow::anyhow;
use group::secp256k1::group_element::Value;
use group::{secp256k1, GroupElement, PartyID};
use im::hashmap;
use itertools::Itertools;
use mpc::Advance;
use mpc::{two_party::Round, AuxiliaryInput, Party};
use pera_types::base_types::{AuthorityName, ObjectID, PeraAddress};
use pera_types::error::{PeraError, PeraResult};
use pera_types::event::Event;
use pera_types::messages_consensus::ConsensusTransaction;

use crate::signature_mpc::bytes_party::{AdvanceResult, MPCParty, SessionInfo};
use pera_types::committee::EpochId;
use pera_types::messages_signature_mpc::MPCRound;
use rand_core::OsRng;
use rayon::prelude::*;
use schemars::_private::NoSerialize;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet, VecDeque};
use std::future::Future;
use std::io::Write;
use std::marker::PhantomData;
use std::sync::{Arc, Weak};
use std::time::Duration;
use std::{io, mem};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::sleep;
use tracing::{debug, error, info};

/// The message a validator can send to the other parties while running a signature MPC session.
#[derive(Clone)]
struct SignatureMPCMessage {
    /// The serialized message
    message: Vec<u8>,
    /// The authority that sent the message
    authority: AuthorityName,
}

/// Convert a given authority name (address) to it's corresponding party ID.
/// The party ID is the index of the authority in the committee.
pub fn authority_name_to_party_id(
    authority_name: AuthorityName,
    epoch_store: &AuthorityPerEpochStore,
) -> PeraResult<PartyID> {
    Ok(epoch_store
        .committee()
        .authority_index(&authority_name)
        // This should never happen, as the validator only accepts messages from committee members
        .ok_or_else(|| {
            PeraError::InvalidCommittee(
                "Received a proof MPC message from a validator that is not in the committee"
                    .to_string(),
            )
        })? as PartyID)
}

/// A Signature MPC session instance
/// It keeps track of the status of the session, the channel to send messages to the instance,
/// and the messages that are pending to be sent to the instance.
pub struct SignatureMPCInstance {
    status: MPCSessionStatus,
    /// The messages that are pending to be executed while advancing the instance
    /// We need to accumulate threshold of those before advancing the instance
    pending_messages: HashMap<PartyID, Vec<u8>>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    epoch_store: Weak<AuthorityPerEpochStore>,
    epoch_id: EpochId,
    /// The total number of parties in the chain
    /// We can calculate the threshold and parties IDs (indexes) from it
    /// To calculate the parties IDs all we need to know is the number of parties, as the IDs are just the indexes of those parties. If there are 3 parties, the IDs are [0, 1, 2].
    number_of_parties: usize,
    session_info: SessionInfo,
    /// The MPC party that being used to run the MPC cryptographic steps. An option because it can be None before the instance has started.
    party: MPCParty,
    auxiliary_input: Vec<u8>,
}

impl SignatureMPCInstance {
    fn new(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Weak<AuthorityPerEpochStore>,
        epoch: EpochId,
        number_of_parties: usize,
        party: MPCParty,
        status: MPCSessionStatus,
        auxiliary_input: Vec<u8>,
        session_info: SessionInfo,
    ) -> Self {
        Self {
            status,
            pending_messages: HashMap::new(),
            consensus_adapter: consensus_adapter.clone(),
            epoch_store: epoch_store.clone(),
            epoch_id: epoch,
            party,
            number_of_parties,
            auxiliary_input,
            session_info,
        }
    }

    fn epoch_store(&self) -> PeraResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(PeraError::EpochEnded(self.epoch_id))
    }

    /// Advances the MPC instance and optionally return a message the validator wants to send to the other MPC parties.
    /// Uses the existing party if it exists, otherwise creates a new one, as this is the first advance.
    fn advance(&mut self, auxiliary_input: Vec<u8>) -> PeraResult {
        let mut party = mem::take(&mut self.party);

        /// Gets the instance existing party or creates a new one if this is the first advance
        let advance_result = match party.advance(self.pending_messages.clone(), auxiliary_input) {
            Ok(res) => res,
            Err(e) => {
                println!("Error: {:?}", e);
                // TODO (#263): Mark and punish the malicious validators that caused this advance to fail
                self.pending_messages.clear();
                return Ok(());
            }
        };
        let msg = match advance_result {
            AdvanceResult::Advance((message, new_party)) => {
                self.status = MPCSessionStatus::Active;
                self.pending_messages.clear();
                self.party = new_party;
                self.new_signature_mpc_message(message)
            }
            AdvanceResult::Finalize(output) => {
                // TODO (#238): Verify the output and write it to the chain
                self.status = MPCSessionStatus::Finalizing(output.clone().into());
                self.new_dwallet_mpc_output_message(output.into(), self.session_info.mpc_round)
            }
        };

        let consensus_adapter = Arc::clone(&self.consensus_adapter);
        let epoch_store = Arc::clone(&self.epoch_store()?);
        if let Some(msg) = msg {
            /// Spawns sending this message asynchronously the [`self.advance`] function will stay synchronous
            /// and can be parallelized with Rayon.
            tokio::spawn(async move {
                let _ = consensus_adapter
                    .submit_to_consensus(&vec![msg], &epoch_store)
                    .await;
            });
        }
        Ok(())
    }

    /// Create a new consensus transaction with the message to be sent to the other MPC parties.
    /// Returns None only if the epoch switched in the middle and was not available.
    fn new_signature_mpc_message(&self, message: Vec<u8>) -> Option<ConsensusTransaction> {
        let Ok(epoch_store) = self.epoch_store() else {
            return None;
        };
        Some(ConsensusTransaction::new_signature_mpc_message(
            epoch_store.name,
            message,
            self.session_info.session_id.clone(),
        ))
    }

    /// Create a new consensus transaction with the flow result (output) to be sent to the other MPC parties.
    /// Returns None if the epoch switched in the middle and was not available or if this party is not the aggregator.
    /// Only the aggregator party should send the output to the other parties.
    fn new_dwallet_mpc_output_message(
        &self,
        output: Vec<u8>,
        mpc_round: MPCRound,
    ) -> Option<ConsensusTransaction> {
        let Ok(epoch_store) = self.epoch_store() else {
            return None;
        };
        if authority_name_to_party_id(epoch_store.name, &epoch_store).unwrap() != 3 {
            return None;
        }
        Some(ConsensusTransaction::new_dwallet_mpc_output(
            output,
            self.session_info.session_id.clone(),
            self.session_info.event_emitter.clone(),
            self.session_info.dwallet_cap_id.clone(),
            mpc_round,
        ))
    }

    /// Stores a message in the pending messages map. The code stores every new message it receives for that instance,
    /// and when we reach the end of delivery we will advance the instance if we have a threshold of messages.
    fn store_message(
        &mut self,
        message: &SignatureMPCMessage,
        epoch_store: Arc<AuthorityPerEpochStore>,
    ) -> PeraResult<()> {
        let party_id = authority_name_to_party_id(message.authority, &epoch_store)?;
        if self.pending_messages.contains_key(&party_id) {
            // TODO(#260): Punish an authority that sends multiple messages in the same round
            return Ok(());
        }

        self.pending_messages
            .insert(party_id, message.message.clone());
        Ok(())
    }

    /// Handles a message by either forwarding it to the instance or ignoring it if the instance is finished.
    fn handle_message(&mut self, message: SignatureMPCMessage) -> PeraResult<()> {
        match self.status {
            MPCSessionStatus::Active => self.store_message(&message, self.epoch_store()?),
            MPCSessionStatus::Finalizing(_) | MPCSessionStatus::Finished(_) => {
                // Do nothing
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

/// Possible statuses of an MPC session:
/// - Pending: The instance has been inserted after we reached [`SignatureMPCManager::max_active_mpc_instances`], so it's waiting
/// for some active instances to finish before it can be activated.
/// - FirstExecution: The [`SignatureMPCInstance::party`] has not yet performed it's first advance. This status is needed
/// so we will be able to filter those instances and advance them, despite they have not received [`threshold_number_of_parties`] messages.
/// - Active: The session is currently running; new messages will be forwarded to the session.
/// - Finalizing: The session is finished and pending on chain write; after receiving an output, it will be verified
/// against the local one, and if they match the status will be changed to Finished.
/// This is needed so we won't write the same output twice to the chain.
/// - Finished: The session removed from active instances; incoming messages will not be forwarded,
/// but will not be marked as malicious.
#[derive(Clone, PartialEq, Debug)]
enum MPCSessionStatus {
    Pending,
    FirstExecution,
    Active,
    Finalizing(Vec<u8>),
    Finished(Vec<u8>),
}

/// The `MPCService` is responsible for managing MPC instances:
/// - keeping track of all MPC instances,
/// - executing all active instances, and
/// - (de)activating instances.
pub struct SignatureMPCManager {
    mpc_instances: HashMap<ObjectID, SignatureMPCInstance>,
    /// Used to keep track of the order in which pending instances are received so they are activated in order of arrival.
    pending_instances_queue: VecDeque<SignatureMPCInstance>,
    // TODO (#257): Make sure the counter is always in sync with the number of active instances.
    active_instances_counter: usize,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    pub epoch_store: Weak<AuthorityPerEpochStore>,
    pub max_active_mpc_instances: usize,
    pub epoch_id: EpochId,
    /// The total number of parties in the chain
    /// We can calculate the threshold and parties IDs (indexes) from it
    pub number_of_parties: usize,
}

/// Needed to be able to iterate over a vector of generic MPCInstances with Rayon
unsafe impl Send for SignatureMPCInstance {}

impl SignatureMPCManager {
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

    /// Tries to verify that the received output for the MPC session matches the one generated locally.
    /// Returns true if the output is correct, false otherwise.
    pub fn try_verify_output(
        &self,
        output: &Vec<u8>,
        session_id: &ObjectID,
    ) -> anyhow::Result<bool> {
        let Some(instance) = self.mpc_instances.get(session_id) else {
            return Ok(false);
        };
        let MPCSessionStatus::Finalizing(stored_output) = &instance.status else {
            return Ok(false);
        };

        Ok(*stored_output == *output)
    }

    /// Advance all the MPC instances that either received enough messages to, or perform the first step of the flow.
    /// We parallelize the advances with Rayon to speed up the process.
    pub async fn handle_end_of_delivery(&mut self) -> PeraResult {
        let mut ready_to_advance = self
            .mpc_instances
            .iter_mut()
            .filter_map(|(_, instance)| {
                let threshold_number_of_parties = ((self.number_of_parties * 2) + 2) / 3;
                if (instance.status == MPCSessionStatus::Active
                    && instance.pending_messages.len() >= threshold_number_of_parties)
                    || (instance.status == MPCSessionStatus::FirstExecution)
                {
                    Some(instance)
                } else {
                    None
                }
            })
            .collect::<Vec<&mut SignatureMPCInstance>>();

        ready_to_advance
            .par_iter_mut()
            // TODO (#263): Mark and punish the malicious validators that caused some advances to return None, a.k.a to fail
            .map(|ref mut instance| instance.advance(instance.auxiliary_input.clone()))
            .collect::<PeraResult<_>>()?;
        Ok(())
    }

    /// Handles a message by forwarding it to the relevant MPC instance
    /// If the instance does not exist, punish the sender
    pub fn handle_message(
        &mut self,
        message: &[u8],
        authority_name: AuthorityName,
        session_id: ObjectID,
    ) -> PeraResult<()> {
        let Some(mut instance) = self.mpc_instances.get_mut(&session_id) else {
            error!(
                "received a message for instance {:?} which does not exist",
                session_id
            );
            // TODO (#261): Punish a validator that sends a message related to a non-existing mpc instance
            return Ok(());
        };
        instance.handle_message(SignatureMPCMessage {
            message: message.to_vec(),
            authority: authority_name,
        })
    }

    /// Spawns a new MPC instance if the number of active instances is below the limit
    /// Otherwise, adds the instance to the pending queue
    pub fn push_new_mpc_instance(
        &mut self,
        auxiliary_input: Vec<u8>,
        party: MPCParty,
        session_info: SessionInfo,
    ) {
        let session_id = session_info.session_id.clone();
        if self.mpc_instances.contains_key(&session_id) {
            // This should never happen, as the session ID is a move UniqueID
            error!(
                "Received start flow event for session ID {:?} that already exists",
                session_id
            );
            return;
        }

        info!("Received start flow event for session ID {:?}", session_id);
        let mut new_instance = SignatureMPCInstance::new(
            Arc::clone(&self.consensus_adapter),
            self.epoch_store.clone(),
            self.epoch_id,
            self.number_of_parties,
            // party.clone(),
            party,
            MPCSessionStatus::Pending,
            auxiliary_input,
            session_info,
        );
        if self.active_instances_counter > self.max_active_mpc_instances {
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

    pub fn finalize_mpc_instance(&mut self, session_id: ObjectID) -> PeraResult {
        let mut instance = self.mpc_instances.get_mut(&session_id).ok_or_else(|| {
            PeraError::InvalidCommittee(format!(
                "Received a finalize event for session ID {:?} that does not exist",
                session_id
            ))
        })?;
        if let MPCSessionStatus::Finalizing(output) = &instance.status {
            instance.status = MPCSessionStatus::Finished(output.clone());
            self.active_instances_counter -= 1;
            info!("Finalized MPCInstance for session_id {:?}", session_id);
            return Ok(());
        }
        Err(PeraError::Unknown(format!(
            "Received a finalize event for session ID {:?} that is not in the finalizing state; current state: {:?}",
            session_id, instance.status
        )))
    }
}
