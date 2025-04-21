// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::validator_set;

use ika::ika::IKA;
use ika_system::staking_pool::{
    PoolTokenExchangeRate,
};
use ika_system::staked_ika::{
    StakedIka,
    validator_id,
    FungibleStakedIka,
};
use ika_system::validator_inner_v1::{ValidatorInnerV1};
use ika_system::validator_cap::{ValidatorCap, ValidatorOperationCap };
use ika_system::validator::{Self, Validator};
use ika_system::bls_committee::{Self, BlsCommittee, new_bls_committee, new_bls_committee_member, total_voting_power, quorum_threshold};
use ika_system::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof;
use sui::bag::{Self, Bag};
use sui::balance::Balance;
use sui::event;
use sui::priority_queue as pq;
use sui::table::{Table};
use sui::object_table::{Self, ObjectTable};
use sui::vec_map::{Self, VecMap};
use sui::vec_set::{Self, VecSet};

public struct ValidatorSet has store {
    /// Total amount of stake from all active validators at the beginning of the epoch.
    total_stake: u64,
    /// A tale that contains all validators
    validators: ObjectTable<ID, Validator>,
    /// The current list of active committee of validators.
    active_committee: BlsCommittee,
    /// The next list of active committee of validators.
    /// It will become the active_committee at the end of the epoch.
    next_epoch_committee: Option<BlsCommittee>,
    /// The current list of previous committee of validators.
    previous_committee: BlsCommittee,
    /// The next list of pending active validators to be next_epoch_committee.
    /// It will start from the last next_epoch_committee and will be
    /// process between middle of the epochs and will be finalize
    /// at the middle of the epoch.
    pending_active_validators: vector<ID>,
    /// Table storing the number of epochs during which a validator's stake has been below the low stake threshold.
    at_risk_validators: VecMap<ID, u64>,
    /// A map storing the records of validator reporting each other.
    /// There is an entry in the map for each validator that has been reported
    /// at least once. The entry VecSet contains all the validators that reported
    /// them. If a validator has never been reported they don't have an entry in this map.
    /// This map persists across epoch: a peer continues being in a reported state until the
    /// reporter doesn't explicitly remove their report.
    /// Note that in case we want to support validator address change in future,
    /// the reports should be based on validator ids
    validator_report_records: VecMap<ID, VecSet<ID>>,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

/// Event containing staking and rewards related information of
/// each validator, emitted during epoch advancement.
public struct ValidatorEpochInfoEventV1 has copy, drop {
    epoch: u64,
    validator_id: ID,
    reference_gas_survey_quote: u64,
    stake: u64,
    voting_power: u64,
    commission_rate: u16,
    pool_staking_reward: u64,
    pool_token_exchange_rate: PoolTokenExchangeRate,
    tallying_rule_reporters: vector<ID>,
    tallying_rule_global_score: u64,
}

/// Event emitted every time a new validator joins the committee.
/// The epoch value corresponds to the first epoch this change takes place.
public struct ValidatorJoinEvent has copy, drop {
    epoch: u64,
    validator_id: ID,
}

/// Event emitted every time a validator leaves the committee.
/// The epoch value corresponds to the first epoch this change takes place.
public struct ValidatorLeaveEvent has copy, drop {
    epoch: u64,
    validator_id: ID,
    is_voluntary: bool,
}

const BASIS_POINT_DENOMINATOR: u128 = 10000;
const MIN_STAKING_THRESHOLD: u64 = 1_000_000_000; // 1 IKA


// Errors
const ENonValidatorInReportRecords: u64 = 0;
const EDuplicateValidator: u64 = 2;
const ENotAValidator: u64 = 4;
const EMinJoiningStakeNotReached: u64 = 5;
const EValidatorNotCandidate: u64 = 7;
const ENotActiveOrPendingValidator: u64 = 9;
const EStakingBelowThreshold: u64 = 10;
const EValidatorAlreadyRemoved: u64 = 11;
const EValidatorSetEmpty: u64 = 13;
const ENotCandidateOrActiveValidator: u64 = 14;
const ENotCandidateOrActiveOrInactiveValidator: u64 = 15;
const ENotCandidateOrActiveOrPendingValidator: u64 = 16;
const ECannotReportOneself: u64 = 17;
const EReportRecordNotFound: u64 = 18;

const EInvalidCap: u64 = 101;

#[error]
const EProcessMidEpochOnlyAfterAdvanceEpoch: vector<u8> = b"Process mid epoch can be called only after advance epoch.";


#[error]
const EAdvanceEpochOnlyAfterProcessMidEpoch: vector<u8> = b"Advance epoch can be called only after process mid epoch.";

#[error]
const EAlreadyInitialized: vector<u8> = b"Protocol cannot be initialized more than one time.";

// ==== initialization ====

public(package) fun new(ctx: &mut TxContext): ValidatorSet {
    // let total_stake = calculate_total_stakes(&init_active_committee);
    // let mut staking_pool_mappings = table::new(ctx);
    // let num_validators = init_active_committee.length();
    // let mut i = 0;
    // while (i < num_validators) {
    //     let validator = &init_active_committee[i];
    //     staking_pool_mappings.add(staking_pool_id(validator), sui_address(validator));
    //     i = i + 1;
    // };
    let validators = ValidatorSet {
        total_stake: 0,
        validators: object_table::new(ctx),
        active_committee: bls_committee::empty(),
        next_epoch_committee: option::none(),
        previous_committee: bls_committee::empty(),
        pending_active_validators: vector[],
        at_risk_validators: vec_map::empty(),
        validator_report_records: vec_map::empty(),
        extra_fields: bag::new(ctx),
    };
    //voting_power::set_voting_power(&mut validators.active_committee);
    validators
}

public(package) fun initialize(self: &mut ValidatorSet) {
    assert!(self.active_committee.members().is_empty(), EAlreadyInitialized);
    self.process_pending_validators();
    self.active_committee = self.next_epoch_committee.extract();
    self.activate_added_validators(0);
    self.total_stake = calculate_total_stakes(self);
}

// ==== functions to add or remove validators ====

/// Called by `ika_system` to add a new validator candidate.
public(package) fun request_add_validator_candidate(
    self: &mut ValidatorSet,
    payment_address: address,
    protocol_pubkey_bytes: vector<u8>,
    network_pubkey_bytes: vector<u8>,
    consensus_pubkey_bytes: vector<u8>,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    proof_of_possession_bytes: vector<u8>,
    name: vector<u8>,
    description: vector<u8>,
    image_url: vector<u8>,
    project_url: vector<u8>,
    network_address: vector<u8>,
    p2p_address: vector<u8>,
    consensus_address: vector<u8>,
    computation_price: u64,
    commission_rate: u16,
    ctx: &mut TxContext,
): (ValidatorCap, ValidatorOperationCap) {

    let (mut validator, cap, operation_cap) = validator::create(
        payment_address,
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        class_groups_pubkey_and_proof_bytes,
        proof_of_possession_bytes,
        name,
        description,
        image_url,
        project_url,
        network_address,
        p2p_address,
        consensus_address,
        computation_price,
        commission_rate,
        ctx,
    );

    let validator_inner_v1 = validator.load_validator_maybe_upgrade();

    let validator_id = validator_inner_v1.validator_id();

    // The next assertions are not critical for the protocol, but they are here to catch problematic configs earlier.
    assert!(
        !is_duplicate_with_active_validator(self, validator_inner_v1)
                && !is_duplicate_with_pending_validator(self, validator_inner_v1)
                && !is_duplicate_with_next_epoch_committee(self, validator_inner_v1),
        EDuplicateValidator,
    );
    assert!(!self.validators.contains(validator_id), EDuplicateValidator);

    assert!(validator_inner_v1.is_candidate(), EValidatorNotCandidate);
    self
        .validators
        .add(
            validator_id,
            validator,
        );
    (cap, operation_cap)
}

/// Called by `ika_system` to remove a validator candidate, and move them to `inactive_committee`.
public(package) fun request_remove_validator_candidate(
    self: &mut ValidatorSet,
    epoch: u64,
    cap: &ValidatorCap,
) {
    let validator_id = cap.validator_id();
    let validator = self.get_validator_mut(validator_id);
    assert!(validator.is_candidate(), EValidatorNotCandidate);

    // Deactivate the staking pool.
    validator.deactivate(epoch);
}

/// Called by `ika_system` to add a new validator to `pending_active_validators`, which will be
/// processed at the end of epoch.
public(package) fun request_add_validator(
    self: &mut ValidatorSet,
    min_joining_stake_amount: u64,
    cap: &ValidatorCap,
) {
    let validator_id = cap.validator_id();
    assert!(self.validators.contains(validator_id), EValidatorNotCandidate);
    // We have to remove and to add again because we can have 2 refs to self
    let mut wrapper = self.validators.remove(validator_id);
    let validator = wrapper.load_validator_maybe_upgrade();
    assert!(
        !is_duplicate_with_active_validator(self, validator)
                && !is_duplicate_with_pending_validator(self, validator)
                && !is_duplicate_with_next_epoch_committee(self, validator),
        EDuplicateValidator,
    );
    assert!(validator.is_candidate(), EValidatorNotCandidate);
    assert!(validator.total_stake_amount() >= min_joining_stake_amount, EMinJoiningStakeNotReached);

    self.validators.add(validator_id, wrapper);

    self.pending_active_validators.push_back(validator_id);
}

public(package) fun assert_no_pending_or_active_duplicates(
    self: &mut ValidatorSet,
    validator_id: ID,
) {

    let active_validator_ids = self.active_committee.validator_ids();
    let pending_active_validators = self.pending_active_validators;

    assert!(self.validators.contains(validator_id), EValidatorNotCandidate);
    // We have to remove and to add again because we can have 2 refs to self
    let mut wrapper = self.validators.remove(validator_id);
    let validator = wrapper.load_validator_maybe_upgrade();

    // Validator here must be active or pending, and thus must be identified as duplicate exactly once.
    assert!(
        count_duplicates_vec(self, &active_validator_ids, validator) +
                count_duplicates_vec(self, &pending_active_validators, validator) == 1,
        EDuplicateValidator,
    );

    self.validators.add(validator_id, wrapper);
}

/// Called by `ika_system`, to remove a validator.
/// The index of the validator is added to `pending_removals` and
/// will be processed at the end of epoch.
/// Only an active validator can request to be removed.
public(package) fun request_remove_validator(
    self: &mut ValidatorSet,
    cap: &ValidatorCap,
) {
    let validator_id = cap.validator_id();
    assert!(!self.pending_active_validators.contains(&validator_id), EValidatorAlreadyRemoved);
    self.pending_active_validators.push_back(validator_id);
}

// ==== staking related functions ====

/// Called by `ika_system`, to add a new stake to the validator.
/// This request is added to the validator's staking pool's pending stake entries, processed at the end
/// of the epoch.
/// Aborts in case the staking amount is smaller than MIN_STAKING_THRESHOLD
public(package) fun request_add_stake(
    self: &mut ValidatorSet,
    epoch: u64,
    validator_id: ID,
    stake: Balance<IKA>,
    ctx: &mut TxContext,
): StakedIka {
    let ika_amount = stake.value();
    assert!(ika_amount >= MIN_STAKING_THRESHOLD, EStakingBelowThreshold);
    let validator = get_candidate_or_active_validator_mut(self, validator_id);
    validator.request_add_stake(epoch, stake, ctx)
}

/// Called by `ika_system`, to withdraw some share of a stake from the validator. The share to withdraw
/// is denoted by `principal_withdraw_amount`. One of two things occurs in this function:
/// 1. If the `staked_ika` is staked with an active validator, the request is added to the validator's
///    staking pool's pending stake withdraw entries, processed at the end of the epoch.
/// 2. If the `staked_ika` was staked with a validator that is no longer active,
///    the stake and any rewards corresponding to it will be immediately processed.
public(package) fun request_withdraw_stake(
    self: &mut ValidatorSet,
    epoch: u64,
    staked_ika: StakedIka,
): Balance<IKA> {
    let validator_id = staked_ika.validator_id();
    let validator = self.get_candidate_or_active_or_inactive_validator_mut(validator_id);
    validator.request_withdraw_stake(epoch, staked_ika)
}

public(package) fun convert_to_fungible_staked_ika(
    self: &mut ValidatorSet,
    epoch: u64,
    staked_ika: StakedIka,
    ctx: &mut TxContext,
): FungibleStakedIka {
    let validator_id = staked_ika.validator_id();
    let validator = self.get_candidate_or_active_or_inactive_validator_mut(validator_id);

    validator.convert_to_fungible_staked_ika(epoch, staked_ika, ctx)
}

public(package) fun redeem_fungible_staked_ika(
    self: &mut ValidatorSet,
    epoch: u64,
    fungible_staked_ika: FungibleStakedIka,
): Balance<IKA> {
    let validator_id = fungible_staked_ika.validator_id();
    let validator = self.get_candidate_or_active_or_inactive_validator_mut(validator_id);

    validator.redeem_fungible_staked_ika(epoch, fungible_staked_ika)
}

// ==== validator config setting functions ====

public(package) fun request_set_commission_rate(
    self: &mut ValidatorSet,
    new_commission_rate: u16,
    cap: &ValidatorCap,
) {
    let validator_id = cap.validator_id();
    let validator = self.get_validator_mut(validator_id);
    validator.request_set_commission_rate(new_commission_rate);
}

// ==== epoch change functions ====


public(package) fun process_mid_epoch(
    self: &mut ValidatorSet,
    epoch: u64,
    lock_active_committee: bool,
    low_stake_threshold: u64,
    very_low_stake_threshold: u64,
    low_stake_grace_period: u64,
) {
    assert!(self.next_epoch_committee.is_none(), EProcessMidEpochOnlyAfterAdvanceEpoch);
    let new_epoch = epoch + 1;

    if (lock_active_committee) {
        // if we lock the committee just keep it the same as last time
        self.next_epoch_committee.fill(self.active_committee)
    } else {
        // kick low stake validators out.
        self.update_and_process_low_stake_departures(
            new_epoch,
            low_stake_threshold,
            very_low_stake_threshold,
            low_stake_grace_period,
        );

        // Note that all their staged next epoch metadata will be effectuated during next `advance_epoch`.
        self.process_pending_validators();
    };
}

/// Update the validator set at the end of epoch.
/// It does the following things:
///   1. Distribute stake award.
///   2. Process pending stake deposits and withdraws for each validator (`adjust_stake`).
///   3. Process pending stake deposits, and withdraws.
///   4. Process pending validator application and withdraws.
///   5. At the end, we calculate the total stake for the new epoch.
public(package) fun advance_epoch(
    self: &mut ValidatorSet,
    epoch: u64,
    new_epoch: u64,
    total_reward: &mut Balance<IKA>,
    reward_slashing_rate: u16,
    ctx: &mut TxContext,
) {
    assert!(self.next_epoch_committee.is_some(), EAdvanceEpochOnlyAfterProcessMidEpoch);

    let total_voting_power = total_voting_power();

    // Compute the reward distribution without taking into account the tallying rule slashing.
    let unadjusted_staking_reward_amounts = self.compute_unadjusted_reward_distribution(
        total_voting_power,
        total_reward.value(),
    );

    // Use the tallying rule report records for the epoch to compute validators that will be
    // punished.
    let slashed_validators = self.compute_slashed_validators();


    let total_slashed_validator_voting_power = self.sum_voting_power_by_validator_indices(
        slashed_validators,
    );

    let slashed_validator_indices = self.get_validator_indices(&slashed_validators);

    // Compute the reward adjustments of slashed validators, to be taken into
    // account in adjusted reward computation.
    let (
        total_staking_reward_adjustment,
        individual_staking_reward_adjustments,
    ) = compute_reward_adjustments(
        slashed_validator_indices,
        reward_slashing_rate,
        &unadjusted_staking_reward_amounts,
    );

    // Compute the adjusted amounts of stake each validator should get given the tallying rule
    // reward adjustments we computed before.
    // `compute_adjusted_reward_distribution` must be called before `distribute_reward` and `adjust_stake_and_computation_price` to
    // make sure we are using the current epoch's stake information to compute reward distribution.
    let (
        adjusted_staking_reward_amounts,
    ) = self.compute_adjusted_reward_distribution(
        total_voting_power,
        total_slashed_validator_voting_power,
        unadjusted_staking_reward_amounts,
        total_staking_reward_adjustment,
        individual_staking_reward_adjustments
    );

    // Distribute the rewards before adjusting stake so that we immediately start compounding
    // the rewards for validators and stakers.
    self.distribute_reward(
        epoch,
        &adjusted_staking_reward_amounts,
        total_reward,
        ctx,
    );

    self.adjust_stake_and_computation_price();

    self.process_pending_stakes_and_withdraws(new_epoch);

    self.previous_committee = self.active_committee;

    // Change to the next validator committee
    self.active_committee = self.next_epoch_committee.extract();

    // Activate validators that were added during `process_mid_epoch`
    self.activate_added_validators(new_epoch);

    // Emit events after we have processed all the rewards distribution and pending stakes.
    self.emit_validator_epoch_events(
        new_epoch,
        &adjusted_staking_reward_amounts,
        &slashed_validators,
    );

    self.total_stake = self.calculate_total_stakes();

    // At this point, self.active_committee are updated for next epoch.
    // Now we process the staged validator metadata.
    self.effectuate_staged_metadata();
}

// Activate validators added during `process_mid_epoch` and kept in `next_epoch_committee`.
fun activate_added_validators(
    self: &mut ValidatorSet,
    new_epoch: u64,
) {
    let members = *self.active_committee.members();
    members.do!(|member| {
        let validator = self.get_validator_mut(member.validator_id());
        if(validator.is_candidate()) {
            validator.activate(new_epoch);
            event::emit(ValidatorJoinEvent {
                epoch: new_epoch,
                validator_id: validator.validator_id(),
            });
        };
    });
}

fun update_and_process_low_stake_departures(
    self: &mut ValidatorSet,
    new_epoch: u64,
    low_stake_threshold: u64,
    very_low_stake_threshold: u64,
    low_stake_grace_period: u64,
) {
    let pending_active_validators = self.pending_active_validators;
    // Iterate through all the active validators, record their low stake status, and kick them out if the condition is met.
    let mut i = pending_active_validators.length();
    while (i > 0) {
        i = i - 1;
        let validator_id = pending_active_validators[i];

        let validator = self.get_validator_mut(validator_id);
        let stake = validator.total_stake_amount();
        if (stake >= low_stake_threshold) {
            // The validator is safe. We remove their entry from the at_risk map if there exists one.
            if (self.at_risk_validators.contains(&validator_id)) {
                self.at_risk_validators.remove(&validator_id);
            }
        } else if (stake >= very_low_stake_threshold) {
            // The stake is a bit below the threshold so we increment the entry of the validator in the map.
            let new_low_stake_period = if (self.at_risk_validators.contains(&validator_id)) {
                let num_epochs = &mut self.at_risk_validators[&validator_id];
                *num_epochs = *num_epochs + 1;
                *num_epochs
            } else {
                self.at_risk_validators.insert(validator_id, 1);
                1
            };

            // If the grace period has passed, the validator has to leave us.
            if (new_low_stake_period > low_stake_grace_period) {
                let _ = self.pending_active_validators.remove(i);
                process_validator_departure(
                    self,
                    new_epoch,
                    validator_id,
                    false, /* the validator is kicked out involuntarily */
                );
            }
        } else {
            // The validator's stake is lower than the very low threshold so we kick them out immediately.
            let _ = self.pending_active_validators.remove(i);
            process_validator_departure(
                self,
                new_epoch,
                validator_id,
                false, /* the validator is kicked out involuntarily */
            );
        }
    }
}

/// Effectuate pending next epoch metadata if they are staged.
fun effectuate_staged_metadata(self: &mut ValidatorSet) {
    let members = *self.active_committee.members();
    members.do!(|member| {
        let validator_id = member.validator_id();
        let validator = self.get_validator_mut(validator_id);
        validator.effectuate_staged_metadata();
    });
}

/// Called by `ika_system` to derive computation price per unit size for the new epoch.
/// Derive the computation price per unit size based on the computation price quote submitted by each validator.
/// The returned computation price should be greater than or equal to 2/3 of the validators submitted
/// computation price, weighted by stake.
public(package) fun derive_computation_price_per_unit_size(self: &mut ValidatorSet, committee: &BlsCommittee): u64 {
    let vs = committee.members();
    let num_validators = vs.length();
    let mut entries = vector[];
    let mut i = 0;
    while (i < num_validators) {
        let vid = vs[i].validator_id();

        let v = self.get_validator_ref(vid);
        entries.push_back(
            pq::new_entry(v.computation_price(), vs[i].voting_power()),
        );
        i = i + 1;
    };
    // Build a priority queue that will pop entries with computation price from the highest to the lowest.
    let mut pq = pq::new(entries);
    let mut sum = 0;
    let threshold = total_voting_power() - quorum_threshold();
    let mut result = 0;
    while (sum < threshold) {
        let (computation_price, voting_power) = pq.pop_max();
        result = computation_price;
        sum = sum + voting_power;
    };
    result
}

// ==== getter functions ====

public fun total_stake(self: &ValidatorSet): u64 {
    self.total_stake
}

public fun validator_total_stake_amount(self: &mut ValidatorSet, validator_id: ID): u64 {
    let validator = get_validator_ref(self, validator_id);
    validator.total_stake_amount()
}

public(package) fun pool_exchange_rates(
    self: &mut ValidatorSet,
    validator_id: ID,
): &Table<u64, PoolTokenExchangeRate> {
    let validator = self.get_validator_ref(validator_id);
    validator.get_staking_pool_ref().exchange_rates()
}

/// Get the total number of pending validators.
public(package) fun pending_active_validators_count(self: &ValidatorSet): u64 {
    self.pending_active_validators.length()
}

/// Returns true if exists in active validators.
public(package) fun is_active_validator(
    self: &ValidatorSet,
    validator_id: ID,
): bool {
    self.active_committee.contains(&validator_id)
}

/// Returns all the validators who are currently reporting `validator_id`
public(package) fun get_reporters_of(self: &ValidatorSet, validator_id: ID): VecSet<ID> {
    if (self.validator_report_records.contains(&validator_id)) {
        self.validator_report_records[&validator_id]
    } else {
        vec_set::empty()
    }
}
// ==== private helpers ====

fun count_duplicates_vec(
    self: &mut ValidatorSet,
    validators: &vector<ID>,
    validator: &ValidatorInnerV1
): u64 {
    let len = validators.length();
    let mut i = 0;
    let mut result = 0;
    while (i < len) {
        let vid = validators[i];
        let v = self.get_validator_mut(vid);
        if (v.is_duplicate(validator)) {
            result = result + 1;
        };
        i = i + 1;
    };
    result
}

public(package) fun is_duplicate_validator(
    self: &mut ValidatorSet,
    validators: &vector<ID>,
    new_validator: &ValidatorInnerV1,
): bool {
    count_duplicates_vec(self, validators, new_validator) > 0
}

/// Checks whether `new_validator` is duplicate with any currently active validators.
/// It differs from `is_active_validator` in that the former checks
/// only the id but this function looks at more metadata.
fun is_duplicate_with_active_validator(self: &mut ValidatorSet, new_validator: &ValidatorInnerV1): bool {
    let active_validator_ids = self.active_committee.validator_ids();
    is_duplicate_validator(self, &active_validator_ids, new_validator)
}

/// Checks whether `new_validator` is duplicate with any next epoch active validators.
fun is_duplicate_with_next_epoch_committee(self: &mut ValidatorSet, new_validator: &ValidatorInnerV1): bool {
    if(self.next_epoch_committee.is_none()) {
        return false
    };
    let next_epoch_active_validator_ids = self.next_epoch_committee.borrow().validator_ids();
    count_duplicates_vec(self, &next_epoch_active_validator_ids, new_validator) > 0
}

/// Checks whether `new_validator` is duplicate with any currently pending validators.
fun is_duplicate_with_pending_validator(self: &mut ValidatorSet, new_validator: &ValidatorInnerV1): bool {
    let pending_active_validators = self.pending_active_validators;
    count_duplicates_vec(self, &pending_active_validators, new_validator) > 0
}

public(package) fun get_validator_mut(
    self: &mut ValidatorSet,
    validator_id: ID,
): &mut ValidatorInnerV1 {
    assert!(self.validators.contains(validator_id), ENotAValidator);
    self.validators.borrow_mut(validator_id).load_validator_maybe_upgrade()
}

public fun get_validator_ref(self: &mut ValidatorSet, validator_id: ID): &ValidatorInnerV1 {
    self.get_validator_mut(validator_id)
}

/// Get mutable reference to either a candidate or an active validator by id.
fun get_candidate_or_active_validator_mut(
    self: &mut ValidatorSet,
    validator_id: ID,
): &mut ValidatorInnerV1 {
    let is_active_validator = self.is_active_validator(validator_id);
    let validator = self.get_validator_mut(validator_id);
    assert!(validator.is_candidate() || is_active_validator, ENotCandidateOrActiveValidator);
    validator
}

/// Get mutable reference to either a candidate or an active or an inactive validator by id.
fun get_candidate_or_active_or_inactive_validator_mut(
    self: &mut ValidatorSet,
    validator_id: ID,
): &mut ValidatorInnerV1 {
    let is_active_validator = self.is_active_validator(validator_id);
    let validator = self.get_validator_mut(validator_id);
    assert!(validator.is_candidate() || validator.is_inactive() || is_active_validator, ENotCandidateOrActiveOrInactiveValidator);
    validator
}


/// Get mutable reference to an active or (if active does not exist) pending or (if pending and
/// active do not exist) by id.
/// Note: this function should be called carefully, only after verifying the transaction
/// sender has the ability to modify the `Validator`.
fun get_active_or_pending_validator_mut(
    self: &mut ValidatorSet,
    validator_id: ID,
): &mut ValidatorInnerV1 {
    assert!(self.active_committee.contains(&validator_id) || self.pending_active_validators.contains(&validator_id), ENotActiveOrPendingValidator);
    let validator = self.get_validator_mut(validator_id);
    validator
}

/// Get mutable reference to an active or (if active does not exist) pending or (if pending and
/// active do not exist) or candidate validator by id.
/// Note: this function should be called carefully, only after verifying the transaction
/// sender has the ability to modify the `Validator`.
fun get_active_or_pending_or_candidate_validator_mut(
    self: &mut ValidatorSet,
    validator_id: ID,
): &mut ValidatorInnerV1 {
    let is_active_validator = self.is_active_validator(validator_id);
    let is_pending_active_validator = self.pending_active_validators.contains(&validator_id);

    let validator = self.get_validator_mut(validator_id);
    assert!(is_active_validator || is_pending_active_validator || validator.is_candidate(), ENotCandidateOrActiveOrPendingValidator);
    validator
}

public(package) fun get_validator_mut_with_operation_cap(
    self: &mut ValidatorSet,
    operation_cap: &ValidatorOperationCap,
): &mut ValidatorInnerV1 {
    let validator_id = operation_cap.validator_id();
    self.get_active_or_pending_validator_mut(validator_id)

}

public(package) fun get_validator_mut_with_operation_cap_including_candidates(
    self: &mut ValidatorSet,
    operation_cap: &ValidatorOperationCap,
): &mut ValidatorInnerV1 {
    let validator_id = operation_cap.validator_id();
    self.get_active_or_pending_or_candidate_validator_mut(validator_id)
}

public(package) fun get_validator_mut_with_cap(
    self: &mut ValidatorSet,
    cap: &ValidatorCap,
): &mut ValidatorInnerV1 {
    let validator_id = cap.validator_id();
    self.get_active_or_pending_validator_mut(validator_id)
}

public(package) fun get_validator_mut_with_cap_including_candidates(
    self: &mut ValidatorSet,
    cap: &ValidatorCap,
): &mut ValidatorInnerV1 {
    let validator_id = cap.validator_id();
    self.get_active_or_pending_or_candidate_validator_mut(validator_id)
}

/// Given a vector of validator ids to look for, return their indices in the validator vector.
/// Aborts if any id isn't in the given validator vector.
fun get_validator_indices(
    self: &ValidatorSet,
    look_for_indices_ids: &vector<ID>,
): vector<u64> {
    let validators = self.active_committee.validator_ids();
    let length = look_for_indices_ids.length();
    let mut i = 0;
    let mut res = vector[];
    while (i < length) {
        let validator_id = look_for_indices_ids[i];
        let (found, index_opt) = validators.index_of(&validator_id);
        assert!(found, ENotAValidator);
        res.push_back(index_opt);
        i = i + 1;
    };
    res
}

/// Verify the operation capability is valid for a Validator.
public(package) fun verify_operation_cap(
    self: &mut ValidatorSet,
    cap: &ValidatorOperationCap,
) {
    let validator_id = cap.validator_id();
    let validator = self.get_validator_ref(validator_id);
    assert!(validator.operation_cap_id() == &object::id(cap), EInvalidCap);
}

fun process_validator_departure(
    self: &mut ValidatorSet,
    new_epoch: u64,
    validator_id: ID,
    is_voluntary: bool,
) {
    if (self.at_risk_validators.contains(&validator_id)) {
        self.at_risk_validators.remove(&validator_id);
    };

    clean_report_records_leaving_validator(&mut self.validator_report_records, validator_id);

    let validator = self.get_validator_mut(validator_id);

    let validator_stake = validator.total_stake_amount();

    // Deactivate the validator and its staking pool
    validator.deactivate(new_epoch);

    self.total_stake = self.total_stake - validator_stake;

    event::emit(ValidatorLeaveEvent {
        epoch: new_epoch,
        validator_id,
        is_voluntary,
    });
}

fun clean_report_records_leaving_validator(
    validator_report_records: &mut VecMap<ID, VecSet<ID>>,
    leaving_validator_id: ID,
) {
    // Remove the records about this validator
    if (validator_report_records.contains(&leaving_validator_id)) {
        validator_report_records.remove(&leaving_validator_id);
    };

    // Remove the reports submitted by this validator
    let reported_validators = validator_report_records.keys();
    let length = reported_validators.length();
    let mut i = 0;
    while (i < length) {
        let reported_validator_id = &reported_validators[i];
        let reporters = &mut validator_report_records[reported_validator_id];
        if (reporters.contains(&leaving_validator_id)) {
            reporters.remove(&leaving_validator_id);
            if (reporters.is_empty()) {
                validator_report_records.remove(reported_validator_id);
            };
        };
        i = i + 1;
    }
}

/// Process the pending new validators. They will be `next_epoch_committee` and activated during `advance_epoch`.
fun process_pending_validators(self: &mut ValidatorSet) {
    let mut next_epoch_active_members = vector[];
    let mut i = 0;
    let length = self.pending_active_validators.length();
    while (i < length) {
        let validator_id = self.pending_active_validators[i];
        let validator = self.get_validator_mut(validator_id);
        next_epoch_active_members.push_back(new_bls_committee_member(validator_id, *validator.protocol_pubkey(), validator.total_stake_amount()));
        i = i + 1;
    };
    let next_epoch_committee = new_bls_committee(next_epoch_active_members);
    self.next_epoch_committee.fill(next_epoch_committee);
}

/// Process all active validators' pending stake deposits and withdraws.
fun process_pending_stakes_and_withdraws(self: &mut ValidatorSet, new_epoch: u64) {
    let members = *self.active_committee.members();
    members.do!(|member| {
        let validator_id = member.validator_id();
        let validator = self.get_validator_mut(validator_id);
        validator.process_pending_stakes_and_withdraws(new_epoch);
    });
}

/// Calculate the total active validator stake.
fun calculate_total_stakes(self: &mut ValidatorSet): u64 {
    let mut stake = 0;
    let members = *self.active_committee.members();
    members.do!(|member| {
        let validator_id = member.validator_id();
        let validator = self.get_validator_mut(validator_id);
        stake = stake + validator.total_stake_amount();
    });

    stake
}

/// Process the pending stake changes for each validator.
fun adjust_stake_and_computation_price(
    self: &mut ValidatorSet,
) {
    let members = *self.active_committee.members();

    members.do!(|member| {
        let validator_id = member.validator_id();
        let validator = self.get_validator_mut(validator_id);
        validator.adjust_stake_and_computation_price();
    });
}

/// Compute both the individual reward adjustments and total reward adjustment for staking rewards
/// as well as storage fund rewards.
fun compute_reward_adjustments(
    mut slashed_validator_indices: vector<u64>,
    reward_slashing_rate: u16,
    unadjusted_staking_reward_amounts: &vector<u64>,
): (
    u64, // sum of staking reward adjustments
    VecMap<u64, u64>, // mapping of individual validator's staking reward adjustment from index -> amount
) {
    let mut total_staking_reward_adjustment = 0;
    let mut individual_staking_reward_adjustments = vec_map::empty();

    while (!slashed_validator_indices.is_empty()) {
        let validator_index = slashed_validator_indices.pop_back();

        // Use the slashing rate to compute the amount of staking rewards slashed from this punished validator.
        let unadjusted_staking_reward = unadjusted_staking_reward_amounts[validator_index];
        let staking_reward_adjustment_u128 =
            unadjusted_staking_reward as u128 * (reward_slashing_rate as u128)
                / BASIS_POINT_DENOMINATOR;

        // Insert into individual mapping and record into the total adjustment sum.
        individual_staking_reward_adjustments.insert(
            validator_index,
            staking_reward_adjustment_u128 as u64,
        );
        total_staking_reward_adjustment =
            total_staking_reward_adjustment + (staking_reward_adjustment_u128 as u64);

    };

    (
        total_staking_reward_adjustment,
        individual_staking_reward_adjustments,
    )
}

/// Process the validator report records of the epoch and return the ids of the
/// non-performant validators according to the input threshold.
fun compute_slashed_validators(
    self: &mut ValidatorSet,
): vector<ID> {
    let mut slashed_validators = vector[];
    while (!self.validator_report_records.is_empty()) {
        let (validator_id, reporters) = self.validator_report_records.pop();
        assert!(
            is_active_validator(self, validator_id),
            ENonValidatorInReportRecords,
        );
        // Sum up the voting power of validators that have reported this validator and check if it has
        // passed the slashing threshold.
        let reporter_votes = sum_voting_power_by_validator_indices(
            self,
            reporters.into_keys(),
        );
        if (reporter_votes >= quorum_threshold()) {
            slashed_validators.push_back(validator_id);
        }
    };
    slashed_validators
}

/// Given the current list of active validators, the total stake and total reward,
/// calculate the amount of reward each validator should get, without taking into
/// account the tallying rule results.
/// Returns the unadjusted amounts of staking reward for each validator.
fun compute_unadjusted_reward_distribution(
    self: &ValidatorSet,
    total_voting_power: u64,
    total_reward: u64,
): vector<u64> {
    let members = self.active_committee.members();
    let reward_amounts = members.map_ref!(|member| {
        // Integer divisions will truncate the results. Because of this, we expect that at the end
        // there will be some reward remaining in `total_reward`.
        // Use u128 to avoid multiplication overflow.
        let voting_power: u128 = member.voting_power() as u128;
        let reward_amount =
            voting_power * (total_reward as u128) / (total_voting_power as u128);
        reward_amount as u64
    });
    reward_amounts
}

/// Use the reward adjustment info to compute the adjusted rewards each validator should get.
/// Returns the staking rewards each validator gets.
/// The staking rewards are shared with the stakers.
fun compute_adjusted_reward_distribution(
    self: &ValidatorSet,
    total_voting_power: u64,
    total_slashed_validator_voting_power: u64,
    unadjusted_staking_reward_amounts: vector<u64>,
    total_staking_reward_adjustment: u64,
    individual_staking_reward_adjustments: VecMap<u64, u64>,
): vector<u64> {
    let total_unslashed_validator_voting_power =
        total_voting_power - total_slashed_validator_voting_power;
    let mut adjusted_staking_reward_amounts = vector[];
    let members = self.active_committee.members();
    let mut i = 0;
    let length = members.length();

    while (i < length) {
        // Integer divisions will truncate the results. Because of this, we expect that at the end
        // there will be some reward remaining in `total_reward`.
        // Use u128 to avoid multiplication overflow.
        let voting_power = members[i].voting_power() as u128;

        // Compute adjusted staking reward.
        let unadjusted_staking_reward_amount = unadjusted_staking_reward_amounts[i];
        let adjusted_staking_reward_amount = // If the validator is one of the slashed ones, then subtract the adjustment.
        if (individual_staking_reward_adjustments.contains(&i)) {
            let adjustment = individual_staking_reward_adjustments[&i];
            unadjusted_staking_reward_amount - adjustment
        } else {
            // Otherwise the slashed rewards should be distributed among the unslashed
            // validators so add the corresponding adjustment.
            let adjustment =
                total_staking_reward_adjustment as u128 * voting_power
                                   / (total_unslashed_validator_voting_power as u128);
            unadjusted_staking_reward_amount + (adjustment as u64)
        };
        adjusted_staking_reward_amounts.push_back(adjusted_staking_reward_amount);
        i = i + 1;
    };
    adjusted_staking_reward_amounts
}

fun distribute_reward(
    self: &mut ValidatorSet,
    epoch: u64,
    adjusted_staking_reward_amounts: &vector<u64>,
    staking_rewards: &mut Balance<IKA>,
    ctx: &mut TxContext,
) {
    let members = *self.active_committee.members();
    let length = members.length();
    assert!(length > 0, EValidatorSetEmpty);
    let mut i = 0;
    while (i < length) {
        let validator_id = members[i].validator_id();
        let validator = self.get_validator_mut(validator_id);
        let staking_reward_amount = adjusted_staking_reward_amounts[i];
        let mut staker_reward = staking_rewards.split(staking_reward_amount);

        // Validator takes a cut of the rewards as commission.
        let validator_commission_amount =
            (staking_reward_amount as u128) * (validator.commission_rate() as u128) / BASIS_POINT_DENOMINATOR;

        // The validator reward commission.
        let validator_reward = staker_reward.split(validator_commission_amount as u64);

        // Add rewards to the validator. Don't try and distribute rewards though if the payout is zero.
        if (validator_reward.value() > 0) {
            let validator_address = validator.payment_address();
            let rewards_stake = validator.request_add_stake(
                epoch,
                validator_reward,
                ctx,
            );
            transfer::public_transfer(rewards_stake, validator_address);
        } else {
            validator_reward.destroy_zero();
        };

        // Add rewards to stake staking pool to auto compound for stakers.
        validator.deposit_stake_rewards(staker_reward);
        i = i + 1;
    }
}

/// Emit events containing information of each validator for the epoch,
/// including stakes, rewards, performance, etc.
fun emit_validator_epoch_events(
    self: &mut ValidatorSet,
    new_epoch: u64,
    pool_staking_reward_amounts: &vector<u64>,
    slashed_validators: &vector<ID>,
) {
    let members = *self.active_committee.members();
    let num_validators = members.length();
    let mut i = 0;
    while (i < num_validators) {
        let member = members[i];
        let validator_id = member.validator_id();
        let validator = self.get_validator_ref(validator_id);
        let tallying_rule_reporters = if (self.validator_report_records.contains(&validator_id)) {
            self.validator_report_records[&validator_id].into_keys()
        } else {
            vector[]
        };
        let tallying_rule_global_score = if (slashed_validators.contains(&validator_id)) 0
        else 1;
        event::emit(ValidatorEpochInfoEventV1 {
            epoch: new_epoch,
            validator_id,
            reference_gas_survey_quote: validator.computation_price(),
            stake: validator.total_stake_amount(),
            voting_power: member.voting_power(),
            commission_rate: validator.commission_rate(),
            pool_staking_reward: pool_staking_reward_amounts[i],
            pool_token_exchange_rate: validator.pool_token_exchange_rate_at_epoch(new_epoch),
            tallying_rule_reporters,
            tallying_rule_global_score,
        });
        i = i + 1;
    }
}

/// Sum up the total stake of a given list of validator indices.
public fun sum_voting_power_by_validator_indices(self: &mut ValidatorSet, validator_ids: vector<ID>): u64 {
    let validator_indices = get_validator_indices(self, &validator_ids);
    let members = self.active_committee.members();
    let sum = validator_indices.fold!(0, |s, i|  {
        s + members[i].voting_power()
    });
    sum
}

/// Report a validator as a bad or non-performant actor in the system.
/// Succeeds if all the following are satisfied:
/// 1. both the reporter in `cap` and the input `reportee_id` are active validators.
/// 2. reporter and reportee not the same address.
/// 3. the cap object is still valid.
/// This function is idempotent.
public(package) fun report_validator(
    self: &mut ValidatorSet,
    operation_cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    // Reportee needs to be an active validator
    assert!(self.is_active_validator(reportee_id), ENotAValidator);
    // Verify the represented reporter address is an active validator, and the capability is still valid.
    assert!(self.is_active_validator(operation_cap.validator_id()), ENotAValidator);
    self.verify_operation_cap(operation_cap);
    report_validator_impl(operation_cap, reportee_id, &mut self.validator_report_records);
}

/// Undo a `report_validator` action. Aborts if
/// 1. the reportee is not a currently active validator or
/// 2. the sender has not previously reported the `reportee_id`, or
/// 3. the cap is not valid
public(package) fun undo_report_validator(
    self: &mut ValidatorSet,
    operation_cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    // Verify the represented reporter address is an active validator, and the capability is still valid.
    assert!(self.is_active_validator(operation_cap.validator_id()), ENotAValidator);
    self.verify_operation_cap(operation_cap);
    undo_report_validator_impl(operation_cap, reportee_id, &mut self.validator_report_records);
}

fun report_validator_impl(
    operation_cap: &ValidatorOperationCap,
    reportee_id: ID,
    validator_report_records: &mut VecMap<ID, VecSet<ID>>,
) {
    let reporter_id = operation_cap.validator_id();
    assert!(reporter_id != reportee_id, ECannotReportOneself);
    if (!validator_report_records.contains(&reportee_id)) {
        validator_report_records.insert(reportee_id, vec_set::singleton(reporter_id));
    } else {
        let reporters = validator_report_records.get_mut(&reportee_id);
        if (!reporters.contains(&reporter_id)) {
            reporters.insert(reporter_id);
        }
    }
}

fun undo_report_validator_impl(
    operation_cap: &ValidatorOperationCap,
    reportee_id: ID,
    validator_report_records: &mut VecMap<ID, VecSet<ID>>,
) {
    assert!(validator_report_records.contains(&reportee_id), EReportRecordNotFound);
    let reporters = validator_report_records.get_mut(&reportee_id);

    let reporter_id = operation_cap.validator_id();
    assert!(reporters.contains(&reporter_id), EReportRecordNotFound);

    reporters.remove(&reporter_id);
    if (reporters.is_empty()) {
        validator_report_records.remove(&reportee_id);
    }
}

/// Return the active validators in `self`
public fun active_committee(self: &ValidatorSet): BlsCommittee {
    self.active_committee
}

/// Return the next epoch active committee in `self`
public fun next_epoch_committee(self: &ValidatorSet): Option<BlsCommittee> {
    self.next_epoch_committee
}

/// Return the pending active validators in `self`
public fun next_pending_active_validators(self: &ValidatorSet): vector<ID> {
    self.pending_active_validators
}

/// Returns true if the `validator_id` is a validator candidate.
public fun is_validator_candidate(self: &mut ValidatorSet, validator_id: ID): bool {
    let validator = self.get_validator_ref(validator_id);
    validator.is_candidate()
}

/// Returns true if the staking pool identified by `validator_id` is of an inactive validator.
public fun is_inactive_validator(self: &mut ValidatorSet, validator_id: ID): bool {
    let validator = self.get_validator_ref(validator_id);
    validator.is_inactive()
}
