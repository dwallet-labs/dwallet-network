// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module base::base_module {

    const B: u64 = 0;

    struct X {
        field0: u64,
        field1: u64,
    }

    public fun public_fun(): u64 { B }
    fun private_fun(): u64 { B }
    entry fun private_entry_fun(_x: u64) { }
}
