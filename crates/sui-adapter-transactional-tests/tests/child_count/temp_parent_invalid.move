// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// DEPRECATED child count no longer tracked
// tests the invalid creation and deletion of a parent object

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use dwallet::tx_context::TxContext;

    struct S has key, store {
        id: dwallet::object::UID,
    }

    public entry fun t(ctx: &mut TxContext) {
        let parent = dwallet::object::new(ctx);
        let child = S { id: dwallet::object::new(ctx) };
        dwallet::dynamic_object_field::add(&mut parent, 0, child);
        dwallet::object::delete(parent);
    }
}

//# run test::m::t --sender A
