use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::bytes_party::{AdvanceResult, MPCParty, MPCSessionInfo};
use anyhow::Context;
use group::PartyID;
use pera_types::base_types::{AuthorityName, EpochId};
use pera_types::dwallet_mpc::{MPCMessage, MPCOutput, MPCSessionStatus};
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::error::{PeraError, PeraResult};
use pera_types::messages_consensus::ConsensusTransaction;
use pera_types::messages_dwallet_mpc::MPCRound;
use std::collections::HashMap;
use std::mem;
use std::sync::{Arc, Weak};
use tracing::warn;

/// The message a Validator can send to the other parties while
/// running a dWallet MPC session.
#[derive(Clone)]
pub struct DWalletMPCMessage {
    /// The serialized message.
    pub(super) message: MPCMessage,
    /// The authority (Validator) that sent the message.
    pub(super) authority: AuthorityName,
}
// todo(zeev): rename all instance to session.

/// Needed to be able to iterate over a vector of generic MPCInstances with Rayon.
unsafe impl Send for DWalletMPCSession {}

/// A dWallet MPC session instance
/// It keeps track of the session, the channel to send messages to the instance,
/// and the messages that are pending to be sent to the instance.
pub struct DWalletMPCSession {
    /// The status of the MPC instance.
    pub(super) status: MPCSessionStatus,
    /// The messages that are pending to be executed while advancing the instance
    /// We need to accumulate the threshold of those before advancing the instance.
    pub(super) pending_messages: HashMap<PartyID, MPCMessage>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    epoch_store: Weak<AuthorityPerEpochStore>,
    epoch_id: EpochId,
    /// The total number of parties in the chain.
    /// We can calculate the threshold and party IDs (indexes) from it.
    /// To calculate the party IDs, all we need to know is the number of parties,
    /// as the IDs are just the indexes of those parties.
    /// If there are three parties, the IDs are [0, 1, 2].
    pub(super) session_info: MPCSessionInfo,
    /// The MPC party being used to run the MPC cryptographic steps.
    /// Party in here is not a Validator, but a cryptographic party.
    party: MPCParty,
    pub(super) auxiliary_input: Vec<u8>,
}
// todo(zeev): rename to DwalletMPCSession.

impl DWalletMPCSession {
    pub(super) fn new(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Weak<AuthorityPerEpochStore>,
        epoch: EpochId,
        party: MPCParty,
        status: MPCSessionStatus,
        auxiliary_input: Vec<u8>,
        session_info: MPCSessionInfo,
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

    fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }

    /// Advances the MPC instance and optionally return a message the validator wants
    /// to send to the other MPC parties.
    /// Uses the existing party if it exists,
    /// otherwise creates a new one, as this is the first advance.
    pub(super) fn advance(&mut self) -> DwalletMPCResult<()> {
        // Take ownership of the MPCParty since advance() moves it because of the underline API.
        let party = mem::take(&mut self.party);

        let advance_result = party
            .advance(self.pending_messages.clone(), &self.auxiliary_input)
            .map_err(|e| {
                warn!("MPCParty advance() error: {:?}", e);
                // TODO (#263): Mark and punish the malicious validators
                // TODO (#263): that caused this advance to fail
                self.pending_messages.clear();
                return DwalletMPCError::MPCSessionError {
                    session_id: self.session_info.session_id,
                    error: format!("failed to advance the MPC party: {:?}", e),
                };
            })?;

        let msg = match advance_result {
            AdvanceResult::Advance((message, new_party)) => {
                self.status = MPCSessionStatus::Active;
                self.pending_messages.clear();
                self.party = new_party;
                self.new_dwallet_mpc_message(message).map_err(|e| {
                    DwalletMPCError::MPCSessionError {
                        session_id: self.session_info.session_id,
                        error: format!("failed to create a new MPC message on advance(): {:?}", e),
                    }
                })
            }
            AdvanceResult::Finalize(output) => {
                // TODO (#238): Verify the output and write it to the chain
                self.status = MPCSessionStatus::Finalizing(output.clone());
                Ok(self.new_dwallet_mpc_output_message(output, self.session_info.mpc_round))
            }
        }?;

        let consensus_adapter = Arc::clone(&self.consensus_adapter);
        let epoch_store = Arc::clone(&self.epoch_store()?);

        // Spawns sending this message asynchronously the
        // [`self.advance`] function will stay synchronous
        // and can be parallelized with `Rayon`.
        tokio::spawn(async move {
            if let Err(e) = consensus_adapter
                .submit_to_consensus(&vec![msg], &epoch_store)
                .await
            {
                warn!("Failed to submit an MPC message to consensus: {:?}", e);
            }
        });
        Ok(())
    }

    /// Create a new consensus transaction with the message to be sent to the other MPC parties.
    /// Returns `None` only if the epoch switched in the middle and was not available.
    fn new_dwallet_mpc_message(&self, message: Vec<u8>) -> DwalletMPCResult<ConsensusTransaction> {
        let epoch_store = self.epoch_store()?;
        Ok(ConsensusTransaction::new_dwallet_mpc_message(
            epoch_store.name,
            message,
            self.session_info.session_id.clone(),
        ))
    }

    /// Create a new consensus transaction with the flow result (output) to be sent
    /// to the other MPC parties.
    fn new_dwallet_mpc_output_message(
        &self,
        output: MPCOutput,
        mpc_round: MPCRound,
    ) -> ConsensusTransaction {
        // todo(zeev): create an issue to make sure that we don't send this message between epochs.
        ConsensusTransaction::new_dwallet_mpc_output(
            output,
            self.session_info.session_id,
            self.session_info.initiating_user_address,
            self.session_info.dwallet_cap_id,
            mpc_round,
        )
    }

    /// Stores a message in the pending messages map.
    /// The code stores every new message it receives for that session,
    /// and when we reach the end of delivery,
    /// we will advance the session if we have a threshold of messages.
    fn store_message(
        &mut self,
        message: &DWalletMPCMessage,
        epoch_store: Arc<AuthorityPerEpochStore>,
    ) -> DwalletMPCResult<()> {
        let party_id = authority_name_to_party_id(&message.authority, &epoch_store)?;
        if self.pending_messages.contains_key(&party_id) {
            // TODO(#260): Punish an authority that sends multiple messages in the same round
            return Ok(());
        }

        self.pending_messages
            .insert(party_id, message.message.clone());
        Ok(())
    }

    /// Handles a message by either forwarding it to the session
    /// or ignoring it if the session was finished.
    pub(super) fn handle_message(&mut self, message: DWalletMPCMessage) -> DwalletMPCResult<()> {
        if let MPCSessionStatus::Active = self.status {
            self.store_message(&message, self.epoch_store()?)
        } else {
            // TODO (#263): Check for malicious messages also after the instance is finished
            Ok(())
        }
    }
}

/// Convert a given authority name (address) to it's corresponding [`PartyID`].
/// The [`PartyID`] is the index of the authority in the committee.
pub fn authority_name_to_party_id(
    authority_name: &AuthorityName,
    epoch_store: &AuthorityPerEpochStore,
) -> DwalletMPCResult<PartyID> {
    epoch_store
        .committee()
        .authority_index(authority_name)
        .map(|index| index as PartyID)
        .ok_or_else(|| DwalletMPCError::AuthorityNameNotFound(*authority_name).into())
}
