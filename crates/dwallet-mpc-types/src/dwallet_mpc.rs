use move_core_types::{ident_str, identifier::IdentStr};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use group::PartyID;
use thiserror::Error;

pub const DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME: &IdentStr = ident_str!("dwallet_2pc_mpc_ecdsa_k1");
pub const VALIDATOR_SET_MODULE_NAME: &IdentStr = ident_str!("validator_set");
pub const START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartDKGFirstRoundEvent");
pub const START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartDKGSecondRoundEvent");
pub const START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartPresignFirstRoundEvent");
pub const START_PRESIGN_SECOND_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartPresignSecondRoundEvent");
pub const START_SIGN_ROUND_EVENT_STRUCT_NAME: &IdentStr = ident_str!("StartSignEvent");
pub const START_BATCHED_SIGN_EVENT_STRUCT_NAME: &IdentStr = ident_str!("StartBatchedSignEvent");
pub const START_BATCHED_PRESIGN_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartBatchedPresignEvent");
pub const LOCKED_NEXT_COMMITTEE_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("LockedNextEpochCommitteeEvent");
pub const VALIDATOR_DATA_FOR_SECRET_SHARE_STRUCT_NAME: &IdentStr =
    ident_str!("ValidatorDataForDWalletSecretShare");
pub const START_NETWORK_DKG_EVENT_STRUCT_NAME: &IdentStr = ident_str!("StartNetworkDKGEvent");

/// Alias for an MPC message.
pub type MPCMessage = Vec<u8>;

/// Alias for an MPC public output.
pub type MPCPublicOutput = Vec<u8>;

/// Alias for an MPC private output.
#[derive(Clone, Debug, PartialEq)]
pub enum MPCPrivateOutput {
    None,
    DecryptionKeyShare(HashMap<PartyID, Vec<u8>>),
}
// pub type MPCPrivateOutput = PrivateOutput;

/// Alias for MPC public input.
pub type MPCPublicInput = Vec<u8>;

pub type MPCRound = usize;

/// Possible statuses of an MPC Session:
///
/// - `Pending`:
///   The instance is queued because the maximum number of active MPC instances
///   [`DWalletMPCManager::max_active_mpc_instances`] has been reached.
///   It is waiting for active instances to complete before activation.
///
/// - `FirstExecution`:
///   Indicates that the [`DWalletMPCInstance::party`] has not yet performed its
///   first advance.
///   This status ensures these instances can be filtered and
///   advanced, even if they have not received the `threshold_number_of_parties`
///   messages.
///
/// - `Active`:
///   The session is currently running, and new messages are forwarded to it
///   for processing.
///
/// - `Finished`:
///   The session has been removed from the active instances.
///   Incoming messages are no longer forwarded to the session,
///   but they are not flagged as malicious.
/// - `Failed`:
///   The session has failed due to an unrecoverable error.
///   This status indicates that the session cannot proceed further.
#[derive(Clone, PartialEq, Debug)]
pub enum MPCSessionStatus {
    Pending,
    FirstExecution,
    Active(MPCRound),
    Finished(MPCPublicOutput, MPCPrivateOutput),
    Failed,
}

impl fmt::Display for MPCSessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MPCSessionStatus::Pending => write!(f, "Pending"),
            MPCSessionStatus::FirstExecution => write!(f, "FirstExecution"),
            MPCSessionStatus::Active(round) => write!(f, "Active - round {}", round),
            MPCSessionStatus::Finished(public_output, private_output) => {
                write!(f, "Finished({:?} {:?})", public_output, private_output)
            }
            MPCSessionStatus::Failed => write!(f, "Failed"),
        }
    }
}

/// Rust representation of the move struct `NetworkDecryptionKeyShares`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, Hash)]
pub struct NetworkDecryptionKeyShares {
    pub epoch: u64,
    pub current_epoch_shares: Vec<Vec<u8>>,
    pub previous_epoch_shares: Vec<Vec<u8>>,
    pub protocol_public_parameters: Vec<u8>,
    pub decryption_public_parameters: Vec<u8>,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash, Copy)]
pub enum DWalletMPCNetworkKeyScheme {
    Secp256k1 = 1,
    Ristretto = 2,
}

// We can't import pera-types here since we import this module in there.
// Therefor we use `thiserror` `#from` to convert this error.
#[derive(Debug, Error)]
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
