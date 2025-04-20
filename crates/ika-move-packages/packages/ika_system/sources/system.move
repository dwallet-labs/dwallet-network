// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// Ika System State Type Upgrade Guide
/// `System` is a thin wrapper around `SystemInnerVX` that provides a versioned interface.
/// The `System` object has a fixed ID 0x5, and the `SystemInnerVX` object is stored as a dynamic field.
/// There are a few different ways to upgrade the `SystemInnerVX` type:
///
/// The simplest and one that doesn't involve a real upgrade is to just add dynamic fields to the `extra_fields` field
/// of `SystemInnerVX` or any of its sub type. This is useful when we are in a rush, or making a small change,
/// or still experimenting a new field.
///
/// To properly upgrade the `SystemInnerVX` type, we need to ship a new framework that does the following:
/// 1. Define a new `SystemInnerVX`type (e.g. `SystemInnerV1`).
/// 2. Define a data migration function that migrates the old `SystemInnerVX` to the new one (i.e. SystemInnerV1).
/// 3. Replace all uses of `SystemInnerVX` with `SystemInnerV1` in both ika_system.move and system_inner_v1.move,
///    with the exception of the `system_inner_v1::create` function, which should always return the init type.
/// 4. Inside `load_inner_maybe_upgrade` function, check the current version in the wrapper, and if it's not the latest version,
///   call the data migration function to upgrade the inner object. Make sure to also update the version in the wrapper.
/// A detailed example can be found in ika/tests/framework_upgrades/mock_ika_systems/shallow_upgrade.
/// Along with the Move change, we also need to update the Rust code to support the new type. This includes:
/// 1. Define a new `SystemInnerVX` struct type that matches the new Move type, and implement the SystemTrait.
/// 2. Update the `System` struct to include the new version as a new enum variant.
/// 3. Update the `get_ika_system_state` function to handle the new version.
/// To test that the upgrade will be successful, we need to modify `ika_system_state_production_upgrade_test` test in
/// protocol_version_tests and trigger a real upgrade using the new framework. We will need to keep this directory as old version,
/// put the new framework in a new directory, and run the test to exercise the upgrade.
///
/// To upgrade Validator type, besides everything above, we also need to:
/// 1. Define a new Validator type (e.g. ValidatorV2).
/// 2. Define a data migration function that migrates the old Validator to the new one (i.e. ValidatorV2).
/// 3. Replace all uses of Validator with ValidatorV2 except the init creation function.
/// 4. In validator_wrapper::upgrade_to_latest, check the current version in the wrapper, and if it's not the latest version,
///  call the data migration function to upgrade it.
/// In Rust, we also need to add a new case in `get_validator_from_table`.
/// Note that it is possible to upgrade SystemInnerVX without upgrading Validator, but not the other way around.
/// And when we only upgrade SystemInnerVX, the version of Validator in the wrapper will not be updated, and hence may become
/// inconsistent with the version of SystemInnerVX. This is fine as long as we don't use the Validator version to determine
/// the SystemInnerVX version, or vice versa.

module ika_system::system;

use ika::ika::IKA;
use ika_system::system_inner_v1::{
    Self,
    SystemParametersV1,
    SystemInnerV1
};
use ika_system::protocol_treasury::ProtocolTreasury;
use ika_system::staking_pool::{PoolTokenExchangeRate};
use ika_system::staked_ika::{StakedIka, FungibleStakedIka};
use ika_system::validator_cap::{ValidatorCap, ValidatorOperationCap};
use ika_system::validator_set::ValidatorSet;
use ika_system::bls_committee::BlsCommittee;
use ika_system::protocol_cap::ProtocolCap;
use ika_system::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof;
use ika_system::dwallet_2pc_mpc_secp256k1::{DWalletCoordinator};
use sui::balance::Balance;
use sui::coin::Coin;
use sui::dynamic_field;
use sui::table::Table;
use sui::clock::Clock;
use sui::package::{UpgradeCap, UpgradeTicket, UpgradeReceipt};

public struct System has key {
    id: UID,
    version: u64,
    package_id: ID,
    new_package_id: Option<ID>,
}

