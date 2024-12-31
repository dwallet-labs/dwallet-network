// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// Pera System State Type Upgrade Guide
/// `PeraSystemState` is a thin wrapper around `PeraSystemStateInner` that provides a versioned interface.
/// The `PeraSystemState` object has a fixed ID 0x5, and the `PeraSystemStateInner` object is stored as a dynamic field.
/// There are a few different ways to upgrade the `PeraSystemStateInner` type:
///
/// The simplest and one that doesn't involve a real upgrade is to just add dynamic fields to the `extra_fields` field
/// of `PeraSystemStateInner` or any of its sub type. This is useful when we are in a rush, or making a small change,
/// or still experimenting a new field.
///
/// To properly upgrade the `PeraSystemStateInner` type, we need to ship a new framework that does the following:
/// 1. Define a new `PeraSystemStateInner`type (e.g. `PeraSystemStateInnerV2`).
/// 2. Define a data migration function that migrates the old `PeraSystemStateInner` to the new one (i.e. PeraSystemStateInnerV2).
/// 3. Replace all uses of `PeraSystemStateInner` with `PeraSystemStateInnerV2` in both pera_system.move and pera_system_state_inner.move,
///    with the exception of the `pera_system_state_inner::create` function, which should always return the genesis type.
/// 4. Inside `load_inner_maybe_upgrade` function, check the current version in the wrapper, and if it's not the latest version,
///   call the data migration function to upgrade the inner object. Make sure to also update the version in the wrapper.
/// A detailed example can be found in pera/tests/framework_upgrades/mock_pera_systems/shallow_upgrade.
/// Along with the Move change, we also need to update the Rust code to support the new type. This includes:
/// 1. Define a new `PeraSystemStateInner` struct type that matches the new Move type, and implement the PeraSystemStateTrait.
/// 2. Update the `PeraSystemState` struct to include the new version as a new enum variant.
/// 3. Update the `get_pera_system_state` function to handle the new version.
/// To test that the upgrade will be successful, we need to modify `pera_system_state_production_upgrade_test` test in
/// protocol_version_tests and trigger a real upgrade using the new framework. We will need to keep this directory as old version,
/// put the new framework in a new directory, and run the test to exercise the upgrade.
///
/// To upgrade Validator type, besides everything above, we also need to:
/// 1. Define a new Validator type (e.g. ValidatorV2).
/// 2. Define a data migration function that migrates the old Validator to the new one (i.e. ValidatorV2).
/// 3. Replace all uses of Validator with ValidatorV2 except the genesis creation function.
/// 4. In validator_wrapper::upgrade_to_latest, check the current version in the wrapper, and if it's not the latest version,
///  call the data migration function to upgrade it.
/// In Rust, we also need to add a new case in `get_validator_from_table`.
/// Note that it is possible to upgrade PeraSystemStateInner without upgrading Validator, but not the other way around.
/// And when we only upgrade PeraSystemStateInner, the version of Validator in the wrapper will not be updated, and hence may become
/// inconsistent with the version of PeraSystemStateInner. This is fine as long as we don't use the Validator version to determine
/// the PeraSystemStateInner version, or vice versa.

module pera_system::pera_system {
    use pera::balance::Balance;

    use pera::coin::Coin;
    use pera_system::staking_pool::StakedPera;
    use pera::pera::PERA;
    use pera::table::Table;
    use pera_system::validator::Validator;
    use pera_system::validator_cap::UnverifiedValidatorOperationCap;
    use pera_system::pera_system_state_inner::{Self, SystemParameters, PeraSystemStateInner, PeraSystemStateInnerV2};
    use pera_system::stake_subsidy::StakeSubsidy;
    use pera_system::staking_pool::PoolTokenExchangeRate;
    use pera::dynamic_field;
    use pera::vec_map::VecMap;
    use pera_system::validator_set::get_active_validators_data;
    use pera_system::dwallet_network_key::{is_valid_key_scheme, start_network_dkg};

    #[test_only] use pera::balance;
    #[test_only] use pera_system::validator_set::ValidatorSet;
    #[test_only] use pera::vec_set::VecSet;

