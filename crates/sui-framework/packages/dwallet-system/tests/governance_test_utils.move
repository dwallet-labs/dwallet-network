// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[test_only]
module dwallet_system::governance_test_utils {
    use dwallet::address;
    use dwallet::balance;
    use dwallet::object;
    use dwallet::dwlt::DWLT;
    use dwallet::coin::{Self, Coin};
    use dwallet_system::staking_pool::{Self, StakedSui, StakingPool};
    use dwallet::test_utils::assert_eq;
    use dwallet::tx_context::{Self, TxContext};
    use dwallet_system::validator::{Self, Validator};
    use dwallet_system::dwallet_system::{Self, DWalletSystemState};
    use dwallet_system::dwallet_system_state_inner;
    use dwallet_system::stake_subsidy;
    use dwallet::test_scenario::{Self, Scenario};
    use dwallet_system::validator_set;
    use std::option;
    use std::vector;
    use dwallet::test_utils;
    use dwallet::balance::Balance;

    const MIST_PER_SUI: u64 = 1_000_000_000;

    public fun create_validator_for_testing(
        addr: address, init_stake_amount_in_sui: u64, ctx: &mut TxContext
    ): Validator {
        let validator = validator::new_for_testing(
            addr,
            x"AA",
            x"BB",
            x"CC",
            x"DD",
            b"ValidatorName",
            b"description",
            b"image_url",
            b"project_url",
            b"/ip4/127.0.0.1/tcp/80",
            b"/ip4/127.0.0.1/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            option::some(balance::create_for_testing<DWLT>(init_stake_amount_in_sui * MIST_PER_SUI)),
            1,
            0,
            true,
            ctx
        );
        validator
    }

    /// Create a validator set with the given stake amounts
    public fun create_validators_with_stakes(stakes: vector<u64>, ctx: &mut TxContext): vector<Validator> {
        let i = 0;
        let validators = vector[];
        while (i < vector::length(&stakes)) {
            let validator = create_validator_for_testing(address::from_u256((i as u256)), *vector::borrow(&stakes, i), ctx);
            vector::push_back(&mut validators, validator);
            i = i + 1
        };
        validators
    }

    public fun create_dwallet_system_state_for_testing(
        validators: vector<Validator>, sui_supply_amount: u64, storage_fund_amount: u64, ctx: &mut TxContext
    ) {
        let system_parameters = dwallet_system_state_inner::create_system_parameters(
            42,  // epoch_duration_ms, doesn't matter what number we put here
            0,   // stake_subsidy_start_epoch

            150, // max_validator_count
            1,   // min_validator_joining_stake
            1,   // validator_low_stake_threshold
            0,   // validator_very_low_stake_threshold
            7,   // validator_low_stake_grace_period
            ctx,
        );

        let stake_subsidy = stake_subsidy::create(
            balance::create_for_testing<DWLT>(sui_supply_amount * MIST_PER_SUI), // sui_supply
            0,   // stake subsidy initial distribution amount
            10,  // stake_subsidy_period_length
            0,   // stake_subsidy_decrease_rate
            ctx,
        );

        dwallet_system::create(
            object::new(ctx), // it doesn't matter what ID sui system state has in tests
            coin::create_treasury_cap_for_testing(ctx),
            validators,
            balance::create_for_testing<DWLT>(storage_fund_amount * MIST_PER_SUI), // storage_fund
            1,   // protocol version
            0,   // chain_start_timestamp_ms
            system_parameters,
            stake_subsidy,
            ctx,
        )
    }

    public fun set_up_dwallet_system_state(addrs: vector<address>) {
        let scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);
        let validators = vector::empty();

        while (!vector::is_empty(&addrs)) {
            vector::push_back(
                &mut validators,
                create_validator_for_testing(vector::pop_back(&mut addrs), 100, ctx)
            );
        };

