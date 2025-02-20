// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::system_inner_v1;

use ika::ika::IKA;
use ika_system::protocol_treasury::ProtocolTreasury;
use ika_system::staking_pool::{PoolTokenExchangeRate};
use ika_system::staked_ika::{StakedIka, FungibleStakedIka};
use ika_system::validator_cap::{ValidatorCap, ValidatorOperationCap};
use ika_system::validator_set::{ValidatorSet};
use ika_system::committee::{Committee};
use ika_system::protocol_cap::ProtocolCap;
use ika_system::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof;
use ika_system::dwallet_2pc_mpc_secp256k1;
use ika_system::dwallet_2pc_mpc_secp256k1_inner::{DWalletNetworkDecryptionKeyCap};
use sui::bag::{Self, Bag};
use sui::balance::{Self, Balance};
use sui::coin::Coin;
use sui::event;
use sui::table::Table;
use sui::vec_set::{VecSet};
use sui::bcs;
use sui::clock::Clock;
use sui::package::{UpgradeCap, UpgradeTicket, UpgradeReceipt};

const CHECKPOINT_MESSAGE_INTENT: vector<u8> = vector[1, 0, 0];

const BASIS_POINT_DENOMINATOR: u16 = 10000;

/// The params of the system.
public struct SystemParametersV1 has store {
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
    validators: ValidatorSet,
    /// A list of system config parameters.
    parameters: SystemParametersV1,
    /// The computation price per unit size for the current epoch.
    computation_price_per_unit_size: u64,
    /// Schedule of stake subsidies given out each epoch.
    protocol_treasury: ProtocolTreasury,
    /// Unix timestamp of the current epoch start.
    epoch_start_timestamp_ms: u64,
    /// The total messages processed.
    total_messages_processed: u64,
    /// The last checkpoint sequence number processed.
    last_processed_checkpoint_sequence_number: Option<u64>,
    /// The last checkpoint sequence number of previous epoch.
    previous_epoch_last_checkpoint_sequence_number: u64,
    /// The fees paid for computation.
    computation_reward: Balance<IKA>,
    /// List of authorized protocol cap ids.
    authorized_protocol_cap_ids: vector<ID>, 
    // TODO: maybe change that later
    dwallet_2pc_mpc_secp256k1_id: Option<ID>,
    // TODO: dummy code, change that later
    dwallet_network_decryption_key: Option<DWalletNetworkDecryptionKeyCap>,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

/// Event containing system-level epoch information, emitted during
/// the epoch advancement message.
public struct SystemEpochInfoEvent has copy, drop {
    epoch: u64,
    protocol_version: u64,
    computation_price_per_unit_size: u64,
    total_stake: u64,
    stake_subsidy_amount: u64,
    total_computation_fees: u64,
    total_stake_rewards_distributed: u64,
    last_processed_checkpoint_sequence_number: u64
}

/// Event emitted after verifing quorum of signature.
public struct SystemQuorumVerifiedEvent has copy, drop {
    epoch: u64,
    total_signers_stake: u64,
}

/// Event emitted during verifing quorum checkpoint submmision signature.
public struct SystemProtocolCapVerifiedEvent has copy, drop {
    epoch: u64,
    protocol_cap_id: ID,
}

/// Event containing system-level checkpoint information, emitted during
/// the checkpoint submmision message.
public struct SystemCheckpointInfoEvent has copy, drop {
    epoch: u64,
    sequence_number: u64,
    timestamp_ms: u64,
}

// Errors
const ELimitExceeded: u64 = 1;
const EBpsTooLarge: u64 = 5;
// const ESafeModeGasNotProcessed: u64 = 7;
const EAdvancedToWrongEpoch: u64 = 8;

#[error]
const EIncorrectEpochInCheckpoint: vector<u8> = b"The checkpoint epoch is incorrect.";

#[error]
const EUnauthorizedProtocolCap: vector<u8> = b"The protocol cap is unauthorized.";

#[error]
const EWrongCheckpointSequenceNumber: vector<u8> = b"The checkpoint sequence number should be the expected next one.";

#[error]
const EActiveCommitteeMustInitialize: vector<u8> = b"Fitst active committee must initialize.";

#[error]
const ECannotInitialize: vector<u8> = b"Too early for initialization time or alreay initialized.";

// ==== functions that can only be called by init ====

/// Create a new IkaSystemState object and make it shared.
/// This function will be called only once in init.
public(package) fun create(
    upgrade_caps: vector<UpgradeCap>,
    validators: ValidatorSet,
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
        validators,
        parameters,
        computation_price_per_unit_size: 0,
        protocol_treasury,
        epoch_start_timestamp_ms,
        total_messages_processed: 0,
        last_processed_checkpoint_sequence_number: option::none(),
        previous_epoch_last_checkpoint_sequence_number: 0,
        computation_reward: balance::zero(),
        authorized_protocol_cap_ids,
        dwallet_2pc_mpc_secp256k1_id: option::none(),
        dwallet_network_decryption_key: option::none(),
        extra_fields: bag::new(ctx),
    };
    system_state
}

