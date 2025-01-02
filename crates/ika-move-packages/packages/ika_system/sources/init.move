// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module ika_system::init;

use ika::ika::IKA;
use ika_system::ika_system;
use ika_system::ika_system_state_inner;
use ika_system::ika_treasury;
use ika_system::staking_pool::StakedIka;
use ika_system::validator;
use ika_system::validator_set::{Self, ValidatorSet};
use sui::balance::{Self, Balance};
use sui::coin::{Coin, TreasuryCap};

public struct Init has key {
    id: UID,
    validators: ValidatorSet,
}

/// Must only be created by `init`.
public struct InitCap has key, store {
    id: UID,
    ika_launch: bool,
}

// Error codes
/// The `create` function was called at incorrect initialize function
const EIncorrectInitializeFunc: u64 = 0;

/// Init function, creates an init cap and transfers it to the sender.
/// This allows the sender to call the function to actually initialize the system
/// with the corresponding parameters. Once that function is called, the cap is destroyed.
fun init(ctx: &mut TxContext) {
    let id = object::new(ctx);
    let init_cap = InitCap {
        id,
        ika_launch: false,
    };
    transfer::transfer(init_cap, ctx.sender());
}

/// Function to initialize ika pre launch and share the system and staking objects.
/// This can only be called once, after which the `InitCap` is destroyed.
public entry fun initialize_ika_pre_launch(cap: &mut InitCap, ctx: &mut TxContext) {
    assert!(!cap.ika_launch, EIncorrectInitializeFunc);
    transfer::share_object(Init {
        id: object::new(ctx),
        validators: validator_set::new(ctx),
    });
    cap.ika_launch = true;
}

/// Function to initialize ika and share the system and staking objects.
/// This can only be called once, after which the `InitCap` is destroyed.
public entry fun initialize_ika_launch(
    cap: InitCap,
    init: Init,
    ika_treasury_cap: TreasuryCap<IKA>,
    protocol_version: u64,
    chain_start_timestamp_ms: u64,
    epoch_duration_ms: u64,
    // Stake Subsidy parameters
    stake_subsidy_start_epoch: u64,
    stake_subsidy_rate: u16,
    stake_subsidy_period_length: u64,
    // Validator committee parameters
    min_validator_count: u64,
    max_validator_count: u64,
    min_validator_joining_stake: u64,
    validator_low_stake_threshold: u64,
    validator_very_low_stake_threshold: u64,
    validator_low_stake_grace_period: u64,
    reward_slashing_rate: u64,
    ctx: &mut TxContext,
) {
    assert!(cap.ika_launch, EIncorrectInitializeFunc);

    let Init {
        id,
        mut validators,
    } = init;

    id.delete();

    validators.launch_ika();

    let storage_fund = balance::zero();

    let system_parameters = ika_system_state_inner::create_system_parameters(
        epoch_duration_ms,
        stake_subsidy_start_epoch,
        // Validator committee parameters
        min_validator_count,
        max_validator_count,
        min_validator_joining_stake,
        validator_low_stake_threshold,
        validator_very_low_stake_threshold,
        validator_low_stake_grace_period,
        reward_slashing_rate,
        ctx,
    );

    let stake_subsidy = ika_treasury::create(
        ika_treasury_cap,
        stake_subsidy_rate,
        stake_subsidy_period_length,
        ctx,
    );

    ika_system::create(
        validators,
        storage_fund,
        protocol_version,
        chain_start_timestamp_ms,
        system_parameters,
        stake_subsidy,
        ctx,
    );

    cap.destroy();
}

fun destroy(cap: InitCap) {
    let InitCap { id, ika_launch: _ } = cap;
    id.delete();
}

// ==== functions to add or remove validators ====

public entry fun request_add_validator_candidate(
    self: &mut Init,
    protocol_pubkey_bytes: vector<u8>,
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
    computation_price: u64,
    commission_rate: u64,
    ctx: &mut TxContext,
) {
    let validator = validator::new(
        ctx.sender(),
        protocol_pubkey_bytes,
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
        computation_price,
        commission_rate,
        ctx,
    );

    self.validators.request_add_validator_candidate(validator, ctx);
}

public entry fun request_remove_validator_candidate(self: &mut Init, ctx: &mut TxContext) {
    self.validators.request_remove_validator_candidate(0, ctx);
}

