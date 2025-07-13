use crate::dwallet_mpc::dwallet_dkg::{
    DWalletDKGFirstParty, DWalletDKGSecondParty, DWalletImportedKeyVerificationParty,
};
use crate::dwallet_mpc::encrypt_user_share::verify_encrypted_share;
use crate::dwallet_mpc::make_dwallet_user_secret_key_shares_public::verify_secret_share;
use crate::dwallet_mpc::mpc_session::MPCSessionLogger;
use crate::dwallet_mpc::mpc_session::PublicInput;
use crate::dwallet_mpc::network_dkg::advance_network_dkg;
use crate::dwallet_mpc::presign::PresignParty;
use crate::dwallet_mpc::reconfiguration::ReconfigurationSecp256k1Party;
use crate::dwallet_mpc::sign::{
    update_expected_decrypters_metrics, verify_partial_signature, SignFirstParty,
};
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{
    MPCMessage, MPCPrivateInput, MPCPrivateOutput, SerializedWrappedMPCPublicOutput,
    VersionedDWalletImportedKeyVerificationOutput, VersionedDecryptionKeyReconfigurationOutput,
    VersionedDwalletDKGFirstRoundPublicOutput, VersionedDwalletDKGSecondRoundPublicOutput,
    VersionedPresignOutput, VersionedSignOutput,
};
use dwallet_rng::RootSeed;
use group::PartyID;
use ika_types::committee::Committee;
use ika_types::crypto::AuthorityPublicKeyBytes;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{
    AsyncProtocol, EncryptedShareVerificationRequestEvent, MPCRequestInput, SessionIdentifier,
};
use itertools::Itertools;
use message_digest::message_digest::message_digest;
use mpc::{AsynchronousRoundResult, WeightedThresholdAccessStructure};
use std::collections::HashMap;
use std::sync::Arc;
use sui_types::base_types::ObjectID;
use tracing::{error, info};
use twopc_mpc::sign::Protocol;

pub(super) mod mpc_computations;
pub(super) mod native_computations;
mod orchestrator;

use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
pub(crate) use mpc_computations::advance_and_serialize;
pub(crate) use orchestrator::CryptographicComputationsOrchestrator;

const MPC_SIGN_SECOND_ROUND: usize = 2;

/// A unique key for a computation request.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) struct ComputationId {
    pub(crate) session_identifier: SessionIdentifier,
    /// The consensus round at which this computation executed (if it is synced with the consensus).
    /// The first MPC round will be `None`, since it isn't synced — it is launched when the
    /// event is received from Sui.
    /// All other MPC rounds will set this to `Some()` with the value being the last consensus
    /// round from which we gathered messages to advance.
    pub(crate) consensus_round: Option<u64>,
    pub(crate) mpc_round: usize,
    pub(crate) attempt_number: usize,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct ComputationRequest {
    pub(crate) party_id: PartyID,
    pub(crate) validator_name: AuthorityPublicKeyBytes,
    pub(crate) committee: Arc<Committee>,
    pub(crate) access_structure: WeightedThresholdAccessStructure,
    pub(crate) private_input: MPCPrivateInput,
    pub(crate) public_input: PublicInput,
    pub(crate) request_input: MPCRequestInput,
    pub(crate) decryption_key_shares:
        Option<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>,
    /// Round -> Messages map.
    pub(crate) messages: HashMap<usize, HashMap<PartyID, MPCMessage>>,
}

