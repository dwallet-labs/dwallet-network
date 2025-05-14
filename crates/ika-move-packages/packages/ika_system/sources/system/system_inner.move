// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::system_inner;

use ika::ika::IKA;
use ika_system::protocol_treasury::ProtocolTreasury;
use ika_system::token_exchange_rate::TokenExchangeRate;
use ika_system::staked_ika::{StakedIka};
use ika_system::validator_cap::{ValidatorCap, ValidatorOperationCap, ValidatorCommissionCap};
use ika_system::validator_set::{ValidatorSet};
use ika_system::bls_committee::{BlsCommittee};
use ika_system::protocol_cap::ProtocolCap;
use ika_system::validator_metadata::ValidatorMetadata;
use ika_system::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof;
use ika_system::dwallet_2pc_mpc_coordinator::{Self, DWalletCoordinator};
use ika_system::dwallet_2pc_mpc_coordinator_inner::{DWalletNetworkDecryptionKeyCap, DWalletCoordinatorInner};
use sui::bag::{Self, Bag};
use sui::balance::{Self, Balance};
use sui::coin::Coin;
use sui::event;
use sui::table::Table;
use sui::vec_set::{VecSet};
use sui::clock::Clock;
use sui::package::{UpgradeCap, UpgradeTicket, UpgradeReceipt};
use std::string::String;
use sui::vec_map::VecMap;
const BASIS_POINT_DENOMINATOR: u16 = 10000;