const EWrongInnerVersion: u64 = 0;
const EInvalidMigration: u64 = 1;
const EHaveNotReachedMidEpochTime: u64 = 2;
const EHaveNotReachedEndEpochTime: u64 = 3;
const ECannotAdvanceEpoch: u64 = 4;

/// Flag to indicate the version of the ika system.
const VERSION: u64 = 1;

// ==== functions that can only be called by init ====

/// Create a new System object and make it shared.
/// This function will be called only once in init.
public(package) fun create(
    package_id: ID,
    upgrade_caps: vector<UpgradeCap>,
    validators: ValidatorSet,
    protocol_version: u64,
    epoch_start_timestamp_ms: u64,
    parameters: SystemParametersV1,
    protocol_treasury: ProtocolTreasury,
    authorized_protocol_cap_ids: vector<ID>,
    ctx: &mut TxContext,
) {
    let system_state = system_inner_v1::create(
        upgrade_caps,
        validators,
        protocol_version,
        epoch_start_timestamp_ms,
        parameters,
        protocol_treasury,
        authorized_protocol_cap_ids,
        ctx,
    );
    let mut self = System {
        id: object::new(ctx),
        version: VERSION,
        package_id,
        new_package_id: option::none(),
    };
    dynamic_field::add(&mut self.id, VERSION, system_state);
    transfer::share_object(self);
}

// ==== entry functions ====

public fun initialize(
    self: &mut System,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    let package_id = self.package_id;
    let self = self.inner_mut();
    self.initialize(clock, package_id, ctx);
}

/// Can be called by anyone who wishes to become a validator candidate and starts accruing delegated
/// stakes in their staking pool. Once they have at least `MIN_VALIDATOR_JOINING_STAKE` amount of stake they
/// can call `request_add_validator` to officially become an active validator at the next epoch.
/// Aborts if the caller is already a pending or active validator, or a validator candidate.
/// Note: `proof_of_possession_bytes` MUST be a valid signature using sui_address and protocol_pubkey_bytes.
/// To produce a valid PoP, run [fn test_proof_of_possession_bytes].
public entry fun request_add_validator_candidate(
    self: &mut System,
    pubkey_bytes: vector<u8>,
    network_pubkey_bytes: vector<u8>,
    consensus_pubkey_bytes: vector<u8>,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    proof_of_possession_bytes: vector<u8>,
    name: vector<u8>,
    description: vector<u8>,
    image_url: vector<u8>,
    project_url: vector<u8>,
    network_address: vector<u8>,
    p2p_address: vector<u8>,
    consensus_address: vector<u8>,
    computation_price: u64,
    commission_rate: u16,
    ctx: &mut TxContext,
) {
    let (cap, operation_cap) = self.request_add_validator_candidate_non_entry(
        ctx.sender(),
        pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        class_groups_pubkey_and_proof_bytes,
        proof_of_possession_bytes,
        name,
        description,
        image_url,
        project_url,
        network_address,
        p2p_address,
        consensus_address,
        computation_price,
        commission_rate,
        ctx,
    );
    transfer::public_transfer(cap, ctx.sender());
    transfer::public_transfer(operation_cap, ctx.sender());
}

public fun request_add_validator_candidate_non_entry(
    self: &mut System,
    payment_address: address,
    protocol_pubkey_bytes: vector<u8>,
    network_pubkey_bytes: vector<u8>,
    consensus_pubkey_bytes: vector<u8>,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    proof_of_possession_bytes: vector<u8>,
    name: vector<u8>,
    description: vector<u8>,
    image_url: vector<u8>,
    project_url: vector<u8>,
    network_address: vector<u8>,
    p2p_address: vector<u8>,
    consensus_address: vector<u8>,
    computation_price: u64,
    commission_rate: u16,
    ctx: &mut TxContext,
): (ValidatorCap, ValidatorOperationCap) {
    let self = self.inner_mut();
    self.request_add_validator_candidate(
        payment_address,
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        class_groups_pubkey_and_proof_bytes,
        proof_of_possession_bytes,
        name,
        description,
        image_url,
        project_url,
        network_address,
        p2p_address,
        consensus_address,
        computation_price,
        commission_rate,
        ctx,
    )
}

