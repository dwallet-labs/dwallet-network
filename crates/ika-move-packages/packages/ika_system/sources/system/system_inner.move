// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::system_inner;

use ika::ika::IKA;
use ika_system::protocol_treasury::ProtocolTreasury;
use ika_system::token_exchange_rate::TokenExchangeRate;
use ika_system::staked_ika::{StakedIka};
use ika_system::validator_cap::{ValidatorCap, ValidatorOperationCap, ValidatorCommissionCap};
use ika_system::validator_set::{ValidatorSet};
use ika_system::bls_committee::{BlsCommittee};
use ika_system::protocol_cap::ProtocolCap;
use ika_system::validator_metadata::ValidatorMetadata;
use ika_system::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof;
use ika_system::dwallet_2pc_mpc_coordinator::{Self, DWalletCoordinator};
use ika_system::dwallet_2pc_mpc_coordinator_inner::{DWalletNetworkEncryptionKeyCap, DWalletCoordinatorInner};
use ika_system::dwallet_pricing::DWalletPricing;
use sui::bag::{Self, Bag};
use sui::balance::{Self, Balance};
use sui::coin::Coin;
use sui::event;
use sui::table::Table;
use sui::vec_set::{VecSet};
use sui::clock::Clock;
use sui::package::{UpgradeCap, UpgradeTicket, UpgradeReceipt};
use sui::bcs;
use std::string::String;
use sui::vec_map::{Self, VecMap};


const PARAMS_MESSAGE_INTENT: vector<u8> = vector[2, 0, 0];

// System checkpoint message data type constants corresponding to system parameters
// Note: the order of these fields, and the number must correspond to the Rust code in
// `crates/ika-types/src/messages_system_checkpoints.rs`.
const SET_NEXT_PROTOCOL_VERSION_MESSAGE_TYPE: u64 = 0;
const SET_EPOCH_DURATION_MS_MESSAGE_TYPE: u64 = 1;
const SET_STAKE_SUBSIDY_START_EPOCH_MESSAGE_TYPE: u64 = 2;
const SET_STAKE_SUBSIDY_RATE_MESSAGE_TYPE: u64 = 3;
const SET_STAKE_SUBSIDY_PERIOD_LENGTH_MESSAGE_TYPE: u64 = 4;
const SET_MIN_VALIDATOR_COUNT_MESSAGE_TYPE: u64 = 5;
const SET_MAX_VALIDATOR_COUNT_MESSAGE_TYPE: u64 = 6;
const SET_MIN_VALIDATOR_JOINING_STAKE_MESSAGE_TYPE: u64 = 7;
const SET_MAX_VALIDATOR_CHANGE_COUNT_MESSAGE_TYPE: u64 = 8;
const SET_REWARD_SLASHING_RATE_MESSAGE_TYPE: u64 = 9;
const SET_APPROVED_UPGRADE_MESSAGE_TYPE: u64 = 10;

