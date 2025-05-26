// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_field, unused_function, unused_variable, unused_use)]
module ika_system::validator_info;

use std::string::String;
use sui::{bls12381::{UncompressedG1, g1_from_bytes, g1_to_uncompressed_g1, bls12381_min_pk_verify}, group_ops::Element};
use sui::table_vec::{Self, TableVec};
use sui::bcs;
use ika_system::{
    extended_field::{Self, ExtendedField},
    validator_metadata::{Self, ValidatorMetadata}
};
use ika_system::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof;
use ika_system::multiaddr;

const MAX_NODE_NAME_LENGTH: u64 = 100;

const MAX_VALIDATOR_TEXT_FIELD_LENGTH: u64 = 259;

const PROOF_OF_POSSESSION_INTENT: vector<u8> = vector[0, 0, 0];
const DEFAULT_EPOCH_ID: u64 = 0;

const BLS_KEY_LEN: u64 = 48;
const ED25519_KEY_LEN: u64 = 32;
//const CLASS_GROUPS_BYTES_LEN: u64 = 241722; // Todo (#): change the way we implement this.

// Error codes
/// The network public key length is invalid.
#[error]
const EInvalidProofOfPossession: vector<u8> = b"Invalid proof_of_possession_bytes field in ValidatorMetadata.";

#[error]
const EInvalidNameLength: vector<u8> = b"Validator name length exceeds maximum allowed length.";

#[error]
const EMetadataInvalidProtocolPubkey: vector<u8> = b"Invalid protocol_pubkey_bytes field in ValidatorMetadata.";

#[error]
const EMetadataInvalidNetworkPubkey: vector<u8> = b"Invalid network_pubkey_bytes field in ValidatorMetadata.";

#[error]
const EMetadataInvalidConsensusPubkey: vector<u8> = b"Invalid consensus_pubkey_bytes field in ValidatorMetadata.";

// #[error]
// const EMetadataInvalidClassGroupsPubkey: vector<u8> = b"Invalid class_groups_pubkey_and_proof_bytes field in ValidatorMetadata.";

#[error]
const EMetadataInvalidNetworkAddress: vector<u8> = b"Invalid network_address field in ValidatorMetadata.";

#[error]
const EMetadataInvalidP2pAddress: vector<u8> = b"Invalid p2p_address field in ValidatorMetadata.";

#[error]
const EMetadataInvalidConsensusAddress: vector<u8> = b"Invalid consensus_address field in ValidatorMetadata.";

// #[error]
// const ECommissionRateTooHigh: vector<u8> = b"Commission rate set by the validator is higher than the threshold.";

#[error]
const EValidatorMetadataExceedingLengthLimit: vector<u8> = b"Validator Metadata is too long.";

// #[error]
// const ENotValidatorCandidate: vector<u8> = b"Intended validator is not a candidate one.";

// #[error]
// const EInvalidStakeAmount: vector<u8> = b"Stake amount is invalid or wrong.";

// #[error]
// const EInactiveValidator: vector<u8> = b"The validator is inactive.";

// #[error]
// const ENewCapNotCreatedByValidatorItself: vector<u8> = b"New Capability is not created by the validator itself.";

// #[error]
// const EInvalidCap: vector<u8> = b"Cap is not valid.";

// #[error]
// const EGasPriceHigherThanThreshold: vector<u8> = b"Validator trying to set computation price higher than threshold.";


/// Represents a validator info in the system.
public struct ValidatorInfo has store {
    name: String,
    validator_id: ID,

    /// The network address of the validator (could also contain extra info such as port, DNS and etc.).
    network_address: String,
    /// The address of the validator used for p2p activities such as state sync (could also contain extra info such as port, DNS and etc.).
    p2p_address: String,
    /// The address of the consensus
    consensus_address: String,

