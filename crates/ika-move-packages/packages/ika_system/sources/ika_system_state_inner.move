// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module ika_system::ika_system_state_inner;

use ika::ika::IKA;
use ika_system::ika_treasury::IkaTreasury;
use ika_system::staking_pool::{StakedIka, FungibleStakedIka, PoolTokenExchangeRate};
use ika_system::storage_fund::{Self, StorageFund};
use ika_system::validator::{Self, Validator};
use ika_system::validator_cap::{UnverifiedValidatorOperationCap, ValidatorOperationCap};
use ika_system::validator_set::{Self, ValidatorSet};
use ika_system::common;
use sui::bag::{Self, Bag};
use sui::balance::{Self, Balance};
use sui::coin::Coin;
use sui::event;
use sui::table::Table;
use sui::vec_map::{Self, VecMap};
use sui::vec_set::{Self, VecSet};
use sui::bcs;


// same as in validator_set
const ACTIVE_VALIDATOR_ONLY: u8 = 1;
const ACTIVE_OR_PENDING_VALIDATOR: u8 = 2;
const ANY_VALIDATOR: u8 = 3;

const SYSTEM_STATE_VERSION_V1: u64 = 1;

const INTENT_APP_ID: u8 = 0;
const INTENT_VERSION: u8 = 0;
const CHECKPOINT_MESSAGE_INTENT_SCOPE: u8 = 1;

const BASIS_POINT_DENOMINATOR: u128 = 10000;