/// Uses SystemParametersV1 as the parameters.
public struct SystemInnerV1 has store {
    /// The current epoch ID, starting from 0.
    epoch: u64,
    /// The current protocol version, starting from 1.
    protocol_version: u64,
    next_protocol_version: Option<u64>,
    /// Upgrade caps for this package and others like ika coin of the ika protocol.
    upgrade_caps: vector<UpgradeCap>,
    /// Approved upgrade for package id to its approved digest.
    approved_upgrades: VecMap<ID, vector<u8>>,
    /// Contains all information about the validators.
    validator_set: ValidatorSet,
    /// The duration of an epoch, in milliseconds.
    epoch_duration_ms: u64,
    /// The starting epoch in which stake subsidies start being paid out
    stake_subsidy_start_epoch: u64,
    /// Schedule of stake subsidies given out each epoch.
    protocol_treasury: ProtocolTreasury,
    /// Unix timestamp of the current epoch start.
    epoch_start_timestamp_ms: u64,
    /// The last processed checkpoint sequence number.
    last_processed_checkpoint_sequence_number: Option<u64>,
    /// The last checkpoint sequence number of the previous epoch.
    previous_epoch_last_checkpoint_sequence_number: u64,
    /// The total messages processed.
    total_messages_processed: u64,
    /// The fees paid for computation.
    remaining_rewards: Balance<IKA>,
    /// List of authorized protocol cap ids.
    authorized_protocol_cap_ids: vector<ID>,
    // TODO: maybe change that later
    dwallet_2pc_mpc_coordinator_id: Option<ID>,
    dwallet_2pc_mpc_coordinator_network_encryption_keys: vector<DWalletNetworkEncryptionKeyCap>,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

/// Event containing system-level epoch information, emitted during
/// the epoch advancement message.
public struct SystemEpochInfoEvent has copy, drop {
    epoch: u64,
    protocol_version: u64,
    total_stake: u64,
    stake_subsidy_amount: u64,
    total_computation_fees: u64,
    total_stake_rewards_distributed: u64,
}

/// Event emitted during verifying quorum checkpoint submission signature.
public struct SystemProtocolCapVerifiedEvent has copy, drop {
    epoch: u64,
    protocol_cap_id: ID,
}

/// Event containing system-level checkpoint information, emitted during
/// the system checkpoint submission message.
public struct SystemCheckpointInfoEvent has copy, drop {
    epoch: u64,
    sequence_number: u64,
    timestamp_ms: u64,
}

/// Event emitted when protocol version is set via checkpoint message.
public struct SetNextProtocolVersionEvent has copy, drop {
    epoch: u64,
    next_protocol_version: u64,
}

/// Event emitted when epoch duration is set via checkpoint message.
public struct SetEpochDurationMsEvent has copy, drop {
    epoch: u64,
    epoch_duration_ms: u64,
}

/// Event emitted when stake subsidy start epoch is set via checkpoint message.
public struct SetStakeSubsidyStartEpochEvent has copy, drop {
    epoch: u64,
    stake_subsidy_start_epoch: u64,
}

/// Event emitted when stake subsidy rate is set via checkpoint message.
public struct SetStakeSubsidyRateEvent has copy, drop {
    epoch: u64,
    stake_subsidy_rate: u16,
}

/// Event emitted when stake subsidy period length is set via checkpoint message.
public struct SetStakeSubsidyPeriodLengthEvent has copy, drop {
    epoch: u64,
    stake_subsidy_period_length: u64,
}

/// Event emitted when minimum validator count is set via checkpoint message.
public struct SetMinValidatorCountEvent has copy, drop {
    epoch: u64,
    min_validator_count: u64,
}

/// Event emitted when maximum validator count is set via checkpoint message.
public struct SetMaxValidatorCountEvent has copy, drop {
    epoch: u64,
    max_validator_count: u64,
}

/// Event emitted when minimum validator joining stake is set via checkpoint message.
public struct SetMinValidatorJoiningStakeEvent has copy, drop {
    epoch: u64,
    min_validator_joining_stake: u64,
}

/// Event emitted when maximum validator change count is set via checkpoint message.
public struct SetMaxValidatorChangeCountEvent has copy, drop {
    epoch: u64,
    max_validator_change_count: u64,
}

/// Event emitted when reward slashing rate is set via checkpoint message.
public struct SetRewardSlashingRateEvent has copy, drop {
    epoch: u64,
    reward_slashing_rate: u16,
}

/// Event emitted when approved upgrade is set via checkpoint message.
public struct SetApprovedUpgradeEvent has copy, drop {
    epoch: u64,
    package_id: ID,
    digest: Option<vector<u8>>,
}

// Errors
const ENextCommitteeNotSetOnAdvanceEpoch: u64 = 0;
const EHaveNotReachedEndEpochTime: u64 = 1;
const EActiveBlsCommitteeMustInitialize: u64 = 2;
const EIncorrectEpochInIkaSystemCheckpoint: u64 = 3;
const EWrongIkaSystemCheckpointSequenceNumber: u64 = 4;
const EApprovedUpgradeNotFound: u64 = 5;

#[error]
const EUnauthorizedProtocolCap: vector<u8> = b"The protocol cap is unauthorized.";

#[error]
const ECannotInitialize: vector<u8> = b"Too early for initialization time or already initialized.";

#[error]
const EWrongEpochState: vector<u8> = b"The system is in the wrong epoch state for the operation.";

#[error]
const EHaveNotReachedMidEpochTime: vector<u8> = b"The system has not reached the mid epoch time.";

// ==== functions that can only be called by init ====

/// Create a new IkaSystemState object and make it shared.
/// This function will be called only once in init.
public(package) fun create(
    upgrade_caps: vector<UpgradeCap>,
    validator_set: ValidatorSet,
    protocol_version: u64,
    epoch_start_timestamp_ms: u64,
    epoch_duration_ms: u64,
    stake_subsidy_start_epoch: u64,
    protocol_treasury: ProtocolTreasury,
    authorized_protocol_cap_ids: vector<ID>,
    ctx: &mut TxContext,
): SystemInnerV1 {
    // This type is fixed as it's created at init. It should not be updated during type upgrade.
    let system_state = SystemInnerV1 {
        epoch: 0,
        protocol_version,
        next_protocol_version: option::none(),
        upgrade_caps,
        approved_upgrades: vec_map::empty(),
        validator_set,
        epoch_duration_ms,
        stake_subsidy_start_epoch,
        protocol_treasury,
        epoch_start_timestamp_ms,
        last_processed_checkpoint_sequence_number: option::none(),
        previous_epoch_last_checkpoint_sequence_number: 0,
        total_messages_processed: 0,
        remaining_rewards: balance::zero(),
        authorized_protocol_cap_ids,
        dwallet_2pc_mpc_coordinator_id: option::none(),
        dwallet_2pc_mpc_coordinator_network_encryption_keys: vector[],
        extra_fields: bag::new(ctx),
    };
    system_state
}

// ==== public(package) functions ====

public(package) fun initialize(
    self: &mut SystemInnerV1,
    pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap<u32, VecMap<u32, vector<u32>>>,
    max_validator_change_count: u64,
    package_id: ID,
    cap: &ProtocolCap,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    self.verify_cap(cap);
    let now = clock.timestamp_ms();
    assert!(self.epoch == 0 && now >= self.epoch_start_timestamp_ms, ECannotInitialize);
    assert!(self.active_committee().members().is_empty(), ECannotInitialize);
    let pending_active_set = self.validator_set.pending_active_set();
    assert!(pending_active_set.size() >= pending_active_set.min_validator_count(), ECannotInitialize);
    self.validator_set.set_max_validator_change_count(max_validator_change_count);

    self.validator_set.process_mid_epoch();
    let mut dwallet_2pc_mpc_coordinator = dwallet_2pc_mpc_coordinator::create_dwallet_coordinator(package_id, self.epoch, self.active_committee(), pricing, supported_curves_to_signature_algorithms_to_hash_schemes, ctx);
    let dwallet_2pc_mpc_coordinator_inner = dwallet_2pc_mpc_coordinator.inner_mut();
    self.advance_epoch(dwallet_2pc_mpc_coordinator_inner, clock, ctx);

    self.dwallet_2pc_mpc_coordinator_id.fill(object::id(&dwallet_2pc_mpc_coordinator));
    dwallet_2pc_mpc_coordinator.share_dwallet_coordinator();
}

/// Can be called by anyone who wishes to become a validator candidate and starts accusing delegated
/// stakes in their staking pool. Once they have at least `MIN_VALIDATOR_JOINING_STAKE` amount of stake they
/// can call `request_add_validator` to officially become an active validator at the next epoch.
/// Aborts if the caller is already a pending or active validator, or a validator candidate.
/// Note: `proof_of_possession_bytes` MUST be a valid signature using proof_of_possession_sender and protocol_pubkey_bytes.
/// To produce a valid PoP, run [fn test_proof_of_possession_bytes].
public(package) fun request_add_validator_candidate(
    self: &mut SystemInnerV1,
    name: String,
    protocol_pubkey_bytes: vector<u8>,
    network_pubkey_bytes: vector<u8>,
    consensus_pubkey_bytes: vector<u8>,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    proof_of_possession_bytes: vector<u8>,
    network_address: String,
    p2p_address: String,
    consensus_address: String,
    commission_rate: u16,
    metadata: ValidatorMetadata,
    ctx: &mut TxContext,
): (ValidatorCap, ValidatorOperationCap, ValidatorCommissionCap) {
    self.validator_set.request_add_validator_candidate(
        self.epoch,
        name,
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        class_groups_pubkey_and_proof_bytes,
        proof_of_possession_bytes,
        network_address,
        p2p_address,
        consensus_address,
        commission_rate,
        metadata,
        ctx,
    )
}

/// Called by a validator candidate to remove themselves from the candidacy. After this call
/// their staking pool becomes deactivate.
public(package) fun request_remove_validator_candidate(
    self: &mut SystemInnerV1,
    cap: &ValidatorCap,
) {
    self.validator_set.request_remove_validator_candidate(self.epoch, cap);
}

/// Called by a validator candidate to add themselves to the active validator set beginning next epoch.
/// Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
/// stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
/// epoch has already reached the maximum.
public(package) fun request_add_validator(
    self: &mut SystemInnerV1,
    cap: &ValidatorCap,
) {
    self.validator_set.request_add_validator(self.epoch, cap);
}

/// A validator can call this function to request a removal in the next epoch.
/// We use the sender of `ctx` to look up the validator
/// (i.e. sender must match the sui_address in the validator).
/// At the end of the epoch, the `validator` object will be returned to the sui_address
/// of the validator.
public(package) fun request_remove_validator(
    self: &mut SystemInnerV1,
    cap: &ValidatorCap,
) {
    self.validator_set.request_remove_validator(self.epoch, cap);
}


public(package) fun set_validator_metadata(
    self: &mut SystemInnerV1,
    cap: &ValidatorOperationCap,
    metadata: ValidatorMetadata,
) {
    self.validator_set.set_validator_metadata(cap, metadata);
}

/// A validator can call this function to set a new commission rate, updated at the end of
/// the epoch.
public(package) fun set_next_commission(
    self: &mut SystemInnerV1,
    new_commission_rate: u16,
    cap: &ValidatorOperationCap,
) {
    self
        .validator_set
        .set_next_commission(
            new_commission_rate,
            cap,
            self.epoch,
        );
}

/// Add stake to a validator's staking pool.
public(package) fun request_add_stake(
    self: &mut SystemInnerV1,
    stake: Coin<IKA>,
    validator_id: ID,
    ctx: &mut TxContext,
): StakedIka {
    self
        .validator_set
        .request_add_stake(
            self.epoch,
            validator_id,
            stake.into_balance(),
            ctx,
        )
}

/// Withdraw some portion of a stake from a validator's staking pool.
public(package) fun request_withdraw_stake(
    self: &mut SystemInnerV1,
    staked_ika: &mut StakedIka,
) {
    self.validator_set.request_withdraw_stake(staked_ika, self.epoch);
}

public(package) fun withdraw_stake(
    self: &mut SystemInnerV1,
    staked_ika: StakedIka,
    ctx: &mut TxContext,
): Coin<IKA> {
    self.validator_set.withdraw_stake(staked_ika, self.epoch, ctx)
}

// public(package) fun convert_to_fungible_staked_ika(
//     self: &mut SystemInnerV1,
//     staked_ika: StakedIka,
//     ctx: &mut TxContext,
// ): FungibleStakedIka {
//     self.validators.convert_to_fungible_staked_ika(self.epoch, staked_ika, ctx)
// }

// public(package) fun redeem_fungible_staked_ika(
//     self: &mut SystemInnerV1,
//     fungible_staked_ika: FungibleStakedIka,
// ): Balance<IKA> {
//     self.validators.redeem_fungible_staked_ika(self.epoch, fungible_staked_ika)
// }

public(package) fun report_validator(
    self: &mut SystemInnerV1,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.validator_set.report_validator(cap, reportee_id);
}

public(package) fun undo_report_validator(
    self: &mut SystemInnerV1,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.validator_set.undo_report_validator(cap, reportee_id);
}

// ==== validator metadata management functions ====

/// Create a new `ValidatorOperationCap` and registers it.
/// The original object is thus revoked.
public(package) fun rotate_operation_cap(self: &mut SystemInnerV1, cap: &ValidatorCap, ctx: &mut TxContext): ValidatorOperationCap {
    self.validator_set.rotate_operation_cap(cap, ctx)
}

public(package) fun rotate_commission_cap(self: &mut SystemInnerV1, cap: &ValidatorCap, ctx: &mut TxContext): ValidatorCommissionCap {
    self.validator_set.rotate_commission_cap(cap, ctx)
}

/// Sets a validator's name.
public(package) fun set_validator_name(
    self: &mut SystemInnerV1,
    name: String,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_validator_name(name, cap);
}

/// Sets a validator's network address.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_network_address(
    self: &mut SystemInnerV1,
    network_address: String,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_next_epoch_network_address(network_address, cap);
}

