// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::validator_cap;

// === Structs ===

/// A capability for controlling the validator, cannot be revoked.
public struct ValidatorCap has key, store {
    id: UID,
    validator_id: ID,
}

/// A capability for validator operations, can be revoked using `ValidatorCap`.
public struct ValidatorOperationCap has key, store {
    id: UID,
    validator_id: ID,
}

/// A capability for validator commission, can be revoked using `ValidatorCap`.
public struct ValidatorCommissionCap has key, store {
    id: UID,
    validator_id: ID,
}

// === Package Functions ===

public(package) fun new_validator_cap(
    validator_id: ID,
    ctx: &mut TxContext,
): ValidatorCap {
    ValidatorCap {
        id: object::new(ctx),
        validator_id
    }
}

public fun validator_id(
    cap: &ValidatorCap,
): ID {
    cap.validator_id
}

public(package) fun validator_operation_cap_validator_id(cap: &ValidatorOperationCap): ID {
    cap.validator_id
}

public use fun validator_operation_cap_validator_id as ValidatorOperationCap.validator_id;

/// Should be only called by the friend modules when adding a `Validator`
/// or rotating an existing validator's `operation_cap_id`.
public(package) fun new_validator_operation_cap(
    validator_id: ID,
    ctx: &mut TxContext,
): ValidatorOperationCap {
    ValidatorOperationCap {
        id: object::new(ctx),
        validator_id,
    }
}

public(package) fun validator_commission_cap_validator_id(cap: &ValidatorCommissionCap): ID {
    cap.validator_id
}

public use fun validator_commission_cap_validator_id as ValidatorCommissionCap.validator_id;

/// Should be only called by the friend modules when adding a `Validator`
/// or rotating an existing validator's `commission_cap_id`.
public(package) fun new_validator_commission_cap(
    validator_id: ID,
    ctx: &mut TxContext,
): ValidatorCommissionCap {
    ValidatorCommissionCap {
        id: object::new(ctx),
        validator_id,
    }
}

#[test_only]
public fun destroy_validator_cap_for_testing(cap: ValidatorCap) {
    let ValidatorCap { id, .. } = cap;
    id.delete();
}

#[test_only]
public fun destroy_validator_operation_cap_for_testing(cap: ValidatorOperationCap) {
    let ValidatorOperationCap { id, .. } = cap;
    id.delete();
}

#[test_only]
public fun destroy_validator_commission_cap_for_testing(cap: ValidatorCommissionCap) {
    let ValidatorCommissionCap { id, .. } = cap;
    id.delete();
}
