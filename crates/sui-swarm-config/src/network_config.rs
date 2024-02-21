// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sui_config::{genesis, Config, NodeConfig};
use sui_types::committee::CommitteeWithNetworkMetadata;
use sui_types::crypto::AccountKeyPair;
use sui_types::multiaddr::Multiaddr;

/// This is a config that is used for testing or local use as it contains the config and keys for
/// all validators
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub validator_configs: Vec<NodeConfig>,
    pub account_keys: Vec<AccountKeyPair>,
    pub genesis: genesis::Genesis,
}

impl Config for NetworkConfig {}

impl NetworkConfig {
    pub fn validator_configs(&self) -> &[NodeConfig] {
        &self.validator_configs
    }

    pub fn net_addresses(&self) -> Vec<Multiaddr> {
        self.genesis
            .committee_with_network()
            .network_metadata
            .into_values()
            .map(|n| n.network_address)
            .collect()
    }

    pub fn committee_with_network(&self) -> CommitteeWithNetworkMetadata {
        self.genesis.committee_with_network()
    }

    pub fn into_validator_configs(self) -> Vec<NodeConfig> {
        self.validator_configs
    }
}
