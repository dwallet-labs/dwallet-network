//! Rust representations of MPC Events.
//!
//! These structs allow Rust programs to interact with on-chain events emitted during the
//! dWallet MPC process in the `pera_system::dwallet` Move module.
//! They include utility functions for detecting and comparing the event types.
use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use pera_mpc_types::dwallet_mpc::{
    DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME, START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME,
    START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME, START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME,
    START_PRESIGN_SECOND_ROUND_EVENT_STRUCT_NAME, START_SIGN_ROUND_EVENT_STRUCT_NAME,
};
use pera_types::{base_types::PeraAddress, id::ID, PERA_SYSTEM_ADDRESS};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents the Rust version of the Move struct `pera_system::dwallet::StartDKGFirstRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartDKGFirstRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// The address of the user that initiated this session.
    pub initiator: PeraAddress,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
}

impl StartDKGFirstRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartDKGFirstRoundEvent`] events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Represents the Rust version of the Move struct `pera_system::dwallet::StartDKGSecondRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartDKGSecondRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: PeraAddress,
    /// The address of the user that initiated this session.
    pub initiator: PeraAddress,
    /// The DKG first decentralized round output.
    pub first_round_output: Vec<u8>,
    /// The DKG centralized round output.
    pub public_key_share_and_proof: Vec<u8>,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
    /// The unique identifier for the first DKG round session.
    pub first_round_session_id: ID,
}

impl StartDKGSecondRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartDKGSecondRoundEvent`] events from the chain
    /// and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Represents the Rust version of the Move struct `pera_system::dwallet::StartPresignFirstRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartPresignFirstRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// The address of the user that initiated this session.
    pub initiator: PeraAddress,
    /// The `DWallet` object's ID associated with the DKG output.
    pub dwallet_id: ID,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
    /// The DKG decentralized final output to use for the presign session.
    pub dkg_output: Vec<u8>,
}

impl StartPresignFirstRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartPresignFirstRoundEvent`] events
    /// from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Represents the Rust version of the Move
/// struct `pera_system::dwallet::StartPresignSecondRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartPresignSecondRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// The address of the user that initiated this session.
    pub initiator: PeraAddress,
    /// The `DWallet` object's ID associated with the DKG output.
    pub dwallet_id: ID,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
    /// The DKG decentralized final output to use for the presign session.
    pub dkg_output: Vec<u8>,
    /// Presign first round output.
    pub first_round_output: Vec<u8>,
    /// A unique identifier for the first Presign round session.
    pub first_round_session_id: ID,
}

impl StartPresignSecondRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartPresignSecondRoundEvent`]
    /// events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: START_PRESIGN_SECOND_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Represents the Rust version of the Move
/// struct `pera_system::dwallet::StartSignRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartSignRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// Unique identifier for the MPC session.
    pub presign_session_id: ID,
    /// The address of the user that initiated this session.
    pub initiator: PeraAddress,
    /// The `DWallet` object's ID associated with the DKG output.
    pub dwallet_id: ID,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
    /// The DKG decentralized final output to use for the presign session.
    pub dkg_output: Vec<u8>,
    /// Hashed messages to sign.
    pub hashed_message: Vec<u8>,
    /// Presign first round output, required for the MPC Sign session.
    pub presign_first_round_output: Vec<u8>,
    /// Presign second round output, required for the MPC Sign session.
    pub presign_second_round_output: Vec<u8>,
    /// Centralized signed message
    pub centralized_signed_message: Vec<u8>,
}

impl StartSignRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartSignRoundEvent`]
    /// events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: START_SIGN_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}
