use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use pera_types::dwallet_mpc::DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME;
use pera_types::{base_types::PeraAddress, id::ID, PERA_SYSTEM_ADDRESS};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartDKGSecondRoundEvent");
pub const START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartDKGFirstRoundEvent");
pub const START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartPresignFirstRoundEvent");
pub const START_PRESIGN_SECOND_ROUND_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("StartPresignSecondRoundEvent");
pub const START_SIGN_ROUND_EVENT_STRUCT_NAME: &IdentStr = ident_str!("StartSignEvent");
pub const START_BATCHED_SIGN_EVENT_STRUCT_NAME: &IdentStr = ident_str!("StartBatchedSignEvent");

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

/// Rust version of the Move [`pera_system::dwallet::StartPresignFirstRoundEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartPresignFirstRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// The address of the user that initiated this session.
    pub sender: PeraAddress,
    /// The `DWallet` object's ID associated with the dkg output.
    pub dwallet_id: ID,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
    /// The DKG decentralized final output to use for the presign session.
    pub dkg_output: Vec<u8>,
}

impl StartPresignFirstRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartPresignFirstRoundEvent`] events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust version of the Move [`pera_system::dwallet::StartPresignFirstRoundEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartPresignSecondRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// The address of the user that initiated this session.
    pub sender: PeraAddress,
    /// The `DWallet` object's ID associated with the dkg output.
    pub dwallet_id: ID,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
    /// The DKG decentralized final output to use for the presign session.
    pub dkg_output: Vec<u8>,
    /// Presign first round output
    pub first_round_output: Vec<u8>,
    /// Unique identifier for the first Presign round session.
    pub first_round_session_id: ID,
}

impl StartPresignSecondRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartPresignSecondRoundEvent`] events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: START_PRESIGN_SECOND_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust version of the Move [`pera_system::dwallet::StartSignEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartSignRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// Unique identifier for the MPC session.
    pub presign_session_id: ID,
    /// The address of the user that initiated this session.
    pub sender: PeraAddress,
    /// The ID of the batch sign session that contains this sign session.
    /// The output of this session will be written to the chain only once, along with the entire batch.
    pub batched_session_id: ID,
    /// The `DWallet` object's ID associated with the dkg output.
    pub dwallet_id: ID,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
    /// The DKG decentralized final output to use for the presign session.
    pub dkg_output: Vec<u8>,
    /// Hashed messages to sign on
    pub hashed_message: Vec<u8>,
    /// Presign first round output, required for the MPC Sign session
    pub presign_first_round_output: Vec<u8>,
    /// Presign second round output, required for the MPC Sign session
    pub presign_second_round_output: Vec<u8>,

    /// Centralized signed message
    pub centralized_signed_message: Vec<u8>,
}

impl StartSignRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartSignRoundEvent`] events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: START_SIGN_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// An event to start a batched sign session, i.e. a sign session that signs on multiple messages simultaneously.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartBatchedSignEvent {
    pub session_id: ID,
    /// An ordered list without duplicates of the messages we need to sign on.
    pub hashed_messages: Vec<Vec<u8>>,
    initiating_user: PeraAddress,
}

impl StartBatchedSignEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartBatchedSignEvent`] events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: START_BATCHED_SIGN_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}
