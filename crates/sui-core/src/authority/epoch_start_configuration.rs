// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use std::fmt;
use sui_types::authenticator_state::get_authenticator_state_obj_initial_shared_version;
use sui_types::base_types::SequenceNumber;
use sui_types::epoch_data::EpochData;
use sui_types::error::SuiResult;
use sui_types::messages_checkpoint::{CheckpointDigest, CheckpointTimestamp};
use sui_types::randomness_state::get_randomness_state_obj_initial_shared_version;
use sui_types::storage::ObjectStore;
use sui_types::sui_system_state::epoch_start_sui_system_state::{
    EpochStartSystemState, EpochStartSystemStateTrait,
};

#[enum_dispatch]
pub trait EpochStartConfigTrait {
    fn epoch_digest(&self) -> CheckpointDigest;
    fn epoch_start_state(&self) -> &EpochStartSystemState;
    fn flags(&self) -> &[EpochFlag];
    fn authenticator_obj_initial_shared_version(&self) -> Option<SequenceNumber>;
    fn randomness_obj_initial_shared_version(&self) -> Option<SequenceNumber>;
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum EpochFlag {
    InMemoryCheckpointRoots,
    PerEpochFinalizedTransactions,
}

/// Parameters of the epoch fixed at epoch start.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[enum_dispatch(EpochStartConfigTrait)]
pub enum EpochStartConfiguration {
    V1(EpochStartConfigurationV1),
    V2(EpochStartConfigurationV2),
    V3(EpochStartConfigurationV3),
    V4(EpochStartConfigurationV4),
}

impl EpochStartConfiguration {
    pub fn new(
        system_state: EpochStartSystemState,
        epoch_digest: CheckpointDigest,
        object_store: &dyn ObjectStore,
    ) -> SuiResult<Self> {
        let authenticator_obj_initial_shared_version =
            get_authenticator_state_obj_initial_shared_version(object_store)?;
        let randomness_obj_initial_shared_version =
            get_randomness_state_obj_initial_shared_version(object_store)?;
        Ok(Self::V4(EpochStartConfigurationV4 {
            system_state,
            epoch_digest,
            flags: EpochFlag::default_flags_for_new_epoch(),
            authenticator_obj_initial_shared_version,
            randomness_obj_initial_shared_version,
        }))
    }

    pub fn epoch_data(&self) -> EpochData {
        EpochData::new(
            self.epoch_start_state().epoch(),
            self.epoch_start_state().epoch_start_timestamp_ms(),
            self.epoch_digest(),
        )
    }

    pub fn epoch_start_timestamp_ms(&self) -> CheckpointTimestamp {
        self.epoch_start_state().epoch_start_timestamp_ms()
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct EpochStartConfigurationV1 {
    system_state: EpochStartSystemState,
    /// epoch_digest is defined as following
    /// (1) For the genesis epoch it is set to 0
    /// (2) For all other epochs it is a digest of the last checkpoint of a previous epoch
    /// Note that this is in line with how epoch start timestamp is defined
    epoch_digest: CheckpointDigest,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct EpochStartConfigurationV2 {
    system_state: EpochStartSystemState,
    epoch_digest: CheckpointDigest,
    flags: Vec<EpochFlag>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct EpochStartConfigurationV3 {
    system_state: EpochStartSystemState,
    epoch_digest: CheckpointDigest,
    flags: Vec<EpochFlag>,
    /// Does the authenticator state object exist at the beginning of the epoch?
    authenticator_obj_initial_shared_version: Option<SequenceNumber>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct EpochStartConfigurationV4 {
    system_state: EpochStartSystemState,
    epoch_digest: CheckpointDigest,
    flags: Vec<EpochFlag>,
    /// Do the state objects exist at the beginning of the epoch?
    authenticator_obj_initial_shared_version: Option<SequenceNumber>,
    randomness_obj_initial_shared_version: Option<SequenceNumber>,
    // TODO: In a new version, make these initial shared versions a vector.
}

impl EpochStartConfigurationV1 {
    pub fn new(system_state: EpochStartSystemState, epoch_digest: CheckpointDigest) -> Self {
        Self {
            system_state,
            epoch_digest,
        }
    }
}

impl EpochStartConfigTrait for EpochStartConfigurationV1 {
    fn epoch_digest(&self) -> CheckpointDigest {
        self.epoch_digest
    }

    fn epoch_start_state(&self) -> &EpochStartSystemState {
        &self.system_state
    }

    fn flags(&self) -> &[EpochFlag] {
        &[]
    }

    fn authenticator_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn randomness_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }
}

impl EpochStartConfigTrait for EpochStartConfigurationV2 {
    fn epoch_digest(&self) -> CheckpointDigest {
        self.epoch_digest
    }

    fn epoch_start_state(&self) -> &EpochStartSystemState {
        &self.system_state
    }

    fn flags(&self) -> &[EpochFlag] {
        &self.flags
    }

    fn authenticator_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }

    fn randomness_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }
}

impl EpochStartConfigTrait for EpochStartConfigurationV3 {
    fn epoch_digest(&self) -> CheckpointDigest {
        self.epoch_digest
    }

    fn epoch_start_state(&self) -> &EpochStartSystemState {
        &self.system_state
    }

    fn flags(&self) -> &[EpochFlag] {
        &self.flags
    }

    fn authenticator_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        self.authenticator_obj_initial_shared_version
    }

    fn randomness_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        None
    }
}

impl EpochStartConfigTrait for EpochStartConfigurationV4 {
    fn epoch_digest(&self) -> CheckpointDigest {
        self.epoch_digest
    }

    fn epoch_start_state(&self) -> &EpochStartSystemState {
        &self.system_state
    }

    fn flags(&self) -> &[EpochFlag] {
        &self.flags
    }

    fn authenticator_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        self.authenticator_obj_initial_shared_version
    }

    fn randomness_obj_initial_shared_version(&self) -> Option<SequenceNumber> {
        self.randomness_obj_initial_shared_version
    }
}

impl EpochFlag {
    pub fn default_flags_for_new_epoch() -> Vec<Self> {
        vec![
            EpochFlag::InMemoryCheckpointRoots,
            EpochFlag::PerEpochFinalizedTransactions,
        ]
    }
}

impl fmt::Display for EpochFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Important - implementation should return low cardinality values because this is used as metric key
        match self {
            EpochFlag::InMemoryCheckpointRoots => write!(f, "InMemoryCheckpointRoots"),
            EpochFlag::PerEpochFinalizedTransactions => write!(f, "PerEpochFinalizedTransactions"),
        }
    }
}
