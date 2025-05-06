// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::{Element, ExtendedField, SystemInnerTrait};
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
}

/// Represents the current committee in the system.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct BlsCommittee {
    pub members: Vec<BlsCommitteeMember>,
    pub aggregated_protocol_pubkey: Element,
    pub quorum_threshold: u64,
    pub validity_threshold: u64,
}

pub type ObjectTable = Table;

/// Rust version of the Move ika_system::validator_set::ValidatorSet type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidatorSetV1 {
    pub total_stake: u64,
    pub validators: ObjectTable, // This now holds StakingPool objects
    pub active_committee: BlsCommittee,
    pub next_epoch_committee: Option<BlsCommittee>,
    pub previous_committee: BlsCommittee,
    pub pending_active_set: ExtendedField,
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

/// Rust version of the Move ika::ika_system::SystemInner type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SystemInnerV1 {
    pub epoch: u64,
    pub protocol_version: u64,
    pub upgrade_caps: Vec<UpgradeCap>,
    pub validator_set: ValidatorSetV1,
    pub parameters: SystemParametersV1,
    pub ika_treasury: IkaTreasuryV1,
    pub epoch_start_timestamp_ms: u64,
    pub total_messages_processed: u64,
    pub computation_reward: Balance,
    pub authorized_protocol_cap_ids: Vec<ObjectID>,
    pub dwallet_2pc_mpc_coordinator_id: Option<ObjectID>,
    pub dwallet_2pc_mpc_coordinator_network_decryption_keys: Vec<DWalletNetworkDecryptionKeyCap>,
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
    presign: PricingPerOperation,
    sign: PricingPerOperation,
    future_sign: PricingPerOperation,
    sign_with_partial_user_signature: PricingPerOperation,
}

/// Rust version of the Move DWalletCoordinatorInner type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DWalletCoordinatorInnerV1 {
    pub current_epoch: u64,
    pub sessions: ObjectTable,
    pub session_start_events: Bag,
    pub number_of_completed_sessions: u64,
    pub started_system_sessions_count: u64,
    pub completed_system_sessions_count: u64,
    pub next_session_sequence_number: u64,
    pub last_session_to_complete_in_current_epoch: u64,
    pub locked_last_session_to_complete_in_current_epoch: bool,
    pub max_active_sessions_buffer: u64,
    pub dwallets: ObjectTable,
    pub dwallet_network_decryption_keys: ObjectTable,
    pub encryption_keys: ObjectTable,
    pub presigns: ObjectTable,
    pub partial_centralized_signed_messages: ObjectTable,
    pub pricing: DWalletPricing2PcMpcSecp256K1,
    pub gas_fee_reimbursement_sui: Balance,
    pub consensus_validation_fee_charged_ika: Balance,
    pub active_committee: BlsCommittee,
    pub previous_committee: BlsCommittee,
    pub total_messages_processed: u64,
    pub last_processed_checkpoint_sequence_number: Option<u64>,
    pub previous_epoch_last_checkpoint_sequence_number: u64,
    pub supported_curves_to_signature_algorithms: VecMap<u8, Vec<u8>>,
    pub supported_signature_algorithms_to_hash_schemes: VecMap<u8, Vec<u8>>,
    pub paused_curves: Vec<u8>,
    pub paused_signature_algorithms: Vec<u8>,
    pub signature_algorithms_allowed_global_presign: Vec<u8>,
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

    fn protocol_version(&self) -> u64 {
        self.protocol_version
    }

    fn upgrade_caps(&self) -> &Vec<UpgradeCap> {
        &self.upgrade_caps
    }

    fn epoch_start_timestamp_ms(&self) -> u64 {
        self.epoch_start_timestamp_ms
    }

    fn epoch_duration_ms(&self) -> u64 {
        self.parameters.epoch_duration_ms
    }

    fn dwallet_2pc_mpc_coordinator_id(&self) -> Option<ObjectID> {
        self.dwallet_2pc_mpc_coordinator_id
    }

    fn dwallet_2pc_mpc_coordinator_network_decryption_keys(
        &self,
    ) -> &Vec<DWalletNetworkDecryptionKeyCap> {
        &self.dwallet_2pc_mpc_coordinator_network_decryption_keys
    }

    fn validator_set(&self) -> &ValidatorSetV1 {
        &self.validator_set
    }

    fn read_bls_committee(
        &self,
        bls_committee: &BlsCommittee,
    ) -> Vec<(ObjectID, (AuthorityName, StakeUnit))> {
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
                        1,
                    ),
                )
            })
            .collect();

        voting_rights
    }

    fn get_ika_active_committee(&self) -> Vec<(ObjectID, (AuthorityName, StakeUnit))> {
        self.read_bls_committee(&self.validator_set.active_committee)
    }

    fn get_ika_next_epoch_committee(&self) -> Option<Vec<(ObjectID, (AuthorityName, StakeUnit))>> {
        let Some(next_epoch_committee) = self.validator_set.next_epoch_committee.as_ref() else {
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

/// Rust version of the Move ika_system::dwallet_2pc_mpc_coordinator_inner::DWalletNetworkDecryptionKeyCap type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DWalletNetworkDecryptionKeyCap {
    pub id: ObjectID,
    pub dwallet_network_decryption_key_id: ObjectID,
}
