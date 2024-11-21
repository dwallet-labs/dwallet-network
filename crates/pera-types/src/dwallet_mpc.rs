use move_core_types::{ident_str, identifier::IdentStr};

/// Name of the Move module for `dwallet_2pc_mpc_ecdsa_k1`.
pub const DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME: &IdentStr = ident_str!("dwallet_2pc_mpc_ecdsa_k1");

/// Name of the Move struct for the `StartDKGFirstRoundEvent`.
pub const START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME: &IdentStr = ident_str!("StartDKGFirstRoundEvent");

/// Name of the Move struct for the `StartDKGSecondRoundEvent`.
pub const START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME: &IdentStr = ident_str!("StartDKGSecondRoundEvent");

/// Alias for an MPC message.
pub type MPCMessage = Vec<u8>;

/// Alias for an MPC output.
pub type MPCOutput = Vec<u8>;

