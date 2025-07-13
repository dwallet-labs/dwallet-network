use crate::dwallet_mpc::mpc_session::MPCSessionLogger;
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{
    MPCMessage, MPCPrivateOutput, SerializedWrappedMPCPublicOutput,
};
use group::PartyID;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use itertools::Itertools;
use mpc::{AsynchronouslyAdvanceable, WeightedThresholdAccessStructure};
use rand_chacha::ChaCha20Rng;
use serde::de::DeserializeOwned;
use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet};
use tracing::error;

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
    mut messages_by_consensus_round: HashMap<u64, HashMap<u64, HashMap<PartyID, MPCMessage>>>,
    access_structure: &WeightedThresholdAccessStructure,
) -> Option<(Option<u64>, HashMap<u64, HashMap<PartyID, MPCMessage>>)> {
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
    let mut messages_for_advance: HashMap<u64, HashMap<PartyID, MPCMessage>> = HashMap::new();

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
    serialized_messages: HashMap<u64, HashMap<PartyID, MPCMessage>>,
    public_input: &P::PublicInput,
    private_input: P::PrivateInput,
    logger: &MPCSessionLogger,
    mut rng: ChaCha20Rng,
) -> DwalletMPCResult<
    mpc::AsynchronousRoundResult<MPCMessage, MPCPrivateOutput, SerializedWrappedMPCPublicOutput>,
> {
    let DeserializeMPCMessagesResponse {
        messages,
        malicious_parties,
    } = deserialize_mpc_messages_and_check_quorum(&serialized_messages, access_structure)?;

    // Update logger with malicious parties detected during deserialization.
    let logger = logger.clone().with_malicious_parties(malicious_parties);
    logger.write_logs_to_disk(session_id, party_id, access_structure, &serialized_messages);

    // When a `ThresholdNotReached` error is received, the system now waits for additional messages
    // (including those from previous rounds) and retries.
    let res = match P::advance_with_guaranteed_output(
        session_id,
        party_id,
        access_structure,
        messages.clone(),
        Some(private_input),
        public_input,
        &mut rng,
    ) {
        Ok(res) => res,
        Err(e) => {
            let general_error = DwalletMPCError::TwoPCMPCError(format!(
                "MPC error in party {party_id} session {} at round #{} {:?}",
                session_id,
                messages.len() + 1,
                e
            ));
            return match e.into() {
                // No threshold was reached, so we can't proceed.
                mpc::Error::ThresholdNotReached => {
                    return Err(DwalletMPCError::TWOPCMPCThresholdNotReached)
                }
                _ => Err(general_error),
            };
        }
    };

    Ok(match res {
        mpc::AsynchronousRoundResult::Advance {
            malicious_parties,
            message,
        } => mpc::AsynchronousRoundResult::Advance {
            malicious_parties,
            message: bcs::to_bytes(&message)?,
        },
        mpc::AsynchronousRoundResult::Finalize {
            malicious_parties,
            private_output,
            public_output,
        } => {
            let public_output: P::PublicOutputValue = public_output.into();
            let private_output = bcs::to_bytes(&private_output)?;

            mpc::AsynchronousRoundResult::Finalize {
                malicious_parties,
                private_output,
                public_output: bcs::to_bytes(&public_output)?,
            }
        }
    })
}

struct DeserializeMPCMessagesResponse<M: DeserializeOwned + Clone> {
    /// Round -> {party -> message}
    messages: HashMap<u64, HashMap<PartyID, M>>,
    malicious_parties: Vec<PartyID>,
}

/// Deserializes the messages received from other parties for the next advancement.
/// Any value that fails to deserialize is considered to be sent by a malicious party.
/// Returns the deserialized messages or an error including the IDs of the malicious parties.
///
/// Note that deserialization of a message depends on the type of the message,
/// which is only known once the event data comes,
/// and so we can only handle this here. Malicious messages are ignored, and we ensure
/// that we still have quorum, otherwise returning a `ThresholdNotReached` error.
fn deserialize_mpc_messages_and_check_quorum<M: DeserializeOwned + Clone>(
    messages: &HashMap<u64, HashMap<PartyID, MPCMessage>>,
    access_structure: &WeightedThresholdAccessStructure,
) -> DwalletMPCResult<DeserializeMPCMessagesResponse<M>> {
    let mut deserialized_results = HashMap::new();
    let mut malicious_parties = Vec::new();

    for (index, message_batch) in messages.iter() {
        let mut valid_messages = HashMap::new();

        for (party_id, message) in message_batch {
            match bcs::from_bytes::<M>(message) {
                Ok(value) => {
                    valid_messages.insert(*party_id, value);
                }
                Err(e) => {
                    tracing::error!(
                        party_id=?party_id,
                        error=?e,
                        "malicious party detected — failed to deserialize a message from party"
                    );
                    malicious_parties.push(*party_id);
                }
            }
        }

        let valid_message_senders = valid_messages.keys().copied().collect();
        if let Err(e) = access_structure.is_authorized_subset(&valid_message_senders) {
            error!(error=?e, "MPC threshold not reached");
            return Err(DwalletMPCError::TWOPCMPCThresholdNotReached);
        }

        if !valid_messages.is_empty() {
            deserialized_results.insert(*index, valid_messages);
        }
    }

    Ok(DeserializeMPCMessagesResponse {
        messages: deserialized_results,
        malicious_parties,
    })
}