/// The params of the system.
public struct SystemParametersV1 has store {
    /// The duration of an epoch, in milliseconds.
    epoch_duration_ms: u64,
    /// The starting epoch in which stake subsidies start being paid out
    stake_subsidy_start_epoch: u64,
    /// How many reward are slashed to punish a validator, in bps.
    reward_slashing_rate: u16,
    /// Lock active committee between epochs.
    lock_active_committee: bool,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

/// Uses SystemParametersV1 as the parameters.
public struct SystemInnerV1 has store {
    /// The current epoch ID, starting from 0.
    epoch: u64,
    /// The current protocol version, starting from 1.
    protocol_version: u64,
    /// Upgrade caps for this package and others like ika coin of the ika protocol.
    upgrade_caps: vector<UpgradeCap>,
    /// Contains all information about the validators.
    validator_set: ValidatorSet,
    /// A list of system config parameters.
    parameters: SystemParametersV1,
    /// Schedule of stake subsidies given out each epoch.
    protocol_treasury: ProtocolTreasury,
    /// Unix timestamp of the current epoch start.
    epoch_start_timestamp_ms: u64,
    /// The total messages processed.
    total_messages_processed: u64,
    /// The fees paid for computation.
    remaining_rewards: Balance<IKA>,
    /// List of authorized protocol cap ids.
    authorized_protocol_cap_ids: vector<ID>, 
    // TODO: maybe change that later
    dwallet_2pc_mpc_coordinator_id: Option<ID>,
    dwallet_2pc_mpc_coordinator_network_decryption_keys: vector<DWalletNetworkDecryptionKeyCap>,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

/// Event containing system-level epoch information, emitted during
/// the epoch advancement message.
public struct SystemEpochInfoEvent has copy, drop {
    epoch: u64,
    protocol_version: u64,
    total_stake: u64,
    stake_subsidy_amount: u64,
    total_computation_fees: u64,
    total_stake_rewards_distributed: u64,
}

/// Event emitted during verifing quorum checkpoint submmision signature.
public struct SystemProtocolCapVerifiedEvent has copy, drop {
    epoch: u64,
    protocol_cap_id: ID,
}

// Errors
const EBpsTooLarge: u64 = 1;
const ENextCommitteeNotSetOnAdvanceEpoch: u64 = 2;

#[error]
const EUnauthorizedProtocolCap: vector<u8> = b"The protocol cap is unauthorized.";

#[error]
const ECannotInitialize: vector<u8> = b"Too early for initialization time or already initialized.";

#[error]
const EWrongEpochState: vector<u8> = b"The system is in the wrong epoch state for the operation.";

#[error]
const EHaveNotReachedMidEpochTime: vector<u8> = b"The system has not reached the mid epoch time.";

// ==== functions that can only be called by init ====

/// Create a new IkaSystemState object and make it shared.
/// This function will be called only once in init.
public(package) fun create(
    upgrade_caps: vector<UpgradeCap>,
    validator_set: ValidatorSet,
    protocol_version: u64,
    epoch_start_timestamp_ms: u64,
    parameters: SystemParametersV1,
    protocol_treasury: ProtocolTreasury,
    authorized_protocol_cap_ids: vector<ID>,
    ctx: &mut TxContext,
): SystemInnerV1 {
    // This type is fixed as it's created at init. It should not be updated during type upgrade.
    let system_state = SystemInnerV1 {
        epoch: 0,
        protocol_version,
        upgrade_caps,
        validator_set,
        parameters,
        protocol_treasury,
        epoch_start_timestamp_ms,
        total_messages_processed: 0,
        remaining_rewards: balance::zero(),
        authorized_protocol_cap_ids,
        dwallet_2pc_mpc_coordinator_id: option::none(),
        dwallet_2pc_mpc_coordinator_network_decryption_keys: vector[],
        extra_fields: bag::new(ctx),
    };
    system_state
}

public(package) fun advance_network_keys(
    self: &SystemInnerV1, dwallet_2pc_mpc_coordinator: &mut DWalletCoordinatorInner
): Balance<IKA> {
    let mut total_reward = sui::balance::zero<IKA>();

    self.dwallet_2pc_mpc_coordinator_network_decryption_keys.do_ref!(|cap| {
        total_reward.join(dwallet_2pc_mpc_coordinator.advance_epoch_dwallet_network_decryption_key(cap));
    });
    total_reward
}

public(package) fun create_system_parameters(
    epoch_duration_ms: u64,
    stake_subsidy_start_epoch: u64,
    // Validator committee parameters
    reward_slashing_rate: u16,
    lock_active_committee: bool,
    ctx: &mut TxContext,
): SystemParametersV1 {
    // Rates can't be higher than 100%.
    assert!(
        reward_slashing_rate <= BASIS_POINT_DENOMINATOR,
        EBpsTooLarge,
    );
    SystemParametersV1 {
        epoch_duration_ms,
        stake_subsidy_start_epoch,
        reward_slashing_rate,
        lock_active_committee,
        extra_fields: bag::new(ctx),
    }
}

// ==== public(package) functions ====

public(package) fun initialize(
    self: &mut SystemInnerV1,    
    clock: &Clock,
    package_id: ID,
    ctx: &mut TxContext,
) {
    let now = clock.timestamp_ms();
    assert!(self.epoch == 0 && now >= self.epoch_start_timestamp_ms, ECannotInitialize);
    assert!(self.active_committee().members().is_empty(), ECannotInitialize);
    let pending_active_set = self.validator_set.pending_active_set();
    assert!(pending_active_set.size() >= pending_active_set.min_validator_count(), ECannotInitialize);
    // self.epoch = self.epoch + 1;
    // self.validators.initialize();

    self.validator_set.process_mid_epoch(
        self.parameters.lock_active_committee,
    );
    let pricing = ika_system::dwallet_pricing::create_dwallet_pricing_2pc_mpc_secp256k1(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ctx);
    let mut dwallet_2pc_mpc_coordinator = dwallet_2pc_mpc_coordinator::create_dwallet_coordinator(package_id, self.epoch, self.active_committee(), pricing, ctx);
    let dwallet_2pc_mpc_coordinator_inner = dwallet_2pc_mpc_coordinator.inner_mut();
    dwallet_2pc_mpc_coordinator_inner.lock_last_active_session_sequence_number();
    self.advance_epoch(dwallet_2pc_mpc_coordinator_inner, clock, ctx);

    self.dwallet_2pc_mpc_coordinator_id.fill(object::id(&dwallet_2pc_mpc_coordinator));
    dwallet_2pc_mpc_coordinator.share_dwallet_coordinator();
}

/// Can be called by anyone who wishes to become a validator candidate and starts accuring delegated
/// stakes in their staking pool. Once they have at least `MIN_VALIDATOR_JOINING_STAKE` amount of stake they
/// can call `request_add_validator` to officially become an active validator at the next epoch.
/// Aborts if the caller is already a pending or active validator, or a validator candidate.
/// Note: `proof_of_possession_bytes` MUST be a valid signature using proof_of_possession_sender and protocol_pubkey_bytes.
/// To produce a valid PoP, run [fn test_proof_of_possession_bytes].
public(package) fun request_add_validator_candidate(
    self: &mut SystemInnerV1,
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
    self.validator_set.request_add_validator_candidate(
        self.epoch,
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
public(package) fun request_remove_validator_candidate(
    self: &mut SystemInnerV1,
    cap: &ValidatorCap,
) {
    self.validator_set.request_remove_validator_candidate(self.epoch, cap);
}

/// Called by a validator candidate to add themselves to the active validator set beginning next epoch.
/// Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
/// stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
/// epoch has already reached the maximum.
public(package) fun request_add_validator(
    self: &mut SystemInnerV1, 
    cap: &ValidatorCap,
) {
    self.validator_set.request_add_validator(self.epoch, cap);
}

/// A validator can call this function to request a removal in the next epoch.
/// We use the sender of `ctx` to look up the validator
/// (i.e. sender must match the sui_address in the validator).
/// At the end of the epoch, the `validator` object will be returned to the sui_address
/// of the validator.
public(package) fun request_remove_validator(
    self: &mut SystemInnerV1,
    cap: &ValidatorCap,
) {
    self.validator_set.request_remove_validator(self.epoch, cap);
}


public(package) fun set_validator_metadata(
    self: &mut SystemInnerV1,
    cap: &ValidatorOperationCap,
    metadata: ValidatorMetadata,
) {
    self.validator_set.set_validator_metadata(cap, metadata);
}

/// A validator can call this function to set a new commission rate, updated at the end of
/// the epoch.
public(package) fun set_next_commission(
    self: &mut SystemInnerV1,
    new_commission_rate: u16,
    cap: &ValidatorOperationCap,
) {
    self
        .validator_set
        .set_next_commission(
            new_commission_rate,
            cap,
            self.epoch,
        );
}

/// Add stake to a validator's staking pool.
public(package) fun request_add_stake(
    self: &mut SystemInnerV1,
    stake: Coin<IKA>,
    validator_id: ID,
    ctx: &mut TxContext,
): StakedIka {
    self
        .validator_set
        .request_add_stake(
            self.epoch,
            validator_id,
            stake.into_balance(),
            ctx,
        )
}

/// Withdraw some portion of a stake from a validator's staking pool.
public(package) fun request_withdraw_stake(
    self: &mut SystemInnerV1,
    staked_ika: &mut StakedIka,
) {
    self.validator_set.request_withdraw_stake(staked_ika, self.epoch);
}

public(package) fun withdraw_stake(
    self: &mut SystemInnerV1,
    staked_ika: StakedIka,
    ctx: &mut TxContext,
): Coin<IKA> {
    self.validator_set.withdraw_stake(staked_ika, self.epoch, ctx)
}

// public(package) fun convert_to_fungible_staked_ika(
//     self: &mut SystemInnerV1,
//     staked_ika: StakedIka,
//     ctx: &mut TxContext,
// ): FungibleStakedIka {
//     self.validators.convert_to_fungible_staked_ika(self.epoch, staked_ika, ctx)
// }

// public(package) fun redeem_fungible_staked_ika(
//     self: &mut SystemInnerV1,
//     fungible_staked_ika: FungibleStakedIka,
// ): Balance<IKA> {
//     self.validators.redeem_fungible_staked_ika(self.epoch, fungible_staked_ika)
// }

public(package) fun report_validator(
    self: &mut SystemInnerV1,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.validator_set.report_validator(cap, reportee_id);
}

public(package) fun undo_report_validator(
    self: &mut SystemInnerV1,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.validator_set.undo_report_validator(cap, reportee_id);
}

// ==== validator metadata management functions ====

/// Create a new `ValidatorOperationCap` and registers it.
/// The original object is thus revoked.
public(package) fun rotate_operation_cap(self: &mut SystemInnerV1, cap: &ValidatorCap, ctx: &mut TxContext): ValidatorOperationCap {
    self.validator_set.rotate_operation_cap(cap, ctx)
}

public(package) fun rotate_commission_cap(self: &mut SystemInnerV1, cap: &ValidatorCap, ctx: &mut TxContext): ValidatorCommissionCap {
    self.validator_set.rotate_commission_cap(cap, ctx)
}

/// Sets a validator's name.
public(package) fun set_validator_name(
    self: &mut SystemInnerV1,
    name: String,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_validator_name(name, cap);
}

/// Sets a validator's network address.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_network_address(
    self: &mut SystemInnerV1,
    network_address: String,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_next_epoch_network_address(network_address, cap);
}

/// Sets a validator's p2p address.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_p2p_address(
    self: &mut SystemInnerV1,
    p2p_address: String,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_next_epoch_p2p_address(p2p_address, cap);
}

/// Sets a validator's consensus address.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_consensus_address(
    self: &mut SystemInnerV1,
    consensus_address: String,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_next_epoch_consensus_address(consensus_address, cap);
}


/// Sets a validator's public key of protocol key and proof of possession.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_protocol_pubkey_bytes(
    self: &mut SystemInnerV1,
    protocol_pubkey_bytes: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    cap: &ValidatorOperationCap,
    ctx: &TxContext,
) {
    self.validator_set.set_next_epoch_protocol_pubkey_bytes(protocol_pubkey_bytes, proof_of_possession_bytes, cap, ctx);
}