    /// The public key bytes corresponding to the private key that the validator
    /// holds to sign checkpoint messages.
    protocol_pubkey_bytes: vector<u8>,
    protocol_pubkey: Element<UncompressedG1>,
    /// The public key bytes corresponding to the private key that the validator
    /// uses to establish TLS connections
    network_pubkey_bytes: vector<u8>,
    /// The public key bytes correstponding to the consensus
    consensus_pubkey_bytes: vector<u8>,
    /// The validator's Class Groups public key and its associated proof.
    /// This key is used for the network DKG process and for resharing the network MPC key.
    class_groups_pubkey_and_proof_bytes: TableVec<vector<u8>>,

    /// "next_epoch" metadata only takes effects in the next epoch.
    /// If none, current value will stay unchanged.
    next_epoch_protocol_pubkey_bytes: Option<vector<u8>>,
    next_epoch_network_pubkey_bytes: Option<vector<u8>>,
    next_epoch_consensus_pubkey_bytes: Option<vector<u8>>,
    next_epoch_class_groups_pubkey_and_proof_bytes: Option<ClassGroupsPublicKeyAndProof>,
    next_epoch_network_address: Option<String>,
    next_epoch_p2p_address: Option<String>,
    next_epoch_consensus_address: Option<String>,
    /// Any extra fields that's not defined statically.
    ///
    metadata: ExtendedField<ValidatorMetadata>,
}

/// A public constructor for the ValidatorInfo.
public(package) fun new(
    name: String,
    validator_id: ID,
    protocol_pubkey_bytes: vector<u8>,
    network_pubkey_bytes: vector<u8>,
    consensus_pubkey_bytes: vector<u8>,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    proof_of_possession_bytes: vector<u8>,
    network_address: String,
    p2p_address: String,
    consensus_address: String,
    metadata: ValidatorMetadata,
    ctx: &mut TxContext,
): ValidatorInfo {
    let protocol_pubkey = g1_to_uncompressed_g1(&g1_from_bytes(&protocol_pubkey_bytes));
    let class_groups_pubkey_and_proof_bytes = class_groups_pubkey_and_proof_bytes.destroy();
    assert!(
        verify_proof_of_possession(
            DEFAULT_EPOCH_ID,
            ctx.sender(),
            protocol_pubkey_bytes,
            proof_of_possession_bytes
        ),
        EInvalidProofOfPossession
    );
    let validator_info = ValidatorInfo {
        validator_id,
        name,
        protocol_pubkey_bytes,
        protocol_pubkey,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        class_groups_pubkey_and_proof_bytes,
        network_address,
        p2p_address,
        consensus_address,
        next_epoch_protocol_pubkey_bytes: option::none(),
        next_epoch_network_pubkey_bytes: option::none(),
        next_epoch_consensus_pubkey_bytes: option::none(),
        next_epoch_class_groups_pubkey_and_proof_bytes: option::none(),
        next_epoch_network_address: option::none(),
        next_epoch_p2p_address: option::none(),
        next_epoch_consensus_address: option::none(),
        metadata: extended_field::new(metadata, ctx),
    };
    validator_info.validate();
    validator_info
}

// === Accessors ===

/// Return the name of the validator info.
public(package) fun metadata(self: &ValidatorInfo): ValidatorMetadata {
    *self.metadata.borrow()
}

public fun validator_id(self: &ValidatorInfo): ID {
    self.validator_id
}

public fun network_address(self: &ValidatorInfo): &String {
    &self.network_address
}

public fun p2p_address(self: &ValidatorInfo): &String {
    &self.p2p_address
}

public fun consensus_address(self: &ValidatorInfo): &String {
    &self.consensus_address
}

public fun protocol_pubkey_bytes(self: &ValidatorInfo): &vector<u8> {
    &self.protocol_pubkey_bytes
}

public fun protocol_pubkey(self: &ValidatorInfo): &Element<UncompressedG1> {
    &self.protocol_pubkey
}