/// Added min_validator_count.
public struct SystemParametersV1 has store {
    /// The duration of an epoch, in milliseconds.
    epoch_duration_ms: u64,
    /// The starting epoch in which stake subsidies start being paid out
    stake_subsidy_start_epoch: u64,
    /// Minimum number of active validators at any moment.
    min_validator_count: u64,
    /// Maximum number of active validators at any moment.
    /// We do not allow the number of validators in any epoch to go above this.
    max_validator_count: u64,
    /// Lower-bound on the amount of stake required to become a validator.
    min_validator_joining_stake: u64,
    /// Validators with stake amount below `validator_low_stake_threshold` are considered to
    /// have low stake and will be escorted out of the validator set after being below this
    /// threshold for more than `validator_low_stake_grace_period` number of epochs.
    validator_low_stake_threshold: u64,
    /// Validators with stake below `validator_very_low_stake_threshold` will be removed
    /// immediately at epoch change, no grace period.
    validator_very_low_stake_threshold: u64,
    /// A validator can have stake below `validator_low_stake_threshold`
    /// for this many epochs before being kicked out.
    validator_low_stake_grace_period: u64,
    /// how many reward are slashed to punish a validator, in bps.
    reward_slashing_rate: u64,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

/// Uses SystemParametersV1 as the parameters.
public struct IkaSystemStateInnerV1 has store {
    /// The current epoch ID, starting from 0.
    epoch: u64,
    /// The current protocol version, starting from 1.
    protocol_version: u64,
    /// The current version of the system state data structure type.
    /// This is always the same as IkaSystemState.version. Keeping a copy here so that
    /// we know what version it is by inspecting IkaSystemStateInner as well.
    system_state_version: u64,
    /// Contains all information about the validators.
    validators: ValidatorSet,
    /// The storage fund.
    storage_fund: StorageFund,
    /// A list of system config parameters.
    parameters: SystemParametersV1,
    /// The computation price per unit size for the current epoch.
    computation_price_per_unit_size: u64,
    /// A map storing the records of validator reporting each other.
    /// There is an entry in the map for each validator that has been reported
    /// at least once. The entry VecSet contains all the validators that reported
    /// them. If a validator has never been reported they don't have an entry in this map.
    /// This map persists across epoch: a peer continues being in a reported state until the
    /// reporter doesn't explicitly remove their report.
    /// Note that in case we want to support validator address change in future,
    /// the reports should be based on validator ids
    validator_report_records: VecMap<address, VecSet<address>>,
    /// Schedule of stake subsidies given out each epoch.
    ika_treasury: IkaTreasury,
    /// Whether the system is running in a downgraded safe mode due to a non-recoverable bug.
    /// This is set whenever we failed to execute advance_epoch, and ended up executing advance_epoch_safe_mode.
    /// It can be reset once we are able to successfully execute advance_epoch.
    /// The rest of the fields starting with `safe_mode_` are accmulated during safe mode
    /// when advance_epoch_safe_mode is executed. They will eventually be processed once we
    /// are out of safe mode.
    safe_mode: bool,
    safe_mode_storage_rewards: Balance<IKA>,
    safe_mode_computation_rewards: Balance<IKA>,
    safe_mode_storage_rebates: u64,
    safe_mode_non_refundable_storage_fee: u64,
    /// Unix timestamp of the current epoch start.
    epoch_start_timestamp_ms: u64,
    /// The total actions processed.
    total_actions_processed: u64,
    /// The last checkpoint sequence number processed.
    last_processed_checkpoint_sequence_number: Option<u64>,
    /// The last checkpoint sequence number of previous epoch.
    previous_epoch_last_checkpoint_sequence_number: u64,
    /// The fees paid for computation.
    computation_reward: Balance<IKA>,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

/// Event containing system-level epoch information, emitted during
/// the epoch advancement transaction.
public struct SystemEpochInfoEvent has copy, drop {
    epoch: u64,
    protocol_version: u64,
    computation_price_per_unit_size: u64,
    total_stake: u64,
    stake_subsidy_amount: u64,
    total_computation_fees: u64,
    total_stake_rewards_distributed: u64,
    last_processed_checkpoint_sequence_number: u64
}

/// Event containing system-level checkpoint information, emitted during
/// the checkpoint submmision transaction.
public struct SystemCheckpointInfoEvent has copy, drop {
    epoch: u64,
    total_signers_stake: u64,
    sequence_number: u64,
    timestamp_ms: u64,
}

// Errors
const ENotValidator: u64 = 0;
const ELimitExceeded: u64 = 1;
#[allow(unused_const)]
const ENotSystemAddress: u64 = 2;
const ECannotReportOneself: u64 = 3;
const EReportRecordNotFound: u64 = 4;
const EBpsTooLarge: u64 = 5;
const ESafeModeGasNotProcessed: u64 = 7;
const EAdvancedToWrongEpoch: u64 = 8;

#[error]
const EIncorrectEpochInCheckpoint: vector<u8> = b"The checkpoint epoch is incorrect.";


// ==== functions that can only be called by genesis ====

/// Create a new IkaSystemState object and make it shared.
/// This function will be called only once in genesis.
public(package) fun create(
    validators: ValidatorSet,
    initial_storage_fund: Balance<IKA>,
    protocol_version: u64,
    epoch_start_timestamp_ms: u64,
    parameters: SystemParametersV1,
    ika_treasury: IkaTreasury,
    ctx: &mut TxContext,
): IkaSystemStateInnerV1 {
    let computation_price_per_unit_size = validators.derive_computation_price_per_unit_size();
    // This type is fixed as it's created at genesis. It should not be updated during type upgrade.
    let system_state = IkaSystemStateInnerV1 {
        epoch: 0,
        protocol_version,
        system_state_version: genesis_system_state_version(),
        validators,
        storage_fund: storage_fund::new(initial_storage_fund),
        parameters,
        computation_price_per_unit_size,
        validator_report_records: vec_map::empty(),
        ika_treasury,
        safe_mode: false,
        safe_mode_storage_rewards: balance::zero(),
        safe_mode_computation_rewards: balance::zero(),
        safe_mode_storage_rebates: 0,
        safe_mode_non_refundable_storage_fee: 0,
        epoch_start_timestamp_ms,
        total_actions_processed: 0,
        last_processed_checkpoint_sequence_number: option::none(),
        previous_epoch_last_checkpoint_sequence_number: 0,
        computation_reward: balance::zero(),
        extra_fields: bag::new(ctx),
    };
    system_state
}

public(package) fun create_system_parameters(
    epoch_duration_ms: u64,
    stake_subsidy_start_epoch: u64,
    // Validator committee parameters
    min_validator_count: u64,
    max_validator_count: u64,
    min_validator_joining_stake: u64,
    validator_low_stake_threshold: u64,
    validator_very_low_stake_threshold: u64,
    validator_low_stake_grace_period: u64,
    reward_slashing_rate: u64,
    ctx: &mut TxContext,
): SystemParametersV1 {
    let bps_denominator_u64 = BASIS_POINT_DENOMINATOR as u64;
    // Rates can't be higher than 100%.
    assert!(
        reward_slashing_rate <= bps_denominator_u64,
        EBpsTooLarge,
    );
    SystemParametersV1 {
        epoch_duration_ms,
        stake_subsidy_start_epoch,
        min_validator_count,
        max_validator_count,
        min_validator_joining_stake,
        validator_low_stake_threshold,
        validator_very_low_stake_threshold,
        validator_low_stake_grace_period,
        reward_slashing_rate,
        extra_fields: bag::new(ctx),
    }
}

// ==== public(package) functions ====

/// Can be called by anyone who wishes to become a validator candidate and starts accuring delegated
/// stakes in their staking pool. Once they have at least `MIN_VALIDATOR_JOINING_STAKE` amount of stake they
/// can call `request_add_validator` to officially become an active validator at the next epoch.
/// Aborts if the caller is already a pending or active validator, or a validator candidate.
/// Note: `proof_of_possession_bytes` MUST be a valid signature using sui_address and protocol_pubkey_bytes.
/// To produce a valid PoP, run [fn test_proof_of_possession_bytes].
public(package) fun request_add_validator_candidate(
    self: &mut IkaSystemStateInnerV1,
    pubkey_bytes: vector<u8>,
    network_pubkey_bytes: vector<u8>,
    worker_pubkey_bytes: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    name: vector<u8>,
    description: vector<u8>,
    image_url: vector<u8>,
    project_url: vector<u8>,
    net_address: vector<u8>,
    p2p_address: vector<u8>,
    primary_address: vector<u8>,
    worker_address: vector<u8>,
    computation_price: u64,
    commission_rate: u64,
    ctx: &mut TxContext,
) {
    let validator = validator::new(
        ctx.sender(),
        pubkey_bytes,
        network_pubkey_bytes,
        worker_pubkey_bytes,
        proof_of_possession_bytes,
        name,
        description,
        image_url,
        project_url,
        net_address,
        p2p_address,
        primary_address,
        worker_address,
        computation_price,
        commission_rate,
        ctx,
    );

    self.validators.request_add_validator_candidate(validator, ctx);
}

/// Called by a validator candidate to remove themselves from the candidacy. After this call
/// their staking pool becomes deactivate.
public(package) fun request_remove_validator_candidate(
    self: &mut IkaSystemStateInnerV1,
    ctx: &mut TxContext,
) {
    self.validators.request_remove_validator_candidate(self.epoch, ctx);
}

/// Called by a validator candidate to add themselves to the active validator set beginning next epoch.
/// Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
/// stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
/// epoch has already reached the maximum.
public(package) fun request_add_validator(self: &mut IkaSystemStateInnerV1, ctx: &TxContext) {
    assert!(
        self.validators.next_epoch_validator_count() < self.parameters.max_validator_count,
        ELimitExceeded,
    );

    self.validators.request_add_validator(self.parameters.min_validator_joining_stake, ctx);
}

/// A validator can call this function to request a removal in the next epoch.
/// We use the sender of `ctx` to look up the validator
/// (i.e. sender must match the sui_address in the validator).
/// At the end of the epoch, the `validator` object will be returned to the sui_address
/// of the validator.
public(package) fun request_remove_validator(self: &mut IkaSystemStateInnerV1, ctx: &TxContext) {
    // Only check min validator condition if the current number of validators satisfy the constraint.
    // This is so that if we somehow already are in a state where we have less than min validators, it no longer matters
    // and is ok to stay so. This is useful for a test setup.
    if (self.validators.active_validators().length() >= self.parameters.min_validator_count) {
        assert!(
            self.validators.next_epoch_validator_count() > self.parameters.min_validator_count,
            ELimitExceeded,
        );
    };

    self.validators.request_remove_validator(ctx)
}

/// A validator can call this function to submit a new gas price quote, to be
/// used for the computation price per unit size calculation at the end of the epoch.
public(package) fun request_set_computation_price(
    self: &mut IkaSystemStateInnerV1,
    cap: &UnverifiedValidatorOperationCap,
    new_computation_price: u64,
) {
    // Verify the represented address is an active or pending validator, and the capability is still valid.
    let verified_cap = self.validators.verify_cap(cap, ACTIVE_OR_PENDING_VALIDATOR);
    let validator = self
        .validators
        .get_validator_mut_with_verified_cap(&verified_cap, false /* include_candidate */);

    validator.request_set_computation_price(verified_cap, new_computation_price);
}

/// This function is used to set new gas price for candidate validators
public(package) fun set_candidate_validator_computation_price(
    self: &mut IkaSystemStateInnerV1,
    cap: &UnverifiedValidatorOperationCap,
    new_computation_price: u64,
) {
    // Verify the represented address is an active or pending validator, and the capability is still valid.
    let verified_cap = self.validators.verify_cap(cap, ANY_VALIDATOR);
    let candidate = self
        .validators
        .get_validator_mut_with_verified_cap(&verified_cap, true /* include_candidate */);
    candidate.set_candidate_computation_price(verified_cap, new_computation_price)
}

/// A validator can call this function to set a new commission rate, updated at the end of
/// the epoch.
public(package) fun request_set_commission_rate(
    self: &mut IkaSystemStateInnerV1,
    new_commission_rate: u64,
    ctx: &TxContext,
) {
    self
        .validators
        .request_set_commission_rate(
            new_commission_rate,
            ctx,
        )
}

/// This function is used to set new commission rate for candidate validators
public(package) fun set_candidate_validator_commission_rate(
    self: &mut IkaSystemStateInnerV1,
    new_commission_rate: u64,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.set_candidate_commission_rate(new_commission_rate)
}

/// Add stake to a validator's staking pool.
public(package) fun request_add_stake(
    self: &mut IkaSystemStateInnerV1,
    stake: Coin<IKA>,
    validator_address: address,
    ctx: &mut TxContext,
): StakedIka {
    self
        .validators
        .request_add_stake(
            self.epoch,
            validator_address,
            stake.into_balance(),
            ctx,
        )
}

/// Add stake to a validator's staking pool using multiple coins.
public(package) fun request_add_stake_mul_coin(
    self: &mut IkaSystemStateInnerV1,
    stakes: vector<Coin<IKA>>,
    stake_amount: option::Option<u64>,
    validator_address: address,
    ctx: &mut TxContext,
): StakedIka {
    let balance = extract_coin_balance(stakes, stake_amount, ctx);
    self.validators.request_add_stake(self.epoch, validator_address, balance, ctx)
}

/// Withdraw some portion of a stake from a validator's staking pool.
public(package) fun request_withdraw_stake(
    self: &mut IkaSystemStateInnerV1,
    staked_ika: StakedIka,
    ctx: &TxContext,
): Balance<IKA> {
    self.validators.request_withdraw_stake(self.epoch, staked_ika, ctx)
}

public(package) fun convert_to_fungible_staked_ika(
    self: &mut IkaSystemStateInnerV1,
    staked_ika: StakedIka,
    ctx: &mut TxContext,
): FungibleStakedIka {
    self.validators.convert_to_fungible_staked_ika(staked_ika, ctx)
}

public(package) fun redeem_fungible_staked_ika(
    self: &mut IkaSystemStateInnerV1,
    fungible_staked_ika: FungibleStakedIka,
    ctx: &TxContext,
): Balance<IKA> {
    self.validators.redeem_fungible_staked_ika(fungible_staked_ika, ctx)
}

/// Report a validator as a bad or non-performant actor in the system.
/// Succeeds if all the following are satisfied:
/// 1. both the reporter in `cap` and the input `reportee_addr` are active validators.
/// 2. reporter and reportee not the same address.
/// 3. the cap object is still valid.
/// This function is idempotent.
public(package) fun report_validator(
    self: &mut IkaSystemStateInnerV1,
    cap: &UnverifiedValidatorOperationCap,
    reportee_addr: address,
) {
    // Reportee needs to be an active validator
    assert!(self.validators.is_active_validator_by_sui_address(reportee_addr), ENotValidator);
    // Verify the represented reporter address is an active validator, and the capability is still valid.
    let verified_cap = self.validators.verify_cap(cap, ACTIVE_VALIDATOR_ONLY);
    report_validator_impl(verified_cap, reportee_addr, &mut self.validator_report_records);
}

/// Undo a `report_validator` action. Aborts if
/// 1. the reportee is not a currently active validator or
/// 2. the sender has not previously reported the `reportee_addr`, or
/// 3. the cap is not valid
public(package) fun undo_report_validator(
    self: &mut IkaSystemStateInnerV1,
    cap: &UnverifiedValidatorOperationCap,
    reportee_addr: address,
) {
    let verified_cap = self.validators.verify_cap(cap, ACTIVE_VALIDATOR_ONLY);
    undo_report_validator_impl(verified_cap, reportee_addr, &mut self.validator_report_records);
}

fun report_validator_impl(
    verified_cap: ValidatorOperationCap,
    reportee_addr: address,
    validator_report_records: &mut VecMap<address, VecSet<address>>,
) {
    let reporter_address = *verified_cap.verified_operation_cap_address();
    assert!(reporter_address != reportee_addr, ECannotReportOneself);
    if (!validator_report_records.contains(&reportee_addr)) {
        validator_report_records.insert(reportee_addr, vec_set::singleton(reporter_address));
    } else {
        let reporters = validator_report_records.get_mut(&reportee_addr);
        if (!reporters.contains(&reporter_address)) {
            reporters.insert(reporter_address);
        }
    }
}

fun undo_report_validator_impl(
    verified_cap: ValidatorOperationCap,
    reportee_addr: address,
    validator_report_records: &mut VecMap<address, VecSet<address>>,
) {
    assert!(validator_report_records.contains(&reportee_addr), EReportRecordNotFound);
    let reporters = validator_report_records.get_mut(&reportee_addr);

    let reporter_addr = *verified_cap.verified_operation_cap_address();
    assert!(reporters.contains(&reporter_addr), EReportRecordNotFound);

    reporters.remove(&reporter_addr);
    if (reporters.is_empty()) {
        validator_report_records.remove(&reportee_addr);
    }
}

// ==== validator metadata management functions ====

/// Create a new `UnverifiedValidatorOperationCap`, transfer it to the
/// validator and registers it. The original object is thus revoked.
public(package) fun rotate_operation_cap(self: &mut IkaSystemStateInnerV1, ctx: &mut TxContext) {
    let validator = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    validator.new_unverified_validator_operation_cap_and_transfer(ctx);
}

/// Update a validator's name.
public(package) fun update_validator_name(
    self: &mut IkaSystemStateInnerV1,
    name: vector<u8>,
    ctx: &TxContext,
) {
    let validator = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);