public entry fun request_add_validator(self: &mut Init, ctx: &mut TxContext) {
    self.validators.request_add_validator(30_000_000*1_000_000_000, ctx);
}

public entry fun request_remove_validator(self: &mut Init, ctx: &mut TxContext) {
    self.validators.request_remove_validator(ctx);
}

/// Add stake to a validator's staking pool.
public entry fun request_add_stake(
    self: &mut Init,
    stake: Coin<IKA>,
    validator_address: address,
    ctx: &mut TxContext,
) {
    let staked_ika = self.request_add_stake_non_entry(stake, validator_address, ctx);
    transfer::public_transfer(staked_ika, ctx.sender());
}

/// The non-entry version of `request_add_stake`, which returns the staked IKA instead of transferring it to the sender.
public fun request_add_stake_non_entry(
    self: &mut Init,
    stake: Coin<IKA>,
    validator_address: address,
    ctx: &mut TxContext,
): StakedIka {
    self.validators.request_add_stake(0, validator_address, stake.into_balance(), ctx)
}

// ==== validator metadata management functions ====

/// Create a new `UnverifiedValidatorOperationCap`, transfer it to the
/// validator candidate and registers it. The original object is thus revoked.
public entry fun rotate_operation_cap(self: &mut Init, ctx: &mut TxContext) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.new_unverified_validator_operation_cap_and_transfer(ctx);
}

/// Update a validator candidate's name.
public entry fun update_candidate_validator_name(
    self: &mut Init,
    name: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_name(name)
}

/// Update a validator candidate's description
public entry fun update_candidate_validator_description(
    self: &mut Init,
    description: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_description(description)
}

/// Update a validator candidate's image url
public entry fun update_candidate_validator_image_url(
    self: &mut Init,
    image_url: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_image_url(image_url)
}

/// Update a validator candidate's project url
public entry fun update_candidate_validator_project_url(
    self: &mut Init,
    project_url: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_project_url(project_url)
}

/// Withdraw stake from a validator's staking pool.
public entry fun request_withdraw_stake(
    self: &mut Init,
    staked_ika: StakedIka,
    ctx: &mut TxContext,
) {
    let withdrawn_stake = self.request_withdraw_stake_non_entry(staked_ika, ctx);
    transfer::public_transfer(withdrawn_stake.into_coin(ctx), ctx.sender());
}

/// Non-entry version of `request_withdraw_stake` that returns the withdrawn IKA instead of transferring it to the sender.
public fun request_withdraw_stake_non_entry(
    self: &mut Init,
    staked_ika: StakedIka,
    ctx: &mut TxContext,
): Balance<IKA> {
    self.validators.request_withdraw_stake(0, staked_ika, ctx)
}

/// Update validator candidate's network address.
public entry fun update_candidate_validator_network_address(
    self: &mut Init,
    network_address: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_network_address(network_address);
}

/// Update validator candidate's p2p address.
public entry fun update_candidate_validator_p2p_address(
    self: &mut Init,
    p2p_address: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_p2p_address(p2p_address)
}

/// Update validator candidate's narwhal primary address.
public entry fun update_candidate_validator_primary_address(
    self: &mut Init,
    primary_address: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_primary_address(primary_address)
}

/// Update validator candidate's narwhal worker address.
public entry fun update_candidate_validator_worker_address(
    self: &mut Init,
    worker_address: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_worker_address(worker_address)
}

/// Update validator candidate's public key of protocol key and proof of possession.
public entry fun update_candidate_validator_protocol_pubkey(
    self: &mut Init,
    protocol_pubkey: vector<u8>,
    proof_of_possession: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_protocol_pubkey(protocol_pubkey, proof_of_possession)
}

/// Update validator candidate's public key of worker key.
public entry fun update_candidate_validator_worker_pubkey(
    self: &mut Init,
    worker_pubkey: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_worker_pubkey(worker_pubkey)
}

/// Update validator candidate's public key of network key.
public entry fun update_candidate_validator_network_pubkey(
    self: &mut Init,
    network_pubkey: vector<u8>,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_ctx_including_candidates(ctx);
    candidate.update_candidate_network_pubkey(network_pubkey)
}

// === Test only ===

#[test_only]
use sui::test_scenario;
#[test_only]
use std::debug;

// ==== tests ====

