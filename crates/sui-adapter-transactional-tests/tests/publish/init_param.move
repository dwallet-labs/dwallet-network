//# init --addresses Test=0x0
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# publish

// initializer not valid due extra non-ctx param

module Test::M1 {
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::{Self, TxContext};
    use dwallet::transfer;

    struct Object has key, store {
        id: UID,
        value: u64,
    }

    // value param invalid
    fun init(ctx: &mut TxContext, value: u64) {
        let singleton = Object { id: object::new(ctx), value };
        transfer::public_transfer(singleton, tx_context::sender(ctx))
    }
}