/// Sets a validator's p2p address.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_p2p_address(
    self: &mut SystemInnerV1,
    p2p_address: String,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_next_epoch_p2p_address(p2p_address, cap);
}

/// Sets a validator's consensus address.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_consensus_address(
    self: &mut SystemInnerV1,
    consensus_address: String,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_next_epoch_consensus_address(consensus_address, cap);
}


/// Sets a validator's public key of protocol key and proof of possession.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_protocol_pubkey_bytes(
    self: &mut SystemInnerV1,
    protocol_pubkey_bytes: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    cap: &ValidatorOperationCap,
    ctx: &TxContext,
) {
    self.validator_set.set_next_epoch_protocol_pubkey_bytes(protocol_pubkey_bytes, proof_of_possession_bytes, cap, ctx);
}

/// Sets a validator's public key of network key.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_network_pubkey_bytes(
    self: &mut SystemInnerV1,
    network_pubkey_bytes: vector<u8>,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_next_epoch_network_pubkey_bytes(network_pubkey_bytes, cap);
}

/// Sets a validator's public key of worker key.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_consensus_pubkey_bytes(
    self: &mut SystemInnerV1,
    consensus_pubkey_bytes: vector<u8>,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_next_epoch_consensus_pubkey_bytes(consensus_pubkey_bytes, cap);
}


