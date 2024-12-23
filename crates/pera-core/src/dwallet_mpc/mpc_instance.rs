use std::collections::HashMap;
use std::sync::{Arc, Weak};

use group::PartyID;
use mpc::{AsynchronousRoundResult, WeightedThresholdAccessStructure};
use twopc_mpc::secp256k1::class_groups::DecryptionKeyShare;

use dwallet_mpc_types::dwallet_mpc::MPCSessionStatus;

use pera_types::base_types::EpochId;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::messages_consensus::ConsensusTransaction;
use pera_types::messages_dwallet_mpc::SessionInfo;

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::mpc_party::MPCParty;
use crate::dwallet_mpc::{authority_name_to_party_id, DWalletMPCMessage};

/// A DWallet MPC session instance
/// It keeps track of the status of the session, the channel to send messages to the instance,
/// and the messages that are pending to be sent to the instance.
pub(super) struct DWalletMPCInstance {
    /// The status of the MPC instance
    pub(super) status: MPCSessionStatus,
    /// The messages that are pending to be executed while advancing the instance
    /// We need to accumulate threshold of those before advancing the instance
    pub(super) pending_messages: Vec<HashMap<PartyID, Vec<u8>>>,
    epoch_store: Weak<AuthorityPerEpochStore>,
    epoch_id: EpochId,
    /// The total number of parties in the chain
    /// We can calculate the threshold and parties IDs (indexes) from it
    /// To calculate the parties IDs all we need to know is the number of parties, as the IDs are just the indexes of those parties. If there are 3 parties, the IDs are [0, 1, 2].
    pub(super) session_info: SessionInfo,
    /// The MPC party that being used to run the MPC cryptographic steps. An option because it can be None before the instance has started.
    pub party: MPCParty,
    pub(super) public_input: Vec<u8>,
    /// The decryption share of the party for mpc sign sessions
    decryption_share: Option<DecryptionKeyShare>,
}

impl DWalletMPCInstance {
    pub(crate) fn new(
        epoch_store: Weak<AuthorityPerEpochStore>,
        epoch: EpochId,
        party: MPCParty,
        status: MPCSessionStatus,
        auxiliary_input: Vec<u8>,
        session_info: SessionInfo,
        decryption_share: Option<DecryptionKeyShare>,
    ) -> Self {
        Self {
            status,
            pending_messages: Vec::new(),
            epoch_store: epoch_store.clone(),
            epoch_id: epoch,
            party,
            public_input: auxiliary_input,
            session_info,
            decryption_share,
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
    pub(super) fn advance(
        &mut self,
        weighted_threshold_access: &WeightedThresholdAccessStructure,
        party_id: PartyID,
    ) -> DwalletMPCResult<(ConsensusTransaction, Vec<PartyID>)> {
        let pending_messages = self.pending_messages.clone();
        let (status, round) = match self.status {
            MPCSessionStatus::Pending | MPCSessionStatus::FirstExecution => {
                (MPCSessionStatus::Active(0), 0)
            }
            MPCSessionStatus::Active(round) => (MPCSessionStatus::Active(round + 1), round + 1),
            _ => {
                return Err(DwalletMPCError::MPCSessionError {
                    session_id: self.session_info.session_id,
                    error: format!(
                        "failed to advance the MPC session, unexpected status: {}",
                        self.status
                    ),
                })
            }
        };
        self.status = status;
        let advance_result = self.party.advance(
            pending_messages,
            self.session_info.flow_session_id,
            party_id,
            weighted_threshold_access,
            self.public_input.clone(),
        );

        match advance_result {
            Ok(AsynchronousRoundResult::Advance {
                malicious_parties,
                message,
            }) => {
                self.pending_messages.insert(round, HashMap::new());
                Ok((
                    self.new_dwallet_mpc_message(message).map_err(|e| {
                        DwalletMPCError::MPCSessionError {
                            session_id: self.session_info.session_id,
                            error: format!(
                                "failed to create a new MPC message on advance(): {:?}",
                                e
                            ),
                        }
                    })?,
                    malicious_parties,
                ))
            }
            Ok(AsynchronousRoundResult::Finalize {
                malicious_parties,
                private_output,
                public_output,
            }) => {
                self.status = MPCSessionStatus::Finished(public_output.clone(), private_output);
                Ok((
                    self.new_dwallet_mpc_output_message(public_output)?,
                    malicious_parties,
                ))
            }
            Err(DwalletMPCError::MaliciousParties(malicious_parties)) => {
                self.restart();
                Err(DwalletMPCError::MaliciousParties(malicious_parties))
            }
            Err(e) => {
                self.status = MPCSessionStatus::Failed;
                Err(e)
            }
        }
    }

    fn restart(&mut self) {
        self.status = MPCSessionStatus::FirstExecution;
    }

    /// Create a new consensus transaction with the message to be sent to the other MPC parties.
    /// Returns None only if the epoch switched in the middle and was not available.
    fn new_dwallet_mpc_message(&self, message: Vec<u8>) -> DwalletMPCResult<ConsensusTransaction> {
        Ok(ConsensusTransaction::new_dwallet_mpc_message(
            self.epoch_store()?.name,
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
    ) -> DwalletMPCResult<ConsensusTransaction> {
        Ok(ConsensusTransaction::new_dwallet_mpc_output(
            self.epoch_store()?.name,
            output,
            self.session_info.clone(),
        ))
    }

    /// Stores a message in the pending messages map.
    /// The code stores every new message it receives for that session,
    /// and when we reach the end of delivery,
    /// we will advance the session if we have a threshold of messages.
    fn store_message(
        &mut self,
        round: usize,
        message: &DWalletMPCMessage,
        epoch_store: Arc<AuthorityPerEpochStore>,
    ) -> DwalletMPCResult<()> {
        let party_id = authority_name_to_party_id(&message.authority, &epoch_store)?;
        if self.pending_messages[round].contains_key(&party_id) {
            return Err(DwalletMPCError::MaliciousParties(vec![party_id]));
        }
        self.pending_messages[round].insert(party_id, message.message.clone());
        Ok(())
    }

    /// Handles a message by either forwarding it to the session
    /// or ignoring it if the session is not active.
    pub(crate) fn handle_message(&mut self, message: &DWalletMPCMessage) -> DwalletMPCResult<()> {
        if let MPCSessionStatus::Active(round) = self.status {
            self.store_message(round, message, self.epoch_store()?)
        } else {
            // Do nothing.
            Ok(())
        }
    }

    pub(crate) fn party(&self) -> &MPCParty {
        &self.party
    }
}

/// Needed to be able to iterate over a vector of generic MPCInstances with Rayon
unsafe impl Send for DWalletMPCInstance {}
