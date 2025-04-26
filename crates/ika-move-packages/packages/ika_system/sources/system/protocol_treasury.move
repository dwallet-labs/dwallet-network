// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::protocol_treasury;

use ika::ika::IKA;
use sui::bag::{Self, Bag};
use sui::balance::Balance;
use sui::coin::TreasuryCap;

public struct ProtocolTreasury has store {
    /// TreasuryCap of IKA tokens.
    treasury_cap: TreasuryCap<IKA>,
    /// Count of the number of times stake subsidies have been distributed.
    stake_subsidy_distribution_counter: u64,
    /// The rate at which the amount per distribution is calculated based on
    /// period nad total supply. Expressed in basis points.
    stake_subsidy_rate: u16,
    /// The amount of stake subsidy to be distrabtured per distribution.
    /// This amount changes based on `stake_subsidy_rate`.
    stake_subsidy_amount_per_distribution: u64,
    /// Number of distributions to occur before the amount per distribution will be recalculated.
    stake_subsidy_period_length: u64,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

const BASIS_POINT_DENOMINATOR: u128 = 10000;

const ESubsidyDecreaseRateTooLarge: u64 = 0;

public(package) fun create(
    treasury_cap: TreasuryCap<IKA>,
    stake_subsidy_rate: u16,
    stake_subsidy_period_length: u64,
    ctx: &mut TxContext,
): ProtocolTreasury {
    // Rate can't be higher than 100%.
    assert!(stake_subsidy_rate <= BASIS_POINT_DENOMINATOR as u16, ESubsidyDecreaseRateTooLarge);

    let stake_subsidy_amount_per_distribution = calculate_stake_subsidy_amount_per_distribution(
        &treasury_cap,
        stake_subsidy_rate,
        stake_subsidy_period_length,
    );

    ProtocolTreasury {
        treasury_cap,
        stake_subsidy_distribution_counter: 0,
        stake_subsidy_rate,
        stake_subsidy_amount_per_distribution,
        stake_subsidy_period_length,
        extra_fields: bag::new(ctx),
    }
}

/// Advance the distribution counter and return the stake subsidy.
public(package) fun stake_subsidy_for_distribution(
    self: &mut ProtocolTreasury,
    ctx: &mut TxContext,
): Balance<IKA> {
    // Mint the reward amount for this stake subsidy
    let stake_subsidy = self.treasury_cap.mint(self.stake_subsidy_amount_per_distribution, ctx);

    self.stake_subsidy_distribution_counter = self.stake_subsidy_distribution_counter + 1;

    // Recalculate subsidy amount per distribution only when the current period ends.
    if (self.stake_subsidy_distribution_counter % self.stake_subsidy_period_length == 0) {
        self.stake_subsidy_amount_per_distribution =
            calculate_stake_subsidy_amount_per_distribution(
                &self.treasury_cap,
                self.stake_subsidy_rate,
                self.stake_subsidy_period_length,
            );
    };

    stake_subsidy.into_balance()
}

fun calculate_stake_subsidy_amount_per_distribution(
    treasury_cap: &TreasuryCap<IKA>,
    stake_subsidy_rate: u16,
    stake_subsidy_period_length: u64,
): u64 {
    let stake_subsidy_total_period_distribution_amount =
        treasury_cap.total_supply() as u128
                * (stake_subsidy_rate as u128) / BASIS_POINT_DENOMINATOR;
    let stake_subsidy_amount_per_distribution =
        stake_subsidy_total_period_distribution_amount / (stake_subsidy_period_length as u128);
    stake_subsidy_amount_per_distribution as u64
}

// TODO: enable voting to chagnes rate
public(package) fun set_stake_subsidy_rate(self: &mut ProtocolTreasury, stake_subsidy_rate: u16) {
    // When stake subsidy rate decreases
    if (self.stake_subsidy_rate > stake_subsidy_rate) {
        let stake_subsidy_rate_diff = self.stake_subsidy_rate - stake_subsidy_rate;
        let stake_subsidy_diff =
            (self.stake_subsidy_amount_per_distribution as u128) * (stake_subsidy_rate_diff as u128) / BASIS_POINT_DENOMINATOR;
        self.stake_subsidy_amount_per_distribution =
            self.stake_subsidy_amount_per_distribution - (stake_subsidy_diff as u64);
        // When stake subsidy rate increases
    } else if (self.stake_subsidy_rate < stake_subsidy_rate) {
        let stake_subsidy_rate_diff = stake_subsidy_rate - self.stake_subsidy_rate;
        let stake_subsidy_diff =
            (self.stake_subsidy_amount_per_distribution as u128) * (stake_subsidy_rate_diff as u128) / BASIS_POINT_DENOMINATOR;
        self.stake_subsidy_amount_per_distribution =
            self.stake_subsidy_amount_per_distribution + (stake_subsidy_diff as u64);
    }
}

/// Returns the stake subsidy amount per distribution.
public fun stake_subsidy_amount_per_distribution(self: &ProtocolTreasury): u64 {
    self.stake_subsidy_amount_per_distribution
}

/// Returns the number of distributions that have occurred.
public(package) fun get_stake_subsidy_distribution_counter(self: &ProtocolTreasury): u64 {
    self.stake_subsidy_distribution_counter
}

#[test_only]
public(package) fun set_stake_subsidy_distribution_counter(self: &mut ProtocolTreasury, stake_subsidy_distribution_counter: u64) {
    self.stake_subsidy_distribution_counter = stake_subsidy_distribution_counter;
}