/// Sets a validator's public key and its associated proof of class groups key.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_class_groups_pubkey_and_proof_bytes(
    self: &mut SystemInnerV1,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_next_epoch_class_groups_pubkey_and_proof_bytes(class_groups_pubkey_and_proof_bytes, cap);
}

/// Sets a validator's pricing vote.
/// The change will only take effects starting from the next epoch.
public(package) fun set_pricing_vote(
    self: &mut SystemInnerV1,
    dwallet_coordinator_inner: &mut DWalletCoordinatorInner,
    pricing: DWalletPricing,
    cap: &ValidatorOperationCap,
) {
    self.validator_set.set_pricing_vote(dwallet_coordinator_inner, pricing, cap);
}

/// This function should be called at the end of an epoch, and advances the system to the next epoch.
/// It does the following things:
/// 1. Add storage charge to the storage fund.
/// 2. Burn the storage rebates from the storage fund. These are already refunded to transaction sender's
///    gas coins.
/// 3. Distribute computation charge to validator stake.
/// 4. Update all validators.
public(package) fun advance_epoch(
    self: &mut SystemInnerV1,
    dwallet_coordinator: &mut DWalletCoordinatorInner,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    let now = clock.timestamp_ms();
    let last_epoch_change = self.epoch_start_timestamp_ms;

    if (self.epoch == 0) assert!(now >= last_epoch_change, EWrongEpochState)
    else assert!(now >= last_epoch_change + self.epoch_duration_ms, EWrongEpochState);
    self.epoch_start_timestamp_ms = now;

    let mut stake_subsidy = balance::zero();

    // during the transition from epoch N to epoch N + 1, self.epoch() will return N
    let current_epoch = self.epoch();
    // Include stake subsidy in the rewards given out to validators and stakers.
    // Delay distributing any stake subsidies until after `stake_subsidy_start_epoch`.
    // And if this epoch is shorter than the regular epoch duration, don't distribute any stake subsidy.
    if (
        current_epoch >= self.stake_subsidy_start_epoch
    ) {
        stake_subsidy.join(self.protocol_treasury.stake_subsidy_for_distribution(ctx));
    };

    let stake_subsidy_amount = stake_subsidy.value();

    let dwallet_computation_and_consensus_validation_rewards = dwallet_coordinator.advance_epoch(self.next_epoch_active_committee(), &self.dwallet_2pc_mpc_coordinator_network_encryption_keys);

    let total_computation_fees = dwallet_computation_and_consensus_validation_rewards.value();

    let mut total_reward = sui::balance::zero<IKA>();
    total_reward.join(dwallet_computation_and_consensus_validation_rewards);
    total_reward.join(stake_subsidy);
    total_reward.join(self.remaining_rewards.withdraw_all());


    let total_reward_amount_before_distribution = total_reward.value();
    let new_epoch = current_epoch + 1;
    self.epoch = new_epoch;
    if (self.next_protocol_version.is_some()) {
        self.protocol_version = self.next_protocol_version.extract();
    };

    self
        .validator_set
        .advance_epoch(
            new_epoch,
            &mut total_reward,
        );

    let new_total_stake = self.validator_set.total_stake();

    let total_reward_amount_after_distribution = total_reward.value();
    let total_reward_distributed =
        total_reward_amount_before_distribution - total_reward_amount_after_distribution;

    // Because of precision issues with integer divisions, we expect that there will be some
    // remaining balance in `remaining_rewards`.
    self.remaining_rewards.join(total_reward);

    event::emit(SystemEpochInfoEvent {
        epoch: self.epoch,
        protocol_version: self.protocol_version,
        total_stake: new_total_stake,
        stake_subsidy_amount,
        total_computation_fees,
        total_stake_rewards_distributed: total_reward_distributed,
    });
}