public(package) fun create_system_parameters(
    epoch_duration_ms: u64,
    stake_subsidy_start_epoch: u64,
    // Validator committee parameters
    min_validator_count: u64,
    max_validator_count: u64,
    min_validator_joining_stake: u64,
    validator_low_stake_threshold: u64,
    validator_very_low_stake_threshold: u64,
    validator_low_stake_grace_period: u64,
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
        min_validator_count,
        max_validator_count,
        min_validator_joining_stake,
        validator_low_stake_threshold,
        validator_very_low_stake_threshold,
        validator_low_stake_grace_period,
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
    self.validators.initialize();
    let pricing = ika_system::dwallet_pricing::create_dwallet_pricing_2pc_mpc_secp256k1(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ctx);
    let (dwallet_2pc_mpc_secp256k1_id, cap) = dwallet_2pc_mpc_secp256k1::create(package_id, self.epoch, self.active_committee(), pricing, ctx);
    self.dwallet_2pc_mpc_secp256k1_id.fill(dwallet_2pc_mpc_secp256k1_id);
    self.dwallet_network_decryption_key.fill(cap);
}

/// Can be called by anyone who wishes to become a validator candidate and starts accuring delegated
/// stakes in their staking pool. Once they have at least `MIN_VALIDATOR_JOINING_STAKE` amount of stake they
/// can call `request_add_validator` to officially become an active validator at the next epoch.
/// Aborts if the caller is already a pending or active validator, or a validator candidate.
/// Note: `proof_of_possession_bytes` MUST be a valid signature using proof_of_possession_sender and protocol_pubkey_bytes.
/// To produce a valid PoP, run [fn test_proof_of_possession_bytes].
public(package) fun request_add_validator_candidate(
    self: &mut SystemInnerV1,
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
    self.validators.request_add_validator_candidate(
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
public(package) fun request_remove_validator_candidate(
    self: &mut SystemInnerV1,
    cap: &ValidatorCap,
) {
    self.validators.request_remove_validator_candidate(self.epoch, cap);
}

/// Called by a validator candidate to add themselves to the active validator set beginning next epoch.
/// Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
/// stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
/// epoch has already reached the maximum.
public(package) fun request_add_validator(
    self: &mut SystemInnerV1, 
    cap: &ValidatorCap,
) {
    assert!(
        self.validators.pending_active_validators_count() < self.parameters.max_validator_count,
        ELimitExceeded,
    );

    self.validators.request_add_validator(self.parameters.min_validator_joining_stake, cap);
}

/// A validator can call this function to request a removal in the next epoch.
/// We use the sender of `ctx` to look up the validator
/// (i.e. sender must match the sui_address in the validator).
/// At the end of the epoch, the `validator` object will be returned to the sui_address
/// of the validator.
public(package) fun request_remove_validator(self: &mut SystemInnerV1, cap: &ValidatorCap) {
    // Only check min validator condition if the current number of validators satisfy the constraint.
    // This is so that if we somehow already are in a state where we have less than min validators, it no longer matters
    // and is ok to stay so. This is useful for a test setup.
    if (self.active_committee().members().length() >= self.parameters.min_validator_count) {
        assert!(
            self.validators.pending_active_validators_count() > self.parameters.min_validator_count,
            ELimitExceeded,
        );
    };

    self.validators.request_remove_validator(cap)
}

/// A validator can call this function to submit a new computation price quote, to be
/// used for the computation price per unit size calculation at the end of the epoch.
public(package) fun request_set_computation_price(
    self: &mut SystemInnerV1,
    operation_cap: &ValidatorOperationCap,
    new_computation_price: u64,
) {
    // Verify that the capability is still valid.
    self.validators.verify_operation_cap(operation_cap);
    let validator = self
        .validators
        .get_validator_mut_with_operation_cap(operation_cap);

    validator.request_set_computation_price(operation_cap, new_computation_price);
}

/// This function is used to set new computation price for candidate validators
public(package) fun set_candidate_validator_computation_price(
    self: &mut SystemInnerV1,
    operation_cap: &ValidatorOperationCap,
    new_computation_price: u64,
) {
    // Verify that the capability is still valid.
    self.validators.verify_operation_cap(operation_cap);
    let candidate = self
        .validators
        .get_validator_mut_with_operation_cap_including_candidates(operation_cap);
    candidate.set_candidate_computation_price(operation_cap, new_computation_price)
}

/// A validator can call this function to set a new commission rate, updated at the end of
/// the epoch.
public(package) fun request_set_commission_rate(
    self: &mut SystemInnerV1,
    new_commission_rate: u16,
    cap: &ValidatorCap,
) {
    self
        .validators
        .request_set_commission_rate(
            new_commission_rate,
            cap,
        )
}

/// This function is used to set new commission rate for candidate validators
public(package) fun set_candidate_validator_commission_rate(
    self: &mut SystemInnerV1,
    new_commission_rate: u16,
    cap: &ValidatorCap,
) {
    let candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.set_candidate_commission_rate(new_commission_rate)
}

/// Add stake to a validator's staking pool.
public(package) fun request_add_stake(
    self: &mut SystemInnerV1,
    stake: Coin<IKA>,
    validator_id: ID,
    ctx: &mut TxContext,
): StakedIka {
    self
        .validators
        .request_add_stake(
            self.epoch,
            validator_id,
            stake.into_balance(),
            ctx,
        )
}

/// Add stake to a validator's staking pool using multiple coins.
public(package) fun request_add_stake_mul_coin(
    self: &mut SystemInnerV1,
    stakes: vector<Coin<IKA>>,
    stake_amount: option::Option<u64>,
    validator_id: ID,
    ctx: &mut TxContext,
): StakedIka {
    let balance = extract_coin_balance(stakes, stake_amount, ctx);
    self.validators.request_add_stake(self.epoch, validator_id, balance, ctx)
}

/// Withdraw some portion of a stake from a validator's staking pool.
public(package) fun request_withdraw_stake(
    self: &mut SystemInnerV1,
    staked_ika: StakedIka,
): Balance<IKA> {
    self.validators.request_withdraw_stake(self.epoch, staked_ika)
}

public(package) fun convert_to_fungible_staked_ika(
    self: &mut SystemInnerV1,
    staked_ika: StakedIka,
    ctx: &mut TxContext,
): FungibleStakedIka {
    self.validators.convert_to_fungible_staked_ika(self.epoch, staked_ika, ctx)
}

public(package) fun redeem_fungible_staked_ika(
    self: &mut SystemInnerV1,
    fungible_staked_ika: FungibleStakedIka,
): Balance<IKA> {
    self.validators.redeem_fungible_staked_ika(self.epoch, fungible_staked_ika)
}

public(package) fun report_validator(
    self: &mut SystemInnerV1,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.validators.report_validator(cap, reportee_id);
}

public(package) fun undo_report_validator(
    self: &mut SystemInnerV1,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.validators.undo_report_validator(cap, reportee_id);
}

// ==== validator metadata management functions ====

/// Create a new `ValidatorOperationCap` and registers it.
/// The original object is thus revoked.
public(package) fun rotate_operation_cap(self: &mut SystemInnerV1, cap: &ValidatorCap, ctx: &mut TxContext): ValidatorOperationCap {
    let validator = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    validator.new_validator_operation_cap(cap, ctx)
}

/// Update a validator's payment address.
public(package) fun update_validator_payment_address(
    self: &mut SystemInnerV1,
    payment_address: address,
    cap: &ValidatorCap
) {
    let validator = self.validators.get_validator_mut_with_cap_including_candidates(cap);

    validator.update_payment_address(payment_address);
}

/// Update a validator's name.
public(package) fun update_validator_name(
    self: &mut SystemInnerV1,
    name: vector<u8>,
    cap: &ValidatorCap
) {
    let validator = self.validators.get_validator_mut_with_cap_including_candidates(cap);

    validator.update_name(name);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update a validator's description
public(package) fun update_validator_description(
    self: &mut SystemInnerV1,
    description: vector<u8>,
    cap: &ValidatorCap
) {
    let validator = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    validator.update_description(description);
}

/// Update a validator's image url
public(package) fun update_validator_image_url(
    self: &mut SystemInnerV1,
    image_url: vector<u8>,
    cap: &ValidatorCap
) {
    let validator = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    validator.update_image_url(image_url);
}

/// Update a validator's project url
public(package) fun update_validator_project_url(
    self: &mut SystemInnerV1,
    project_url: vector<u8>,
    cap: &ValidatorCap
) {
    let validator = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    validator.update_project_url(project_url);
}

/// Update a validator's network address.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_network_address(
    self: &mut SystemInnerV1,
    network_address: vector<u8>,
    cap: &ValidatorCap
) {
    let validator = self.validators.get_validator_mut_with_cap(cap);
    validator.update_next_epoch_network_address(network_address);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update candidate validator's network address.
public(package) fun update_candidate_validator_network_address(
    self: &mut SystemInnerV1,
    network_address: vector<u8>,
    cap: &ValidatorCap
) {
    let candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.update_candidate_network_address(network_address);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update a validator's p2p address.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_p2p_address(
    self: &mut SystemInnerV1,
    p2p_address: vector<u8>,
    cap: &ValidatorCap
) {
    let validator = self.validators.get_validator_mut_with_cap(cap);
    validator.update_next_epoch_p2p_address(p2p_address);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update candidate validator's p2p address.
public(package) fun update_candidate_validator_p2p_address(
    self: &mut SystemInnerV1,
    p2p_address: vector<u8>,
    cap: &ValidatorCap
) {
    let candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.update_candidate_p2p_address(p2p_address);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update a validator's consensus address.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_consensus_address(
    self: &mut SystemInnerV1,
    consensus_address: vector<u8>,
    cap: &ValidatorCap
) {
    let validator = self.validators.get_validator_mut_with_cap(cap);
    validator.update_next_epoch_consensus_address(consensus_address);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update candidate validator's consensus address.
public(package) fun update_candidate_validator_consensus_address(
    self: &mut SystemInnerV1,
    consensus_address: vector<u8>,
    cap: &ValidatorCap
) {
    let candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.update_candidate_consensus_address(consensus_address);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update a validator's public key of protocol key and proof of possession.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_protocol_pubkey_bytes(
    self: &mut SystemInnerV1,
    protocol_pubkey: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    cap: &ValidatorCap,
    ctx: &TxContext,
) {
    let validator = self.validators.get_validator_mut_with_cap(cap);
    validator.update_next_epoch_protocol_pubkey_bytes(protocol_pubkey, proof_of_possession_bytes, ctx);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update candidate validator's public key of protocol key and proof of possession.
public(package) fun update_candidate_validator_protocol_pubkey_bytes(
    self: &mut SystemInnerV1,
    protocol_pubkey: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    cap: &ValidatorCap,
    ctx: &TxContext,
) {
    let candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.update_candidate_protocol_pubkey_bytes(protocol_pubkey, proof_of_possession_bytes, ctx);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update a validator's public key of worker key.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_consensus_pubkey_bytes(
    self: &mut SystemInnerV1,
    consensus_pubkey_bytes: vector<u8>,
    cap: &ValidatorCap
) {
    let validator = self.validators.get_validator_mut_with_cap(cap);
    validator.update_next_epoch_consensus_pubkey_bytes(consensus_pubkey_bytes);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update candidate validator's public key of worker key.
public(package) fun update_candidate_validator_consensus_pubkey_bytes(
    self: &mut SystemInnerV1,
    consensus_pubkey_bytes: vector<u8>,
    cap: &ValidatorCap
) {
    let candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.update_candidate_consensus_pubkey_bytes(consensus_pubkey_bytes);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update a validator's public key and its associated proof of class groups key.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_class_groups_pubkey_and_proof_bytes(
    self: &mut SystemInnerV1,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorCap,
) {
    let validator = self.validators.get_validator_mut_with_cap(cap);
    validator.update_next_epoch_class_groups_pubkey_and_proof_bytes(class_groups_pubkey_and_proof);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update candidate validator's public key and its associated proof of class groups key.
public(package) fun update_candidate_validator_class_groups_pubkey_and_proof_bytes(
    self: &mut SystemInnerV1,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorCap
) {
    let candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.update_candidate_class_groups_pubkey_and_proof_bytes(class_groups_pubkey_and_proof);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update a validator's public key of network key.
/// The change will only take effects starting from the next epoch.
public(package) fun update_validator_next_epoch_network_pubkey_bytes(
    self: &mut SystemInnerV1,
    network_pubkey: vector<u8>,
    cap: &ValidatorCap
) {
    let validator = self.validators.get_validator_mut_with_cap(cap);
    validator.update_next_epoch_network_pubkey_bytes(network_pubkey);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}

/// Update candidate validator's public key of network key.
public(package) fun update_candidate_validator_network_pubkey_bytes(
    self: &mut SystemInnerV1,
    network_pubkey: vector<u8>,
    cap: &ValidatorCap
) {
    let candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.update_candidate_network_pubkey_bytes(network_pubkey);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
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
    new_epoch: u64,
    next_protocol_version: u64,
    epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
    ctx: &mut TxContext,
) {
    let prev_epoch_start_timestamp = self.epoch_start_timestamp_ms;
    self.epoch_start_timestamp_ms = epoch_start_timestamp_ms;

    // TODO: remove this in later upgrade.
    if (self.parameters.stake_subsidy_start_epoch > 0) {
        self.parameters.stake_subsidy_start_epoch = 20;
    };

    let mut stake_subsidy = balance::zero();

    // during the transition from epoch N to epoch N + 1, self.epoch() will return N
    let epoch = self.epoch();
    // Include stake subsidy in the rewards given out to validators and stakers.
    // Delay distributing any stake subsidies until after `stake_subsidy_start_epoch`.
    // And if this epoch is shorter than the regular epoch duration, don't distribute any stake subsidy.
    if (
        epoch >= self.parameters.stake_subsidy_start_epoch  &&
            epoch_start_timestamp_ms >= prev_epoch_start_timestamp + self.parameters.epoch_duration_ms
    ) {
        stake_subsidy.join(self.protocol_treasury.stake_subsidy_for_distribution(ctx));
    };


    let computation_reward_amount_before_distribution = self.computation_reward.value();

    let stake_subsidy_amount = stake_subsidy.value();
    let mut total_reward = sui::balance::zero<IKA>();
    total_reward.join(self.computation_reward.withdraw_all());
    total_reward.join(stake_subsidy);
    let total_reward_amount_before_distribution = total_reward.value();

    self.epoch = self.epoch + 1;
    // Sanity check to make sure we are advancing to the right epoch.
    assert!(new_epoch == self.epoch, EAdvancedToWrongEpoch);

    self
        .validators
        .advance_epoch(
            epoch,
            new_epoch,
            &mut total_reward,
            self.parameters.reward_slashing_rate,

            ctx,
        );

    let new_total_stake = self.validators.total_stake();

    let total_reward_amount_after_distribution = total_reward.value();
    let total_reward_distributed =
        total_reward_amount_before_distribution - total_reward_amount_after_distribution;

    // Because of precision issues with integer divisions, we expect that there will be some
    // remaining balance in `computation_reward`.
    self.computation_reward.join(total_reward);

    self.protocol_version = next_protocol_version;

    let active_committee = self.active_committee();
    // Derive the computation price per unit size for the new epoch
    self.computation_price_per_unit_size = self.validators.derive_computation_price_per_unit_size(&active_committee);

    let last_processed_checkpoint_sequence_number = *self.last_processed_checkpoint_sequence_number.borrow();
    self.previous_epoch_last_checkpoint_sequence_number = last_processed_checkpoint_sequence_number;

    event::emit(SystemEpochInfoEvent {
        epoch: self.epoch,
        protocol_version: self.protocol_version,
        computation_price_per_unit_size: self.computation_price_per_unit_size,
        total_stake: new_total_stake,
        stake_subsidy_amount,
        total_computation_fees: computation_reward_amount_before_distribution,
        total_stake_rewards_distributed: total_reward_distributed,
        last_processed_checkpoint_sequence_number,
    });
}

public(package) fun process_mid_epoch(
    self: &mut SystemInnerV1,
) {
    self.validators.process_mid_epoch(
        self.epoch, 
        self.parameters.lock_active_committee,
        self.parameters.validator_low_stake_threshold,
        self.parameters.validator_very_low_stake_threshold,
        self.parameters.validator_low_stake_grace_period,
    );
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
    self.validators.validator_total_stake_amount(validator_id)
}

/// Returns all the validators who are currently reporting `validator_id`
public(package) fun get_reporters_of(self: &SystemInnerV1, validator_id: ID): VecSet<ID> {
    self.get_reporters_of(validator_id)
}

public(package) fun pool_exchange_rates(
    self: &mut SystemInnerV1,
    validator_id: ID,
): &Table<u64, PoolTokenExchangeRate> {
    let validators = &mut self.validators;
    validators.pool_exchange_rates(validator_id)
}

public(package) fun active_committee(self: &SystemInnerV1): Committee {
    let validator_set = &self.validators;
    validator_set.active_committee()
}

public struct TestMessageEvent has drop, copy {
    epoch: u64,
    sequence_number: u64,
    authority: u32,
    num: u64,
}

public(package) fun process_checkpoint_message_by_cap(
    self: &mut SystemInnerV1,
    cap: &ProtocolCap,
    message: vector<u8>,
    ctx: &mut TxContext,
) {
    let protocol_cap_id = object::id(cap);

    assert!(self.authorized_protocol_cap_ids.contains(&protocol_cap_id), EUnauthorizedProtocolCap);

    event::emit(SystemProtocolCapVerifiedEvent {
        epoch: self.epoch,
        protocol_cap_id: object::id(cap),
    });

    self.process_checkpoint_message(message, ctx);
}

public(package) fun process_checkpoint_message_by_quorum(
    self: &mut SystemInnerV1,
    signature: vector<u8>,
    signers_bitmap: vector<u8>,
    message: vector<u8>,
    ctx: &mut TxContext,
) {
    let mut intent_bytes = CHECKPOINT_MESSAGE_INTENT;
    intent_bytes.append(message);
    intent_bytes.append(bcs::to_bytes(&self.epoch));

    let total_signers_stake = self.active_committee().verify_certificate(&signature, &signers_bitmap, &intent_bytes);

    event::emit(SystemQuorumVerifiedEvent {
        epoch: self.epoch,
        total_signers_stake,
    });

    self.process_checkpoint_message(message, ctx);
}

fun process_checkpoint_message(
    self: &mut SystemInnerV1,
    message: vector<u8>,
    ctx: &mut TxContext,
) {
    assert!(!self.active_committee().members().is_empty(), EActiveCommitteeMustInitialize);

    // first let's make sure it's the correct checkpoint message
    let mut bcs_body = bcs::new(copy message);

    let epoch = bcs_body.peel_u64();
    assert!(epoch == self.epoch, EIncorrectEpochInCheckpoint);

    let sequence_number = bcs_body.peel_u64();

    if(self.last_processed_checkpoint_sequence_number.is_none()) {
        assert!(sequence_number == 0, EWrongCheckpointSequenceNumber);
        self.last_processed_checkpoint_sequence_number.fill(sequence_number);
    } else {
        assert!(sequence_number > 0 && *self.last_processed_checkpoint_sequence_number.borrow() + 1 == sequence_number, EWrongCheckpointSequenceNumber);
        self.last_processed_checkpoint_sequence_number.swap(sequence_number);
    };

    //let network_total_messages = bcs_body.peel_u64();
    //let previous_digest = bcs_body.peel_option!(|previous_digest| previous_digest.peel_vec_u8() );
    let timestamp_ms = bcs_body.peel_u64();

    event::emit(SystemCheckpointInfoEvent {
        epoch,
        sequence_number,
        timestamp_ms,
    });

    // now let's process message

    //assert!(false, 456);

    let len = bcs_body.peel_vec_length();
    let mut i = 0;
    while (i < len) {
        let message_data_type = bcs_body.peel_vec_length();
            if (message_data_type == 0) {
                // InitiateProcessMidEpoch
                self.process_mid_epoch();
            } else if (message_data_type == 1) {
                // EndOfEpochMessage
                let len = bcs_body.peel_vec_length();
                let mut i = 0;
                while (i < len) {
                    let end_of_epch_message_type = bcs_body.peel_vec_length();
                    // AdvanceEpoch 
                    if(end_of_epch_message_type == 0) {
                        let new_epoch = bcs_body.peel_u64();
                        let next_protocol_version = bcs_body.peel_u64();
                        let epoch_start_timestamp_ms = bcs_body.peel_u64();
                        self.advance_epoch(new_epoch, next_protocol_version, epoch_start_timestamp_ms, ctx);
                    };
                    i = i + 1;
                };
            } else if (message_data_type == 2) {
                //TestMessage
                let authority = bcs_body.peel_u32();
                let num = bcs_body.peel_u64();
                event::emit(TestMessageEvent {
                    epoch,
                    sequence_number,
                    authority,
                    num,
                });
            };
        i = i + 1;
    };
    self.total_messages_processed = self.total_messages_processed + i;
}

#[allow(lint(self_transfer))]
/// Extract required Balance from vector of Coin<IKA>, transfer the remainder back to sender.
fun extract_coin_balance(
    mut coins: vector<Coin<IKA>>,
    amount: option::Option<u64>,
    ctx: &mut TxContext,
): Balance<IKA> {
    let mut merged_coin = coins.pop_back();
    merged_coin.join_vec(coins);

    let mut total_balance = merged_coin.into_balance();
    // return the full amount if amount is not specified
    if (amount.is_some()) {
        let amount = amount.destroy_some();
        let balance = total_balance.split(amount);
        // transfer back the remainder if non zero.
        if (total_balance.value() > 0) {
            transfer::public_transfer(total_balance.into_coin(ctx), ctx.sender());
        } else {
            total_balance.destroy_zero();
        };
        balance
    } else {
        total_balance
    }
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
public(package) fun validators(self: &SystemInnerV1): &ValidatorSet {
    &self.validators
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
    min_joining_stake_for_testing: u64,
    cap: &ValidatorCap,
) {
    assert!(
        self.validators.pending_active_validators_count() < self.parameters.max_validator_count,
        ELimitExceeded,
    );

    self.validators.request_add_validator(min_joining_stake_for_testing, cap);
}

#[test_only]
public(package) fun set_stake_subsidy_stake_subsidy_distribution_counter(
    self: &mut SystemInnerV1,
    counter: u64,
) {
    self.protocol_treasury.set_stake_subsidy_distribution_counter(counter)
}

#[test_only]
public(package) fun epoch_duration_ms(self: &SystemInnerV1): u64 {
    self.parameters.epoch_duration_ms
}

// CAUTION: THIS CODE IS ONLY FOR TESTING AND THIS MACRO MUST NEVER EVER BE REMOVED.  Creates a
// candidate validator - bypassing the proof of possession check and other metadata validation
// in the process.
#[test_only]
public(package) fun request_add_validator_candidate_for_testing(
    self: &mut SystemInnerV1,
    protocol_pubkey_bytes: vector<u8>,
    network_pubkey_bytes: vector<u8>,
    consensus_pubkey_bytes_bytes: vector<u8>,
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
    let (validator, cap, operation_cap) = validator_inner_v1::new_for_testing(
        ctx.sender(),
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
        option::none(),
        computation_price,
        commission_rate,
        false, // not an initial validator active at init
        ctx,
    );

    self.validators.request_add_validator_candidate(validator, ctx);
    (cap, operation_cap)
}
