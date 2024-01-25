// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// DEPRECATED child count no longer tracked
// tests valid freezing of an object that has children
// child is deleted and parent is frozen in one transaction

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use dwallet::tx_context::{Self, TxContext};
    use dwallet::dynamic_object_field as ofield;

    struct S has key, store {
        id: dwallet::object::UID,
    }

    struct R has key, store {
        id: dwallet::object::UID,
        s: S,
    }

    public entry fun mint(ctx: &mut TxContext) {
        let id = dwallet::object::new(ctx);
        dwallet::transfer::public_transfer(S { id }, tx_context::sender(ctx))
    }

    public entry fun add(parent: &mut S, idx: u64, ctx: &mut TxContext) {
        let child = S { id: dwallet::object::new(ctx) };
        ofield::add(&mut parent.id, idx, child);
    }

    public entry fun remove(parent: &mut S, idx: u64) {
        let S { id } = ofield::remove(&mut parent.id, idx);
        dwallet::object::delete(id)
    }

    public entry fun remove_and_add(parent: &mut S, idx: u64) {
        let child: S = ofield::remove(&mut parent.id, idx);
        ofield::add(&mut parent.id, idx, child)
    }

    public entry fun remove_and_wrap(parent: &mut S, idx: u64, ctx: &mut TxContext) {
        let child: S = ofield::remove(&mut parent.id, idx);
        ofield::add(&mut parent.id, idx, R { id: dwallet::object::new(ctx), s: child })
    }

    public entry fun delete(s: S) {
        let S { id } = s;
        dwallet::object::delete(id)
    }

    public entry fun wrap(s: S, ctx: &mut TxContext) {
        let r = R { id: dwallet::object::new(ctx), s };
        dwallet::transfer::public_transfer(r, tx_context::sender(ctx))
    }

    public entry fun remove_and_freeze(s: S, idx: u64) {
        let S { id } = ofield::remove(&mut s.id, idx);
        dwallet::object::delete(id);
        dwallet::transfer::public_freeze_object(s)
    }
}

//# run test::m::mint --sender A

//# run test::m::add --sender A --args object(2,0) 0

//# view-object 2,0

//# run test::m::remove_and_freeze --sender A --args object(2,0) 0
