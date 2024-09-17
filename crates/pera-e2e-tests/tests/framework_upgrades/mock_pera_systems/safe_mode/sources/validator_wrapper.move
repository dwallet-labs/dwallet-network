// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module pera_system::validator_wrapper {
    use pera::versioned::Versioned;

    public struct ValidatorWrapper has store {
        inner: Versioned
    }
}
