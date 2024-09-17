// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module pera_system::validator_wrapper {
    use pera::versioned::Versioned;
    use pera::versioned;
    use pera::tx_context::TxContext;
    use pera_system::validator::Validator;

    const EInvalidVersion: u64 = 0;

    public struct ValidatorWrapper has store {
        inner: Versioned
    }

    // Validator corresponds to version 1.
    public(package) fun create_v1(validator: Validator, ctx: &mut TxContext): ValidatorWrapper {
        ValidatorWrapper {
            inner: versioned::create(1, validator, ctx)
        }
    }

    /// This function should always return the latest supported version.
    /// If the inner version is old, we upgrade it lazily in-place.
    public(package) fun load_validator_maybe_upgrade(self: &mut ValidatorWrapper): &mut Validator {
        upgrade_to_latest(self);
        versioned::load_value_mut<Validator>(&mut self.inner)
    }

    /// Destroy the wrapper and retrieve the inner validator object.
    public(package) fun destroy(mut self: ValidatorWrapper): Validator {
        upgrade_to_latest(&mut self);
        let ValidatorWrapper { inner } = self;
        versioned::destroy(inner)
    }

    fun upgrade_to_latest(self: &mut ValidatorWrapper) {
        let version = version(self);
        // TODO: When new versions are added, we need to explicitly upgrade here.
        assert!(version == 1, EInvalidVersion);
    }

    fun version(self: &ValidatorWrapper): u64 {
        versioned::version(&self.inner)
    }
}
