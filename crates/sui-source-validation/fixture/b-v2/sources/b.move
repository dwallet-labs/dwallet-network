// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module b::b {
    public fun b(): u64 {
        42
    }

    public fun c(): u64 {
        b() + 1
    }
}
