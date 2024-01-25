// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dwallet_system::dwallet_system_state_inner {
    use dwallet::balance::{Self, Balance};
    use dwallet::dwlt::DWLT;
    use dwallet::tx_context::TxContext;
    use dwallet::bag::{Self, Bag};
    use dwallet::table::{Self, Table};
    use dwallet::object::ID;

    use dwallet_system::validator::Validator;
    use dwallet_system::validator_wrapper::ValidatorWrapper;
    use dwallet_system::validator_wrapper;
    use dwallet_system::validator;
    use dwallet::object;

    friend dwallet_system::dwallet_system;

    const SYSTEM_STATE_VERSION_V1: u64 = 18446744073709551605;  // u64::MAX - 10

    struct SystemParameters has store {
        epoch_duration_ms: u64,
        extra_fields: Bag,
    }

    struct ValidatorSet has store {
        active_validators: vector<Validator>,
        inactive_validators: Table<ID, ValidatorWrapper>,
        extra_fields: Bag,
    }

    struct DWalletSystemStateInner has store {
        epoch: u64,
        protocol_version: u64,
        system_state_version: u64,
        validators: ValidatorSet,
        storage_fund: Balance<DWLT>,
        parameters: SystemParameters,
        reference_gas_price: u64,
        safe_mode: bool,
        epoch_start_timestamp_ms: u64,
        extra_fields: Bag,
    }

    public(friend) fun create(
        validators: vector<Validator>,
        storage_fund: Balance<DWLT>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        ctx: &mut TxContext,
    ): DWalletSystemStateInner {
        let validators = new_validator_set(validators, ctx);
        let system_state = DWalletSystemStateInner {
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

    public(friend) fun advance_epoch(
        self: &mut DWalletSystemStateInner,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_reward: Balance<DWLT>,
        computation_reward: Balance<DWLT>,
        storage_rebate_amount: u64,
        epoch_start_timestamp_ms: u64,
    ) : Balance<DWLT> {
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

    public(friend) fun protocol_version(self: &DWalletSystemStateInner): u64 { self.protocol_version }
    public(friend) fun system_state_version(self: &DWalletSystemStateInner): u64 { self.system_state_version }
    public(friend) fun genesis_system_state_version(): u64 {
        SYSTEM_STATE_VERSION_V1
    }

    public(friend) fun add_dummy_inactive_validator_for_testing(self: &mut DWalletSystemStateInner, ctx: &mut TxContext) {
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