    public struct PeraSystemState has key {
        id: UID,
        version: u64,
    }

    const ENotSystemAddress: u64 = 0;
    const EWrongInnerVersion: u64 = 1;
    #[error]
    const EInvalidKeyType: u8 = 2;
    const ENotDwalletAdminAddress: u64 = 3;

    // ==== functions that can only be called by genesis ====

    /// Create a new PeraSystemState object and make it shared.
    /// This function will be called only once in genesis.
    public(package) fun create(
        id: UID,
        validators: vector<Validator>,
        storage_fund: Balance<PERA>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        parameters: SystemParameters,
        stake_subsidy: StakeSubsidy,
        dwallet_admin_address: address,
        ctx: &mut TxContext,
    ) {
        let system_state = pera_system_state_inner::create(
            validators,
            storage_fund,
            protocol_version,
            epoch_start_timestamp_ms,
            parameters,
            stake_subsidy,
            dwallet_admin_address,
            ctx,
        );
        let version = pera_system_state_inner::genesis_system_state_version();
        let mut self = PeraSystemState {
            id,
            version,
        };
        dynamic_field::add(&mut self.id, version, system_state);
        transfer::share_object(self);
    }

    public(package) fun dwallet_admin_address(
        wrapper: &mut PeraSystemState,
    ): address  {
        let self = load_system_state_mut(wrapper);
        self.dwallet_admin_address()
    }

    // ==== entry functions ====

    /// Function to create start the network DKG for the dwallet mpc network key of a given key type.
    public entry fun request_start_network_dkg(key_type: u8, system_state: &mut PeraSystemState, ctx: &mut TxContext) {
        assert!(ctx.sender() == system_state.dwallet_admin_address(), ENotDwalletAdminAddress);
        let self = system_state.load_system_state_mut();
        let validators = self.active_validators();
        let validators_data = get_active_validators_data(validators);
        start_network_dkg(key_type, validators_data, ctx);
    }

    /// Can be called by anyone who wishes to become a validator candidate and starts accuring delegated
    /// stakes in their staking pool. Once they have at least `MIN_VALIDATOR_JOINING_STAKE` amount of stake they
    /// can call `request_add_validator` to officially become an active validator at the next epoch.
    /// Aborts if the caller is already a pending or active validator, or a validator candidate.
    /// Note: `proof_of_possession` MUST be a valid signature using pera_address and protocol_pubkey_bytes.
    /// To produce a valid PoP, run [fn test_proof_of_possession].
    public entry fun request_add_validator_candidate(
        wrapper: &mut PeraSystemState,
        pubkey_bytes: vector<u8>,
        network_pubkey_bytes: vector<u8>,
        worker_pubkey_bytes: vector<u8>,
        cg_pubkey_and_proof: vector<u8>,
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
        let self = load_system_state_mut(wrapper);
        self.request_add_validator_candidate(
            pubkey_bytes,
            network_pubkey_bytes,
            worker_pubkey_bytes,
            cg_pubkey_and_proof,
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
            ctx,
        )
    }

    /// Called by a validator candidate to remove themselves from the candidacy. After this call
    /// their staking pool becomes deactivate.
    public entry fun request_remove_validator_candidate(
        wrapper: &mut PeraSystemState,
        ctx: &mut TxContext,
    ) {
        let self = load_system_state_mut(wrapper);
        self.request_remove_validator_candidate(ctx)
    }

    /// Called by a validator candidate to add themselves to the active validator set beginning next epoch.
    /// Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
    /// stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
    /// epoch has already reached the maximum.
    public entry fun request_add_validator(
        wrapper: &mut PeraSystemState,
        ctx: &mut TxContext,
    ) {
        let self = load_system_state_mut(wrapper);
        self.request_add_validator(ctx)
    }

    /// A validator can call this function to request a removal in the next epoch.
    /// We use the sender of `ctx` to look up the validator
    /// (i.e. sender must match the pera_address in the validator).
    /// At the end of the epoch, the `validator` object will be returned to the pera_address
    /// of the validator.
    public entry fun request_remove_validator(
        wrapper: &mut PeraSystemState,
        ctx: &mut TxContext,
    ) {
        let self = load_system_state_mut(wrapper);
        self.request_remove_validator(ctx)
    }

