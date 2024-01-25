// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_field)]
module dwallet::object {
    /// A test version of the UID type to allow us to have types with
    /// `key` in these test packages. It has a different structure to
    /// the real UID, but that is not relevant.
    struct UID has store {
        id: address,
    }
}
