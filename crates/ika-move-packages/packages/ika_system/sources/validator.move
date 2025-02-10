// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::validator;

use ika_system::validator_inner_v1::{Self, ValidatorInnerV1};
use ika_system::validator_cap::{Self, ValidatorCap, ValidatorOperationCap};
use sui::versioned::{Self, Versioned};

const EInvalidVersion: u64 = 0;

/// Flag to indicate the version of the ika validator.
const VERSION: u64 = 1;

public struct Validator has key, store {
    id: UID,
    inner: Versioned,
}

// Validator corresponds to version 1.
public(package) fun create(
    payment_address: address,
    protocol_pubkey_bytes: vector<u8>,
    network_pubkey_bytes: vector<u8>,
    consensus_pubkey_bytes: vector<u8>,
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
): (Validator, ValidatorCap, ValidatorOperationCap) {
    let validator_uid = object::new(ctx);

    let validator_id = validator_uid.to_inner();

    let cap = validator_cap::new_validator_cap(validator_id, ctx);

    let operation_cap = validator_cap::new_validator_operation_cap(
        validator_id,
        ctx,
    );

    let cap_id = object::id(&cap);
    let operation_cap_id = object::id(&operation_cap);

    let validator_inner_v1 = validator_inner_v1::create(
        validator_id,
        cap_id,
        operation_cap_id,
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
    );

    let validator = Validator {
        id: validator_uid,
        inner: versioned::create(VERSION, validator_inner_v1, ctx),
    };
    (validator, cap, operation_cap)
}

/// This function should always return the latest supported version.
/// If the inner version is old, we upgrade it lazily in-place.
public(package) fun load_validator_maybe_upgrade(self: &mut Validator): &mut ValidatorInnerV1 {
    upgrade_to_latest(self);
    versioned::load_value_mut(&mut self.inner)
}

/// Destroy the wrapper and retrieve the inner validator object.
public(package) fun destroy(self: Validator): ValidatorInnerV1 {
    upgrade_to_latest(&self);
    let Validator { id, inner } = self;
    id.delete();
    versioned::destroy(inner)
}

#[test_only]
/// Load the inner validator with assumed type. This should be used for testing only.
public(package) fun get_inner_validator_ref(self: &Validator): &ValidatorInnerV1 {
    versioned::load_value(&self.inner)
}

fun upgrade_to_latest(self: &Validator) {
    let version = version(self);
    // TODO: When new versions are added, we need to explicitly upgrade here.
    assert!(version == VERSION, EInvalidVersion);
}

fun version(self: &Validator): u64 {
    versioned::version(&self.inner)
}
