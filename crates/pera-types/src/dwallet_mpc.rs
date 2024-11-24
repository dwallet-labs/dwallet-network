use std::fmt;
use move_core_types::{ident_str, identifier::IdentStr};

/// Name of the Move module for `dwallet_2pc_mpc_ecdsa_k1`.
pub const DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME: &IdentStr = ident_str!("dwallet_2pc_mpc_ecdsa_k1");

/// Name of the Move struct for the `StartDKGFirstRoundEvent`.
pub const START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartDKGFirstRoundEvent");

/// Name of the Move struct for the `StartDKGSecondRoundEvent`.
pub const START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartDKGSecondRoundEvent");

/// Alias for an MPC message.
pub type MPCMessage = Vec<u8>;

/// Alias for an MPC output.
pub type MPCOutput = Vec<u8>;

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
/// - `Finalizing`:
///   The session has completed execution and is awaiting processing in the Move VM.
///   Once an output is received, it will be verified against the local result.
///   If they match, the status transitions to `Finished`.
///   This prevents the same output from being written to the chain multiple times.
///
/// - `Finished`:
///   The session has been removed from the active instances.
///   Incoming messages are no longer forwarded to the session,
///   but they are not flagged as malicious.
#[derive(Clone, PartialEq, Debug)]
pub enum MPCSessionStatus {
    Pending,
    FirstExecution,
    Active,
    Finalizing(MPCOutput),
    Finished(MPCOutput),
}

impl fmt::Display for MPCSessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MPCSessionStatus::Pending => write!(f, "Pending"),
            MPCSessionStatus::FirstExecution => write!(f, "FirstExecution"),
            MPCSessionStatus::Active => write!(f, "Active"),
            MPCSessionStatus::Finalizing(output) => write!(f, "Finalizing({:?})", output),
            MPCSessionStatus::Finished(output) => write!(f, "Finished({:?})", output),
        }
    }
}
