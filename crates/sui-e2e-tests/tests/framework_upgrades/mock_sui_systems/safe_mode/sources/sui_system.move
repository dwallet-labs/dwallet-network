// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dwallet_system::dwallet_system {
    use dwallet::balance::Balance;
    use dwallet::object::UID;
    use dwallet::dwlt::DWLT;
    use dwallet::transfer;
    use dwallet::tx_context::{Self, TxContext};
    use dwallet::dynamic_field;

    use dwallet_system::validator::Validator;
    use dwallet_system::dwallet_system_state_inner::DWalletSystemStateInner;
    use dwallet_system::dwallet_system_state_inner;

    friend dwallet_system::genesis;

    struct DWalletSystemState has key {
        id: UID,
        version: u64,
    }

    public(friend) fun create(
        id: UID,
        validators: vector<Validator>,
        storage_fund: Balance<DWLT>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = dwallet_system_state_inner::create(
            validators,
            storage_fund,
            protocol_version,
            epoch_start_timestamp_ms,
            epoch_duration_ms,
            ctx,
        );
        let version = dwallet_system_state_inner::genesis_system_state_version();
        let self = DWalletSystemState {
            id,
            version,
        };
        dynamic_field::add(&mut self.id, version, system_state);
        transfer::share_object(self);
    }

    fun advance_epoch(
        storage_reward: Balance<DWLT>,
        computation_reward: Balance<DWLT>,
        wrapper: &mut DWalletSystemState,
        _new_epoch: u64,
        _next_protocol_version: u64,
        storage_rebate: u64,
        _non_refundable_storage_fee: u64,
        _storage_fund_reinvest_rate: u64,
        _reward_slashing_rate: u64,
        _epoch_start_timestamp_ms: u64,
        ctx: &mut TxContext,
    ) : Balance<DWLT> {
        let self = load_system_state_mut(wrapper);
        assert!(tx_context::sender(ctx) == @0x1, 0); // aborts here
        dwallet_system_state_inner::advance_epoch(
            self,
            storage_reward,
            computation_reward,
            storage_rebate,
        )
    }

    fun load_system_state_mut(self: &mut DWalletSystemState): &mut DWalletSystemStateInner {
        let version = self.version;
        dynamic_field::borrow_mut(&mut self.id, version)
    }
}