    /// A validator can call this entry function to submit a new gas price quote, to be
    /// used for the reference gas price calculation at the end of the epoch.
    public entry fun request_set_gas_price(
        wrapper: &mut PeraSystemState,
        cap: &UnverifiedValidatorOperationCap,
        new_gas_price: u64,
    ) {
        let self = load_system_state_mut(wrapper);
        self.request_set_gas_price(cap, new_gas_price)
    }

    /// This entry function is used to set new gas price for candidate validators
    public entry fun set_candidate_validator_gas_price(
        wrapper: &mut PeraSystemState,
        cap: &UnverifiedValidatorOperationCap,
        new_gas_price: u64,
    ) {
        let self = load_system_state_mut(wrapper);
        self.set_candidate_validator_gas_price(cap, new_gas_price)
    }

    /// A validator can call this entry function to set a new commission rate, updated at the end of
    /// the epoch.
    public entry fun request_set_commission_rate(
        wrapper: &mut PeraSystemState,
        new_commission_rate: u64,
        ctx: &mut TxContext,
    ) {
        let self = load_system_state_mut(wrapper);
        self.request_set_commission_rate(new_commission_rate, ctx)
    }

    /// This entry function is used to set new commission rate for candidate validators
    public entry fun set_candidate_validator_commission_rate(
        wrapper: &mut PeraSystemState,
        new_commission_rate: u64,
        ctx: &mut TxContext,
    ) {
        let self = load_system_state_mut(wrapper);
        self.set_candidate_validator_commission_rate(new_commission_rate, ctx)
    }

    /// Add stake to a validator's staking pool.
    public entry fun request_add_stake(
        wrapper: &mut PeraSystemState,
        stake: Coin<PERA>,
        validator_address: address,
        ctx: &mut TxContext,
    ) {
        let staked_pera = request_add_stake_non_entry(wrapper, stake, validator_address, ctx);
        transfer::public_transfer(staked_pera, ctx.sender());
    }

    /// The non-entry version of `request_add_stake`, which returns the staked PERA instead of transferring it to the sender.
    public fun request_add_stake_non_entry(
        wrapper: &mut PeraSystemState,
        stake: Coin<PERA>,
        validator_address: address,
        ctx: &mut TxContext,
    ): StakedPera {
        let self = load_system_state_mut(wrapper);
        self.request_add_stake(stake, validator_address, ctx)
    }

    /// Add stake to a validator's staking pool using multiple coins.
    public entry fun request_add_stake_mul_coin(
        wrapper: &mut PeraSystemState,
        stakes: vector<Coin<PERA>>,
        stake_amount: option::Option<u64>,
        validator_address: address,
        ctx: &mut TxContext,
    ) {
        let self = load_system_state_mut(wrapper);
        let staked_pera = self.request_add_stake_mul_coin(stakes, stake_amount, validator_address, ctx);
        transfer::public_transfer(staked_pera, ctx.sender());
    }

    /// Withdraw stake from a validator's staking pool.
    public entry fun request_withdraw_stake(
        wrapper: &mut PeraSystemState,
        staked_pera: StakedPera,
        ctx: &mut TxContext,
    ) {
        let withdrawn_stake = request_withdraw_stake_non_entry(wrapper, staked_pera, ctx);
        transfer::public_transfer(withdrawn_stake.into_coin(ctx), ctx.sender());
    }

    /// Non-entry version of `request_withdraw_stake` that returns the withdrawn PERA instead of transferring it to the sender.
    public fun request_withdraw_stake_non_entry(
        wrapper: &mut PeraSystemState,
        staked_pera: StakedPera,
        ctx: &mut TxContext,
    ): Balance<PERA> {
        let self = load_system_state_mut(wrapper);
        self.request_withdraw_stake(staked_pera, ctx)
    }

    /// Report a validator as a bad or non-performant actor in the system.
    /// Succeeds if all the following are satisfied:
    /// 1. both the reporter in `cap` and the input `reportee_addr` are active validators.
    /// 2. reporter and reportee not the same address.
    /// 3. the cap object is still valid.
    /// This function is idempotent.
    public entry fun report_validator(
        wrapper: &mut PeraSystemState,
        cap: &UnverifiedValidatorOperationCap,
        reportee_addr: address,
    ) {
        let self = load_system_state_mut(wrapper);
        self.report_validator(cap, reportee_addr)
    }