    validator.update_name(name);
}

/// Update a validator's description
public(package) fun update_validator_description(
    self: &mut IkaSystemStateInnerV1,
    description: vector<u8>,
    ctx: &TxContext,
) {
    let validator = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    validator.update_description(description);
}

/// Update a validator's image url
public(package) fun update_validator_image_url(
    self: &mut IkaSystemStateInnerV1,
    image_url: vector<u8>,
    ctx: &TxContext,
) {
    let validator = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    validator.update_image_url(image_url);
}

/// Update a validator's project url
public(package) fun update_validator_project_url(
    self: &mut IkaSystemStateInnerV1,
    project_url: vector<u8>,
    ctx: &TxContext,
) {
    let validator = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    validator.update_project_url(project_url);
}

/// Update a validator's network address.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_network_address(
    self: &mut IkaSystemStateInnerV1,
    network_address: vector<u8>,
    ctx: &TxContext,
) {
    let validator = self.validators.get_validator_mut_with_ctx(ctx);
    validator.update_next_epoch_network_address(network_address);
    let validator: &Validator = validator; // Force immutability for the following call
    self.validators.assert_no_pending_or_active_duplicates(validator);
}

/// Update candidate validator's network address.
public(package) fun update_candidate_validator_network_address(
    self: &mut IkaSystemStateInnerV1,
    network_address: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_network_address(network_address);
}

