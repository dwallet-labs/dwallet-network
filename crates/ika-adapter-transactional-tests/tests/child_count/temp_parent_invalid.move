// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests the invalid creation and deletion of a parent object

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    public struct S has key, store {
        id: ika::object::UID,
    }

    public entry fun t(ctx: &mut TxContext) {
        let mut parent = ika::object::new(ctx);
        let child = S { id: ika::object::new(ctx) };
        ika::dynamic_object_field::add(&mut parent, 0, child);
        ika::object::delete(parent);
    }
}

//# run test::m::t --sender A
