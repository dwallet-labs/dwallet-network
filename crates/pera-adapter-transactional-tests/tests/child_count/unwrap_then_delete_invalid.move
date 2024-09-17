// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// DEPRECATED child count no longer tracked
// tests invalid deletion of an object that has children

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

    public entry fun mint(ctx: &mut TxContext) {
        let s = S { id: pera::object::new(ctx) };
        pera::transfer::transfer(s, tx_context::sender(ctx))
    }

    public entry fun add(parent: &mut S, idx: u64, ctx: &mut TxContext) {
        let child = S { id: pera::object::new(ctx) };
        ofield::add(&mut parent.id, idx, child);
    }

    public entry fun wrap(s: S, ctx: &mut TxContext) {
        let r = R { id: pera::object::new(ctx), s };
        pera::transfer::transfer(r, tx_context::sender(ctx))
    }

    public entry fun delete(r: R) {
        let R { id, s } = r;
        pera::object::delete(id);
        let S { id } = s;
        pera::object::delete(id);
    }
}

//# run test::m::mint --sender A

//# run test::m::add --sender A --args object(2,0) 0

//# run test::m::wrap --sender A --args object(2,0)
