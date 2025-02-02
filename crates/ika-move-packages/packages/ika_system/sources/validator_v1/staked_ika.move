// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::staked_ika;

use ika::ika::IKA;
use sui::balance::{Balance};

/// StakedIka objects cannot be split to below this amount.
const MIN_STAKING_THRESHOLD: u64 = 1_000_000_000; // 1 IKA

const EInsufficientPoolTokenBalance: u64 = 0;
const EWrongPool: u64 = 1;
const EInsufficientIkaTokenBalance: u64 = 3;

const EIncompatibleStakedIka: u64 = 12;

const EStakedIkaBelowThreshold: u64 = 18;


/// A self-custodial object holding the staked IKA tokens.
public struct StakedIka has key, store {
    id: UID,
    /// ID of the validator we are staking with.
    validator_id: ID,
    /// The epoch at which the stake becomes active.
    stake_activation_epoch: u64,
    /// The staked IKA tokens.
    principal: Balance<IKA>,
}

/// An alternative to `StakedIka` that holds the pool token amount instead of the IKA balance.
/// StakedIka objects can be converted to FungibleStakedIkas after the initial warmup period.
/// The advantage of this is that you can now merge multiple StakedIka objects from different
/// activation epochs into a single FungibleStakedIka object.
public struct FungibleStakedIka has key, store {
    id: UID,
    /// ID of the validator we are staking with.
    validator_id: ID,
    /// The pool token amount.
    value: u64,
}

// ==== initializer ====

/// Create a new staked ika.
public(package) fun create(
    validator_id: ID,
    stake_activation_epoch: u64,
    principal: Balance<IKA>,
    ctx: &mut TxContext,
): StakedIka {
    StakedIka {
        id: object::new(ctx),
        validator_id,
        stake_activation_epoch,
        principal
    }
}


public(package) fun into_balance(staked_ika: StakedIka): Balance<IKA> {
    let StakedIka {
        id,
        validator_id: _,
        stake_activation_epoch: _,
        principal,
    } = staked_ika;
    object::delete(id);
    principal
}

/// Create a new fungible staked ika.
public(package) fun create_fungible(
    validator_id: ID,
    value: u64,
    ctx: &mut TxContext,
): FungibleStakedIka {
    FungibleStakedIka {
        id: object::new(ctx),
        validator_id,
        value
    }
}

// ==== getters and misc utility functions ====


public fun validator_id(staked_ika: &StakedIka): ID { staked_ika.validator_id }

public use fun fungible_staked_ika_validator_id as FungibleStakedIka.validator_id;

public fun fungible_staked_ika_validator_id(fungible_staked_ika: &FungibleStakedIka): ID {
    fungible_staked_ika.validator_id
}

public fun staked_ika_amount(staked_ika: &StakedIka): u64 { staked_ika.principal.value() }

/// Allows calling `.amount()` on `StakedIka` to invoke `staked_ika_amount`
public use fun staked_ika_amount as StakedIka.amount;

public fun stake_activation_epoch(staked_ika: &StakedIka): u64 {
    staked_ika.stake_activation_epoch
}


public use fun fungible_staked_ika_value as FungibleStakedIka.value;

public fun fungible_staked_ika_value(fungible_staked_ika: &FungibleStakedIka): u64 {
    fungible_staked_ika.value
}

public use fun split_fungible_staked_ika as FungibleStakedIka.split;

public fun split_fungible_staked_ika(
    fungible_staked_ika: &mut FungibleStakedIka,
    split_amount: u64,
    ctx: &mut TxContext,
): FungibleStakedIka {
    assert!(split_amount <= fungible_staked_ika.value, EInsufficientPoolTokenBalance);

    fungible_staked_ika.value = fungible_staked_ika.value - split_amount;

    FungibleStakedIka {
        id: object::new(ctx),
        validator_id: fungible_staked_ika.validator_id,
        value: split_amount,
    }
}

public use fun join_fungible_staked_ika as FungibleStakedIka.join;

public fun join_fungible_staked_ika(self: &mut FungibleStakedIka, other: FungibleStakedIka) {
    let FungibleStakedIka { id, validator_id, value } = other;
    assert!(self.validator_id == validator_id, EWrongPool);

    object::delete(id);

    self.value = self.value + value;
}

/// Split StakedIka `self` to two parts, one with principal `split_amount`,
/// and the remaining principal is left in `self`.
/// All the other parameters of the StakedIka like `stake_activation_epoch` or `pool_id` remain the same.
public fun split(self: &mut StakedIka, split_amount: u64, ctx: &mut TxContext): StakedIka {
    let original_amount = self.principal.value();
    assert!(split_amount <= original_amount, EInsufficientIkaTokenBalance);
    let remaining_amount = original_amount - split_amount;
    // Both resulting parts should have at least MIN_STAKING_THRESHOLD.
    assert!(remaining_amount >= MIN_STAKING_THRESHOLD, EStakedIkaBelowThreshold);
    assert!(split_amount >= MIN_STAKING_THRESHOLD, EStakedIkaBelowThreshold);
    StakedIka {
        id: object::new(ctx),
        validator_id: self.validator_id,
        stake_activation_epoch: self.stake_activation_epoch,
        principal: self.principal.split(split_amount),
    }
}

/// Split the given StakedIka to the two parts, one with principal `split_amount`,
/// transfer the newly split part to the sender address.
public entry fun split_staked_ika(stake: &mut StakedIka, split_amount: u64, ctx: &mut TxContext) {
    transfer::transfer(split(stake, split_amount, ctx), ctx.sender());
}

/// Allows calling `.split_to_sender()` on `StakedIka` to invoke `split_staked_ika`
public use fun split_staked_ika as StakedIka.split_to_sender;

/// Consume the staked ika `other` and add its value to `self`.
/// Aborts if some of the staking parameters are incompatible (pool id, stake activation epoch, etc.)
public entry fun join_staked_ika(self: &mut StakedIka, other: StakedIka) {
    assert!(is_equal_staking_metadata(self, &other), EIncompatibleStakedIka);
    let StakedIka {
        id,
        validator_id: _,
        stake_activation_epoch: _,
        principal,
    } = other;

    id.delete();
    self.principal.join(principal);
}

/// Allows calling `.join()` on `StakedIka` to invoke `join_staked_ika`
public use fun join_staked_ika as StakedIka.join;

/// Returns true if all the staking parameters of the staked ika except the principal are identical
public fun is_equal_staking_metadata(self: &StakedIka, other: &StakedIka): bool {
    (self.validator_id == other.validator_id) &&
        (self.stake_activation_epoch == other.stake_activation_epoch)
}

public(package) fun destroy(
    fungible_staked_ika: FungibleStakedIka
) {
    let FungibleStakedIka { id, .. } = fungible_staked_ika;
    id.delete();
}
