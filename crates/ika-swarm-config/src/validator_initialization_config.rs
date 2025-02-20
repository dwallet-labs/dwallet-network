// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::net::{IpAddr, SocketAddr};

use dwallet_classgroups_types::{
    generate_class_groups_keypair_and_proof_from_seed, ClassGroupsKeyPairAndProof,
};
use fastcrypto::traits::{KeyPair, ToFromBytes};
use ika_config::local_ip_utils;
use ika_config::validator_info::ValidatorInfo;
use ika_types::crypto::{
    generate_proof_of_possession, get_key_pair_from_rng, AccountKeyPair, AuthorityKeyPair,
    AuthorityPublicKeyBytes, NetworkKeyPair, NetworkPublicKey,
};
use ika_types::sui::{DEFAULT_COMMISSION_RATE, DEFAULT_VALIDATOR_COMPUTATION_PRICE};
use serde::{Deserialize, Serialize};
use sui_types::base_types::SuiAddress;
use sui_types::crypto::{PublicKey, SuiKeyPair};
use sui_types::multiaddr::Multiaddr;

pub const DEFAULT_NUMBER_OF_AUTHORITIES: usize = 4;

// All information needed to build a NodeConfig for a validator.
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatorInitializationConfig {
    pub name: Option<String>,
    pub class_groups_key_pair_and_proof: Box<ClassGroupsKeyPairAndProof>,
    #[serde(default = "default_bls12381_key_pair")]
    pub key_pair: AuthorityKeyPair,
    #[serde(default = "default_ed25519_key_pair")]
    pub worker_key_pair: NetworkKeyPair,
    #[serde(default = "default_ika_key_pair")]
    pub account_key_pair: SuiKeyPair,
    #[serde(default = "default_ed25519_key_pair")]
    pub network_key_pair: NetworkKeyPair,
    pub network_address: Multiaddr,
    pub p2p_address: Multiaddr,
    pub p2p_listen_address: Option<SocketAddr>,
    #[serde(default = "default_socket_address")]
    pub metrics_address: SocketAddr,
    pub computation_price: u64,
    pub commission_rate: u16,
    pub consensus_address: Multiaddr,
    #[serde(default = "default_stake")]
    pub stake: u64,
}

impl ValidatorInitializationConfig {
    pub fn to_validator_info(&self) -> ValidatorInfo {
        let name = self.name.clone().unwrap_or("".to_string());
        let class_groups_public_key_and_proof = self.class_groups_key_pair_and_proof.public_bytes();
        let protocol_public_key: AuthorityPublicKeyBytes = self.key_pair.public().into();
        let account_key: PublicKey = self.account_key_pair.public();
        let network_public_key: NetworkPublicKey = self.network_key_pair.public().clone();
        let worker_public_key: NetworkPublicKey = self.worker_key_pair.public().clone();
        let network_address = self.network_address.clone();
        let consensus_address = self.consensus_address.clone();

        ValidatorInfo {
            name,
            class_groups_public_key_and_proof,
            protocol_public_key,
            consensus_public_key: worker_public_key,
            network_public_key,
            account_address: SuiAddress::from(&account_key),
            computation_price: self.computation_price,
            commission_rate: self.commission_rate,
            network_address,
            p2p_address: self.p2p_address.clone(),
            consensus_address,
            description: String::new(),
            image_url: String::new(),
            project_url: String::new(),
            proof_of_possession: generate_proof_of_possession(
                &self.key_pair,
                (&self.account_key_pair.public()).into(),
            ),
        }
    }
}

#[derive(Default)]
pub struct ValidatorInitializationConfigBuilder {
    protocol_key_pair: Option<AuthorityKeyPair>,
    account_key_pair: Option<AccountKeyPair>,
    class_groups_key_pair_and_proof: Option<Box<ClassGroupsKeyPairAndProof>>,
    ip: Option<String>,
    computation_price: Option<u64>,
    /// If set, the validator will use deterministic addresses based on the port offset.
    /// This is useful for benchmarking.
    port_offset: Option<u16>,
    /// Whether to use a specific p2p listen ip address. This is useful for testing on AWS.
    p2p_listen_ip_address: Option<IpAddr>,
}

