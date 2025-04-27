// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use enum_dispatch::enum_dispatch;
use std::collections::HashMap;

use crate::committee::{Committee, CommitteeWithNetworkMetadata, NetworkMetadata, StakeUnit};
use crate::crypto::{AuthorityName, AuthorityPublicKey, NetworkPublicKey};
use crate::messages_dwallet_mpc::DWalletNetworkDecryptionKeyData;
use anemo::types::{PeerAffinity, PeerInfo};
use anemo::PeerId;
use consensus_config::{Authority, Committee as ConsensusCommittee};
use dwallet_mpc_types::dwallet_mpc::ClassGroupsPublicKeyAndProofBytes;
use fastcrypto::bls12381;
use fastcrypto::traits::{KeyPair, ToFromBytes};
use ika_protocol_config::ProtocolVersion;
use rand::prelude::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use sui_types::base_types::{EpochId, ObjectID};
use sui_types::multiaddr::Multiaddr;
use tracing::{error, warn};

#[enum_dispatch]
pub trait EpochStartSystemTrait {
    fn epoch(&self) -> EpochId;
    fn protocol_version(&self) -> ProtocolVersion;
    fn epoch_start_timestamp_ms(&self) -> u64;
    fn epoch_duration_ms(&self) -> u64;
    fn get_ika_committee(&self) -> Committee;
    fn get_ika_committee_with_network_metadata(&self) -> CommitteeWithNetworkMetadata;
    fn get_consensus_committee(&self) -> ConsensusCommittee;
    fn get_validator_as_p2p_peers(&self, excluding_self: AuthorityName) -> Vec<PeerInfo>;
    fn get_authority_names_to_peer_ids(&self) -> HashMap<AuthorityName, PeerId>;
    fn get_authority_names_to_hostnames(&self) -> HashMap<AuthorityName, String>;
    fn get_dwallet_network_decryption_keys(
        &self,
    ) -> &HashMap<ObjectID, DWalletNetworkDecryptionKeyData>;
}

/// This type captures the minimum amount of information from `System` needed by a validator
/// to run the protocol. This allows us to decouple from the actual `System` type, and hence
/// do not need to evolve it when we upgrade the `System` type.
/// Evolving EpochStartSystem is also a lot easier in that we could add optional fields
/// and fill them with None for older versions. When we absolutely must delete fields, we could
/// also add new db tables to store the new version. This is OK because we only store one copy of
/// this as part of EpochStartConfiguration for the most recent epoch in the db.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[enum_dispatch(EpochStartSystemTrait)]
pub enum EpochStartSystem {
    V1(EpochStartSystemV1),
}

impl EpochStartSystem {
    pub fn new_v1(
        epoch: EpochId,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        active_validators: Vec<EpochStartValidatorInfoV1>,
        dwallet_network_decryption_keys: HashMap<ObjectID, DWalletNetworkDecryptionKeyData>,
    ) -> Self {
        Self::V1(EpochStartSystemV1 {
            epoch,
            protocol_version,
            epoch_start_timestamp_ms,
            epoch_duration_ms,
            active_validators,
            dwallet_network_decryption_keys,
        })
    }

    pub fn new_for_testing_with_epoch(epoch: EpochId) -> Self {
        Self::V1(EpochStartSystemV1::new_for_testing_with_epoch(epoch))
    }

