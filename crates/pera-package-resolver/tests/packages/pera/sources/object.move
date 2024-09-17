// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_field)]
module pera::object {
    /// A test version of the UID type to allow us to have types with
    /// `key` in these test packages. It has a different public structure to
    /// the real UID, but that is not relevant.
    public struct UID has store {
        id: address,
    }
}
