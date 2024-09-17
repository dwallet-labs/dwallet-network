// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// DEPRECATED child count no longer tracked
// tests valid deletion of an object that has children
// Both child and parent are deleted in one transaction

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use pera::dynamic_object_field as ofield;

    public struct S has key, store {
        id: pera::object::UID,
    }

    public struct R has key, store {
        id: pera::object::UID,
        s: S,
    }

    public entry fun mint(ctx: &mut TxContext) {
        let id = pera::object::new(ctx);
        pera::transfer::public_transfer(S { id }, tx_context::sender(ctx))
    }

    public entry fun add(parent: &mut S, idx: u64, ctx: &mut TxContext) {
        let child = S { id: pera::object::new(ctx) };
        ofield::add(&mut parent.id, idx, child);
    }

    public entry fun remove(parent: &mut S, idx: u64) {
        let S { id } = ofield::remove(&mut parent.id, idx);
        pera::object::delete(id)
    }

    public entry fun remove_and_add(parent: &mut S, idx: u64) {
        let child: S = ofield::remove(&mut parent.id, idx);
        ofield::add(&mut parent.id, idx, child)
    }

    public entry fun remove_and_wrap(parent: &mut S, idx: u64, ctx: &mut TxContext) {
        let child: S = ofield::remove(&mut parent.id, idx);
        ofield::add(&mut parent.id, idx, R { id: pera::object::new(ctx), s: child })
    }

    public entry fun delete(s: S) {
        let S { id } = s;
        pera::object::delete(id)
    }

    public entry fun wrap(s: S, ctx: &mut TxContext) {
        let r = R { id: pera::object::new(ctx), s };
        pera::transfer::public_transfer(r, tx_context::sender(ctx))
    }

    public entry fun remove_and_delete(mut s: S, idx: u64) {
        let S { id } = ofield::remove(&mut s.id, idx);
        pera::object::delete(id);
        let S { id } = s;
        pera::object::delete(id)
    }
}

//
// Test deleting parent and child in the same txn, parent first
//

//# run test::m::mint --sender A

//# run test::m::add --sender A --args object(2,0) 0

//# view-object 2,0

//# run test::m::remove_and_delete --sender A --args object(2,0) 0
