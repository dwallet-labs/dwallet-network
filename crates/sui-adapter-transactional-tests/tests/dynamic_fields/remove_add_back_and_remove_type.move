// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// This test attempts to remove a child, add it back, remove it again, and then transfer/delete it.
// This is an interesting test case because when child objects are removed, added back and removed again,
// they won't show up in the child_object_effects in the object runtiem. We must look at either transfers
// or deleted_object_ids to figure them out.

//# init --addresses test=0x0 --accounts A

//# publish
module test::m1 {
    use dwallet::object::{Self, UID};
    use dwallet::transfer;
    use dwallet::tx_context::{Self, TxContext};

    struct Object has key, store {
        id: UID,
    }

    struct C1 has key, store {
        id: UID,
    }

    struct C2 has key, store {
        id: UID,
    }

    public entry fun create(ctx: &mut TxContext) {
        let o = Object { id: object::new(ctx) };
        transfer::public_transfer(o, tx_context::sender(ctx))
    }

    public entry fun test_dof(parent: &mut Object, ctx: &mut TxContext) {
        let c1 = C1 { id: object::new(ctx) };
        dwallet::dynamic_object_field::add(&mut parent.id, 0, c1);
        let C1 { id } = dwallet::dynamic_object_field::remove(&mut parent.id, 0);
        object::delete(id);

        let c2 = C2 { id: object::new(ctx) };
        dwallet::dynamic_object_field::add(&mut parent.id, 0, c2);
        let C2 { id } = dwallet::dynamic_object_field::remove(&mut parent.id, 0);
        object::delete(id);
    }

    public entry fun test_df(parent: &mut Object) {
        dwallet::dynamic_field::add(&mut parent.id, 0, b"true");
        let _: vector<u8> = dwallet::dynamic_field::remove(&mut parent.id, 0);
        dwallet::dynamic_field::add(&mut parent.id, 0, true);
        let _: bool = dwallet::dynamic_field::remove(&mut parent.id, 0);
    }
}

//# run test::m1::create --sender A

//# run test::m1::test_dof --args object(2,0) --sender A

//# run test::m1::test_df --args object(2,0) --sender A
