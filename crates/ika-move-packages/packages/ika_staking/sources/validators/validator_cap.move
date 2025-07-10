// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_staking::validator_cap;

// === Structs ===

/// A capability for controlling the validator, cannot be revoked.
public struct ValidatorCap<phantom Witness: drop> has key, store {
  id: UID,
  validator_id: ID,
}

/// A capability for validator operations, can be revoked using `ValidatorCap`.
public struct ValidatorOperationCap<phantom Witness: drop> has key, store {
  id: UID,
  validator_id: ID,
}

/// A capability for validator commission, can be revoked using `ValidatorCap`.
public struct ValidatorCommissionCap<phantom Witness: drop> has key, store {
  id: UID,
  validator_id: ID,
}

/// A one time witness for the validator capability.
public struct VerifiedValidatorCap<phantom Witness: drop> has drop {
  validator_id: ID,
}

/// A one time witness for the validator operation capability.
public struct VerifiedValidatorOperationCap<phantom Witness: drop> has drop {
  validator_id: ID,
}

/// A one time witness for the validator commission capability.
public struct VerifiedValidatorCommissionCap<phantom Witness: drop> has drop {
  validator_id: ID,
}

// === Package Functions ===

public(package) fun new_validator_cap_internal<Witness: drop>(
  validator_id: ID,
  ctx: &mut TxContext,
): ValidatorCap<Witness> {
  ValidatorCap<Witness> {
    id: object::new(ctx),
    validator_id,
  }
}

public(package) fun new_validator_operation_cap_internal<Witness: drop>(
  validator_id: ID,
  ctx: &mut TxContext,
): ValidatorOperationCap<Witness> {
  ValidatorOperationCap<Witness> {
    id: object::new(ctx),
    validator_id,
  }
}

public(package) fun new_validator_commission_cap_internal<Witness: drop>(
  validator_id: ID,
  ctx: &mut TxContext,
): ValidatorCommissionCap<Witness> {
  ValidatorCommissionCap<Witness> {
    id: object::new(ctx),
    validator_id,
  }
}

public(package) fun create_verified_validator_cap_internal<Witness: drop>(
  cap: &ValidatorCap<Witness>,
): VerifiedValidatorCap<Witness> {
  VerifiedValidatorCap<Witness> {
    validator_id: cap.validator_id,
  }
}

public(package) fun create_verified_validator_operation_cap_internal<Witness: drop>(
  cap: &ValidatorOperationCap<Witness>,
): VerifiedValidatorOperationCap<Witness> {
  VerifiedValidatorOperationCap<Witness> {
    validator_id: cap.validator_id,
  }
}

public(package) fun create_verified_validator_commission_cap_internal<Witness: drop>(
  cap: &ValidatorCommissionCap<Witness>,
): VerifiedValidatorCommissionCap<Witness> {
  VerifiedValidatorCommissionCap<Witness> {
    validator_id: cap.validator_id,
  }
}

public fun validator_id<Witness: drop>(cap: &ValidatorCap<Witness>): ID {
  cap.validator_id
}

public(package) fun validator_operation_cap_validator_id<Witness: drop>(
  cap: &ValidatorOperationCap<Witness>,
): ID {
  cap.validator_id
}

public use fun validator_operation_cap_validator_id as ValidatorOperationCap.validator_id;

public(package) fun validator_commission_cap_validator_id<Witness: drop>(
  cap: &ValidatorCommissionCap<Witness>,
): ID {
  cap.validator_id
}

public use fun validator_commission_cap_validator_id as ValidatorCommissionCap.validator_id;

// === Public Functions ===

public fun new_validator_cap<Witness: drop>(
  validator_id: ID,
  _: Witness,
  ctx: &mut TxContext,
): ValidatorCap<Witness> {
  new_validator_cap_internal(validator_id, ctx)
}

/// Should be only called by the friend modules when adding a `Validator`
/// or rotating an existing validator's `operation_cap_id`.
public fun new_validator_operation_cap<Witness: drop>(
  validator_id: ID,
  _: Witness,
  ctx: &mut TxContext,
): ValidatorOperationCap<Witness> {
  new_validator_operation_cap_internal(validator_id, ctx)
}

/// Should be only called by the friend modules when adding a `Validator`
/// or rotating an existing validator's `commission_cap_id`.
public fun new_validator_commission_cap<Witness: drop>(
  validator_id: ID,
  _: Witness,
  ctx: &mut TxContext,
): ValidatorCommissionCap<Witness> {
  new_validator_commission_cap_internal(validator_id, ctx)
}

public fun create_verified_validator_cap<Witness: drop>(
  cap: &ValidatorCap<Witness>,
  _: Witness,
): VerifiedValidatorCap<Witness> {
  create_verified_validator_cap_internal(cap)
}

public fun create_verified_validator_operation_cap<Witness: drop>(
  cap: &ValidatorOperationCap<Witness>,
  _: Witness,
): VerifiedValidatorOperationCap<Witness> {
  create_verified_validator_operation_cap_internal(cap)
}

public fun create_verified_validator_commission_cap<Witness: drop>(
  cap: &ValidatorCommissionCap<Witness>,
  _: Witness,
): VerifiedValidatorCommissionCap<Witness> {
  create_verified_validator_commission_cap_internal(cap)
}

public fun verified_validator_cap_validator_id<Witness: drop>(
  cap: &VerifiedValidatorCap<Witness>,
): ID {
  cap.validator_id
}

public use fun verified_validator_cap_validator_id as VerifiedValidatorCap.validator_id;

public fun verified_validator_operation_cap_validator_id<Witness: drop>(
  cap: &VerifiedValidatorOperationCap<Witness>,
): ID {
  cap.validator_id
}

public use fun verified_validator_operation_cap_validator_id as
  VerifiedValidatorOperationCap.validator_id;

public fun verified_validator_commission_cap_validator_id<Witness: drop>(
  cap: &VerifiedValidatorCommissionCap<Witness>,
): ID {
  cap.validator_id
}

public use fun verified_validator_commission_cap_validator_id as
  VerifiedValidatorCommissionCap.validator_id;

// === Test Functions ===

#[test_only]
public fun destroy_validator_cap_for_testing<Witness: drop>(cap: ValidatorCap<Witness>) {
  let ValidatorCap { id, .. } = cap;
  id.delete();
}

#[test_only]
public fun destroy_validator_operation_cap_for_testing<Witness: drop>(
  cap: ValidatorOperationCap<Witness>,
) {
  let ValidatorOperationCap { id, .. } = cap;
  id.delete();
}

#[test_only]
public fun destroy_validator_commission_cap_for_testing<Witness: drop>(
  cap: ValidatorCommissionCap<Witness>,
) {
  let ValidatorCommissionCap { id, .. } = cap;
  id.delete();
}
