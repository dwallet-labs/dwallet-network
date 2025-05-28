// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::validator_cap;

/// A capability for controlling the validator, cannot be revoked.
public struct ValidatorCap has key, store {
    id: UID,
    validator_id: ID,
}

public(package) fun new_validator_cap(
    validator_id: ID,
    ctx: &mut TxContext,
): ValidatorCap {
    let cap = ValidatorCap {
        id: object::new(ctx),
        validator_id
    };
    cap
}

public fun validator_id(
    cap: &ValidatorCap,
): ID {
    cap.validator_id
}

/// A capability for validator operations, can be revoked using `ValidatorCap`.
public struct ValidatorOperationCap has key, store {
    id: UID,
    validator_id: ID,
}

public(package) fun validator_operation_cap_validator_id(cap: &ValidatorOperationCap): ID {
    cap.validator_id
}

public use fun validator_operation_cap_validator_id as ValidatorOperationCap.validator_id;

/// Should be only called by the friend modules when adding a `Validator`
/// or rotating an existing validaotr's `operation_cap_id`.
public(package) fun new_validator_operation_cap(
    validator_id: ID,
    ctx: &mut TxContext,
): ValidatorOperationCap {
    let operation_cap = ValidatorOperationCap {
        id: object::new(ctx),
        validator_id,
    };
    operation_cap
}

/// A capability for validator commission, can be revoked using `ValidatorCap`.
public struct ValidatorCommissionCap has key, store {
    id: UID,
    validator_id: ID,
}

public(package) fun validator_commission_cap_validator_id(cap: &ValidatorCommissionCap): ID {
    cap.validator_id
}

public use fun validator_commission_cap_validator_id as ValidatorCommissionCap.validator_id;

/// Should be only called by the friend modules when adding a `Validator`
/// or rotating an existing validaotr's `commission_cap_id`.
public(package) fun new_validator_commission_cap(
    validator_id: ID,
    ctx: &mut TxContext,
): ValidatorCommissionCap {
    let commission_cap = ValidatorCommissionCap {
        id: object::new(ctx),
        validator_id,
    };
    commission_cap
}