#[test]
fun test_full_init() {
    let publisher = @0xCAFE;
    let validator1 = @0xFACE1;
    let validator2 = @0xFACE2;
    let validator3 = @0xFACE3;
    let validator4 = @0xFACE4;

    let staker1 = @0xFACA1;
    let staker2 = @0xFACA2;
    let staker3 = @0xFACA3;
    let staker4 = @0xFACA4;
    let staker5 = @0xFACA5;
    let staker6 = @0xFACA6;
    let staker7 = @0xFACA7;
    let staker8 = @0xFACA8;

    let mut scenario = test_scenario::begin(publisher);
    ika::ika::init_for_testing(scenario.ctx());

    scenario.next_tx(publisher);

    let mut treasury_cap = scenario.take_from_address<sui::coin::TreasuryCap<IKA>>(publisher);

    let stake1 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
    let stake2 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
    let stake3 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
    let stake4 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
    let stake5 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
    let stake6 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
    let stake7 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
    let stake8 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());

    init(scenario.ctx());

    scenario.next_tx(publisher);
    let mut init_cap = scenario.take_from_address<InitCap>(publisher);

    initialize_ika_pre_launch(&mut init_cap, scenario.ctx());

    scenario.next_tx(publisher);
    let mut init = test_scenario::take_shared<Init>(&scenario);

    // create candidates

    scenario.next_tx(validator1);
    init.request_add_validator_candidate(
        vector[1],
        vector[1],
        vector[1],
        vector[1],
        b"validator1",
        b"validator1",
        b"validator1",
        b"validator1",
        b"validator1",
        b"validator1",
        b"validator1",
        b"validator1",
        1000,
        1000,
        scenario.ctx(),
    );

    scenario.next_tx(validator2);
    init.request_add_validator_candidate(
        vector[2],
        vector[2],
        vector[2],
        vector[2],
        b"validator2",
        b"validator2",
        b"validator2",
        b"validator2",
        b"validator2",
        b"validator2",
        b"validator2",
        b"validator2",
        1000,
        1000,
        scenario.ctx(),
    );

    scenario.next_tx(validator3);
    init.request_add_validator_candidate(
        vector[3],
        vector[3],
        vector[3],
        vector[3],
        b"validator3",
        b"validator3",
        b"validator3",
        b"validator3",
        b"validator3",
        b"validator3",
        b"validator3",
        b"validator3",
        1000,
        1000,
        scenario.ctx(),
    );

    scenario.next_tx(validator4);
    init.request_add_validator_candidate(
        vector[4],
        vector[4],
        vector[4],
        vector[4],
        b"validator4",
        b"validator4",
        b"validator4",
        b"validator4",
        b"validator4",
        b"validator4",
        b"validator4",
        b"validator4",
        1000,
        1000,
        scenario.ctx(),
    );

    // stake

    scenario.next_tx(staker1);
    init.request_add_stake(stake1, validator1, scenario.ctx());

    scenario.next_tx(staker2);
    let staked1 = scenario.take_from_address<StakedIka>(staker1);

    init.request_add_stake(stake2, validator2, scenario.ctx());

    scenario.next_tx(staker3);
    init.request_add_stake(stake3, validator3, scenario.ctx());

    scenario.next_tx(staker4);
    init.request_add_stake(stake4, validator4, scenario.ctx());

    scenario.next_tx(staker5);
    init.request_add_stake(stake5, validator4, scenario.ctx());

    scenario.next_tx(staker6);
    init.request_add_stake(stake6, validator1, scenario.ctx());

    scenario.next_tx(staker7);
    init.request_add_stake(stake7, validator1, scenario.ctx());

    scenario.next_tx(staker8);
    init.request_add_stake(stake8, validator4, scenario.ctx());

    scenario.next_tx(staker1);
    init.request_withdraw_stake(staked1, scenario.ctx());

    // add validators

    scenario.next_tx(validator1);
    init.request_add_validator(scenario.ctx());

    scenario.next_tx(validator2);
    init.request_add_validator(scenario.ctx());

    scenario.next_tx(validator3);
    init.request_add_validator(scenario.ctx());

    scenario.next_tx(validator4);
    init.request_add_validator(scenario.ctx());

    initialize_ika_launch(
        init_cap,
        init,
        treasury_cap,
        1,
        1733261167371,
        86400000,
        0,
        8,
        365,
        4,
        150,
        30_000_000*1_000_000_000,
        20_000_000*1_000_000_000,
        15_000_000*1_000_000_000,
        7,
        10000,
        scenario.ctx(),
    );

    debug::print(&scenario.end());
}
