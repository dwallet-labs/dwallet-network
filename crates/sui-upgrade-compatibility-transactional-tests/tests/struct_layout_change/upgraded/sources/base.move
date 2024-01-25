// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module base::base_module {
    struct X {
        field0: u64,
        field1: u64,
        extra_field: u64,
    }

    public fun public_fun(): u64 { 0 }
    fun private_fun(): u64 { 0 }
    entry fun private_entry_fun(_x: u64) { }
}
