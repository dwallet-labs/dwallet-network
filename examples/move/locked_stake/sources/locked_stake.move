// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module locked_stake::locked_stake {
    use ika::coin;
    use ika::balance::{Self, Balance};
    use ika::vec_map::{Self, VecMap};
    use ika::ika::IKA;
    use ika_system::staking_pool::StakedIka;
    use ika_system::ika_system::{Self, IkaSystemState};
    use locked_stake::epoch_time_lock::{Self, EpochTimeLock};

    const EInsufficientBalance: u64 = 0;
    const EStakeObjectNonExistent: u64 = 1;

    /// An object that locks IKA tokens and stake objects until a given epoch, and allows
    /// staking and unstaking operations when locked.
    public struct LockedStake has key {
        id: UID,
        staked_ika: VecMap<ID, StakedIka>,
        ika: Balance<IKA>,
        locked_until_epoch: EpochTimeLock,
    }

    // ============================= basic operations =============================

    /// Create a new LockedStake object with empty staked_ika and ika balance given a lock time.
    /// Aborts if the given epoch has already passed.
    public fun new(locked_until_epoch: u64, ctx: &mut TxContext): LockedStake {
        LockedStake {
            id: object::new(ctx),
            staked_ika: vec_map::empty(),
            ika: balance::zero(),
            locked_until_epoch: epoch_time_lock::new(locked_until_epoch, ctx),
        }
    }

    /// Unlocks and returns all the assets stored inside this LockedStake object.
    /// Aborts if the unlock epoch is in the future.
    public fun unlock(ls: LockedStake, ctx: &TxContext): (VecMap<ID, StakedIka>, Balance<IKA>) {
        let LockedStake { id, staked_ika, ika, locked_until_epoch } = ls;
        epoch_time_lock::destroy(locked_until_epoch, ctx);
        object::delete(id);
        (staked_ika, ika)
    }

    /// Deposit a new stake object to the LockedStake object.
    public fun deposit_staked_ika(ls: &mut LockedStake, staked_ika: StakedIka) {
        let id = object::id(&staked_ika);
        // This insertion can't abort since each object has a unique id.
        vec_map::insert(&mut ls.staked_ika, id, staked_ika);
    }

    /// Deposit ika balance to the LockedStake object.
    public fun deposit_ika(ls: &mut LockedStake, ika: Balance<IKA>) {
        balance::join(&mut ls.ika, ika);
    }

    /// Take `amount` of IKA from the ika balance, stakes it, and puts the stake object
    /// back into the staked ika vec map.
    public fun stake(
        ls: &mut LockedStake,
        ika_system: &mut IkaSystemState,
        amount: u64,
        validator_address: address,
        ctx: &mut TxContext
    ) {
        assert!(balance::value(&ls.ika) >= amount, EInsufficientBalance);
        let stake = ika_system::request_add_stake_non_entry(
            ika_system,
            coin::from_balance(balance::split(&mut ls.ika, amount), ctx),
            validator_address,
            ctx
        );
        deposit_staked_ika(ls, stake);
    }

    /// Unstake the stake object with `staked_ika_id` and puts the resulting principal
    /// and rewards back into the locked ika balance.
    /// Returns the amount of IKA unstaked, including both principal and rewards.
    /// Aborts if no stake exists with the given id.
    public fun unstake(
        ls: &mut LockedStake,
        ika_system: &mut IkaSystemState,
        staked_ika_id: ID,
        ctx: &mut TxContext
    ): u64 {
        assert!(vec_map::contains(&ls.staked_ika, &staked_ika_id), EStakeObjectNonExistent);
        let (_, stake) = vec_map::remove(&mut ls.staked_ika, &staked_ika_id);
        let ika_balance = ika_system::request_withdraw_stake_non_entry(ika_system, stake, ctx);
        let amount = balance::value(&ika_balance);
        deposit_ika(ls, ika_balance);
        amount
    }

    // ============================= getters =============================

    public fun staked_ika(ls: &LockedStake): &VecMap<ID, StakedIka> {
        &ls.staked_ika
    }

    public fun ika_balance(ls: &LockedStake): u64 {
        balance::value(&ls.ika)
    }

    public fun locked_until_epoch(ls: &LockedStake): u64 {
        epoch_time_lock::epoch(&ls.locked_until_epoch)
    }

    // TODO: possibly add some scenarios like switching stake, creating a new LockedStake and transferring
    // it to the sender, etc. But these can also be done as PTBs.
}
