// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::dwallet_mpc::mpc_session::{MPCRoundToMessagesHashMap, MPCSessionLogger};
use commitment::CommitmentSizedNumber;
use group::PartyID;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use itertools::Itertools;
use mpc::{
    AsynchronousRoundGODResult, AsynchronouslyAdvanceable, WeightedThresholdAccessStructure,
};
use rand_chacha::ChaCha20Rng;
use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet};

pub(crate) mod dwallet_dkg;
pub(crate) mod network_dkg;
pub(crate) mod presign;
pub(crate) mod reconfiguration;
pub(crate) mod sign;

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
    current_mpc_round: u64,
    rounds_to_delay: u64,
    mpc_round_to_threshold_not_reached_consensus_rounds: HashMap<u64, HashSet<u64>>,
    mut messages_by_consensus_round: HashMap<u64, MPCRoundToMessagesHashMap>,
    access_structure: &WeightedThresholdAccessStructure,
) -> Option<(Option<u64>, MPCRoundToMessagesHashMap)> {
    // The first round needs no messages as input, and is always ready to advance.
    if current_mpc_round == 1 {
        return Some((None, HashMap::new()));
    }

    let threshold_not_reached_consensus_rounds_for_current_mpc_round =
        mpc_round_to_threshold_not_reached_consensus_rounds
            .get(&current_mpc_round)
            .cloned()
            .unwrap_or_default();
    let mut delayed_rounds = 0;
    let mut got_new_messages_since_last_threshold_not_reached = false;
    let mut messages_for_advance: MPCRoundToMessagesHashMap = HashMap::new();

    // Make sure the messages are consecutive by inserting the default value (i.e. empty message map) for missing rounds.
    // This is just an extra step, as we should update sessions in every consensus round even if no new message were received.
    // It is important for computing delay.
    if let Some(&first_consensus_round) = messages_by_consensus_round.keys().min() {
        if let Some(&last_consensus_round) = messages_by_consensus_round.keys().max() {
            for consensus_round in first_consensus_round..=last_consensus_round {
                messages_by_consensus_round
                    .entry(consensus_round)
                    .or_default();
            }
        }
    }

    let sorted_messages_by_consensus_round = messages_by_consensus_round
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
            if mpc_round < current_mpc_round {
                for (sender_party_id, message) in mpc_round_messages {
                    let mpc_round_messages_map = messages_for_advance.entry(mpc_round).or_default();

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
            messages_for_advance.get(&(current_mpc_round - 1))
        {
            let previous_round_message_senders: HashSet<PartyID> =
                previous_round_messages.keys().cloned().collect();

            access_structure
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

mod tests {
    use super::*;

    #[test]
    fn test_build_messages_to_advance() {
        let messages_to_advance = build_messages_to_advance(
            // current MPC round
            2,
            // rounds to delay
            3,
            // mpc_round_to_threshold_not_reached_consensus_rounds
            HashMap::new(),
            // messages_by_consensus_round
            HashMap::from([(
                1,
                HashMap::from([(3, HashMap::from([(1 as PartyID, vec![1])]))]),
            )]),
            &WeightedThresholdAccessStructure::new(
                3,
                HashMap::from([
                    (1 as PartyID, 1),
                    (2 as PartyID, 1),
                    (3 as PartyID, 1),
                    (4 as PartyID, 1),
                ]),
            )
            .unwrap(),
        );
    }
}

/// Advances the state of an MPC party and serializes the result into bytes.
///
/// This helper function wraps around a party `P`'s `advance()` method,
/// converting its output into a serialized byte format.
/// This abstraction allows the system's generic components to operate uniformly on byte arrays,
/// rather than requiring generics to handle the different message and output types
/// for each MPC protocol.
///
/// By maintaining a structured transition between instantiated types, and their
/// serialized forms, this function ensures compatibility across various components.
pub(crate) fn advance_and_serialize<P: AsynchronouslyAdvanceable>(
    session_id: CommitmentSizedNumber,
    party_id: PartyID,
    access_structure: &WeightedThresholdAccessStructure,
    serialized_messages: MPCRoundToMessagesHashMap,
    public_input: &P::PublicInput,
    private_input: P::PrivateInput,
    logger: &MPCSessionLogger,
    mut rng: ChaCha20Rng,
) -> DwalletMPCResult<AsynchronousRoundGODResult> {
    // Update logger with malicious parties detected during deserialization.
    logger.write_logs_to_disk(session_id, party_id, access_structure, &serialized_messages);

    // When a `ThresholdNotReached` error is received, the system now waits for additional messages
    // (including those from previous rounds) and retries.
    match P::advance_with_guaranteed_output(
        session_id,
        party_id,
        access_structure,
        serialized_messages.clone(),
        Some(private_input),
        public_input,
        &mut rng,
    ) {
        Ok(res) => Ok(res),
        Err(e) => {
            let general_error = DwalletMPCError::TwoPCMPCError(format!(
                "MPC error in party {party_id} session {} at round #{} {:?}",
                session_id,
                serialized_messages.len() + 1,
                e
            ));
            match e.into() {
                // No threshold was reached, so we can't proceed.
                mpc::Error::ThresholdNotReached => {
                    Err(DwalletMPCError::TWOPCMPCThresholdNotReached)
                }
                _ => Err(general_error),
            }
        }
    }
}
