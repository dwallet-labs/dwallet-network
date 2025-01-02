// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use fastcrypto::encoding::{Base64, Encoding};
use fastcrypto::hash::HashFunction;
use ika_types::sui::ika_system_state::{
    IkaSystemState, IkaSystemStateTrait, IkaSystemStateWrapper, IkaValidatorInit,
};
use ika_types::{
    committee::{Committee, EpochId, ProtocolVersion},
    error::IkaResult,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fs, path::Path};
use sui_types::authenticator_state::{get_authenticator_state, AuthenticatorStateInner};
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::clock::Clock;
use sui_types::committee::CommitteeWithNetworkMetadata;
use sui_types::crypto::DefaultHash;
use sui_types::deny_list_v1::{get_coin_deny_list, PerTypeDenyList};
use sui_types::effects::{TransactionEffects, TransactionEvents};
use sui_types::messages_checkpoint::{
    CertifiedCheckpointSummary, CheckpointContents, CheckpointSummary, VerifiedCheckpoint,
};
use sui_types::object::Object;
use sui_types::storage::ObjectStore;
use sui_types::transaction::Transaction;
use tracing::trace;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InitiationParameters {
    /// protocol version that the chain starts at.
    #[serde(default = "InitiationParameters::default_protocol_version")]
    pub protocol_version: u64,

    #[serde(default = "InitiationParameters::default_chain_start_timestamp_ms")]
    pub chain_start_timestamp_ms: u64,

    /// The duration of an epoch, in milliseconds.
    #[serde(default = "InitiationParameters::default_epoch_duration_ms")]
    pub epoch_duration_ms: u64,

    // Stake Subsidy parameters
    /// The starting epoch in which stake subsidies start being paid out.
    #[serde(default = "InitiationParameters::default_stake_subsidy_start_epoch")]
    pub stake_subsidy_start_epoch: u64,

    /// The rate at which the amount per distribution is calculated based on
    /// period nad total supply. Expressed in basis points.
    #[serde(default = "InitiationParameters::default_stake_subsidy_rate")]
    pub stake_subsidy_rate: u16,

    /// Number of distributions to occur before the amount per distribution will be recalculated.
    #[serde(default = "InitiationParameters::default_stake_subsidy_period_length")]
    pub stake_subsidy_period_length: u64,

    // Validator committee parameters
    /// Minimum number of active validators at any moment.
    #[serde(default = "InitiationParameters::default_min_validator_count")]
    pub min_validator_count: u64,

    /// Maximum number of active validators at any moment.
    /// We do not allow the number of validators in any epoch to go above this.
    #[serde(default = "InitiationParameters::default_max_validator_count")]
    pub max_validator_count: u64,

    /// Lower-bound on the amount of stake required to become a validator.
    #[serde(default = "InitiationParameters::default_min_validator_joining_stake")]
    pub min_validator_joining_stake: u64,

    /// Validators with stake amount below `validator_low_stake_threshold` are considered to
    /// have low stake and will be escorted out of the validator set after being below this
    /// threshold for more than `validator_low_stake_grace_period` number of epochs.
    #[serde(default = "InitiationParameters::default_validator_low_stake_threshold")]
    pub validator_low_stake_threshold: u64,

    /// Validators with stake below `validator_very_low_stake_threshold` will be removed
    /// immediately at epoch change, no grace period.
    #[serde(default = "InitiationParameters::default_validator_very_low_stake_threshold")]
    pub validator_very_low_stake_threshold: u64,

    /// A validator can have stake below `validator_low_stake_threshold`
    /// for this many epochs before being kicked out.
    #[serde(default = "InitiationParameters::default_validator_low_stake_grace_period")]
    pub validator_low_stake_grace_period: u64,

    /// how many reward are slashed to punish a validator, in bps.
    #[serde(default = "InitiationParameters::default_reward_slashing_rate")]
    pub reward_slashing_rate: u64,
}

impl InitiationParameters {
    pub fn new() -> Self {
        Self {
            protocol_version: Self::default_protocol_version(),
            chain_start_timestamp_ms: Self::default_chain_start_timestamp_ms(),
            epoch_duration_ms: Self::default_epoch_duration_ms(),
            stake_subsidy_start_epoch: Self::default_stake_subsidy_start_epoch(),
            stake_subsidy_rate: Self::default_stake_subsidy_rate(),
            stake_subsidy_period_length: Self::default_stake_subsidy_period_length(),
            min_validator_count: Self::default_min_validator_count(),
            max_validator_count: Self::default_max_validator_count(),
            min_validator_joining_stake: Self::default_min_validator_joining_stake(),
            validator_low_stake_threshold: Self::default_validator_low_stake_threshold(),
            validator_very_low_stake_threshold: Self::default_validator_very_low_stake_threshold(),
            validator_low_stake_grace_period: Self::default_validator_low_stake_grace_period(),
            reward_slashing_rate: Self::default_reward_slashing_rate(),
        }
    }

    fn default_protocol_version() -> u64 {
        ProtocolVersion::MAX.as_u64()
    }

    fn default_chain_start_timestamp_ms() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    fn default_epoch_duration_ms() -> u64 {
        // 24 hrs
        24 * 60 * 60 * 1000
    }

    fn default_stake_subsidy_start_epoch() -> u64 {
        0
    }

    fn default_stake_subsidy_rate() -> u16 {
        // 10%
        1000
    }

    fn default_stake_subsidy_period_length() -> u64 {
        // 1 year
        365
    }

    fn default_min_validator_count() -> u64 {
        ika_types::governance::MIN_VALIDATOR_COUNT
    }

    fn default_max_validator_count() -> u64 {
        ika_types::governance::MAX_VALIDATOR_COUNT
    }

    fn default_min_validator_joining_stake() -> u64 {
        ika_types::governance::MIN_VALIDATOR_JOINING_STAKE_NIKA
    }

    fn default_validator_low_stake_threshold() -> u64 {
        ika_types::governance::VALIDATOR_LOW_STAKE_THRESHOLD_NIKA
    }

    fn default_validator_very_low_stake_threshold() -> u64 {
        ika_types::governance::VALIDATOR_VERY_LOW_STAKE_THRESHOLD_NIKA
    }

    fn default_validator_low_stake_grace_period() -> u64 {
        ika_types::governance::VALIDATOR_LOW_STAKE_GRACE_PERIOD
    }

    fn default_reward_slashing_rate() -> u64 {
        ika_types::governance::REWARD_SLASHING_RATE
    }
}

impl Default for InitiationParameters {
    fn default() -> Self {
        Self::new()
    }
}
