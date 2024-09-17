// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module c::c {
    public struct C {
        x: u64
    }

    public struct D {
        x: u64,
        y: u64,
    }

    public fun c(): u64 {
        43
    }
}
