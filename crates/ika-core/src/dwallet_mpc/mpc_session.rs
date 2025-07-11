mod advance;
mod input;
mod logger;
mod mpc_event_data;

use class_groups::dkg::Secp256k1Party;
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{
    MPCMessage, MPCPrivateInput, MPCPrivateOutput, MPCSessionPublicOutput, MPCSessionStatus,
    SerializedWrappedMPCPublicOutput, VersionedDWalletImportedKeyVerificationOutput,
    VersionedDecryptionKeyReconfigurationOutput, VersionedDwalletDKGFirstRoundPublicOutput,
    VersionedDwalletDKGSecondRoundPublicOutput, VersionedPresignOutput, VersionedSignOutput,
};
use group::helpers::DeduplicateAndSort;
use group::PartyID;
use ika_types::crypto::AuthorityPublicKeyBytes;
use itertools::Itertools;
use mpc::{AsynchronousRoundResult, WeightedThresholdAccessStructure};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::runtime::Handle;
use tracing::{debug, error, info, warn};
use twopc_mpc::sign::Protocol;

use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::dwallet_dkg::{
    DWalletDKGFirstParty, DWalletDKGSecondParty, DWalletImportedKeyVerificationParty,
};
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::encrypt_user_share::verify_encrypted_share;
use crate::dwallet_mpc::make_dwallet_user_secret_key_shares_public::verify_secret_share;
use crate::dwallet_mpc::network_dkg::{advance_network_dkg, DwalletMPCNetworkKeys};
use crate::dwallet_mpc::presign::PresignParty;
use crate::dwallet_mpc::reconfiguration::ReconfigurationSecp256k1Party;
use crate::dwallet_mpc::sign::{verify_partial_signature, SignFirstParty};
use crate::dwallet_mpc::{message_digest, party_ids_to_authority_names};
use crate::stake_aggregator::StakeAggregator;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_consensus::ConsensusTransaction;
use ika_types::messages_dwallet_mpc::{
    AsyncProtocol, DWalletMPCEvent, DWalletMPCMessage, EncryptedShareVerificationRequestEvent,
    MPCRequestInput, MPCSessionRequest, MaliciousReport, SessionIdentifier, SessionType,
    ThresholdNotReachedReport, NETWORK_ENCRYPTION_KEY_DKG_STR_KEY,
    NETWORK_ENCRYPTION_KEY_RECONFIGURATION_STR_KEY,
};
use sui_types::base_types::{EpochId, ObjectID};

pub(crate) use advance::advance_and_serialize;
use dwallet_rng::RootSeed;
use ika_types::committee::{ClassGroupsEncryptionKeyAndProof, Committee};
use ika_types::messages_dwallet_mpc::MPCRequestInput::{NetworkEncryptionKeyDkg, NetworkEncryptionKeyReconfiguration};
pub(crate) use input::session_input_from_event;
pub(crate) use logger::MPCSessionLogger;
use crate::dwallet_mpc::mpc_session::mpc_event_data::MPCEventData;

/// A dWallet MPC session.
#[derive(Clone)]
pub(crate) struct DWalletMPCSession {
    pub(super) session_identifier: SessionIdentifier,
    epoch_id: EpochId,
    validator_name: AuthorityPublicKeyBytes,
    pub(crate) party_id: PartyID,

    /// The status of the MPC session.
    pub(super) status: MPCSessionStatus,

    committee: Arc<Committee>,
    access_structure: WeightedThresholdAccessStructure,

    /// The current MPC round number of the session.
    /// Starts at `1` and increments after each successful advance of the session.
    /// In round `1` We start the flow, without messages, from the event trigger.
    pub(super) current_mpc_round: usize,

    /// A map between an MPC round and the list of consensus rounds at which we tried to advance and failed.
    /// The total number of attempts to advance that failed in the session can be computed by summing the number of failed attempts.
    pub(crate) mpc_round_to_threshold_not_reached_consensus_rounds: HashMap<usize, Vec<u64>>,

    pub(crate) mpc_event_data: Option<MPCEventData>,

    /// All the messages that have been received for this session.
    /// We need to accumulate a threshold of those before advancing the session.
    /// TODO(Scaly): By consensus round, not mpc round
    /// HashMap{R1: Map{Validator1->Message, Validator2->Message}, R2: Map{Validator1->Message} ...}
    pub(super) messages_by_consensus_round: HashMap<u64, HashMap<usize, HashMap<PartyID, MPCMessage>>>,