/// Sets a validator's public key of worker key.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_consensus_pubkey_bytes(
    self: &mut SystemInnerV1,
    consensus_pubkey_bytes: vector<u8>,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_next_epoch_consensus_pubkey_bytes(consensus_pubkey_bytes, cap);
}


/// Sets a validator's public key and its associated proof of class groups key.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_class_groups_pubkey_and_proof_bytes(
    self: &mut SystemInnerV1,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_next_epoch_class_groups_pubkey_and_proof_bytes(class_groups_pubkey_and_proof_bytes, cap);
}


/// Sets a validator's public key of network key.
/// The change will only take effects starting from the next epoch.
public(package) fun set_next_epoch_network_pubkey_bytes(
    self: &mut SystemInnerV1,
    network_pubkey_bytes: vector<u8>,
    cap: &ValidatorOperationCap
) {
    self.validator_set.set_next_epoch_network_pubkey_bytes(network_pubkey_bytes, cap);
}

/// This function should be called at the end of an epoch, and advances the system to the next epoch.
/// It does the following things:
/// 1. Add storage charge to the storage fund.
/// 2. Burn the storage rebates from the storage fund. These are already refunded to transaction sender's
///    gas coins.
/// 3. Distribute computation charge to validator stake.
/// 4. Update all validators.
public(package) fun advance_epoch(
    self: &mut SystemInnerV1,
    dwallet_coordinator: &mut DWalletCoordinatorInner,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    let now = clock.timestamp_ms();
    let last_epoch_change = self.epoch_start_timestamp_ms;

    if (self.epoch == 0) assert!(now >= last_epoch_change, EWrongEpochState)
    else assert!(now >= last_epoch_change + self.parameters.epoch_duration_ms, EWrongEpochState);
    self.epoch_start_timestamp_ms = now;

    let mut stake_subsidy = balance::zero();

    // during the transition from epoch N to epoch N + 1, self.epoch() will return N
    let current_epoch = self.epoch();
    // Include stake subsidy in the rewards given out to validators and stakers.
    // Delay distributing any stake subsidies until after `stake_subsidy_start_epoch`.
    // And if this epoch is shorter than the regular epoch duration, don't distribute any stake subsidy.
    if (
        current_epoch >= self.parameters.stake_subsidy_start_epoch
    ) {
        stake_subsidy.join(self.protocol_treasury.stake_subsidy_for_distribution(ctx));
    };

    let stake_subsidy_amount = stake_subsidy.value();

    let consensus_validation_rewards = dwallet_coordinator.advance_epoch(self.next_epoch_active_committee());
    let computation_rewards = self.advance_network_keys(dwallet_coordinator);

    let total_computation_fees = consensus_validation_rewards.value() + computation_rewards.value();

    let mut total_reward = sui::balance::zero<IKA>();
    total_reward.join(consensus_validation_rewards);
    total_reward.join(computation_rewards);
    total_reward.join(stake_subsidy);
    total_reward.join(self.remaining_rewards.withdraw_all());


    let total_reward_amount_before_distribution = total_reward.value();
    let new_epoch = current_epoch + 1;
    self.epoch = new_epoch;

    self
        .validator_set
        .advance_epoch(
            new_epoch,
            &mut total_reward,
            self.parameters.reward_slashing_rate,
        );

    let new_total_stake = self.validator_set.total_stake();

    let total_reward_amount_after_distribution = total_reward.value();
    let total_reward_distributed =
         total_reward_amount_before_distribution - total_reward_amount_after_distribution;

    // Because of precision issues with integer divisions, we expect that there will be some
    // remaining balance in `remaining_rewards`.
    self.remaining_rewards.join(total_reward);

    //let active_committee = self.active_committee();
    // // Derive the computation price per unit size for the new epoch
    //self.computation_price_per_unit_size = self.validators.derive_computation_price_per_unit_size(&active_committee);

    event::emit(SystemEpochInfoEvent {
        epoch: self.epoch,
        protocol_version: self.protocol_version,
        total_stake: new_total_stake,
        stake_subsidy_amount,
        total_computation_fees,
        total_stake_rewards_distributed: total_reward_distributed,
    });
}

