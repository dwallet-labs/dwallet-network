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

// // TODO: move this, should be done by the MPC service
// impl DWalletMPCSession {
//     /// Advances the MPC session and sends the advancement result to the other validators.
//     /// The consensus submission logic is being spawned as a separate tokio task, as it's an IO
//     /// heavy task.
//     /// Rayon, which is good for CPU heavy tasks, is used to perform the cryptographic
//     /// computation, and Tokio, which is good for IO heavy tasks, is used to submit the result to
//     /// the consensus.
//     pub(super) fn advance(&mut self, tokio_runtime_handle: &Handle) -> DwalletMPCResult<()> {
//         // TODO: no handle I think.
//
//         // Make sure we transfer only the messages up to the current round
//         // (exclude messages that might be received from future rounds)
//         self.messages_by_consensus_round
//             .retain(|round, _| round < &self.current_mpc_round);
//         // Safe to unwrap as advance can only be called after the event is received.
//         let mpc_protocol = self.mpc_event_data.clone().unwrap().request_input;
//         match self.advance_specific_party() {
//             Ok(AsynchronousRoundResult::Advance {
//                    malicious_parties,
//                    message,
//                }) => {
//                 let session_id = self.session_identifier;
//                 let round_number = self.messages_by_consensus_round.len();
//                 info!(
//                     mpc_protocol=?mpc_protocol,
//                     session_id=?session_id,
//                     validator=?self.validator_name,
//                     round=?round_number,
//                     "Advanced MPC session"
//                 );
//                 let consensus_adapter = self.consensus_adapter.clone();
//                 if !malicious_parties.is_empty() {
//                     self.report_malicious_actors(malicious_parties, &self.committee)?;
//                 }
//                 let message = self.new_dwallet_mpc_message(message)?;
//
//                 // TODO: not from here! send in channel
//                 // tokio_runtime_handle.spawn(async move {
//                 //     if let Err(err) = consensus_adapter
//                 //         .submit_to_consensus(&[message], &epoch_store)
//                 //         .await
//                 //     {
//                 //         error!(
//                 //             mpc_protocol=?mpc_protocol,
//                 //             session_id=?session_id,
//                 //             validator=?validator_name,
//                 //             round=?round_number,
//                 //             err=?err,
//                 //             "failed to submit an MPC message to consensus"
//                 //         );
//                 //     }
//                 // });
//
//                 Ok(())
//             }
//             Ok(AsynchronousRoundResult::Finalize {
//                    malicious_parties,
//                    private_output: _,
//                    public_output,
//                }) => {
//                 info!(
//                     mpc_protocol=?&mpc_protocol,
//                     session_identifier=?self.session_identifier,
//                     validator=?&self.validator_name,
//                     "Reached public output (Finalize) for session"
//                 );
//                 let consensus_adapter = self.consensus_adapter.clone();
//                 if !malicious_parties.is_empty() {
//                     warn!(
//                         mpc_protocol=?&mpc_protocol,
//                         session_identifier=?self.session_identifier,
//                         validator=?&self.validator_name,
//                         ?malicious_parties,
//                         "Malicious Parties detected on MPC session Finalize",
//                     );
//                     self.report_malicious_actors(malicious_parties, &self.committee)?;
//                 }
//                 let consensus_message = self.new_dwallet_mpc_output_message(
//                     MPCSessionPublicOutput::CompletedSuccessfully(public_output.clone()),
//                 )?;
//                 let session_id_clone = self.session_identifier;
//
//                 // TODO: not from here!
//                 // tokio_runtime_handle.spawn(async move {
//                 //     if let Err(err) = consensus_adapter
//                 //         .submit_to_consensus(&[consensus_message], &epoch_store)
//                 //         .await
//                 //     {
//                 //         error!(
//                 //             mpc_protocol=?mpc_protocol,
//                 //             session_id=?session_id_clone,
//                 //             validator=?validator_name,
//                 //             err=?err,
//                 //             "failed to submit an MPC output message to consensus",
//                 //         );
//                 //     }
//                 // });
//
//                 Ok(())
//             }
//             Err(DwalletMPCError::TWOPCMPCThresholdNotReached) => {
//                 error!(
//                     err=?DwalletMPCError::TWOPCMPCThresholdNotReached,
//                     session_identifier=?self.session_identifier,
//                     validator=?self.validator_name,
//                     crypto_round=?self.current_mpc_round,
//                     party_id=?self.party_id,
//                     mpc_protocol=?&mpc_protocol,
//                     "MPC session failed"
//                 );
//                 // self.report_threshold_not_reached(tokio_runtime_handle)
//             }
//             Err(err) => {
//                 error!(
//                     session_identifier=?self.session_identifier,
//                     validator=?self.validator_name,
//                     crypto_round=?self.current_mpc_round,
//                     party_id=?self.party_id,
//                     error=?err,
//                     mpc_protocol=?mpc_protocol,
//                     epoch=?self.epoch_id,
//                     "failed to advance the MPC session"
//                 );
//
//                 let consensus_adapter = self.consensus_adapter.clone();
//                 let consensus_message =
//                     self.new_dwallet_mpc_output_message(MPCSessionPublicOutput::SessionFailed)?;
//                 let session_id_clone = self.session_identifier;
//                 let epoch_id_clone = self.epoch_id;
//
//                 // TODO(Scaly): what is this code
//                 // tokio_runtime_handle.spawn(async move {
//                 //     if let Err(err) = consensus_adapter
//                 //         .submit_to_consensus(&[consensus_message], &epoch_store)
//                 //         .await
//                 //     {
//                 //         error!(
//                 //             mpc_protocol=?&mpc_protocol,
//                 //             session_id=?session_id_clone,
//                 //             validator=?validator_name,
//                 //             epoch=?epoch_id_clone,
//                 //             error=?err,
//                 //             "failed to submit an MPC SessionFailed message to consensus");
//                 //     }
//                 // });
//
//                 Err(err)
//             }
//         }
//     }
// }

struct DeserializeMPCMessagesResponse<M: DeserializeOwned + Clone> {
    /// round -> {party -> message}
    messages: HashMap<usize, HashMap<PartyID, M>>,
    #[allow(dead_code)]
    malicious_parties: Vec<PartyID>,
}

/// Deserializes the messages received from other parties for the next advancement.
/// Any value that fails to deserialize is considered to be sent by a malicious party.
/// Returns the deserialized messages or an error including the IDs of the malicious parties.
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