    /// Undo a `report_validator` action. Aborts if
    /// 1. the reportee is not a currently active validator or
    /// 2. the sender has not previously reported the `reportee_addr`, or
    /// 3. the cap is not valid
    public entry fun undo_report_validator(
        wrapper: &mut PeraSystemState,
        cap: &UnverifiedValidatorOperationCap,
        reportee_addr: address,
    ) {
        let self = load_system_state_mut(wrapper);
        self.undo_report_validator(cap, reportee_addr)
    }

    // ==== validator metadata management functions ====

    /// Create a new `UnverifiedValidatorOperationCap`, transfer it to the
    /// validator and registers it. The original object is thus revoked.
    public entry fun rotate_operation_cap(
        self: &mut PeraSystemState,
        ctx: &mut TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.rotate_operation_cap(ctx)
    }

    /// Update a validator's name.
    public entry fun update_validator_name(
        self: &mut PeraSystemState,
        name: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_validator_name(name, ctx)
    }

    /// Update a validator's description
    public entry fun update_validator_description(
        self: &mut PeraSystemState,
        description: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_validator_description(description, ctx)
    }

    /// Update a validator's image url
    public entry fun update_validator_image_url(
        self: &mut PeraSystemState,
        image_url: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_validator_image_url(image_url, ctx)
    }

    /// Update a validator's project url
    public entry fun update_validator_project_url(
        self: &mut PeraSystemState,
        project_url: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_validator_project_url(project_url, ctx)
    }

    /// Update a validator's network address.
    /// The change will only take effects starting from the next epoch.
    public entry fun update_validator_next_epoch_network_address(
        self: &mut PeraSystemState,
        network_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_validator_next_epoch_network_address(network_address, ctx)
    }

    /// Update candidate validator's network address.
    public entry fun update_candidate_validator_network_address(
        self: &mut PeraSystemState,
        network_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_candidate_validator_network_address(network_address, ctx)
    }

    /// Update a validator's p2p address.
    /// The change will only take effects starting from the next epoch.
    public entry fun update_validator_next_epoch_p2p_address(
        self: &mut PeraSystemState,
        p2p_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_validator_next_epoch_p2p_address(p2p_address, ctx)
    }

    /// Update candidate validator's p2p address.
    public entry fun update_candidate_validator_p2p_address(
        self: &mut PeraSystemState,
        p2p_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_candidate_validator_p2p_address(p2p_address, ctx)
    }

    /// Update a validator's narwhal primary address.
    /// The change will only take effects starting from the next epoch.
    public entry fun update_validator_next_epoch_primary_address(
        self: &mut PeraSystemState,
        primary_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_validator_next_epoch_primary_address(primary_address, ctx)
    }

    /// Update candidate validator's narwhal primary address.
    public entry fun update_candidate_validator_primary_address(
        self: &mut PeraSystemState,
        primary_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_candidate_validator_primary_address(primary_address, ctx)
    }

    /// Update a validator's narwhal worker address.
    /// The change will only take effects starting from the next epoch.
    public entry fun update_validator_next_epoch_worker_address(
        self: &mut PeraSystemState,
        worker_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_validator_next_epoch_worker_address(worker_address, ctx)
    }

    /// Update candidate validator's narwhal worker address.
    public entry fun update_candidate_validator_worker_address(
        self: &mut PeraSystemState,
        worker_address: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_candidate_validator_worker_address(worker_address, ctx)
    }

    /// Update a validator's public key of protocol key and proof of possession.
    /// The change will only take effects starting from the next epoch.
    public entry fun update_validator_next_epoch_protocol_pubkey(
        self: &mut PeraSystemState,
        protocol_pubkey: vector<u8>,
        proof_of_possession: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_validator_next_epoch_protocol_pubkey(protocol_pubkey, proof_of_possession, ctx)
    }

