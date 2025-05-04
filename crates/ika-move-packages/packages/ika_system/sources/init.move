// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::init;

use std::type_name;
use ika::ika::IKA;
use ika_system::system;
use ika_system::system_inner;
use ika_system::protocol_treasury;
use ika_system::display;
use ika_system::validator_set::{Self};
use ika_system::protocol_cap::{Self, ProtocolCap};
use sui::coin::{TreasuryCap};
use sui::package::{Self, Publisher, UpgradeCap};

/// The provided upgrade cap does not belong to this package.
const EInvalidUpgradeCap: u64 = 1;

/// The OTW to create `Publisher` and `Display` objects.
public struct INIT has drop {}

/// Must only be created by `init`.
public struct InitCap has key, store {
    id: UID,
    publisher: Publisher,
}

/// Init function, creates an init cap and transfers it to the sender.
/// This allows the sender to call the function to actually initialize the system
/// with the corresponding parameters. Once that function is called, the cap is destroyed.
fun init(otw: INIT, ctx: &mut TxContext) {
    let id = object::new(ctx);
    let publisher = package::claim(otw, ctx);
    let init_cap = InitCap { id, publisher };
    transfer::transfer(init_cap, ctx.sender());
}


/// Function to initialize ika and share the system object.
/// This can only be called once, after which the `InitCap` is destroyed.
public fun initialize(
    init_cap: InitCap,
    ika_upgrade_cap: UpgradeCap,
    ika_system_upgrade_cap: UpgradeCap,
    protocol_treasury_cap: TreasuryCap<IKA>,
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
    reward_slashing_rate: u16,
    lock_active_committee: bool,
    ctx: &mut TxContext,
): ProtocolCap {
    let InitCap { id, publisher } = init_cap;
    id.delete();

    let ika_package_id = ika_upgrade_cap.package();
    let ika_system_package_id = ika_system_upgrade_cap.package();

    assert!(
        type_name::get<IKA>().get_address() == ika_package_id.to_address().to_ascii_string(),
        EInvalidUpgradeCap,
    );

    assert!(
        type_name::get<InitCap>().get_address() == ika_system_package_id.to_address().to_ascii_string(),
        EInvalidUpgradeCap,
    );

    let upgrade_caps = vector[ika_upgrade_cap, ika_system_upgrade_cap];

    let validators = validator_set::new(
        min_validator_count,
        max_validator_count,
        min_validator_joining_stake,
        ctx,
    );

    let system_parameters = system_inner::create_system_parameters(
        epoch_duration_ms,
        stake_subsidy_start_epoch,
        // Validator committee parameters
        reward_slashing_rate,
        lock_active_committee,
        ctx,
    );

    let stake_subsidy = protocol_treasury::create(
        protocol_treasury_cap,
        stake_subsidy_rate,
        stake_subsidy_period_length,
        ctx,
    );

    let protocol_cap = protocol_cap::new_protocol_cap(ctx);

    let authorized_protocol_cap_ids = vector[object::id(&protocol_cap)];

    system::create(
        ika_system_package_id,
        upgrade_caps,
        validators,
        protocol_version,
        chain_start_timestamp_ms,
        system_parameters,
        stake_subsidy,
        authorized_protocol_cap_ids,
        ctx,
    );

    display::create(publisher, ctx);
    
    protocol_cap
}


// // === Test only ===

// #[test_only]
// use sui::test_scenario;
// #[test_only]
// use std::debug;

// // ==== tests ====

// #[test]
// fun test_full_init() {
//     let publisher = @0xCAFE;
//     let validator1 = @0xFACE1;
//     let validator2 = @0xFACE2;
//     let validator3 = @0xFACE3;
//     let validator4 = @0xFACE4;

//     let staker1 = @0xFACA1;
//     let staker2 = @0xFACA2;
//     let staker3 = @0xFACA3;
//     let staker4 = @0xFACA4;
//     let staker5 = @0xFACA5;
//     let staker6 = @0xFACA6;
//     let staker7 = @0xFACA7;
//     let staker8 = @0xFACA8;

//     let mut scenario = test_scenario::begin(publisher);
//     ika::ika::init_for_testing(scenario.ctx());

//     scenario.next_tx(publisher);