impl ValidatorInitializationConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_protocol_key_pair(mut self, key_pair: AuthorityKeyPair) -> Self {
        self.protocol_key_pair = Some(key_pair);
        self
    }

    pub fn with_account_key_pair(mut self, key_pair: AccountKeyPair) -> Self {
        self.account_key_pair = Some(key_pair);
        self
    }

    pub fn with_class_groups_key_pair_and_proof(
        mut self,
        key_pair: Box<ClassGroupsKeyPairAndProof>,
    ) -> Self {
        self.class_groups_key_pair_and_proof = Some(key_pair);
        self
    }

    pub fn with_ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn with_computation_price(mut self, computation_price: u64) -> Self {
        self.computation_price = Some(computation_price);
        self
    }

    pub fn with_deterministic_ports(mut self, port_offset: u16) -> Self {
        self.port_offset = Some(port_offset);
        self
    }

    pub fn with_p2p_listen_ip_address(mut self, p2p_listen_ip_address: IpAddr) -> Self {
        self.p2p_listen_ip_address = Some(p2p_listen_ip_address);
        self
    }

    pub fn build<R: rand::RngCore + rand::CryptoRng>(
        self,
        rng: &mut R,
    ) -> ValidatorInitializationConfig {
        let ip = self.ip.unwrap_or_else(local_ip_utils::get_new_ip);
        let localhost = local_ip_utils::localhost_for_testing();

        let protocol_key_pair = self
            .protocol_key_pair
            .unwrap_or_else(|| get_key_pair_from_rng(rng).1);
        let account_key_pair = self
            .account_key_pair
            .unwrap_or_else(|| get_key_pair_from_rng(rng).1);
        let computation_price = self
            .computation_price
            .unwrap_or(DEFAULT_VALIDATOR_COMPUTATION_PRICE);
        let class_groups_key_pair_and_proof = self
            .class_groups_key_pair_and_proof
            .clone()
            .unwrap_or_else(|| {
                Box::new(generate_class_groups_keypair_and_proof_from_seed(
                    protocol_key_pair
                        .copy()
                        .private()
                        .as_bytes()
                        .try_into()
                        // Safe to unwrap because the key is 32 bytes.
                        .unwrap(),
                ))
            });

        let (worker_key_pair, network_key_pair): (NetworkKeyPair, NetworkKeyPair) =
            (get_key_pair_from_rng(rng).1, get_key_pair_from_rng(rng).1);

        let (network_address, p2p_address, metrics_address, consensus_address) =
            if let Some(offset) = self.port_offset {
                (
                    local_ip_utils::new_deterministic_tcp_address_for_testing(&ip, offset),
                    local_ip_utils::new_deterministic_udp_address_for_testing(&ip, offset + 1),
                    local_ip_utils::new_deterministic_tcp_address_for_testing(&ip, offset + 2)
                        .with_zero_ip(),
                    local_ip_utils::new_deterministic_udp_address_for_testing(&ip, offset + 3),
                )
            } else {
                (
                    local_ip_utils::new_tcp_address_for_testing(&ip),
                    local_ip_utils::new_udp_address_for_testing(&ip),
                    local_ip_utils::new_tcp_address_for_testing(&localhost),
                    local_ip_utils::new_udp_address_for_testing(&ip),
                )
            };

        let p2p_listen_address = self
            .p2p_listen_ip_address
            .map(|ip| SocketAddr::new(ip, p2p_address.port().unwrap()));

        ValidatorInitializationConfig {
            key_pair: protocol_key_pair,
            class_groups_key_pair_and_proof,
            worker_key_pair,
            account_key_pair: account_key_pair.into(),
            network_key_pair,
            network_address,
            p2p_address,
            p2p_listen_address,
            metrics_address: metrics_address.to_socket_addr().unwrap(),
            computation_price,
            commission_rate: DEFAULT_COMMISSION_RATE,
            consensus_address,
            stake: ika_types::governance::MIN_VALIDATOR_JOINING_STAKE_NIKA,
            name: None,
        }
    }
}

fn default_socket_address() -> SocketAddr {
    local_ip_utils::new_local_tcp_socket_for_testing()
}

fn default_multiaddr_address() -> Multiaddr {
    local_ip_utils::new_local_tcp_address_for_testing()
}

fn default_stake() -> u64 {
    ika_types::governance::VALIDATOR_LOW_STAKE_THRESHOLD_NIKA
}

fn default_bls12381_key_pair() -> AuthorityKeyPair {
    get_key_pair_from_rng(&mut rand::rngs::OsRng).1
}

fn default_ed25519_key_pair() -> NetworkKeyPair {
    get_key_pair_from_rng(&mut rand::rngs::OsRng).1
}

fn default_ika_key_pair() -> SuiKeyPair {
    SuiKeyPair::Ed25519(get_key_pair_from_rng(&mut rand::rngs::OsRng).1)
}