/// Update a validator's p2p address.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_p2p_address(
    self: &mut IkaSystemStateInnerV1,
    p2p_address: vector<u8>,
    ctx: &TxContext,
) {
    let validator = self.validators.get_validator_mut_with_ctx(ctx);
    validator.update_next_epoch_p2p_address(p2p_address);
    let validator: &Validator = validator; // Force immutability for the following call
    self.validators.assert_no_pending_or_active_duplicates(validator);
}

/// Update candidate validator's p2p address.
public(package) fun update_candidate_validator_p2p_address(
    self: &mut IkaSystemStateInnerV1,
    p2p_address: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_p2p_address(p2p_address);
}

/// Update a validator's narwhal primary address.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_primary_address(
    self: &mut IkaSystemStateInnerV1,
    primary_address: vector<u8>,
    ctx: &TxContext,
) {
    let validator = self.validators.get_validator_mut_with_ctx(ctx);
    validator.update_next_epoch_primary_address(primary_address);
}

/// Update candidate validator's narwhal primary address.
public(package) fun update_candidate_validator_primary_address(
    self: &mut IkaSystemStateInnerV1,
    primary_address: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_primary_address(primary_address);
}

/// Update a validator's narwhal worker address.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_worker_address(
    self: &mut IkaSystemStateInnerV1,
    worker_address: vector<u8>,
    ctx: &TxContext,
) {
    let validator = self.validators.get_validator_mut_with_ctx(ctx);
    validator.update_next_epoch_worker_address(worker_address);
}

