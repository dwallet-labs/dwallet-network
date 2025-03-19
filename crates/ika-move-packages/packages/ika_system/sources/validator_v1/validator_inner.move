// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_const)]
module ika_system::validator_inner_v1;

use ika::ika::IKA;
use ika_system::staking_pool::{
    Self,
    PoolTokenExchangeRate,
    StakingPool,
};
use ika_system::staked_ika::{
    StakedIka,
    FungibleStakedIka
};
use ika_system::validator_cap::{Self, ValidatorCap, ValidatorOperationCap };
use ika_system::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof;
use std::string::String;
use sui::bag::{Self, Bag};
use sui::balance::Balance;
use sui::event;
use sui::url::{Self, Url};
use sui::{bls12381::{UncompressedG1, g1_from_bytes, g1_to_uncompressed_g1, bls12381_min_pk_verify}, group_ops::Element};
use sui::bcs;
use sui::table_vec::TableVec;

#[error]
const EInvalidProofOfPossession: vector<u8> = b"Invalid proof_of_possession_bytes field in ValidatorMetadata.";

#[error]
const EMetadataInvalidProtocolPubkey: vector<u8> = b"Invalid protocol_pubkey_bytes field in ValidatorMetadata.";

#[error]
const EMetadataInvalidNetworkPubkey: vector<u8> = b"Invalid network_pubkey_bytes field in ValidatorMetadata.";

#[error]
const EMetadataInvalidConsensusPubkey: vector<u8> = b"Invalid consensus_pubkey_bytes field in ValidatorMetadata.";

#[error]
const EMetadataInvalidClassGroupsPubkey: vector<u8> = b"Invalid class_groups_pubkey_and_proof_bytes field in ValidatorMetadata.";

#[error]
const EMetadataInvalidNetworkAddress: vector<u8> = b"Invalid network_address field in ValidatorMetadata.";

#[error]
const EMetadataInvalidP2pAddress: vector<u8> = b"Invalid p2p_address field in ValidatorMetadata.";

#[error]
const EMetadataInvalidConsensusAddress: vector<u8> = b"Invalid consensus_address field in ValidatorMetadata.";

#[error]
const ECommissionRateTooHigh: vector<u8> = b"Commission rate set by the validator is higher than the threshold.";

#[error]
const EValidatorMetadataExceedingLengthLimit: vector<u8> = b"Validator Metadata is too long.";

#[error]
const ENotValidatorCandidate: vector<u8> = b"Intended validator is not a candidate one.";

#[error]
const EInvalidStakeAmount: vector<u8> = b"Stake amount is invalid or wrong.";

#[error]
const EInactiveValidator: vector<u8> = b"The validator is inactive.";

#[error]
const ENewCapNotCreatedByValidatorItself: vector<u8> = b"New Capability is not created by the validator itself.";

#[error]
const EInvalidCap: vector<u8> = b"Cap is not valid.";

#[error]
const EGasPriceHigherThanThreshold: vector<u8> = b"Validator trying to set computation price higher than threshold.";

// TODO: potentially move this value to onchain config.
const MAX_COMMISSION_RATE: u16 = 2_000; // Max rate is 20%, which is 2000 base points

const MAX_VALIDATOR_METADATA_LENGTH: u64 = 256;

// TODO: Move this to onchain config when we have a good way to do it.
/// Max computation price a validator can set is 100K NIKA.
const MAX_VALIDATOR_COMPUTATION_PRICE: u64 = 100_000;


const PROOF_OF_POSSESSION_INTENT: vector<u8> = vector[0, 0, 0];
const DEFAULT_EPOCH_ID: u64 = 0;

const BLS_KEY_LEN: u64 = 48;
const ED25519_KEY_LEN: u64 = 32;
const CLASS_GROUPS_BYTES_LEN: u64 = 241722; // Todo (#): change the way we implement this.

public struct ValidatorMetadata has store {
    /// The address to receive the payments
    payment_address: address,
    /// The address of the proof of possesion sender
    proof_of_possession_sender: address,
    /// The public key bytes corresponding to the private key that the validator
    /// holds to sign checkpoint messages.
    protocol_pubkey_bytes: vector<u8>,
    protocol_pubkey: Element<UncompressedG1>,
    /// This is a proof that the validator has ownership of the protocol private key
    proof_of_possession_bytes: vector<u8>,
    /// The public key bytes corresponding to the private key that the validator
    /// uses to establish TLS connections
    network_pubkey_bytes: vector<u8>,
    /// The public key bytes correstponding to the consensus
    consensus_pubkey_bytes: vector<u8>,
    /// The validator's Class Groups public key and its associated proof.  
    /// This key is used for the network DKG process and for resharing the network MPC key.
    class_groups_pubkey_and_proof_bytes: TableVec<vector<u8>>,

