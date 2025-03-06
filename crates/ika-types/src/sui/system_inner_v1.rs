// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::{Element, SystemInnerTrait};
use serde::{Deserialize, Serialize};
use sui_types::balance::Balance;
use sui_types::base_types::ObjectID;
use sui_types::coin::TreasuryCap;
use sui_types::collection_types::{Bag, Table, TableVec, VecMap, VecSet};
use sui_types::id::ID;

/// Rust version of the Move ika::ika_system::SystemParameters type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SystemParametersV1 {
    /// The duration of an epoch, in milliseconds.
    pub epoch_duration_ms: u64,

    /// The starting epoch in which stake subsidies start being paid out
    pub stake_subsidy_start_epoch: u64,

    /// Minimum number of active validators at any moment.
    pub min_validator_count: u64,

    /// Maximum number of active validators at any moment.
    /// We do not allow the number of validators in any epoch to go above this.
    pub max_validator_count: u64,

    /// Lower-bound on the amount of stake required to become a validator.
    pub min_validator_joining_stake: u64,

    /// Validators with stake amount below `validator_low_stake_threshold` are considered to
    /// have low stake and will be escorted out of the validator set after being below this
    /// threshold for more than `validator_low_stake_grace_period` number of epochs.
    pub validator_low_stake_threshold: u64,

    /// Validators with stake below `validator_very_low_stake_threshold` will be removed
    /// immediately at epoch change, no grace period.
    pub validator_very_low_stake_threshold: u64,

    /// A validator can have stake below `validator_low_stake_threshold`
    /// for this many epochs before being kicked out.
    pub validator_low_stake_grace_period: u64,

    /// how many reward are slashed to punish a validator, in bps.
    pub reward_slashing_rate: u16,

    /// Lock active committee between epochs.
    pub lock_active_committee: bool,

    /// Any extra fields that's not defined statically.
    pub extra_fields: Bag,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct BlsCommitteeMember {
    pub validator_id: ObjectID,
    pub protocol_pubkey: Element,
    pub voting_power: u64,
    pub stake: u64,
}

/// Represents the current committee in the system.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct BlsCommittee {
    pub members: Vec<BlsCommitteeMember>,
    pub aggregated_protocol_pubkey: Element,
}

pub type ObjectTable = Table;

/// Rust version of the Move ika_system::validator_set::ValidatorSet type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidatorSetV1 {
    pub total_stake: u64,
    pub validators: ObjectTable,
    pub active_committee: BlsCommittee,
    pub next_epoch_active_committee: Option<BlsCommittee>,
    pub previous_committee: BlsCommittee,
    pub pending_active_validators: Vec<ObjectID>,
    pub at_risk_validators: VecMap<ID, u64>,
    pub validator_report_records: VecMap<ObjectID, VecSet<ObjectID>>,
    pub extra_fields: Bag,
}

/// Rust version of the Move sui::package::UpgradeCap.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct UpgradeCap {
    pub id: ObjectID,
    pub package: ObjectID,
    pub version: u64,
    pub policy: u8,
}

/// Rust version of the Move ika_system::ika_system::IkaSystemStateInner type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SystemInnerV1 {
    pub epoch: u64,
    pub protocol_version: u64,
    pub upgrade_caps: Vec<UpgradeCap>,
    pub validators: ValidatorSetV1,
    pub parameters: SystemParametersV1,
    pub computation_price_per_unit_size: u64,
    pub ika_treasury: IkaTreasuryV1,
    pub epoch_start_timestamp_ms: u64,
    pub total_messages_processed: u64,
    pub last_processed_checkpoint_sequence_number: Option<u32>,
    pub computation_reward: Balance,
    pub authorized_protocol_cap_ids: Vec<ObjectID>,
    pub dwallet_2pc_mpc_secp256k1_id: Option<ObjectID>,
    pub dwallet_2pc_mpc_secp256k1_network_decryption_keys: Vec<DWalletNetworkDecryptionKeyCap>,
    pub extra_fields: Bag,
    // TODO: Use getters instead of all pub.
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct IkaTreasuryV1 {
    /// TreasuryCap of IKA tokens.
    pub treasury_cap: TreasuryCap,

    /// Count of the number of times stake subsidies have been distributed.
    pub stake_subsidy_distribution_counter: u64,

    /// The rate at which the amount per distribution is calculated based on
    /// period nad total supply. Expressed in basis points.
    pub stake_subsidy_rate: u16,

    /// The amount of stake subsidy to be distrabtured per distribution.
    /// This amount changes based on `stake_subsidy_rate`.
    pub stake_subsidy_amount_per_distribution: u64,

    /// Number of distributions to occur before the amount per distribution will be recalculated.
    pub stake_subsidy_period_length: u64,

    pub extra_fields: Bag,
}

