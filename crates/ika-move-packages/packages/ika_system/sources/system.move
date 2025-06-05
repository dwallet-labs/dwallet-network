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
/// 3. Replace all uses of `SystemInnerVX` with `SystemInnerV1` in both ika_system.move and system_inner.move,
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

// === Imports ===

use std::string::String;
use ika::ika::IKA;
use ika_system::{
    bls_committee::BlsCommittee,
    class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof,
    dwallet_2pc_mpc_coordinator::DWalletCoordinator,
    dwallet_pricing::DWalletPricing,
    protocol_treasury::ProtocolTreasury,
    staked_ika::StakedIka,
    system_inner::{Self, SystemInnerV1, ProtocolCap},
    token_exchange_rate::TokenExchangeRate,
    validator_cap::{ValidatorCap, ValidatorCommissionCap, ValidatorOperationCap},
    validator_metadata::ValidatorMetadata,
    validator_set::ValidatorSet
};
use sui::{
    clock::Clock,
    coin::Coin,
    dynamic_field,
    package::{UpgradeCap, UpgradeReceipt, UpgradeTicket},
    table::Table,
    vec_map::VecMap
};

// === Errors ===
const EWrongInnerVersion: u64 = 0;
const EInvalidMigration: u64 = 1;

// === Constants ===
/// Flag to indicate the version of the ika system.
const VERSION: u64 = 1;

// === Structs ===
public struct System has key {
    id: UID,
    version: u64,
    package_id: ID,
    new_package_id: Option<ID>,
}

// === Functions that can only be called by init ===