    network_dkg_third_round_delay: usize,
    decryption_key_reconfiguration_third_round_delay: usize,

    dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
}

impl DWalletMPCSession {
    pub(crate) fn new(
        validator_name: AuthorityPublicKeyBytes,
        committee: Arc<Committee>,
        epoch: EpochId,
        status: MPCSessionStatus,
        session_identifier: SessionIdentifier,
        party_id: PartyID,
        access_structure: WeightedThresholdAccessStructure,
        mpc_event_data: Option<MPCEventData>,
        network_dkg_third_round_delay: usize,
        decryption_key_reconfiguration_third_round_delay: usize,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
        root_seed: RootSeed,
    ) -> Self {
        Self {
            status,
            messages_by_consensus_round: HashMap::new(),
            epoch_id: epoch,
            session_identifier,
            current_mpc_round: 1,
            mpc_round_to_threshold_not_reached_consensus_rounds: HashMap::new(),
            party_id,
            access_structure,
            mpc_event_data,
            network_dkg_third_round_delay,
            decryption_key_reconfiguration_third_round_delay,
            validator_name,
            committee,
            dwallet_mpc_metrics,
            root_seed,
        }
    }

    pub(crate) fn clear_data(&mut self) {
        self.mpc_event_data = None;
        self.messages_by_consensus_round = HashMap::new();
    }

    /// Create a new consensus transaction with the flow result (output) to be
    /// sent to the other MPC parties.
    /// Errors if the epoch was switched in the middle and was not available.
    fn new_dwallet_mpc_output_message(
        &self,
        output: MPCSessionPublicOutput,
    ) -> DwalletMPCResult<ConsensusTransaction> {
        let output = bcs::to_bytes(&output)?;
        let Some(mpc_event_data) = &self.mpc_event_data else {
            return Err(DwalletMPCError::MissingEventDrivenData);
        };
        Ok(ConsensusTransaction::new_dwallet_mpc_output(
            self.validator_name,
            output,
            MPCSessionRequest {
                session_type: mpc_event_data.session_type.clone(),
                session_identifier: self.session_identifier,
                session_sequence_number: mpc_event_data.session_sequence_number,
                request_input: mpc_event_data.request_input.clone(),
                epoch: self.epoch_id,
                requires_network_key_data: mpc_event_data.requires_network_key_data,
                requires_next_active_committee: mpc_event_data.requires_next_active_committee,
            },
        ))
    }

    /// Create a new consensus transaction with the message to be sent to the other MPC parties.
    /// Returns Error only if the epoch switched in the middle and was not available.
    fn new_dwallet_mpc_message(
        &self,
        message: MPCMessage,
    ) -> DwalletMPCResult<ConsensusTransaction> {
        // MPC event data can not be none, when sending a message.
        let Some(mpc_event_data) = &self.mpc_event_data else {
            return Err(DwalletMPCError::MissingEventDrivenData);
        };
        let session_request = MPCSessionRequest {
            session_type: mpc_event_data.session_type.clone(),
            request_input: mpc_event_data.request_input.clone(),
            epoch: self.epoch_id,
            session_identifier: self.session_identifier,
            session_sequence_number: mpc_event_data.session_sequence_number,
            requires_network_key_data: mpc_event_data.requires_network_key_data,
            requires_next_active_committee: mpc_event_data.requires_next_active_committee,
        };
        Ok(ConsensusTransaction::new_dwallet_mpc_message(
            self.validator_name,
            message,
            self.session_identifier,
            self.current_mpc_round,
            session_request,
        ))
    }

    /// Report that the session failed because of malicious actors.
    /// Once a quorum of validators reports the same actor, it is considered malicious.
    /// The session will be continued, and the malicious actors will be ignored.
    fn new_dwallet_report_failed_session_with_malicious_actors(
        &self,
        report: MaliciousReport,
    ) -> DwalletMPCResult<ConsensusTransaction> {
        Ok(
            ConsensusTransaction::new_dwallet_mpc_session_failed_with_malicious(
                self.validator_name,
                report,
            ),
        )
    }

    fn new_dwallet_report_threshold_not_reached(
        &self,
        report: ThresholdNotReachedReport,
    ) -> DwalletMPCResult<ConsensusTransaction> {
        Ok(
            ConsensusTransaction::new_dwallet_mpc_session_threshold_not_reached(
                self.validator_name,
                report,
            ),
        )
    }

