// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use std::fmt;
use sui_types::authenticator_state::get_authenticator_state_obj_initial_shared_version;
use sui_types::base_types::SequenceNumber;
use sui_types::bridge::{get_bridge_obj_initial_shared_version, is_bridge_committee_initiated};
use sui_types::deny_list_v1::get_deny_list_obj_initial_shared_version;
use ika_types::error::IkaResult;
use ika_types::messages_checkpoint::{CheckpointMessageDigest, CheckpointTimestamp};
use sui_types::randomness_state::get_randomness_state_obj_initial_shared_version;
use sui_types::storage::ObjectStore;
use ika_types::sui::ika_system_state::epoch_start_ika_system_state::{
    EpochStartSystemState, EpochStartSystemStateTrait,
};


#[enum_dispatch]
pub trait EpochStartConfigTrait {
    fn epoch_start_state(&self) -> &EpochStartSystemState;
    fn authenticator_obj_initial_shared_version(&self) -> Option<SequenceNumber>;
    fn randomness_obj_initial_shared_version(&self) -> Option<SequenceNumber>;
    fn coin_deny_list_obj_initial_shared_version(&self) -> Option<SequenceNumber>;
    fn bridge_obj_initial_shared_version(&self) -> Option<SequenceNumber>;
    fn bridge_committee_initiated(&self) -> bool;
}


/// Parameters of the epoch fixed at epoch start.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[enum_dispatch(EpochStartConfigTrait)]
pub enum EpochStartConfiguration {
    V1(EpochStartConfigurationV1),
    V2(EpochStartConfigurationV2),
    V3(EpochStartConfigurationV3),
    V4(EpochStartConfigurationV4),
    V5(EpochStartConfigurationV5),
    V6(EpochStartConfigurationV6),
}

impl EpochStartConfiguration {
    pub fn new(
        system_state: EpochStartSystemState,
        //object_store: &dyn ObjectStore,
    ) -> IkaResult<Self> {
        // let authenticator_obj_initial_shared_version =
        //     get_authenticator_state_obj_initial_shared_version(object_store).unwrap();//?;
        // let randomness_obj_initial_shared_version =
        //     get_randomness_state_obj_initial_shared_version(object_store).unwrap();//?;
        // let coin_deny_list_obj_initial_shared_version =
        //     get_deny_list_obj_initial_shared_version(object_store);
        // let bridge_obj_initial_shared_version =
        //     get_bridge_obj_initial_shared_version(object_store).unwrap();//?;
        // let bridge_committee_initiated = is_bridge_committee_initiated(object_store).unwrap();//?;
        Ok(Self::V6(EpochStartConfigurationV6 {
            system_state,
            authenticator_obj_initial_shared_version: None,
            randomness_obj_initial_shared_version: None,
            coin_deny_list_obj_initial_shared_version: None,
            bridge_obj_initial_shared_version: None,
            bridge_committee_initiated: false,
        }))
    }

    pub fn new_at_next_epoch_for_testing(&self) -> Self {
        // We only need to implement this function for the latest version.
        // When a new version is introduced, this function should be updated.
        match self {
            Self::V6(config) => {
                Self::V6(EpochStartConfigurationV6 {
                    system_state: config.system_state.new_at_next_epoch_for_testing(),
                    authenticator_obj_initial_shared_version: config.authenticator_obj_initial_shared_version,
                    randomness_obj_initial_shared_version: config.randomness_obj_initial_shared_version,
                    coin_deny_list_obj_initial_shared_version: config.coin_deny_list_obj_initial_shared_version,
                    bridge_obj_initial_shared_version: config.bridge_obj_initial_shared_version,
                    bridge_committee_initiated: config.bridge_committee_initiated,
                })
            }
            _ => panic!("This function is only implemented for the latest version of EpochStartConfiguration"),
        }
    }