/// Update candidate validator's narwhal worker address.
public(package) fun update_candidate_validator_worker_address(
    self: &mut IkaSystemStateInnerV1,
    worker_address: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_worker_address(worker_address);
}

/// Update a validator's public key of protocol key and proof of possession.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_protocol_pubkey(
    self: &mut IkaSystemStateInnerV1,
    protocol_pubkey: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    ctx: &TxContext,
) {
    let validator = self.validators.get_validator_mut_with_ctx(ctx);
    validator.update_next_epoch_protocol_pubkey(protocol_pubkey, proof_of_possession_bytes);
    let validator: &Validator = validator; // Force immutability for the following call
    self.validators.assert_no_pending_or_active_duplicates(validator);
}

/// Update candidate validator's public key of protocol key and proof of possession.
public(package) fun update_candidate_validator_protocol_pubkey(
    self: &mut IkaSystemStateInnerV1,
    protocol_pubkey: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_protocol_pubkey(protocol_pubkey, proof_of_possession_bytes);
}

/// Update a validator's public key of worker key.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_worker_pubkey(
    self: &mut IkaSystemStateInnerV1,
    worker_pubkey: vector<u8>,
    ctx: &TxContext,
) {
    let validator = self.validators.get_validator_mut_with_ctx(ctx);
    validator.update_next_epoch_worker_pubkey(worker_pubkey);
    let validator: &Validator = validator; // Force immutability for the following call
    self.validators.assert_no_pending_or_active_duplicates(validator);
}

/// Update candidate validator's public key of worker key.
public(package) fun update_candidate_validator_worker_pubkey(
    self: &mut IkaSystemStateInnerV1,
    worker_pubkey: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_worker_pubkey(worker_pubkey);
}

/// Update a validator's public key of network key.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_network_pubkey(
    self: &mut IkaSystemStateInnerV1,
    network_pubkey: vector<u8>,
    ctx: &TxContext,
) {
    let validator = self.validators.get_validator_mut_with_ctx(ctx);
    validator.update_next_epoch_network_pubkey(network_pubkey);
    let validator: &Validator = validator; // Force immutability for the following call
    self.validators.assert_no_pending_or_active_duplicates(validator);
}

/// Update candidate validator's public key of network key.
public(package) fun update_candidate_validator_network_pubkey(
    self: &mut IkaSystemStateInnerV1,
    network_pubkey: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_network_pubkey(network_pubkey);
}