public(package) fun process_mid_epoch(
    self: &mut SystemInnerV1,
    clock: &Clock,
    dwallet_coordinator_inner: &mut DWalletCoordinatorInner,
    ctx: &mut TxContext,
) {
    assert!(self.validator_set.next_epoch_active_committee().is_none() && clock.timestamp_ms() > self.epoch_start_timestamp_ms + (self.epoch_duration_ms / 2), EHaveNotReachedMidEpochTime);

    self.validator_set.process_mid_epoch();
    let next_epoch_active_committee = self.validator_set.next_epoch_active_committee().extract();
    dwallet_coordinator_inner.mid_epoch_reconfiguration(next_epoch_active_committee, &self.dwallet_2pc_mpc_coordinator_network_encryption_keys, ctx);
}

public(package) fun lock_last_active_session_sequence_number(
    self: &SystemInnerV1,
    dwallet_coordinator: &mut DWalletCoordinatorInner,
    clock: &Clock,
) {
    assert!(clock.timestamp_ms() > self.epoch_start_timestamp_ms + (self.epoch_duration_ms()), EHaveNotReachedEndEpochTime);
    dwallet_coordinator.lock_last_active_session_sequence_number();
}

/// Return the current epoch number. Useful for applications that need a coarse-grained concept of time,
/// since epochs are ever-increasing and epoch changes are intended to happen every 24 hours.
public(package) fun epoch(self: &SystemInnerV1): u64 {
    self.epoch
}

public(package) fun protocol_version(self: &SystemInnerV1): u64 {
    self.protocol_version
}

public(package) fun upgrade_caps(self: &SystemInnerV1): &vector<UpgradeCap> {
    &self.upgrade_caps
}

/// Returns unix timestamp of the start of current epoch
public(package) fun epoch_start_timestamp_ms(self: &SystemInnerV1): u64 {
    self.epoch_start_timestamp_ms
}

/// Returns the total amount staked with `validator_id`.
/// Aborts if `validator_id` is not an active validator.
public(package) fun validator_stake_amount(
    self: &mut SystemInnerV1,
    validator_id: ID,
): u64 {
    self.validator_set.validator_total_stake_amount(validator_id)
}

/// Returns all the validators who are currently reporting `validator_id`
public(package) fun get_reporters_of(self: &SystemInnerV1, validator_id: ID): VecSet<ID> {
    self.get_reporters_of(validator_id)
}

public(package) fun token_exchange_rates(
    self: &SystemInnerV1,
    validator_id: ID,
): &Table<u64, TokenExchangeRate> {
    let validators = &self.validator_set;
    validators.token_exchange_rates(validator_id)
}

