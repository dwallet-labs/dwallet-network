mod input;
mod logger;
mod mpc_event_data;

use dwallet_mpc_types::dwallet_mpc::{MPCMessage, MPCSessionStatus};
use group::PartyID;
use ika_types::crypto::AuthorityPublicKeyBytes;
use itertools::Itertools;
use mpc::WeightedThresholdAccessStructure;
use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet};
use tracing::{debug, error, info};

use ika_types::messages_dwallet_mpc::{DWalletMPCMessage, MPCRequestInput, SessionIdentifier};

pub(crate) use crate::dwallet_mpc::mpc_session::mpc_event_data::MPCEventData;
pub(crate) use input::{session_input_from_event, PublicInput};
pub(crate) use logger::MPCSessionLogger;

/// A dWallet MPC session.
#[derive(Clone)]
pub(crate) struct DWalletMPCSession {
    pub(super) session_identifier: SessionIdentifier,
    validator_name: AuthorityPublicKeyBytes,
    pub(crate) party_id: PartyID,

    /// The status of the MPC session.
    pub(super) status: MPCSessionStatus,

    access_structure: WeightedThresholdAccessStructure,

    /// The current MPC round number of the session.
    /// Starts at `1` and increments after each successful advance of the session.
    /// In round `1` We start the flow, without messages, from the event trigger.
    pub(super) current_mpc_round: usize,

    /// A map between an MPC round, and the list of consensus rounds at which we tried to
    /// advance and failed.
    /// The total number of attempts to advance that failed in the session can be
    /// computed by summing the number of failed attempts.
    pub(crate) mpc_round_to_threshold_not_reached_consensus_rounds: HashMap<usize, HashSet<u64>>,

    pub(crate) mpc_event_data: Option<MPCEventData>,

    /// All the messages that have been received for this session from each party, by consensus round and then by MPC round.
    /// Used to build the input of messages to advance each round of the session.
    pub(super) messages_by_consensus_round:
        HashMap<u64, HashMap<usize, HashMap<PartyID, MPCMessage>>>,

    network_dkg_third_round_delay: usize,
    decryption_key_reconfiguration_third_round_delay: usize,
}

impl DWalletMPCSession {
    pub(crate) fn new(
        validator_name: AuthorityPublicKeyBytes,
        status: MPCSessionStatus,
        session_identifier: SessionIdentifier,
        party_id: PartyID,
        access_structure: WeightedThresholdAccessStructure,
        mpc_event_data: Option<MPCEventData>,
        network_dkg_third_round_delay: usize,
        decryption_key_reconfiguration_third_round_delay: usize,
    ) -> Self {
        Self {
            status,
            messages_by_consensus_round: HashMap::new(),
            session_identifier,
            current_mpc_round: 1,
            mpc_round_to_threshold_not_reached_consensus_rounds: HashMap::new(),
            party_id,
            access_structure,
            mpc_event_data,
            network_dkg_third_round_delay,
            decryption_key_reconfiguration_third_round_delay,
            validator_name,
        }
    }

    pub(crate) fn clear_data(&mut self) {
        self.mpc_event_data = None;
        self.messages_by_consensus_round = HashMap::new();
    }