impl ComputationRequest {
    /// Perform a cryptographic computation.
    /// Notice: `root_seed` must be kept private!
    pub(crate) fn compute(
        self,
        computation_id: ComputationId,
        root_seed: RootSeed,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    ) -> DwalletMPCResult<
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
            mpc_protocol=?self.request_input,
            validator=?self.validator_name,
            session_identifier=?computation_id.session_identifier,
            mpc_round=?computation_id.mpc_round,
            access_structure=?self.access_structure,
            ?messages_skeleton,
            "Advancing an MPC session"
        );
        let session_id =
            CommitmentSizedNumber::from_le_slice(&computation_id.session_identifier.into_bytes());
        let party_to_authority_map = self.committee.party_to_authority_map();
        let mpc_protocol_name = self.request_input.to_string();

        // Create a base logger with common parameters.
        let base_logger = MPCSessionLogger::new()
            .with_protocol_name(mpc_protocol_name.clone())
            .with_party_to_authority_map(party_to_authority_map.clone());

        // Derive a one-time use, MPC protocol and round specific, deterministic random generator
        // from the private seed.
        // This should only be used to `advance()` this specific round, and is guaranteed to be
        // deterministic — if we attempt to run the round twice, the same message will be generated.
        // SECURITY NOTICE: don't use for anything else other than (this particular) `advance()`,
        // and keep private!
        let rng = root_seed.mpc_round_rng(
            session_id,
            computation_id.mpc_round as u64,
            computation_id.attempt_number as u64,
        );

        match &self.request_input {
            MPCRequestInput::DWalletImportedKeyVerificationRequest(event_data) => {
                let PublicInput::DWalletImportedKeyVerificationRequest(public_input) =
                    &self.public_input
                else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?self.request_input,
                        validator=?self.validator_name,
                        session_identifier=?computation_id.session_identifier,
                        mpc_round=?computation_id.mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                let result = advance_and_serialize::<DWalletImportedKeyVerificationParty>(
                    session_id,
                    self.party_id,
                    &self.access_structure,
                    self.messages,
                    public_input,
                    (),
                    &base_logger,
                    rng,
                )?;
                match result {
                    AsynchronousRoundResult::Finalize {
                        public_output,
                        malicious_parties,
                        private_output,
                    } => {
                        // Verify the encrypted share before finalizing, guaranteeing a two-for-one
                        // computation of both that the key import was successful, and
                        // the encrypted user share is valid.
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

                        // Wrap the public output with its version.
                        let public_output = bcs::to_bytes(
                            &VersionedDWalletImportedKeyVerificationOutput::V1(public_output),
                        )?;

                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        })
                    }
                    AsynchronousRoundResult::Advance {
                        malicious_parties,
                        message,
                    } => Ok(AsynchronousRoundResult::Advance {
                        malicious_parties,
                        message,
                    }),
                }
            }
            MPCRequestInput::DKGFirst(..) => {
                info!(
                    mpc_protocol=?self.request_input,
                    validator=?self.validator_name,
                    session_identifier=?computation_id.session_identifier,
                    mpc_round=?computation_id.mpc_round,
                    "Advancing DKG first party",
                );
                let PublicInput::DKGFirst(public_input) = &self.public_input else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?self.request_input,
                        validator=?self.validator_name,
                        session_identifier=?computation_id.session_identifier,
                        mpc_round=?computation_id.mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                let result = advance_and_serialize::<DWalletDKGFirstParty>(
                    session_id,
                    self.party_id,
                    &self.access_structure,
                    self.messages,
                    public_input,
                    (),
                    &base_logger,
                    rng,
                )?;

                match result {
                    AsynchronousRoundResult::Finalize {
                        public_output,
                        malicious_parties,
                        private_output,
                    } => {
                        // Wrap the public output with its version.
                        let public_output = bcs::to_bytes(
                            &VersionedDwalletDKGFirstRoundPublicOutput::V1(public_output),
                        )?;

                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        })
                    }
                    AsynchronousRoundResult::Advance {
                        malicious_parties,
                        message,
                    } => Ok(AsynchronousRoundResult::Advance {
                        malicious_parties,
                        message,
                    }),
                }
            }
            MPCRequestInput::DKGSecond(event_data) => {
                let PublicInput::DKGSecond(public_input) = &self.public_input else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?self.request_input,
                        validator=?self.validator_name,
                        session_identifier=?computation_id.session_identifier,
                        mpc_round=?computation_id.mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "session public input does not match the session type"
                    );

                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                let result = advance_and_serialize::<DWalletDKGSecondParty>(
                    session_id,
                    self.party_id,
                    &self.access_structure,
                    self.messages,
                    public_input,
                    (),
                    &base_logger,
                    rng,
                )?;

                if let AsynchronousRoundResult::Finalize { public_output, .. } = &result {
                    // Verify the encrypted share before finalizing, guaranteeing a two-for-one
                    // computation of both that the dkg was successful, and the encrypted user share is valid.
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

                match result {
                    AsynchronousRoundResult::Finalize {
                        public_output,
                        malicious_parties,
                        private_output,
                    } => {
                        // Wrap the public output with its version.
                        let public_output = bcs::to_bytes(
                            &VersionedDwalletDKGSecondRoundPublicOutput::V1(public_output),
                        )?;
                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        })
                    }
                    AsynchronousRoundResult::Advance {
                        malicious_parties,
                        message,
                    } => Ok(AsynchronousRoundResult::Advance {
                        malicious_parties,
                        message,
                    }),
                }
            }
            MPCRequestInput::Presign(..) => {
                let PublicInput::Presign(public_input) = &self.public_input else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?self.request_input,
                        validator=?self.validator_name,
                        session_identifier=?computation_id.session_identifier,
                        mpc_round=?computation_id.mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                let result = advance_and_serialize::<PresignParty>(
                    session_id,
                    self.party_id,
                    &self.access_structure,
                    self.messages,
                    public_input,
                    (),
                    &base_logger,
                    rng,
                )?;

                match result {
                    AsynchronousRoundResult::Finalize {
                        public_output,
                        malicious_parties,
                        private_output,
                    } => {
                        // Wrap the public output with its version.
                        let public_output =
                            bcs::to_bytes(&VersionedPresignOutput::V1(public_output))?;
                        Ok(AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        })
                    }
                    AsynchronousRoundResult::Advance {
                        malicious_parties,
                        message,
                    } => Ok(AsynchronousRoundResult::Advance {
                        malicious_parties,
                        message,
                    }),
                }
            }
            MPCRequestInput::Sign(..) => {
                if let Some(decryption_key_shares) = self.decryption_key_shares.clone() {
                    let raw_decryption_key_shares = decryption_key_shares
                        .iter()
                        .map(|(party_id, share)| (*party_id, share.decryption_key_share))
                        .collect::<HashMap<_, _>>();

                    // Extend base logger with decryption key shares for Sign protocol
                    let logger =
                        base_logger.with_decryption_key_shares(raw_decryption_key_shares.clone());
                    let PublicInput::Sign(public_input) = &self.public_input else {
                        error!(
                            should_never_happen=?true,
                            mpc_protocol=?self.request_input,
                            validator=?self.validator_name,
                            session_identifier=?computation_id.session_identifier,
                            mpc_round=?computation_id.mpc_round,
                            access_structure=?self.access_structure,
                            ?messages_skeleton,
                            "session public input does not match the session type"
                        );
                        return Err(DwalletMPCError::InvalidSessionPublicInput);
                    };

                    if computation_id.mpc_round == MPC_SIGN_SECOND_ROUND {
                        if let Some(sign_first_round_messages) = self.messages.get(&1) {
                            let decrypters = sign_first_round_messages.keys().copied().collect();
                            update_expected_decrypters_metrics(
                                &public_input.expected_decrypters,
                                decrypters,
                                &self.access_structure,
                                dwallet_mpc_metrics,
                            );
                        }
                    }

                    let result = advance_and_serialize::<SignFirstParty>(
                        session_id,
                        self.party_id,
                        &self.access_structure,
                        self.messages,
                        public_input,
                        decryption_key_shares,
                        &logger,
                        rng,
                    )?;

                    match result {
                        AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        } => {
                            // Wrap the public output with its version.
                            let public_output =
                                bcs::to_bytes(&VersionedSignOutput::V1(public_output))?;

                            Ok(AsynchronousRoundResult::Finalize {
                                public_output,
                                malicious_parties,
                                private_output,
                            })
                        }
                        AsynchronousRoundResult::Advance {
                            malicious_parties,
                            message,
                        } => Ok(AsynchronousRoundResult::Advance {
                            malicious_parties,
                            message,
                        }),
                    }
                } else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?self.request_input,
                        validator=?self.validator_name,
                        session_identifier=?computation_id.session_identifier,
                        mpc_round=?computation_id.mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "no decryption key shares for a session that requires them (sign)"
                    );

                    Err(DwalletMPCError::InvalidSessionPublicInput)
                }
            }
            MPCRequestInput::NetworkEncryptionKeyDkg(key_scheme, _init_event) => {
                advance_network_dkg(
                    session_id,
                    &self.access_structure,
                    &self.public_input,
                    self.party_id,
                    key_scheme,
                    self.messages,
                    bcs::from_bytes(
                        &self
                            .private_input
                            .clone()
                            .ok_or(DwalletMPCError::MissingMPCPrivateInput)?,
                    )?,
                    &base_logger,
                    rng,
                )
            }
            MPCRequestInput::EncryptedShareVerification(verification_data) => {
                let PublicInput::EncryptedShareVerification(public_input) = &self.public_input
                else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?self.request_input,
                        validator=?self.validator_name,
                        session_identifier=?computation_id.session_identifier,
                        mpc_round=?computation_id.mpc_round,
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
                let PublicInput::PartialSignatureVerification(public_input) = &self.public_input
                else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?self.request_input,
                        validator=?self.validator_name,
                        session_identifier=?computation_id.session_identifier,
                        mpc_round=?computation_id.mpc_round,
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
                    &self.public_input
                else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?self.request_input,
                        validator=?self.validator_name,
                        session_identifier=?computation_id.session_identifier,
                        mpc_round=?computation_id.mpc_round,
                        access_structure=?self.access_structure,
                        ?messages_skeleton,
                        "session public input does not match the session type"
                    );
                    return Err(DwalletMPCError::InvalidSessionPublicInput);
                };

                if let Some(decryption_key_shares) = self.decryption_key_shares.clone() {
                    let decryption_key_shares = decryption_key_shares
                        .iter()
                        .map(|(party_id, share)| (*party_id, share.decryption_key_share))
                        .collect::<HashMap<_, _>>();

                    // Extend base logger with decryption key shares for Reconfiguration protocol
                    let logger =
                        base_logger.with_decryption_key_shares(decryption_key_shares.clone());

                    let result = advance_and_serialize::<ReconfigurationSecp256k1Party>(
                        session_id,
                        self.party_id,
                        &self.access_structure,
                        self.messages,
                        public_input,
                        decryption_key_shares,
                        &logger,
                        rng,
                    )?;

                    match result {
                        AsynchronousRoundResult::Finalize {
                            public_output,
                            malicious_parties,
                            private_output,
                        } => {
                            // Wrap the public output with its version.
                            let public_output = bcs::to_bytes(
                                &VersionedDecryptionKeyReconfigurationOutput::V1(public_output),
                            )?;

                            Ok(AsynchronousRoundResult::Finalize {
                                public_output,
                                malicious_parties,
                                private_output,
                            })
                        }
                        AsynchronousRoundResult::Advance {
                            malicious_parties,
                            message,
                        } => Ok(AsynchronousRoundResult::Advance {
                            malicious_parties,
                            message,
                        }),
                    }
                } else {
                    error!(
                    should_never_happen=?true,
                    mpc_protocol=?self.request_input,
                    validator=?self.validator_name,
                    session_identifier=?computation_id.session_identifier,
                    mpc_round=?computation_id.mpc_round,
                    access_structure=?self.access_structure,
                    ?messages_skeleton,
                    "no decryption key shares for a session that requires them (reconfiguration)"
                    );

                    Err(DwalletMPCError::InvalidSessionPublicInput)
                }
            }
            MPCRequestInput::MakeDWalletUserSecretKeySharesPublicRequest(init_event) => {
                let PublicInput::MakeDWalletUserSecretKeySharesPublicParameters(public_input) =
                    &self.public_input
                else {
                    error!(
                        should_never_happen=?true,
                        mpc_protocol=?self.request_input,
                        validator=?self.validator_name,
                        session_identifier=?computation_id.session_identifier,
                        mpc_round=?computation_id.mpc_round,
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
                            session_identifier=?computation_id.session_identifier,
                            validator=?self.validator_name,
                            mpc_round=?computation_id.mpc_round,
                            "failed to verify secret share"
                        );
                        Err(DwalletMPCError::DWalletSecretNotMatchedDWalletOutput)
                    }
                }
            }
        }
    }
}
