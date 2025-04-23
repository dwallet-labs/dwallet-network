// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::{Element, SystemInnerTrait};
use crate::committee::StakeUnit;
use crate::crypto::{AuthorityName, AuthorityPublicKey};
use fastcrypto::traits::ToFromBytes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sui_types::balance::Balance;
use sui_types::base_types::ObjectID;
use sui_types::coin::TreasuryCap;
use sui_types::collection_types::{Bag, Table, VecMap, VecSet};
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
    pub next_epoch_committee: Option<BlsCommittee>,
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
    pub computation_reward: Balance,
    pub authorized_protocol_cap_ids: Vec<ObjectID>,
    pub dwallet_2pc_mpc_secp256k1_id: Option<ObjectID>,
    pub dwallet_2pc_mpc_secp256k1_network_decryption_keys: Vec<DWalletNetworkDecryptionKeyCap>,
    pub extra_fields: Bag,
    // TODO: Use getters instead of all pub.
}

/// Rust version of the Move PricingPerOperation type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PricingPerOperation {
    pub consensus_validation_ika: u64,
    pub computation_ika: u64,
    pub gas_fee_reimbursement_sui: u64,
}

/// Rust version of the Move DWalletPricing2PcMpcSecp256K1 type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DWalletPricing2PcMpcSecp256K1 {
    id: ObjectID,
    dkg_first_round: PricingPerOperation,
    dkg_second_round: PricingPerOperation,
    re_encrypt_user_share: PricingPerOperation,
    ecdsa_presign: PricingPerOperation,
    ecdsa_sign: PricingPerOperation,
    ecdsa_future_sign: PricingPerOperation,
    ecdsa_sign_with_partial_user_signature: PricingPerOperation,
}

/// Rust version of the Move DWalletCoordinatorInner type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DWalletCoordinatorInnerV1 {
    pub current_epoch: u64,
    pub sessions: ObjectTable,
    pub session_start_events: Bag,
    pub number_of_completed_sessions: u64,
    pub started_immediate_sessions_count: u64,
    pub completed_immediate_sessions_count: u64,
    pub next_session_sequence_number: u64,
    pub last_session_to_complete_in_current_epoch: u64,
    pub locked_last_session_to_complete_in_current_epoch: bool,
    pub max_active_sessions_buffer: u64,
    pub dwallets: ObjectTable,
    pub dwallet_network_decryption_keys: ObjectTable,
    pub encryption_keys: ObjectTable,
    pub ecdsa_partial_centralized_signed_messages: ObjectTable,
    pub pricing: DWalletPricing2PcMpcSecp256K1,
    pub gas_fee_reimbursement_sui: Balance,
    pub consensus_validation_fee_charged_ika: Balance,
    pub active_committee: BlsCommittee,
    pub previous_committee: BlsCommittee,
    pub total_messages_processed: u64,
    pub last_processed_checkpoint_sequence_number: Option<u64>,
    pub previous_epoch_last_checkpoint_sequence_number: u64,
    pub extra_fields: Bag,
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

    fn last_processed_checkpoint_sequence_number(&self) -> Option<u64> {
        self.last_processed_checkpoint_sequence_number
    }

    fn previous_epoch_last_checkpoint_sequence_number(&self) -> u64 {
        self.previous_epoch_last_checkpoint_sequence_number
    }

    fn epoch_duration_ms(&self) -> u64 {
        self.parameters.epoch_duration_ms
    }

    fn dwallet_2pc_mpc_secp256k1_id(&self) -> Option<ObjectID> {
        self.dwallet_2pc_mpc_secp256k1_id
    }

    fn dwallet_2pc_mpc_secp256k1_network_decryption_keys(
        &self,
    ) -> &Vec<DWalletNetworkDecryptionKeyCap> {
        &self.dwallet_2pc_mpc_secp256k1_network_decryption_keys
    }

    fn validators(&self) -> &ValidatorSetV1 {
        &self.validators
    }

    fn read_bls_committee(
        &self,
        bls_committee: &BlsCommittee,
    ) -> HashMap<ObjectID, (AuthorityName, StakeUnit)> {
        let committee_validator_ids: Vec<_> = bls_committee
            .members
            .iter()
            .map(|member| member.validator_id)
            .collect();

        let voting_rights = bls_committee
            .members
            .iter()
            .filter(|v| committee_validator_ids.contains(&v.validator_id))
            .map(|v| {
                (
                    v.validator_id,
                    (
                        // AuthorityName is derived from the protocol public key;
                        // therefore, it is safe to unwrap.
                        (&AuthorityPublicKey::from_bytes(v.protocol_pubkey.clone().bytes.as_ref())
                            .unwrap())
                            .into(),
                        v.voting_power,
                    ),
                )
            })
            .collect();

        voting_rights
    }

    fn get_ika_active_committee(&self) -> HashMap<ObjectID, (AuthorityName, StakeUnit)> {
        self.read_bls_committee(&self.validators.active_committee)
    }

    fn get_ika_next_epoch_committee(
        &self,
    ) -> Option<HashMap<ObjectID, (AuthorityName, StakeUnit)>> {
        let Some(next_epoch_committee) = self.validators.next_epoch_committee.as_ref() else {
            return None;
        };
        Some(self.read_bls_committee(next_epoch_committee))
    }
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
