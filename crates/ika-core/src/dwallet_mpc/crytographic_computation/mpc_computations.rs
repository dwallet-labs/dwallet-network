use crate::dwallet_mpc::dwallet_dkg::{
    DWalletDKGFirstParty, DWalletDKGSecondParty, DWalletImportedKeyVerificationParty,
};
use crate::dwallet_mpc::encrypt_user_share::verify_encrypted_share;
use crate::dwallet_mpc::make_dwallet_user_secret_key_shares_public::verify_secret_share;
use crate::dwallet_mpc::mpc_session::PublicInput;
use crate::dwallet_mpc::mpc_session::{DWalletMPCSession, MPCSessionLogger};
use crate::dwallet_mpc::network_dkg::advance_network_dkg;
use crate::dwallet_mpc::presign::PresignParty;
use crate::dwallet_mpc::reconfiguration::ReconfigurationSecp256k1Party;
use crate::dwallet_mpc::sign::{verify_partial_signature, SignFirstParty};
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{
    MPCMessage, MPCPrivateOutput, MPCSessionPublicOutput, SerializedWrappedMPCPublicOutput,
    VersionedDWalletImportedKeyVerificationOutput, VersionedDecryptionKeyReconfigurationOutput,
    VersionedDwalletDKGFirstRoundPublicOutput, VersionedDwalletDKGSecondRoundPublicOutput,
    VersionedPresignOutput, VersionedSignOutput,
};
use dwallet_rng::RootSeed;
use group::PartyID;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{EncryptedShareVerificationRequestEvent, MPCRequestInput};
use itertools::Itertools;
use message_digest::message_digest::message_digest;
use mpc::{AsynchronousRoundResult, AsynchronouslyAdvanceable, WeightedThresholdAccessStructure};
use rand_chacha::ChaCha20Rng;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use tracing::{error, info, warn};

pub(crate) mod dwallet_dkg;
pub(crate) mod network_dkg;
pub(crate) mod presign;
pub(crate) mod reconfiguration;
pub(crate) mod sign;

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
    access_threshold: &WeightedThresholdAccessStructure,
    serialized_messages: HashMap<usize, HashMap<PartyID, MPCMessage>>,
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
    } = deserialize_mpc_messages(&serialized_messages);

    // Update logger with malicious parties detected during deserialization.
    let logger = logger.clone().with_malicious_parties(malicious_parties);

    logger.write_logs_to_disk(session_id, party_id, access_threshold, &serialized_messages);

    // When a `ThresholdNotReached` error is received, the system now waits for additional messages
    // (including those from previous rounds) and retries.
    let res = match P::advance_with_guaranteed_output(
        session_id,
        party_id,
        access_threshold,
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
    /// round -> {party -> message}
    messages: HashMap<usize, HashMap<PartyID, M>>,
    malicious_parties: Vec<PartyID>,
}

/// Deserializes the messages received from other parties for the next advancement.
/// Any value that fails to deserialize is considered to be sent by a malicious party.
/// Returns the deserialized messages or an error including the IDs of the malicious parties.
///
/// Note that deserialization of a message depends on the type of the message which is only known once the event data comes,
/// and so we can only handle this here. Malicious messages are ignored, and if this makes it so that a quorum is not reached,
/// the `advance_with_guaranteed_output()` method will return a `ThresholdNotReached` error, which will make us wait for more messages before advancing, as desired.
fn deserialize_mpc_messages<M: DeserializeOwned + Clone>(
    messages: &HashMap<usize, HashMap<PartyID, MPCMessage>>,
) -> DeserializeMPCMessagesResponse<M> {
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

        if !valid_messages.is_empty() {
            deserialized_results.insert(*index, valid_messages);
        }
    }
    DeserializeMPCMessagesResponse {
        messages: deserialized_results,
        malicious_parties,
    }
}