/// Called by a validator candidate to remove themselves from the candidacy. After this call
/// their staking pool becomes deactivate.
public entry fun request_remove_validator_candidate(
    self: &mut System,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.request_remove_validator_candidate(cap)
}

/// Called by a validator candidate to add themselves to the active validator set beginning next epoch.
/// Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
/// stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
/// epoch has already reached the maximum.
public entry fun request_add_validator(
    self: &mut System,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.request_add_validator(cap)
}

/// A validator can call this function to request a removal in the next epoch.
/// We use the sender of `ctx` to look up the validator
/// (i.e. sender must match the sui_address in the validator).
/// At the end of the epoch, the `validator` object will be returned to the sui_address
/// of the validator.
public entry fun request_remove_validator(self: &mut System, cap: &ValidatorCap) {
    let self = self.inner_mut();
    self.request_remove_validator(cap)
}

/// A validator can call this entry function to submit a new computation price quote, to be
/// used for the computation price per unit size calculation at the end of the epoch.
public entry fun request_set_computation_price(
    self: &mut System,
    cap: &ValidatorOperationCap,
    new_computation_price: u64,
) {
    let self = self.inner_mut();
    self.request_set_computation_price(cap, new_computation_price)
}

/// This entry function is used to set new computation price for candidate validators
public entry fun set_candidate_validator_computation_price(
    self: &mut System,
    cap: &ValidatorOperationCap,
    new_computation_price: u64,
) {
    let self = self.inner_mut();
    self.set_candidate_validator_computation_price(cap, new_computation_price)
}

/// A validator can call this entry function to set a new commission rate, updated at the end of
/// the epoch.
public entry fun request_set_commission_rate(
    self: &mut System,
    new_commission_rate: u16,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.request_set_commission_rate(new_commission_rate, cap)
}

/// This entry function is used to set new commission rate for candidate validators
public entry fun set_candidate_validator_commission_rate(
    self: &mut System,
    new_commission_rate: u16,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.set_candidate_validator_commission_rate(new_commission_rate, cap)
}

/// Add stake to a validator's staking pool.
public entry fun request_add_stake(
    self: &mut System,
    stake: Coin<IKA>,
    validator_id: ID,
    ctx: &mut TxContext,
) {
    let staked_ika = self.request_add_stake_non_entry(stake, validator_id, ctx);
    transfer::public_transfer(staked_ika, ctx.sender());
}

/// The non-entry version of `request_add_stake`, which returns the staked IKA instead of transferring it to the sender.
public fun request_add_stake_non_entry(
    self: &mut System,
    stake: Coin<IKA>,
    validator_id: ID,
    ctx: &mut TxContext,
): StakedIka {
    let self = self.inner_mut();
    self.request_add_stake(stake, validator_id, ctx)
}

/// Add stake to a validator's staking pool using multiple coins.
public entry fun request_add_stake_mul_coin(
    self: &mut System,
    stakes: vector<Coin<IKA>>,
    stake_amount: option::Option<u64>,
    validator_id: ID,
    ctx: &mut TxContext,
) {
    let self = self.inner_mut();
    let staked_ika = self.request_add_stake_mul_coin(stakes, stake_amount, validator_id, ctx);
    transfer::public_transfer(staked_ika, ctx.sender());
}

/// Withdraw stake from a validator's staking pool.
public entry fun request_withdraw_stake(
    self: &mut System,
    staked_ika: StakedIka,
    ctx: &mut TxContext,
) {
    let withdrawn_stake = self.request_withdraw_stake_non_entry(staked_ika);
    transfer::public_transfer(withdrawn_stake.into_coin(ctx), ctx.sender());
}

/// Convert StakedIka into a FungibleStakedIka object.
public fun convert_to_fungible_staked_ika(
    self: &mut System,
    staked_ika: StakedIka,
    ctx: &mut TxContext,
): FungibleStakedIka {
    let self = self.inner_mut();
    self.convert_to_fungible_staked_ika(staked_ika, ctx)
}

/// Convert FungibleStakedIka into a StakedIka object.
public fun redeem_fungible_staked_ika(
    self: &mut System,
    fungible_staked_ika: FungibleStakedIka,
): Balance<IKA> {
    let self = self.inner_mut();
    self.redeem_fungible_staked_ika(fungible_staked_ika)
}