public(package) fun active_committee(self: &SystemInnerV1): BlsCommittee {
    let validator_set = &self.validator_set;
    validator_set.active_committee()
}

public(package) fun next_epoch_active_committee(self: &SystemInnerV1): BlsCommittee {
    let next_epoch_committee = self.validator_set.next_epoch_active_committee();
    assert!(next_epoch_committee.is_some(), ENextCommitteeNotSetOnAdvanceEpoch);
    return *next_epoch_committee.borrow()
}

fun verify_cap(
    self: &SystemInnerV1,
    cap: &ProtocolCap,
) {
    let protocol_cap_id = object::id(cap);

    assert!(self.authorized_protocol_cap_ids.contains(&protocol_cap_id), EUnauthorizedProtocolCap);

    event::emit(SystemProtocolCapVerifiedEvent {
        epoch: self.epoch,
        protocol_cap_id: object::id(cap),
    });
}

public(package) fun request_dwallet_network_encryption_key_dkg_by_cap(
    self: &mut SystemInnerV1,
    dwallet_2pc_mpc_coordinator: &mut DWalletCoordinator,
    cap: &ProtocolCap,
    ctx: &mut TxContext,
) {
    self.verify_cap(cap);
    let key_cap = dwallet_2pc_mpc_coordinator.request_dwallet_network_encryption_key_dkg(ctx);
    self.dwallet_2pc_mpc_coordinator_network_encryption_keys.push_back(key_cap);
}

public(package) fun set_supported_and_pricing(
    self: &SystemInnerV1,
    dwallet_2pc_mpc_coordinator_inner: &mut DWalletCoordinatorInner,
    default_pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap<u32, VecMap<u32, vector<u32>>>,
    protocol_cap: &ProtocolCap,
) {
    self.verify_cap(protocol_cap);
    dwallet_2pc_mpc_coordinator_inner.set_supported_and_pricing(default_pricing, supported_curves_to_signature_algorithms_to_hash_schemes);
}

public(package) fun set_paused_curves_and_signature_algorithms(
    self: &SystemInnerV1,
    dwallet_2pc_mpc_coordinator_inner: &mut DWalletCoordinatorInner,
    paused_curves: vector<u32>,
    paused_signature_algorithms: vector<u32>,
    paused_hash_schemes: vector<u32>,
    protocol_cap: &ProtocolCap,
) {
    self.verify_cap(protocol_cap);
    dwallet_2pc_mpc_coordinator_inner.set_paused_curves_and_signature_algorithms(paused_curves, paused_signature_algorithms, paused_hash_schemes);
}

public(package) fun authorize_upgrade_by_cap(
    self: &mut SystemInnerV1,
    cap: &ProtocolCap,
    package_id: ID,
    digest: vector<u8>,
): UpgradeTicket {
    self.verify_cap(cap);

    self.authorize_upgrade(package_id, digest)
}

public(package) fun authorize_upgrade_by_approval(
    self: &mut SystemInnerV1,
    package_id: ID,
): UpgradeTicket {
    assert!(self.approved_upgrades.contains(&package_id), EApprovedUpgradeNotFound);
    let (_, digest) = self.approved_upgrades.remove(&package_id);
    self.authorize_upgrade(package_id, digest)
}

fun authorize_upgrade(
    self: &mut SystemInnerV1,
    package_id: ID,
    digest: vector<u8>,
): UpgradeTicket  {
    let index = self.upgrade_caps.find_index!(|c| c.package() == package_id).extract();
    let policy = self.upgrade_caps[index].policy();
    self.upgrade_caps[index].authorize(policy, digest)
}

public(package) fun commit_upgrade(
    self: &mut SystemInnerV1,
    receipt: UpgradeReceipt,
): ID {
    let receipt_cap_id = receipt.cap();
    let index = self.upgrade_caps.find_index!(|c| object::id(c) == receipt_cap_id).extract();
    let old_package_id = self.upgrade_caps[index].package();
    self.upgrade_caps[index].commit(receipt);
    old_package_id
}

public(package) fun process_checkpoint_message_by_cap(
    self: &mut SystemInnerV1,
    cap: &ProtocolCap,
    message: vector<u8>,
    ctx: &mut TxContext,
)  {
    self.verify_cap(cap);
    self.process_checkpoint_message(message, ctx);
}

public(package) fun process_checkpoint_message_by_quorum(
    self: &mut SystemInnerV1,
    signature: vector<u8>,
    signers_bitmap: vector<u8>,
    message: vector<u8>,
    ctx: &mut TxContext,
) {
    let active_committee = self.validator_set.active_committee();
    assert!(!active_committee.members().is_empty(), EActiveBlsCommitteeMustInitialize);

    let mut intent_bytes = PARAMS_MESSAGE_INTENT;
    intent_bytes.append(message);
    intent_bytes.append(bcs::to_bytes(&self.epoch));

    active_committee.verify_certificate(self.epoch, &signature, &signers_bitmap, &intent_bytes);

    self.process_checkpoint_message(message, ctx);
}

