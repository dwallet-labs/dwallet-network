// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// DEPRECATED child count no longer tracked
// tests the invalid creation and deletion of a parent object

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    public struct S has key, store {
        id: pera::object::UID,
    }

    public entry fun t(ctx: &mut TxContext) {
        let mut parent = pera::object::new(ctx);
        let child = S { id: pera::object::new(ctx) };
        pera::dynamic_object_field::add(&mut parent, 0, child);
        pera::object::delete(parent);
    }
}

//# run test::m::t --sender A