//     let mut treasury_cap = scenario.take_from_address<sui::coin::TreasuryCap<IKA>>(publisher);

//     let stake1 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
//     let stake2 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
//     let stake3 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
//     let stake4 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
//     let stake5 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
//     let stake6 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
//     let stake7 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());
//     let stake8 = sui::coin::mint(&mut treasury_cap, 40_000_000*1_000_000_000, scenario.ctx());

//     init(scenario.ctx());

//     scenario.next_tx(publisher);
//     let mut init_cap = scenario.take_from_address<InitCap>(publisher);

//     initialize_ika_pre_launch(&mut init_cap, scenario.ctx());

//     scenario.next_tx(publisher);
//     let mut init = test_scenario::take_shared<Init>(&scenario);

//     // create candidates

//     scenario.next_tx(validator1);
//     init.request_add_validator_candidate(
//         vector[1],
//         vector[1],
//         vector[1],
//         vector[1],
//         b"validator1",
//         b"validator1",
//         b"validator1",
//         b"validator1",
//         b"validator1",
//         b"validator1",
//         b"validator1",
//         1000,
//         1000,
//         scenario.ctx(),
//     );

//     scenario.next_tx(validator2);
//     init.request_add_validator_candidate(
//         vector[2],
//         vector[2],
//         vector[2],
//         vector[2],
//         b"validator2",
//         b"validator2",
//         b"validator2",
//         b"validator2",
//         b"validator2",
//         b"validator2",
//         b"validator2",
//         1000,
//         1000,
//         scenario.ctx(),
//     );

//     scenario.next_tx(validator3);
//     init.request_add_validator_candidate(
//         vector[3],
//         vector[3],
//         vector[3],
//         vector[3],
//         b"validator3",
//         b"validator3",
//         b"validator3",
//         b"validator3",
//         b"validator3",
//         b"validator3",
//         b"validator3",
//         1000,
//         1000,
//         scenario.ctx(),
//     );

//     scenario.next_tx(validator4);
//     init.request_add_validator_candidate(
//         vector[4],
//         vector[4],
//         vector[4],
//         vector[4],
//         b"validator4",
//         b"validator4",
//         b"validator4",
//         b"validator4",
//         b"validator4",
//         b"validator4",
//         b"validator4",
//         1000,
//         1000,
//         scenario.ctx(),
//     );

//     // stake

//     scenario.next_tx(staker1);
//     init.request_add_stake(stake1, validator1, scenario.ctx());

//     scenario.next_tx(staker2);
//     let staked1 = scenario.take_from_address<StakedIka>(staker1);

//     init.request_add_stake(stake2, validator2, scenario.ctx());

//     scenario.next_tx(staker3);
//     init.request_add_stake(stake3, validator3, scenario.ctx());

//     scenario.next_tx(staker4);
//     init.request_add_stake(stake4, validator4, scenario.ctx());

//     scenario.next_tx(staker5);
//     init.request_add_stake(stake5, validator4, scenario.ctx());

//     scenario.next_tx(staker6);
//     init.request_add_stake(stake6, validator1, scenario.ctx());

//     scenario.next_tx(staker7);
//     init.request_add_stake(stake7, validator1, scenario.ctx());

//     scenario.next_tx(staker8);
//     init.request_add_stake(stake8, validator4, scenario.ctx());

//     scenario.next_tx(staker1);
//     init.request_withdraw_stake(staked1, scenario.ctx());

//     // add validators

//     scenario.next_tx(validator1);
//     init.request_add_validator(scenario.ctx());

//     scenario.next_tx(validator2);
//     init.request_add_validator(scenario.ctx());

//     scenario.next_tx(validator3);
//     init.request_add_validator(scenario.ctx());

//     scenario.next_tx(validator4);
//     init.request_add_validator(scenario.ctx());

//     initialize_ika_launch(
//         init_cap,
//         init,
//         treasury_cap,
//         1,
//         1733261167371,
//         86400000,
//         0,
//         8,
//         365,
//         4,
//         150,
//         30_000_000*1_000_000_000,
//         20_000_000*1_000_000_000,
//         15_000_000*1_000_000_000,
//         7,
//         10000,
//         scenario.ctx(),
//     );

//     debug::print(&scenario.end());
// }