impl SystemInnerTrait for SystemInnerV1 {
    fn epoch(&self) -> u64 {
        self.epoch
    }

    fn computation_price_per_unit_size(&self) -> u64 {
        self.computation_price_per_unit_size
    }

    fn protocol_version(&self) -> u64 {
        self.protocol_version
    }

    fn upgrade_caps(&self) -> &Vec<UpgradeCap> {
        &self.upgrade_caps
    }

    fn epoch_start_timestamp_ms(&self) -> u64 {
        self.epoch_start_timestamp_ms
    }

    fn last_processed_checkpoint_sequence_number(&self) -> Option<u32> {
        self.last_processed_checkpoint_sequence_number
    }

    fn epoch_duration_ms(&self) -> u64 {
        self.parameters.epoch_duration_ms
    }

    fn dwallet_2pc_mpc_secp256k1_id(&self) -> Option<ObjectID> {
        self.dwallet_2pc_mpc_secp256k1_id
    }

    //
    // fn get_current_epoch_committee(&self) -> CommitteeWithNetworkMetadata {
    //     let validators = self
    //         .validators
    //         .active_validators
    //         .iter()
    //         .map(|validator| {
    //             let verified_metadata = validator.verified_metadata();
    //             let name = verified_metadata.ika_pubkey_bytes();
    //             (
    //                 name,
    //                 (
    //                     validator.voting_power,
    //                     NetworkMetadata {
    //                         network_address: verified_metadata.network_address.clone(),
    //                         consensus_address: verified_metadata.consensus_address.clone(),
    //                         network_public_key: Some(verified_metadata.network_pubkey.clone()),
    //                     },
    //                 ),
    //             )
    //         })
    //         .collect();
    //     CommitteeWithNetworkMetadata::new(self.epoch, validators)
    // }
    //
    // fn into_epoch_start_state(self) -> EpochStartSystemState {
    //     EpochStartSystemState::new_v1(
    //         self.epoch,
    //         self.protocol_version,
    //         self.computation_price_per_unit_size,
    //         self.epoch_start_timestamp_ms,
    //         self.parameters.epoch_duration_ms,
    //         self.validators
    //             .active_validators
    //             .iter()
    //             .map(|validator| {
    //                 let metadata = validator.verified_metadata();
    //                 EpochStartValidatorInfoV1 {
    //                     sui_address: metadata.proof_of_possession_sender,
    //                     protocol_pubkey: metadata.protocol_pubkey.clone(),
    //                     network_pubkey: metadata.network_pubkey.clone(),
    //                     consensus_pubkey: metadata.consensus_pubkey.clone(),
    //                     ika_network_address: metadata.network_address.clone(),
    //                     p2p_address: metadata.p2p_address.clone(),
    //                     consensus_address: metadata.consensus_address.clone(),
    //                     voting_power: validator.voting_power,
    //                     hostname: metadata.name.clone(),
    //                 }
    //             })
    //             .collect(),
    //     )
    // }
}

/// Rust version of the Move ika_system::validator_cap::ValidatorCap type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidatorCapV1 {
    pub id: ObjectID,
    pub validator_id: ObjectID,
}

/// Rust version of the Move ika_system::validator_cap::ValidatorOperationCap type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidatorOperationCapV1 {
    pub id: ObjectID,
    pub validator_id: ObjectID,
}

/// Rust version of the Move ika_system::dwallet_2pc_mpc_secp256k1_inner::DWalletNetworkDecryptionKeyCap type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DWalletNetworkDecryptionKeyCap {
    pub id: ObjectID,
    pub dwallet_network_decryption_key_id: ObjectID,
}
