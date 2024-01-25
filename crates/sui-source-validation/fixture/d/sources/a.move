// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module d::d {
    use b::b::b;
    use c::c::c;

    public fun d(): u64 {
        let var = 123;
        let _ = var + 4;
        b() + c()
    }
}
