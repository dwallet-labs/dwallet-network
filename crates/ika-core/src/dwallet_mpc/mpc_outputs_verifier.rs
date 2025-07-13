//! A module to verify the dWallet MPC outputs.
//! The module handles storing the outputs received for each session,
//! and deciding whether an output is valid
//! by checking if an authorized validator set voted for it.
//! Any validator that voted for a different output is considered malicious.

use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::stake_aggregator::StakeAggregator;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCSessionPublicOutput, SerializedWrappedMPCPublicOutput,
};
use ika_types::committee::Committee;
use ika_types::crypto::AuthorityName;
use ika_types::crypto::AuthorityPublicKeyBytes;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::message::{
    DKGFirstRoundOutput, DKGSecondRoundOutput, DWalletCheckpointMessageKind,
    DWalletImportedKeyVerificationOutput, EncryptedUserShareOutput, MPCNetworkDKGOutput,
    MPCNetworkReconfigurationOutput, MakeDWalletUserSecretKeySharesPublicOutput,
    PartialSignatureVerificationOutput, PresignOutput, SignOutput,
};
use ika_types::messages_dwallet_mpc::{
    MPCRequestInput, MPCSessionRequest, SessionIdentifier, SessionType,
};
use itertools::Itertools;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tracing::info;

const FIVE_KILO_BYTES: usize = 5 * 1024;

/// Verify the DWallet MPC outputs.
///
/// Stores all the outputs received for each session,
/// and decides whether an output is valid
/// by checking if a validators with quorum of stake voted for it.
pub struct DWalletMPCOutputsVerifier {
    /// The outputs received for each MPC session.
    mpc_sessions_outputs: HashMap<SessionIdentifier, SessionOutputsData>,
    dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
}

/// The data needed to manage the outputs of an MPC session.
struct SessionOutputsData {
    /// Maps session's output to the authorities that voted for it.
    /// The key must contain the session info, and the output to prevent
    /// malicious behavior, such as sending the correct output, but from a faulty session.
    session_output_to_voting_authorities:
        HashMap<(SerializedWrappedMPCPublicOutput, MPCSessionRequest), StakeAggregator<(), true>>,
    /// Needed to make sure an authority does not send two outputs for the same session.
    authorities_that_sent_output: HashSet<AuthorityName>,
    current_result: OutputVerificationStatus,
}

impl SessionOutputsData {
    fn clear_data(&mut self) {
        self.session_output_to_voting_authorities.clear();
        self.authorities_that_sent_output.clear();
    }
}

/// The result of verifying an incoming output for an MPC session.
/// We need to differentiate between a duplicate and a malicious output,
/// as the output can be sent twice by honest parties.
#[derive(PartialEq, Clone)]
pub enum OutputVerificationStatus {
    FirstQuorumReached(Vec<DWalletCheckpointMessageKind>),
    Malicious,
    /// We need more votes to decide if the output is valid or not.
    NotEnoughVotes,
    /// The output has already been verified and committed to the chain.
    /// This happens every time since all honest parties send the same output.
    AlreadyCommitted,
}

pub struct OutputVerificationResult {
    pub result: OutputVerificationStatus,
    pub malicious_actors: Vec<AuthorityName>,
}

impl DWalletMPCOutputsVerifier {
    pub fn new(dwallet_mpc_metrics: Arc<DWalletMPCMetrics>) -> Self {
        DWalletMPCOutputsVerifier {
            mpc_sessions_outputs: HashMap::new(),
            dwallet_mpc_metrics,
        }
    }