/// Create a new System object and make it shared.
/// This function will be called only once in init.
public(package) fun create(
    package_id: ID,
    upgrade_caps: vector<UpgradeCap>,
    validators: ValidatorSet,
    protocol_version: u64,
    epoch_start_timestamp_ms: u64,
    epoch_duration_ms: u64,
    stake_subsidy_start_epoch: u64,
    protocol_treasury: ProtocolTreasury,
    ctx: &mut TxContext,
): ProtocolCap {
    let (system_state, protocol_cap) = system_inner::create(
        upgrade_caps,
        validators,
        protocol_version,
        epoch_start_timestamp_ms,
        epoch_duration_ms,
        stake_subsidy_start_epoch,
        protocol_treasury,
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
    protocol_cap
}

// === Package Functions ===

public fun initialize(
    self: &mut System,
    pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap<u32, VecMap<u32, vector<u32>>>,
    max_validator_count: u64,
    cap: &ProtocolCap,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    let package_id = self.package_id;
    self.inner_mut().initialize(pricing, supported_curves_to_signature_algorithms_to_hash_schemes, max_validator_count, package_id, cap, clock, ctx);
}

/// Can be called by anyone who wishes to become a validator candidate and starts accruing delegated
/// stakes in their staking pool. Once they have at least `MIN_VALIDATOR_JOINING_STAKE` amount of stake they
/// can call `request_add_validator` to officially become an active validator at the next epoch.
/// Aborts if the caller is already a pending or active validator, or a validator candidate.
/// Note: `proof_of_possession_bytes` MUST be a valid signature using sui_address and protocol_pubkey_bytes.
/// To produce a valid PoP, run [fn test_proof_of_possession_bytes].
public fun request_add_validator_candidate(
    self: &mut System,
    name: String,
    protocol_pubkey_bytes: vector<u8>,
    network_pubkey_bytes: vector<u8>,
    consensus_pubkey_bytes: vector<u8>,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    proof_of_possession_bytes: vector<u8>,
    network_address: String,
    p2p_address: String,
    consensus_address: String,
    commission_rate: u16,
    metadata: ValidatorMetadata,
    ctx: &mut TxContext,
): (ValidatorCap, ValidatorOperationCap, ValidatorCommissionCap) {
    self.inner_mut().request_add_validator_candidate(
        name,
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        class_groups_pubkey_and_proof_bytes,
        proof_of_possession_bytes,
        network_address,
        p2p_address,
        consensus_address,
        commission_rate,
        metadata,
        ctx,
    )
}

/// Called by a validator candidate to remove themselves from the candidacy. After this call
/// their staking pool becomes deactivate.
public fun request_remove_validator_candidate(
    self: &mut System,
    cap: &ValidatorCap,
) {
    self.inner_mut().request_remove_validator_candidate(cap)
}

/// Called by a validator candidate to add themselves to the active validator set beginning next epoch.
/// Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
/// stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
/// epoch has already reached the maximum.
public fun request_add_validator(
    self: &mut System,
    cap: &ValidatorCap,
) {
    self.inner_mut().request_add_validator(cap)
}

/// A validator can call this function to request a removal in the next epoch.
/// We use the sender of `ctx` to look up the validator
/// (i.e. sender must match the sui_address in the validator).
/// At the end of the epoch, the `validator` object will be returned to the sui_address
/// of the validator.
public fun request_remove_validator(self: &mut System, cap: &ValidatorCap) {
    self.inner_mut().request_remove_validator(cap)
}

/// A validator can call this function to set a new commission rate, updated at the end of
/// the epoch.
public fun set_next_commission(
    self: &mut System,
    new_commission_rate: u16,
    cap: &ValidatorOperationCap,
) {
    self.inner_mut().set_next_commission(new_commission_rate, cap)
}

/// Add stake to a validator's staking pool.
public fun request_add_stake(
    self: &mut System,
    stake: Coin<IKA>,
    validator_id: ID,
    ctx: &mut TxContext,
): StakedIka {
    self.inner_mut().request_add_stake(stake, validator_id, ctx)
}

/// Marks the amount as a withdrawal to be processed and removes it from the stake weight of the
/// node. Allows the user to call withdraw_stake after the epoch change to the next epoch and
/// shard transfer is done.
public fun request_withdraw_stake(
    self: &mut System,
    staked_ika: &mut StakedIka,
) {
    self.inner_mut().request_withdraw_stake(staked_ika);
}

/// Withdraws the staked amount from the staking pool.
public fun withdraw_stake(
    self: &mut System,
    staked_ika: StakedIka,
    ctx: &mut TxContext,
): Coin<IKA> {
    self.inner_mut().withdraw_stake(staked_ika, ctx)
}

/// Report a validator as a bad or non-performant actor in the system.
/// Succeeds if all the following are satisfied:
/// 1. both the reporter in `cap` and the input `reportee_id` are active validators.
/// 2. reporter and reportee not the same address.
/// 3. the cap object is still valid.
/// This function is idempotent.
public fun report_validator(
    self: &mut System,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.inner_mut().report_validator(cap, reportee_id)
}

/// Undo a `report_validator` action. Aborts if
/// 1. the reportee is not a currently active validator or
/// 2. the sender has not previously reported the `reportee_id`, or
/// 3. the cap is not valid
public fun undo_report_validator(
    self: &mut System,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.inner_mut().undo_report_validator(cap, reportee_id)
}

// === validator metadata management functions ===

/// Create a new `ValidatorOperationCap` and registers it. The original object is thus revoked.
public fun rotate_operation_cap(self: &mut System, cap: &ValidatorCap, ctx: &mut TxContext): ValidatorOperationCap {
    self.inner_mut().rotate_operation_cap(cap, ctx)
}

/// Create a new `ValidatorCommissionCap` and registers it. The original object is thus revoked.
public fun rotate_commission_cap(self: &mut System, cap: &ValidatorCap, ctx: &mut TxContext): ValidatorCommissionCap {
    self.inner_mut().rotate_commission_cap(cap, ctx)
}

/// Set a validator's name.
public fun set_validator_name(
    self: &mut System,
    name: String,
    cap: &ValidatorOperationCap,
) {
    self.inner_mut().set_validator_name(name, cap);
}

public fun set_validator_metadata(
    self: &mut System,
    metadata: ValidatorMetadata,
    cap: &ValidatorOperationCap,
) {
    self.inner_mut().set_validator_metadata(cap, metadata);
}

/// Sets a validator's network address.
/// The change will only take effects starting from the next epoch.
public fun set_next_epoch_network_address(
    self: &mut System,
    network_address: String,
    cap: &ValidatorOperationCap,
) {
    self.inner_mut().set_next_epoch_network_address(network_address, cap)
}

/// Sets a validator's p2p address.
/// The change will only take effects starting from the next epoch.
public fun set_next_epoch_p2p_address(
    self: &mut System,
    p2p_address: String,
    cap: &ValidatorOperationCap,
) {
    self.inner_mut().set_next_epoch_p2p_address(p2p_address, cap)
}

/// Sets a validator's consensus address.
/// The change will only take effects starting from the next epoch.
public fun set_next_epoch_consensus_address(
    self: &mut System,
    consensus_address: String,
    cap: &ValidatorOperationCap,
) {
    self.inner_mut().set_next_epoch_consensus_address(consensus_address, cap)
}

/// Sets a validator's public key of protocol key and proof of possession.
/// The change will only take effects starting from the next epoch.
public fun set_next_epoch_protocol_pubkey_bytes(
    self: &mut System,
    protocol_pubkey: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    cap: &ValidatorOperationCap,
    ctx: &mut TxContext,
) {
    self.inner_mut().set_next_epoch_protocol_pubkey_bytes(protocol_pubkey, proof_of_possession_bytes, cap, ctx)
}

/// Sets a validator's public key of network key.
/// The change will only take effects starting from the next epoch.
public fun set_next_epoch_network_pubkey_bytes(
    self: &mut System,
    network_pubkey: vector<u8>,
    cap: &ValidatorOperationCap,
) {
    self.inner_mut().set_next_epoch_network_pubkey_bytes(network_pubkey, cap)
}

/// Sets a validator's public key of worker key.
/// The change will only take effects starting from the next epoch.
public fun set_next_epoch_consensus_pubkey_bytes(
    self: &mut System,
    consensus_pubkey_bytes: vector<u8>,
    cap: &ValidatorOperationCap,
) {
    self.inner_mut().set_next_epoch_consensus_pubkey_bytes(consensus_pubkey_bytes, cap)
}

/// Sets a validator's public key of class groups key and its associated proof.
/// The change will only take effects starting from the next epoch.
public fun set_next_epoch_class_groups_pubkey_and_proof_bytes(
    self: &mut System,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorOperationCap,
) {
    self.inner_mut().set_next_epoch_class_groups_pubkey_and_proof_bytes(class_groups_pubkey_and_proof, cap)
}

/// Sets a validator's pricing vote.
/// The change will only take effects starting from the next epoch.
public fun set_pricing_vote(
    self: &mut System,
    dwallet_coordinator: &mut DWalletCoordinator,
    pricing: DWalletPricing,
    cap: &ValidatorOperationCap,
) {
    self.inner_mut().set_pricing_vote(dwallet_coordinator.inner_mut(), pricing, cap)
}

/// Getter of the pool token exchange rate of a validator. Works for both active and inactive pools.
public fun token_exchange_rates(
    self: &System,
    validator_id: ID,
): &Table<u64, TokenExchangeRate> {
    self.inner().token_exchange_rates(validator_id)
}

/// Getter returning ids of the currently active validators.
public fun active_committee(self: &mut System): BlsCommittee {
    self.inner().active_committee()
}

/// Locks the committee of the next epoch to allow starting the reconfiguration process.
public fun request_reconfig_mid_epoch(
    self: &mut System, dwallet_coordinator: &mut DWalletCoordinator, clock: &Clock, ctx: &mut TxContext
) {
    self.inner_mut().process_mid_epoch(clock, dwallet_coordinator.inner_mut(), ctx);
}

/// Locks the MPC sessions that should get completed as part of the current epoch.
public fun request_lock_epoch_sessions(
    self: &mut System, dwallet_coordinator: &mut DWalletCoordinator, clock: &Clock
) {
    self.inner_mut().lock_last_active_session_sequence_number(dwallet_coordinator.inner_mut(), clock);
}

/// Advances the epoch to the next epoch.
public fun request_advance_epoch(self: &mut System, dwallet_coordinator: &mut DWalletCoordinator, clock: &Clock, ctx: &mut TxContext) {
    let inner_system = self.inner_mut();
    let inner_dwallet = dwallet_coordinator.inner_mut();
    inner_system.advance_epoch(inner_dwallet, clock, ctx);
}

public fun request_dwallet_network_encryption_key_dkg_by_cap(
    self: &mut System,
    dwallet_2pc_mpc_coordinator: &mut DWalletCoordinator,
    cap: &ProtocolCap,
    ctx: &mut TxContext,
) {
    self.inner_mut().request_dwallet_network_encryption_key_dkg_by_cap(dwallet_2pc_mpc_coordinator.inner_mut(), cap, ctx);
}

public fun set_supported_and_pricing(
    self: &mut System,
    dwallet_2pc_mpc_coordinator: &mut DWalletCoordinator,
    default_pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap<u32, VecMap<u32, vector<u32>>>,
    protocol_cap: &ProtocolCap,
) {
    let dwallet_2pc_mpc_coordinator_inner = dwallet_2pc_mpc_coordinator.inner_mut();
    self.inner_mut().set_supported_and_pricing(dwallet_2pc_mpc_coordinator_inner, default_pricing, supported_curves_to_signature_algorithms_to_hash_schemes, protocol_cap);
}

public fun set_paused_curves_and_signature_algorithms(
    self: &mut System,
    dwallet_2pc_mpc_coordinator: &mut DWalletCoordinator,
    paused_curves: vector<u32>,
    paused_signature_algorithms: vector<u32>,
    paused_hash_schemes: vector<u32>,
    protocol_cap: &ProtocolCap,
) {
    let dwallet_2pc_mpc_coordinator_inner = dwallet_2pc_mpc_coordinator.inner_mut();
    self.inner_mut().set_paused_curves_and_signature_algorithms(dwallet_2pc_mpc_coordinator_inner, paused_curves, paused_signature_algorithms, paused_hash_schemes, protocol_cap);
}

// === Upgrades ===

public fun authorize_upgrade_by_cap(
    self: &mut System,
    cap: &ProtocolCap,
    package_id: ID,
    digest: vector<u8>,
): UpgradeTicket {
    self.inner_mut().authorize_upgrade_by_cap(cap, package_id, digest)
}

public fun authorize_upgrade_by_approval(
    self: &mut System,
    package_id: ID,
): UpgradeTicket {
    self.inner_mut().authorize_upgrade_by_approval(package_id)
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

public fun process_checkpoint_message_by_cap(
    self: &mut System,
    cap: &ProtocolCap,
    message: vector<u8>,
    ctx: &mut TxContext,
) {
    self.inner_mut().process_checkpoint_message_by_cap(cap, message, ctx);
}

public fun process_checkpoint_message_by_quorum(
    self: &mut System,
    signature: vector<u8>,
    signers_bitmap: vector<u8>,
    message: vector<u8>,
    ctx: &mut TxContext,
) {
    self.inner_mut().process_checkpoint_message_by_quorum(signature, signers_bitmap, message, ctx);
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

// === Utility functions ===

/// Calculate the rewards for an amount with value `staked_principal`, staked in the validator with
/// the given `validator_id` between `activation_epoch` and `withdraw_epoch`.
///
/// This function can be used with `dev_inspect` to calculate the expected rewards for a `StakedIka`
/// object or, more generally, the returns provided by a given validator over a given period.
public fun calculate_rewards(
    self: &System,
    validator_id: ID,
    staked_principal: u64,
    activation_epoch: u64,
    withdraw_epoch: u64,
): u64 {
    self.inner().calculate_rewards(validator_id, staked_principal, activation_epoch, withdraw_epoch)
}

/// Call `staked_ika::can_withdraw_early` to allow calling this method in applications.
public fun can_withdraw_staked_ika_early(self: &System, staked_ika: &StakedIka): bool {
    self.inner().can_withdraw_staked_ika_early(staked_ika)
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

// === Test Functions ===

#[test_only]
/// Return the current epoch number. Useful for applications that need a coarse-grained concept of time,
/// since epochs are ever-increasing and epoch changes are intended to happen every 24 hours.
public fun epoch(self: &mut System): u64 {
    self.inner().epoch()
}

#[test_only]
/// Returns unix timestamp of the start of current epoch
public fun epoch_start_timestamp_ms(self: &mut System): u64 {
    self.inner().epoch_start_timestamp_ms()
}

#[test_only]
/// Returns the total amount staked with `validator_id`.
/// Aborts if `validator_id` is not an active validator.
public fun validator_stake_amount(self: &mut System, validator_id: ID): u64 {
    self.inner_mut().validator_stake_amount(validator_id)
}

#[test_only]
use sui::vec_set::VecSet;

#[test_only]
/// Returns all the validators who are currently reporting `validator_id`
public fun get_reporters_of(self: &mut System, validator_id: ID): VecSet<ID> {
    self.inner().get_reporters_of(validator_id)
}

#[test_only]
/// Return the current validator set
public fun validator_set(self: &mut System): &ValidatorSet {
    self.inner().validator_set()
}

#[test_only]
public fun set_epoch_for_testing(self: &mut System, epoch_num: u64) {
    self.inner_mut().set_epoch_for_testing(epoch_num)
}

#[test_only]
public fun request_add_validator_for_testing(
    self: &mut System,
    cap: &ValidatorCap,
) {
    self.inner_mut().request_add_validator_for_testing(cap)
}

#[test_only]
public fun get_stake_subsidy_stake_subsidy_distribution_counter(self: &mut System): u64 {
    self.inner().get_stake_subsidy_stake_subsidy_distribution_counter()
}

#[test_only]
public fun set_stake_subsidy_stake_subsidy_distribution_counter(self: &mut System, counter: u64) {
    self.inner_mut().set_stake_subsidy_stake_subsidy_distribution_counter(counter)
}

#[test_only]
public fun inner_mut_for_testing(self: &mut System): &mut SystemInnerV1 {
    self.inner_mut()
}