public fun network_pubkey_bytes(self: &ValidatorInfo): &vector<u8> {
    &self.network_pubkey_bytes
}

public fun consensus_pubkey_bytes(self: &ValidatorInfo): &vector<u8> {
    &self.consensus_pubkey_bytes
}

public fun class_groups_pubkey_and_proof_bytes(self: &ValidatorInfo): &TableVec<vector<u8>> {
    &self.class_groups_pubkey_and_proof_bytes
}

public fun next_epoch_network_address(self: &ValidatorInfo): &Option<String> {
    &self.next_epoch_network_address
}

public fun next_epoch_p2p_address(self: &ValidatorInfo): &Option<String> {
    &self.next_epoch_p2p_address
}

public fun next_epoch_consensus_address(self: &ValidatorInfo): &Option<String> {
    &self.next_epoch_consensus_address
}

public fun next_epoch_protocol_pubkey_bytes(self: &ValidatorInfo): &Option<vector<u8>> {
    &self.next_epoch_protocol_pubkey_bytes
}

public fun next_epoch_network_pubkey_bytes(self: &ValidatorInfo): &Option<vector<u8>> {
    &self.next_epoch_network_pubkey_bytes
}

public fun next_epoch_consensus_pubkey_bytes(self: &ValidatorInfo): &Option<vector<u8>> {
    &self.next_epoch_consensus_pubkey_bytes
}

public fun next_epoch_class_groups_pubkey_and_proof_bytes(self: &ValidatorInfo): &Option<ClassGroupsPublicKeyAndProof> {
    &self.next_epoch_class_groups_pubkey_and_proof_bytes
}


// === Modifiers ===

/// Sets the name of the validator info.
public(package) fun set_name(self: &mut ValidatorInfo, name: String) {
    self.name = name;
    self.validate();
}

/// Sets the network address or host of the validator info.
public(package) fun set_network_address(self: &mut ValidatorInfo, network_address: String) {
    self.network_address = network_address;
    self.validate();
}

/// Sets the metadata of the validator info.
public(package) fun set_validator_metadata(self: &mut ValidatorInfo, metadata: ValidatorMetadata) {
    self.metadata.swap(metadata);
}

/// Sets network address of this validator, taking effects from next epoch
public(package) fun set_next_epoch_network_address(
    self: &mut ValidatorInfo,
    network_address: String,
) {
    self.next_epoch_network_address = option::some(network_address);
    self.validate();
}

/// Sets p2p address of this validator, taking effects from next epoch
public(package) fun set_next_epoch_p2p_address(self: &mut ValidatorInfo, p2p_address: String) {
    self.next_epoch_p2p_address = option::some(p2p_address);
    self.validate();
}

/// Sets primary address of this validator, taking effects from next epoch
public(package) fun set_next_epoch_consensus_address(
    self: &mut ValidatorInfo,
    consensus_address: String,
) {
    self.next_epoch_consensus_address = option::some(consensus_address);
    self.validate();
}

/// Sets protocol public key of this validator, taking effects from next epoch
public(package) fun set_next_epoch_protocol_pubkey_bytes(
    self: &mut ValidatorInfo,
    protocol_pubkey_bytes: vector<u8>,
    proof_of_possession_bytes: vector<u8>,
    ctx: &TxContext,
) {
    assert!(
        verify_proof_of_possession(
            DEFAULT_EPOCH_ID,
            ctx.sender(),
            protocol_pubkey_bytes,
            proof_of_possession_bytes
        ),
        EInvalidProofOfPossession
    );
    self.next_epoch_protocol_pubkey_bytes = option::some(protocol_pubkey_bytes);
    self.validate();
}

/// Sets network public key of this validator, taking effects from next epoch
public(package) fun set_next_epoch_network_pubkey_bytes(
    self: &mut ValidatorInfo,
    network_pubkey_bytes: vector<u8>,
) {
    self.next_epoch_network_pubkey_bytes = option::some(network_pubkey_bytes);
    self.validate();
}