/// Non-entry version of `request_withdraw_stake` that returns the withdrawn IKA instead of transferring it to the sender.
public fun request_withdraw_stake_non_entry(
    self: &mut System,
    staked_ika: StakedIka,
): Balance<IKA> {
    let self = self.inner_mut();
    self.request_withdraw_stake(staked_ika)
}

/// Report a validator as a bad or non-performant actor in the system.
/// Succeeds if all the following are satisfied:
/// 1. both the reporter in `cap` and the input `reportee_id` are active validators.
/// 2. reporter and reportee not the same address.
/// 3. the cap object is still valid.
/// This function is idempotent.
public entry fun report_validator(
    self: &mut System,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    let self = self.inner_mut();
    self.report_validator(cap, reportee_id)
}

/// Undo a `report_validator` action. Aborts if
/// 1. the reportee is not a currently active validator or
/// 2. the sender has not previously reported the `reportee_id`, or
/// 3. the cap is not valid
public entry fun undo_report_validator(
    self: &mut System,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    let self = self.inner_mut();
    self.undo_report_validator(cap, reportee_id)
}

// ==== validator metadata management functions ====

/// Create a new `ValidatorOperationCap`, transfer it to the
/// validator and registers it. The original object is thus revoked.
public entry fun rotate_operation_cap(self: &mut System, cap: &ValidatorCap, ctx: &mut TxContext) {
    let operation_cap = self.rotate_operation_cap_non_entry(cap, ctx);
    transfer::public_transfer(operation_cap, ctx.sender());
}

/// Create a new `ValidatorOperationCap` and registers it. The original object is thus revoked.
public fun rotate_operation_cap_non_entry(self: &mut System, cap: &ValidatorCap, ctx: &mut TxContext): ValidatorOperationCap {
    let self = self.inner_mut();
    self.rotate_operation_cap(cap, ctx)
}

/// Update a validator's payment address.
public entry fun update_validator_payment_address(
    self: &mut System,
    payment_address: address,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_validator_payment_address(payment_address, cap)
}

/// Update a validator's name.
public entry fun update_validator_name(
    self: &mut System,
    name: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_validator_name(name, cap)
}

/// Update a validator's description
public entry fun update_validator_description(
    self: &mut System,
    description: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_validator_description(description, cap)
}

/// Update a validator's image url
public entry fun update_validator_image_url(
    self: &mut System,
    image_url: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_validator_image_url(image_url, cap)
}

/// Update a validator's project url
public entry fun update_validator_project_url(
    self: &mut System,
    project_url: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_validator_project_url(project_url, cap)
}

/// Update a validator's network address.
/// The change will only take effects starting from the next epoch.
public entry fun update_validator_next_epoch_network_address(
    self: &mut System,
    network_address: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_validator_next_epoch_network_address(network_address, cap)
}

/// Update candidate validator's network address.
public entry fun update_candidate_validator_network_address(
    self: &mut System,
    network_address: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_candidate_validator_network_address(network_address, cap)
}

/// Update a validator's p2p address.
/// The change will only take effects starting from the next epoch.
public entry fun update_validator_next_epoch_p2p_address(
    self: &mut System,
    p2p_address: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_validator_next_epoch_p2p_address(p2p_address, cap)
}

/// Update candidate validator's p2p address.
public entry fun update_candidate_validator_p2p_address(
    self: &mut System,
    p2p_address: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_candidate_validator_p2p_address(p2p_address, cap)
}

/// Update a validator's consensus address.
/// The change will only take effects starting from the next epoch.
public entry fun update_validator_next_epoch_consensus_address(
    self: &mut System,
    consensus_address: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_validator_next_epoch_consensus_address(consensus_address, cap)
}

/// Update candidate validator's consensus address.
public entry fun update_candidate_validator_consensus_address(
    self: &mut System,
    consensus_address: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_candidate_validator_consensus_address(consensus_address, cap)
}

/// Update a validator's public key of protocol key and proof of possession.
/// The change will only take effects starting from the next epoch.
public entry fun update_validator_next_epoch_protocol_pubkey_bytes(
    self: &mut System,
    protocol_pubkey: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    cap: &ValidatorCap,
    ctx: &TxContext,
) {
    let self = self.inner_mut();
    self.update_validator_next_epoch_protocol_pubkey_bytes(protocol_pubkey, proof_of_possession_bytes, cap, ctx)
}