/// This function should be called at the end of an epoch, and advances the system to the next epoch.
/// It does the following things:
/// 1. Add storage charge to the storage fund.
/// 2. Burn the storage rebates from the storage fund. These are already refunded to transaction sender's
///    gas coins.
/// 3. Distribute computation charge to validator stake.
/// 4. Update all validators.
public(package) fun advance_epoch(
    self: &mut IkaSystemStateInnerV1,
    new_epoch: u64,
    next_protocol_version: u64,
    // mut storage_reward: Balance<IKA>,
    // mut computation_reward: Balance<IKA>,
    // mut storage_rebate_amount: u64,
    // mut non_refundable_storage_fee_amount: u64,
    // storage_fund_reinvest_rate: u64, // share of storage fund's rewards that's reinvested
    // into storage fund, in basis point.
    epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
    ctx: &mut TxContext,
) {
    let prev_epoch_start_timestamp = self.epoch_start_timestamp_ms;
    self.epoch_start_timestamp_ms = epoch_start_timestamp_ms;

    // TODO: remove this in later upgrade.
    if (self.parameters.stake_subsidy_start_epoch > 0) {
        self.parameters.stake_subsidy_start_epoch = 20;
    };

    // // Accumulate the gas summary during safe_mode before processing any rewards:
    // let safe_mode_storage_rewards = self.safe_mode_storage_rewards.withdraw_all();
    // storage_reward.join(safe_mode_storage_rewards);
    // let safe_mode_computation_rewards = self.safe_mode_computation_rewards.withdraw_all();
    // computation_reward.join(safe_mode_computation_rewards);
    // storage_rebate_amount = storage_rebate_amount + self.safe_mode_storage_rebates;
    // self.safe_mode_storage_rebates = 0;
    // non_refundable_storage_fee_amount =
    //     non_refundable_storage_fee_amount + self.safe_mode_non_refundable_storage_fee;
    // self.safe_mode_non_refundable_storage_fee = 0;

    //let total_validators_stake = self.validators.total_stake();
    //let storage_fund_balance = self.storage_fund.total_balance();
    //let total_stake = storage_fund_balance + total_validators_stake;

    // let storage_charge = storage_reward.value();
    //let computation_charge = self.computation_reward.value();
    let mut stake_subsidy = balance::zero();

    // during the transition from epoch N to epoch N + 1, self.epoch() will return N
    let old_epoch = self.epoch();
    // Include stake subsidy in the rewards given out to validators and stakers.
    // Delay distributing any stake subsidies until after `stake_subsidy_start_epoch`.
    // And if this epoch is shorter than the regular epoch duration, don't distribute any stake subsidy.
    if (
        old_epoch >= self.parameters.stake_subsidy_start_epoch  &&
            epoch_start_timestamp_ms >= prev_epoch_start_timestamp + self.parameters.epoch_duration_ms
    ) {
        // // special case for epoch 560 -> 561 change bug. add extra subsidies for "safe mode"
        // // where reward distribution was skipped. use distribution counter and epoch check to
        // // avoiding affecting devnet and testnet
        // if (self.stake_subsidy.get_stake_subsidy_distribution_counter() == 540 && old_epoch > 560) {
        //     // safe mode was entered on the change from 560 to 561. so 560 was the first epoch without proper subsidy distribution
        //     let first_safe_mode_epoch = 560;
        //     let safe_mode_epoch_count = old_epoch - first_safe_mode_epoch;
        //     safe_mode_epoch_count.do!(|_| {
        //         stake_subsidy.join(self.stake_subsidy.advance_epoch());
        //     });
        //     // done with catchup for safe mode epochs. distribution counter is now >540, we won't hit this again
        //     // fall through to the normal logic, which will add subsidies for the current epoch
        // };
        stake_subsidy.join(self.ika_treasury.stake_subsidy_for_distribution(ctx));
    };


    //computation_reward.join(stake_subsidy);

    //let total_stake_u128 = total_stake as u128;
    // let computation_charge_u128 = computation_charge as u128;

    // let storage_fund_reward_amount =
    //     storage_fund_balance as u128 * computation_charge_u128 / total_stake_u128;
    // let mut storage_fund_reward = computation_reward.split(storage_fund_reward_amount as u64);
    // let storage_fund_reinvestment_amount =
    //     storage_fund_reward_amount * (storage_fund_reinvest_rate as u128) / BASIS_POINT_DENOMINATOR;
    // let storage_fund_reinvestment = storage_fund_reward.split(
    //     storage_fund_reinvestment_amount as u64,
    // );



    let computation_reward_amount_before_distribution = self.computation_reward.value();

    let stake_subsidy_amount = stake_subsidy.value();
    let mut total_reward = sui::balance::zero<IKA>();
    total_reward.join(self.computation_reward.withdraw_all());
    total_reward.join(stake_subsidy);
    let total_reward_amount_before_distribution = total_reward.value();

    self.epoch = self.epoch + 1;
    // Sanity check to make sure we are advancing to the right epoch.
    assert!(new_epoch == self.epoch, EAdvancedToWrongEpoch);

    self
        .validators
        .advance_epoch(
            old_epoch,
            new_epoch,
            &mut total_reward,
            // &mut computation_reward,
            // &mut storage_fund_reward,
            &mut self.validator_report_records,
            self.parameters.reward_slashing_rate,
            self.parameters.validator_low_stake_threshold,
            self.parameters.validator_very_low_stake_threshold,
            self.parameters.validator_low_stake_grace_period,
            ctx,
        );



    let new_total_stake = self.validators.total_stake();

    let total_reward_amount_after_distribution = total_reward.value();
    let total_reward_distributed =
         total_reward_amount_before_distribution - total_reward_amount_after_distribution;

    // Because of precision issues with integer divisions, we expect that there will be some
    // remaining balance in `computation_reward`.
    self.computation_reward.join(total_reward);

    self.protocol_version = next_protocol_version;

    // Derive the computation price per unit size for the new epoch
    self.computation_price_per_unit_size = self.validators.derive_computation_price_per_unit_size();
    // // Because of precision issues with integer divisions, we expect that there will be some
    // // remaining balance in `storage_fund_reward` and `computation_reward`.
    // // All of these go to the storage fund.
    // let mut leftover_staking_rewards = storage_fund_reward;
    // leftover_staking_rewards.join(computation_reward);
    // let leftover_storage_fund_inflow = leftover_staking_rewards.value();

    // let refunded_storage_rebate = self
    //     .storage_fund
    //     .advance_epoch(
    //         storage_reward,
    //         storage_fund_reinvestment, 
    //         leftover_staking_rewards,
    //         storage_rebate_amount,
    //         non_refundable_storage_fee_amount,
    //     );

    let last_processed_checkpoint_sequence_number = *self.last_processed_checkpoint_sequence_number.borrow();
    self.previous_epoch_last_checkpoint_sequence_number = last_processed_checkpoint_sequence_number;

    event::emit(SystemEpochInfoEvent {
        epoch: self.epoch,
        protocol_version: self.protocol_version,
        computation_price_per_unit_size: self.computation_price_per_unit_size,
        total_stake: new_total_stake,
        stake_subsidy_amount,
        total_computation_fees: computation_reward_amount_before_distribution,
        total_stake_rewards_distributed: total_reward_distributed,
        last_processed_checkpoint_sequence_number,
    });
    self.safe_mode = false;
    // Double check that the gas from safe mode has been processed.
    assert!(
        self.safe_mode_storage_rebates == 0
            && self.safe_mode_storage_rewards.value() == 0
            && self.safe_mode_computation_rewards.value() == 0,
        ESafeModeGasNotProcessed,
    );

    // // Return the storage rebate split from storage fund that's already refunded to the transaction senders.
    // // This will be burnt at the last step of epoch change programmable transaction.
    // refunded_storage_rebate
}

/// Return the current epoch number. Useful for applications that need a coarse-grained concept of time,
/// since epochs are ever-increasing and epoch changes are intended to happen every 24 hours.
public(package) fun epoch(self: &IkaSystemStateInnerV1): u64 {
    self.epoch
}

public(package) fun protocol_version(self: &IkaSystemStateInnerV1): u64 {
    self.protocol_version
}

public(package) fun system_state_version(self: &IkaSystemStateInnerV1): u64 {
    self.system_state_version
}

/// This function always return the genesis system state version, which is used to create the system state in genesis.
/// It should never change for a given network.
public(package) fun genesis_system_state_version(): u64 {
    SYSTEM_STATE_VERSION_V1
}

