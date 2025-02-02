// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests invalid wrapping of a parent object with children, in a single transaction

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use ika::dynamic_object_field as ofield;

    public struct S has key, store {
        id: ika::object::UID,
    }

    public struct R has key {
        id: ika::object::UID,
        s: S,
    }

    public entry fun test_wrap(ctx: &mut TxContext) {
        let mut id = ika::object::new(ctx);
        let child = S { id: ika::object::new(ctx) };
        ofield::add(&mut id, 0, child);
        let parent = S { id };
        let r = R { id: ika::object::new(ctx), s: parent };
        ika::transfer::transfer(r, tx_context::sender(ctx))
    }
}

//# run test::m::test_wrap --sender A
