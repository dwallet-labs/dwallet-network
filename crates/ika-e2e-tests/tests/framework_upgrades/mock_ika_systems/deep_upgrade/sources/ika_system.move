// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module ika_system::ika_system {
    use std::vector;

    use ika::balance::Balance;
    use ika::object::UID;
    use ika::ika::IKA;
    use ika::transfer;
    use ika::tx_context::{Self, TxContext};
    use ika::dynamic_field;

    use ika_system::validator::Validator;
    use ika_system::ika_system_state_inner::{Self, IkaSystemStateInnerV2, IkaSystemStateInner};

    public struct IkaSystemState has key {
        id: UID,
        version: u64,
    }

    public(package) fun create(
        id: UID,
        validators: vector<Validator>,
        storage_fund: Balance<IKA>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = ika_system_state_inner::create(
            validators,
            storage_fund,
            protocol_version,
            epoch_start_timestamp_ms,
            epoch_duration_ms,
            ctx,
        );
        let version = ika_system_state_inner::genesis_system_state_version();
        let mut self = IkaSystemState {
            id,
            version,
        };
        dynamic_field::add(&mut self.id, version, system_state);
        transfer::share_object(self);
    }

    fun advance_epoch(
        storage_reward: Balance<IKA>,
        computation_reward: Balance<IKA>,
        wrapper: &mut IkaSystemState,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_rebate: u64,
        _non_refundable_storage_fee: u64,
        _storage_fund_reinvest_rate: u64, // share of storage fund's rewards that's reinvested
                                         // into storage fund, in basis point.
        _reward_slashing_rate: u64, // how much rewards are slashed to punish a validator, in bps.
        epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
        ctx: &mut TxContext,
    ) : Balance<IKA> {
        let self = load_system_state_mut(wrapper);
        assert!(tx_context::sender(ctx) == @0x0, 0);
        let storage_rebate = ika_system_state_inner::advance_epoch(
            self,
            new_epoch,
            next_protocol_version,
            storage_reward,
            computation_reward,
            storage_rebate,
            epoch_start_timestamp_ms,
        );

        storage_rebate
    }

    public fun active_validator_addresses(wrapper: &mut IkaSystemState): vector<address> {
        vector::empty()
    }

    fun load_system_state_mut(self: &mut IkaSystemState): &mut IkaSystemStateInnerV2 {
        load_inner_maybe_upgrade(self)
    }

    fun load_inner_maybe_upgrade(self: &mut IkaSystemState): &mut IkaSystemStateInnerV2 {
        let mut version = self.version;
        if (version == ika_system_state_inner::genesis_system_state_version()) {
            let inner: IkaSystemStateInner = dynamic_field::remove(&mut self.id, version);
            let new_inner = ika_system_state_inner::v1_to_v2(inner);
            version = ika_system_state_inner::system_state_version(&new_inner);
            dynamic_field::add(&mut self.id, version, new_inner);
            self.version = version;
        };

        let inner: &mut IkaSystemStateInnerV2 = dynamic_field::borrow_mut(&mut self.id, version);
        assert!(ika_system_state_inner::system_state_version(inner) == version, 0);
        inner
    }
}