/// Update candidate validator's public key of protocol key and proof of possession.
public entry fun update_candidate_validator_protocol_pubkey_bytes(
    self: &mut System,
    protocol_pubkey: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    cap: &ValidatorCap,
    ctx: &TxContext,
) {
    let self = self.inner_mut();
    self.update_candidate_validator_protocol_pubkey_bytes(protocol_pubkey, proof_of_possession_bytes, cap, ctx)
}

/// Update a validator's public key of worker key.
/// The change will only take effects starting from the next epoch.
public entry fun update_validator_next_epoch_consensus_pubkey_bytes(
    self: &mut System,
    consensus_pubkey_bytes: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_validator_next_epoch_consensus_pubkey_bytes(consensus_pubkey_bytes, cap)
}

/// Update candidate validator's public key of worker key.
public entry fun update_candidate_validator_consensus_pubkey_bytes(
    self: &mut System,
    consensus_pubkey_bytes: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_candidate_validator_consensus_pubkey_bytes(consensus_pubkey_bytes, cap)
}

/// Update a validator's public key of class groups key and its associated proof.
/// The change will only take effects starting from the next epoch.
public entry fun update_validator_next_epoch_class_groups_pubkey_and_proof_bytes(
    self: &mut System,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_validator_next_epoch_class_groups_pubkey_and_proof_bytes(class_groups_pubkey_and_proof, cap)
}

/// Update candidate validator's public key of class groups key and its associated proof.
public entry fun update_candidate_validator_class_groups_pubkey_and_proof_bytes(
    self: &mut System,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_candidate_validator_class_groups_pubkey_and_proof_bytes(class_groups_pubkey_and_proof, cap)
}

/// Update a validator's public key of network key.
/// The change will only take effects starting from the next epoch.
public entry fun update_validator_next_epoch_network_pubkey_bytes(
    self: &mut System,
    network_pubkey: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_validator_next_epoch_network_pubkey_bytes(network_pubkey, cap)
}

/// Update candidate validator's public key of network key.
public entry fun update_candidate_validator_network_pubkey_bytes(
    self: &mut System,
    network_pubkey: vector<u8>,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.update_candidate_validator_network_pubkey_bytes(network_pubkey, cap)
}

/// Getter of the pool token exchange rate of a validator. Works for both active and inactive pools.
public fun pool_exchange_rates(
    self: &mut System,
    validator_id: ID,
): &Table<u64, PoolTokenExchangeRate> {
    let self = self.inner_mut();
    self.pool_exchange_rates(validator_id)
}

/// Getter returning ids of the currently active validators.
public fun active_committee(self: &mut System): BlsCommittee {
    let self = self.inner();
    self.active_committee()
}

public fun process_checkpoint_message_by_cap(
    self: &mut System,
    cap: &ProtocolCap,
    message: vector<u8>,
    ctx: &mut TxContext,
) {
    let self = self.inner_mut();
    self.process_checkpoint_message_by_cap(cap, message, ctx);
}

// TODO: split dwallet_2pc_mpc_secp256k1 to its own checkpoint
public fun process_checkpoint_message_by_quorum(
    self: &mut System,
    dwallet_2pc_mpc_secp256k1: &mut DWalletCoordinator,
    signature: vector<u8>,
    signers_bitmap: vector<u8>,
    mut message: vector<u8>,
    message2: vector<u8>,
    message3: vector<u8>,
    message4: vector<u8>,
    ctx: &mut TxContext,
) {
    message.append(message2);
    message.append(message3);
    message.append(message4);

    let self = self.inner_mut();
    self.process_checkpoint_message_by_quorum(dwallet_2pc_mpc_secp256k1, signature, signers_bitmap, message, ctx);
}

/// Locks the committee of the next epoch to allow starting the reconfiguration process.
public fun request_reconfig_mid_epoch(
    self: &mut System, dwallet_coordinator: &mut DWalletCoordinator, clock: &Clock, ctx: &mut TxContext
) {
    let inner = self.inner_mut();
    assert!(clock.timestamp_ms() > inner.epoch_start_timestamp_ms() + (inner.epoch_duration_ms() / 2), EHaveNotReachedMidEpochTime);
    // add check -> key state
    inner.emit_start_reshare_events(dwallet_coordinator.inner_mut(), ctx);
    self.inner_mut().process_mid_epoch();
}

