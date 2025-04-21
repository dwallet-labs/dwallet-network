use move_core_types::{ident_str, identifier::IdentStr};
use serde::{Deserialize, Serialize};
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
    Active,
    Finished,
    Failed,
}

impl fmt::Display for MPCSessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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

pub type ClassGroupsPublicKeyAndProofBytes = Vec<u8>;
