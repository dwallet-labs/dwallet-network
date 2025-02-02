// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::staking_pool;

use ika::ika::IKA;
use sui::bag::{Self, Bag};
use sui::balance::{Self, Balance};
use sui::table::{Self, Table};
use ika_system::staked_ika::{Self, StakedIka, FungibleStakedIka};

const EWrongPool: u64 = 1;
const ETokenBalancesDoNotMatchExchangeRate: u64 = 9;
const EDelegationToInactivePool: u64 = 10;
const EDeactivationOfInactivePool: u64 = 11;
const EPoolAlreadyActive: u64 = 14;
const EActivationOfInactivePool: u64 = 16;
const EDelegationOfZeroIka: u64 = 17;
const ECannotMintFungibleStakedIkaYet: u64 = 19;
const EInvariantFailure: u64 = 20;

/// A staking pool embedded in each validator struct in the system state object.
public struct StakingPool has store {
    validator_id: ID,
    /// The epoch at which this pool became active.
    /// The value is `None` if the pool is pre-active and `Some(<epoch_number>)` if active or inactive.
    activation_epoch: Option<u64>,
    /// The epoch at which this staking pool ceased to be active. `None` = {pre-active, active},
    /// `Some(<epoch_number>)` if in-active, and it was de-activated at epoch `<epoch_number>`.
    deactivation_epoch: Option<u64>,
    /// The total number of IKA tokens in this pool, including the IKA in the rewards_pool, as well as in all the principal
    /// in the `StakedIka` object, updated at epoch boundaries.
    ika_balance: u64,
    /// The epoch stake rewards will be added here at the end of each epoch.
    rewards_pool: Balance<IKA>,
    /// Total number of pool tokens issued by the pool.
    pool_token_balance: u64,
    /// Exchange rate history of previous epochs. Key is the epoch number.
    /// The entries start from the `activation_epoch` of this pool and contains exchange rates at the beginning of each epoch,
    /// i.e., right after the rewards for the previous epoch have been deposited into the pool.
    exchange_rates: Table<u64, PoolTokenExchangeRate>,
    /// Pending stake amount for this epoch, emptied at epoch boundaries.
    pending_stake: u64,
    /// Pending stake withdrawn during the current epoch, emptied at epoch boundaries.
    /// This includes both the principal and rewards IKA withdrawn.
    pending_total_ika_withdraw: u64,
    /// Pending pool token withdrawn during the current epoch, emptied at epoch boundaries.
    pending_pool_token_withdraw: u64,
    /// The total supply of fungible staked ika
    fungible_total_supply: u64,
    /// The total principal balance of fungible staked ika, without rewards which
    /// are withdrawn from the reward pool.
    fungible_principal: Balance<IKA>,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

/// Struct representing the exchange rate of the stake pool token to IKA.
public struct PoolTokenExchangeRate has store, copy, drop {
    ika_amount: u64,
    pool_token_amount: u64,
}

// ==== initializer ====

/// Create a new, empty staking pool.
public(package) fun new(validator_id: ID, ctx: &mut TxContext): StakingPool {
    let exchange_rates = table::new(ctx);
    StakingPool {
        validator_id,
        activation_epoch: option::none(),
        deactivation_epoch: option::none(),
        ika_balance: 0,
        rewards_pool: balance::zero(),
        pool_token_balance: 0,
        exchange_rates,
        pending_stake: 0,
        pending_total_ika_withdraw: 0,
        pending_pool_token_withdraw: 0,
        fungible_total_supply: 0,
        fungible_principal: balance::zero(),
        extra_fields: bag::new(ctx),
    }
}

// ==== stake requests ====

/// Request to stake to a staking pool. The stake starts counting at the beginning of the next epoch,
public(package) fun request_add_stake(
    pool: &mut StakingPool,
    stake: Balance<IKA>,
    stake_activation_epoch: u64,
    validator_id: ID,
    ctx: &mut TxContext,
): StakedIka {
    let ika_amount = stake.value();
    assert!(!is_inactive(pool), EDelegationToInactivePool);
    assert!(ika_amount > 0, EDelegationOfZeroIka);
    let staked_ika = staked_ika::create(
        validator_id,
        stake_activation_epoch,
        stake,
        ctx
    );
    pool.pending_stake = pool.pending_stake + ika_amount;
    staked_ika
}

/// Request to withdraw the given stake plus rewards from a staking pool.
/// Both the principal and corresponding rewards in IKA are withdrawn.
/// A proportional amount of pool token withdraw is recorded and processed at epoch change time.
public(package) fun request_withdraw_stake(
    pool: &mut StakingPool,
    epoch: u64,
    staked_ika: StakedIka,
): Balance<IKA> {
    // stake is inactive
    if (staked_ika.stake_activation_epoch() > epoch && epoch != 0) {
        let principal = staked_ika.into_balance();
        pool.pending_stake = pool.pending_stake - principal.value();

        return principal
    };

    let (pool_token_withdraw_amount, mut principal_withdraw) = withdraw_from_principal(
        pool,
        staked_ika,
    );
    let principal_withdraw_amount = principal_withdraw.value();

    let rewards_withdraw = withdraw_rewards(
        pool,
        principal_withdraw_amount,
        pool_token_withdraw_amount,
        epoch,
    );
    let total_ika_withdraw_amount = principal_withdraw_amount + rewards_withdraw.value();

    pool.pending_total_ika_withdraw = pool.pending_total_ika_withdraw + total_ika_withdraw_amount;
    pool.pending_pool_token_withdraw =
        pool.pending_pool_token_withdraw + pool_token_withdraw_amount;

    // If the pool is inactive, we immediately process the withdrawal.
    if (is_inactive(pool)) process_pending_stake_withdraw(pool);

    // TODO: implement withdraw bonding period here.
    principal_withdraw.join(rewards_withdraw);
    principal_withdraw
}

public(package) fun redeem_fungible_staked_ika(
    self: &mut StakingPool,
    epoch: u64,
    fungible_staked_ika: FungibleStakedIka,
): Balance<IKA> {
    let validator_id = fungible_staked_ika.validator_id();
    let value = fungible_staked_ika.value();

    assert!(validator_id == self.validator_id, EWrongPool);

    fungible_staked_ika.destroy();

    let latest_exchange_rate = self.pool_token_exchange_rate_at_epoch(epoch);

    let (principal_amount, rewards_amount) = calculate_fungible_staked_ika_withdraw_amount(
        latest_exchange_rate,
        value,
        balance::value(&self.fungible_principal),
        self.fungible_total_supply,
    );

    self.fungible_total_supply = self.fungible_total_supply - value;

    let mut ika_out = balance::split(&mut self.fungible_principal, principal_amount);
    balance::join(
        &mut ika_out,
        balance::split(&mut self.rewards_pool, rewards_amount),
    );

    self.pending_total_ika_withdraw = self.pending_total_ika_withdraw + balance::value(&ika_out);
    self.pending_pool_token_withdraw = self.pending_pool_token_withdraw + value;

    ika_out
}

/// written in separate function so i can test with random values
/// returns (principal_withdraw_amount, rewards_withdraw_amount)
fun calculate_fungible_staked_ika_withdraw_amount(
    latest_exchange_rate: PoolTokenExchangeRate,
    fungible_staked_ika_value: u64,
    fungible_staked_ika_principal_amount: u64, // fungible_staked_ika_data.principal.value()
    fungible_staked_ika_total_supply: u64, // fungible_staked_ika_data.total_supply
): (u64, u64) {
    // 1. if the entire fungible staked total supply supply is redeemed, how much ika should we receive?
    let total_ika_amount = get_ika_amount(
        &latest_exchange_rate,
        fungible_staked_ika_total_supply,
    );

    // min with total_ika_amount to prevent underflow
    let fungible_staked_ika_principal_amount = std::u64::min(
        fungible_staked_ika_principal_amount,
        total_ika_amount,
    );

    // 2. how much do we need to withdraw from the rewards pool?
    let total_rewards = total_ika_amount - fungible_staked_ika_principal_amount;

    // 3. proportionally withdraw from both wrt the fungible_staked_ika_value.
    let principal_withdraw_amount =
        (
            (fungible_staked_ika_value as u128)
            * (fungible_staked_ika_principal_amount as u128)
            / (fungible_staked_ika_total_supply as u128),
        ) as u64;

    let rewards_withdraw_amount =
        (
            (fungible_staked_ika_value as u128)
            * (total_rewards as u128)
            / (fungible_staked_ika_total_supply as u128),
        ) as u64;

    // invariant check, just in case
    let expected_ika_amount = get_ika_amount(&latest_exchange_rate, fungible_staked_ika_value);
    assert!(
        principal_withdraw_amount + rewards_withdraw_amount <= expected_ika_amount,
        EInvariantFailure,
    );

    (principal_withdraw_amount, rewards_withdraw_amount)
}

/// Convert the given staked IKA to an FungibleStakedIka object
public(package) fun convert_to_fungible_staked_ika(
    self: &mut StakingPool,
    epoch: u64,
    staked_ika: StakedIka,
    ctx: &mut TxContext,
): FungibleStakedIka {
    let validator_id = staked_ika.validator_id();
    let stake_activation_epoch = staked_ika.stake_activation_epoch();
    let principal = staked_ika.into_balance();

    assert!(validator_id == self.validator_id, EWrongPool);
    assert!(epoch >= stake_activation_epoch, ECannotMintFungibleStakedIkaYet);


    let exchange_rate_at_staking_epoch = self.pool_token_exchange_rate_at_epoch(
        stake_activation_epoch,
    );

    let pool_token_amount = get_token_amount(
        &exchange_rate_at_staking_epoch,
        balance::value(&principal),
    );

    self.fungible_total_supply = self.fungible_total_supply + pool_token_amount;
    balance::join(&mut self.fungible_principal, principal);

    staked_ika::create_fungible(validator_id, pool_token_amount, ctx)
}

/// Withdraw the principal IKA stored in the StakedIka object, and calculate the corresponding amount of pool
/// tokens using exchange rate at staking epoch.
/// Returns values are amount of pool tokens withdrawn and withdrawn principal portion of IKA.
public(package) fun withdraw_from_principal(
    pool: &StakingPool,
    staked_ika: StakedIka,
): (u64, Balance<IKA>) {
    // Check that the stake information matches the pool.
    assert!(staked_ika.validator_id() == pool.validator_id, EWrongPool);

    let exchange_rate_at_staking_epoch = pool_token_exchange_rate_at_epoch(
        pool,
        staked_ika.stake_activation_epoch(),
    );
    let principal_withdraw = staked_ika.into_balance();
    let pool_token_withdraw_amount = get_token_amount(
        &exchange_rate_at_staking_epoch,
        principal_withdraw.value(),
    );

    (pool_token_withdraw_amount, principal_withdraw)
}

// ==== functions called at epoch boundaries ===

/// Called at epoch advancement times to add rewards (in IKA) to the staking pool.
public(package) fun deposit_rewards(pool: &mut StakingPool, rewards: Balance<IKA>) {
    pool.ika_balance = pool.ika_balance + rewards.value();
    pool.rewards_pool.join(rewards);
}

public(package) fun process_pending_stakes_and_withdraws(pool: &mut StakingPool, new_epoch: u64) {
    process_pending_stake_withdraw(pool);
    process_pending_stake(pool);
    pool
        .exchange_rates
        .add(
            new_epoch,
            PoolTokenExchangeRate {
                ika_amount: pool.ika_balance,
                pool_token_amount: pool.pool_token_balance,
            },
        );
    check_balance_invariants(pool, new_epoch);
}

/// Called at epoch boundaries to process pending stake withdraws requested during the epoch.
/// Also called immediately upon withdrawal if the pool is inactive.
fun process_pending_stake_withdraw(pool: &mut StakingPool) {
    pool.ika_balance = pool.ika_balance - pool.pending_total_ika_withdraw;
    pool.pool_token_balance = pool.pool_token_balance - pool.pending_pool_token_withdraw;
    pool.pending_total_ika_withdraw = 0;
    pool.pending_pool_token_withdraw = 0;
}

/// Called at epoch boundaries to process the pending stake.
public(package) fun process_pending_stake(pool: &mut StakingPool) {
    // Use the most up to date exchange rate with the rewards deposited and withdraws effectuated.
    let latest_exchange_rate = PoolTokenExchangeRate {
        ika_amount: pool.ika_balance,
        pool_token_amount: pool.pool_token_balance,
    };
    pool.ika_balance = pool.ika_balance + pool.pending_stake;
    pool.pool_token_balance = get_token_amount(&latest_exchange_rate, pool.ika_balance);
    pool.pending_stake = 0;
}

/// This function does the following:
///     1. Calculates the total amount of IKA (including principal and rewards) that the provided pool tokens represent
///        at the current exchange rate.
///     2. Using the above number and the given `principal_withdraw_amount`, calculates the rewards portion of the
///        stake we should withdraw.
///     3. Withdraws the rewards portion from the rewards pool at the current exchange rate. We only withdraw the rewards
///        portion because the principal portion was already taken out of the staker's self custodied StakedIka.
fun withdraw_rewards(
    pool: &mut StakingPool,
    principal_withdraw_amount: u64,
    pool_token_withdraw_amount: u64,
    epoch: u64,
): Balance<IKA> {
    let exchange_rate = pool_token_exchange_rate_at_epoch(pool, epoch);
    let total_ika_withdraw_amount = get_ika_amount(&exchange_rate, pool_token_withdraw_amount);
    let mut reward_withdraw_amount = if (total_ika_withdraw_amount >= principal_withdraw_amount) {
        total_ika_withdraw_amount - principal_withdraw_amount
    } else 0;
    // This may happen when we are withdrawing everything from the pool and
    // the rewards pool balance may be less than reward_withdraw_amount.
    // TODO: FIGURE OUT EXACTLY WHY THIS CAN HAPPEN.
    reward_withdraw_amount = reward_withdraw_amount.min(pool.rewards_pool.value());
    pool.rewards_pool.split(reward_withdraw_amount)
}

// ==== preactive pool related ====

/// Called by `validator` module to activate a staking pool.
public(package) fun activate_staking_pool(pool: &mut StakingPool, activation_epoch: u64) {
    // Add the initial exchange rate to the table.
    pool
        .exchange_rates
        .add(
            activation_epoch,
            initial_exchange_rate(),
        );
    // Check that the pool is preactive and not inactive.
    assert!(is_candidate(pool), EPoolAlreadyActive);
    assert!(!is_inactive(pool), EActivationOfInactivePool);
    // Fill in the active epoch.
    pool.activation_epoch.fill(activation_epoch);
}

// ==== inactive pool related ====

/// Deactivate a staking pool by setting the `deactivation_epoch`. After
/// this pool deactivation, the pool stops earning rewards. Only stake
/// withdraws can be made to the pool.
public(package) fun deactivate_staking_pool(pool: &mut StakingPool, deactivation_epoch: u64) {
    // We can't deactivate an already deactivated pool.
    assert!(!is_inactive(pool), EDeactivationOfInactivePool);
    pool.deactivation_epoch = option::some(deactivation_epoch);
}

// ==== getters and misc utility functions ====

public fun ika_balance(pool: &StakingPool): u64 { pool.ika_balance }

/// Returns true if the input staking pool is candidate.
public fun is_candidate(pool: &StakingPool): bool {
    pool.activation_epoch.is_none()
}

/// Returns true if the input staking pool is inactive.
public fun is_inactive(pool: &StakingPool): bool {
    pool.deactivation_epoch.is_some()
}


public fun pool_token_exchange_rate_at_epoch(
    pool: &StakingPool,
    epoch: u64,
): PoolTokenExchangeRate {
    // If the pool is preactive then the exchange rate is always 1:1.
    if (is_candidate_at_epoch(pool, epoch)) {
        return initial_exchange_rate()
    };
    let clamped_epoch = pool.deactivation_epoch.get_with_default(epoch);
    let mut epoch = clamped_epoch.min(epoch);
    let activation_epoch = *pool.activation_epoch.borrow();

    // Find the latest epoch that's earlier than the given epoch with an entry in the table
    while (epoch >= activation_epoch) {
        if (pool.exchange_rates.contains(epoch)) {
            return pool.exchange_rates[epoch]
        };
        epoch = epoch - 1;
    };
    // This line really should be unreachable. Do we want an assert false here?
    initial_exchange_rate()
}

/// Returns the total value of the pending staking requests for this staking pool.
public fun pending_stake_amount(staking_pool: &StakingPool): u64 {
    staking_pool.pending_stake
}

/// Returns the total withdrawal from the staking pool this epoch.
public fun pending_stake_withdraw_amount(staking_pool: &StakingPool): u64 {
    staking_pool.pending_total_ika_withdraw
}

public(package) fun exchange_rates(pool: &StakingPool): &Table<u64, PoolTokenExchangeRate> {
    &pool.exchange_rates
}

public fun ika_amount(exchange_rate: &PoolTokenExchangeRate): u64 {
    exchange_rate.ika_amount
}

public fun pool_token_amount(exchange_rate: &PoolTokenExchangeRate): u64 {
    exchange_rate.pool_token_amount
}

/// Returns true if the provided staking pool is preactive at the provided epoch.
fun is_candidate_at_epoch(pool: &StakingPool, epoch: u64): bool {
    // Either the pool is currently preactive or the pool's starting epoch is later than the provided epoch.
    is_candidate(pool) || (*pool.activation_epoch.borrow() > epoch)
}

fun get_ika_amount(exchange_rate: &PoolTokenExchangeRate, token_amount: u64): u64 {
    // When either amount is 0, that means we have no stakes with this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    if (exchange_rate.ika_amount == 0 || exchange_rate.pool_token_amount == 0) {
        return token_amount
    };
    let res =
        exchange_rate.ika_amount as u128
                * (token_amount as u128)
                / (exchange_rate.pool_token_amount as u128);
    res as u64
}

