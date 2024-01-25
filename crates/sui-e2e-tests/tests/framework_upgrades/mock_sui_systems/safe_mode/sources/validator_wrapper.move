// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dwallet_system::validator_wrapper {
    use dwallet::versioned::Versioned;

    friend dwallet_system::dwallet_system_state_inner;

    struct ValidatorWrapper has store {
        inner: Versioned
    }
}
