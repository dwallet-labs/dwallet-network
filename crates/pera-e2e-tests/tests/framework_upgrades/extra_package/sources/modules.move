// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module pera_extra::msim_extra_1 {
    use pera::object::{Self, UID};
    use pera::transfer;
    use pera::tx_context::TxContext;

    public struct S has key { id: UID }

    fun init(ctx: &mut TxContext) {
        transfer::share_object(S {
            id: object::new(ctx)
        })
    }

    public fun canary(): u64 {
        43
    }
}