    /// Stores an incoming message.
    /// This function performs no checks, it simply stores the message in the map.
    ///
    /// If a party sent a message twice, it will be overridden.
    /// Whilst that is malicious, it has no effect since the messages come in order, so all validators end up seeing the same map.
    /// Other malicious activities like sending a message for a wrong round are also not reported since they have no practical impact for similar reasons.
    pub(crate) fn store_message(
        &mut self,
        consensus_round: u64,
        sender_party_id: PartyID,
        message: DWalletMPCMessage,
    ) {
        // TODO: filter malicious, or in caller.
        debug!(
            session_id=?message.session_identifier,
            from_authority=?message.authority,
            receiving_authority=?self.validator_name,
            mpc_round=?message.round_number,
            message_size_bytes=?message.message.len(),
            "Received a dWallet MPC message",
        );
        
        if sender_party_id == self.party_id && self.current_mpc_round <= message.round_number {
            // Received a message from ourselves from the consensus, so it's safe to advance the round.
            let mpc_protocol = self.mpc_event_data.as_ref().map(|event_data| event_data.request_input.to_string()).unwrap_or_default();
            let new_mpc_round = message.round_number + 1;
            info!(
                session_id=?message.session_identifier,
                authority=?self.validator_name,
                message_mpc_round=?message.round_number,
                current_mpc_round=self.current_mpc_round,
                new_mpc_round,
                mpc_protocol,
                "Advancing current MPC round",
            );

            self.current_mpc_round = new_mpc_round;
        }

        let consensus_round_messages_map = self
            .messages_by_consensus_round
            .entry(consensus_round)
            .or_default();

        let mpc_round_messages_map = consensus_round_messages_map
            .entry(message.round_number)
            .or_default();

        // TODO: only insert if doesn't exist - update comment
        if !mpc_round_messages_map.contains_key(&sender_party_id) {
            mpc_round_messages_map.insert(sender_party_id, message.message);
        }
    }

    /// Returns the number of additional (delay) consensus rounds the session should wait for before advancing.
    ///
    /// This method returns the protocol-specific delay for certain MPC rounds in specific protocols
    /// (NetworkDkg, DecryptionKeyReconfiguration).
    ///
    /// - **NetworkDkg protocol**: requires delay for the third round
    ///   using `network_dkg_third_round_delay` config.
    /// - **DecryptionKeyReconfiguration protocol**: requires delay for the third round
    ///   using `decryption_key_reconfiguration_third_round_delay` config.
    /// - **Other protocols**: No delay required, always ready to advance
    ///
    fn consensus_rounds_delay_for_mpc_round(&self) -> usize {
        match self.mpc_event_data.as_ref().unwrap().request_input {
            MPCRequestInput::NetworkEncryptionKeyDkg(_, _) if self.current_mpc_round == 3 => self.network_dkg_third_round_delay,
            MPCRequestInput::NetworkEncryptionKeyReconfiguration(_) if self.current_mpc_round == 3 => self.decryption_key_reconfiguration_third_round_delay,
            _ => 0,
        }
    }

    // TODO(Scaly): should I log?
    // info!(
    //     ?self.consensus_rounds_since_quorum_reached,
    //     ?self.current_mpc_round,
    //     ?self.agreed_mpc_protocol,
    //     ?self.session_identifier,
    //     messages_count_for_current_round=?self.messages_by_consensus_round.get(&(self.current_mpc_round - 1)).unwrap_or(&HashMap::new()).len(),
    //     "Quorum reached for MPC session and delay passed, advancing to next round",
    // );
    //
    // info!(
    //     ?self.consensus_rounds_since_quorum_reached,
    //     ?self.current_mpc_round,
    //     ?self.agreed_mpc_protocol,
    //     messages_count_for_current_round=?self.messages_by_consensus_round.get(&(self.current_mpc_round - 1)).unwrap_or(&HashMap::new()).len(),
    //     "Quorum reached for MPC session but delay not passed yet, waiting for another round",
    // );

