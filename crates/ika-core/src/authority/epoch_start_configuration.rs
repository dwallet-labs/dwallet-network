// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use ika_types::error::IkaResult;
use ika_types::sui::epoch_start_system::{EpochStartSystem, EpochStartSystemTrait};

#[enum_dispatch]
pub trait EpochStartConfigTrait {
    fn epoch_start_state(&self) -> &EpochStartSystem;
}

/// Parameters of the epoch fixed at epoch start.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[enum_dispatch(EpochStartConfigTrait)]
pub enum EpochStartConfiguration {
    V1(EpochStartConfigurationV1),
}

impl EpochStartConfiguration {
    pub fn new(system_state: EpochStartSystem) -> IkaResult<Self> {
        Ok(Self::V1(EpochStartConfigurationV1 { system_state }))
    }

    pub fn new_at_next_epoch_for_testing(&self) -> Self {
        // We only need to implement this function for the latest version.
        // When a new version is introduced, this function should be updated.
        match self {
            Self::V1(config) => Self::V1(EpochStartConfigurationV1 {
                system_state: config.system_state.new_at_next_epoch_for_testing(),
            }),
        }
    }

    pub fn epoch_start_timestamp_ms(&self) -> u64 {
        self.epoch_start_state().epoch_start_timestamp_ms()
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct EpochStartConfigurationV1 {
    system_state: EpochStartSystem,
    // Do the state objects exist at the beginning of the epoch?
}

impl EpochStartConfigurationV1 {
    pub fn new(system_state: EpochStartSystem) -> Self {
        Self { system_state }
    }
}

impl EpochStartConfigTrait for EpochStartConfigurationV1 {
    fn epoch_start_state(&self) -> &EpochStartSystem {
        &self.system_state
    }
}