    pub fn new_at_next_epoch_for_testing(&self) -> Self {
        // Only need to support the latest version for testing.
        match self {
            Self::V1(state) => Self::V1(EpochStartSystemV1 {
                epoch: state.epoch + 1,
                protocol_version: state.protocol_version,
                epoch_start_timestamp_ms: state.epoch_start_timestamp_ms,
                epoch_duration_ms: state.epoch_duration_ms,
                active_validators: state.active_validators.clone(),
                dwallet_network_decryption_keys: state.dwallet_network_decryption_keys.clone(),
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct EpochStartSystemV1 {
    epoch: EpochId,
    protocol_version: u64,
    epoch_start_timestamp_ms: u64,
    epoch_duration_ms: u64,
    active_validators: Vec<EpochStartValidatorInfoV1>,
    dwallet_network_decryption_keys: HashMap<ObjectID, DWalletNetworkDecryptionKeyData>,
}

impl EpochStartSystemV1 {
    pub fn new_for_testing() -> Self {
        Self::new_for_testing_with_epoch(0)
    }

    pub fn new_for_testing_with_epoch(epoch: EpochId) -> Self {
        Self {
            epoch,
            protocol_version: ProtocolVersion::MAX.as_u64(),
            epoch_start_timestamp_ms: 0,
            epoch_duration_ms: 1000,
            active_validators: vec![],
            dwallet_network_decryption_keys: HashMap::new(),
        }
    }
}

impl EpochStartSystemTrait for EpochStartSystemV1 {
    fn epoch(&self) -> EpochId {
        self.epoch
    }

    fn protocol_version(&self) -> ProtocolVersion {
        ProtocolVersion::new(self.protocol_version)
    }

    fn epoch_start_timestamp_ms(&self) -> u64 {
        self.epoch_start_timestamp_ms
    }

    fn epoch_duration_ms(&self) -> u64 {
        self.epoch_duration_ms
    }

    fn get_ika_committee_with_network_metadata(&self) -> CommitteeWithNetworkMetadata {
        let validators = self
            .active_validators
            .iter()
            .map(|validator| {
                (
                    validator.authority_name(),
                    (
                        validator.voting_power,
                        NetworkMetadata {
                            network_address: validator.network_address.clone(),
                            consensus_address: validator.consensus_address.clone(),
                            network_public_key: Some(validator.network_pubkey.clone()),
                            class_groups_public_key_and_proof: validator
                                .class_groups_public_key_and_proof
                                .clone(),
                        },
                    ),
                )
            })
            .collect();

        CommitteeWithNetworkMetadata::new(self.epoch, validators)
    }

    fn get_ika_committee(&self) -> Committee {
        let voting_rights = self
            .active_validators
            .iter()
            .map(|validator| (validator.authority_name(), validator.voting_power))
            .collect();
        let class_groups_public_keys_and_proofs = self
            .active_validators
            .iter()
            .map(|validator| {
                (
                    validator.authority_name(),
                    validator.class_groups_public_key_and_proof.clone(),
                )
            })
            .collect();
        Committee::new(
            self.epoch,
            voting_rights,
            class_groups_public_keys_and_proofs,
        )
    }

    fn get_consensus_committee(&self) -> ConsensusCommittee {
        let ika_committee = self.get_ika_committee();
        let mut authorities = vec![];
        for (i, (name, stake)) in ika_committee.members().enumerate() {
            let active_validator = &self.active_validators[i];
            if name.0 != active_validator.protocol_pubkey.as_bytes() {
                error!(
                    "Mismatched authority order between Ika and Mysticeti! Index {}, Mysticeti authority {:?}\nIka authority name {:?}",
                    i, name, active_validator.protocol_pubkey.as_bytes()
                );
            }
            authorities.push(Authority {
                stake: *stake as consensus_config::Stake,
                address: active_validator.consensus_address.clone(),
                hostname: active_validator.hostname.clone(),
                authority_key: consensus_config::AuthorityPublicKey::new(
                    // This key is not really in use
                    // TODO(omersadika) - try to make a PR to change that
                    bls12381::min_sig::BLS12381KeyPair::generate(&mut StdRng::from_seed([0; 32]))
                        .public()
                        .clone(),
                ),
                protocol_key: consensus_config::ProtocolPublicKey::new(
                    active_validator.consensus_pubkey.clone(),
                ),
                network_key: consensus_config::NetworkPublicKey::new(
                    active_validator.network_pubkey.clone(),
                ),
            });
        }

        ConsensusCommittee::new(self.epoch as consensus_config::Epoch, authorities)
    }

    fn get_validator_as_p2p_peers(&self, excluding_self: AuthorityName) -> Vec<PeerInfo> {
        self.active_validators
            .iter()
            .filter(|validator| validator.authority_name() != excluding_self)
            .map(|validator| {
                let address = validator
                    .p2p_address
                    .to_anemo_address()
                    .into_iter()
                    .collect::<Vec<_>>();
                let peer_id = PeerId(validator.network_pubkey.0.to_bytes());
                if address.is_empty() {
                    warn!(
                        ?peer_id,
                        "Peer has invalid p2p address: {}", &validator.p2p_address
                    );
                }
                PeerInfo {
                    peer_id,
                    affinity: PeerAffinity::High,
                    address,
                }
            })
            .collect()
    }

    fn get_authority_names_to_peer_ids(&self) -> HashMap<AuthorityName, PeerId> {
        self.active_validators
            .iter()
            .map(|validator| {
                let name = validator.authority_name();
                let peer_id = PeerId(validator.network_pubkey.0.to_bytes());

                (name, peer_id)
            })
            .collect()
    }

    fn get_authority_names_to_hostnames(&self) -> HashMap<AuthorityName, String> {
        self.active_validators
            .iter()
            .map(|validator| {
                let name = validator.authority_name();
                let hostname = validator.hostname.clone();

                (name, hostname)
            })
            .collect()
    }

    fn get_dwallet_network_decryption_keys(
        &self,
    ) -> &HashMap<ObjectID, DWalletNetworkDecryptionKeyData> {
        &self.dwallet_network_decryption_keys
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EpochStartValidatorInfoV1 {
    pub validator_id: ObjectID,
    pub protocol_pubkey: AuthorityPublicKey,
    pub network_pubkey: NetworkPublicKey,
    pub consensus_pubkey: NetworkPublicKey,
    pub class_groups_public_key_and_proof: ClassGroupsPublicKeyAndProofBytes,
    pub network_address: Multiaddr,
    pub p2p_address: Multiaddr,
    pub consensus_address: Multiaddr,
    pub voting_power: StakeUnit,
    pub hostname: String,
}

impl EpochStartValidatorInfoV1 {
    pub fn authority_name(&self) -> AuthorityName {
        (&self.protocol_pubkey).into()
    }
}

#[cfg(test)]
mod test {
    use super::super::epoch_start_system::{
        EpochStartSystemTrait, EpochStartSystemV1, EpochStartValidatorInfoV1,
    };
    use fastcrypto::traits::KeyPair;
    use ika_protocol_config::ProtocolVersion;
    use mysten_network::Multiaddr;
    use rand::thread_rng;
    use sui_types::base_types::SuiAddress;
    use sui_types::committee::CommitteeTrait;
    use sui_types::crypto::{get_key_pair, AuthorityKeyPair, NetworkKeyPair};

    #[test]
    fn test_ika_and_mysticeti_committee_are_same() {
        // GIVEN
        let mut active_validators = vec![];

        for i in 0..10 {
            let (sui_address, protocol_key): (SuiAddress, AuthorityKeyPair) = get_key_pair();
            let narwhal_network_key = NetworkKeyPair::generate(&mut thread_rng());

            active_validators.push(EpochStartValidatorInfoV1 {
                protocol_pubkey: protocol_key.public().clone(),
                network_pubkey: narwhal_network_key.public().clone(),
                consensus_pubkey: narwhal_network_key.public().clone(),
                network_address: Multiaddr::empty(),
                p2p_address: Multiaddr::empty(),
                consensus_address: Multiaddr::empty(),
                voting_power: 1_000,
                hostname: format!("host-{i}").to_string(),
            })
        }

        let state = EpochStartSystemV1 {
            epoch: 10,
            protocol_version: ProtocolVersion::MAX.as_u64(),
            epoch_start_timestamp_ms: 0,
            epoch_duration_ms: 0,
            active_validators,
            dwallet_network_decryption_keys: Default::default(),
        };

        // WHEN
        let ika_committee = state.get_ika_committee();
        let consensus_committee = state.get_consensus_committee();

        // THEN
        // assert the validators details
        assert_eq!(ika_committee.num_members(), 10);
        assert_eq!(ika_committee.num_members(), consensus_committee.size());
        assert_eq!(
            ika_committee.validity_threshold(),
            consensus_committee.validity_threshold()
        );
        assert_eq!(
            ika_committee.quorum_threshold(),
            consensus_committee.quorum_threshold()
        );
        assert_eq!(state.epoch, consensus_committee.epoch());

        for (authority_index, consensus_authority) in consensus_committee.authorities() {
            let ika_authority_name = ika_committee
                .authority_by_index(authority_index.value() as u32)
                .unwrap();

            assert_eq!(
                consensus_authority.authority_key.to_bytes(),
                ika_authority_name.0,
                "Mysten & IKA committee member of same index correspond to different public key"
            );
            assert_eq!(
                consensus_authority.stake,
                ika_committee.weight(ika_authority_name),
                "Mysten & IKA committee member stake differs"
            );
        }
    }
}
