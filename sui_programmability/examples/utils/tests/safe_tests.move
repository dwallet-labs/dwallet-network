// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[test_only]
module utils::safe_tests {
    use utils::safe::{Self, Safe, TransferCapability, OwnerCapability};
    use dwallet::test_scenario::{Self as ts, Scenario, ctx};
    use dwallet::coin::{Self, Coin};
    use dwallet::object::{Self, ID};
    use dwallet::dwlt::DWLT;
    use dwallet::transfer;
    use dwallet::test_utils;

    const TEST_SENDER_ADDR: address = @0x1;
    const TEST_OWNER_ADDR: address = @0x1337;
    const TEST_DELEGATEE_ADDR: address = @0x1ce1ce1ce;

    fun create_safe(scenario: &mut Scenario, owner: address, stored_amount: u64) {
        ts::next_tx(scenario, owner);
        {
            let coin = coin::mint_for_testing<DWLT>(stored_amount, ctx(scenario));
            safe::create(coin, ctx(scenario));
        };
    }

    // Delegates the safe to delegatee and return the capability ID.
    fun delegate_safe(scenario: &mut Scenario, owner: address, delegate_to: address, delegate_amount: u64): ID {
        let id;
        ts::next_tx(scenario, owner);
        let safe = ts::take_shared<Safe<DWLT>>(scenario);
        let cap = ts::take_from_sender<OwnerCapability<DWLT>>(scenario);
        let capability = safe::create_transfer_capability(&mut safe, &cap, delegate_amount, ctx(scenario));
        id = object::id(&capability);
        transfer::public_transfer(capability, delegate_to);
        ts::return_to_sender(scenario, cap);
        ts::return_shared(safe);
        id
    }

    fun withdraw_as_delegatee(scenario: &mut Scenario, delegatee: address, withdraw_amount: u64) {
        ts::next_tx(scenario, delegatee);
        let safe = ts::take_shared<Safe<DWLT>>(scenario);
        let capability = ts::take_from_sender<TransferCapability<DWLT>>(scenario);
        let balance = safe::debit(&mut safe, &mut capability, withdraw_amount);
        test_utils::destroy(balance);

        ts::return_to_sender(scenario, capability);
        ts::return_shared(safe);
    }

    fun revoke_capability(scenario: &mut Scenario, owner: address, capability_id: ID) {
        ts::next_tx(scenario, owner);
        let safe = ts::take_shared<Safe<DWLT>>(scenario);
        let cap = ts::take_from_sender<OwnerCapability<DWLT>>(scenario);
        safe::revoke_transfer_capability(&mut safe, &cap, capability_id);

        ts::return_to_sender(scenario, cap);
        ts::return_shared(safe);
    }

    #[test]
    /// Ensure that all funds can be withdrawn by the owners
    fun test_safe_create_and_withdraw_funds_as_owner() {
        let owner = TEST_OWNER_ADDR;
        let scenario_val = ts::begin(TEST_SENDER_ADDR);
        let scenario = &mut scenario_val;

        let initial_funds = 1000u64;
        create_safe(scenario, owner, initial_funds);

        ts::next_tx(scenario, owner);
        let safe = ts::take_shared<Safe<DWLT>>(scenario);
        let cap = ts::take_from_sender<OwnerCapability<DWLT>>(scenario);

        safe::withdraw(&mut safe, &cap, initial_funds, ts::ctx(scenario));
        ts::next_tx(scenario, owner);
        let withdrawn_coin = ts::take_from_sender<Coin<DWLT>>(scenario);
        assert!(coin::value(&withdrawn_coin) == initial_funds, 0);

        test_utils::destroy(withdrawn_coin);
        ts::return_to_sender(scenario, cap);
        ts::return_shared(safe);


        ts::end(scenario_val);
    }

    #[test]
    /// Ensure that all funds can be withdrawn to a delegator
    fun test_safe_create_and_withdraw_funds_as_delegatee() {
        let owner = TEST_OWNER_ADDR;
        let delegatee = TEST_DELEGATEE_ADDR;
        let scenario_val = ts::begin(TEST_SENDER_ADDR);
        let scenario = &mut scenario_val;

        let initial_funds = 1000u64;
        let delegated_funds = 1000u64;
        // Create Safe
        create_safe(scenario, owner, initial_funds);
        delegate_safe(scenario, owner, delegatee, delegated_funds);
        withdraw_as_delegatee(scenario, delegatee, delegated_funds);
        ts::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = safe::EOverdrawn)]
    /// Ensure that funds cannot be over withdrawn
    fun test_safe_attempt_to_over_withdraw() {
        let owner = TEST_OWNER_ADDR;
        let delegatee = TEST_DELEGATEE_ADDR;
        let scenario_val = ts::begin(TEST_SENDER_ADDR);
        let scenario = &mut scenario_val;

        let initial_funds = 1000u64;
        let delegated_funds = 1000u64;
        // Create Safe
        create_safe(scenario, owner, initial_funds);
        delegate_safe(scenario, owner, delegatee, delegated_funds);

        // Withdraw all funds
        withdraw_as_delegatee(scenario, delegatee, delegated_funds);
        // Attempt to withdraw by 1 coin.
        withdraw_as_delegatee(scenario, delegatee, 1);

        ts::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = safe::ETransferCapabilityRevoked)]
    /// Ensure that funds cannot be over withdrawn
    fun test_safe_withdraw_revoked() {
        let owner = TEST_OWNER_ADDR;
        let delegatee = TEST_DELEGATEE_ADDR;
        let scenario_val = ts::begin(TEST_SENDER_ADDR);
        let scenario = &mut scenario_val;

        let initial_funds = 1000u64;
        let delegated_funds = 1000u64;
        // Create Safe
        create_safe(scenario, owner, initial_funds);
        let capability_id = delegate_safe(scenario, owner, delegatee, delegated_funds);

        revoke_capability(scenario, owner, capability_id);

        // Withdraw funds
        withdraw_as_delegatee(scenario, delegatee, delegated_funds);

        ts::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = safe::ETransferCapabilityRevoked)]
    /// Ensure owner cannot withdraw funds after revoking itself.
    fun test_safe_withdraw_self_revoked() {
        let owner = TEST_OWNER_ADDR;
        let scenario_val = ts::begin(owner);
        let scenario = &mut scenario_val;

        let initial_funds = 1000u64;
        create_safe(scenario, owner, initial_funds);

        ts::next_tx(scenario, owner);
        let cap = ts::take_from_sender<OwnerCapability<DWLT>>(scenario);
        let safe = ts::take_shared<Safe<DWLT>>(scenario);
        let transfer_capability = safe::create_transfer_capability(&mut safe, &cap, initial_funds, ctx(scenario));
        // Function under test
        safe::self_revoke_transfer_capability(&mut safe, &transfer_capability);
        ts::return_shared(safe);

        // Try withdraw funds with transfer capability.
        ts::next_tx(scenario, owner);
        let safe = ts::take_shared<Safe<DWLT>>(scenario);
        let balance = safe::debit(&mut safe, &mut transfer_capability, 1000u64);
        test_utils::destroy(balance);

        ts::return_shared(safe);
        ts::return_to_sender(scenario, cap);
        ts::return_to_sender(scenario, transfer_capability);
        ts::end(scenario_val);
    }
}