public(package) fun process_checkpoint_message(self: &mut SystemInnerV1, message: vector<u8>, _ctx: &mut TxContext) {
    let mut bcs_body = bcs::new(copy message);

    let epoch = bcs_body.peel_u64();
    assert!(epoch == self.epoch, EIncorrectEpochInIkaSystemCheckpoint);

    let sequence_number = bcs_body.peel_u64();

    if(self.last_processed_checkpoint_sequence_number.is_none()) {
        assert!(sequence_number == 0, EWrongIkaSystemCheckpointSequenceNumber);
        self.last_processed_checkpoint_sequence_number.fill(sequence_number);
    } else {
        assert!(sequence_number > 0 && *self.last_processed_checkpoint_sequence_number.borrow() + 1 == sequence_number, EWrongIkaSystemCheckpointSequenceNumber);
        self.last_processed_checkpoint_sequence_number.swap(sequence_number);
    };

    let timestamp_ms = bcs_body.peel_u64();

    event::emit(SystemCheckpointInfoEvent {
        epoch,
        sequence_number,
        timestamp_ms,
    });

    let len = bcs_body.peel_vec_length();
    let mut i = 0;
    // Note: the order of these fields, and the number must correspond to the Rust code in
    // `crates/ika-types/src/messages_system_checkpoints.rs`.
    while (i < len) {
        let message_data_type = bcs_body.peel_vec_length();
        // Parses params message BCS bytes directly.
        match (message_data_type) {
            SET_NEXT_PROTOCOL_VERSION_MESSAGE_TYPE => {
                let next_protocol_version = bcs_body.peel_u64();
                self.next_protocol_version.fill(next_protocol_version);
                event::emit(SetNextProtocolVersionEvent {
                    epoch: self.epoch,
                    next_protocol_version,
                });
            },
            SET_EPOCH_DURATION_MS_MESSAGE_TYPE => {
                let epoch_duration_ms = bcs_body.peel_u64();
                self.epoch_duration_ms = epoch_duration_ms;
                event::emit(SetEpochDurationMsEvent {
                    epoch: self.epoch,
                    epoch_duration_ms,
                });
            },
            SET_STAKE_SUBSIDY_START_EPOCH_MESSAGE_TYPE => {
                let stake_subsidy_start_epoch = bcs_body.peel_u64();
                self.stake_subsidy_start_epoch = stake_subsidy_start_epoch;
                event::emit(SetStakeSubsidyStartEpochEvent {
                    epoch: self.epoch,
                    stake_subsidy_start_epoch,
                });
            },
            SET_STAKE_SUBSIDY_RATE_MESSAGE_TYPE => {
                let stake_subsidy_rate = bcs_body.peel_u16();
                self.protocol_treasury.set_stake_subsidy_rate(stake_subsidy_rate);
                event::emit(SetStakeSubsidyRateEvent {
                    epoch: self.epoch,
                    stake_subsidy_rate,
                });
            },
            SET_STAKE_SUBSIDY_PERIOD_LENGTH_MESSAGE_TYPE => {
                let stake_subsidy_period_length = bcs_body.peel_u64();
                self.protocol_treasury.set_stake_subsidy_period_length(stake_subsidy_period_length);
                event::emit(SetStakeSubsidyPeriodLengthEvent {
                    epoch: self.epoch,
                    stake_subsidy_period_length,
                });
            },
            SET_MIN_VALIDATOR_COUNT_MESSAGE_TYPE => {
                let min_validator_count = bcs_body.peel_u64();
                self.validator_set.set_min_validator_count(min_validator_count);
                event::emit(SetMinValidatorCountEvent {
                    epoch: self.epoch,
                    min_validator_count,
                });
            },
            SET_MAX_VALIDATOR_COUNT_MESSAGE_TYPE => {
                let max_validator_count = bcs_body.peel_u64();
                self.validator_set.set_max_validator_count(max_validator_count);
                event::emit(SetMaxValidatorCountEvent {
                    epoch: self.epoch,
                    max_validator_count,
                });
            },
            SET_MIN_VALIDATOR_JOINING_STAKE_MESSAGE_TYPE => {
                let min_validator_joining_stake = bcs_body.peel_u64();
                self.validator_set.set_min_validator_joining_stake(min_validator_joining_stake);
                event::emit(SetMinValidatorJoiningStakeEvent {
                    epoch: self.epoch,
                    min_validator_joining_stake,
                });
            },
            SET_MAX_VALIDATOR_CHANGE_COUNT_MESSAGE_TYPE => {
                let max_validator_change_count = bcs_body.peel_u64();
                self.validator_set.set_max_validator_change_count(max_validator_change_count);
                event::emit(SetMaxValidatorChangeCountEvent {
                    epoch: self.epoch,
                    max_validator_change_count,
                });
            },
            SET_REWARD_SLASHING_RATE_MESSAGE_TYPE => {
                let reward_slashing_rate = bcs_body.peel_u16();
                self.validator_set.set_reward_slashing_rate(reward_slashing_rate);
                event::emit(SetRewardSlashingRateEvent {
                    epoch: self.epoch,
                    reward_slashing_rate,
                });
            },
            SET_APPROVED_UPGRADE_MESSAGE_TYPE => {
                let package_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let digest = bcs_body.peel_option!(|bcs| bcs.peel_vec_u8());
                self.set_approved_upgrade(package_id, digest);
                event::emit(SetApprovedUpgradeEvent {
                    epoch: self.epoch,
                    package_id,
                    digest,
                });
            },
            _ => {
                // Unknown message type - skip
            }
        };
        i = i + 1;
    };
    self.total_messages_processed = self.total_messages_processed + i;
}

