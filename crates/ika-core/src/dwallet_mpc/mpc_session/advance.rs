use crate::dwallet_mpc::mpc_session::{DWalletMPCSession, MPCSessionLogger};
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{MPCMessage, MPCPrivateOutput, MPCSessionPublicOutput, SerializedWrappedMPCPublicOutput, VersionedDecryptionKeyReconfigurationOutput, VersionedDwalletDKGFirstRoundPublicOutput, VersionedDwalletDKGSecondRoundPublicOutput, VersionedDWalletImportedKeyVerificationOutput, VersionedPresignOutput, VersionedSignOutput};
use group::PartyID;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use mpc::{AsynchronouslyAdvanceable, AsynchronousRoundResult, WeightedThresholdAccessStructure};
use rand_chacha::ChaCha20Rng;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use itertools::Itertools;
use tracing::{error, info, warn};
use dwallet_rng::RootSeed;
use ika_types::messages_dwallet_mpc::{EncryptedShareVerificationRequestEvent, MPCRequestInput};
use message_digest::message_digest::message_digest;
use crate::dwallet_mpc::cryptographic_computations_orchestrator::{ComputationId, ComputationRequest};
use crate::dwallet_mpc::dwallet_dkg::{DWalletDKGFirstParty, DWalletDKGSecondParty, DWalletImportedKeyVerificationParty};
use crate::dwallet_mpc::encrypt_user_share::verify_encrypted_share;
use crate::dwallet_mpc::make_dwallet_user_secret_key_shares_public::verify_secret_share;
use crate::dwallet_mpc::mpc_session::input::PublicInput;
use crate::dwallet_mpc::network_dkg::advance_network_dkg;
use crate::dwallet_mpc::presign::PresignParty;
use crate::dwallet_mpc::reconfiguration::ReconfigurationSecp256k1Party;
use crate::dwallet_mpc::sign::{SignFirstParty, verify_partial_signature};

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