    pub fn epoch_start_timestamp_ms(&self) -> CheckpointTimestamp {
        self.epoch_start_state().epoch_start_timestamp_ms()
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct EpochStartConfigurationV1 {
    system_state: EpochStartSystemState,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct EpochStartConfigurationV2 {
    system_state: EpochStartSystemState,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct EpochStartConfigurationV3 {
    system_state: EpochStartSystemState,
    /// Does the authenticator state object exist at the beginning of the epoch?
    authenticator_obj_initial_shared_version: Option<SequenceNumber>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct EpochStartConfigurationV4 {
    system_state: EpochStartSystemState,
    /// Do the state objects exist at the beginning of the epoch?
    authenticator_obj_initial_shared_version: Option<SequenceNumber>,
    randomness_obj_initial_shared_version: Option<SequenceNumber>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct EpochStartConfigurationV5 {
    system_state: EpochStartSystemState,
    /// Do the state objects exist at the beginning of the epoch?
    authenticator_obj_initial_shared_version: Option<SequenceNumber>,
    randomness_obj_initial_shared_version: Option<SequenceNumber>,
    coin_deny_list_obj_initial_shared_version: Option<SequenceNumber>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct EpochStartConfigurationV6 {
    system_state: EpochStartSystemState,
    /// Do the state objects exist at the beginning of the epoch?
    authenticator_obj_initial_shared_version: Option<SequenceNumber>,
    randomness_obj_initial_shared_version: Option<SequenceNumber>,
    coin_deny_list_obj_initial_shared_version: Option<SequenceNumber>,
    bridge_obj_initial_shared_version: Option<SequenceNumber>,
    bridge_committee_initiated: bool,
}

impl EpochStartConfigurationV1 {
    pub fn new(system_state: EpochStartSystemState) -> Self {
        Self {
            system_state,
        }
    }
}

impl EpochStartConfigTrait for EpochStartConfigurationV1 {

    fn epoch_start_state(&self) -> &EpochStartSystemState {
        &self.system_state
    }
    
    fn authenticator_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn randomness_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn coin_deny_list_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn bridge_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn bridge_committee_initiated(&self) -> bool {
        false
    }
}

impl EpochStartConfigTrait for EpochStartConfigurationV2 {

    fn epoch_start_state(&self) -> &EpochStartSystemState {
        &self.system_state
    }

    fn authenticator_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn randomness_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn coin_deny_list_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn bridge_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn bridge_committee_initiated(&self) -> bool {
        false
    }
}

impl EpochStartConfigTrait for EpochStartConfigurationV3 {

    fn epoch_start_state(&self) -> &EpochStartSystemState {
        &self.system_state
    }

    fn authenticator_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        self.authenticator_obj_initial_shared_version
    }

    fn randomness_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn coin_deny_list_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn bridge_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }
    fn bridge_committee_initiated(&self) -> bool {
        false
    }
}

impl EpochStartConfigTrait for EpochStartConfigurationV4 {

    fn epoch_start_state(&self) -> &EpochStartSystemState {
        &self.system_state
    }

    fn authenticator_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        self.authenticator_obj_initial_shared_version
    }

    fn randomness_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        self.randomness_obj_initial_shared_version
    }

    fn coin_deny_list_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn bridge_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn bridge_committee_initiated(&self) -> bool {
        false
    }
}

impl EpochStartConfigTrait for EpochStartConfigurationV5 {

    fn epoch_start_state(&self) -> &EpochStartSystemState {
        &self.system_state
    }

    fn authenticator_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        self.authenticator_obj_initial_shared_version
    }

    fn randomness_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        self.randomness_obj_initial_shared_version
    }

    fn coin_deny_list_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        self.coin_deny_list_obj_initial_shared_version
    }

    fn bridge_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }
    fn bridge_committee_initiated(&self) -> bool {
        false
    }
}

impl EpochStartConfigTrait for EpochStartConfigurationV6 {

    fn epoch_start_state(&self) -> &EpochStartSystemState {
        &self.system_state
    }

    fn authenticator_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        self.authenticator_obj_initial_shared_version
    }

    fn randomness_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        self.randomness_obj_initial_shared_version
    }

    fn coin_deny_list_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        self.coin_deny_list_obj_initial_shared_version
    }

    fn bridge_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        self.bridge_obj_initial_shared_version
    }

    fn bridge_committee_initiated(&self) -> bool {
        self.bridge_committee_initiated
    }
}