public(package) fun process_mid_epoch(
    self: &mut SystemInnerV1,
    clock: &Clock,
    dwallet_coordinator_inner: &mut DWalletCoordinatorInner,
    ctx: &mut TxContext,
) {
    assert!(self.validator_set.next_epoch_active_committee().is_none() && clock.timestamp_ms() > self.epoch_start_timestamp_ms + (self.parameters.epoch_duration_ms / 2), EHaveNotReachedMidEpochTime);

    self.validator_set.process_mid_epoch(
        self.parameters.lock_active_committee,
    );
    self.dwallet_2pc_mpc_coordinator_network_decryption_keys.do_ref!(|cap| dwallet_coordinator_inner.emit_start_reshare_event(cap, ctx));
}

/// Return the current epoch number. Useful for applications that need a coarse-grained concept of time,
/// since epochs are ever-increasing and epoch changes are intended to happen every 24 hours.
public(package) fun epoch(self: &SystemInnerV1): u64 {
    self.epoch
}

public(package) fun protocol_version(self: &SystemInnerV1): u64 {
    self.protocol_version
}

public(package) fun upgrade_caps(self: &SystemInnerV1): &vector<UpgradeCap> {
    &self.upgrade_caps
}

/// Returns unix timestamp of the start of current epoch
public(package) fun epoch_start_timestamp_ms(self: &SystemInnerV1): u64 {
    self.epoch_start_timestamp_ms
}

