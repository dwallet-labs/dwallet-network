// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module z::a {
    public fun bar(x: u64): u64 {
        z::b::foo(dwallet::math::max(x, 42))
    }
}
