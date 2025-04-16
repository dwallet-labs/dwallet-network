//! Rust representations of MPC Events.
//!
//! These structs allow Rust programs to interact with on-chain events emitted during the
//! dWallet MPC process in the `ika_system::dwallet` Move module.
//! They include utility functions for detecting and comparing the event types.

use dwallet_mpc_types::dwallet_mpc::{
    LOCKED_NEXT_COMMITTEE_EVENT_STRUCT_NAME, VALIDATOR_SET_MODULE_NAME,
};
use ika_types::messages_dwallet_mpc::{DWalletMPCEventTrait, IkaPackagesConfig};
use move_core_types::language_storage::StructTag;

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