/// Sets consensus public key of this validator, taking effects from next epoch
public(package) fun set_next_epoch_consensus_pubkey_bytes(
    self: &mut ValidatorInfo,
    consensus_pubkey_bytes: vector<u8>,
) {
    self.next_epoch_consensus_pubkey_bytes = option::some(consensus_pubkey_bytes);
    self.validate();
}

/// Sets class groups public key and its associated proof of this validator, taking effects from next epoch
public(package) fun set_next_epoch_class_groups_pubkey_and_proof_bytes(
    self: &mut ValidatorInfo,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof
) {
    let old_value = self.next_epoch_class_groups_pubkey_and_proof_bytes.swap_or_fill(class_groups_pubkey_and_proof);
    old_value.destroy!(|v| {
        v.drop();
    });
    self.validate();
}



/// Effectutate all staged next epoch metadata for this validator.
/// NOTE: this function SHOULD ONLY be called by validator_set when
/// advancing an epoch.
public(package) fun roatate_next_epoch_info(self: &mut ValidatorInfo) {
    if (self.next_epoch_network_address.is_some()) {
        self.network_address = self.next_epoch_network_address.extract();
        self.next_epoch_network_address = option::none();
    };

    if (self.next_epoch_p2p_address.is_some()) {
        self.p2p_address = self.next_epoch_p2p_address.extract();
        self.next_epoch_p2p_address = option::none();
    };

    if (self.next_epoch_consensus_address.is_some()) {
        self.consensus_address = self.next_epoch_consensus_address.extract();
        self.next_epoch_consensus_address = option::none();
    };

    if (self.next_epoch_protocol_pubkey_bytes.is_some()) {
        self.protocol_pubkey_bytes =
        self.next_epoch_protocol_pubkey_bytes.extract();
        self.next_epoch_protocol_pubkey_bytes = option::none();
        self.protocol_pubkey = g1_to_uncompressed_g1(&g1_from_bytes(&self.protocol_pubkey_bytes));
    };

    if (self.next_epoch_network_pubkey_bytes.is_some()) {
        self.network_pubkey_bytes =
            self.next_epoch_network_pubkey_bytes.extract();
        self.next_epoch_network_pubkey_bytes = option::none();
    };

    if (self.next_epoch_consensus_pubkey_bytes.is_some()) {
        self.consensus_pubkey_bytes = self.next_epoch_consensus_pubkey_bytes.extract();
        self.next_epoch_consensus_pubkey_bytes = option::none();
    };

    if (self.next_epoch_class_groups_pubkey_and_proof_bytes.is_some()) {
        let next_epoch_class_groups_pubkey_and_proof_bytes = self.next_epoch_class_groups_pubkey_and_proof_bytes.extract();
        update_class_groups_key_and_proof(&mut self.class_groups_pubkey_and_proof_bytes, next_epoch_class_groups_pubkey_and_proof_bytes);
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

/// Aborts if validator info is invalid
public fun validate(self: &ValidatorInfo) {
        // Verify name length.
    assert!(self.name.length() <= MAX_NODE_NAME_LENGTH, EInvalidNameLength);

    // Verify address length.
    assert!(
        self.network_address.length() <= MAX_VALIDATOR_TEXT_FIELD_LENGTH
                && self.p2p_address.length() <= MAX_VALIDATOR_TEXT_FIELD_LENGTH
                && self.consensus_address.length() <= MAX_VALIDATOR_TEXT_FIELD_LENGTH
                && self.name.length() <= MAX_VALIDATOR_TEXT_FIELD_LENGTH,
        EValidatorMetadataExceedingLengthLimit,
    );

    assert!(multiaddr::validate_tcp(&self.network_address), EMetadataInvalidNetworkAddress);
    if (self.next_epoch_network_address.is_some()) {
        assert!(self.next_epoch_network_address.borrow().length() <= MAX_VALIDATOR_TEXT_FIELD_LENGTH, EValidatorMetadataExceedingLengthLimit);
        assert!(multiaddr::validate_tcp(self.next_epoch_network_address.borrow()), EMetadataInvalidNetworkAddress);
    };

    assert!(multiaddr::validate_udp(&self.p2p_address), EMetadataInvalidP2pAddress);
    if (self.next_epoch_p2p_address.is_some()) {
        assert!(self.next_epoch_p2p_address.borrow().length() <= MAX_VALIDATOR_TEXT_FIELD_LENGTH, EValidatorMetadataExceedingLengthLimit);
        assert!(multiaddr::validate_udp(self.next_epoch_p2p_address.borrow()), EMetadataInvalidP2pAddress);
    };

    assert!(multiaddr::validate_udp(&self.consensus_address), EMetadataInvalidConsensusAddress);
    if (self.next_epoch_consensus_address.is_some()) {
        assert!(self.next_epoch_consensus_address.borrow().length() <= MAX_VALIDATOR_TEXT_FIELD_LENGTH, EValidatorMetadataExceedingLengthLimit);
        assert!(multiaddr::validate_udp(self.next_epoch_consensus_address.borrow()), EMetadataInvalidConsensusAddress);
    };

    assert!(self.network_pubkey_bytes.length() == ED25519_KEY_LEN, EMetadataInvalidNetworkPubkey);
    if (self.next_epoch_network_pubkey_bytes.is_some()) {
        assert!(self.next_epoch_network_pubkey_bytes.borrow().length() == ED25519_KEY_LEN, EMetadataInvalidNetworkPubkey);
    };
    assert!(self.consensus_pubkey_bytes.length() == ED25519_KEY_LEN, EMetadataInvalidConsensusPubkey);
    if (self.next_epoch_consensus_pubkey_bytes.is_some()) {
        assert!(self.next_epoch_consensus_pubkey_bytes.borrow().length() == ED25519_KEY_LEN, EMetadataInvalidConsensusPubkey);
    };

    assert!(self.protocol_pubkey_bytes.length() == BLS_KEY_LEN, EMetadataInvalidProtocolPubkey);
    if (self.next_epoch_protocol_pubkey_bytes.is_some()) {
        assert!(self.next_epoch_protocol_pubkey_bytes.borrow().length() == BLS_KEY_LEN, EMetadataInvalidProtocolPubkey);
    };

    // TODO(omersadika): add test for next epoch
}

/// Destroy the validator info.
public(package) fun destroy(self: ValidatorInfo) {
    let ValidatorInfo { metadata, mut class_groups_pubkey_and_proof_bytes, next_epoch_class_groups_pubkey_and_proof_bytes, .. } = self;
    metadata.destroy();
    while(class_groups_pubkey_and_proof_bytes.length() != 0) {
        class_groups_pubkey_and_proof_bytes.pop_back();
    };
    class_groups_pubkey_and_proof_bytes.destroy_empty();
    next_epoch_class_groups_pubkey_and_proof_bytes.destroy!(|c| c.drop());
}

// === Testing ===

#[test_only]
/// Create a validator info with dummy name & address
public fun new_for_testing(public_key: vector<u8>): ValidatorInfo {
    let ctx = &mut tx_context::dummy();
    let validator_id = ctx.fresh_object_address().to_id();
    let protocol_pubkey = g1_to_uncompressed_g1(&g1_from_bytes(&public_key));
    let class_groups_bytes = table_vec::empty(ctx);

    ValidatorInfo {
        validator_id,
        name: b"node".to_string(),
        network_address: b"127.0.0.1".to_string(),
        p2p_address: b"127.0.0.1".to_string(),
        consensus_address: b"127.0.0.1".to_string(),
        protocol_pubkey_bytes: public_key,
        protocol_pubkey,
        network_pubkey_bytes: vector[],
        consensus_pubkey_bytes: vector[],
        class_groups_pubkey_and_proof_bytes: class_groups_bytes,
        next_epoch_protocol_pubkey_bytes: option::none(),
        next_epoch_network_pubkey_bytes: option::none(),
        next_epoch_consensus_pubkey_bytes: option::none(),
        next_epoch_class_groups_pubkey_and_proof_bytes: option::none(),
        next_epoch_network_address: option::none(),
        next_epoch_p2p_address: option::none(),
        next_epoch_consensus_address: option::none(),
        metadata: extended_field::new(validator_metadata::default(), ctx),
    }
}
public fun is_duplicate(self: &ValidatorInfo, other: &ValidatorInfo): bool {
            self.name == other.name
            || self.network_address == other.network_address
            || self.p2p_address == other.p2p_address
            || self.protocol_pubkey_bytes == other.protocol_pubkey_bytes
            || self.network_pubkey_bytes == other.network_pubkey_bytes
            || self.network_pubkey_bytes == other.consensus_pubkey_bytes
            || self.consensus_pubkey_bytes == other.consensus_pubkey_bytes
            || self.consensus_pubkey_bytes == other.network_pubkey_bytes
            // All next epoch parameters.
            || is_equal_some(&self.next_epoch_network_address, &other.next_epoch_network_address)
            || is_equal_some(&self.next_epoch_p2p_address, &other.next_epoch_p2p_address)
            || is_equal_some(&self.next_epoch_protocol_pubkey_bytes, &other.next_epoch_protocol_pubkey_bytes)
            || is_equal_some(&self.next_epoch_network_pubkey_bytes, &other.next_epoch_network_pubkey_bytes)
            || is_equal_some(&self.next_epoch_network_pubkey_bytes, &other.next_epoch_consensus_pubkey_bytes)
            || is_equal_some(&self.next_epoch_consensus_pubkey_bytes, &other.next_epoch_consensus_pubkey_bytes)
            || is_equal_some(&self.next_epoch_consensus_pubkey_bytes, &other.next_epoch_network_pubkey_bytes)
            // My next epoch parameters with other current epoch parameters.
            || is_equal_some_and_value(&self.next_epoch_network_address, &other.network_address)
            || is_equal_some_and_value(&self.next_epoch_p2p_address, &other.p2p_address)
            || is_equal_some_and_value(&self.next_epoch_protocol_pubkey_bytes, &other.protocol_pubkey_bytes)
            || is_equal_some_and_value(&self.next_epoch_network_pubkey_bytes, &other.network_pubkey_bytes)
            || is_equal_some_and_value(&self.next_epoch_network_pubkey_bytes, &other.consensus_pubkey_bytes)
            || is_equal_some_and_value(&self.next_epoch_consensus_pubkey_bytes, &other.consensus_pubkey_bytes)
            || is_equal_some_and_value(&self.next_epoch_consensus_pubkey_bytes, &other.network_pubkey_bytes)
            // Other next epoch parameters with my current epoch parameters.
            || is_equal_some_and_value(&other.next_epoch_network_address, &self.network_address)
            || is_equal_some_and_value(&other.next_epoch_p2p_address, &self.p2p_address)
            || is_equal_some_and_value(&other.next_epoch_protocol_pubkey_bytes, &self.protocol_pubkey_bytes)
            || is_equal_some_and_value(&other.next_epoch_network_pubkey_bytes, &self.network_pubkey_bytes)
            || is_equal_some_and_value(&other.next_epoch_network_pubkey_bytes, &self.consensus_pubkey_bytes)
            || is_equal_some_and_value(&other.next_epoch_consensus_pubkey_bytes, &self.consensus_pubkey_bytes)
            || is_equal_some_and_value(&other.next_epoch_consensus_pubkey_bytes, &self.network_pubkey_bytes)
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