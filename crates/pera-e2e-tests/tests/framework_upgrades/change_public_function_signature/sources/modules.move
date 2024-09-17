// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module pera_system::msim_extra_1 {
    use pera::object::UID;
    use pera::tx_context::TxContext;

    public struct Type has drop {
        x: u64,
    }

    public struct Obj has key {
        id: UID,
    }

    public struct AlmostObj {
        id: UID,
    }

    public fun canary(): u64 {
        private_function(46)
    }

    entry fun mint(_ctx: &mut TxContext) {}

    entry fun entry_fun() {}

    fun private_function(x: u64): u64 {
        x + 1
    }

    fun private_function_2(x: u64): u64 { x }
    fun private_function_3(_x: u64) {}

    public fun generic<T: copy + drop>(_t: T, _x: u64) {}
}
