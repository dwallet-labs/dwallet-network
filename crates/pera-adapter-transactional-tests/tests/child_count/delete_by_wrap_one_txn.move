// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// DEPRECATED child count no longer tracked
// tests invalid wrapping of a parent object with children, in a single transaction

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use pera::dynamic_object_field as ofield;

    public struct S has key, store {
        id: pera::object::UID,
    }

    public struct R has key {
        id: pera::object::UID,
        s: S,
    }

    public entry fun test_wrap(ctx: &mut TxContext) {
        let mut id = pera::object::new(ctx);
        let child = S { id: pera::object::new(ctx) };
        ofield::add(&mut id, 0, child);
        let parent = S { id };
        let r = R { id: pera::object::new(ctx), s: parent };
        pera::transfer::transfer(r, tx_context::sender(ctx))
    }
}

//# run test::m::test_wrap --sender A
