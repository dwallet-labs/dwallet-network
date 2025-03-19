//! Rust representations of MPC Events.
//!
//! These structs allow Rust programs to interact with on-chain events emitted during the
//! dWallet MPC process in the `ika_system::dwallet` Move module.
//! They include utility functions for detecting and comparing the event types.

use crate::dwallet_mpc::network_dkg::network_dkg_session_info;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME, DWALLET_MODULE_NAME,
    DWALLET_MPC_EVENT_STRUCT_NAME, LOCKED_NEXT_COMMITTEE_EVENT_STRUCT_NAME,
    START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME, START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME,
    START_NETWORK_DKG_EVENT_STRUCT_NAME, START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME,
    START_PRESIGN_SECOND_ROUND_EVENT_STRUCT_NAME, START_SIGN_ROUND_EVENT_STRUCT_NAME,
    VALIDATOR_DATA_FOR_SECRET_SHARE_STRUCT_NAME, VALIDATOR_SET_MODULE_NAME,
};
use ika_types::dwallet_mpc_error::DwalletMPCError;
use ika_types::messages_dwallet_mpc::{
    DWalletMPCEventTrait, DWalletMPCSuiEvent, IkaPackagesConfig, MPCProtocolInitData, SessionInfo,
};
use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use move_core_types::language_storage::{StructTag, TypeTag};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sui_types::base_types::ObjectID;
use sui_types::{base_types::SuiAddress, id::ID, SUI_SYSTEM_ADDRESS};

/// Rust version of the Move [`ika_system::validator_set::LockedNextEpochCommitteeEvent`] type.
pub struct LockedNextEpochCommitteeEvent {
    epoch: u64,
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
