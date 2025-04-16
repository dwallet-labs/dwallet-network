use move_core_types::{ident_str, identifier::IdentStr};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

// TODO (#650): Rename Move structs
pub const DWALLET_MPC_EVENT_STRUCT_NAME: &IdentStr = ident_str!("DWalletEvent");
pub const DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME: &IdentStr = ident_str!("dwallet_2pc_mpc_secp256k1");
pub const VALIDATOR_SET_MODULE_NAME: &IdentStr = ident_str!("validator_set");
/// There's a wrapper and inner struct to support Move upgradable contracts. Read this doc for further explanations:
/// https://docs.sui.io/concepts/sui-move-concepts/packages/upgrade.
pub const DWALLET_MODULE_NAME: &IdentStr = ident_str!("dwallet_2pc_mpc_secp256k1_inner");
pub const START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("DWalletDKGFirstRoundRequestEvent");
// TODO (#650): Rename Move structs
pub const START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("DWalletDKGSecondRoundRequestEvent");
// TODO (#650): Rename Move structs
pub const START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("ECDSAPresignRequestEvent");
pub const START_PRESIGN_SECOND_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartPresignSecondRoundEvent");
pub const START_SIGN_ROUND_EVENT_STRUCT_NAME: &IdentStr = ident_str!("ECDSASignRequestEvent");
pub const LOCKED_NEXT_COMMITTEE_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("LockedNextEpochCommitteeEvent");
pub const VALIDATOR_DATA_FOR_SECRET_SHARE_STRUCT_NAME: &IdentStr =
    ident_str!("ValidatorDataForDWalletSecretShare");
pub const START_NETWORK_DKG_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("DWalletNetworkDKGDecryptionKeyRequestEvent");

/// Alias for an MPC message.
pub type MPCMessage = Vec<u8>;

/// Represents a chunk of an MPC message, with metadata for reassembly.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MPCMessageSlice {
    /// A fragment of the original message.
    pub fragment: MPCMessage,
    /// The position of this chunk in the original message sequence.
    pub sequence_number: u64,
    /// Total number of chunks in the message, used only in the first slice.
    pub number_of_chunks: Option<usize>,
}

/// Represents the state of the message building process.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageState {
    /// All chunks received; full message reconstructed.
    Complete(MPCMessage),
    /// Still waiting on some chunks; maps sequence numbers to slices.
    Incomplete(HashMap<u64, MPCMessageSlice>),
}

/// Builds and reconstructs messages from incoming slices.
/// Some MPC messages might be greater than the maximum size of a consensus message limit.
/// The `MPCMessageBuilder` is used to split the message into smaller chunks,
/// to avoid exceeding the maximum size limit.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MPCMessageBuilder {
    /// Current state of the message.
    pub messages: MessageState,
}

impl MPCMessageBuilder {
    pub fn empty() -> Self {
        Self {
            messages: MessageState::Incomplete(HashMap::new()),
        }
    }

    /// Splits a message into smaller chunks and returns a new builder with those chunks.
    /// Ensures at least one chunk is created, even if the message is empty.
    pub fn split(message: Vec<u8>, chunk_size: usize) -> Self {
        let chunks: Vec<Vec<u8>> = if message.is_empty() {
            // Ensure at least one slice for empty messages.
            vec![vec![]]
        } else {
            message
                .chunks(chunk_size)
                .map(|chunk| chunk.to_vec())
                .collect()
        };

        let number_of_chunks = chunks.len();

        let messages = chunks
            .into_iter()
            .enumerate()
            .map(|(i, message)| {
                (
                    i as u64,
                    MPCMessageSlice {
                        fragment: message,
                        sequence_number: i as u64,
                        number_of_chunks: if i == 0 { Some(number_of_chunks) } else { None },
                    },
                )
            })
            .collect();

        Self {
            messages: MessageState::Incomplete(messages),
        }
    }

    /// Adds a message slice to the builder and attempts to complete the full message.
    /// If all slices are present, the message state is updated to `Complete`.
    pub fn add_and_try_complete(&mut self, message: MPCMessageSlice) {
        if let MessageState::Incomplete(messages) = &mut self.messages {
            messages.insert(message.sequence_number, message);

            if let Some(slice) = messages.get(&0) {
                if let Some(expected_chunks) = slice.number_of_chunks {
                    if messages.len() == expected_chunks {
                        let complete_message = (0..expected_chunks as u64)
                            .map(|i| messages.get(&i))
                            .collect::<Option<Vec<_>>>()
                            .map(|slices| {
                                slices
                                    .into_iter()
                                    .flat_map(|slice| slice.fragment.clone())
                                    .collect::<Vec<_>>()
                            });

                        if let Some(message) = complete_message {
                            self.messages = MessageState::Complete(message);
                        }
                    }
                }
            }
        }
    }
}

