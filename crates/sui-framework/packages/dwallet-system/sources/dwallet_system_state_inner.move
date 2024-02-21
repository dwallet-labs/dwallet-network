// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dwallet_system::dwallet_system_state_inner {
    use dwallet::balance::{Self, Balance};
    use dwallet::coin::{Self, Coin};
    use dwallet::object::{ID};
    use dwallet_system::staking_pool::{stake_activation_epoch, StakedSui};
    use dwallet::dwlt::DWLT;
    use dwallet::transfer;
    use dwallet::tx_context::{Self, TxContext};
    use dwallet_system::validator::{Self, Validator};
    use dwallet_system::validator_set::{Self, ValidatorSet};
    use dwallet_system::validator_cap::{Self, UnverifiedValidatorOperationCap, ValidatorOperationCap};
    use dwallet_system::stake_subsidy::{Self, StakeSubsidy};
    use dwallet_system::storage_fund::{Self, StorageFund};
    use dwallet_system::staking_pool::PoolTokenExchangeRate;
    use dwallet::vec_map::{Self, VecMap};
    use dwallet::vec_set::{Self, VecSet};
    use std::option;
    use std::vector;
    use dwallet::pay;
    use dwallet::event;
    use dwallet::table::Table;
    use dwallet::bag::Bag;
    use dwallet::bag;

    friend dwallet_system::genesis;
    friend dwallet_system::dwallet_system;

    #[test_only]
    friend dwallet_system::governance_test_utils;

    // same as in validator_set
    const ACTIVE_VALIDATOR_ONLY: u8 = 1;
    const ACTIVE_OR_PENDING_VALIDATOR: u8 = 2;
    const ANY_VALIDATOR: u8 = 3;

    const SYSTEM_STATE_VERSION_V1: u64 = 1;

    /// A list of system config parameters.
    struct SystemParameters has store {
        /// The duration of an epoch, in milliseconds.
        epoch_duration_ms: u64,

        /// The starting epoch in which stake subsidies start being paid out
        stake_subsidy_start_epoch: u64,

        /// Maximum number of active validators at any moment.
        /// We do not allow the number of validators in any epoch to go above this.
        max_validator_count: u64,

        /// Lower-bound on the amount of stake required to become a validator.
        min_validator_joining_stake: u64,

        /// Validators with stake amount below `validator_low_stake_threshold` are considered to
        /// have low stake and will be escorted out of the validator set after being below this
        /// threshold for more than `validator_low_stake_grace_period` number of epochs.
        validator_low_stake_threshold: u64,

        /// Validators with stake below `validator_very_low_stake_threshold` will be removed
        /// immediately at epoch change, no grace period.
        validator_very_low_stake_threshold: u64,

        /// A validator can have stake below `validator_low_stake_threshold`
        /// for this many epochs before being kicked out.
        validator_low_stake_grace_period: u64,

        /// Any extra fields that's not defined statically.
        extra_fields: Bag,
    }

    /// Added min_validator_count.
    struct SystemParametersV2 has store {
        /// The duration of an epoch, in milliseconds.
        epoch_duration_ms: u64,

        /// The starting epoch in which stake subsidies start being paid out
        stake_subsidy_start_epoch: u64,

        /// Minimum number of active validators at any moment.
        min_validator_count: u64,

        /// Maximum number of active validators at any moment.
        /// We do not allow the number of validators in any epoch to go above this.
        max_validator_count: u64,

        /// Lower-bound on the amount of stake required to become a validator.
        min_validator_joining_stake: u64,

        /// Validators with stake amount below `validator_low_stake_threshold` are considered to
        /// have low stake and will be escorted out of the validator set after being below this
        /// threshold for more than `validator_low_stake_grace_period` number of epochs.
        validator_low_stake_threshold: u64,

        /// Validators with stake below `validator_very_low_stake_threshold` will be removed
        /// immediately at epoch change, no grace period.
        validator_very_low_stake_threshold: u64,

        /// A validator can have stake below `validator_low_stake_threshold`
        /// for this many epochs before being kicked out.
        validator_low_stake_grace_period: u64,

        /// Any extra fields that's not defined statically.
        extra_fields: Bag,
    }

    /// The top-level object containing all information of the Sui system.
    struct DWalletSystemStateInner has store {
        /// The current epoch ID, starting from 0.
        epoch: u64,
        /// The current protocol version, starting from 1.
        protocol_version: u64,
        /// The current version of the system state data structure type.
        /// This is always the same as DWalletSystemState.version. Keeping a copy here so that
        /// we know what version it is by inspecting DWalletSystemStateInner as well.
        system_state_version: u64,
        /// Contains all information about the validators.
        validators: ValidatorSet,
        /// The storage fund.
        storage_fund: StorageFund,
        /// A list of system config parameters.
        parameters: SystemParameters,
        /// The reference gas price for the current epoch.
        reference_gas_price: u64,
        /// A map storing the records of validator reporting each other.
        /// There is an entry in the map for each validator that has been reported
        /// at least once. The entry VecSet contains all the validators that reported
        /// them. If a validator has never been reported they don't have an entry in this map.
        /// This map persists across epoch: a peer continues being in a reported state until the
        /// reporter doesn't explicitly remove their report.
        /// Note that in case we want to support validator address change in future,
        /// the reports should be based on validator ids
        validator_report_records: VecMap<address, VecSet<address>>,
        /// Schedule of stake subsidies given out each epoch.
        stake_subsidy: StakeSubsidy,

        /// Whether the system is running in a downgraded safe mode due to a non-recoverable bug.
        /// This is set whenever we failed to execute advance_epoch, and ended up executing advance_epoch_safe_mode.
        /// It can be reset once we are able to successfully execute advance_epoch.
        /// The rest of the fields starting with `safe_mode_` are accmulated during safe mode
        /// when advance_epoch_safe_mode is executed. They will eventually be processed once we
        /// are out of safe mode.
        safe_mode: bool,
        safe_mode_storage_rewards: Balance<DWLT>,
        safe_mode_computation_rewards: Balance<DWLT>,
        safe_mode_storage_rebates: u64,
        safe_mode_non_refundable_storage_fee: u64,

        /// Unix timestamp of the current epoch start
        epoch_start_timestamp_ms: u64,
        /// Any extra fields that's not defined statically.
        extra_fields: Bag,
    }

    /// Uses SystemParametersV2 as the parameters.
    struct DWalletSystemStateInnerV2 has store {
        /// The current epoch ID, starting from 0.
        epoch: u64,
        /// The current protocol version, starting from 1.
        protocol_version: u64,
        /// The current version of the system state data structure type.
        /// This is always the same as DWalletSystemState.version. Keeping a copy here so that
        /// we know what version it is by inspecting DWalletSystemStateInner as well.
        system_state_version: u64,
        /// Contains all information about the validators.
        validators: ValidatorSet,
        /// The storage fund.
        storage_fund: StorageFund,
        /// A list of system config parameters.
        parameters: SystemParametersV2,
        /// The reference gas price for the current epoch.
        reference_gas_price: u64,
        /// A map storing the records of validator reporting each other.
        /// There is an entry in the map for each validator that has been reported
        /// at least once. The entry VecSet contains all the validators that reported
        /// them. If a validator has never been reported they don't have an entry in this map.
        /// This map persists across epoch: a peer continues being in a reported state until the
        /// reporter doesn't explicitly remove their report.
        /// Note that in case we want to support validator address change in future,
        /// the reports should be based on validator ids
        validator_report_records: VecMap<address, VecSet<address>>,
        /// Schedule of stake subsidies given out each epoch.
        stake_subsidy: StakeSubsidy,

        /// Whether the system is running in a downgraded safe mode due to a non-recoverable bug.
        /// This is set whenever we failed to execute advance_epoch, and ended up executing advance_epoch_safe_mode.
        /// It can be reset once we are able to successfully execute advance_epoch.
        /// The rest of the fields starting with `safe_mode_` are accmulated during safe mode
        /// when advance_epoch_safe_mode is executed. They will eventually be processed once we
        /// are out of safe mode.
        safe_mode: bool,
        safe_mode_storage_rewards: Balance<DWLT>,
        safe_mode_computation_rewards: Balance<DWLT>,
        safe_mode_storage_rebates: u64,
        safe_mode_non_refundable_storage_fee: u64,

        /// Unix timestamp of the current epoch start
        epoch_start_timestamp_ms: u64,
        /// Any extra fields that's not defined statically.
        extra_fields: Bag,
    }

    /// Event containing system-level epoch information, emitted during
    /// the epoch advancement transaction.
    struct SystemEpochInfoEvent has copy, drop {
        epoch: u64,
        protocol_version: u64,
        reference_gas_price: u64,
        total_stake: u64,
        storage_fund_reinvestment: u64,
        storage_charge: u64,
        storage_rebate: u64,
        storage_fund_balance: u64,
        stake_subsidy_amount: u64,
        total_gas_fees: u64,
        total_stake_rewards_distributed: u64,
        leftover_storage_fund_inflow: u64,
    }

    // Errors
    const ENotValidator: u64 = 0;
    const ELimitExceeded: u64 = 1;
    #[allow(unused_const)]
    const ENotSystemAddress: u64 = 2;
    const ECannotReportOneself: u64 = 3;
    const EReportRecordNotFound: u64 = 4;
    const EBpsTooLarge: u64 = 5;
    const EStakeWithdrawBeforeActivation: u64 = 6;
    const ESafeModeGasNotProcessed: u64 = 7;
    const EAdvancedToWrongEpoch: u64 = 8;

    const BASIS_POINT_DENOMINATOR: u128 = 10000;

    // ==== functions that can only be called by genesis ====

    /// Create a new DWalletSystemState object and make it shared.
    /// This function will be called only once in genesis.
    public(friend) fun create(
        validators: vector<Validator>,
        initial_storage_fund: Balance<DWLT>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        parameters: SystemParameters,
        stake_subsidy: StakeSubsidy,
        ctx: &mut TxContext,
    ): DWalletSystemStateInner {
        let validators = validator_set::new(validators, ctx);
        let reference_gas_price = validator_set::derive_reference_gas_price(&validators);
        // This type is fixed as it's created at genesis. It should not be updated during type upgrade.
        let system_state = DWalletSystemStateInner {
            epoch: 0,
            protocol_version,
            system_state_version: genesis_system_state_version(),
            validators,
            storage_fund: storage_fund::new(initial_storage_fund),
            parameters,
            reference_gas_price,
            validator_report_records: vec_map::empty(),
            stake_subsidy,
            safe_mode: false,
            safe_mode_storage_rewards: balance::zero(),
            safe_mode_computation_rewards: balance::zero(),
            safe_mode_storage_rebates: 0,
            safe_mode_non_refundable_storage_fee: 0,
            epoch_start_timestamp_ms,
            extra_fields: bag::new(ctx),
        };
        system_state
    }

    public(friend) fun create_system_parameters(
        epoch_duration_ms: u64,
        stake_subsidy_start_epoch: u64,

        // Validator committee parameters
        max_validator_count: u64,
        min_validator_joining_stake: u64,
        validator_low_stake_threshold: u64,
        validator_very_low_stake_threshold: u64,
        validator_low_stake_grace_period: u64,
        ctx: &mut TxContext,
    ): SystemParameters {
        SystemParameters {
            epoch_duration_ms,
            stake_subsidy_start_epoch,
            max_validator_count,
            min_validator_joining_stake,
            validator_low_stake_threshold,
            validator_very_low_stake_threshold,
            validator_low_stake_grace_period,
            extra_fields: bag::new(ctx),
        }
    }

    public(friend) fun v1_to_v2(self: DWalletSystemStateInner): DWalletSystemStateInnerV2 {
        let DWalletSystemStateInner {
            epoch,
            protocol_version,
            system_state_version: _,
            validators,
            storage_fund,
            parameters,
            reference_gas_price,
            validator_report_records,
            stake_subsidy,
            safe_mode,
            safe_mode_storage_rewards,
            safe_mode_computation_rewards,
            safe_mode_storage_rebates,
            safe_mode_non_refundable_storage_fee,
            epoch_start_timestamp_ms,
            extra_fields: state_extra_fields,
        } = self;
        let SystemParameters {
            epoch_duration_ms,
            stake_subsidy_start_epoch,
            max_validator_count,
            min_validator_joining_stake,
            validator_low_stake_threshold,
            validator_very_low_stake_threshold,
            validator_low_stake_grace_period,
            extra_fields: param_extra_fields,
        } = parameters;
        DWalletSystemStateInnerV2 {
            epoch,
            protocol_version,
            system_state_version: 2,
            validators,
            storage_fund,
            parameters: SystemParametersV2 {
                epoch_duration_ms,
                stake_subsidy_start_epoch,
                min_validator_count: 4,
                max_validator_count,
                min_validator_joining_stake,
                validator_low_stake_threshold,
                validator_very_low_stake_threshold,
                validator_low_stake_grace_period,
                extra_fields: param_extra_fields,
            },
            reference_gas_price,
            validator_report_records,
            stake_subsidy,
            safe_mode,
            safe_mode_storage_rewards,
            safe_mode_computation_rewards,
            safe_mode_storage_rebates,
            safe_mode_non_refundable_storage_fee,
            epoch_start_timestamp_ms,
            extra_fields: state_extra_fields
        }
    }

    // ==== public(friend) functions ====

    /// Can be called by anyone who wishes to become a validator candidate and starts accuring delegated
    /// stakes in their staking pool. Once they have at least `MIN_VALIDATOR_JOINING_STAKE` amount of stake they
    /// can call `request_add_validator` to officially become an active validator at the next epoch.
    /// Aborts if the caller is already a pending or active validator, or a validator candidate.
    /// Note: `proof_of_possession` MUST be a valid signature using sui_address and protocol_pubkey_bytes.
    /// To produce a valid PoP, run [fn test_proof_of_possession].
    public(friend) fun request_add_validator_candidate(
        self: &mut DWalletSystemStateInnerV2,
        pubkey_bytes: vector<u8>,
        network_pubkey_bytes: vector<u8>,
        worker_pubkey_bytes: vector<u8>,
        proof_of_possession: vector<u8>,
        name: vector<u8>,
        description: vector<u8>,
        image_url: vector<u8>,
        project_url: vector<u8>,
        net_address: vector<u8>,
        p2p_address: vector<u8>,
        primary_address: vector<u8>,
        worker_address: vector<u8>,
        gas_price: u64,
        commission_rate: u64,
        ctx: &mut TxContext,
    ) {
        let validator = validator::new(
            tx_context::sender(ctx),
            pubkey_bytes,
            network_pubkey_bytes,
            worker_pubkey_bytes,
            proof_of_possession,
            name,
            description,
            image_url,
            project_url,
            net_address,
            p2p_address,
            primary_address,
            worker_address,
            gas_price,
            commission_rate,
            ctx
        );

        validator_set::request_add_validator_candidate(&mut self.validators, validator, ctx);
    }

    /// Called by a validator candidate to remove themselves from the candidacy. After this call
    /// their staking pool becomes deactivate.
    public(friend) fun request_remove_validator_candidate(
        self: &mut DWalletSystemStateInnerV2,
        ctx: &mut TxContext,
    ) {
        validator_set::request_remove_validator_candidate(&mut self.validators, ctx);
    }

    /// Called by a validator candidate to add themselves to the active validator set beginning next epoch.
    /// Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
    /// stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
    /// epoch has already reached the maximum.
    public(friend) fun request_add_validator(
        self: &mut DWalletSystemStateInnerV2,
        ctx: &TxContext,
    ) {
        assert!(
            validator_set::next_epoch_validator_count(&self.validators) < self.parameters.max_validator_count,
            ELimitExceeded,
        );

        validator_set::request_add_validator(&mut self.validators, self.parameters.min_validator_joining_stake, ctx);
    }

    /// A validator can call this function to request a removal in the next epoch.
    /// We use the sender of `ctx` to look up the validator
    /// (i.e. sender must match the sui_address in the validator).
    /// At the end of the epoch, the `validator` object will be returned to the sui_address
    /// of the validator.
    public(friend) fun request_remove_validator(
        self: &mut DWalletSystemStateInnerV2,
        ctx: &TxContext,
    ) {
        // Only check min validator condition if the current number of validators satisfy the constraint.
        // This is so that if we somehow already are in a state where we have less than min validators, it no longer matters
        // and is ok to stay so. This is useful for a test setup.
        if (vector::length(validator_set::active_validators(&self.validators)) >= self.parameters.min_validator_count) {
            assert!(
                validator_set::next_epoch_validator_count(&self.validators) > self.parameters.min_validator_count,
                ELimitExceeded,
            );
        };

        validator_set::request_remove_validator(
            &mut self.validators,
            ctx,
        )
    }

    /// A validator can call this function to submit a new gas price quote, to be
    /// used for the reference gas price calculation at the end of the epoch.
    public(friend) fun request_set_gas_price(
        self: &mut DWalletSystemStateInnerV2,
        cap: &UnverifiedValidatorOperationCap,
        new_gas_price: u64,
    ) {
        // Verify the represented address is an active or pending validator, and the capability is still valid.
        let verified_cap = validator_set::verify_cap(&mut self.validators, cap, ACTIVE_OR_PENDING_VALIDATOR);
        let validator = validator_set::get_validator_mut_with_verified_cap(&mut self.validators, &verified_cap, false /* include_candidate */);

        validator::request_set_gas_price(validator, verified_cap, new_gas_price);
    }

    /// This function is used to set new gas price for candidate validators
    public(friend) fun set_candidate_validator_gas_price(
        self: &mut DWalletSystemStateInnerV2,
        cap: &UnverifiedValidatorOperationCap,
        new_gas_price: u64,
    ) {
        // Verify the represented address is an active or pending validator, and the capability is still valid.
        let verified_cap = validator_set::verify_cap(&mut self.validators, cap, ANY_VALIDATOR);
        let candidate = validator_set::get_validator_mut_with_verified_cap(&mut self.validators, &verified_cap, true /* include_candidate */);
        validator::set_candidate_gas_price(candidate, verified_cap, new_gas_price)
    }

    /// A validator can call this function to set a new commission rate, updated at the end of
    /// the epoch.
    public(friend) fun request_set_commission_rate(
        self: &mut DWalletSystemStateInnerV2,
        new_commission_rate: u64,
        ctx: &TxContext,
    ) {
        validator_set::request_set_commission_rate(
            &mut self.validators,
            new_commission_rate,
            ctx
        )
    }

    /// This function is used to set new commission rate for candidate validators
    public(friend) fun set_candidate_validator_commission_rate(
        self: &mut DWalletSystemStateInnerV2,
        new_commission_rate: u64,
        ctx: &TxContext,
    ) {
        let candidate = validator_set::get_validator_mut_with_ctx_including_candidates(&mut self.validators, ctx);
        validator::set_candidate_commission_rate(candidate, new_commission_rate)
    }

    /// Add stake to a validator's staking pool.
    public(friend) fun request_add_stake(
        self: &mut DWalletSystemStateInnerV2,
        stake: Coin<DWLT>,
        validator_address: address,
        ctx: &mut TxContext,
    ) : StakedSui {
        validator_set::request_add_stake(
            &mut self.validators,
            validator_address,
            coin::into_balance(stake),
            ctx,
        )
    }

    /// Add stake to a validator's staking pool using multiple coins.
    public(friend) fun request_add_stake_mul_coin(
        self: &mut DWalletSystemStateInnerV2,
        stakes: vector<Coin<DWLT>>,
        stake_amount: option::Option<u64>,
        validator_address: address,
        ctx: &mut TxContext,
    ) : StakedSui {
        let balance = extract_coin_balance(stakes, stake_amount, ctx);
        validator_set::request_add_stake(&mut self.validators, validator_address, balance, ctx)
    }

    /// Withdraw some portion of a stake from a validator's staking pool.
    public(friend) fun request_withdraw_stake(
        self: &mut DWalletSystemStateInnerV2,
        staked_sui: StakedSui,
        ctx: &TxContext,
    ) : Balance<DWLT> {
        assert!(
            stake_activation_epoch(&staked_sui) <= tx_context::epoch(ctx),
            EStakeWithdrawBeforeActivation
        );
        validator_set::request_withdraw_stake(
            &mut self.validators, staked_sui, ctx,
        )
    }

    /// Report a validator as a bad or non-performant actor in the system.
    /// Succeeds if all the following are satisfied:
    /// 1. both the reporter in `cap` and the input `reportee_addr` are active validators.
    /// 2. reporter and reportee not the same address.
    /// 3. the cap object is still valid.
    /// This function is idempotent.
    public(friend) fun report_validator(
        self: &mut DWalletSystemStateInnerV2,
        cap: &UnverifiedValidatorOperationCap,
        reportee_addr: address,
    ) {
        // Reportee needs to be an active validator
        assert!(validator_set::is_active_validator_by_sui_address(&self.validators, reportee_addr), ENotValidator);
        // Verify the represented reporter address is an active validator, and the capability is still valid.
        let verified_cap = validator_set::verify_cap(&mut self.validators, cap, ACTIVE_VALIDATOR_ONLY);
        report_validator_impl(verified_cap, reportee_addr, &mut self.validator_report_records);
    }


    /// Undo a `report_validator` action. Aborts if
    /// 1. the reportee is not a currently active validator or
    /// 2. the sender has not previously reported the `reportee_addr`, or
    /// 3. the cap is not valid
    public(friend) fun undo_report_validator(
        self: &mut DWalletSystemStateInnerV2,
        cap: &UnverifiedValidatorOperationCap,
        reportee_addr: address,
    ) {
        let verified_cap = validator_set::verify_cap(&mut self.validators, cap, ACTIVE_VALIDATOR_ONLY);
        undo_report_validator_impl(verified_cap, reportee_addr, &mut self.validator_report_records);
    }

    fun report_validator_impl(
        verified_cap: ValidatorOperationCap,
        reportee_addr: address,
        validator_report_records: &mut VecMap<address, VecSet<address>>,
    ) {
        let reporter_address = *validator_cap::verified_operation_cap_address(&verified_cap);
        assert!(reporter_address != reportee_addr, ECannotReportOneself);
        if (!vec_map::contains(validator_report_records, &reportee_addr)) {
            vec_map::insert(validator_report_records, reportee_addr, vec_set::singleton(reporter_address));
        } else {
            let reporters = vec_map::get_mut(validator_report_records, &reportee_addr);
            if (!vec_set::contains(reporters, &reporter_address)) {
                vec_set::insert(reporters, reporter_address);
            }
        }
    }

    fun undo_report_validator_impl(
        verified_cap: ValidatorOperationCap,
        reportee_addr: address,
        validator_report_records: &mut VecMap<address, VecSet<address>>,
    ) {
        assert!(vec_map::contains(validator_report_records, &reportee_addr), EReportRecordNotFound);
        let reporters = vec_map::get_mut(validator_report_records, &reportee_addr);

        let reporter_addr = *validator_cap::verified_operation_cap_address(&verified_cap);
        assert!(vec_set::contains(reporters, &reporter_addr), EReportRecordNotFound);

        vec_set::remove(reporters, &reporter_addr);
        if (vec_set::is_empty(reporters)) {
            vec_map::remove(validator_report_records, &reportee_addr);
        }
    }

    // ==== validator metadata management functions ====

    /// Create a new `UnverifiedValidatorOperationCap`, transfer it to the
    /// validator and registers it. The original object is thus revoked.
    public(friend) fun rotate_operation_cap(
        self: &mut DWalletSystemStateInnerV2,
        ctx: &mut TxContext,
    ) {
        let validator = validator_set::get_validator_mut_with_ctx_including_candidates(&mut self.validators, ctx);
        validator::new_unverified_validator_operation_cap_and_transfer(validator, ctx);
    }

    /// Update a validator's name.
    public(friend) fun update_validator_name(
        self: &mut DWalletSystemStateInnerV2,
        name: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = validator_set::get_validator_mut_with_ctx_including_candidates(&mut self.validators, ctx);

        validator::update_name(validator, name);
    }

    /// Update a validator's description
    public(friend) fun update_validator_description(
        self: &mut DWalletSystemStateInnerV2,
        description: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = validator_set::get_validator_mut_with_ctx_including_candidates(&mut self.validators, ctx);
        validator::update_description(validator, description);
    }

    /// Update a validator's image url
    public(friend) fun update_validator_image_url(
        self: &mut DWalletSystemStateInnerV2,
        image_url: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = validator_set::get_validator_mut_with_ctx_including_candidates(&mut self.validators, ctx);
        validator::update_image_url(validator, image_url);
    }

    /// Update a validator's project url
    public(friend) fun update_validator_project_url(
        self: &mut DWalletSystemStateInnerV2,
        project_url: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = validator_set::get_validator_mut_with_ctx_including_candidates(&mut self.validators, ctx);
        validator::update_project_url(validator, project_url);
    }

    /// Update a validator's network address.
    /// The change will only take effects starting from the next epoch.
    public(friend) fun update_validator_next_epoch_network_address(
        self: &mut DWalletSystemStateInnerV2,
        network_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = validator_set::get_validator_mut_with_ctx(&mut self.validators, ctx);
        validator::update_next_epoch_network_address(validator, network_address);
        let validator :&Validator = validator; // Force immutability for the following call
        validator_set::assert_no_pending_or_active_duplicates(&self.validators, validator);
    }

    /// Update candidate validator's network address.
    public(friend) fun update_candidate_validator_network_address(
        self: &mut DWalletSystemStateInnerV2,
        network_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = validator_set::get_validator_mut_with_ctx_including_candidates(&mut self.validators, ctx);
        validator::update_candidate_network_address(candidate, network_address);
    }

    /// Update a validator's p2p address.
    /// The change will only take effects starting from the next epoch.
    public(friend) fun update_validator_next_epoch_p2p_address(
        self: &mut DWalletSystemStateInnerV2,
        p2p_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = validator_set::get_validator_mut_with_ctx(&mut self.validators, ctx);
        validator::update_next_epoch_p2p_address(validator, p2p_address);
        let validator :&Validator = validator; // Force immutability for the following call
        validator_set::assert_no_pending_or_active_duplicates(&self.validators, validator);
    }

    /// Update candidate validator's p2p address.
    public(friend) fun update_candidate_validator_p2p_address(
        self: &mut DWalletSystemStateInnerV2,
        p2p_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = validator_set::get_validator_mut_with_ctx_including_candidates(&mut self.validators, ctx);
        validator::update_candidate_p2p_address(candidate, p2p_address);
    }

    /// Update a validator's narwhal primary address.
    /// The change will only take effects starting from the next epoch.
    public(friend) fun update_validator_next_epoch_primary_address(
        self: &mut DWalletSystemStateInnerV2,
        primary_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = validator_set::get_validator_mut_with_ctx(&mut self.validators, ctx);
        validator::update_next_epoch_primary_address(validator, primary_address);
    }

    /// Update candidate validator's narwhal primary address.
    public(friend) fun update_candidate_validator_primary_address(
        self: &mut DWalletSystemStateInnerV2,
        primary_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = validator_set::get_validator_mut_with_ctx_including_candidates(&mut self.validators, ctx);
        validator::update_candidate_primary_address(candidate, primary_address);
    }

    /// Update a validator's narwhal worker address.
    /// The change will only take effects starting from the next epoch.
    public(friend) fun update_validator_next_epoch_worker_address(
        self: &mut DWalletSystemStateInnerV2,
        worker_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = validator_set::get_validator_mut_with_ctx(&mut self.validators, ctx);
        validator::update_next_epoch_worker_address(validator, worker_address);
    }

    /// Update candidate validator's narwhal worker address.
    public(friend) fun update_candidate_validator_worker_address(
        self: &mut DWalletSystemStateInnerV2,
        worker_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = validator_set::get_validator_mut_with_ctx_including_candidates(&mut self.validators, ctx);
        validator::update_candidate_worker_address(candidate, worker_address);
    }

    /// Update a validator's public key of protocol key and proof of possession.
    /// The change will only take effects starting from the next epoch.
    public(friend) fun update_validator_next_epoch_protocol_pubkey(
        self: &mut DWalletSystemStateInnerV2,
        protocol_pubkey: vector<u8>,
        proof_of_possession: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = validator_set::get_validator_mut_with_ctx(&mut self.validators, ctx);
        validator::update_next_epoch_protocol_pubkey(validator, protocol_pubkey, proof_of_possession);
        let validator :&Validator = validator; // Force immutability for the following call
        validator_set::assert_no_pending_or_active_duplicates(&self.validators, validator);
    }

    /// Update candidate validator's public key of protocol key and proof of possession.
    public(friend) fun update_candidate_validator_protocol_pubkey(
        self: &mut DWalletSystemStateInnerV2,
        protocol_pubkey: vector<u8>,
        proof_of_possession: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = validator_set::get_validator_mut_with_ctx_including_candidates(&mut self.validators, ctx);
        validator::update_candidate_protocol_pubkey(candidate, protocol_pubkey, proof_of_possession);
    }

    /// Update a validator's public key of worker key.
    /// The change will only take effects starting from the next epoch.
    public(friend) fun update_validator_next_epoch_worker_pubkey(
        self: &mut DWalletSystemStateInnerV2,
        worker_pubkey: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = validator_set::get_validator_mut_with_ctx(&mut self.validators, ctx);
        validator::update_next_epoch_worker_pubkey(validator, worker_pubkey);
        let validator :&Validator = validator; // Force immutability for the following call
        validator_set::assert_no_pending_or_active_duplicates(&self.validators, validator);
    }

    /// Update candidate validator's public key of worker key.
    public(friend) fun update_candidate_validator_worker_pubkey(
        self: &mut DWalletSystemStateInnerV2,
        worker_pubkey: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = validator_set::get_validator_mut_with_ctx_including_candidates(&mut self.validators, ctx);
        validator::update_candidate_worker_pubkey(candidate, worker_pubkey);
    }

    /// Update a validator's public key of network key.
    /// The change will only take effects starting from the next epoch.
    public(friend) fun update_validator_next_epoch_network_pubkey(
        self: &mut DWalletSystemStateInnerV2,
        network_pubkey: vector<u8>,
        ctx: &TxContext,
    ) {
        let validator = validator_set::get_validator_mut_with_ctx(&mut self.validators, ctx);
        validator::update_next_epoch_network_pubkey(validator, network_pubkey);
        let validator :&Validator = validator; // Force immutability for the following call
        validator_set::assert_no_pending_or_active_duplicates(&self.validators, validator);
    }

    /// Update candidate validator's public key of network key.
    public(friend) fun update_candidate_validator_network_pubkey(
        self: &mut DWalletSystemStateInnerV2,
        network_pubkey: vector<u8>,
        ctx: &TxContext,
    ) {
        let candidate = validator_set::get_validator_mut_with_ctx_including_candidates(&mut self.validators, ctx);
        validator::update_candidate_network_pubkey(candidate, network_pubkey);
    }

    /// This function should be called at the end of an epoch, and advances the system to the next epoch.
    /// It does the following things:
    /// 1. Add storage charge to the storage fund.
    /// 2. Burn the storage rebates from the storage fund. These are already refunded to transaction sender's
    ///    gas coins.
    /// 3. Distribute computation charge to validator stake.
    /// 4. Update all validators.
    public(friend) fun advance_epoch(
        self: &mut DWalletSystemStateInnerV2,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_reward: Balance<DWLT>,
        computation_reward: Balance<DWLT>,
        storage_rebate_amount: u64,
        non_refundable_storage_fee_amount: u64,
        storage_fund_reinvest_rate: u64, // share of storage fund's rewards that's reinvested
                                         // into storage fund, in basis point.
        reward_slashing_rate: u64, // how much rewards are slashed to punish a validator, in bps.
        epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
        ctx: &mut TxContext,
    ) : Balance<DWLT> {
        let prev_epoch_start_timestamp = self.epoch_start_timestamp_ms;
        self.epoch_start_timestamp_ms = epoch_start_timestamp_ms;

        let bps_denominator_u64 = (BASIS_POINT_DENOMINATOR as u64);
        // Rates can't be higher than 100%.
        assert!(
            storage_fund_reinvest_rate <= bps_denominator_u64
            && reward_slashing_rate <= bps_denominator_u64,
            EBpsTooLarge,
        );

        // TODO: remove this in later upgrade.
        if (self.parameters.stake_subsidy_start_epoch > 0) {
            self.parameters.stake_subsidy_start_epoch = 20;
        };

        // Accumulate the gas summary during safe_mode before processing any rewards:
        let safe_mode_storage_rewards = balance::withdraw_all(&mut self.safe_mode_storage_rewards);
        balance::join(&mut storage_reward, safe_mode_storage_rewards);
        let safe_mode_computation_rewards = balance::withdraw_all(&mut self.safe_mode_computation_rewards);
        balance::join(&mut computation_reward, safe_mode_computation_rewards);
        storage_rebate_amount = storage_rebate_amount + self.safe_mode_storage_rebates;
        self.safe_mode_storage_rebates = 0;
        non_refundable_storage_fee_amount = non_refundable_storage_fee_amount + self.safe_mode_non_refundable_storage_fee;
        self.safe_mode_non_refundable_storage_fee = 0;

        let total_validators_stake = validator_set::total_stake(&self.validators);
        let storage_fund_balance = storage_fund::total_balance(&self.storage_fund);
        let total_stake = storage_fund_balance + total_validators_stake;

        let storage_charge = balance::value(&storage_reward);
        let computation_charge = balance::value(&computation_reward);

        // Include stake subsidy in the rewards given out to validators and stakers.
        // Delay distributing any stake subsidies until after `stake_subsidy_start_epoch`.
        // And if this epoch is shorter than the regular epoch duration, don't distribute any stake subsidy.
        let stake_subsidy =
            if (tx_context::epoch(ctx) >= self.parameters.stake_subsidy_start_epoch  &&
                epoch_start_timestamp_ms >= prev_epoch_start_timestamp + self.parameters.epoch_duration_ms)
            {
                stake_subsidy::advance_epoch(&mut self.stake_subsidy)
            } else {
                balance::zero()
            };

        let stake_subsidy_amount = balance::value(&stake_subsidy);
        balance::join(&mut computation_reward, stake_subsidy);

        let total_stake_u128 = (total_stake as u128);
        let computation_charge_u128 = (computation_charge as u128);

        let storage_fund_reward_amount = (storage_fund_balance as u128) * computation_charge_u128 / total_stake_u128;
        let storage_fund_reward = balance::split(&mut computation_reward, (storage_fund_reward_amount as u64));
        let storage_fund_reinvestment_amount =
            storage_fund_reward_amount * (storage_fund_reinvest_rate as u128) / BASIS_POINT_DENOMINATOR;
        let storage_fund_reinvestment = balance::split(
            &mut storage_fund_reward,
            (storage_fund_reinvestment_amount as u64),
        );

        self.epoch = self.epoch + 1;
        // Sanity check to make sure we are advancing to the right epoch.
        assert!(new_epoch == self.epoch, EAdvancedToWrongEpoch);

        let computation_reward_amount_before_distribution = balance::value(&computation_reward);
        let storage_fund_reward_amount_before_distribution = balance::value(&storage_fund_reward);

        validator_set::advance_epoch(
            &mut self.validators,
            &mut computation_reward,
            &mut storage_fund_reward,
            &mut self.validator_report_records,
            reward_slashing_rate,
            self.parameters.validator_low_stake_threshold,
            self.parameters.validator_very_low_stake_threshold,
            self.parameters.validator_low_stake_grace_period,
            ctx,
        );

        let new_total_stake = validator_set::total_stake(&self.validators);

        let computation_reward_amount_after_distribution = balance::value(&computation_reward);
        let storage_fund_reward_amount_after_distribution = balance::value(&storage_fund_reward);
        let computation_reward_distributed = computation_reward_amount_before_distribution - computation_reward_amount_after_distribution;
        let storage_fund_reward_distributed = storage_fund_reward_amount_before_distribution - storage_fund_reward_amount_after_distribution;

        self.protocol_version = next_protocol_version;

        // Derive the reference gas price for the new epoch
        self.reference_gas_price = validator_set::derive_reference_gas_price(&self.validators);
        // Because of precision issues with integer divisions, we expect that there will be some
        // remaining balance in `storage_fund_reward` and `computation_reward`.
        // All of these go to the storage fund.
        let leftover_staking_rewards = storage_fund_reward;
        balance::join(&mut leftover_staking_rewards, computation_reward);
        let leftover_storage_fund_inflow = balance::value(&leftover_staking_rewards);

        let refunded_storage_rebate =
            storage_fund::advance_epoch(
                &mut self.storage_fund,
                storage_reward,
                storage_fund_reinvestment,
                leftover_staking_rewards,
                storage_rebate_amount,
                non_refundable_storage_fee_amount,
            );

        event::emit(
            SystemEpochInfoEvent {
                epoch: self.epoch,
                protocol_version: self.protocol_version,
                reference_gas_price: self.reference_gas_price,
                total_stake: new_total_stake,
                storage_charge,
                storage_fund_reinvestment: (storage_fund_reinvestment_amount as u64),
                storage_rebate: storage_rebate_amount,
                storage_fund_balance: storage_fund::total_balance(&self.storage_fund),
                stake_subsidy_amount,
                total_gas_fees: computation_charge,
                total_stake_rewards_distributed: computation_reward_distributed + storage_fund_reward_distributed,
                leftover_storage_fund_inflow,
            }
        );
        self.safe_mode = false;
        // Double check that the gas from safe mode has been processed.
        assert!(self.safe_mode_storage_rebates == 0
            && balance::value(&self.safe_mode_storage_rewards) == 0
            && balance::value(&self.safe_mode_computation_rewards) == 0, ESafeModeGasNotProcessed);

        // Return the storage rebate split from storage fund that's already refunded to the transaction senders.
        // This will be burnt at the last step of epoch change programmable transaction.
        refunded_storage_rebate
    }

    /// Return the current epoch number. Useful for applications that need a coarse-grained concept of time,
    /// since epochs are ever-increasing and epoch changes are intended to happen every 24 hours.
    public(friend) fun epoch(self: &DWalletSystemStateInnerV2): u64 {
        self.epoch
    }

    public(friend) fun protocol_version(self: &DWalletSystemStateInnerV2): u64 {
        self.protocol_version
    }

    public(friend) fun system_state_version(self: &DWalletSystemStateInnerV2): u64 {
        self.system_state_version
    }

    /// This function always return the genesis system state version, which is used to create the system state in genesis.
    /// It should never change for a given network.
    public(friend) fun genesis_system_state_version(): u64 {
        SYSTEM_STATE_VERSION_V1
    }

    /// Returns unix timestamp of the start of current epoch
    public(friend) fun epoch_start_timestamp_ms(self: &DWalletSystemStateInnerV2): u64 {
        self.epoch_start_timestamp_ms
    }

    /// Returns the total amount staked with `validator_addr`.
    /// Aborts if `validator_addr` is not an active validator.
    public(friend) fun validator_stake_amount(self: &DWalletSystemStateInnerV2, validator_addr: address): u64 {
        validator_set::validator_total_stake_amount(&self.validators, validator_addr)
    }

    /// Returns the staking pool id of a given validator.
    /// Aborts if `validator_addr` is not an active validator.
    public(friend) fun validator_staking_pool_id(self: &DWalletSystemStateInnerV2, validator_addr: address): ID {

        validator_set::validator_staking_pool_id(&self.validators, validator_addr)
    }

    /// Returns reference to the staking pool mappings that map pool ids to active validator addresses
    public(friend) fun validator_staking_pool_mappings(self: &DWalletSystemStateInnerV2): &Table<ID, address> {

        validator_set::staking_pool_mappings(&self.validators)
    }

    /// Returns all the validators who are currently reporting `addr`
    public(friend) fun get_reporters_of(self: &DWalletSystemStateInnerV2, addr: address): VecSet<address> {

        if (vec_map::contains(&self.validator_report_records, &addr)) {
            *vec_map::get(&self.validator_report_records, &addr)
        } else {
            vec_set::empty()
        }
    }

    public(friend) fun get_storage_fund_total_balance(self: &DWalletSystemStateInnerV2): u64 {
        storage_fund::total_balance(&self.storage_fund)
    }

    public(friend) fun get_storage_fund_object_rebates(self: &DWalletSystemStateInnerV2): u64 {
        storage_fund::total_object_storage_rebates(&self.storage_fund)
    }

    public(friend) fun pool_exchange_rates(
        self: &mut DWalletSystemStateInnerV2,
        pool_id: &ID
    ): &Table<u64, PoolTokenExchangeRate>  {
        let validators = &mut self.validators;
        validator_set::pool_exchange_rates(validators, pool_id)
    }

    public(friend) fun active_validator_addresses(self: &DWalletSystemStateInnerV2): vector<address> {
        let validator_set = &self.validators;
        validator_set::active_validator_addresses(validator_set)
    }

    #[lint_allow(self_transfer)]
    /// Extract required Balance from vector of Coin<SUI>, transfer the remainder back to sender.
    fun extract_coin_balance(coins: vector<Coin<DWLT>>, amount: option::Option<u64>, ctx: &mut TxContext): Balance<DWLT> {
        let merged_coin = vector::pop_back(&mut coins);
        pay::join_vec(&mut merged_coin, coins);

        let total_balance = coin::into_balance(merged_coin);
        // return the full amount if amount is not specified
        if (option::is_some(&amount)) {
            let amount = option::destroy_some(amount);
            let balance = balance::split(&mut total_balance, amount);
            // transfer back the remainder if non zero.
            if (balance::value(&total_balance) > 0) {
                transfer::public_transfer(coin::from_balance(total_balance, ctx), tx_context::sender(ctx));
            } else {
                balance::destroy_zero(total_balance);
            };
            balance
        } else {
            total_balance
        }
    }

    #[test_only]
    /// Return the current validator set
    public(friend) fun validators(self: &DWalletSystemStateInnerV2): &ValidatorSet {
        &self.validators
    }

    #[test_only]
    /// Return the currently active validator by address
    public(friend) fun active_validator_by_address(self: &DWalletSystemStateInnerV2, validator_address: address): &Validator {
        validator_set::get_active_validator_ref(validators(self), validator_address)
    }

    #[test_only]
    /// Return the currently pending validator by address
    public(friend) fun pending_validator_by_address(self: &DWalletSystemStateInnerV2, validator_address: address): &Validator {
        validator_set::get_pending_validator_ref(validators(self), validator_address)
    }

    #[test_only]
    /// Return the currently candidate validator by address
    public(friend) fun candidate_validator_by_address(self: &DWalletSystemStateInnerV2, validator_address: address): &Validator {
        validator_set::get_candidate_validator_ref(validators(self), validator_address)
    }

    #[test_only]
    public(friend) fun get_stake_subsidy_distribution_counter(self: &DWalletSystemStateInnerV2): u64 {
        stake_subsidy::get_distribution_counter(&self.stake_subsidy)
    }

    #[test_only]
    public(friend) fun set_epoch_for_testing(self: &mut DWalletSystemStateInnerV2, epoch_num: u64) {
        self.epoch = epoch_num
    }

    #[test_only]
    public(friend) fun request_add_validator_for_testing(
        self: &mut DWalletSystemStateInnerV2,
        min_joining_stake_for_testing: u64,
        ctx: &TxContext,
    ) {
        assert!(
            validator_set::next_epoch_validator_count(&self.validators) < self.parameters.max_validator_count,
            ELimitExceeded,
        );

        validator_set::request_add_validator(&mut self.validators, min_joining_stake_for_testing, ctx);
    }

    // CAUTION: THIS CODE IS ONLY FOR TESTING AND THIS MACRO MUST NEVER EVER BE REMOVED.  Creates a
    // candidate validator - bypassing the proof of possession check and other metadata validation
    // in the process.
    #[test_only]
    public(friend) fun request_add_validator_candidate_for_testing(
        self: &mut DWalletSystemStateInnerV2,
        pubkey_bytes: vector<u8>,
        network_pubkey_bytes: vector<u8>,
        worker_pubkey_bytes: vector<u8>,
        proof_of_possession: vector<u8>,
        name: vector<u8>,
        description: vector<u8>,
        image_url: vector<u8>,
        project_url: vector<u8>,
        net_address: vector<u8>,
        p2p_address: vector<u8>,
        primary_address: vector<u8>,
        worker_address: vector<u8>,
        gas_price: u64,
        commission_rate: u64,
        ctx: &mut TxContext,
    ) {
        let validator = validator::new_for_testing(
            tx_context::sender(ctx),
            pubkey_bytes,
            network_pubkey_bytes,
            worker_pubkey_bytes,
            proof_of_possession,
            name,
            description,
            image_url,
            project_url,
            net_address,
            p2p_address,
            primary_address,
            worker_address,
            option::none(),
            gas_price,
            commission_rate,
            false, // not an initial validator active at genesis
            ctx
        );

        validator_set::request_add_validator_candidate(&mut self.validators, validator, ctx);
    }

}