fun get_token_amount(exchange_rate: &PoolTokenExchangeRate, ika_amount: u64): u64 {
    // When either amount is 0, that means we have no stakes with this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    if (exchange_rate.ika_amount == 0 || exchange_rate.pool_token_amount == 0) {
        return ika_amount
    };
    let res =
        exchange_rate.pool_token_amount as u128
                * (ika_amount as u128)
                / (exchange_rate.ika_amount as u128);
    res as u64
}

fun initial_exchange_rate(): PoolTokenExchangeRate {
    PoolTokenExchangeRate { ika_amount: 0, pool_token_amount: 0 }
}

fun check_balance_invariants(pool: &StakingPool, epoch: u64) {
    let exchange_rate = pool_token_exchange_rate_at_epoch(pool, epoch);
    // check that the pool token balance and ika balance ratio matches the exchange rate stored.
    let expected = get_token_amount(&exchange_rate, pool.ika_balance);
    let actual = pool.pool_token_balance;
    assert!(expected == actual, ETokenBalancesDoNotMatchExchangeRate)
}

// ==== test-related functions ====

// Given the `staked_ika` receipt calculate the current rewards (in terms of IKA) for it.
#[test_only]
public fun calculate_rewards(pool: &StakingPool, staked_ika: &StakedIka, current_epoch: u64): u64 {
    let staked_amount = staked_ika.amount();
    let pool_token_withdraw_amount = {
        let exchange_rate_at_staking_epoch = pool_token_exchange_rate_at_epoch(
            pool,
            staked_ika.stake_activation_epoch(),
        );
        get_token_amount(&exchange_rate_at_staking_epoch, staked_amount)
    };

    let new_epoch_exchange_rate = pool_token_exchange_rate_at_epoch(pool, current_epoch);
    let total_ika_withdraw_amount = get_ika_amount(
        &new_epoch_exchange_rate,
        pool_token_withdraw_amount,
    );

    let mut reward_withdraw_amount = if (total_ika_withdraw_amount >= staked_amount) {
        total_ika_withdraw_amount - staked_amount
    } else 0;
    reward_withdraw_amount = reward_withdraw_amount.min(pool.rewards_pool.value());

    staked_amount + reward_withdraw_amount
}