/// Set approved upgrade for a package id.
/// If `digest` is `some`, it will be inserted into the `approved_upgrades` map.
/// If `digest` is `none`, it will be removed from the `approved_upgrades` map.
fun set_approved_upgrade(
    self: &mut SystemInnerV1,
    package_id: ID,
    mut digest: Option<vector<u8>>,
) {
    if(digest.is_some()) {
        if(self.approved_upgrades.contains(&package_id)) {
            *self.approved_upgrades.get_mut(&package_id) = digest.extract();
        } else {
            self.approved_upgrades.insert(package_id, digest.extract());
        }
    } else {
        if(self.approved_upgrades.contains(&package_id)) {
            self.approved_upgrades.remove(&package_id);
        }
    }
}

// === Utility functions ===

/// Calculate the rewards for an amount with value `staked_principal`, staked in the validator with
/// the given `validator_id` between `activation_epoch` and `withdraw_epoch`.
public(package) fun calculate_rewards(
    self: &SystemInnerV1,
    node_id: ID,
    staked_principal: u64,
    activation_epoch: u64,
    withdraw_epoch: u64,
): u64 {
    self.validator_set.calculate_rewards(node_id, staked_principal, activation_epoch, withdraw_epoch)
}

/// Check whether StakedIka can be withdrawn directly.
public(package) fun can_withdraw_staked_ika_early(self: &SystemInnerV1, staked_ika: &StakedIka): bool {
    self.validator_set.can_withdraw_staked_ika_early(staked_ika, self.epoch)
}


#[test_only]
/// Return the current validator set
public(package) fun validator_set(self: &SystemInnerV1): &ValidatorSet {
    &self.validator_set
}

#[test_only]
public(package) fun get_stake_subsidy_stake_subsidy_distribution_counter(self: &SystemInnerV1): u64 {
    self.protocol_treasury.get_stake_subsidy_distribution_counter()
}

#[test_only]
public(package) fun set_epoch_for_testing(self: &mut SystemInnerV1, epoch_num: u64) {
    self.epoch = epoch_num
}

#[test_only]
public(package) fun request_add_validator_for_testing(
    self: &mut SystemInnerV1,
    cap: &ValidatorCap,
) {
    self.validator_set.request_add_validator(self.epoch, cap);
}

#[test_only]
public(package) fun set_stake_subsidy_stake_subsidy_distribution_counter(
    self: &mut SystemInnerV1,
    counter: u64,
) {
    self.protocol_treasury.set_stake_subsidy_distribution_counter(counter)
}

public(package) fun epoch_duration_ms(self: &SystemInnerV1): u64 {
    self.epoch_duration_ms
}

// // CAUTION: THIS CODE IS ONLY FOR TESTING AND THIS MACRO MUST NEVER EVER BE REMOVED.  Creates a
// // candidate validator - bypassing the proof of possession check and other metadata validation
// // in the process.
// #[test_only]
// public(package) fun request_add_validator_candidate_for_testing(
//     self: &mut SystemInnerV1,
//     protocol_pubkey_bytes: vector<u8>,
//     network_pubkey_bytes: vector<u8>,
//     consensus_pubkey_bytes_bytes: vector<u8>,
//     class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
//     proof_of_possession_bytes: vector<u8>,
//     name: vector<u8>,
//     description: vector<u8>,
//     image_url: vector<u8>,
//     project_url: vector<u8>,
//     network_address: vector<u8>,
//     p2p_address: vector<u8>,
//     consensus_address: vector<u8>,
//     computation_price: u64,
//     commission_rate: u16,
//     ctx: &mut TxContext,
// ): (ValidatorCap, ValidatorOperationCap) {
//     let (validator, cap, operation_cap) = validator_inner_v1::new_for_testing(
//         ctx.sender(),
//         protocol_pubkey_bytes,
//         network_pubkey_bytes,
//         consensus_pubkey_bytes_bytes,
//         class_groups_pubkey_and_proof_bytes,
//         proof_of_possession_bytes,
//         name,
//         description,
//         image_url,
//         project_url,
//         network_address,
//         p2p_address,
//         consensus_address,
//         option::none(),
//         computation_price,
//         commission_rate,
//         false, // not an initial validator active at init
//         ctx,
//     );

//     self.validators.request_add_validator_candidate(validator, ctx);
//     (cap, operation_cap)
// }
