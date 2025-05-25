use move_core_types::{ident_str, identifier::IdentStr};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

// TODO (#650): Rename Move structs
pub const DWALLET_MPC_EVENT_STRUCT_NAME: &IdentStr = ident_str!("DWalletEvent");
pub const DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME: &IdentStr =
    ident_str!("dwallet_2pc_mpc_coordinator");
pub const VALIDATOR_SET_MODULE_NAME: &IdentStr = ident_str!("validator_set");
/// There's a wrapper and inner struct to support Move upgradable contracts. Read this doc for further explanations:
/// https://docs.sui.io/concepts/sui-move-concepts/packages/upgrade.
pub const DWALLET_MODULE_NAME: &IdentStr = ident_str!("dwallet_2pc_mpc_coordinator_inner");
pub const DWALLET_DKG_FIRST_ROUND_REQUEST_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("DWalletDKGFirstRoundRequestEvent");
pub const DWALLET_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_REQUEST_EVENT: &IdentStr =
    ident_str!("MakeDWalletUserSecretKeySharePublicRequestEvent");
pub const DWALLET_IMPORTED_KEY_VERIFICATION_REQUEST_EVENT: &IdentStr =
    ident_str!("DWalletImportedKeyVerificationRequestEvent");
// TODO (#650): Rename Move structs
pub const DWALLET_DKG_SECOND_ROUND_REQUEST_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("DWalletDKGSecondRoundRequestEvent");
// TODO (#650): Rename Move structs
pub const PRESIGN_REQUEST_EVENT_STRUCT_NAME: &IdentStr = ident_str!("PresignRequestEvent");
pub const SIGN_REQUEST_EVENT_STRUCT_NAME: &IdentStr = ident_str!("SignRequestEvent");
pub const LOCKED_NEXT_COMMITTEE_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("LockedNextEpochCommitteeEvent");
pub const VALIDATOR_DATA_FOR_SECRET_SHARE_STRUCT_NAME: &IdentStr =
    ident_str!("ValidatorDataForDWalletSecretShare");
pub const START_NETWORK_DKG_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("DWalletNetworkDKGEncryptionKeyRequestEvent");

/// Alias for an MPC message.
pub type MPCMessage = Vec<u8>;

/// MPC session public output sent through the consensus.
/// Used to indicate the session status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MPCSessionPublicOutput {
    CompletedSuccessfully(SerializedWrappedMPCPublicOutput),
    SessionFailed,
}

/// Alias for an MPC public output wrapped with version.
pub type SerializedWrappedMPCPublicOutput = Vec<u8>;

/// The MPC Public Output for Class Groups based protocols.
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, Hash, PartialOrd,
)]
pub enum MPCPublicOutput {
    /// Serialized Public Output.
    V1(Vec<u8>),
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, Hash)]
pub enum NetworkDecryptionKeyPublicOutputType {
    NetworkDkg,
    Reshare,
}

/// Network decryption key shares for the MPC protocol.
/// Created for each DKG protocol and modified for each Reshare Protocol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, Hash)]
pub struct NetworkDecryptionKeyPublicData {
    /// The epoch of the last version update.
    pub epoch: u64,

    pub state: NetworkDecryptionKeyPublicOutputType,
    /// The public output of the `latest` decryption key update (NetworkDKG/Reshare).
    pub latest_public_output: MPCPublicOutput,

    /// The public parameters of the decryption key shares,
    /// updated only after a successful network DKG or Reshare.
    pub decryption_key_share_public_parameters: Vec<u8>,

    /// The public output of the `NetworkDKG` process (the first and only one).
    /// On first instance it will be equal to `latest_public_output`.
    pub network_dkg_output: MPCPublicOutput,
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash, Copy)]
pub enum DWalletMPCNetworkKeyScheme {
    Secp256k1 = 0,
    Ristretto = 1,
}

// We can't import ika-types here since we import this module in there.
// Therefore, we use `thiserror` `#from` to convert this error.
#[derive(Debug, Error, Clone)]
pub enum DwalletNetworkMPCError {
    #[error("invalid DWalletMPCNetworkKey value: {0}")]
    InvalidDWalletMPCNetworkKey(u32),
}

impl TryFrom<u32> for DWalletMPCNetworkKeyScheme {
    type Error = DwalletNetworkMPCError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DWalletMPCNetworkKeyScheme::Secp256k1),
            1 => Ok(DWalletMPCNetworkKeyScheme::Ristretto),
            v => Err(DwalletNetworkMPCError::InvalidDWalletMPCNetworkKey(v)),
        }
    }
}

pub type ClassGroupsPublicKeyAndProofBytes = Vec<u8>;
