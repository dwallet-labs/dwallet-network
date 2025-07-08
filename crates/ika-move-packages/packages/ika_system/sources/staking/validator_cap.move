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

/// A one time witness for the validator capability.
public struct VerifiedValidatorCap has drop {
    validator_id: ID,
}

/// A one time witness for the validator operation capability.
public struct VerifiedValidatorOperationCap has drop {
    validator_id: ID,
}

/// A one time witness for the validator commission capability.
public struct VerifiedValidatorCommissionCap has drop {
    validator_id: ID,
}


// === Package Functions ===

public(package) fun new_validator_cap(validator_id: ID, ctx: &mut TxContext): ValidatorCap {
    ValidatorCap {
        id: object::new(ctx),
        validator_id,
    }
}

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

public(package) fun create_verified_validator_cap(cap: &ValidatorCap): VerifiedValidatorCap {
    VerifiedValidatorCap {
        validator_id: cap.validator_id,
    }
}

public(package) fun create_verified_validator_operation_cap(cap: &ValidatorOperationCap): VerifiedValidatorOperationCap {
    VerifiedValidatorOperationCap {
        validator_id: cap.validator_id,
    }
}

public(package) fun create_verified_validator_commission_cap(cap: &ValidatorCommissionCap): VerifiedValidatorCommissionCap {
    VerifiedValidatorCommissionCap {
        validator_id: cap.validator_id,
    }
}


public(package) fun validator_id(cap: &ValidatorCap): ID {
    cap.validator_id
}

public(package) fun validator_operation_cap_validator_id(cap: &ValidatorOperationCap): ID {
    cap.validator_id
}

public use fun validator_operation_cap_validator_id as ValidatorOperationCap.validator_id;

public(package) fun validator_commission_cap_validator_id(cap: &ValidatorCommissionCap): ID {
    cap.validator_id
}

public use fun validator_commission_cap_validator_id as ValidatorCommissionCap.validator_id;

// === Public Functions ===

public fun verified_validator_cap_validator_id(cap: &VerifiedValidatorCap): ID {
    cap.validator_id
}

public use fun verified_validator_cap_validator_id as VerifiedValidatorCap.validator_id;

public fun verified_validator_operation_cap_validator_id(cap: &VerifiedValidatorOperationCap): ID {
    cap.validator_id
}

public use fun verified_validator_operation_cap_validator_id as VerifiedValidatorOperationCap.validator_id;

public fun verified_validator_commission_cap_validator_id(cap: &VerifiedValidatorCommissionCap): ID {
    cap.validator_id
}

public use fun verified_validator_commission_cap_validator_id as VerifiedValidatorCommissionCap.validator_id;


// === Test Functions ===

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
