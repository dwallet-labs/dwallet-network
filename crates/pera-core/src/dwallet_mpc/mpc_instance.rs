use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::bytes_party::{AdvanceResult, MPCParty, SessionInfo};
use group::PartyID;
use pera_types::base_types::{AuthorityName, EpochId};
use pera_types::error::{PeraError, PeraResult};
use pera_types::messages_consensus::ConsensusTransaction;
use pera_types::messages_dwallet_mpc::MPCRound;
use std::collections::HashMap;
use std::mem;
use std::sync::{Arc, Weak};

/// The message a validator can send to the other parties while running a dwallet MPC session.
#[derive(Clone)]
pub struct DWalletMPCMessage {
    /// The serialized message
    pub(crate) message: Vec<u8>,
    /// The authority that sent the message
    pub(crate) authority: AuthorityName,
}

/// A DWallet MPC session instance
/// It keeps track of the status of the session, the channel to send messages to the instance,
/// and the messages that are pending to be sent to the instance.
pub struct DWalletMPCInstance {
    /// The status of the MPC instance
    pub(crate) status: MPCSessionStatus,
    /// The messages that are pending to be executed while advancing the instance
    /// We need to accumulate threshold of those before advancing the instance
    pub(crate) pending_messages: HashMap<PartyID, Vec<u8>>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    epoch_store: Weak<AuthorityPerEpochStore>,
    epoch_id: EpochId,
    /// The total number of parties in the chain
    /// We can calculate the threshold and parties IDs (indexes) from it
    /// To calculate the parties IDs all we need to know is the number of parties, as the IDs are just the indexes of those parties. If there are 3 parties, the IDs are [0, 1, 2].
    pub(crate) session_info: SessionInfo,
    /// The MPC party that being used to run the MPC cryptographic steps. An option because it can be None before the instance has started.
    party: MPCParty,
    pub(crate) auxiliary_input: Vec<u8>,
}

impl DWalletMPCInstance {
    pub(crate) fn new(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Weak<AuthorityPerEpochStore>,
        epoch: EpochId,
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
    pub(crate) fn advance(&mut self, auxiliary_input: Vec<u8>) -> PeraResult {
        let party = mem::take(&mut self.party);

        // Gets the instance existing party or creates a new one if this is the first advance
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
                self.new_dwallet_mpc_message(message)
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
            // Spawns sending this message asynchronously the [`self.advance`] function will stay synchronous
            // and can be parallelized with Rayon.
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
    fn new_dwallet_mpc_message(&self, message: Vec<u8>) -> Option<ConsensusTransaction> {
        let Ok(epoch_store) = self.epoch_store() else {
            return None;
        };
        Some(ConsensusTransaction::new_dwallet_mpc_message(
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
        Some(ConsensusTransaction::new_dwallet_mpc_output(
            output,
            self.session_info.session_id.clone(),
            self.session_info.initiating_user_address.clone(),
            self.session_info.dwallet_cap_id.clone(),
            mpc_round,
        ))
    }

    /// Stores a message in the pending messages map. The code stores every new message it receives for that instance,
    /// and when we reach the end of delivery we will advance the instance if we have a threshold of messages.
    fn store_message(
        &mut self,
        message: &DWalletMPCMessage,
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
    pub(crate) fn handle_message(&mut self, message: DWalletMPCMessage) -> PeraResult<()> {
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
/// - Pending: The instance has been inserted after we reached [`DWalletMPCManager::max_active_mpc_instances`], so it's waiting
/// for some active instances to finish before it can be activated.
/// - FirstExecution: The [`DWalletMPCInstance::party`] has not yet performed it's first advance. This status is needed
/// so we will be able to filter those instances and advance them, despite they have not received [`threshold_number_of_parties`] messages.
/// - Active: The session is currently running; new messages will be forwarded to the session.
/// - Finalizing: The session is finished and pending on chain write; after receiving an output, it will be verified
/// against the local one, and if they match the status will be changed to Finished.
/// This is needed so we won't write the same output twice to the chain.
/// - Finished: The session removed from active instances; incoming messages will not be forwarded,
/// but will not be marked as malicious.
#[derive(Clone, PartialEq, Debug)]
pub enum MPCSessionStatus {
    Pending,
    FirstExecution,
    Active,
    Finalizing(Vec<u8>),
    Finished(Vec<u8>),
}

/// Needed to be able to iterate over a vector of generic MPCInstances with Rayon
unsafe impl Send for DWalletMPCInstance {}

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
                "Received a dwallet MPC message from a validator that is not in the committee"
                    .to_string(),
            )
        })? as PartyID)
}