    /// Stores the given MPC output, and checks if any of the received
    /// outputs already received a quorum of votes.
    /// If so, the output is returned along with a vector of malicious actors,
    /// i.e., parties that voted for other outputs.
    // TODO (#311): Make sure validator don't mark other validators as malicious
    // TODO (#311): or take any active action while syncing
    pub fn try_verify_output(
        &mut self,
        output: &[u8],
        session_request: &MPCSessionRequest,
        origin_authority: AuthorityName,
        validator_name: AuthorityPublicKeyBytes,
        committee: Arc<Committee>,
    ) -> DwalletMPCResult<OutputVerificationResult> {
        // TODO (#876): Set the maximum message size to the smallest size possible.
        info!(
            mpc_protocol=?session_request.request_input,
            session_identifier=?session_request.session_identifier,
            from_authority=?origin_authority,
            receiving_authority=?validator_name,
            output_size_bytes=?output.len(),
            "Received DWallet MPC output",
        );

        let session_output_data = self
            .mpc_sessions_outputs
            .entry(session_request.session_identifier)
            .or_insert(SessionOutputsData {
                session_output_to_voting_authorities: HashMap::new(),
                authorities_that_sent_output: HashSet::new(),
                current_result: OutputVerificationStatus::NotEnoughVotes,
            });
        if session_output_data.current_result == OutputVerificationStatus::AlreadyCommitted {
            return Ok(OutputVerificationResult {
                result: OutputVerificationStatus::AlreadyCommitted,
                malicious_actors: vec![],
            });
        }
        // Sent more than once.
        if session_output_data
            .authorities_that_sent_output
            .contains(&origin_authority)
        {
            // Duplicate.
            return Ok(OutputVerificationResult {
                result: OutputVerificationStatus::AlreadyCommitted,
                malicious_actors: vec![],
            });
        }
        session_output_data
            .authorities_that_sent_output
            .insert(origin_authority);

        if session_output_data
            .session_output_to_voting_authorities
            .entry((output.to_owned(), session_request.clone()))
            .or_insert(StakeAggregator::new(committee))
            .insert_generic(origin_authority, ())
            .is_quorum_reached()
        {
            session_output_data.current_result = OutputVerificationStatus::AlreadyCommitted;
            session_output_data.clear_data();
            let mpc_event_data = session_request.request_input.clone();
            self.dwallet_mpc_metrics.add_completion(&mpc_event_data);
            return Ok(OutputVerificationResult {
                result: OutputVerificationStatus::FirstQuorumReached(
                    self.process_dwallet_transaction(output.to_owned(), session_request.clone())?,
                ),
                malicious_actors: vec![],
            });
        }
        Ok(OutputVerificationResult {
            result: OutputVerificationStatus::NotEnoughVotes,
            malicious_actors: vec![],
        })
    }

