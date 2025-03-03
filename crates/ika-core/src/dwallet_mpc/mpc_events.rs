//! Rust representations of MPC Events.
//!
//! These structs allow Rust programs to interact with on-chain events emitted during the
//! dWallet MPC process in the `ika_system::dwallet` Move module.
//! They include utility functions for detecting and comparing the event types.

use crate::dwallet_mpc::network_dkg::network_dkg_session_info;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME, DWALLET_MODULE_NAME,
    DWALLET_MPC_EVENT_STRUCT_NAME, LOCKED_NEXT_COMMITTEE_EVENT_STRUCT_NAME,
    START_BATCHED_PRESIGN_EVENT_STRUCT_NAME, START_BATCHED_SIGN_EVENT_STRUCT_NAME,
    START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME, START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME,
    START_NETWORK_DKG_EVENT_STRUCT_NAME, START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME,
    START_PRESIGN_SECOND_ROUND_EVENT_STRUCT_NAME, START_SIGN_ROUND_EVENT_STRUCT_NAME,
    VALIDATOR_DATA_FOR_SECRET_SHARE_STRUCT_NAME, VALIDATOR_SET_MODULE_NAME,
};
use ika_types::dwallet_mpc_error::DwalletMPCError;
use ika_types::messages_dwallet_mpc::{
    DWalletMPCEventTrait, DWalletMPCSuiEvent, IkaPackagesConfig, MPCProtocolInitData, SessionInfo,
    SignData, SingleSignSessionData,
};
use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use move_core_types::language_storage::{StructTag, TypeTag};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sui_types::base_types::ObjectID;
use sui_types::{base_types::SuiAddress, id::ID, SUI_SYSTEM_ADDRESS};

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

/// Represents the Rust version of the Move
/// struct `ika_system::dwallet::StartSignEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartSignEvent<D> {
    pub sign_id: ID,
    /// The `DWallet` object's ID associated with the DKG output.
    pub(super) dwallet_id: ID,
    /// The public output of the decentralized party in the dWallet DKG process.
    pub(super) dwallet_decentralized_public_output: Vec<u8>,
    pub hash_scheme: u8,
    /// Hashed messages to Sign.
    pub(super) message: Vec<u8>,
    /// The dWallet mpc network key version
    pub(super) dwallet_mpc_network_key_id: ID,
    presign_id: ID,

    /// The presign protocol output as bytes.
    pub(crate) presign: Vec<u8>,

    /// The centralized party signature of a message.
    pub(crate) message_centralized_signature: Vec<u8>,

    /// Indicates whether the future sign feature was used to start the session.
    pub(crate) is_future_sign: bool,
}

impl<D: DWalletMPCEventTrait> DWalletMPCEventTrait for StartSignEvent<D> {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartSignEvent`]
    /// events from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: START_SIGN_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

impl DWalletMPCEventTrait for StartBatchedSignEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartBatchedSignEvent`]
    /// events from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_package_id,
            name: START_BATCHED_SIGN_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

impl DWalletMPCEventTrait for StartBatchedPresignEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartBatchedPresignEvent`]
    /// events from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_package_id,
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

impl DWalletMPCEventTrait for ValidatorDataForNetworkDKG {
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_package_id,
            name: VALIDATOR_DATA_FOR_SECRET_SHARE_STRUCT_NAME.to_owned(),
            module: VALIDATOR_SET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

impl DWalletMPCEventTrait for LockedNextEpochCommitteeEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`LockedNextEpochCommitteeEvent`] events from the chain and trigger the
    /// start of the chain's re-share flow.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_package_id,
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

impl DWalletMPCEventTrait for StartNetworkDKGEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartNetworkDKGEvent`] events from the chain and initiate the MPC session.
    /// It is used to trigger the start of the network DKG process.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_package_id,
            name: START_NETWORK_DKG_EVENT_STRUCT_NAME.to_owned(),
            module: ident_str!("dwallet_network_key").to_owned(),
            type_params: vec![],
        }
    }
}