/// Returns unix timestamp of the start of current epoch
public(package) fun epoch_start_timestamp_ms(self: &IkaSystemStateInnerV1): u64 {
    self.epoch_start_timestamp_ms
}

/// Returns the total amount staked with `validator_addr`.
/// Aborts if `validator_addr` is not an active validator.
public(package) fun validator_stake_amount(
    self: &IkaSystemStateInnerV1,
    validator_addr: address,
): u64 {
    self.validators.validator_total_stake_amount(validator_addr)
}

/// Returns the voting power for `validator_addr`.
/// Aborts if `validator_addr` is not an active validator.
public(package) fun active_validator_voting_powers(
    self: &IkaSystemStateInnerV1,
): VecMap<address, u64> {
    let mut active_validators = active_validator_addresses(self);
    let mut voting_powers = vec_map::empty();
    while (!vector::is_empty(&active_validators)) {
        let validator = vector::pop_back(&mut active_validators);
        let voting_power = validator_set::validator_voting_power(&self.validators, validator);
        vec_map::insert(&mut voting_powers, validator, voting_power);
    };
    voting_powers
}

/// Returns the staking pool id of a given validator.
/// Aborts if `validator_addr` is not an active validator.
public(package) fun validator_staking_pool_id(
    self: &IkaSystemStateInnerV1,
    validator_addr: address,
): ID { self.validators.validator_staking_pool_id(validator_addr) }

/// Returns reference to the staking pool mappings that map pool ids to active validator addresses
public(package) fun validator_staking_pool_mappings(
    self: &IkaSystemStateInnerV1,
): &Table<ID, address> { self.validators.staking_pool_mappings() }

/// Returns all the validators who are currently reporting `addr`
public(package) fun get_reporters_of(self: &IkaSystemStateInnerV1, addr: address): VecSet<address> {
    if (self.validator_report_records.contains(&addr)) {
        self.validator_report_records[&addr]
    } else {
        vec_set::empty()
    }
}

public(package) fun get_storage_fund_total_balance(self: &IkaSystemStateInnerV1): u64 {
    self.storage_fund.total_balance()
}

public(package) fun get_storage_fund_object_rebates(self: &IkaSystemStateInnerV1): u64 {
    self.storage_fund.total_object_storage_rebates()
}

public(package) fun validator_address_by_pool_id(
    self: &mut IkaSystemStateInnerV1,
    pool_id: &ID,
): address {
    self.validators.validator_address_by_pool_id(pool_id)
}

public(package) fun pool_exchange_rates(
    self: &mut IkaSystemStateInnerV1,
    pool_id: &ID,
): &Table<u64, PoolTokenExchangeRate> {
    let validators = &mut self.validators;
    validators.pool_exchange_rates(pool_id)
}

public(package) fun active_validator_addresses(self: &IkaSystemStateInnerV1): vector<address> {
    let validator_set = &self.validators;
    validator_set.active_validator_addresses()
}

public struct TestMessageEvent has drop, copy {
    epoch: u64,
    sequence_number: u64,
    authority: u32,
    num: u64,
    total_signers_stake: u64,
}

#[error]
const EWrongCheckpointSequenceNumber: vector<u8> = b"The checkpoint sequence number should be the expected next one.";

public(package) fun process_checkpoint_message(
    self: &mut IkaSystemStateInnerV1,
    signature: vector<u8>,
    signers: vector<u16>,
    message: vector<u8>,
    ctx: &mut TxContext,
) {

    // first let's make sure it's the correct checkpoint message
    let mut bcs_body = bcs::new(copy message);

    let epoch = bcs_body.peel_u64();
    assert!(epoch == self.epoch, EIncorrectEpochInCheckpoint);

    let sequence_number = bcs_body.peel_u64();

    if(self.last_processed_checkpoint_sequence_number.is_none()) {
        assert!(sequence_number == 0, EWrongCheckpointSequenceNumber);
        self.last_processed_checkpoint_sequence_number.fill(sequence_number);
    } else {
        assert!(sequence_number > 0 && *self.last_processed_checkpoint_sequence_number.borrow() + 1 == sequence_number, EWrongCheckpointSequenceNumber);
        self.last_processed_checkpoint_sequence_number.swap(sequence_number);
    };

    //let network_total_messages = bcs_body.peel_u64();
    //let previous_digest = bcs_body.peel_option!(|previous_digest| previous_digest.peel_vec_u8() );
    let timestamp_ms = bcs_body.peel_u64();

    // second let's verify certificate
    let intent_scope = CHECKPOINT_MESSAGE_INTENT_SCOPE;
    let intent_version = INTENT_VERSION;
    let intent_app = INTENT_APP_ID;
    let mut intent_bytes = vector[];
    common::bcs_output_u32_as_uleb128(&mut intent_bytes, intent_scope as u32);
    common::bcs_output_u32_as_uleb128(&mut intent_bytes, intent_version as u32);
    common::bcs_output_u32_as_uleb128(&mut intent_bytes, intent_app as u32);
    //message.do_ref!(|key_byte| intent_bytes.append(bcs::to_bytes(key_byte)));
    intent_bytes.append(message);
    intent_bytes.append(bcs::to_bytes(&self.epoch));

    let total_signers_stake = self.validators.verify_certificate(&signature, &signers, &intent_bytes);

    event::emit(SystemCheckpointInfoEvent {
        epoch,
        total_signers_stake,
        sequence_number,
        timestamp_ms,
    });

    // now let's process message

    //assert!(false, 456);

    let len = bcs_body.peel_vec_length();
    let mut i = 0;
    while (i < len) {
        let action_data_version = bcs_body.peel_vec_length();
        if(action_data_version == 0) {
            let action_data_type = bcs_body.peel_vec_length();
            if (action_data_type == 0) {
                let authority = bcs_body.peel_u32();
                let num = bcs_body.peel_u64();
                event::emit(TestMessageEvent {
                    epoch,
                    sequence_number,
                    authority,
                    num,
                    total_signers_stake,
                });
                // EndOfEpochMessage
            } else if (action_data_type == 1) {
                let len = bcs_body.peel_vec_length();
                let mut i = 0;
                while (i < len) {
                    let end_of_epch_message_type = bcs_body.peel_vec_length();
                    // AdvanceEpoch 
                    if(end_of_epch_message_type == 0) {
                        let new_epoch = bcs_body.peel_u64();
                        let next_protocol_version = bcs_body.peel_u64();
                        let epoch_start_timestamp_ms = bcs_body.peel_u64();
                        self.advance_epoch(new_epoch, next_protocol_version, epoch_start_timestamp_ms, ctx);
                    };
                    i = i + 1;
                };
            };
        };
        i = i + 1;
    };
    self.total_actions_processed = self.total_actions_processed + i;
}

