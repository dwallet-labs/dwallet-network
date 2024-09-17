// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module pera_system::pera_system_state_inner {
    use pera::balance::{Self, Balance};
    use pera::pera::PERA;
    use pera::tx_context::TxContext;
    use pera::bag::{Self, Bag};
    use pera::table::{Self, Table};
    use pera::object::ID;

    use pera_system::validator::Validator;
    use pera_system::validator_wrapper::ValidatorWrapper;
    use pera_system::validator_wrapper;
    use pera_system::validator;
    use pera::object;

    const SYSTEM_STATE_VERSION_V1: u64 = 18446744073709551605;  // u64::MAX - 10

    public struct SystemParameters has store {
        epoch_duration_ms: u64,
        extra_fields: Bag,
    }

    public struct ValidatorSet has store {
        active_validators: vector<Validator>,
        inactive_validators: Table<ID, ValidatorWrapper>,
        extra_fields: Bag,
    }

    public struct PeraSystemStateInner has store {
        epoch: u64,
        protocol_version: u64,
        system_state_version: u64,
        validators: ValidatorSet,
        storage_fund: Balance<PERA>,
        parameters: SystemParameters,
        reference_gas_price: u64,
        safe_mode: bool,
        epoch_start_timestamp_ms: u64,
        extra_fields: Bag,
    }

    public(package) fun create(
        validators: vector<Validator>,
        storage_fund: Balance<PERA>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        ctx: &mut TxContext,
    ): PeraSystemStateInner {
        let validators = new_validator_set(validators, ctx);
        let mut system_state = PeraSystemStateInner {
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
        // Add a dummy inactive validator so that we could test validator upgrade through wrapper latter.
        add_dummy_inactive_validator_for_testing(&mut system_state, ctx);
        system_state
    }

    public(package) fun advance_epoch(
        self: &mut PeraSystemStateInner,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_reward: Balance<PERA>,
        computation_reward: Balance<PERA>,
        storage_rebate_amount: u64,
        epoch_start_timestamp_ms: u64,
    ) : Balance<PERA> {
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

    public(package) fun protocol_version(self: &PeraSystemStateInner): u64 { self.protocol_version }
    public(package) fun system_state_version(self: &PeraSystemStateInner): u64 { self.system_state_version }
    public(package) fun genesis_system_state_version(): u64 {
        SYSTEM_STATE_VERSION_V1
    }

    public(package) fun add_dummy_inactive_validator_for_testing(self: &mut PeraSystemStateInner, ctx: &mut TxContext) {
        // Add a new entry to the inactive validator table for upgrade testing.
        let dummy_inactive_validator = validator_wrapper::create_v1(
            validator::new_dummy_inactive_validator(ctx),
            ctx,
        );
        table::add(&mut self.validators.inactive_validators, object::id_from_address(@0x0), dummy_inactive_validator);
    }

    fun new_validator_set(init_active_validators: vector<Validator>, ctx: &mut TxContext): ValidatorSet {
        ValidatorSet {
            active_validators: init_active_validators,
            inactive_validators: table::new(ctx),
            extra_fields: bag::new(ctx),
        }
    }
}