impl ComputationRequest {
    pub(crate) fn compute(self, computation_id: ComputationId, root_seed: RootSeed,) -> DwalletMPCResult<
        AsynchronousRoundResult<MPCMessage, MPCPrivateOutput, SerializedWrappedMPCPublicOutput>,
    > {
        let messages_skeleton = self
            .messages
            .iter()
            .map(|(round, messages_map)| {
                (
                    *round,
                    messages_map.keys().copied().sorted().collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<_, _>>();

        info!(
            mpc_protocol=?self.input,
            validator=?self.validator_name,
            session_identifier=?computation_id.session_identifier,
            crypto_round=?computation_id.mpc_round,
            access_structure=?self.access_structure,
            ?messages_skeleton,
            "Advancing MPC session"
        );
        let session_identifier =
            CommitmentSizedNumber::from_le_slice(&computation_id.session_identifier.into_bytes());
        let party_to_authority_map = self.committee.party_to_authority_map();
        let mpc_protocol_name = self.input.to_string();

        // Create a base logger with common parameters.
        let base_logger = MPCSessionLogger::new()
            .with_protocol_name(mpc_protocol_name.clone())
            .with_party_to_authority_map(party_to_authority_map.clone());

        // Derive a one-time use, MPC protocol and round specific, deterministic random generator
        // from the private seed. This should only be used to `advance()` this specific round,
        // and is guaranteed to be deterministic - if we attempt to run the round twice, the same message will be generated.
        // SECURITY NOTICE: don't use for anything else other than (this particular) `advance()`, and keep private!
        let rng = root_seed.mpc_round_rng(
            session_identifier,
            self.mpc_round as u64,
            self.attempt_number as u64,
        );

        match &self.input {
            MPCRequestInput::DWalletImportedKeyVerificationRequest(event_data) => {
                let PublicInput::DWalletImportedKeyVerificationRequest(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                // TODO: what
                // access_structure: WeightedThresholdAccessStructure,

                let result = advance_and_serialize::<DWalletImportedKeyVerificationParty>(
                    session_identifier,
                    self.party_id,
                    &self.access_structure,
                    self.messages,
                    public_input,
                    (),
                    &base_logger,
                    rng,
                );
                match result.clone() {
                    Ok(AsynchronousRoundResult::Finalize {
                           public_output,
                           malicious_parties,
                           private_output,
                       }) => {
                        verify_encrypted_share(
                            &EncryptedShareVerificationRequestEvent {
                                decentralized_public_output: bcs::to_bytes(
                                    &VersionedDwalletDKGSecondRoundPublicOutput::V1(
                                        public_output.clone(),
                                    ),
                                )?,
                                encrypted_centralized_secret_share_and_proof: event_data
                                    .event_data
                                    .encrypted_centralized_secret_share_and_proof
                                    .clone(),
                                encryption_key: event_data.event_data.encryption_key.clone(),
                                encryption_key_id: event_data.event_data.encryption_key_id,
                                dwallet_network_encryption_key_id: event_data
                                    .event_data
                                    .dwallet_network_encryption_key_id,
                                curve: event_data.event_data.curve,

                                // Fields not relevant for verification; passing empty values.
                                dwallet_id: ObjectID::new([0; 32]),
                                source_encrypted_user_secret_key_share_id: ObjectID::new([0; 32]),
                                encrypted_user_secret_key_share_id: ObjectID::new([0; 32]),
                            },
                            public_input.protocol_public_parameters.clone(),
                        )?;
                        let public_output = bcs::to_bytes(
                            &VersionedDWalletImportedKeyVerificationOutput::V1(public_output),
                        )?;
                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        })
                    }
                    _ => result,
                }
            }
            MPCRequestInput::DKGFirst(..) => {
                info!(
                    mpc_protocol=?mpc_event_data.request_input,
                    validator=?self.validator_name,
                    session_identifier=?self.session_identifier,
                    crypto_round=?self.current_mpc_round,
                    "Advancing DKG first party",
                );
                let PublicInput::DKGFirst(public_input) = &mpc_event_data.public_input else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                let result = advance_and_serialize::<DWalletDKGFirstParty>(
                    session_identifier,
                    self.party_id,
                    &self.access_structure,
                    self.messages_by_consensus_round.clone(),
                    public_input,
                    (),
                    &base_logger,
                    rng,
                );
                match result.clone() {
                    Ok(AsynchronousRoundResult::Finalize {
                           public_output,
                           malicious_parties,
                           private_output,
                       }) => {
                        let public_output = bcs::to_bytes(
                            &VersionedDwalletDKGFirstRoundPublicOutput::V1(public_output),
                        )?;
                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        })
                    }
                    _ => result,
                }
            }
            MPCRequestInput::DKGSecond(event_data) => {
                let PublicInput::DKGSecond(public_input) = &mpc_event_data.public_input else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                let result = advance_and_serialize::<DWalletDKGSecondParty>(
                    session_identifier,
                    self.party_id,
                    &self.access_structure,
                    self.messages_by_consensus_round.clone(),
                    public_input,
                    (),
                    &base_logger,
                    rng,
                )?;
                if let AsynchronousRoundResult::Finalize { public_output, .. } = &result {
                    verify_encrypted_share(
                        &EncryptedShareVerificationRequestEvent {
                            decentralized_public_output: bcs::to_bytes(
                                &VersionedDwalletDKGSecondRoundPublicOutput::V1(
                                    public_output.clone(),
                                ),
                            )?,
                            encrypted_centralized_secret_share_and_proof: event_data
                                .event_data
                                .encrypted_centralized_secret_share_and_proof
                                .clone(),
                            encryption_key: event_data.event_data.encryption_key.clone(),
                            encryption_key_id: event_data.event_data.encryption_key_id,
                            dwallet_network_encryption_key_id: event_data
                                .event_data
                                .dwallet_network_encryption_key_id,
                            curve: event_data.event_data.curve,

                            // Fields not relevant for verification; passing empty values.
                            dwallet_id: ObjectID::new([0; 32]),
                            source_encrypted_user_secret_key_share_id: ObjectID::new([0; 32]),
                            encrypted_user_secret_key_share_id: ObjectID::new([0; 32]),
                        },
                        public_input.protocol_public_parameters.clone(),
                    )?;
                }
                match result.clone() {
                    AsynchronousRoundResult::Finalize {
                        public_output,
                        malicious_parties,
                        private_output,
                    } => {
                        let public_output = bcs::to_bytes(
                            &VersionedDwalletDKGSecondRoundPublicOutput::V1(public_output),
                        )?;
                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        })
                    }
                    _ => Ok(result),
                }
            }
            MPCRequestInput::Presign(..) => {
                let PublicInput::Presign(public_input) = &mpc_event_data.public_input else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                let result = advance_and_serialize::<PresignParty>(
                    session_identifier,
                    self.party_id,
                    &self.access_structure,
                    self.messages_by_consensus_round.clone(),
                    public_input,
                    (),
                    &base_logger,
                    rng,
                );
                match result.clone() {
                    Ok(AsynchronousRoundResult::Finalize {
                           public_output,
                           malicious_parties,
                           private_output,
                       }) => {
                        let public_output =
                            bcs::to_bytes(&VersionedPresignOutput::V1(public_output))?;
                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        })
                    }
                    _ => result,
                }
            }
            MPCRequestInput::Sign(..) => {
                if let Some(decryption_key_shares) = mpc_event_data.decryption_key_shares.clone() {
                    let raw_decryption_key_shares = decryption_key_shares
                        .iter()
                        .map(|(party_id, share)| (*party_id, share.decryption_key_share))
                        .collect::<HashMap<_, _>>();

                    // Extend base logger with decryption key shares for Sign protocol
                    let logger =
                        base_logger.with_decryption_key_shares(raw_decryption_key_shares.clone());
                    let PublicInput::Sign(public_input) = &mpc_event_data.public_input else {
                        error!(
                            should_never_happen =? true,
                            mpc_protocol=?mpc_event_data.request_input,
                            validator=?self.validator_name,
                            session_identifier=?self.session_identifier,
                            crypto_round=?self.current_mpc_round,
                            access_structure=?self.access_structure,
                            ?messages_skeleton,
                            "session public input does not match the session type"
                        );
                        return Err(DwalletMPCError::InvalidSessionPublicInput);
                    };
                    let result = advance_and_serialize::<SignFirstParty>(
                        session_identifier,
                        self.party_id,
                        &self.access_structure,
                        self.messages_by_consensus_round.clone(),
                        public_input,
                        decryption_key_shares,
                        &logger,
                        rng,
                    );
                    self.update_expected_decrypters_metrics(&public_input.expected_decrypters)?;
                    match result.clone() {
                        Ok(AsynchronousRoundResult::Finalize {
                               public_output,
                               malicious_parties,
                               private_output,
                           }) => {
                            let public_output =
                                bcs::to_bytes(&VersionedSignOutput::V1(public_output))?;
                            Ok(AsynchronousRoundResult::Finalize {
                                public_output,
                                malicious_parties,
                                private_output,
                            })
                        }
                        _ => result,
                    }
                } else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "no decryption key shares for a session that requires them (sign)"
                    );

                    Err(DwalletMPCError::InvalidSessionPublicInput)
                }
            }
            MPCRequestInput::NetworkEncryptionKeyDkg(key_scheme, _init_event) => {
                advance_network_dkg(
                    session_identifier,
                    &self.access_structure,
                    &self.mpc_event_data.clone().unwrap(),
                    self.party_id,
                    key_scheme,
                    self.messages_by_consensus_round.clone(),
                    bcs::from_bytes(
                        &mpc_event_data
                            .private_input
                            .clone()
                            .ok_or(DwalletMPCError::MissingMPCPrivateInput)?,
                    )?,
                    &base_logger,
                    rng,
                )
            }
            MPCRequestInput::EncryptedShareVerification(verification_data) => {
                let PublicInput::EncryptedShareVerification(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };
                match verify_encrypted_share(&verification_data.event_data, public_input.clone()) {
                    Ok(_) => Ok(AsynchronousRoundResult::Finalize {
                        public_output: vec![],
                        private_output: vec![],
                        malicious_parties: vec![],
                    }),
                    Err(err) => Err(err),
                }
            }
            MPCRequestInput::PartialSignatureVerification(event_data) => {
                let hashed_message = bcs::to_bytes(
                    &message_digest(
                        &event_data.event_data.message,
                        &event_data.event_data.hash_scheme.try_into().unwrap(),
                    )
                        .map_err(|err| DwalletMPCError::TwoPCMPCError(err.to_string()))?,
                )?;
                let PublicInput::PartialSignatureVerification(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };
                verify_partial_signature(
                    &hashed_message,
                    &event_data.event_data.dkg_output,
                    &event_data.event_data.presign,
                    &event_data.event_data.message_centralized_signature,
                    public_input,
                )?;

                Ok(AsynchronousRoundResult::Finalize {
                    public_output: vec![],
                    private_output: vec![],
                    malicious_parties: vec![],
                })
            }
            MPCRequestInput::NetworkEncryptionKeyReconfiguration(_) => {
                let PublicInput::NetworkEncryptionKeyReconfiguration(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                if let Some(decryption_key_shares) = mpc_event_data.decryption_key_shares.clone() {
                    let decryption_key_shares = decryption_key_shares
                        .iter()
                        .map(|(party_id, share)| (*party_id, share.decryption_key_share))
                        .collect::<HashMap<_, _>>();

                    // Extend base logger with decryption key shares for Reconfiguration protocol
                    let logger =
                        base_logger.with_decryption_key_shares(decryption_key_shares.clone());

                    let result = advance_and_serialize::<ReconfigurationSecp256k1Party>(
                        session_identifier,
                        self.party_id,
                        &self.access_structure,
                        self.messages_by_consensus_round.clone(),
                        public_input,
                        decryption_key_shares,
                        &logger,
                        rng,
                    );

                    match result.clone() {
                        Ok(AsynchronousRoundResult::Finalize {
                               public_output,
                               malicious_parties,
                               private_output,
                           }) => {
                            let public_output = bcs::to_bytes(
                                &VersionedDecryptionKeyReconfigurationOutput::V1(public_output),
                            )?;
                            Ok(AsynchronousRoundResult::Finalize {
                                public_output,
                                malicious_parties,
                                private_output,
                            })
                        }
                        _ => result,
                    }
                } else {
                    error!(
                    should_never_happen =? true,
                    mpc_protocol=?mpc_event_data.request_input,
                    validator=?self.validator_name,
                    session_identifier=?self.session_identifier,
                    crypto_round=?self.current_mpc_round,
                    access_structure=?self.access_structure,
                    ?messages_skeleton,
                    "no decryption key shares for a session that requires them (reconfiguration)"
                    );

                    Err(DwalletMPCError::InvalidSessionPublicInput)
                }
            }
            MPCRequestInput::MakeDWalletUserSecretKeySharesPublicRequest(init_event) => {
                let PublicInput::MakeDWalletUserSecretKeySharesPublicPublicInput(public_input) =
                    &mpc_event_data.public_input
                else {
                    error!(
                        should_never_happen =? true,
                        mpc_protocol=?mpc_event_data.request_input,
                        validator=?self.validator_name,
                        session_identifier=?self.session_identifier,
                        crypto_round=?self.current_mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };
                match verify_secret_share(
                    public_input.clone(),
                    init_event.event_data.public_user_secret_key_shares.clone(),
                    init_event.event_data.public_output.clone(),
                ) {
                    Ok(..) => Ok(AsynchronousRoundResult::Finalize {
                        public_output: init_event.event_data.public_user_secret_key_shares.clone(),
                        private_output: vec![],
                        malicious_parties: vec![],
                    }),
                    Err(err) => {
                        error!(
                            ?err,
                            session_identifier=?self.session_identifier,
                            validator=?self.validator_name,
                            crypto_round=?self.current_mpc_round,
                            "failed to verify secret share"
                        );
                        Err(DwalletMPCError::DWalletSecretNotMatchedDWalletOutput)
                    }
                }
            }
        }
    }

    /// Advances the MPC session and sends the advancement result to the other validators.
    /// The consensus submission logic is being spawned as a separate tokio task, as it's an IO
    /// heavy task.
    /// Rayon, which is good for CPU heavy tasks, is used to perform the cryptographic
    /// computation, and Tokio, which is good for IO heavy tasks, is used to submit the result to
    /// the consensus.
    pub(super) fn advance(&mut self, tokio_runtime_handle: &Handle) -> DwalletMPCResult<()> {
        // TODO: no handle I think.

        // Make sure we transfer only the messages up to the current round
        // (exclude messages that might be received from future rounds)
        self.messages_by_consensus_round
            .retain(|round, _| round < &self.current_mpc_round);
        // Safe to unwrap as advance can only be called after the event is received.
        let mpc_protocol = self.mpc_event_data.clone().unwrap().request_input;
        match self.advance_specific_party() {
            Ok(AsynchronousRoundResult::Advance {
                   malicious_parties,
                   message,
               }) => {
                let session_id = self.session_identifier;
                let round_number = self.messages_by_consensus_round.len();
                info!(
                    mpc_protocol=?mpc_protocol,
                    session_id=?session_id,
                    validator=?self.validator_name,
                    round=?round_number,
                    "Advanced MPC session"
                );
                let consensus_adapter = self.consensus_adapter.clone();
                if !malicious_parties.is_empty() {
                    self.report_malicious_actors(malicious_parties, &self.committee)?;
                }
                let message = self.new_dwallet_mpc_message(message)?;

                // TODO: not from here! send in channel
                // tokio_runtime_handle.spawn(async move {
                //     if let Err(err) = consensus_adapter
                //         .submit_to_consensus(&[message], &epoch_store)
                //         .await
                //     {
                //         error!(
                //             mpc_protocol=?mpc_protocol,
                //             session_id=?session_id,
                //             validator=?validator_name,
                //             round=?round_number,
                //             err=?err,
                //             "failed to submit an MPC message to consensus"
                //         );
                //     }
                // });

                Ok(())
            }
            Ok(AsynchronousRoundResult::Finalize {
                   malicious_parties,
                   private_output: _,
                   public_output,
               }) => {
                info!(
                    mpc_protocol=?&mpc_protocol,
                    session_identifier=?self.session_identifier,
                    validator=?&self.validator_name,
                    "Reached public output (Finalize) for session"
                );
                let consensus_adapter = self.consensus_adapter.clone();
                if !malicious_parties.is_empty() {
                    warn!(
                        mpc_protocol=?&mpc_protocol,
                        session_identifier=?self.session_identifier,
                        validator=?&self.validator_name,
                        ?malicious_parties,
                        "Malicious Parties detected on MPC session Finalize",
                    );
                    self.report_malicious_actors(malicious_parties, &self.committee)?;
                }
                let consensus_message = self.new_dwallet_mpc_output_message(
                    MPCSessionPublicOutput::CompletedSuccessfully(public_output.clone()),
                )?;
                let session_id_clone = self.session_identifier;

                // TODO: not from here!
                // tokio_runtime_handle.spawn(async move {
                //     if let Err(err) = consensus_adapter
                //         .submit_to_consensus(&[consensus_message], &epoch_store)
                //         .await
                //     {
                //         error!(
                //             mpc_protocol=?mpc_protocol,
                //             session_id=?session_id_clone,
                //             validator=?validator_name,
                //             err=?err,
                //             "failed to submit an MPC output message to consensus",
                //         );
                //     }
                // });

                Ok(())
            }
            Err(DwalletMPCError::TWOPCMPCThresholdNotReached) => {
                error!(
                    err=?DwalletMPCError::TWOPCMPCThresholdNotReached,
                    session_identifier=?self.session_identifier,
                    validator=?self.validator_name,
                    crypto_round=?self.current_mpc_round,
                    party_id=?self.party_id,
                    mpc_protocol=?&mpc_protocol,
                    "MPC session failed"
                );
                self.report_threshold_not_reached(tokio_runtime_handle)
            }
            Err(err) => {
                error!(
                    session_identifier=?self.session_identifier,
                    validator=?self.validator_name,
                    crypto_round=?self.current_mpc_round,
                    party_id=?self.party_id,
                    error=?err,
                    mpc_protocol=?mpc_protocol,
                    epoch=?self.epoch_id,
                    "failed to advance the MPC session"
                );

                let consensus_adapter = self.consensus_adapter.clone();
                let consensus_message =
                    self.new_dwallet_mpc_output_message(MPCSessionPublicOutput::SessionFailed)?;
                let session_id_clone = self.session_identifier;
                let epoch_id_clone = self.epoch_id;

                // TODO(Scaly): what is this code
                // tokio_runtime_handle.spawn(async move {
                //     if let Err(err) = consensus_adapter
                //         .submit_to_consensus(&[consensus_message], &epoch_store)
                //         .await
                //     {
                //         error!(
                //             mpc_protocol=?&mpc_protocol,
                //             session_id=?session_id_clone,
                //             validator=?validator_name,
                //             epoch=?epoch_id_clone,
                //             error=?err,
                //             "failed to submit an MPC SessionFailed message to consensus");
                //     }
                // });

                Err(err)
            }
        }
    }
}

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