    /// Update candidate validator's public key of protocol key and proof of possession.
    public entry fun update_candidate_validator_protocol_pubkey(
        self: &mut PeraSystemState,
        protocol_pubkey: vector<u8>,
        proof_of_possession: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_candidate_validator_protocol_pubkey(protocol_pubkey, proof_of_possession, ctx)
    }

    /// Update a validator's public key of worker key.
    /// The change will only take effects starting from the next epoch.
    public entry fun update_validator_next_epoch_worker_pubkey(
        self: &mut PeraSystemState,
        worker_pubkey: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_validator_next_epoch_worker_pubkey(worker_pubkey, ctx)
    }

    /// Update candidate validator's public key of worker key.
    public entry fun update_candidate_validator_worker_pubkey(
        self: &mut PeraSystemState,
        worker_pubkey: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_candidate_validator_worker_pubkey(worker_pubkey, ctx)
    }

    /// Update a validator's public key of network key.
    /// The change will only take effects starting from the next epoch.
    public entry fun update_validator_next_epoch_network_pubkey(
        self: &mut PeraSystemState,
        network_pubkey: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_validator_next_epoch_network_pubkey(network_pubkey, ctx)
    }

    /// Update candidate validator's public key of network key.
    public entry fun update_candidate_validator_network_pubkey(
        self: &mut PeraSystemState,
        network_pubkey: vector<u8>,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(self);
        self.update_candidate_validator_network_pubkey(network_pubkey, ctx)
    }

    /// Getter of the pool token exchange rate of a staking pool. Works for both active and inactive pools.
    public fun pool_exchange_rates(
        wrapper: &mut PeraSystemState,
        pool_id: &ID
    ): &Table<u64, PoolTokenExchangeRate> {
        let self = load_system_state_mut(wrapper);
        self.pool_exchange_rates(pool_id)
    }

    /// Getter returning addresses of the currently active validators.
    public fun active_validator_addresses(wrapper: &mut PeraSystemState): vector<address> {
        let self = load_system_state(wrapper);
        self.active_validator_addresses()
    }

    /// Getter
    public fun decryption_key_shares(wrapper: &mut PeraSystemState, key_type: u8, key_version: u64): vector<u8> {
        let self = load_system_state(wrapper);
        self.decryption_key_shares(key_type, key_version)
    }

    #[allow(unused_function)]
    /// This function should be called at the end of an epoch, and advances the system to the next epoch.
    /// It does the following things:
    /// 1. Add storage charge to the storage fund.
    /// 2. Burn the storage rebates from the storage fund. These are already refunded to transaction sender's
    ///    gas coins.
    /// 3. Distribute computation charge to validator stake.
    /// 4. Update all validators.
    fun advance_epoch(
        storage_reward: Balance<PERA>,
        computation_reward: Balance<PERA>,
        wrapper: &mut PeraSystemState,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_rebate: u64,
        non_refundable_storage_fee: u64,
        storage_fund_reinvest_rate: u64, // share of storage fund's rewards that's reinvested
        // into storage fund, in basis point.
        reward_slashing_rate: u64, // how much rewards are slashed to punish a validator, in bps.
        epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
        ctx: &mut TxContext,
    ): Balance<PERA> {
        let self = load_system_state_mut(wrapper);
        // Validator will make a special system call with sender set as 0x0.
        assert!(ctx.sender() == @0x0, ENotSystemAddress);
        let storage_rebate = self.advance_epoch(
            new_epoch,
            next_protocol_version,
            storage_reward,
            computation_reward,
            storage_rebate,
            non_refundable_storage_fee,
            storage_fund_reinvest_rate,
            reward_slashing_rate,
            epoch_start_timestamp_ms,
            ctx,
        );

        storage_rebate
    }

    fun load_system_state(self: &mut PeraSystemState): &PeraSystemStateInnerV2 {
        load_inner_maybe_upgrade(self)
    }

    fun load_system_state_mut(self: &mut PeraSystemState): &mut PeraSystemStateInnerV2 {
        load_inner_maybe_upgrade(self)
    }

