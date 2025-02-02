// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[allow(unused_field)]
module ika::object {
    /// A test version of the UID type to allow us to have types with
    /// `key` in these test packages. It has a different public structure to
    /// the real UID, but that is not relevant.
    public struct UID has store {
        id: address,
    }
}
