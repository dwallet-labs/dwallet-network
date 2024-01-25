// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// DEPRECATED child count no longer tracked
// tests invalid wrapping of a parent object with children, in a single transaction

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use dwallet::tx_context::{Self, TxContext};
    use dwallet::dynamic_object_field as ofield;

    struct S has key, store {
        id: dwallet::object::UID,
    }

    struct R has key {
        id: dwallet::object::UID,
        s: S,
    }

    public entry fun test_wrap(ctx: &mut TxContext) {
        let id = dwallet::object::new(ctx);
        let child = S { id: dwallet::object::new(ctx) };
        ofield::add(&mut id, 0, child);
        let parent = S { id };
        let r = R { id: dwallet::object::new(ctx), s: parent };
        dwallet::transfer::transfer(r, tx_context::sender(ctx))
    }
}

//# run test::m::test_wrap --sender A