    // TODO(Scaly): should I store it ?
    // TODO(Scaly): unit test
    pub(crate) fn get_messages_to_advance(&self) -> Option<HashMap<usize, HashMap<PartyID, MPCMessage>>> {
        if self.mpc_event_data.is_none() {
            // Cannot advance a session before the MPC event requesting it was received.
            return None;
        }
        if self.current_mpc_round == 1 {
            // The first round needs no messages as input, and is always ready to advance.
            return Some(HashMap::new());
        }

        let threshold_not_reached_consensus_rounds = self.mpc_round_to_threshold_not_reached_consensus_rounds.get(&self.current_mpc_round).unwrap_or_default();
        let rounds_to_delay = self.consensus_rounds_delay_for_mpc_round();
        let mut delayed_rounds = 0;
        let mut got_new_messages_since_last_threshold_not_reached = false;
        let mut messages_for_advance = HashMap::new();
        let sorted_messages_by_consensus_round = self.messages_by_consensus_round.clone().into_iter().sorted_by(|(first_consensus_round, _), (second_consensus_round, _)| first_consensus_round.cmp(second_consensus_round));
        for (consensus_round, consensus_round_messages) in sorted_messages_by_consensus_round {
            // Update messages to advance the current round by joining the messages received at the current consensus round
            // with the ones we collected so far, ignoring duplicates.
            for (mpc_round, mpc_round_messages) in consensus_round_messages {
                if mpc_round < self.current_mpc_round {
                    for (sender_party_id, message) in mpc_round_messages {
                        let mpc_round_messages_map = messages_for_advance
                            .entry(mpc_round)
                            .or_default();

                        if !mpc_round_messages_map.contains_key(&sender_party_id) {
                            // Always take the first message sent in consensus by a particular party for a particular round.
                            mpc_round_messages_map.insert(sender_party_id, message);

                            got_new_messages_since_last_threshold_not_reached = true;
                        }
                    }
                }
            }

            // Check if we have the threshold of messages for the previous round
            // to advance to the next round.
            let is_quorum_reached = if let Some(previous_round_messages) =
                messages_for_advance.get(&(self.current_mpc_round - 1))
            {
                let previous_round_message_senders: HashSet<PartyID> =
                    previous_round_messages.keys().cloned().collect();

                self.access_structure
                    .is_authorized_subset(&previous_round_message_senders)
                    .is_ok()
            } else {
                false
            };

            if is_quorum_reached {
                if delayed_rounds != rounds_to_delay {
                    // Wait for the delay.
                    // We set the map of messages by consensus round at each consensus round for each session,
                    // even if no messages were received, so this count is accurate as iterating the messages by consensus round goes through all consensus rounds to date.
                    delayed_rounds += 1;
                } else if threshold_not_reached_consensus_rounds.contains(consensus_round) {
                    // We already tried executing this MPC round at the current consensus round, no point in trying again.
                    // Wait for new messages in later rounds before retrying.
                    got_new_messages_since_last_threshold_not_reached = false;
                } else if got_new_messages_since_last_threshold_not_reached {
                    // We have a quorum of previous round messages,
                    // we delayed the execution as and if required,
                    // and we know we haven't tried to advance the current MPC round with this set of messages so we have a chance at advancing (and reaching threshold):
                    // Let's try advancing with this set of messages!
                    return Some(messages_for_advance);
                }
            }
        }

        // If we reached here, we either got no quorum of previous round messages,
        // or we need to delay execution further,
        // or we need to wait for more messages before retrying after a threshold not reached has occurred.
        // This session is not ready to advance.
        None
    }

    // Gets the current *total* attempt number, meaning the number of threshold not reached from any mpc round in this session, plus 1.
    pub(crate) fn get_attempt_number(&self) -> usize {
        let threshold_not_reached_count = self.mpc_round_to_threshold_not_reached_consensus_rounds.values().flatten().len();

        threshold_not_reached_count + 1
    }

    fn update_expected_decrypters_metrics(
        &self,
        expected_decrypters: &HashSet<PartyID>,
    ) -> DwalletMPCResult<()> {
        if self.current_mpc_round != 2 {
            return Ok(());
        }
        let participating_expected_decrypters: HashSet<PartyID> = expected_decrypters
            .iter()
            .filter(|party_id| {
                self.messages_by_consensus_round
                    .get(&(self.current_mpc_round - 1))
                    .is_some_and(|messages| messages.contains_key(*party_id))
            })
            .copied()
            .collect();
        if self
            .access_structure
            .is_authorized_subset(&participating_expected_decrypters)
            .is_ok()
        {
            self.dwallet_mpc_metrics
                .number_of_expected_sign_sessions
                .inc();
        } else {
            self.dwallet_mpc_metrics
                .number_of_unexpected_sign_sessions
                .inc();
        }
        Ok(())
    }
}
