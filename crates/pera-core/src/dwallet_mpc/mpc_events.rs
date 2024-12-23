//! Rust representations of MPC Events.
//!
//! These structs allow Rust programs to interact with on-chain events emitted during the
//! dWallet MPC process in the `pera_system::dwallet` Move module.
//! They include utility functions for detecting and comparing the event types.

use dwallet_mpc_types::dwallet_mpc::{
    DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME, LOCKED_NEXT_COMMITTEE_EVENT_STRUCT_NAME,
    START_BATCHED_PRESIGN_EVENT_STRUCT_NAME, START_BATCHED_SIGN_EVENT_STRUCT_NAME,
    START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME, START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME,
    START_NETWORK_DKG_EVENT_STRUCT_NAME,
    START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME, START_PRESIGN_SECOND_ROUND_EVENT_STRUCT_NAME,
    START_SIGN_ROUND_EVENT_STRUCT_NAME,
};
use move_core_types::ident_str;
use move_core_types::language_storage::StructTag;
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
    /// The DKG decentralized final output to use for the presign session.
    pub dkg_output: Vec<u8>,
    pub batch_session_id: ID,
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
    /// The DKG decentralized final output to use for the presign session.
    pub dkg_output: Vec<u8>,
    /// Presign first round output.
    pub first_round_output: Vec<u8>,
    /// A unique identifier for the first Presign round session.
    pub first_round_session_id: ID,
    pub batch_session_id: ID,
}

/// An event to start a batched sign session, i.e.,
/// a sign session that signs on multiple messages simultaneously.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartBatchedSignEvent {
    pub session_id: ID,
    /// An ordered list without duplicates of the messages we need to sign on.
    pub hashed_messages: Vec<Vec<u8>>,
    pub initiator: PeraAddress,
}

/// A representation of the Move event to start a batched presign session, i.e.,
/// a presign session that creates multiple presigns at once.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartBatchedPresignEvent {
    pub session_id: ID,
    pub batch_size: u64,
    pub initiator: PeraAddress,
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

// todo(zeev): check this.
/// Represents the Rust version of the Move
/// struct `pera_system::dwallet::StartSignRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartSignRoundEvent {
    /// Unique identifier for the MPC session.
    pub(super) session_id: ID,
    /// Unique identifier for the MPC session.
    pub(super) presign_session_id: ID,
    /// The address of the user that initiated this session.
    pub(super) initiator: PeraAddress,
    /// The ID of the batch sign session that contains this sign session.
    /// The output of this session will be written to the chain only once,
    /// along with the entire batch.
    pub(super) batched_session_id: ID,
    /// The `DWallet` object's ID associated with the DKG output.
    pub(super) dwallet_id: ID,
    /// The DKG decentralized final output to use for the presign session.
    pub(super) dkg_output: Vec<u8>,
    /// Hashed messages to Sign.
    pub(super) hashed_message: Vec<u8>,
    /// The serialized final presign output, constructed from the outputs of
    /// both the first and second presign MPC rounds.
    pub(super) presign: Vec<u8>,
    /// Centralized signed message
    pub(super) centralized_signed_message: Vec<u8>,
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

impl StartBatchedSignEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartBatchedSignEvent`]
    /// events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: START_BATCHED_SIGN_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

impl StartBatchedPresignEvent {
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: START_BATCHED_PRESIGN_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust version of the Move [`pera_system::validator_set::LockedNextEpochCommitteeEvent`] type.
pub struct LockedNextEpochCommitteeEvent {
    next_committee_validators: Vec<ValidatorDataForDWalletSecretReShare>,
    epoch: u64,
}

struct ValidatorDataForDWalletSecretReShare {
    class_groups_public_key_and_proof_bytes: Vec<u8>,
    protocol_pubkey_bytes: Vec<u8>,
}

impl LockedNextEpochCommitteeEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`LockedNextEpochCommitteeEvent`] events from the chain and trigger the
    /// start of the chain's re-share flow.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: LOCKED_NEXT_COMMITTEE_EVENT_STRUCT_NAME.to_owned(),
            module: ident_str!("validator_set").to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust version of the Move [`pera_system::dwallet_network_key::StartNetworkDKGEvent`] type.
/// It is used to trigger the start of the network DKG process.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartNetworkDKGEvent {
    pub(crate) session_id: ID,
    pub(crate) key_scheme: u8,
}

impl StartNetworkDKGEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartNetworkDKGEvent`] events from the chain and initiate the MPC session.
    /// It is used to trigger the start of the network DKG process.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: START_NETWORK_DKG_EVENT_STRUCT_NAME.to_owned(),
            module: ident_str!("dwallet_network_key").to_owned(),
            type_params: vec![],
        }
    }
}