    fn process_dwallet_transaction(
        &self,
        output: Vec<u8>,
        session_request: MPCSessionRequest,
    ) -> DwalletMPCResult<Vec<DWalletCheckpointMessageKind>> {
        info!(
            mpc_protocol=?session_request.request_input,
            session_identifier=?session_request.session_identifier,
            "Creating session output message for checkpoint"
        );
        let (is_rejected, output) = match bcs::from_bytes(&output)? {
            MPCSessionPublicOutput::CompletedSuccessfully(output) => (false, output),
            MPCSessionPublicOutput::SessionFailed => (true, vec![]),
        };
        match &session_request.request_input {
            MPCRequestInput::DKGFirst(request_input) => {
                let SessionType::User = request_input.session_type else {
                    unreachable!("DKGFirst round should be a user session");
                };
                let tx = DWalletCheckpointMessageKind::RespondDWalletDKGFirstRoundOutput(
                    DKGFirstRoundOutput {
                        dwallet_id: request_input.event_data.dwallet_id.to_vec(),
                        output,
                        session_sequence_number: request_input.session_sequence_number,
                        rejected: is_rejected,
                    },
                );
                Ok(vec![tx])
            }
            MPCRequestInput::DKGSecond(request_input) => {
                let SessionType::User = request_input.session_type else {
                    unreachable!("DKGSecond round should be a user session");
                };
                let tx = DWalletCheckpointMessageKind::RespondDWalletDKGSecondRoundOutput(
                    DKGSecondRoundOutput {
                        output,
                        dwallet_id: request_input.event_data.dwallet_id.to_vec(),
                        encrypted_secret_share_id: request_input
                            .event_data
                            .encrypted_user_secret_key_share_id
                            .to_vec(),
                        rejected: is_rejected,
                        session_sequence_number: request_input.session_sequence_number,
                    },
                );
                Ok(vec![tx])
            }
            MPCRequestInput::Presign(request_input) => {
                let SessionType::User = request_input.session_type else {
                    unreachable!("Presign round should be a user session");
                };
                let tx = DWalletCheckpointMessageKind::RespondDWalletPresign(PresignOutput {
                    presign: output,
                    dwallet_id: request_input.event_data.dwallet_id.map(|id| id.to_vec()),
                    presign_id: request_input.event_data.presign_id.to_vec(),
                    rejected: is_rejected,
                    session_sequence_number: request_input.session_sequence_number,
                });
                Ok(vec![tx])
            }
            MPCRequestInput::Sign(request_input) => {
                let SessionType::User = request_input.session_type else {
                    unreachable!("Sign round should be a user session");
                };
                let tx = DWalletCheckpointMessageKind::RespondDWalletSign(SignOutput {
                    signature: output,
                    dwallet_id: request_input.event_data.dwallet_id.to_vec(),
                    is_future_sign: request_input.event_data.is_future_sign,
                    sign_id: request_input.event_data.sign_id.to_vec(),
                    rejected: is_rejected,
                    session_sequence_number: request_input.session_sequence_number,
                });
                Ok(vec![tx])
            }
            MPCRequestInput::EncryptedShareVerification(request_input) => {
                let SessionType::User = request_input.session_type else {
                    unreachable!("EncryptedShareVerification round should be a user session");
                };
                let tx = DWalletCheckpointMessageKind::RespondDWalletEncryptedUserShare(
                    EncryptedUserShareOutput {
                        dwallet_id: request_input.event_data.dwallet_id.to_vec(),
                        encrypted_user_secret_key_share_id: request_input
                            .event_data
                            .encrypted_user_secret_key_share_id
                            .to_vec(),
                        rejected: is_rejected,
                        session_sequence_number: request_input.session_sequence_number,
                    },
                );
                Ok(vec![tx])
            }
            MPCRequestInput::PartialSignatureVerification(request_input) => {
                let SessionType::User = request_input.session_type else {
                    unreachable!("PartialSignatureVerification round should be a user session");
                };
                let tx =
                    DWalletCheckpointMessageKind::RespondDWalletPartialSignatureVerificationOutput(
                        PartialSignatureVerificationOutput {
                            dwallet_id: request_input.event_data.dwallet_id.to_vec(),
                            partial_centralized_signed_message_id: request_input
                                .event_data
                                .partial_centralized_signed_message_id
                                .to_vec(),
                            rejected: is_rejected,
                            session_sequence_number: request_input.session_sequence_number,
                        },
                    );
                Ok(vec![tx])
            }
            MPCRequestInput::NetworkEncryptionKeyDkg(key_scheme, request_input) => match key_scheme
            {
                DWalletMPCNetworkKeyScheme::Secp256k1 => {
                    let slices = if is_rejected {
                        vec![MPCNetworkDKGOutput {
                            dwallet_network_encryption_key_id: request_input
                                .event_data
                                .dwallet_network_encryption_key_id
                                .clone()
                                .to_vec(),
                            public_output: vec![],
                            supported_curves: vec![DWalletMPCNetworkKeyScheme::Secp256k1 as u32],
                            is_last: true,
                            rejected: true,
                            session_sequence_number: request_input.session_sequence_number,
                        }]
                    } else {
                        Self::slice_public_output_into_messages(
                            output,
                            |public_output_chunk, is_last| MPCNetworkDKGOutput {
                                dwallet_network_encryption_key_id: request_input
                                    .event_data
                                    .dwallet_network_encryption_key_id
                                    .clone()
                                    .to_vec(),
                                public_output: public_output_chunk,
                                supported_curves: vec![
                                    DWalletMPCNetworkKeyScheme::Secp256k1 as u32,
                                ],
                                is_last,
                                rejected: false,
                                session_sequence_number: request_input.session_sequence_number,
                            },
                        )
                    };

                    let messages: Vec<_> = slices
                        .into_iter()
                        .map(DWalletCheckpointMessageKind::RespondDWalletMPCNetworkDKGOutput)
                        .collect();
                    Ok(messages)
                }
                DWalletMPCNetworkKeyScheme::Ristretto => {
                    Err(DwalletMPCError::UnsupportedNetworkDKGKeyScheme)
                }
            },
            MPCRequestInput::NetworkEncryptionKeyReconfiguration(request_input) => {
                let slices = if is_rejected {
                    vec![MPCNetworkReconfigurationOutput {
                        dwallet_network_encryption_key_id: request_input
                            .event_data
                            .dwallet_network_encryption_key_id
                            .clone()
                            .to_vec(),
                        public_output: vec![],
                        supported_curves: vec![DWalletMPCNetworkKeyScheme::Secp256k1 as u32],
                        is_last: true,
                        rejected: true,
                        session_sequence_number: request_input.session_sequence_number,
                    }]
                } else {
                    Self::slice_public_output_into_messages(
                        output,
                        |public_output_chunk, is_last| MPCNetworkReconfigurationOutput {
                            dwallet_network_encryption_key_id: request_input
                                .event_data
                                .dwallet_network_encryption_key_id
                                .clone()
                                .to_vec(),
                            public_output: public_output_chunk,
                            supported_curves: vec![DWalletMPCNetworkKeyScheme::Secp256k1 as u32],
                            is_last,
                            rejected: false,
                            session_sequence_number: request_input.session_sequence_number,
                        },
                    )
                };

                let messages: Vec<_> = slices
                    .into_iter()
                    .map(
                        DWalletCheckpointMessageKind::RespondDWalletMPCNetworkReconfigurationOutput,
                    )
                    .collect();
                Ok(messages)
            }
            MPCRequestInput::MakeDWalletUserSecretKeySharesPublicRequest(request_input) => {
                let SessionType::User = request_input.session_type else {
                    unreachable!(
                        "MakeDWalletUserSecretKeySharesPublic round should be a user session"
                    );
                };
                let tx = DWalletCheckpointMessageKind::RespondMakeDWalletUserSecretKeySharesPublic(
                    MakeDWalletUserSecretKeySharesPublicOutput {
                        dwallet_id: request_input.event_data.dwallet_id.to_vec(),
                        public_user_secret_key_shares: request_input
                            .event_data
                            .public_user_secret_key_shares
                            .clone(),
                        rejected: is_rejected,
                        session_sequence_number: request_input.session_sequence_number,
                    },
                );
                Ok(vec![tx])
            }
            MPCRequestInput::DWalletImportedKeyVerificationRequest(request_input) => {
                let SessionType::User = request_input.session_type else {
                    unreachable!(
                        "MakeDWalletUserSecretKeySharesPublic round should be a user session"
                    );
                };
                let tx = DWalletCheckpointMessageKind::RespondDWalletImportedKeyVerificationOutput(
                    DWalletImportedKeyVerificationOutput {
                        dwallet_id: request_input.event_data.dwallet_id.to_vec().clone(),
                        public_output: output,
                        encrypted_user_secret_key_share_id: request_input
                            .event_data
                            .encrypted_user_secret_key_share_id
                            .to_vec()
                            .clone(),
                        rejected: is_rejected,
                        session_sequence_number: request_input.session_sequence_number,
                    },
                );
                Ok(vec![tx])
            }
        }
    }

    /// Break down the key to slices because of chain transaction size limits.
    /// Limit 16 KB per Tx `pure` argument.
    fn slice_public_output_into_messages<T>(
        public_output: Vec<u8>,
        func: impl Fn(Vec<u8>, bool) -> T,
    ) -> Vec<T> {
        let mut slices = Vec::new();
        // We set a total of 5 KB since we need 6 KB buffer for other params.

        let public_chunks = public_output.chunks(FIVE_KILO_BYTES).collect_vec();
        let empty: &[u8] = &[];
        // Take the max of the two lengths to ensure we have enough slices.
        for i in 0..public_chunks.len() {
            // If the chunk is missing, use an empty slice, as the size of the slices can be different.
            let public_chunk = public_chunks.get(i).unwrap_or(&empty);
            slices.push(func(public_chunk.to_vec(), i == public_chunks.len() - 1));
        }
        slices
    }
}
