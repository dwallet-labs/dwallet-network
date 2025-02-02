// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module ika_system::ika_system_state_inner {
    use std::vector;

    use ika::balance::{Self, Balance};
    use ika::ika::IKA;
    use ika::tx_context::TxContext;
    use ika::bag::{Self, Bag};
    use ika::table::{Self, Table};
    use ika::object::ID;

    use ika_system::validator::{Validator, ValidatorV2};
    use ika_system::validator_wrapper::ValidatorWrapper;
    use ika_system::validator_wrapper;
    use ika::object;
    use ika_system::validator;

    const SYSTEM_STATE_VERSION_V1: u64 = 18446744073709551605;  // u64::MAX - 10
    // Not using MAX - 9 since it's already used in the shallow upgrade test.
    const SYSTEM_STATE_VERSION_V2: u64 = 18446744073709551607;  // u64::MAX - 8

    public struct SystemParameters has store {
        epoch_duration_ms: u64,
        extra_fields: Bag,
    }

    public struct ValidatorSet has store {
        active_validators: vector<Validator>,
        inactive_validators: Table<ID, ValidatorWrapper>,
        extra_fields: Bag,
    }

    public struct ValidatorSetV2 has store {
        active_validators: vector<ValidatorV2>,
        inactive_validators: Table<ID, ValidatorWrapper>,
        extra_fields: Bag,
    }

    public struct IkaSystemStateInner has store {
        epoch: u64,
        protocol_version: u64,
        system_state_version: u64,
        validators: ValidatorSet,
        storage_fund: Balance<IKA>,
        parameters: SystemParameters,
        reference_gas_price: u64,
        safe_mode: bool,
        epoch_start_timestamp_ms: u64,
        extra_fields: Bag,
    }

    public struct IkaSystemStateInnerV2 has store {
        new_dummy_field: u64,
        epoch: u64,
        protocol_version: u64,
        system_state_version: u64,
        validators: ValidatorSetV2,
        storage_fund: Balance<IKA>,
        parameters: SystemParameters,
        reference_gas_price: u64,
        safe_mode: bool,
        epoch_start_timestamp_ms: u64,
        extra_fields: Bag,
    }

    public(package) fun create(
        validators: vector<Validator>,
        storage_fund: Balance<IKA>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        ctx: &mut TxContext,
    ): IkaSystemStateInner {
        let validators = new_validator_set(validators, ctx);
        let system_state = IkaSystemStateInner {
            epoch: 0,
            protocol_version,
            system_state_version: genesis_system_state_version(),
            validators,
            storage_fund,
            parameters: SystemParameters {
                epoch_duration_ms,
                extra_fields: bag::new(ctx),
            },
            reference_gas_price: 1,
            safe_mode: false,
            epoch_start_timestamp_ms,
            extra_fields: bag::new(ctx),
        };
        system_state
    }

    public(package) fun advance_epoch(
        self: &mut IkaSystemStateInnerV2,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_reward: Balance<IKA>,
        computation_reward: Balance<IKA>,
        storage_rebate_amount: u64,
        epoch_start_timestamp_ms: u64,
    ) : Balance<IKA> {
        touch_dummy_inactive_validator(self);

        self.epoch_start_timestamp_ms = epoch_start_timestamp_ms;
        self.epoch = self.epoch + 1;
        assert!(new_epoch == self.epoch, 0);
        self.safe_mode = false;
        self.protocol_version = next_protocol_version;

        balance::join(&mut self.storage_fund, computation_reward);
        balance::join(&mut self.storage_fund, storage_reward);
        let storage_rebate = balance::split(&mut self.storage_fund, storage_rebate_amount);
        storage_rebate
    }

    public(package) fun protocol_version(self: &IkaSystemStateInnerV2): u64 { self.protocol_version }
    public(package) fun system_state_version(self: &IkaSystemStateInnerV2): u64 { self.system_state_version }
    public(package) fun genesis_system_state_version(): u64 {
        SYSTEM_STATE_VERSION_V1
    }

    fun new_validator_set(init_active_validators: vector<Validator>, ctx: &mut TxContext): ValidatorSet {
        ValidatorSet {
            active_validators: init_active_validators,
            inactive_validators: table::new(ctx),
            extra_fields: bag::new(ctx),
        }
    }

    public(package) fun v1_to_v2(v1: IkaSystemStateInner): IkaSystemStateInnerV2 {
        let IkaSystemStateInner {
            epoch,
            protocol_version,
            system_state_version: old_system_state_version,
            validators,
            storage_fund,
            parameters,
            reference_gas_price,
            safe_mode,
            epoch_start_timestamp_ms,
            extra_fields,
        } = v1;
        let new_validator_set = validator_set_v1_to_v2(validators);
        assert!(old_system_state_version == SYSTEM_STATE_VERSION_V1, 0);
        IkaSystemStateInnerV2 {
            new_dummy_field: 100,
            epoch,
            protocol_version,
            system_state_version: SYSTEM_STATE_VERSION_V2,
            validators: new_validator_set,
            storage_fund,
            parameters,
            reference_gas_price,
            safe_mode,
            epoch_start_timestamp_ms,
            extra_fields,
        }
    }

    /// Load the dummy inactive validator added in the base version, trigger it to be upgraded.
    fun touch_dummy_inactive_validator(self: &mut IkaSystemStateInnerV2) {
        let validator_wrapper = table::borrow_mut(&mut self.validators.inactive_validators, object::id_from_address(@0x0));
        let _ = validator_wrapper::load_validator_maybe_upgrade(validator_wrapper);
    }

    fun validator_set_v1_to_v2(v1: ValidatorSet): ValidatorSetV2 {
        let ValidatorSet {
            mut active_validators,
            inactive_validators,
            extra_fields,
        } = v1;
        let mut new_active_validators = vector[];
        while (!vector::is_empty(&active_validators)) {
            let validator = vector::pop_back(&mut active_validators);
            let validator = validator::v1_to_v2(validator);
            vector::push_back(&mut new_active_validators, validator);
        };
        vector::destroy_empty(active_validators);
        vector::reverse(&mut new_active_validators);
        ValidatorSetV2 {
            active_validators: new_active_validators,
            inactive_validators,
            extra_fields,
        }
    }
}