#[allow(lint(self_transfer))]
/// Extract required Balance from vector of Coin<IKA>, transfer the remainder back to sender.
fun extract_coin_balance(
    mut coins: vector<Coin<IKA>>,
    amount: option::Option<u64>,
    ctx: &mut TxContext,
): Balance<IKA> {
    let mut merged_coin = coins.pop_back();
    merged_coin.join_vec(coins);

    let mut total_balance = merged_coin.into_balance();
    // return the full amount if amount is not specified
    if (amount.is_some()) {
        let amount = amount.destroy_some();
        let balance = total_balance.split(amount);
        // transfer back the remainder if non zero.
        if (total_balance.value() > 0) {
            transfer::public_transfer(total_balance.into_coin(ctx), ctx.sender());
        } else {
            total_balance.destroy_zero();
        };
        balance
    } else {
        total_balance
    }
}

#[test_only]
/// Return the current validator set
public(package) fun validators(self: &IkaSystemStateInnerV1): &ValidatorSet {
    &self.validators
}

#[test_only]
/// Return the currently active validator by address
public(package) fun active_validator_by_address(
    self: &IkaSystemStateInnerV1,
    validator_address: address,
): &Validator {
    self.validators().get_active_validator_ref(validator_address)
}

#[test_only]
/// Return the currently pending validator by address
public(package) fun pending_validator_by_address(
    self: &IkaSystemStateInnerV1,
    validator_address: address,
): &Validator {
    self.validators().get_pending_validator_ref(validator_address)
}

#[test_only]
/// Return the currently candidate validator by address
public(package) fun candidate_validator_by_address(
    self: &IkaSystemStateInnerV1,
    validator_address: address,
): &Validator {
    validators(self).get_candidate_validator_ref(validator_address)
}

#[test_only]
public(package) fun get_stake_subsidy_stake_subsidy_distribution_counter(self: &IkaSystemStateInnerV1): u64 {
    self.ika_treasury.get_stake_subsidy_distribution_counter()
}

#[test_only]
public(package) fun set_epoch_for_testing(self: &mut IkaSystemStateInnerV1, epoch_num: u64) {
    self.epoch = epoch_num
}

#[test_only]
public(package) fun request_add_validator_for_testing(
    self: &mut IkaSystemStateInnerV1,
    min_joining_stake_for_testing: u64,
    ctx: &TxContext,
) {
    assert!(
        self.validators.next_epoch_validator_count() < self.parameters.max_validator_count,
        ELimitExceeded,
    );

    self.validators.request_add_validator(min_joining_stake_for_testing, ctx);
}

#[test_only]
public(package) fun set_stake_subsidy_stake_subsidy_distribution_counter(
    self: &mut IkaSystemStateInnerV1,
    counter: u64,
) {
    self.ika_treasury.set_stake_subsidy_distribution_counter(counter)
}

#[test_only]
public(package) fun epoch_duration_ms(self: &IkaSystemStateInnerV1): u64 {
    self.parameters.epoch_duration_ms
}

// CAUTION: THIS CODE IS ONLY FOR TESTING AND THIS MACRO MUST NEVER EVER BE REMOVED.  Creates a
// candidate validator - bypassing the proof of possession check and other metadata validation
// in the process.
#[test_only]
public(package) fun request_add_validator_candidate_for_testing(
    self: &mut IkaSystemStateInnerV1,
    pubkey_bytes: vector<u8>,
    network_pubkey_bytes: vector<u8>,
    worker_pubkey_bytes: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    name: vector<u8>,
    description: vector<u8>,
    image_url: vector<u8>,
    project_url: vector<u8>,
    net_address: vector<u8>,
    p2p_address: vector<u8>,
    primary_address: vector<u8>,
    worker_address: vector<u8>,
    computation_price: u64,
    commission_rate: u64,
    ctx: &mut TxContext,
) {
    let validator = validator::new_for_testing(
        ctx.sender(),
        pubkey_bytes,
        network_pubkey_bytes,
        worker_pubkey_bytes,
        proof_of_possession_bytes,
        name,
        description,
        image_url,
        project_url,
        net_address,
        p2p_address,
        primary_address,
        worker_address,
        option::none(),
        computation_price,
        commission_rate,
        false, // not an initial validator active at genesis
        ctx,
    );

    self.validators.request_add_validator_candidate(validator, ctx);
}