/// Alias for an MPC public output.
pub type MPCPublicOutput = Vec<u8>;

/// Alias for an MPC private output.
pub type MPCPrivateOutput = Vec<u8>;

/// Alias for MPC public input.
pub type MPCPublicInput = Vec<u8>;

/// Alias for MPC private input.
pub type MPCPrivateInput = Option<Vec<u8>>;

/// Possible statuses of an MPC Session:
///
/// - `Pending`:
///   The instance is queued because the maximum number of active MPC instances
///   [`DWalletMPCManager::max_active_mpc_instances`] has been reached.
///   It is waiting for active instances to complete before activation.
///
/// - `Active`:
///   The session is currently running, and new messages are forwarded to it
///   for processing.
///
/// - `Finished`:
///   The session has been removed from the active instances.
///   Incoming messages are no longer forwarded to the session,
///   but they are not flagged as malicious.
///
/// - `Failed`:
///   The session has failed due to an unrecoverable error.
///   This status indicates that the session cannot proceed further.
#[derive(Clone, PartialEq, Debug)]
pub enum MPCSessionStatus {
    Pending,
    Active,
    Finished,
    Failed,
}

impl fmt::Display for MPCSessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MPCSessionStatus::Pending => write!(f, "Pending"),
            MPCSessionStatus::Active => write!(f, "Active"),
            MPCSessionStatus::Finished => write!(f, "Finished"),
            MPCSessionStatus::Failed => write!(f, "Failed"),
        }
    }
}

/// Rust representation of the Move struct `NetworkDecryptionKeyShares`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, Hash)]
pub struct NetworkDecryptionKeyShares {
    /// The epoch of the last version update.
    pub epoch: u64,

    /// Decryption key shares for the current epoch.
    /// These keys together represent the network decryption key.
    /// Each key is encrypted with the class groups key of each validator.
    /// So only the validator can decrypt their own key.
    pub current_epoch_encryptions_of_shares_per_crt_prime: Vec<u8>,

    /// Decryption key shares for the previous epoch.
    /// Updated at the reconfiguration.
    pub previous_epoch_encryptions_of_shares_per_crt_prime: Vec<u8>,

    /// Public parameters from the network DKG, used to create the
    /// protocol public parameters.
    /// Updated only after a successful network DKG.
    pub encryption_scheme_public_parameters: Vec<u8>,

    /// The public parameters of the decryption key shares,
    /// updated only after a successful network DKG.
    pub decryption_key_share_public_parameters: Vec<u8>,

    /// The network encryption key, updated only after a successful network DKG.
    pub encryption_key: Vec<u8>,

    /// Validators' verification keys.
    pub public_verification_keys: Vec<u8>,
    pub setup_parameters_per_crt_prime: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct NetworkDecryptionKeyOnChainOutput {
    pub encryption_key: Vec<u8>,
    pub decryption_key_share_public_parameters: Vec<u8>,
    pub encryption_scheme_public_parameters: Vec<u8>,
    pub public_verification_keys: Vec<u8>,
    pub setup_parameters_per_crt_prime: Vec<u8>,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash, Copy)]
pub enum DWalletMPCNetworkKeyScheme {
    Secp256k1 = 1,
    Ristretto = 2,
}

// We can't import ika-types here since we import this module in there.
// Therefore, we use `thiserror` `#from` to convert this error.
#[derive(Debug, Error, Clone)]
pub enum DwalletNetworkMPCError {
    #[error("invalid DWalletMPCNetworkKey value: {0}")]
    InvalidDWalletMPCNetworkKey(u8),
}

impl TryFrom<u8> for DWalletMPCNetworkKeyScheme {
    type Error = DwalletNetworkMPCError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DWalletMPCNetworkKeyScheme::Secp256k1),
            2 => Ok(DWalletMPCNetworkKeyScheme::Ristretto),
            v => Err(DwalletNetworkMPCError::InvalidDWalletMPCNetworkKey(v)),
        }
    }
}

impl NetworkDecryptionKeyShares {
    pub fn get_on_chain_output(&self) -> NetworkDecryptionKeyOnChainOutput {
        NetworkDecryptionKeyOnChainOutput {
            encryption_key: self.encryption_key.clone(),
            decryption_key_share_public_parameters: self
                .decryption_key_share_public_parameters
                .clone(),
            encryption_scheme_public_parameters: self.encryption_scheme_public_parameters.clone(),
            public_verification_keys: self.public_verification_keys.clone(),
            setup_parameters_per_crt_prime: self.setup_parameters_per_crt_prime.clone(),
        }
    }
}

pub type ClassGroupsPublicKeyAndProofBytes = Vec<u8>;