/// Locks the MPC sessions that should get completed as part of the current epoch.
public fun request_lock_epoch_sessions(
    self: &mut System, dwallet_coordinator: &mut DWalletCoordinator, clock: &Clock, _ctx: &TxContext
) {
    let inner = self.inner_mut();
    assert!(clock.timestamp_ms() > inner.epoch_start_timestamp_ms() + (inner.epoch_duration_ms()), EHaveNotReachedEndEpochTime);
    dwallet_coordinator.inner_mut().lock_last_active_session_sequence_number();
}

/// Advances the epoch to the next epoch.
public fun request_advance_epoch(self: &mut System, dwallet_coordinator: &mut DWalletCoordinator, clock: &Clock, ctx: &mut TxContext) {
    let inner_system = self.inner_mut();
    let inner_dwallet = dwallet_coordinator.inner_mut();
    assert!(inner_dwallet.all_current_epoch_sessions_completed(), ECannotAdvanceEpoch);
    inner_system.advance_epoch(clock.timestamp_ms(), ctx);
    dwallet_coordinator.advance_epoch(inner_system.active_committee());
    inner_system.advance_network_keys(dwallet_coordinator);
}

public fun request_dwallet_network_decryption_key_dkg_by_cap(
    self: &mut System,
    dwallet_2pc_mpc_secp256k1: &mut DWalletCoordinator,
    cap: &ProtocolCap,
    ctx: &mut TxContext,
) {
    let self = self.inner_mut();
    self.request_dwallet_network_decryption_key_dkg_by_cap(dwallet_2pc_mpc_secp256k1, cap, ctx);
}

// === Upgrades ===

public fun authorize_update_message_by_cap(
    self: &mut System,
    cap: &ProtocolCap,
    package_id: ID,
    digest: vector<u8>,
): UpgradeTicket {
    let self = self.inner_mut();
    self.authorize_update_message_by_cap(cap, package_id, digest)
}

public fun commit_upgrade(
    self: &mut System,
    receipt: UpgradeReceipt,
) {
    let new_package_id = receipt.package();
    let old_package_id = self.inner_mut().commit_upgrade(receipt);
    if (self.package_id == old_package_id) {
        self.new_package_id = option::some(new_package_id);
    }
}

/// Migrate the staking object to the new package id.
///
/// This function sets the new package id and version and can be modified in future versions
/// to migrate changes in the `system_inner` object if needed.
public fun migrate(
        self: &mut System,
) {
    assert!(self.version < VERSION, EInvalidMigration);

    // Move the old system state inner to the new version.
    let system_inner: SystemInnerV1 = dynamic_field::remove(&mut self.id, self.version);
    dynamic_field::add(&mut self.id, VERSION, system_inner);
    self.version = VERSION;

    // Set the new package id.
    assert!(self.new_package_id.is_some(), EInvalidMigration);
    self.package_id = self.new_package_id.extract();
}

// === Internals ===

/// Get a mutable reference to `SystemInnerVX` from the `System`.
fun inner_mut(self: &mut System): &mut SystemInnerV1 {
    assert!(self.version == VERSION, EWrongInnerVersion);
    dynamic_field::borrow_mut(&mut self.id, VERSION)
}

/// Get an immutable reference to `SystemInnerVX` from the `System`.
fun inner(self: &System): &SystemInnerV1 {
    assert!(self.version == VERSION, EWrongInnerVersion);
    dynamic_field::borrow(&self.id, VERSION)
}

#[test_only]
/// Return the current epoch number. Useful for applications that need a coarse-grained concept of time,
/// since epochs are ever-increasing and epoch changes are intended to happen every 24 hours.
public fun epoch(self: &mut System): u64 {
    let self = self.inner();
    self.epoch()
}

#[test_only]
/// Returns unix timestamp of the start of current epoch
public fun epoch_start_timestamp_ms(self: &mut System): u64 {
    let self = self.inner();
    self.epoch_start_timestamp_ms()
}

