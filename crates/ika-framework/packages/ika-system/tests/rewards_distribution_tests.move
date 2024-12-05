// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module ika_system::rewards_distribution_tests {
    use ika::balance;
    use ika::test_scenario::{Self, Scenario};
    use ika_system::ika_system::IkaSystemState;
    use ika_system::validator_cap::UnverifiedValidatorOperationCap;
    use ika_system::governance_test_utils::{
        advance_epoch,
        advance_epoch_with_reward_amounts,
        advance_epoch_with_reward_amounts_and_slashing_rates,
        assert_validator_total_stake_amounts,
        assert_validator_non_self_stake_amounts,
        assert_validator_self_stake_amounts,
        create_validator_for_testing,
        create_ika_system_state_for_testing,
        stake_with,
        total_ika_balance, unstake
    };
    use ika::test_utils::assert_eq;
    use ika::address;

    const VALIDATOR_ADDR_1: address = @0x1;
    const VALIDATOR_ADDR_2: address = @0x2;
    const VALIDATOR_ADDR_3: address = @0x3;
    const VALIDATOR_ADDR_4: address = @0x4;

    const STAKER_ADDR_1: address = @0x42;
    const STAKER_ADDR_2: address = @0x43;
    const STAKER_ADDR_3: address = @0x44;
    const STAKER_ADDR_4: address = @0x45;

    const NIKA_PER_IKA: u64 = 1_000_000_000;

    #[test]
    fun test_validator_rewards() {
        set_up_ika_system_state();
        let mut scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        // need to advance epoch so validator's staking starts counting
        advance_epoch(scenario);

        advance_epoch_with_reward_amounts(0, 100, scenario);
        assert_validator_total_stake_amounts(
            validator_addrs(),
            vector[125 * NIKA_PER_IKA, 225 * NIKA_PER_IKA, 325 * NIKA_PER_IKA, 425 * NIKA_PER_IKA],
            scenario
        );

        stake_with(VALIDATOR_ADDR_2, VALIDATOR_ADDR_2, 720, scenario);

        advance_epoch(scenario);
        advance_epoch_with_reward_amounts(0, 100, scenario);
        // Even though validator 2 has a lot more stake now, it should not get more rewards because
        // the voting power is capped at 10%.
        assert_validator_total_stake_amounts(
            validator_addrs(),
            vector[150 * NIKA_PER_IKA, 970 * NIKA_PER_IKA, 350 * NIKA_PER_IKA, 450 * NIKA_PER_IKA],
            scenario
        );
        scenario_val.end();
    }

    #[test]
    fun test_stake_subsidy() {
        set_up_ika_system_state_with_big_amounts();
        let mut scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        // need to advance epoch so validator's staking starts counting
        advance_epoch(scenario);

        advance_epoch_with_reward_amounts(0, 100, scenario);
        assert_validator_total_stake_amounts(validator_addrs(), vector[100_000_025 * NIKA_PER_IKA, 200_000_025 * NIKA_PER_IKA, 300_000_025 * NIKA_PER_IKA, 400_000_025 * NIKA_PER_IKA], scenario);
        scenario_val.end();
    }

    #[test]
    fun test_stake_rewards() {
        set_up_ika_system_state();
        let mut scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 200, scenario);
        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_2, 100, scenario);
        advance_epoch(scenario);

        assert_validator_total_stake_amounts(validator_addrs(), vector[300 * NIKA_PER_IKA, 300 * NIKA_PER_IKA, 300 * NIKA_PER_IKA, 400 * NIKA_PER_IKA], scenario);
        assert_validator_self_stake_amounts(validator_addrs(), vector[100 * NIKA_PER_IKA, 200 * NIKA_PER_IKA, 300 * NIKA_PER_IKA, 400 * NIKA_PER_IKA], scenario);

        // Each pool gets 30 IKA.
        advance_epoch_with_reward_amounts(0, 120, scenario);
        assert_validator_self_stake_amounts(validator_addrs(), vector[110 * NIKA_PER_IKA, 220 * NIKA_PER_IKA, 330 * NIKA_PER_IKA, 430 * NIKA_PER_IKA], scenario);
        unstake(STAKER_ADDR_1, 0, scenario);
        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_1, 600, scenario);
        // Each pool gets 30 IKA.
        advance_epoch_with_reward_amounts(0, 120, scenario);
        // staker 1 receives only 20 IKA of rewards, not 40 since we are using pre-epoch exchange rate.
        assert_eq(total_ika_balance(STAKER_ADDR_1, scenario), 220 * NIKA_PER_IKA);
        assert_validator_self_stake_amounts(validator_addrs(), vector[140 * NIKA_PER_IKA, 240 * NIKA_PER_IKA, 360 * NIKA_PER_IKA, 460 * NIKA_PER_IKA], scenario);
        unstake(STAKER_ADDR_2, 0, scenario);
        assert_eq(total_ika_balance(STAKER_ADDR_2, scenario), 120 * NIKA_PER_IKA); // 20 IKA of rewards received

        advance_epoch_with_reward_amounts(0, 40, scenario);

        unstake(STAKER_ADDR_2, 0, scenario); // unstake 600 principal IKA
        // additional 600 IKA of principal and 46 IKA of rewards withdrawn to Coin<IKA>
        // For this stake, the staking exchange rate is 100 : 140 and the unstaking
        // exchange rate is 528 : 750 -ish so the total ika withdraw will be:
        // (600 * 100 / 140) * 750 / 528 = ~608. Together with the 120 IKA we already have,
        // that would be about 728 IKA.
        // TODO: Come up with better numbers and clean it up!
        assert_eq(total_ika_balance(STAKER_ADDR_2, scenario), 728108108107);
        scenario_val.end();
    }

    #[test]
    fun test_stake_tiny_rewards() {
        set_up_ika_system_state_with_big_amounts();
        let mut scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        // stake a large amount
        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 200000000, scenario);

        advance_epoch(scenario);

        advance_epoch_with_reward_amounts(0, 150000, scenario);

        // stake a small amount
        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 10, scenario);
        advance_epoch_with_reward_amounts(0, 130, scenario);

        // unstake the stakes
        unstake(STAKER_ADDR_1, 1, scenario);

        // and advance epoch should succeed
        advance_epoch_with_reward_amounts(0, 150, scenario);
        scenario_val.end();
    }

    #[test]
    fun test_validator_commission() {
        set_up_ika_system_state();
        let mut scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 100, scenario);
        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_2, 100, scenario);
        advance_epoch(scenario);
        // V1: 200, V2: 300, V3: 300, V4: 400

        set_commission_rate_and_advance_epoch(VALIDATOR_ADDR_2, 2000, scenario); // 50% commission
        advance_epoch_with_reward_amounts(0, 120, scenario);
        // V1: 230, V2: 330, V3: 330, V4: 430
        // 2 IKA, or 20 % of staker_2's rewards, goes to validator_2
        assert_validator_non_self_stake_amounts(validator_addrs(), vector[115 * NIKA_PER_IKA, 108 * NIKA_PER_IKA, 0, 0], scenario);
        assert_validator_self_stake_amounts(validator_addrs(), vector[115 * NIKA_PER_IKA, 222 * NIKA_PER_IKA, 330 * NIKA_PER_IKA, 430 * NIKA_PER_IKA], scenario);

        set_commission_rate_and_advance_epoch(VALIDATOR_ADDR_1, 1000, scenario); // 10% commission

        advance_epoch_with_reward_amounts(0, 240, scenario);
        assert_validator_total_stake_amounts(validator_addrs(), vector[290 * NIKA_PER_IKA, 390 * NIKA_PER_IKA, 390 * NIKA_PER_IKA, 490 * NIKA_PER_IKA], scenario);

        // Staker 1 rewards in the recent distribution is 0.9 x 30 = 27 IKA
        // Validator 1 rewards in the recent distribution is 60 - 27 = 33 IKA

        // Staker 2 amounts for 0.8 * 60 * (108 / 330) + 108 = 123.709 IKA
        // Validator 2 amounts for 390 - 123.709 = 266.291 IKA
        assert_validator_non_self_stake_amounts(validator_addrs(), vector[142 * NIKA_PER_IKA, 123709090909, 0, 0], scenario);
        assert_validator_self_stake_amounts(validator_addrs(), vector[148 * NIKA_PER_IKA, 266290909091, 390 * NIKA_PER_IKA, 490 * NIKA_PER_IKA], scenario);

        scenario_val.end();
    }

    #[test]
    fun test_rewards_slashing() {
        set_up_ika_system_state();
        let mut scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        advance_epoch(scenario);

        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 100, scenario);
        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_2, 100, scenario);

        advance_epoch(scenario);

        // validator_2 is reported by 3 other validators, so 75% of total stake.
        report_validator(VALIDATOR_ADDR_1, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_4, VALIDATOR_ADDR_2, scenario);

        // validator_1 is reported by only 1 other validator, which is 25% of total stake.
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_1, scenario);

        // 3600 IKA of total rewards, 50% threshold and 10% reward slashing.
        // So validator_2 is the only one whose rewards should get slashed.
        advance_epoch_with_reward_amounts_and_slashing_rates(
            0, 3600, 1000, scenario
        );

        // Without reward slashing, the validator's stakes should be [100+450, 200+600, 300+900, 400+900]
        // after the last epoch advancement.
        // Since 60 IKA, or 10% of validator_2's rewards (600) are slashed, she only has 800 - 60 = 740 now.
        // There are in total 90 IKA of rewards slashed (60 from the validator, and 30 from her staker)
        // so the unslashed validators each get their share of additional rewards, which is 30.
        assert_validator_self_stake_amounts(validator_addrs(), vector[565 * NIKA_PER_IKA, 740 * NIKA_PER_IKA, 1230 * NIKA_PER_IKA, 1330 * NIKA_PER_IKA], scenario);

        // Unstake so we can check the stake rewards as well.
        unstake(STAKER_ADDR_1, 0, scenario);
        unstake(STAKER_ADDR_2, 0, scenario);

        // Same analysis as above. Delegator 1 has 3 additional IKA, and 10% of staker 2's rewards are slashed.
        assert!(total_ika_balance(STAKER_ADDR_1, scenario) == 565 * NIKA_PER_IKA);
        assert!(total_ika_balance(STAKER_ADDR_2, scenario) == 370 * NIKA_PER_IKA);
        scenario_val.end();
    }

    #[test]
    fun test_entire_rewards_slashing() {
        set_up_ika_system_state();
        let mut scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        advance_epoch(scenario);

        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 100, scenario);
        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_2, 100, scenario);

        advance_epoch(scenario);

        // validator_2 is reported by 3 other validators, so 75% of total stake.
        report_validator(VALIDATOR_ADDR_1, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_4, VALIDATOR_ADDR_2, scenario);


        // 3600 IKA of total rewards, 100% reward slashing.
        // So validator_2 is the only one whose rewards should get slashed.
        advance_epoch_with_reward_amounts_and_slashing_rates(
            0, 3600, 10_000, scenario
        );

        // Without reward slashing, the validator's stakes should be [100+450, 200+600, 300+900, 400+900]
        // after the last epoch advancement.
        // The entire rewards of validator 2's staking pool are slashed, which is 900 IKA.
        // so the unslashed validators each get their share of additional rewards, which is 300.
        assert_validator_self_stake_amounts(validator_addrs(), vector[(550 + 150) * NIKA_PER_IKA, 200 * NIKA_PER_IKA, (1200 + 300) * NIKA_PER_IKA, (1300 + 300) * NIKA_PER_IKA], scenario);

        // Unstake so we can check the stake rewards as well.
        unstake(STAKER_ADDR_1, 0, scenario);
        unstake(STAKER_ADDR_2, 0, scenario);

        // Same analysis as above. Staker 1 has 150 additional IKA, and since all of staker 2's rewards are slashed she only gets back her principal.
        assert!(total_ika_balance(STAKER_ADDR_1, scenario) == (550 + 150) * NIKA_PER_IKA);
        assert!(total_ika_balance(STAKER_ADDR_2, scenario) == 100 * NIKA_PER_IKA);
        scenario_val.end();
    }

    #[test]
    fun test_rewards_slashing_with_storage_fund() {
        set_up_ika_system_state();
        let mut scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        // Put 300 IKA into the storage fund.
        advance_epoch_with_reward_amounts(300, 0, scenario);

        // Add a few stakes.
        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_3, 100, scenario);
        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_4, 100, scenario);
        advance_epoch(scenario);

        // validator_4 is reported by 3 other validators, so 75% of total stake.
        report_validator(VALIDATOR_ADDR_1, VALIDATOR_ADDR_4, scenario);
        report_validator(VALIDATOR_ADDR_2, VALIDATOR_ADDR_4, scenario);
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_4, scenario);

        // 1000 IKA of storage rewards, 1500 IKA of computation rewards, 50% slashing threshold
        // and 20% slashing rate
        advance_epoch_with_reward_amounts_and_slashing_rates(
            1000, 1500, 2000, scenario
        );

        // Each unslashed validator staking pool gets 300 IKA of computation rewards + 75 IKA of storage fund rewards +
        // 20 IKA (1/3) of validator 4's slashed computation reward and 5 IKA (1/3) of validator 4's slashed
        // storage fund reward, so in total it gets 400 IKA of rewards.
        // Validator 3 has a delegator with her so she gets 320 * 3/4 + 75 + 5 = 320 IKA of rewards.
        // Validator 4's should get 300 * 4/5 * (1 - 20%) = 192 in computation rewards and 75 * (1 - 20%) = 60 in storage rewards.
        assert_validator_self_stake_amounts(validator_addrs(), vector[500 * NIKA_PER_IKA, 600 * NIKA_PER_IKA, 620 * NIKA_PER_IKA, 652 * NIKA_PER_IKA], scenario);

        // Unstake so we can check the stake rewards as well.
        unstake(STAKER_ADDR_1, 0, scenario);
        unstake(STAKER_ADDR_2, 0, scenario);

        // Staker 1 gets 320 * 1/4 = 80 IKA of rewards.
        assert_eq(total_ika_balance(STAKER_ADDR_1, scenario), (100 + 80) * NIKA_PER_IKA);
        // Staker 2 gets 300 * 1/5 * (1 - 20%) = 48 IKA of rewards.
        assert_eq(total_ika_balance(STAKER_ADDR_2, scenario), (100 + 48) * NIKA_PER_IKA);

        scenario_val.end();
    }

    #[test]
    fun test_everyone_slashed() {
        // This test is to make sure that if everyone is slashed, our protocol works as expected without aborting
        // and all rewards go to the storage fund.
        set_up_ika_system_state();
        let mut scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        report_validator(VALIDATOR_ADDR_1, VALIDATOR_ADDR_4, scenario);
        report_validator(VALIDATOR_ADDR_2, VALIDATOR_ADDR_4, scenario);
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_4, scenario);
        report_validator(VALIDATOR_ADDR_1, VALIDATOR_ADDR_3, scenario);
        report_validator(VALIDATOR_ADDR_2, VALIDATOR_ADDR_3, scenario);
        report_validator(VALIDATOR_ADDR_4, VALIDATOR_ADDR_3, scenario);
        report_validator(VALIDATOR_ADDR_1, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_4, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_2, VALIDATOR_ADDR_1, scenario);
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_1, scenario);
        report_validator(VALIDATOR_ADDR_4, VALIDATOR_ADDR_1, scenario);

        advance_epoch_with_reward_amounts_and_slashing_rates(
            1000, 3000, 10_000, scenario
        );

        // All validators should have 0 rewards added so their stake stays the same.
        assert_validator_self_stake_amounts(validator_addrs(), vector[100 * NIKA_PER_IKA, 200 * NIKA_PER_IKA, 300 * NIKA_PER_IKA, 400 * NIKA_PER_IKA], scenario);

        scenario.next_tx(@0x0);
        // Storage fund balance should increase by 4000 IKA.
        let mut system_state = scenario.take_shared<IkaSystemState>();
        assert_eq(system_state.get_storage_fund_total_balance(), 4000 * NIKA_PER_IKA);

        // The entire 1000 IKA of storage rewards should go to the object rebate portion of the storage fund.
        assert_eq(system_state.get_storage_fund_object_rebates(), 1000 * NIKA_PER_IKA);

        test_scenario::return_shared(system_state);
        scenario_val.end();
    }

    #[test]
    fun test_mul_rewards_withdraws_at_same_epoch() {
        set_up_ika_system_state();
        let mut scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 220, scenario);

        advance_epoch_with_reward_amounts(0, 40, scenario);

        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_1, 480, scenario);

        // Staker 1 gets 2/3 * 1/4 * 120 = 20 IKA here.
        advance_epoch_with_reward_amounts(0, 120, scenario);

        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 130, scenario);
        stake_with(STAKER_ADDR_3, VALIDATOR_ADDR_1, 390, scenario);

        // Staker 1 gets 20 IKA here and staker 2 gets 40 IKA here.
        advance_epoch_with_reward_amounts(0, 280, scenario);
        stake_with(STAKER_ADDR_3, VALIDATOR_ADDR_1, 280, scenario);
        stake_with(STAKER_ADDR_4, VALIDATOR_ADDR_1, 1400, scenario);

        // Staker 1 gets 30 IKA, staker 2 gets 40 IKA and staker 3 gets 30 IKA.
        advance_epoch_with_reward_amounts(0, 440, scenario);

        scenario.next_tx(@0x0);
        let mut system_state = scenario.take_shared<IkaSystemState>();
        // Check that we have the right amount of IKA in the staking pool.
        assert_eq(system_state.validator_stake_amount(VALIDATOR_ADDR_1), 140 * 23 * NIKA_PER_IKA);
        test_scenario::return_shared(system_state);

        // Withdraw all stakes at once.
        unstake(STAKER_ADDR_1, 0, scenario);
        unstake(STAKER_ADDR_1, 0, scenario);
        unstake(STAKER_ADDR_2, 0, scenario);
        unstake(STAKER_ADDR_3, 0, scenario);
        unstake(STAKER_ADDR_3, 0, scenario);
        unstake(STAKER_ADDR_4, 0, scenario);

        // staker 1's first stake was active for 3 epochs so got 20 * 3 = 60 IKA of rewards
        // and her second stake was active for only one epoch and got 10 IKA of rewards.
        assert_eq(total_ika_balance(STAKER_ADDR_1, scenario), (220 + 130 + 20 * 3 + 10) * NIKA_PER_IKA);
        // staker 2's stake was active for 2 epochs so got 40 * 2 = 80 IKA of rewards
        assert_eq(total_ika_balance(STAKER_ADDR_2, scenario), (480 + 40 * 2) * NIKA_PER_IKA);
        // staker 3's first stake was active for 1 epoch and got 30 IKA of rewards
        // and her second stake didn't get any rewards.
        assert_eq(total_ika_balance(STAKER_ADDR_3, scenario), (390 + 280 + 30) * NIKA_PER_IKA);
        // staker 4 joined and left in an epoch where no rewards were earned so she got no rewards.
        assert_eq(total_ika_balance(STAKER_ADDR_4, scenario), 1400 * NIKA_PER_IKA);

        advance_epoch_with_reward_amounts(0, 0, scenario);

        scenario.next_tx(@0x0);
        let mut system_state = scenario.take_shared<IkaSystemState>();
        // Since all the stakes are gone the pool is empty except for the validator's original stake.
        assert_eq(system_state.validator_stake_amount(VALIDATOR_ADDR_1), 140 * NIKA_PER_IKA);
        test_scenario::return_shared(system_state);
        scenario_val.end();
    }

    #[test]
    fun test_uncapped_rewards() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;

        let ctx = scenario.ctx();
        let mut validators = vector[];

        let num_validators = 20;
        let mut i = 0;
        // Create a set of 20 validators, each with 481 + i * 2 IKA of stake.
        // The stake total sums up to be 481 + 483 + ... + 517 + 519 = 1000 IKA.
        while (i < num_validators) {
            let validator = create_validator_for_testing(address::from_u256(i as u256), (481 + i * 2), ctx);
            validators.push_back(validator);
            i = i + 1;
        };

        create_ika_system_state_for_testing(validators, 0, 0, ctx);
        // Each validator's stake gets doubled.
        advance_epoch_with_reward_amounts(0, 10000, scenario);

        let mut i = 0;
        scenario.next_tx(@0x0);
        // Check that each validator has the correct amount of IKA in their stake pool.
        let mut system_state = scenario.take_shared<IkaSystemState>();
        while (i < num_validators) {
            let addr = address::from_u256(i as u256);
            assert_eq(system_state.validator_stake_amount(addr), (962 + i * 4) * NIKA_PER_IKA);
            i = i + 1;
        };
        test_scenario::return_shared(system_state);
        scenario_val.end();
    }

    fun set_up_ika_system_state() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = scenario.ctx();

        let validators = vector[
            create_validator_for_testing(VALIDATOR_ADDR_1, 100, ctx),
            create_validator_for_testing(VALIDATOR_ADDR_2, 200, ctx),
            create_validator_for_testing(VALIDATOR_ADDR_3, 300, ctx),
            create_validator_for_testing(VALIDATOR_ADDR_4, 400, ctx),
        ];
        create_ika_system_state_for_testing(validators, 1000, 0, ctx);
        scenario_val.end();
    }

    fun set_up_ika_system_state_with_big_amounts() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = scenario.ctx();

        let validators = vector[
            create_validator_for_testing(VALIDATOR_ADDR_1, 100000000, ctx),
            create_validator_for_testing(VALIDATOR_ADDR_2, 200000000, ctx),
            create_validator_for_testing(VALIDATOR_ADDR_3, 300000000, ctx),
            create_validator_for_testing(VALIDATOR_ADDR_4, 400000000, ctx),
        ];
        create_ika_system_state_for_testing(validators, 1000000000, 0, ctx);
        scenario_val.end();
    }

    fun validator_addrs() : vector<address> {
        vector[VALIDATOR_ADDR_1, VALIDATOR_ADDR_2, VALIDATOR_ADDR_3, VALIDATOR_ADDR_4]
    }

    fun set_commission_rate_and_advance_epoch(addr: address, commission_rate: u64, scenario: &mut Scenario) {
        scenario.next_tx(addr);
        let mut system_state = scenario.take_shared<IkaSystemState>();
        let ctx = scenario.ctx();
        system_state.request_set_commission_rate(commission_rate, ctx);
        test_scenario::return_shared(system_state);
        advance_epoch(scenario);
    }

    fun report_validator(reporter: address, reportee: address, scenario: &mut Scenario) {
        scenario.next_tx(reporter);
        let mut system_state = scenario.take_shared<IkaSystemState>();
        let cap = scenario.take_from_sender<UnverifiedValidatorOperationCap>();
        system_state.report_validator(&cap, reportee);
        scenario.return_to_sender(cap);
        test_scenario::return_shared(system_state);
    }

    fun check_distribution_counter_invariant(system: &mut IkaSystemState, ctx: &TxContext) {
        assert!(ctx.epoch() == system.epoch());
        // first subsidy distribution was at epoch 20, so counter should always be ahead by 20
        assert_eq(system.get_stake_subsidy_distribution_counter() + 20, ctx.epoch());
    }

    #[test]
    fun test_stake_subsidy_with_safe_mode_epoch_562_to_563() {
        set_up_ika_system_state_with_big_amounts();

        let mut test = test_scenario::begin(VALIDATOR_ADDR_1);
        let mut ika_system = test.take_shared<IkaSystemState>();
        let ctx = test.ctx();
        // mimic state during epoch 562, if we're in safe mode since the 560 -> 561 epoch change
        let start_epoch: u64 = 562;
        let start_distribution_counter = 540;
        let epoch_start_time = 100000000000;
        let epoch_duration = ika_system.inner_mut_for_testing().epoch_duration_ms();

        // increment epoch number (safe mode emulation)
        start_epoch.do!(|_| ctx.increment_epoch_number());
        ika_system.set_epoch_for_testing(start_epoch);
        ika_system.set_stake_subsidy_distribution_counter(start_distribution_counter);

        assert!(ctx.epoch() == start_epoch);
        assert!(ctx.epoch() == ika_system.epoch());
        assert!(ika_system.get_stake_subsidy_distribution_counter() == start_distribution_counter);

        // perform advance epoch
        ika_system
            .inner_mut_for_testing()
            .advance_epoch(start_epoch + 1, 65, balance::zero(), balance::zero(), 0, 0, 0, 0, epoch_start_time, ctx)
            .destroy_for_testing(); // balance returned from `advance_epoch`
        ctx.increment_epoch_number();

        // should distribute 3 epochs worth of subsidies: 560, 561, 562
        assert_eq(ika_system.get_stake_subsidy_distribution_counter(), start_distribution_counter + 3);
        check_distribution_counter_invariant(&mut ika_system, ctx);

        // ensure that next epoch change only distributes one epoch's worth
        ika_system
            .inner_mut_for_testing()
            .advance_epoch(start_epoch + 2, 65, balance::zero(), balance::zero(), 0, 0, 0, 0, epoch_start_time + epoch_duration, ctx)
            .destroy_for_testing(); // balance returned from `advance_epoch`
        ctx.increment_epoch_number();

        // should distribute 1 epoch's worth of subsidies: 563 only
        assert_eq(ika_system.get_stake_subsidy_distribution_counter(), start_distribution_counter + 4);
        check_distribution_counter_invariant(&mut ika_system, ctx);

        test_scenario::return_shared(ika_system);
        test.end();
    }

    #[test]
    fun test_stake_subsidy_with_safe_mode_epoch_563_to_564() {
        set_up_ika_system_state_with_big_amounts();

        let mut test = test_scenario::begin(VALIDATOR_ADDR_1);
        let mut ika_system = test.take_shared<IkaSystemState>();
        let ctx = test.ctx();
        // mimic state during epoch 563, if we're in safe mode since the 560 -> 561 epoch change
        let start_epoch: u64 = 563;
        let start_distribution_counter = 540;
        let epoch_start_time = 100000000000;
        let epoch_duration = ika_system.inner_mut_for_testing().epoch_duration_ms();

        // increment epoch number (safe mode emulation)
        start_epoch.do!(|_| ctx.increment_epoch_number());
        ika_system.set_epoch_for_testing(start_epoch);
        ika_system.set_stake_subsidy_distribution_counter(start_distribution_counter);

        assert!(ctx.epoch() == start_epoch);
        assert!(ctx.epoch() == ika_system.epoch());
        assert!(ika_system.get_stake_subsidy_distribution_counter() == start_distribution_counter);

        // perform advance epoch
        ika_system
            .inner_mut_for_testing()
            .advance_epoch(start_epoch + 1, 65, balance::zero(), balance::zero(), 0, 0, 0, 0, epoch_start_time, ctx)
            .destroy_for_testing(); // balance returned from `advance_epoch`
        ctx.increment_epoch_number();

        // should distribute 4 epochs worth of subsidies: 560, 561, 562, 563
        assert_eq(ika_system.get_stake_subsidy_distribution_counter(), start_distribution_counter + 4);
        check_distribution_counter_invariant(&mut ika_system, ctx);

        // ensure that next epoch change only distributes one epoch's worth
        ika_system
            .inner_mut_for_testing()
            .advance_epoch(start_epoch + 2, 65, balance::zero(), balance::zero(), 0, 0, 0, 0, epoch_start_time + epoch_duration, ctx)
            .destroy_for_testing(); // balance returned from `advance_epoch`
        ctx.increment_epoch_number();

        // should distribute 1 epoch's worth of subsidies
        assert_eq(ika_system.get_stake_subsidy_distribution_counter(), start_distribution_counter + 5);
        check_distribution_counter_invariant(&mut ika_system, ctx);

        test_scenario::return_shared(ika_system);
        test.end();
    }

    #[test]
    // Test that the fix for the subsidy distribution doesn't affect testnet,
    // where the distribution has no epoch delay, and the condition could result
    // in arithmetic error.
    fun test_stake_subsidy_with_safe_mode_testnet() {
        use std::unit_test::assert_eq;

        set_up_ika_system_state_with_big_amounts();

        let mut test = test_scenario::begin(VALIDATOR_ADDR_1);
        let mut ika_system = test.take_shared<IkaSystemState>();

        let ctx = test.ctx();

        // increment epoch number (safe mode emulation)
        540u64.do!(|_| ctx.increment_epoch_number());
        ika_system.set_epoch_for_testing(540);
        ika_system.set_stake_subsidy_distribution_counter(540);

        assert!(ctx.epoch() == 540);
        assert!(ika_system.get_stake_subsidy_distribution_counter() == 540);

        // perform advance epoch
        ika_system
            .inner_mut_for_testing()
            .advance_epoch(541, 65, balance::zero(), balance::zero(), 0, 0, 0, 0, 100000000000, ctx)
            .destroy_for_testing(); // balance returned from `advance_epoch`

        assert_eq!(ika_system.get_stake_subsidy_distribution_counter(), 541);

        test_scenario::return_shared(ika_system);
        test.end();
    }
}