/// Returns the total amount staked with `validator_id`.
/// Aborts if `validator_id` is not an active validator.
public(package) fun validator_stake_amount(
    self: &mut SystemInnerV1,
    validator_id: ID,
): u64 {
    self.validator_set.validator_total_stake_amount(validator_id)
}

/// Returns all the validators who are currently reporting `validator_id`
public(package) fun get_reporters_of(self: &SystemInnerV1, validator_id: ID): VecSet<ID> {
    self.get_reporters_of(validator_id)
}

public(package) fun token_exchange_rates(
    self: &SystemInnerV1,
    validator_id: ID,
): &Table<u64, TokenExchangeRate> {
    let validators = &self.validator_set;
    validators.token_exchange_rates(validator_id)
}

public(package) fun active_committee(self: &SystemInnerV1): BlsCommittee {
    let validator_set = &self.validator_set;
    validator_set.active_committee()
}

public(package) fun next_epoch_active_committee(self: &SystemInnerV1): BlsCommittee {
    let next_epoch_committee = self.validator_set.next_epoch_active_committee();
    assert!(next_epoch_committee.is_some(), ENextCommitteeNotSetOnAdvanceEpoch);
    return *next_epoch_committee.borrow()
}

fun verify_cap(
    self: &SystemInnerV1,
    cap: &ProtocolCap,
) {
    let protocol_cap_id = object::id(cap);

    assert!(self.authorized_protocol_cap_ids.contains(&protocol_cap_id), EUnauthorizedProtocolCap);

    event::emit(SystemProtocolCapVerifiedEvent {
        epoch: self.epoch,
        protocol_cap_id: object::id(cap),
    });
}