    /// Stores an incoming message, and increases the `current_mpc_round` upon seeing a message
    /// sent from us for the current round.
    /// This guarantees we are in sync, as our state mutates in sync with the view of the
    /// consensus, which is shared with the other validators.
    ///
    /// This function performs no checks, it simply stores the message in the map.
    ///
    /// If a party sent a message twice, the second message will be ignored.
    /// Whilst that is malicious, it has no effect since the messages come in order,
    /// so all validators end up seeing the same map.
    /// Other malicious activities like sending a message for a wrong round are also not
    /// reported since they have no practical impact for similar reasons.
    pub(crate) fn store_message(
        &mut self,
        consensus_round: u64,
        sender_party_id: PartyID,
        message: DWalletMPCMessage,
    ) {
        debug!(
            session_identifier=?message.session_identifier,
            from_authority=?message.authority,
            receiving_authority=?self.validator_name,
            mpc_round=?message.round_number,
            message_size_bytes=?message.message.len(),
            "Received a dWallet MPC message",
        );

        if sender_party_id == self.party_id && self.current_mpc_round <= message.round_number {
            // Received a message from ourselves from the consensus, so it's safe to advance the round.
            let mpc_protocol = self
                .mpc_event_data
                .as_ref()
                .map(|event_data| event_data.request_input.to_string())
                .unwrap_or_default();
            let new_mpc_round = message.round_number + 1;
            info!(
                session_identifier=?message.session_identifier,
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

        if let Vacant(e) = mpc_round_messages_map.entry(sender_party_id) {
            e.insert(message.message);
        }
    }

    /// This function iterates over the messages from different parties sent for
    /// different MPC rounds, ordered by the consensus round they were received.
    ///
    /// It builds a list of messages to advance the current round `current_mpc_round`,
    /// using all the messages from the first consensus round to the first that satisfies the
    /// following conditions:
    /// - a quorum of messages from the previous round `current_mpc_round — 1` must exist
    ///   (except for the first round, which is always ready to advance requiring no messages as input.)
    /// - a minimum number of consensus rounds that was required to delay the execution
    ///   (to allow more messages to come in before advancing)
    ///   has passed since the first consensus round where we got a quorum for this round.
    /// - This quorum must be "fresh", in the sense we never tried to advance with it before.
    ///   There is only one case in which we attempt to advance the same round twice:
    ///   when we get a threshold not reached error.
    ///   Therefore, if such an error occurred for a consensus round, we don't stop the search,
    ///   and wait for at least one new message to come in a later consensus round before returning
    ///   the messages to advance with.
    ///
    /// Duplicate messages are ignored — the first message a party has sent for an MPC round
    /// is always used.
    pub(crate) fn build_messages_to_advance(
        &self,
    ) -> Option<(Option<u64>, HashMap<usize, HashMap<PartyID, MPCMessage>>)> {
        // The first round needs no messages as input, and is always ready to advance.
        if self.current_mpc_round == 1 {
            return Some((None, HashMap::new()));
        }

        let threshold_not_reached_consensus_rounds_for_current_mpc_round = self
            .mpc_round_to_threshold_not_reached_consensus_rounds
            .get(&self.current_mpc_round)
            .cloned()
            .unwrap_or_default();
        let rounds_to_delay = self.consensus_rounds_delay_for_mpc_round();
        let mut delayed_rounds = 0;
        let mut got_new_messages_since_last_threshold_not_reached = false;
        let mut messages_for_advance: HashMap<usize, HashMap<PartyID, MPCMessage>> = HashMap::new();

        let sorted_messages_by_consensus_round = self
            .messages_by_consensus_round
            .clone()
            .into_iter()
            .sorted_by(|(first_consensus_round, _), (second_consensus_round, _)| {
                first_consensus_round.cmp(second_consensus_round)
            });
        for (consensus_round, consensus_round_messages) in sorted_messages_by_consensus_round {
            // Update messages to advance the current round by joining the messages
            // received at the current consensus round
            // with the ones we collected so far, ignoring duplicates.
            for (mpc_round, mpc_round_messages) in consensus_round_messages {
                if mpc_round < self.current_mpc_round {
                    for (sender_party_id, message) in mpc_round_messages {
                        let mpc_round_messages_map =
                            messages_for_advance.entry(mpc_round).or_default();

                        if let Vacant(e) = mpc_round_messages_map.entry(sender_party_id) {
                            // Always take the first message sent in consensus by a
                            // particular party for a particular round.
                            e.insert(message);
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
                    // We set the map of messages by consensus round at each consensus round for
                    // each session, even if no messages were received, so this count is
                    // accurate as iterating the messages by consensus round goes through all
                    // consensus rounds to date.
                    delayed_rounds += 1;
                } else if threshold_not_reached_consensus_rounds_for_current_mpc_round
                    .contains(&consensus_round)
                {
                    // We already tried executing this MPC round at the current consensus round, no point in trying again.
                    // Wait for new messages in later rounds before retrying.
                    got_new_messages_since_last_threshold_not_reached = false;
                } else if got_new_messages_since_last_threshold_not_reached {
                    // We have a quorum of previous round messages,
                    // we delayed the execution as and if required,
                    // and we know we haven't tried to advance the current MPC round with this
                    // set of messages, so we have a chance at advancing (and reaching threshold):
                    // Let's try advancing with this set of messages!
                    return Some((Some(consensus_round), messages_for_advance));
                }
            }
        }

        // If we reached here, we either got no quorum of previous round messages,
        // or we need to delay execution further,
        // or we need to wait for more messages before retrying after a threshold not reached has occurred.
        // This session is not ready to advance.
        None
    }

    /// Computes the current *total* attempt number, meaning the number of thresholds
    /// not reached from any mpc round in this session, plus 1.
    pub(crate) fn get_attempt_number(&self) -> usize {
        let threshold_not_reached_consensus_rounds = self
            .mpc_round_to_threshold_not_reached_consensus_rounds
            .values()
            .flatten()
            .collect::<Vec<_>>();
        let threshold_not_reached_count = threshold_not_reached_consensus_rounds.len();

        threshold_not_reached_count + 1
    }

    /// Records a threshold not reached error that we got when advancing
    /// this session with messages up to `consensus_round`.
    pub(crate) fn record_threshold_not_reached(&mut self, consensus_round: u64) {
        let request_input = &self.mpc_event_data.as_ref().unwrap().request_input;

        error!(
            mpc_protocol=?request_input,
            validator=?self.validator_name,
            session_identifier=?self.session_identifier,
            mpc_round=?self.current_mpc_round,
            "threshold was not reached for session"
        );

        self.mpc_round_to_threshold_not_reached_consensus_rounds
            .entry(self.current_mpc_round)
            .or_default()
            .insert(consensus_round);
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
    fn consensus_rounds_delay_for_mpc_round(&self) -> usize {
        match self.mpc_event_data.as_ref().unwrap().request_input {
            MPCRequestInput::NetworkEncryptionKeyDkg(_, _) if self.current_mpc_round == 3 => {
                self.network_dkg_third_round_delay
            }
            MPCRequestInput::NetworkEncryptionKeyReconfiguration(_)
                if self.current_mpc_round == 3 =>
            {
                self.decryption_key_reconfiguration_third_round_delay
            }
            _ => 0,
        }
    }
}
