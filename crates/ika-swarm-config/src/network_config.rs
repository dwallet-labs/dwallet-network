// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::validator_initialization_config::ValidatorInitializationConfig;
use ika_config::{Config, NodeConfig};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sui_types::base_types::ObjectID;

/// This is a config that is used for testing or local use as it contains the config and keys for
/// all validators
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub validator_configs: Vec<NodeConfig>,
    pub fullnode_configs: Vec<NodeConfig>,
    pub validator_initialization_configs: Vec<ValidatorInitializationConfig>,
    pub ika_package_id: ObjectID,
    pub ika_system_package_id: ObjectID,
    pub ika_system_object_id: ObjectID,
}

impl Config for NetworkConfig {}

impl NetworkConfig {
    pub fn validator_configs(&self) -> &[NodeConfig] {
        &self.validator_configs
    }

    pub fn into_validator_configs(self) -> Vec<NodeConfig> {
        self.validator_configs
    }

    pub fn fullnode_configs(&self) -> &[NodeConfig] {
        &self.fullnode_configs
    }

    pub fn into_fullnode_configs(self) -> Vec<NodeConfig> {
        self.fullnode_configs
    }
}