        create_dwallet_system_state_for_testing(validators, 1000, 0, ctx);
        test_scenario::end(scenario_val);
    }

    public fun advance_epoch(scenario: &mut Scenario) {
        advance_epoch_with_reward_amounts(0, 0, scenario);
    }

    public fun advance_epoch_with_reward_amounts_return_rebate(
        storage_charge: u64, computation_charge: u64, stoarge_rebate: u64, non_refundable_storage_rebate: u64, scenario: &mut Scenario,
    ): Balance<DWLT> {
        test_scenario::next_tx(scenario, @0x0);
        let new_epoch = tx_context::epoch(test_scenario::ctx(scenario)) + 1;
        let system_state = test_scenario::take_shared<DWalletSystemState>(scenario);

        let ctx = test_scenario::ctx(scenario);

        let storage_rebate = dwallet_system::advance_epoch_for_testing(
            &mut system_state, new_epoch, 1, storage_charge, computation_charge, stoarge_rebate, non_refundable_storage_rebate, 0, 0, 0, ctx,
        );
        test_scenario::return_shared(system_state);
        test_scenario::next_epoch(scenario, @0x0);
        storage_rebate
    }

    public fun advance_epoch_with_reward_amounts(
        storage_charge: u64, computation_charge: u64, scenario: &mut Scenario
    ) {
        let storage_rebate = advance_epoch_with_reward_amounts_return_rebate(storage_charge * MIST_PER_SUI, computation_charge * MIST_PER_SUI, 0, 0, scenario);
        test_utils::destroy(storage_rebate)
    }

    public fun advance_epoch_with_reward_amounts_and_slashing_rates(
        storage_charge: u64,
        computation_charge: u64,
        reward_slashing_rate: u64,
        scenario: &mut Scenario
    ) {
        test_scenario::next_tx(scenario, @0x0);
        let new_epoch = tx_context::epoch(test_scenario::ctx(scenario)) + 1;
        let system_state = test_scenario::take_shared<DWalletSystemState>(scenario);

        let ctx = test_scenario::ctx(scenario);

        let storage_rebate = dwallet_system::advance_epoch_for_testing(
            &mut system_state, new_epoch, 1, storage_charge * MIST_PER_SUI, computation_charge * MIST_PER_SUI, 0, 0, 0, reward_slashing_rate, 0, ctx
        );
        test_utils::destroy(storage_rebate);
        test_scenario::return_shared(system_state);
        test_scenario::next_epoch(scenario, @0x0);
    }

    public fun stake_with(
        staker: address, validator: address, amount: u64, scenario: &mut Scenario
    ) {
        test_scenario::next_tx(scenario, staker);
        let system_state = test_scenario::take_shared<DWalletSystemState>(scenario);

        let ctx = test_scenario::ctx(scenario);

        dwallet_system::request_add_stake(&mut system_state, coin::mint_for_testing(amount * MIST_PER_SUI, ctx), validator, ctx);
        test_scenario::return_shared(system_state);
    }

    public fun unstake(
        staker: address, staked_sui_idx: u64, scenario: &mut Scenario
    ) {
        test_scenario::next_tx(scenario, staker);
        let stake_sui_ids = test_scenario::ids_for_sender<StakedSui>(scenario);
        let staked_sui = test_scenario::take_from_sender_by_id(scenario, *vector::borrow(&stake_sui_ids, staked_sui_idx));
        let system_state = test_scenario::take_shared<DWalletSystemState>(scenario);

        let ctx = test_scenario::ctx(scenario);
        dwallet_system::request_withdraw_stake(&mut system_state, staked_sui, ctx);
        test_scenario::return_shared(system_state);
    }

    public fun add_validator_full_flow(validator: address, name: vector<u8>, net_addr: vector<u8>, init_stake_amount: u64, pubkey: vector<u8>, pop: vector<u8>, scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, validator);
        let system_state = test_scenario::take_shared<DWalletSystemState>(scenario);
        let ctx = test_scenario::ctx(scenario);

        dwallet_system::request_add_validator_candidate(
            &mut system_state,
            pubkey,
            vector[171, 2, 39, 3, 139, 105, 166, 171, 153, 151, 102, 197, 151, 186, 140, 116, 114, 90, 213, 225, 20, 167, 60, 69, 203, 12, 180, 198, 9, 217, 117, 38],
            vector[171, 3, 39, 3, 139, 105, 166, 171, 153, 151, 102, 197, 151, 186, 140, 116, 114, 90, 213, 225, 20, 167, 60, 69, 203, 12, 180, 198, 9, 217, 117, 38],
            pop,
            name,
            b"description",
            b"image_url",
            b"project_url",
            net_addr,
            net_addr,
            net_addr,
            net_addr,
            1,
            0,
            ctx
        );
        dwallet_system::request_add_stake(&mut system_state, coin::mint_for_testing<DWLT>(init_stake_amount * MIST_PER_SUI, ctx), validator, ctx);
        dwallet_system::request_add_validator_for_testing(&mut system_state, 0, ctx);
        test_scenario::return_shared(system_state);
    }

    public fun add_validator_candidate(validator: address, name: vector<u8>, net_addr: vector<u8>, pubkey: vector<u8>, pop: vector<u8>, scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, validator);
        let system_state = test_scenario::take_shared<DWalletSystemState>(scenario);
        let ctx = test_scenario::ctx(scenario);

        dwallet_system::request_add_validator_candidate(
            &mut system_state,
            pubkey,
            vector[171, 2, 39, 3, 139, 105, 166, 171, 153, 151, 102, 197, 151, 186, 140, 116, 114, 90, 213, 225, 20, 167, 60, 69, 203, 12, 180, 198, 9, 217, 117, 38],
            vector[171, 3, 39, 3, 139, 105, 166, 171, 153, 151, 102, 197, 151, 186, 140, 116, 114, 90, 213, 225, 20, 167, 60, 69, 203, 12, 180, 198, 9, 217, 117, 38],
            pop,
            name,
            b"description",
            b"image_url",
            b"project_url",
            net_addr,
            net_addr,
            net_addr,
            net_addr,
            1,
            0,
            ctx
        );
        test_scenario::return_shared(system_state);
    }

    public fun remove_validator_candidate(validator: address, scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, validator);
        let system_state = test_scenario::take_shared<DWalletSystemState>(scenario);
        let ctx = test_scenario::ctx(scenario);

        dwallet_system::request_remove_validator_candidate(
            &mut system_state,
            ctx
        );
        test_scenario::return_shared(system_state);
    }

    public fun add_validator(validator: address, scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, validator);
        let system_state = test_scenario::take_shared<DWalletSystemState>(scenario);
        let ctx = test_scenario::ctx(scenario);

        dwallet_system::request_add_validator_for_testing(
            &mut system_state,
            0,
            ctx
        );
        test_scenario::return_shared(system_state);
    }

    public fun remove_validator(validator: address, scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, validator);
        let system_state = test_scenario::take_shared<DWalletSystemState>(scenario);

        let ctx = test_scenario::ctx(scenario);

        dwallet_system::request_remove_validator(
            &mut system_state,
            ctx
        );
        test_scenario::return_shared(system_state);
    }

    public fun assert_validator_self_stake_amounts(validator_addrs: vector<address>, stake_amounts: vector<u64>, scenario: &mut Scenario) {
        let i = 0;
        while (i < vector::length(&validator_addrs)) {
            let validator_addr = *vector::borrow(&validator_addrs, i);
            let amount = *vector::borrow(&stake_amounts, i);

            test_scenario::next_tx(scenario, validator_addr);
            let system_state = test_scenario::take_shared<DWalletSystemState>(scenario);
            let stake_plus_rewards = stake_plus_current_rewards_for_validator(validator_addr, &mut system_state, scenario);
            assert_eq(stake_plus_rewards, amount);
            test_scenario::return_shared(system_state);
            i = i + 1;
        };
    }

    public fun assert_validator_total_stake_amounts(validator_addrs: vector<address>, stake_amounts: vector<u64>, scenario: &mut Scenario) {
        let i = 0;
        while (i < vector::length(&validator_addrs)) {
            let validator_addr = *vector::borrow(&validator_addrs, i);
            let amount = *vector::borrow(&stake_amounts, i);

            test_scenario::next_tx(scenario, validator_addr);
            let system_state = test_scenario::take_shared<DWalletSystemState>(scenario);
            let validator_amount = dwallet_system::validator_stake_amount(&mut system_state, validator_addr);
            assert!(validator_amount == amount, validator_amount);
            test_scenario::return_shared(system_state);
            i = i + 1;
        };
    }

    public fun assert_validator_non_self_stake_amounts(validator_addrs: vector<address>, stake_amounts: vector<u64>, scenario: &mut Scenario) {
        let i = 0;
        while (i < vector::length(&validator_addrs)) {
            let validator_addr = *vector::borrow(&validator_addrs, i);
            let amount = *vector::borrow(&stake_amounts, i);
            test_scenario::next_tx(scenario, validator_addr);
            let system_state = test_scenario::take_shared<DWalletSystemState>(scenario);
            let non_self_stake_amount = dwallet_system::validator_stake_amount(&mut system_state, validator_addr) - stake_plus_current_rewards_for_validator(validator_addr, &mut system_state, scenario);
            assert_eq(non_self_stake_amount, amount);
            test_scenario::return_shared(system_state);
            i = i + 1;
        };
    }

    /// Return the rewards for the validator at `addr` in terms of SUI.
    public fun stake_plus_current_rewards_for_validator(addr: address, system_state: &mut DWalletSystemState, scenario: &mut Scenario): u64 {
        let validator_ref = validator_set::get_active_validator_ref(dwallet_system::validators(system_state), addr);
        let amount = stake_plus_current_rewards(addr, validator::get_staking_pool_ref(validator_ref), scenario);
        amount
    }

    public fun stake_plus_current_rewards(addr: address, staking_pool: &StakingPool, scenario: &mut Scenario): u64 {
        let sum = 0;
        test_scenario::next_tx(scenario, addr);
        let stake_ids = test_scenario::ids_for_sender<StakedSui>(scenario);
        let current_epoch = tx_context::epoch(test_scenario::ctx(scenario));

        while (!vector::is_empty(&stake_ids)) {
            let staked_sui_id = vector::pop_back(&mut stake_ids);
            let staked_sui = test_scenario::take_from_sender_by_id<StakedSui>(scenario, staked_sui_id);
            sum = sum + staking_pool::calculate_rewards(staking_pool, &staked_sui, current_epoch);
            test_scenario::return_to_sender(scenario, staked_sui);
        };
        sum
    }

    public fun total_sui_balance(addr: address, scenario: &mut Scenario): u64 {
        let sum = 0;
        test_scenario::next_tx(scenario, addr);
        let coin_ids = test_scenario::ids_for_sender<Coin<DWLT>>(scenario);
        let i = 0;
        while (i < vector::length(&coin_ids)) {
            let coin = test_scenario::take_from_sender_by_id<Coin<DWLT>>(scenario, *vector::borrow(&coin_ids, i));
            sum = sum + coin::value(&coin);
            test_scenario::return_to_sender(scenario, coin);
            i = i + 1;
        };
        sum
    }
}
