use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use pera_types::dwallet_mpc::DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME;
use pera_types::{
    base_types::{ObjectID, PeraAddress},
    id::ID,
    PERA_SYSTEM_ADDRESS,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const CREATED_DKG_FIRST_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("CreatedDKGSessionEvent");
pub const COMPLETED_DKG_FIRST_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("CompletedDKGRoundEvent");
pub const CREATED_DKG_SECOND_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartDKGSecondRoundEvent");
pub const COMPLETED_DKG_SECOND_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("CompletedSecondDKGRoundEvent");

/// Rust version of the Move [`pera_system::dwallet::CreatedDKGSessionEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CreatedDKGFirstRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// The address of the user that initiated this session.
    pub sender: PeraAddress,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
}

impl CreatedDKGFirstRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`CreatedDKGFirstRoundEvent`] events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: CREATED_DKG_FIRST_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust version of the Move [`pera_system::dwallet::CompletedProofMPCSessionEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CompletedProofMPCSessionEvent {
    /// Unique identifier for the MPC session.
    session_id: ID,
    /// The address of the user that initiated this session.
    sender: PeraAddress,
}

/// Rust version of the Move [`pera_system::dwallet::CompletedDKGRoundEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CompletedDKGFirstRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// The address of the user that initiated this session.
    pub sender: PeraAddress,
}

impl CompletedDKGFirstRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`CompletedDKGFirstRoundEvent`] events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: COMPLETED_DKG_FIRST_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust version of the Move [`pera_system::dwallet::StartDKGSecondRoundEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartDKGSecondRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// The address of the user that initiated this session.
    pub sender: PeraAddress,
    /// The DKG first decentralized round output
    pub first_round_output: Vec<u8>,
    /// The DKG centralized round output
    pub public_key_share_and_proof: Vec<u8>,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
    /// Unique identifier for the first DKG round session.
    pub first_round_session_id: ID,
}

impl StartDKGSecondRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartDKGSecondRoundEvent`] events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: CREATED_DKG_SECOND_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust version of the Move [`pera_system::dwallet::CompletedDKGSecondRoundEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CompletedDKGSecondRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// The address of the user that initiated this session.
    pub sender: PeraAddress,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
    /// The `DWallet` object's ID
    pub dwallet_id: ID,
    /// The DKG second round output
    pub value: Vec<u8>,
}

impl CompletedDKGSecondRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`CompletedDKGSecondRoundEvent`] events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: COMPLETED_DKG_SECOND_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}
