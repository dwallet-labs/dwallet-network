// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module base_addr::friend_module {

    struct X has store, drop {
        v: bool,
    }

    struct Y has store, drop {
        v: u64,
    }

    public fun make_x(v: bool): X {
        X { v }
    }

    public fun make_y(v: u64): Y {
        Y { v }
    }
}
