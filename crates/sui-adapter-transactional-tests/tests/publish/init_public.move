//# init --addresses Test=0x0
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# publish

// initializer not valid due to public visibility

module Test::M1 {
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::{Self, TxContext};
    use dwallet::transfer;

    struct Object has key, store {
        id: UID,
        value: u64,
    }

    // public initializer - should not be executed
    public fun init(ctx: &mut TxContext) {
        let value = 42;
        let singleton = Object { id: object::new(ctx), value };
        transfer::public_transfer(singleton, tx_context::sender(ctx))
    }
}
