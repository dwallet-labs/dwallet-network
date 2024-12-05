// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module ika_system::validator_wrapper {
    use ika::versioned::Versioned;

    public struct ValidatorWrapper has store {
        inner: Versioned
    }
}