public(package) fun request_dwallet_network_decryption_key_dkg_by_cap(
    self: &mut SystemInnerV1,
    dwallet_2pc_mpc_coordinator: &mut DWalletCoordinator,
    cap: &ProtocolCap,
    ctx: &mut TxContext,
) {
    self.verify_cap(cap);
    let key_cap = dwallet_2pc_mpc_coordinator.request_dwallet_network_decryption_key_dkg(ctx);
    self.dwallet_2pc_mpc_coordinator_network_decryption_keys.push_back(key_cap);
}

public(package) fun set_supported_curves_and_signature_algorithms_and_hash_schemes(
    self: &SystemInnerV1,
    dwallet_2pc_mpc_coordinator_inner: &mut DWalletCoordinatorInner,
    supported_curves_to_signature_algorithms: VecMap<u8, vector<u8>>,
    supported_signature_algorithms_to_hash_schemes: VecMap<u8, vector<u8>>,
    protocol_cap: &ProtocolCap,
) {
    self.verify_cap(protocol_cap);
    dwallet_2pc_mpc_coordinator_inner.set_supported_curves_and_signature_algorithms_and_hash_schemes(supported_curves_to_signature_algorithms, supported_signature_algorithms_to_hash_schemes);
}

public(package) fun set_paused_curves_and_signature_algorithms(
    self: &SystemInnerV1,
    dwallet_2pc_mpc_coordinator_inner: &mut DWalletCoordinatorInner,
    paused_curves: vector<u8>,
    paused_signature_algorithms: vector<u8>,
    paused_hash_schemes: vector<u8>,
    protocol_cap: &ProtocolCap,
) {
    self.verify_cap(protocol_cap);
    dwallet_2pc_mpc_coordinator_inner.set_paused_curves_and_signature_algorithms(paused_curves, paused_signature_algorithms, paused_hash_schemes);
}

public(package) fun authorize_update_message_by_cap(
    self: &mut SystemInnerV1,
    cap: &ProtocolCap,
    package_id: ID,
    digest: vector<u8>,
): UpgradeTicket {
    let protocol_cap_id = object::id(cap);

    assert!(self.authorized_protocol_cap_ids.contains(&protocol_cap_id), EUnauthorizedProtocolCap);

    event::emit(SystemProtocolCapVerifiedEvent {
        epoch: self.epoch,
        protocol_cap_id: object::id(cap),
    });

    self.authorize_update_message(package_id, digest)
}

