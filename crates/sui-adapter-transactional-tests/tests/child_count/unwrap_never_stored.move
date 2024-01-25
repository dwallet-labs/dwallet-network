// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// DEPRECATED child count no longer tracked
// tests deleting a wrapped object that has never been in storage

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use dwallet::tx_context::{Self, TxContext};

    struct S has key, store {
        id: dwallet::object::UID,
    }

    struct R has key {
        id: dwallet::object::UID,
        s: S,
    }

    public entry fun create(ctx: &mut TxContext) {
        let parent = dwallet::object::new(ctx);
        let child = S { id: dwallet::object::new(ctx) };
        dwallet::transfer::transfer(R { id: parent, s: child }, tx_context::sender(ctx))
    }

    public entry fun delete(r: R) {
        let R { id, s } = r;
        dwallet::object::delete(id);
        let S { id } = s;
        dwallet::object::delete(id);
    }
}

//
// Test sharing
//

//# run test::m::create --sender A

//# run test::m::delete --args object(2,0) --sender A
