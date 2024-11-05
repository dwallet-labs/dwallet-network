use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use pera_types::base_types::ObjectID;
use pera_types::dwallet_mpc::DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME;
use pera_types::{base_types::PeraAddress, id::ID, PERA_SYSTEM_ADDRESS};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartDKGSecondRoundEvent");
pub const START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartDKGFirstRoundEvent");

/// Rust version of the Move [`pera_system::dwallet::StartDKGFirstRoundEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartDKGFirstRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// The address of the user that initiated this session.
    pub sender: PeraAddress,
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

/// Rust version of the Move [`pera_system::dwallet::StartDKGSecondRoundEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartDKGSecondRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: PeraAddress,
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
            name: START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}