#[test_only]
public(package) fun pending_pool_token_withdraw_amount(pool: &StakingPool): u64 {
    pool.pending_pool_token_withdraw
}

// ==== tests ====

#[random_test]
fun test_calculate_fungible_staked_ika_withdraw_amount(
    mut total_ika_amount: u64,
    // these are all in basis points
    mut pool_token_frac: u16,
    mut fungible_staked_ika_data_total_supply_frac: u16,
    mut fungible_staked_ika_data_principal_frac: u16,
    mut fungible_staked_ika_value_bps: u16,
) {
    use std::u128::max;

    total_ika_amount = std::u64::max(total_ika_amount, 1);

    pool_token_frac = pool_token_frac % 10000;
    fungible_staked_ika_data_total_supply_frac = fungible_staked_ika_data_total_supply_frac % 10000;
    fungible_staked_ika_data_principal_frac = fungible_staked_ika_data_principal_frac % 10000;
    fungible_staked_ika_value_bps = fungible_staked_ika_value_bps % 10000;

    let total_pool_token_amount = max(
        (total_ika_amount as u128) * (pool_token_frac as u128) / 10000,
        1,
    );

    let exchange_rate = PoolTokenExchangeRate {
        ika_amount: total_ika_amount,
        pool_token_amount: total_pool_token_amount as u64,
    };

    let fungible_staked_ika_data_total_supply = max(
        total_pool_token_amount * (fungible_staked_ika_data_total_supply_frac as u128) / 10000,
        1,
    );
    let fungible_staked_ika_value =
        fungible_staked_ika_data_total_supply
            * (fungible_staked_ika_value_bps as u128) / 10000;

    let max_principal = get_ika_amount(
        &exchange_rate,
        fungible_staked_ika_data_total_supply as u64,
    );
    let fungible_staked_ika_data_principal_amount = max(
        (max_principal as u128) * (fungible_staked_ika_data_principal_frac as u128) / 10000,
        1,
    );

    let (principal_amount, rewards_amount) = calculate_fungible_staked_ika_withdraw_amount(
        exchange_rate,
        fungible_staked_ika_value as u64,
        fungible_staked_ika_data_principal_amount as u64,
        fungible_staked_ika_data_total_supply as u64,
    );

    let expected_out = get_ika_amount(&exchange_rate, fungible_staked_ika_value as u64);

    assert!(principal_amount + rewards_amount <= expected_out, 0);

    let min_out = if (expected_out > 2) expected_out - 2 else 0;
    assert!(principal_amount + rewards_amount >= min_out, 0);
}