#[test_only]
/// Returns the total amount staked with `validator_id`.
/// Aborts if `validator_id` is not an active validator.
public fun validator_stake_amount(self: &mut System, validator_id: ID): u64 {
    let self = self.inner_mut();
    self.validator_stake_amount(validator_id)
}

#[test_only]
/// Returns all the validators who are currently reporting `validator_id`
public fun get_reporters_of(self: &mut System, validator_id: ID): VecSet<ID> {
    let self = self.inner();
    self.get_reporters_of(validator_id)
}

#[test_only]
/// Return the current validator set
public fun validators(self: &mut System): &ValidatorSet {
    let self = self.inner();
    self.validators()
}

#[test_only]
public fun set_epoch_for_testing(self: &mut System, epoch_num: u64) {
    let self = self.inner_mut();
    self.set_epoch_for_testing(epoch_num)
}

#[test_only]
public fun request_add_validator_for_testing(
    self: &mut System,
    min_joining_stake_for_testing: u64,
    cap: &ValidatorCap,
) {
    let self = self.inner_mut();
    self.request_add_validator_for_testing(min_joining_stake_for_testing, cap)
}

#[test_only]
public fun get_stake_subsidy_stake_subsidy_distribution_counter(self: &mut System): u64 {
    let self = self.inner();
    self.get_stake_subsidy_stake_subsidy_distribution_counter()
}

#[test_only]
public fun set_stake_subsidy_stake_subsidy_distribution_counter(self: &mut System, counter: u64) {
    let self = self.inner_mut();
    self.set_stake_subsidy_stake_subsidy_distribution_counter(counter)
}

#[test_only]
public fun inner_mut_for_testing(self: &mut System): &mut SystemInnerV1 {
    self.inner_mut()
}

// CAUTION: THIS CODE IS ONLY FOR TESTING AND THIS MACRO MUST NEVER EVER BE REMOVED.  Creates a
// candidate validator - bypassing the proof of possession check and other metadata validation
// in the process.
#[test_only]
public fun request_add_validator_candidate_for_testing(
    self: &mut System,
    protocol_pubkey_bytes: vector<u8>,
    network_pubkey_bytes: vector<u8>,
    consensus_pubkey_bytes_bytes: vector<u8>,
    class_groups_pubkey_and_proof_bytes: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    name: vector<u8>,
    description: vector<u8>,
    image_url: vector<u8>,
    project_url: vector<u8>,
    network_address: vector<u8>,
    p2p_address: vector<u8>,
    consensus_address: vector<u8>,
    computation_price: u64,
    commission_rate: u16,
    ctx: &mut TxContext,
): (ValidatorCap, ValidatorOperationCap) {
    let self = self.inner_mut();
    self.request_add_validator_candidate_for_testing(
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes_bytes,
        class_groups_pubkey_and_proof_bytes,
        proof_of_possession_bytes,
        name,
        description,
        image_url,
        project_url,
        network_address,
        p2p_address,
        consensus_address,
        computation_price,
        commission_rate,
        ctx,
    )
}

// // CAUTION: THIS CODE IS ONLY FOR TESTING AND THIS MACRO MUST NEVER EVER BE REMOVED.
// #[test_only]
// public(package) fun advance_epoch_for_testing(
//     self: &mut System,
//     new_epoch: u64,
//     next_protocol_version: u64,
//     storage_charge: u64,
//     computation_charge: u64,
//     storage_rebate: u64,
//     non_refundable_storage_fee: u64,
//     storage_fund_reinvest_rate: u64,
//     reward_slashing_rate: u64,
//     epoch_start_timestamp_ms: u64,
//     ctx: &mut TxContext,
// ): Balance<IKA> {
//     let storage_reward = balance::create_for_testing(storage_charge);
//     let computation_reward = balance::create_for_testing(computation_charge);
//     let storage_rebate = advance_epoch(
//         storage_reward,
//         computation_reward,
//         wrapper,
//         new_epoch,
//         next_protocol_version,
//         storage_rebate,
//         non_refundable_storage_fee,
//         storage_fund_reinvest_rate,
//         reward_slashing_rate,
//         epoch_start_timestamp_ms,
//         ctx,
//     );
//     storage_rebate
// }