    fun load_inner_maybe_upgrade(self: &mut PeraSystemState): &mut PeraSystemStateInnerV2 {
        if (self.version == 1) {
            let v1: PeraSystemStateInner = dynamic_field::remove(&mut self.id, self.version);
            let v2 = v1.v1_to_v2();
            self.version = 2;
            dynamic_field::add(&mut self.id, self.version, v2);
        };

        let inner: &mut PeraSystemStateInnerV2 = dynamic_field::borrow_mut(
            &mut self.id,
            self.version
        );
        assert!(inner.system_state_version() == self.version, EWrongInnerVersion);
        inner
    }

    #[allow(unused_function)]
    /// Returns the voting power of the active validators, values are voting power in the scale of 10000.
    fun validator_voting_powers(wrapper: &mut PeraSystemState): VecMap<address, u64> {
        let self = load_system_state(wrapper);
        pera_system_state_inner::active_validator_voting_powers(self)
    }

    #[allow(unused_function)]
    /// Lock the next epoch's validator set
    /// The chain agrees on the next epoch committee in order to pass
    /// the chain's DWallet MPC secret to it.
    fun lock_next_epoch_committee(wrapper: &mut PeraSystemState, ctx: &TxContext) {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);
        let self = load_system_state_mut(wrapper);
        self.lock_next_epoch_committee();
    }

    #[allow(unused_function)]
    /// Store the encrypted decryption key shares from the network DKG re-sharing.
    /// The chain agrees on on the same public output.
    fun store_decryption_key_shares(
        wrapper: &mut PeraSystemState,
        shares: vector<vector<u8>>,
        key_scheme: u8,
        ctx: &TxContext
    ) {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);
        assert!(is_valid_key_scheme(key_scheme), EInvalidKeyType);
        let self = load_system_state_mut(wrapper);
        self.store_decryption_key_shares(shares, key_scheme);
    }

    #[allow(unused_function)]
    /// Store the encrypted decryption key shares from the network DKG protocol public output.
    /// The chain agrees on on the same public output.
    fun new_decryption_key_shares_version(
        wrapper: &mut PeraSystemState,
        shares: vector<vector<u8>>,
        protocol_public_parameters: vector<u8>,
        decryption_public_parameters: vector<u8>,
        encryption_key: vector<u8>,
        reconstructed_commitments_to_sharing: vector<u8>,
        key_scheme: u8,
        ctx: &TxContext
    ) {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);
        assert!(is_valid_key_scheme(key_scheme), EInvalidKeyType);
        let self = load_system_state_mut(wrapper);
        self.new_decryption_key_shares_version(
            shares,
            protocol_public_parameters,
            decryption_public_parameters,
            encryption_key,
            reconstructed_commitments_to_sharing,
            key_scheme,
        );
    }

    #[test_only]
    public fun validator_voting_powers_for_testing(wrapper: &mut PeraSystemState): VecMap<address, u64> {
        validator_voting_powers(wrapper)
    }

    #[test_only]
    /// Return the current epoch number. Useful for applications that need a coarse-grained concept of time,
    /// since epochs are ever-increasing and epoch changes are intended to happen every 24 hours.
    public fun epoch(wrapper: &mut PeraSystemState): u64 {
        let self = load_system_state(wrapper);
        self.epoch()
    }

    #[test_only]
    /// Returns unix timestamp of the start of current epoch
    public fun epoch_start_timestamp_ms(wrapper: &mut PeraSystemState): u64 {
        let self = load_system_state(wrapper);
        self.epoch_start_timestamp_ms()
    }

    #[test_only]
    /// Returns the total amount staked with `validator_addr`.
    /// Aborts if `validator_addr` is not an active validator.
    public fun validator_stake_amount(wrapper: &mut PeraSystemState, validator_addr: address): u64 {
        let self = load_system_state(wrapper);
        self.validator_stake_amount(validator_addr)
    }

    #[test_only]
    /// Returns the staking pool id of a given validator.
    /// Aborts if `validator_addr` is not an active validator.
    public fun validator_staking_pool_id(wrapper: &mut PeraSystemState, validator_addr: address): ID {
        let self = load_system_state(wrapper);
        self.validator_staking_pool_id(validator_addr)
    }

    #[test_only]
    /// Returns reference to the staking pool mappings that map pool ids to active validator addresses
    public fun validator_staking_pool_mappings(wrapper: &mut PeraSystemState): &Table<ID, address> {
        let self = load_system_state(wrapper);
        self.validator_staking_pool_mappings()
    }

    #[test_only]
    /// Returns all the validators who are currently reporting `addr`
    public fun get_reporters_of(wrapper: &mut PeraSystemState, addr: address): VecSet<address> {
        let self = load_system_state(wrapper);
        self.get_reporters_of(addr)
    }

    #[test_only]
    /// Return the current validator set
    public fun validators(wrapper: &mut PeraSystemState): &ValidatorSet {
        let self = load_system_state(wrapper);
        self.validators()
    }

    #[test_only]
    /// Return the currently active validator by address
    public fun active_validator_by_address(self: &mut PeraSystemState, validator_address: address): &Validator {
        validators(self).get_active_validator_ref(validator_address)
    }

    #[test_only]
    /// Return the currently pending validator by address
    public fun pending_validator_by_address(self: &mut PeraSystemState, validator_address: address): &Validator {
        validators(self).get_pending_validator_ref(validator_address)
    }

    #[test_only]
    /// Return the currently candidate validator by address
    public fun candidate_validator_by_address(self: &mut PeraSystemState, validator_address: address): &Validator {
        validators(self).get_candidate_validator_ref(validator_address)
    }

    #[test_only]
    public fun set_epoch_for_testing(wrapper: &mut PeraSystemState, epoch_num: u64) {
        let self = load_system_state_mut(wrapper);
        self.set_epoch_for_testing(epoch_num)
    }

    #[test_only]
    public fun request_add_validator_for_testing(
        wrapper: &mut PeraSystemState,
        min_joining_stake_for_testing: u64,
        ctx: &TxContext,
    ) {
        let self = load_system_state_mut(wrapper);
        self.request_add_validator_for_testing(min_joining_stake_for_testing, ctx)
    }

    #[test_only]
    public fun get_storage_fund_total_balance(wrapper: &mut PeraSystemState): u64 {
        let self = load_system_state(wrapper);
        self.get_storage_fund_total_balance()
    }

    #[test_only]
    public fun get_storage_fund_object_rebates(wrapper: &mut PeraSystemState): u64 {
        let self = load_system_state(wrapper);
        self.get_storage_fund_object_rebates()
    }

    #[test_only]
    public fun get_stake_subsidy_distribution_counter(wrapper: &mut PeraSystemState): u64 {
        let self = load_system_state(wrapper);
        self.get_stake_subsidy_distribution_counter()
    }

    // CAUTION: THIS CODE IS ONLY FOR TESTING AND THIS MACRO MUST NEVER EVER BE REMOVED.  Creates a
    // candidate validator - bypassing the proof of possession check and other metadata validation
    // in the process.
    #[test_only]
    public entry fun request_add_validator_candidate_for_testing(
        wrapper: &mut PeraSystemState,
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
        let self = load_system_state_mut(wrapper);
        self.request_add_validator_candidate_for_testing(
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
        )
    }

    // CAUTION: THIS CODE IS ONLY FOR TESTING AND THIS MACRO MUST NEVER EVER BE REMOVED.
    #[test_only]
    public(package) fun advance_epoch_for_testing(
        wrapper: &mut PeraSystemState,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_charge: u64,
        computation_charge: u64,
        storage_rebate: u64,
        non_refundable_storage_fee: u64,
        storage_fund_reinvest_rate: u64,
        reward_slashing_rate: u64,
        epoch_start_timestamp_ms: u64,
        ctx: &mut TxContext,
    ): Balance<PERA> {
        let storage_reward = balance::create_for_testing(storage_charge);
        let computation_reward = balance::create_for_testing(computation_charge);
        let storage_rebate = advance_epoch(
            storage_reward,
            computation_reward,
            wrapper,
            new_epoch,
            next_protocol_version,
            storage_rebate,
            non_refundable_storage_fee,
            storage_fund_reinvest_rate,
            reward_slashing_rate,
            epoch_start_timestamp_ms,
            ctx,
        );
        storage_rebate
    }
}