fun authorize_update_message(
    self: &mut SystemInnerV1,
    package_id: ID,
    digest: vector<u8>,
): UpgradeTicket  {
    let index = self.upgrade_caps.find_index!(|c| c.package() == package_id).extract();
    let policy = self.upgrade_caps[index].policy();
    self.upgrade_caps[index].authorize(policy, digest)
}

public(package) fun commit_upgrade(
    self: &mut SystemInnerV1,
    receipt: UpgradeReceipt,
): ID {
    let receipt_cap_id = receipt.cap();
    let index = self.upgrade_caps.find_index!(|c| object::id(c) == receipt_cap_id).extract();
    let old_package_id = self.upgrade_caps[index].package();
    self.upgrade_caps[index].commit(receipt);
    old_package_id
}

#[test_only]
/// Return the current validator set
public(package) fun validator_set(self: &SystemInnerV1): &ValidatorSet {
    &self.validator_set
}

#[test_only]
public(package) fun get_stake_subsidy_stake_subsidy_distribution_counter(self: &SystemInnerV1): u64 {
    self.protocol_treasury.get_stake_subsidy_distribution_counter()
}

#[test_only]
public(package) fun set_epoch_for_testing(self: &mut SystemInnerV1, epoch_num: u64) {
    self.epoch = epoch_num
}

#[test_only]
public(package) fun request_add_validator_for_testing(
    self: &mut SystemInnerV1,
    cap: &ValidatorCap,
) {
    self.validator_set.request_add_validator(self.epoch, cap);
}

#[test_only]
public(package) fun set_stake_subsidy_stake_subsidy_distribution_counter(
    self: &mut SystemInnerV1,
    counter: u64,
) {
    self.protocol_treasury.set_stake_subsidy_distribution_counter(counter)
}

public(package) fun epoch_duration_ms(self: &SystemInnerV1): u64 {
    self.parameters.epoch_duration_ms
}

// // CAUTION: THIS CODE IS ONLY FOR TESTING AND THIS MACRO MUST NEVER EVER BE REMOVED.  Creates a
// // candidate validator - bypassing the proof of possession check and other metadata validation
// // in the process.
// #[test_only]
// public(package) fun request_add_validator_candidate_for_testing(
//     self: &mut SystemInnerV1,
//     protocol_pubkey_bytes: vector<u8>,
//     network_pubkey_bytes: vector<u8>,
//     consensus_pubkey_bytes_bytes: vector<u8>,
//     class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
//     proof_of_possession_bytes: vector<u8>,
//     name: vector<u8>,
//     description: vector<u8>,
//     image_url: vector<u8>,
//     project_url: vector<u8>,
//     network_address: vector<u8>,
//     p2p_address: vector<u8>,
//     consensus_address: vector<u8>,
//     computation_price: u64,
//     commission_rate: u16,
//     ctx: &mut TxContext,
// ): (ValidatorCap, ValidatorOperationCap) {
//     let (validator, cap, operation_cap) = validator_inner_v1::new_for_testing(
//         ctx.sender(),
//         protocol_pubkey_bytes,
//         network_pubkey_bytes,
//         consensus_pubkey_bytes_bytes,
//         class_groups_pubkey_and_proof_bytes,
//         proof_of_possession_bytes,
//         name,
//         description,
//         image_url,
//         project_url,
//         network_address,
//         p2p_address,
//         consensus_address,
//         option::none(),
//         computation_price,
//         commission_rate,
//         false, // not an initial validator active at init
//         ctx,
//     );

//     self.validators.request_add_validator_candidate(validator, ctx);
//     (cap, operation_cap)
// }