    /// A unique human-readable name of this validator.
    name: String,
    description: String,
    image_url: Url,
    project_url: Url,
    /// The network address of the validator (could also contain extra info such as port, DNS and etc.).
    network_address: String,
    /// The address of the validator used for p2p activities such as state sync (could also contain extra info such as port, DNS and etc.).
    p2p_address: String,
    /// The address of the consensus
    consensus_address: String,
    /// "next_epoch" metadata only takes effects in the next epoch.
    /// If none, current value will stay unchanged.
    next_epoch_protocol_pubkey_bytes: Option<vector<u8>>,
    next_epoch_proof_of_possession_bytes: Option<vector<u8>>,
    next_epoch_network_pubkey_bytes: Option<vector<u8>>,
    next_epoch_consensus_pubkey_bytes: Option<vector<u8>>,
    next_epoch_class_groups_pubkey_and_proof_bytes: Option<ClassGroupsPublicKeyAndProof>,
    next_epoch_network_address: Option<String>,
    next_epoch_p2p_address: Option<String>,
    next_epoch_consensus_address: Option<String>,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

public struct ValidatorInnerV1 has store {
    validator_id: ID,
    /// Summary of the validator.
    metadata: ValidatorMetadata,
    /// The ID of this validator's `ValidatorCap`
    cap_id: ID,
    /// The ID of this validator's current valid `UnverifiedValidatorOperationCap`
    operation_cap_id: ID,
    /// Gas price quote, updated only at end of epoch.
    computation_price: u64,
    /// Staking pool for this validator.
    staking_pool: StakingPool,
    /// Commission rate of the validator, in basis point.
    commission_rate: u16,
    /// Total amount of stake that would be active in the next epoch.
    next_epoch_stake: u64,
    /// This validator's computation price quote for the next epoch.
    next_epoch_computation_price: u64,
    /// The commission rate of the validator starting the next epoch, in basis point.
    next_epoch_commission_rate: u16,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

/// Event emitted when a new stake request is received.
public struct StakingRequestEvent has copy, drop {
    validator_id: ID,
    staked_ika_id: ID,
    epoch: u64,
    amount: u64,
}

/// Event emitted when a new unstake request is received.
public struct UnstakingRequestEvent has copy, drop {
    validator_id: ID,
    staked_ika_id: ID,
    stake_activation_epoch: u64,
    unstaking_epoch: u64,
    principal_amount: u64,
    reward_amount: u64,
}

/// Event emitted when a staked IKA is converted to a fungible staked IKA.
public struct ConvertingToFungibleStakedIkaEvent has copy, drop {
    validator_id: ID,
    stake_activation_epoch: u64,
    staked_ika_principal_amount: u64,
    fungible_staked_ika_amount: u64,
}

/// Event emitted when a fungible staked IKA is redeemed.
public struct RedeemingFungibleStakedIkaEvent has copy, drop {
    validator_id: ID,
    fungible_staked_ika_amount: u64,
    ika_amount: u64,
}

public(package) fun create_metadata(
    payment_address: address,
    protocol_pubkey_bytes: vector<u8>,
    network_pubkey_bytes: vector<u8>,
    consensus_pubkey_bytes: vector<u8>,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    proof_of_possession_bytes: vector<u8>,
    name: String,
    description: String,
    image_url: Url,
    project_url: Url,
    network_address: String,
    p2p_address: String,
    consensus_address: String,
    extra_fields: Bag,
    ctx: &TxContext,
): ValidatorMetadata {
    let protocol_pubkey = g1_to_uncompressed_g1(&g1_from_bytes(&protocol_pubkey_bytes));
    let class_groups_pubkey_and_proof_bytes = class_groups_pubkey_and_proof_bytes.destroy();
    let metadata = ValidatorMetadata {
        payment_address,
        proof_of_possession_sender: ctx.sender(),
        protocol_pubkey_bytes,
        protocol_pubkey,
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
        next_epoch_protocol_pubkey_bytes: option::none(),
        next_epoch_network_pubkey_bytes: option::none(),
        next_epoch_consensus_pubkey_bytes: option::none(),
        next_epoch_class_groups_pubkey_and_proof_bytes: option::none(),
        next_epoch_proof_of_possession_bytes: option::none(),
        next_epoch_network_address: option::none(),
        next_epoch_p2p_address: option::none(),
        next_epoch_consensus_address: option::none(),
        extra_fields,
    };
    metadata
}

public(package) fun create(
    validator_id: ID,
    cap_id: ID,
    operation_cap_id: ID,
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
): ValidatorInnerV1 {
    assert!(commission_rate <= MAX_COMMISSION_RATE, ECommissionRateTooHigh);
    assert!(computation_price < MAX_VALIDATOR_COMPUTATION_PRICE, EGasPriceHigherThanThreshold);

    let metadata = create_metadata(
        payment_address,
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        class_groups_pubkey_and_proof_bytes,
        proof_of_possession_bytes,
        name.to_ascii_string().to_string(),
        description.to_ascii_string().to_string(),
        url::new_unsafe_from_bytes(image_url),
        url::new_unsafe_from_bytes(project_url),
        network_address.to_ascii_string().to_string(),
        p2p_address.to_ascii_string().to_string(),
        consensus_address.to_ascii_string().to_string(),
        bag::new(ctx),
        ctx,
    );

    // Checks that the keys & addresses & PoP are valid.
    validate_metadata(&metadata);

    create_from_metadata(
        validator_id,
        cap_id,
        operation_cap_id,
        metadata,
        computation_price,
        commission_rate,
        ctx,
    )
}

/// Deactivate this validator's staking pool
public(package) fun deactivate(self: &mut ValidatorInnerV1, deactivation_epoch: u64) {
    self.staking_pool.deactivate_staking_pool(deactivation_epoch)
}

public(package) fun activate(self: &mut ValidatorInnerV1, activation_epoch: u64) {
    self.staking_pool.activate_staking_pool(activation_epoch);
}

/// Process pending stake and pending withdraws, and update the computation price.
public(package) fun adjust_stake_and_computation_price(self: &mut ValidatorInnerV1) {
    self.computation_price = self.next_epoch_computation_price;
    self.commission_rate = self.next_epoch_commission_rate;
}

/// Request to add stake to the validator's staking pool, processed at the end of the epoch.
public(package) fun request_add_stake(
    self: &mut ValidatorInnerV1,
    epoch: u64,
    stake: Balance<IKA>,
    ctx: &mut TxContext,
): StakedIka {
    let stake_amount = stake.value();
    let validator_id = self.validator_id();
    assert!(stake_amount > 0, EInvalidStakeAmount);
    let stake_epoch = epoch + 1;
    let staked_ika = self.staking_pool.request_add_stake(stake, stake_epoch, validator_id, ctx);
    // Process stake right away if staking pool is preactive.
    if (self.staking_pool.is_candidate()) {
        self.staking_pool.process_pending_stake();
    };
    self.next_epoch_stake = self.next_epoch_stake + stake_amount;
    event::emit(StakingRequestEvent {
        validator_id,
        staked_ika_id: object::id(&staked_ika),
        epoch: epoch,
        amount: stake_amount,
    });
    staked_ika
}

public(package) fun convert_to_fungible_staked_ika(
    self: &mut ValidatorInnerV1,
    epoch: u64,
    staked_ika: StakedIka,
    ctx: &mut TxContext,
): FungibleStakedIka {
    let stake_activation_epoch = staked_ika.stake_activation_epoch();
    let staked_ika_principal_amount = staked_ika.staked_ika_amount();

    let fungible_staked_ika = self.staking_pool.convert_to_fungible_staked_ika(epoch, staked_ika, ctx);

    event::emit(ConvertingToFungibleStakedIkaEvent {
        validator_id: self.validator_id(),
        stake_activation_epoch,
        staked_ika_principal_amount,
        fungible_staked_ika_amount: fungible_staked_ika.value(),
    });

    fungible_staked_ika
}

public(package) fun redeem_fungible_staked_ika(
    self: &mut ValidatorInnerV1,
    epoch: u64,
    fungible_staked_ika: FungibleStakedIka,
): Balance<IKA> {
    let fungible_staked_ika_amount = fungible_staked_ika.value();

    let ika = self.staking_pool.redeem_fungible_staked_ika(epoch, fungible_staked_ika);

    self.next_epoch_stake = self.next_epoch_stake - ika.value();

    event::emit(RedeemingFungibleStakedIkaEvent {
        validator_id: self.validator_id(),
        fungible_staked_ika_amount,
        ika_amount: ika.value(),
    });

    ika
}

/// Request to withdraw stake from the validator's staking pool, processed at the end of the epoch.
public(package) fun request_withdraw_stake(
    self: &mut ValidatorInnerV1,
    epoch: u64,
    staked_ika: StakedIka,
): Balance<IKA> {
    let principal_amount = staked_ika.staked_ika_amount();
    let stake_activation_epoch = staked_ika.stake_activation_epoch();
    let staked_ika_id = object::id(&staked_ika);
    let withdrawn_stake = self.staking_pool.request_withdraw_stake(epoch, staked_ika);
    let withdraw_amount = withdrawn_stake.value();
    let reward_amount = withdraw_amount - principal_amount;
    self.next_epoch_stake = self.next_epoch_stake - withdraw_amount;
    event::emit(UnstakingRequestEvent {
        validator_id: self.validator_id(),
        staked_ika_id,
        stake_activation_epoch,
        unstaking_epoch: epoch,
        principal_amount,
        reward_amount,
    });
    withdrawn_stake
}

/// Request to set new computation price for the next epoch.
/// Need to present a `ValidatorOperationCap`.
public(package) fun request_set_computation_price(
    self: &mut ValidatorInnerV1,
    operation_cap: &ValidatorOperationCap,
    new_price: u64,
) {
    assert!(!is_inactive(self), EInactiveValidator);
    assert!(new_price < MAX_VALIDATOR_COMPUTATION_PRICE, EGasPriceHigherThanThreshold);
    let validator_id = operation_cap.validator_id();
    assert!(validator_id == self.validator_id(), EInvalidCap);
    assert!(object::id(operation_cap) == self.operation_cap_id(), EInvalidCap);
    self.next_epoch_computation_price = new_price;
}

/// Set new computation price for the candidate validator.
public(package) fun set_candidate_computation_price(
    self: &mut ValidatorInnerV1,
    operation_cap: &ValidatorOperationCap,
    new_price: u64,
) {
    assert!(is_candidate(self), ENotValidatorCandidate);
    assert!(new_price < MAX_VALIDATOR_COMPUTATION_PRICE, EGasPriceHigherThanThreshold);
    let validator_id = operation_cap.validator_id();
    assert!(validator_id == self.validator_id(), EInvalidCap);
    assert!(object::id(operation_cap) == self.operation_cap_id(), EInvalidCap);
    self.next_epoch_computation_price = new_price;
    self.computation_price = new_price;
}

/// Request to set new commission rate for the next epoch.
public(package) fun request_set_commission_rate(self: &mut ValidatorInnerV1, new_commission_rate: u16) {
    assert!(!is_inactive(self), EInactiveValidator);
    assert!(new_commission_rate <= MAX_COMMISSION_RATE, ECommissionRateTooHigh);
    self.next_epoch_commission_rate = new_commission_rate;
}

/// Set new commission rate for the candidate validator.
public(package) fun set_candidate_commission_rate(self: &mut ValidatorInnerV1, new_commission_rate: u16) {
    assert!(is_candidate(self), ENotValidatorCandidate);
    assert!(new_commission_rate <= MAX_COMMISSION_RATE, ECommissionRateTooHigh);
    self.next_epoch_commission_rate = new_commission_rate;
    self.commission_rate = new_commission_rate;
}

/// Deposit stakes rewards into the validator's staking pool, called at the end of the epoch.
public(package) fun deposit_stake_rewards(self: &mut ValidatorInnerV1, reward: Balance<IKA>) {
    self.next_epoch_stake = self.next_epoch_stake + reward.value();
    self.staking_pool.deposit_rewards(reward);
}

/// Process pending stakes and withdraws, called at the end of the epoch.
public(package) fun process_pending_stakes_and_withdraws(self: &mut ValidatorInnerV1, new_epoch: u64) {
    self.staking_pool.process_pending_stakes_and_withdraws(new_epoch);
    // TODO: bring this assertion back when we are ready.
    assert!(self.total_stake_amount() == self.next_epoch_stake, EInvalidStakeAmount);
}

/// Returns true if the validator is candidate.
public fun is_candidate(self: &ValidatorInnerV1): bool {
    self.staking_pool.is_candidate()
}

/// Returns true if the validator is inactive.
public fun is_inactive(self: &ValidatorInnerV1): bool {
    self.staking_pool.is_inactive()
}

public fun validator_id(self: &ValidatorInnerV1): ID {
    self.validator_id
}

public fun metadata(self: &ValidatorInnerV1): &ValidatorMetadata {
    &self.metadata
}

public fun payment_address(self: &ValidatorInnerV1): address {
    self.metadata.payment_address
}

public fun name(self: &ValidatorInnerV1): &String {
    &self.metadata.name
}

public fun description(self: &ValidatorInnerV1): &String {
    &self.metadata.description
}

public fun image_url(self: &ValidatorInnerV1): &Url {
    &self.metadata.image_url
}

public fun project_url(self: &ValidatorInnerV1): &Url {
    &self.metadata.project_url
}

public fun network_address(self: &ValidatorInnerV1): &String {
    &self.metadata.network_address
}

public fun p2p_address(self: &ValidatorInnerV1): &String {
    &self.metadata.p2p_address
}

public fun consensus_address(self: &ValidatorInnerV1): &String {
    &self.metadata.consensus_address
}

public fun protocol_pubkey_bytes(self: &ValidatorInnerV1): &vector<u8> {
    &self.metadata.protocol_pubkey_bytes
}

public fun protocol_pubkey(self: &ValidatorInnerV1): &Element<UncompressedG1> {
    &self.metadata.protocol_pubkey
}

public fun proof_of_possession_bytes(self: &ValidatorInnerV1): &vector<u8> {
    &self.metadata.proof_of_possession_bytes
}

public fun network_pubkey_bytes(self: &ValidatorInnerV1): &vector<u8> {
    &self.metadata.network_pubkey_bytes
}

public fun consensus_pubkey_bytes(self: &ValidatorInnerV1): &vector<u8> {
    &self.metadata.consensus_pubkey_bytes
}

public fun class_groups_pubkey_and_proof_bytes(self: &ValidatorInnerV1): &TableVec<vector<u8>> {
    &self.metadata.class_groups_pubkey_and_proof_bytes
}

public fun next_epoch_network_address(self: &ValidatorInnerV1): &Option<String> {
    &self.metadata.next_epoch_network_address
}

public fun next_epoch_p2p_address(self: &ValidatorInnerV1): &Option<String> {
    &self.metadata.next_epoch_p2p_address
}

public fun next_epoch_consensus_address(self: &ValidatorInnerV1): &Option<String> {
    &self.metadata.next_epoch_consensus_address
}

public fun next_epoch_protocol_pubkey_bytes(self: &ValidatorInnerV1): &Option<vector<u8>> {
    &self.metadata.next_epoch_protocol_pubkey_bytes
}

public fun next_epoch_proof_of_possession_bytes(self: &ValidatorInnerV1): &Option<vector<u8>> {
    &self.metadata.next_epoch_proof_of_possession_bytes
}

public fun next_epoch_network_pubkey_bytes(self: &ValidatorInnerV1): &Option<vector<u8>> {
    &self.metadata.next_epoch_network_pubkey_bytes
}

public fun next_epoch_consensus_pubkey_bytes(self: &ValidatorInnerV1): &Option<vector<u8>> {
    &self.metadata.next_epoch_consensus_pubkey_bytes
}

public fun next_epoch_class_groups_pubkey_and_proof_bytes(self: &ValidatorInnerV1): &Option<ClassGroupsPublicKeyAndProof> {
    &self.metadata.next_epoch_class_groups_pubkey_and_proof_bytes
}

public fun operation_cap_id(self: &ValidatorInnerV1): &ID {
    &self.operation_cap_id
}

public fun next_epoch_computation_price(self: &ValidatorInnerV1): u64 {
    self.next_epoch_computation_price
}

/// Return the total amount staked with this validator

public fun total_stake_amount(self: &ValidatorInnerV1): u64 {
    self.staking_pool.ika_balance()
}

public fun pending_stake_amount(self: &ValidatorInnerV1): u64 {
    self.staking_pool.pending_stake_amount()
}

public fun pending_stake_withdraw_amount(self: &ValidatorInnerV1): u64 {
    self.staking_pool.pending_stake_withdraw_amount()
}

public fun computation_price(self: &ValidatorInnerV1): u64 {
    self.computation_price
}

public fun commission_rate(self: &ValidatorInnerV1): u16 {
    self.commission_rate
}

public fun pool_token_exchange_rate_at_epoch(self: &ValidatorInnerV1, epoch: u64): PoolTokenExchangeRate {
    self.staking_pool.pool_token_exchange_rate_at_epoch(epoch)
}

// MUSTFIX: We need to check this when updating metadata as well.
public fun is_duplicate(self: &ValidatorInnerV1, other: &ValidatorInnerV1): bool {
            self.metadata.name == other.metadata.name
            || self.metadata.network_address == other.metadata.network_address
            || self.metadata.p2p_address == other.metadata.p2p_address
            || self.metadata.protocol_pubkey_bytes == other.metadata.protocol_pubkey_bytes
            || self.metadata.network_pubkey_bytes == other.metadata.network_pubkey_bytes
            || self.metadata.network_pubkey_bytes == other.metadata.consensus_pubkey_bytes
            || self.metadata.consensus_pubkey_bytes == other.metadata.consensus_pubkey_bytes
            || self.metadata.consensus_pubkey_bytes == other.metadata.network_pubkey_bytes
            // All next epoch parameters.
            || is_equal_some(&self.metadata.next_epoch_network_address, &other.metadata.next_epoch_network_address)
            || is_equal_some(&self.metadata.next_epoch_p2p_address, &other.metadata.next_epoch_p2p_address)
            || is_equal_some(&self.metadata.next_epoch_protocol_pubkey_bytes, &other.metadata.next_epoch_protocol_pubkey_bytes)
            || is_equal_some(&self.metadata.next_epoch_network_pubkey_bytes, &other.metadata.next_epoch_network_pubkey_bytes)
            || is_equal_some(&self.metadata.next_epoch_network_pubkey_bytes, &other.metadata.next_epoch_consensus_pubkey_bytes)
            || is_equal_some(&self.metadata.next_epoch_consensus_pubkey_bytes, &other.metadata.next_epoch_consensus_pubkey_bytes)
            || is_equal_some(&self.metadata.next_epoch_consensus_pubkey_bytes, &other.metadata.next_epoch_network_pubkey_bytes)
            // My next epoch parameters with other current epoch parameters.
            || is_equal_some_and_value(&self.metadata.next_epoch_network_address, &other.metadata.network_address)
            || is_equal_some_and_value(&self.metadata.next_epoch_p2p_address, &other.metadata.p2p_address)
            || is_equal_some_and_value(&self.metadata.next_epoch_protocol_pubkey_bytes, &other.metadata.protocol_pubkey_bytes)
            || is_equal_some_and_value(&self.metadata.next_epoch_network_pubkey_bytes, &other.metadata.network_pubkey_bytes)
            || is_equal_some_and_value(&self.metadata.next_epoch_network_pubkey_bytes, &other.metadata.consensus_pubkey_bytes)
            || is_equal_some_and_value(&self.metadata.next_epoch_consensus_pubkey_bytes, &other.metadata.consensus_pubkey_bytes)
            || is_equal_some_and_value(&self.metadata.next_epoch_consensus_pubkey_bytes, &other.metadata.network_pubkey_bytes)
            // Other next epoch parameters with my current epoch parameters.
            || is_equal_some_and_value(&other.metadata.next_epoch_network_address, &self.metadata.network_address)
            || is_equal_some_and_value(&other.metadata.next_epoch_p2p_address, &self.metadata.p2p_address)
            || is_equal_some_and_value(&other.metadata.next_epoch_protocol_pubkey_bytes, &self.metadata.protocol_pubkey_bytes)
            || is_equal_some_and_value(&other.metadata.next_epoch_network_pubkey_bytes, &self.metadata.network_pubkey_bytes)
            || is_equal_some_and_value(&other.metadata.next_epoch_network_pubkey_bytes, &self.metadata.consensus_pubkey_bytes)
            || is_equal_some_and_value(&other.metadata.next_epoch_consensus_pubkey_bytes, &self.metadata.consensus_pubkey_bytes)
            || is_equal_some_and_value(&other.metadata.next_epoch_consensus_pubkey_bytes, &self.metadata.network_pubkey_bytes)
}

fun is_equal_some_and_value<T>(a: &Option<T>, b: &T): bool {
    if (a.is_none()) {
        false
    } else {
        a.borrow() == b
    }
}

fun is_equal_some<T>(a: &Option<T>, b: &Option<T>): bool {
    if (a.is_none() || b.is_none()) {
        false
    } else {
        a.borrow() == b.borrow()
    }
}

// ==== Validator Metadata Management Functions ====

/// Create a new `ValidatorOperationCap`, and registers it,
/// thus revoking the previous cap's permission.
public(package) fun new_validator_operation_cap(
    self: &mut ValidatorInnerV1,
    cap: &ValidatorCap,
    ctx: &mut TxContext,
): ValidatorOperationCap {
    let validator_id = cap.validator_id();
    assert!(validator_id == self.validator_id(), ENewCapNotCreatedByValidatorItself);
    let operation_cap = validator_cap::new_validator_operation_cap(validator_id, ctx);
    self.operation_cap_id = object::id(&operation_cap);
    operation_cap
}

/// Update payment address of the validator.
public(package) fun update_payment_address(self: &mut ValidatorInnerV1, payment_address: address) {
    self.metadata.payment_address = payment_address;
    validate_metadata(&self.metadata);
}

/// Update name of the validator.
public(package) fun update_name(self: &mut ValidatorInnerV1, name: vector<u8>) {
    self.metadata.name = name.to_ascii_string().to_string();
    validate_metadata(&self.metadata);
}

/// Update description of the validator.
public(package) fun update_description(self: &mut ValidatorInnerV1, description: vector<u8>) {
    self.metadata.description = description.to_ascii_string().to_string();
    validate_metadata(&self.metadata);
}

/// Update image url of the validator.
public(package) fun update_image_url(self: &mut ValidatorInnerV1, image_url: vector<u8>) {
    self.metadata.image_url = url::new_unsafe_from_bytes(image_url);
    validate_metadata(&self.metadata);
}

/// Update project url of the validator.
public(package) fun update_project_url(self: &mut ValidatorInnerV1, project_url: vector<u8>) {
    self.metadata.project_url = url::new_unsafe_from_bytes(project_url);
    validate_metadata(&self.metadata);
}

/// Update network address of this validator, taking effects from next epoch
public(package) fun update_next_epoch_network_address(
    self: &mut ValidatorInnerV1,
    network_address: vector<u8>,
) {
    assert!(!is_inactive(self), EInactiveValidator);
    let network_address = network_address.to_ascii_string().to_string();
    self.metadata.next_epoch_network_address = option::some(network_address);
    validate_metadata(&self.metadata);
}

/// Update network address of this candidate validator
public(package) fun update_candidate_network_address(
    self: &mut ValidatorInnerV1,
    network_address: vector<u8>,
) {
    assert!(is_candidate(self), ENotValidatorCandidate);
    let network_address = network_address.to_ascii_string().to_string();
    self.metadata.network_address = network_address;
    validate_metadata(&self.metadata);
}

/// Update p2p address of this validator, taking effects from next epoch
public(package) fun update_next_epoch_p2p_address(self: &mut ValidatorInnerV1, p2p_address: vector<u8>) {
    assert!(!is_inactive(self), EInactiveValidator);
    let p2p_address = p2p_address.to_ascii_string().to_string();
    self.metadata.next_epoch_p2p_address = option::some(p2p_address);
    validate_metadata(&self.metadata);
}

/// Update p2p address of this candidate validator
public(package) fun update_candidate_p2p_address(self: &mut ValidatorInnerV1, p2p_address: vector<u8>) {
    assert!(is_candidate(self), ENotValidatorCandidate);
    let p2p_address = p2p_address.to_ascii_string().to_string();
    self.metadata.p2p_address = p2p_address;
    validate_metadata(&self.metadata);
}

/// Update primary address of this validator, taking effects from next epoch
public(package) fun update_next_epoch_consensus_address(
    self: &mut ValidatorInnerV1,
    consensus_address: vector<u8>,
) {
    assert!(!is_inactive(self), EInactiveValidator);
    let consensus_address = consensus_address.to_ascii_string().to_string();
    self.metadata.next_epoch_consensus_address = option::some(consensus_address);
    validate_metadata(&self.metadata);
}

/// Update primary address of this candidate validator
public(package) fun update_candidate_consensus_address(
    self: &mut ValidatorInnerV1,
    consensus_address: vector<u8>,
) {
    assert!(is_candidate(self), ENotValidatorCandidate);
    let consensus_address = consensus_address.to_ascii_string().to_string();
    self.metadata.consensus_address = consensus_address;
    validate_metadata(&self.metadata);
}

/// Update protocol public key of this validator, taking effects from next epoch
public(package) fun update_next_epoch_protocol_pubkey_bytes(
    self: &mut ValidatorInnerV1,
    protocol_pubkey_bytes: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    ctx: &TxContext,
) {
    assert!(!is_inactive(self), EInactiveValidator);
    self.metadata.proof_of_possession_sender = ctx.sender();
    self.metadata.next_epoch_protocol_pubkey_bytes = option::some(protocol_pubkey_bytes);
    self.metadata.next_epoch_proof_of_possession_bytes = option::some(proof_of_possession_bytes);
    validate_metadata(&self.metadata);
}

/// Update protocol public key of this candidate validator
public(package) fun update_candidate_protocol_pubkey_bytes(
    self: &mut ValidatorInnerV1,
    protocol_pubkey_bytes: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    ctx: &TxContext,
) {
    assert!(is_candidate(self), ENotValidatorCandidate);
    self.metadata.proof_of_possession_sender = ctx.sender();
    self.metadata.protocol_pubkey_bytes = protocol_pubkey_bytes;
    self.metadata.proof_of_possession_bytes = proof_of_possession_bytes;
    validate_metadata(&self.metadata);
}

/// Update network public key of this validator, taking effects from next epoch
public(package) fun update_next_epoch_network_pubkey_bytes(
    self: &mut ValidatorInnerV1,
    network_pubkey_bytes: vector<u8>,
) {
    assert!(!is_inactive(self), EInactiveValidator);
    self.metadata.next_epoch_network_pubkey_bytes = option::some(network_pubkey_bytes);
    validate_metadata(&self.metadata);
}

/// Update network public key of this candidate validator
public(package) fun update_candidate_network_pubkey_bytes(
    self: &mut ValidatorInnerV1,
    network_pubkey_bytes: vector<u8>,
) {
    assert!(is_candidate(self), ENotValidatorCandidate);
    self.metadata.network_pubkey_bytes = network_pubkey_bytes;
    validate_metadata(&self.metadata);
}

/// Update consensus public key of this validator, taking effects from next epoch
public(package) fun update_next_epoch_consensus_pubkey_bytes(
    self: &mut ValidatorInnerV1,
    consensus_pubkey_bytes: vector<u8>,
) {
    assert!(!is_inactive(self), EInactiveValidator);
    self.metadata.next_epoch_consensus_pubkey_bytes = option::some(consensus_pubkey_bytes);
    validate_metadata(&self.metadata);
}

/// Update class groups public key and its associated proof of this validator, taking effects from next epoch
public(package) fun update_next_epoch_class_groups_pubkey_and_proof_bytes(
    self: &mut ValidatorInnerV1,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof
) {
    assert!(!is_inactive(self), EInactiveValidator);

    let old_value = self.metadata.next_epoch_class_groups_pubkey_and_proof_bytes.swap_or_fill(class_groups_pubkey_and_proof);
    old_value.destroy!(|v| {
        v.drop();
    });
    validate_metadata(&self.metadata);
}

/// Update consensus public key of this candidate validator
public(package) fun update_candidate_consensus_pubkey_bytes(
    self: &mut ValidatorInnerV1,
    consensus_pubkey_bytes: vector<u8>,
) {
    assert!(is_candidate(self), ENotValidatorCandidate);
    self.metadata.consensus_pubkey_bytes = consensus_pubkey_bytes;
    validate_metadata(&self.metadata);
}

/// Update class groups public key and its associated proof of this candidate validator
public(package) fun update_candidate_class_groups_pubkey_and_proof_bytes(
    self: &mut ValidatorInnerV1,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof,
) {
    assert!(is_candidate(self), ENotValidatorCandidate);
    update_class_groups_key_and_proof(&mut self.metadata.class_groups_pubkey_and_proof_bytes, class_groups_pubkey_and_proof);
    validate_metadata(&self.metadata);
}

/// Effectutate all staged next epoch metadata for this validator.
/// NOTE: this function SHOULD ONLY be called by validator_set when
/// advancing an epoch.
public(package) fun effectuate_staged_metadata(self: &mut ValidatorInnerV1) {
    if (next_epoch_network_address(self).is_some()) {
        self.metadata.network_address = self.metadata.next_epoch_network_address.extract();
        self.metadata.next_epoch_network_address = option::none();
    };

    if (next_epoch_p2p_address(self).is_some()) {
        self.metadata.p2p_address = self.metadata.next_epoch_p2p_address.extract();
        self.metadata.next_epoch_p2p_address = option::none();
    };

    if (next_epoch_consensus_address(self).is_some()) {
        self.metadata.consensus_address = self.metadata.next_epoch_consensus_address.extract();
        self.metadata.next_epoch_consensus_address = option::none();
    };

    if (next_epoch_protocol_pubkey_bytes(self).is_some()) {
        self.metadata.protocol_pubkey_bytes =
        self.metadata.next_epoch_protocol_pubkey_bytes.extract();
        self.metadata.next_epoch_protocol_pubkey_bytes = option::none();
        self.metadata.protocol_pubkey = g1_to_uncompressed_g1(&g1_from_bytes(&self.metadata.protocol_pubkey_bytes));
        self.metadata.proof_of_possession_bytes = self.metadata.next_epoch_proof_of_possession_bytes.extract();
        self.metadata.next_epoch_proof_of_possession_bytes = option::none();
    };

    if (next_epoch_network_pubkey_bytes(self).is_some()) {
        self.metadata.network_pubkey_bytes =
            self.metadata.next_epoch_network_pubkey_bytes.extract();
        self.metadata.next_epoch_network_pubkey_bytes = option::none();
    };

    if (next_epoch_consensus_pubkey_bytes(self).is_some()) {
        self.metadata.consensus_pubkey_bytes = self.metadata.next_epoch_consensus_pubkey_bytes.extract();
        self.metadata.next_epoch_consensus_pubkey_bytes = option::none();
    };

    if (next_epoch_class_groups_pubkey_and_proof_bytes(self).is_some()) {
        let next_epoch_class_groups_pubkey_and_proof_bytes = self.metadata.next_epoch_class_groups_pubkey_and_proof_bytes.extract();
        update_class_groups_key_and_proof(&mut self.metadata.class_groups_pubkey_and_proof_bytes, next_epoch_class_groups_pubkey_and_proof_bytes);
    };
}

fun update_class_groups_key_and_proof (
    class_groups_pubkey_and_proof: &mut TableVec<vector<u8>>,
    new_class_groups_key_and_proof: ClassGroupsPublicKeyAndProof,
) {
    let mut new_class_groups_key_and_proof = new_class_groups_key_and_proof.destroy();
    let mut i = class_groups_pubkey_and_proof.length() - 1; 
    while (!new_class_groups_key_and_proof.is_empty()) {
        *class_groups_pubkey_and_proof.borrow_mut(i) = new_class_groups_key_and_proof.pop_back();
        i = i  - 1;
    };
    new_class_groups_key_and_proof.destroy_empty();
}

/// Verify the provided proof of possession using the contained public key and the provided
/// signature.
public(package) fun verify_proof_of_possession(
    epoch: u64,
    sender_address: address,
    bls_key: vector<u8>,
    pop_signature: vector<u8>,
): bool {
    let mut intent_bytes = PROOF_OF_POSSESSION_INTENT;
    let mut message = vector<u8>[];
    message.append(bls_key);
    message.append(sui::address::to_bytes(sender_address));
    intent_bytes.append(bcs::to_bytes(&message));
    intent_bytes.append(bcs::to_bytes(&epoch));
    bls12381_min_pk_verify(
        &pop_signature,
        &bls_key,
        &intent_bytes,
    )
}

/// Aborts if validator metadata is invalid
public fun validate_metadata(metadata: &ValidatorMetadata) {
    assert!(
        metadata.network_address.length() <= MAX_VALIDATOR_METADATA_LENGTH
                && metadata.p2p_address.length() <= MAX_VALIDATOR_METADATA_LENGTH
                && metadata.consensus_address.length() <= MAX_VALIDATOR_METADATA_LENGTH
                && metadata.name.length() <= MAX_VALIDATOR_METADATA_LENGTH
                && metadata.description.length() <= MAX_VALIDATOR_METADATA_LENGTH
                && metadata.image_url.inner_url().length() <= MAX_VALIDATOR_METADATA_LENGTH
                && metadata.project_url.inner_url().length() <= MAX_VALIDATOR_METADATA_LENGTH,
        EValidatorMetadataExceedingLengthLimit,
    );
    if (metadata.next_epoch_network_address.is_some()) {
        assert!(metadata.next_epoch_network_address.borrow().length() <= MAX_VALIDATOR_METADATA_LENGTH, EValidatorMetadataExceedingLengthLimit);
    };
    if (metadata.next_epoch_p2p_address.is_some()) {
        assert!(metadata.next_epoch_p2p_address.borrow().length() <= MAX_VALIDATOR_METADATA_LENGTH, EValidatorMetadataExceedingLengthLimit);
    };
    if (metadata.next_epoch_consensus_address.is_some()) {
        assert!(metadata.next_epoch_consensus_address.borrow().length() <= MAX_VALIDATOR_METADATA_LENGTH, EValidatorMetadataExceedingLengthLimit);
    };

    assert!(metadata.network_pubkey_bytes.length() == ED25519_KEY_LEN, EMetadataInvalidNetworkPubkey);
    if (metadata.next_epoch_network_pubkey_bytes.is_some()) {
        assert!(metadata.next_epoch_network_pubkey_bytes.borrow().length() == ED25519_KEY_LEN, EMetadataInvalidNetworkPubkey);
    };
    assert!(metadata.consensus_pubkey_bytes.length() == ED25519_KEY_LEN, EMetadataInvalidConsensusPubkey);
    if (metadata.next_epoch_consensus_pubkey_bytes.is_some()) {
        assert!(metadata.next_epoch_consensus_pubkey_bytes.borrow().length() == ED25519_KEY_LEN, EMetadataInvalidConsensusPubkey);
    };

    assert!(metadata.protocol_pubkey_bytes.length() == BLS_KEY_LEN, EMetadataInvalidProtocolPubkey);
    assert!(
        verify_proof_of_possession(
            DEFAULT_EPOCH_ID, 
            metadata.proof_of_possession_sender, 
            metadata.protocol_pubkey_bytes, 
            metadata.proof_of_possession_bytes
        ), 
        EInvalidProofOfPossession
    );
    if (metadata.next_epoch_protocol_pubkey_bytes.is_some()) {
        assert!(metadata.next_epoch_protocol_pubkey_bytes.borrow().length() == BLS_KEY_LEN, EMetadataInvalidProtocolPubkey);
        assert!(
            verify_proof_of_possession(
                DEFAULT_EPOCH_ID, 
                metadata.proof_of_possession_sender, 
                *metadata.next_epoch_protocol_pubkey_bytes.borrow(), 
                *metadata.next_epoch_proof_of_possession_bytes.borrow()
            ),
            EInvalidProofOfPossession
        );
    };

    // TODO(omersadika): add test for next epoch
}

public(package) fun get_staking_pool_ref(self: &ValidatorInnerV1): &StakingPool {
    &self.staking_pool
}

/// Create a new validator from the given `ValidatorMetadata`, called by both `new` and `new_for_testing`.
fun create_from_metadata(
    validator_id: ID,
    cap_id: ID,
    operation_cap_id: ID,
    metadata: ValidatorMetadata,
    computation_price: u64,
    commission_rate: u16,
    ctx: &mut TxContext,
): ValidatorInnerV1 {
    let staking_pool = staking_pool::new(validator_id, ctx);

    let validator_inner_v1 = ValidatorInnerV1 {
        validator_id,
        metadata,
        // Initialize the voting power to be 0.
        // At the epoch change where this validator is actually added to the
        // active validator set, the voting power will be updated accordingly.
        cap_id,
        operation_cap_id,
        computation_price,
        staking_pool,
        commission_rate,
        next_epoch_stake: 0,
        next_epoch_computation_price: computation_price,
        next_epoch_commission_rate: commission_rate,
        extra_fields: bag::new(ctx),
    };
    validator_inner_v1
}

// CAUTION: THIS CODE IS ONLY FOR TESTING AND THIS MACRO MUST NEVER EVER BE REMOVED.
// Creates a validator - bypassing the proof of possession check and other metadata
// validation in the process.
// Note: `proof_of_possession_bytes` MUST be a valid signature using sui_address and
// protocol_pubkey_bytes. To produce a valid PoP, run [fn test_proof_of_possession].
#[test_only]
public(package) fun create_for_testing(
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
    mut initial_stake_option: Option<Balance<IKA>>,
    computation_price: u64,
    commission_rate: u16,
    is_active_at_genesis: bool,
    ctx: &mut TxContext,
): (ValidatorInnerV1, ValidatorCap, UnverifiedValidatorOperationCap) {
    let (mut validator, cap, operation_cap) = create_from_metadata(
        create_metadata(
            payment_address,
            protocol_pubkey_bytes,
            network_pubkey_bytes,
            consensus_pubkey_bytes,
            class_groups_pubkey_and_proof_bytes,
            proof_of_possession_bytes,
            name.to_ascii_string().to_string(),
            description.to_ascii_string().to_string(),
            url::new_unsafe_from_bytes(image_url),
            url::new_unsafe_from_bytes(project_url),
            network_address.to_ascii_string().to_string(),
            p2p_address.to_ascii_string().to_string(),
            consensus_address.to_ascii_string().to_string(),
            bag::new(ctx),
            ctx
        ),
        computation_price,
        commission_rate,
        ctx,
    );

    // // Add the validator's starting stake to the staking pool if there exists one.
    // if (initial_stake_option.is_some()) {
    //     let staked_ika = request_add_stake_at_genesis(
    //         &mut validator,
    //         0,
    //         initial_stake_option.extract(),
    //         sui_address, // give the stake to the validator
    //         ctx
    //     );
    //     transfer::public_transfer(staked_ika, ctx.sender());
    // };
    initial_stake_option.destroy_none();

    if (is_active_at_genesis) {
        activate(&mut validator, 0);
    };

    (validator, cap, operation_cap)
}
