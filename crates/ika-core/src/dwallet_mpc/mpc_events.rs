//! Rust representations of MPC Events.
//!
//! These structs allow Rust programs to interact with on-chain events emitted during the
//! dWallet MPC process in the `ika_system::dwallet` Move module.
//! They include utility functions for detecting and comparing the event types.

use dwallet_mpc_types::dwallet_mpc::{
    DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME, DWALLET_MODULE_NAME,
    LOCKED_NEXT_COMMITTEE_EVENT_STRUCT_NAME, START_BATCHED_PRESIGN_EVENT_STRUCT_NAME,
    START_BATCHED_SIGN_EVENT_STRUCT_NAME, START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME,
    START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME, START_NETWORK_DKG_EVENT_STRUCT_NAME,
    START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME, START_PRESIGN_SECOND_ROUND_EVENT_STRUCT_NAME,
    START_SIGN_ROUND_EVENT_STRUCT_NAME, VALIDATOR_DATA_FOR_SECRET_SHARE_STRUCT_NAME,
    VALIDATOR_SET_MODULE_NAME,
};
use move_core_types::ident_str;
use move_core_types::language_storage::{StructTag, TypeTag};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sui_types::{base_types::SuiAddress, id::ID, SUI_SYSTEM_ADDRESS};

/// Represents the Rust version of the Move struct `ika_system::dwallet::StartDKGFirstRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartDKGFirstRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// The address of the user that initiated this session.
    pub initiator: SuiAddress,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
}

impl StartDKGFirstRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartDKGFirstRoundEvent`] events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Represents the Rust version of the Move struct `ika_system::dwallet_2pc_mpc_ecdsa_k1::StartPresignFirstRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartPresignFirstRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// The address of the user that initiated this session.
    pub initiator: SuiAddress,
    /// The `DWallet` object's ID associated with the DKG output.
    pub dwallet_id: ID,
    /// The DKG decentralized final output to use for the presign session.
    pub dkg_output: Vec<u8>,
    pub batch_session_id: ID,
    /// The dWallet mpc network key version
    pub(super) dwallet_mpc_network_key_version: u8,
}

impl StartPresignFirstRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartPresignFirstRoundEvent`] events
    /// from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Represents the Rust version of the Move
/// struct `ika_system::dwallet_2pc_mpc_ecdsa_k1::StartPresignSecondRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartPresignSecondRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ID,
    /// The address of the user that initiated this session.
    pub initiator: SuiAddress,
    /// The `DWallet` object's ID associated with the DKG output.
    pub dwallet_id: ID,
    /// The DKG decentralized final output to use for the presign session.
    pub dkg_output: Vec<u8>,
    /// Presign first round output.
    pub first_round_output: Vec<u8>,
    /// A unique identifier for the first Presign round session.
    pub first_round_session_id: ID,
    pub batch_session_id: ID,
    /// The dWallet mpc network key version
    pub(super) dwallet_mpc_network_key_version: u8,
}

/// An event to start a batched sign session, i.e.,
/// a sign session that signs on multiple messages simultaneously.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartBatchedSignEvent {
    pub session_id: ID,
    /// An ordered list without duplicates of the messages we need to sign on.
    pub hashed_messages: Vec<Vec<u8>>,
    pub initiator: SuiAddress,
}

/// A representation of the Move event to start a batched presign session, i.e.,
/// a presign session that creates multiple presigns at once.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartBatchedPresignEvent {
    pub session_id: ID,
    pub batch_size: u64,
    pub initiator: SuiAddress,
}

impl StartPresignSecondRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartPresignSecondRoundEvent`]
    /// events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: START_PRESIGN_SECOND_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Represents the Rust version of the Move
/// struct `ika_system::dwallet::StartSignEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartSignEvent<D> {
    /// Unique identifier for the MPC session.
    pub(super) session_id: ID,
    /// The address of the user that initiated this session.
    pub(super) initiator: SuiAddress,
    /// The ID of the batch sign session that contains this sign session.
    /// The output of this session will be written to the chain only once,
    /// along with the entire batch.
    pub(super) batched_session_id: ID,
    /// The `DWallet` object's ID associated with the DKG output.
    pub(super) dwallet_id: ID,
    /// The public output of the decentralized party in the dWallet DKG process.
    pub(super) dwallet_decentralized_public_output: Vec<u8>,
    /// Hashed messages to Sign.
    pub(super) hashed_message: Vec<u8>,
    /// The dWallet mpc network key version
    pub(super) dwallet_mpc_network_key_version: u8,
    /// The type of data that can be stored with the object.
    /// Specific to each Digital Signature Algorithm.
    pub(crate) signature_algorithm_data: D,
    /// Indicates whether the future sign feature was used to start the session.
    pub(crate) is_future_sign: bool,
}

impl<D> StartSignEvent<D> {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartSignEvent`]
    /// events from the chain and initiate the MPC session.
    pub fn type_(type_param: TypeTag) -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: START_SIGN_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![type_param],
        }
    }
}

impl StartBatchedSignEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartBatchedSignEvent`]
    /// events from the chain and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: START_BATCHED_SIGN_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

impl StartBatchedPresignEvent {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: START_BATCHED_PRESIGN_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust version of the Move [`ika_system::validator_set::LockedNextEpochCommitteeEvent`] type.
pub struct LockedNextEpochCommitteeEvent {
    epoch: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
struct ValidatorDataForDWalletSecretReShare {
    cg_pubkey_and_proof: Vec<u8>,
    protocol_pubkey_bytes: Vec<u8>,
}

/// The data we need to know about a validator to run a re-share/network-dkg flow with it.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct ValidatorDataForNetworkDKG {
    /// The class groups encryption key of the validator,
    /// used to encrypt the validator's secret share to it.
    pub(crate) cg_pubkey_and_proof: Vec<u8>,
    /// The Ika public key of the validator, used as an identifier for the validator.
    pub(crate) protocol_pubkey_bytes: Vec<u8>,
}

impl ValidatorDataForNetworkDKG {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: VALIDATOR_DATA_FOR_SECRET_SHARE_STRUCT_NAME.to_owned(),
            module: VALIDATOR_SET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

impl LockedNextEpochCommitteeEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`LockedNextEpochCommitteeEvent`] events from the chain and trigger the
    /// start of the chain's re-share flow.
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: LOCKED_NEXT_COMMITTEE_EVENT_STRUCT_NAME.to_owned(),
            module: VALIDATOR_SET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust version of the Move [`ika_system::dwallet_network_key::StartNetworkDKGEvent`] type.
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
            address: SUI_SYSTEM_ADDRESS,
            name: START_NETWORK_DKG_EVENT_STRUCT_NAME.to_owned(),
            module: ident_str!("dwallet_network_key").to_owned(),
            type_params: vec![],
        }
    }
}
