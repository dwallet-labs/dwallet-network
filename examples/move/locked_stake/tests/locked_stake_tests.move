// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module locked_stake::locked_stake_tests {

    use ika_system::governance_test_utils::{advance_epoch, set_up_ika_system_state};
    use ika_system::ika_system::{Self, IkaSystemState};
    use ika::coin;
    use ika::test_scenario;
    use ika::test_utils::{assert_eq, destroy};
    use ika::vec_map;
    use ika::balance;
    use locked_stake::locked_stake as ls;
    use locked_stake::epoch_time_lock;

    const NIKA_PER_IKA: u64 = 1_000_000_000;

    #[test]
    #[expected_failure(abort_code = epoch_time_lock::EEpochAlreadyPassed)]
    fun test_incorrect_creation() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;

        set_up_ika_system_state(vector[@0x1, @0x2, @0x3]);

        // Advance epoch twice so we are now at epoch 2.
        advance_epoch(scenario);
        advance_epoch(scenario);
        let ctx = test_scenario::ctx(scenario);
        assert_eq(tx_context::epoch(ctx), 2);

        // Create a locked stake with epoch 1. Should fail here.
        let ls = ls::new(1, ctx);

        destroy(ls);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_deposit_stake_unstake() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;

        set_up_ika_system_state(vector[@0x1, @0x2, @0x3]);

        let mut ls = ls::new(10, test_scenario::ctx(scenario));

        // Deposit 100 IKA.
        ls::deposit_ika(&mut ls, balance::create_for_testing(100 * NIKA_PER_IKA));

        assert_eq(ls::ika_balance(&ls), 100 * NIKA_PER_IKA);

        test_scenario::next_tx(scenario, @0x1);
        let mut system_state = test_scenario::take_shared<IkaSystemState>(scenario);

        // Stake 10 of the 100 IKA.
        ls::stake(&mut ls, &mut system_state, 10 * NIKA_PER_IKA, @0x1, test_scenario::ctx(scenario));
        test_scenario::return_shared(system_state);

        assert_eq(ls::ika_balance(&ls), 90 * NIKA_PER_IKA);
        assert_eq(vec_map::size(ls::staked_ika(&ls)), 1);

        test_scenario::next_tx(scenario, @0x1);
        let mut system_state = test_scenario::take_shared<IkaSystemState>(scenario);
        let ctx = test_scenario::ctx(scenario);

        // Create a StakedIka object and add it to the LockedStake object.
        let staked_ika = ika_system::request_add_stake_non_entry(
            &mut system_state, coin::mint_for_testing(20 * NIKA_PER_IKA, ctx), @0x2, ctx);
        test_scenario::return_shared(system_state);

        ls::deposit_staked_ika(&mut ls, staked_ika);
        assert_eq(ls::ika_balance(&ls), 90 * NIKA_PER_IKA);
        assert_eq(vec_map::size(ls::staked_ika(&ls)), 2);
        advance_epoch(scenario);

        test_scenario::next_tx(scenario, @0x1);
        let (staked_ika_id, _) = vec_map::get_entry_by_idx(ls::staked_ika(&ls), 0);
        let mut system_state = test_scenario::take_shared<IkaSystemState>(scenario);

        // Unstake both stake objects
        ls::unstake(&mut ls, &mut system_state, *staked_ika_id, test_scenario::ctx(scenario));
        test_scenario::return_shared(system_state);
        assert_eq(ls::ika_balance(&ls), 100 * NIKA_PER_IKA);
        assert_eq(vec_map::size(ls::staked_ika(&ls)), 1);

        test_scenario::next_tx(scenario, @0x1);
        let (staked_ika_id, _) = vec_map::get_entry_by_idx(ls::staked_ika(&ls), 0);
        let mut system_state = test_scenario::take_shared<IkaSystemState>(scenario);
        ls::unstake(&mut ls, &mut system_state, *staked_ika_id, test_scenario::ctx(scenario));
        test_scenario::return_shared(system_state);
        assert_eq(ls::ika_balance(&ls), 120 * NIKA_PER_IKA);
        assert_eq(vec_map::size(ls::staked_ika(&ls)), 0);

        destroy(ls);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_unlock_correct_epoch() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;

        set_up_ika_system_state(vector[@0x1, @0x2, @0x3]);

        let mut ls = ls::new(2, test_scenario::ctx(scenario));

        ls::deposit_ika(&mut ls, balance::create_for_testing(100 * NIKA_PER_IKA));

        assert_eq(ls::ika_balance(&ls), 100 * NIKA_PER_IKA);

        test_scenario::next_tx(scenario, @0x1);
        let mut system_state = test_scenario::take_shared<IkaSystemState>(scenario);
        ls::stake(&mut ls, &mut system_state, 10 * NIKA_PER_IKA, @0x1, test_scenario::ctx(scenario));
        test_scenario::return_shared(system_state);

        advance_epoch(scenario);
        advance_epoch(scenario);
        advance_epoch(scenario);
        advance_epoch(scenario);

        let (staked_ika, ika_balance) = ls::unlock(ls, test_scenario::ctx(scenario));
        assert_eq(balance::value(&ika_balance), 90 * NIKA_PER_IKA);
        assert_eq(vec_map::size(&staked_ika), 1);

        destroy(staked_ika);
        destroy(ika_balance);
        test_scenario::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = epoch_time_lock::EEpochNotYetEnded)]
    fun test_unlock_incorrect_epoch() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;

        set_up_ika_system_state(vector[@0x1, @0x2, @0x3]);

        let ls = ls::new(2, test_scenario::ctx(scenario));
        let (staked_ika, ika_balance) = ls::unlock(ls, test_scenario::ctx(scenario));
        destroy(staked_ika);
        destroy(ika_balance);
        test_scenario::end(scenario_val);
    }
}
