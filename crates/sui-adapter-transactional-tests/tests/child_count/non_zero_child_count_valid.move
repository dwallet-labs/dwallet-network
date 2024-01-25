// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// DEPRECATED child count no longer tracked
// tests valid transfers of an object that has children

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
        let child = S { id: dwallet::object::new(ctx) };
        ofield::add(&mut id, 0, child);
        dwallet::transfer::public_transfer(S { id }, tx_context::sender(ctx))
    }

    public entry fun mint_and_share(ctx: &mut TxContext) {
        let id = dwallet::object::new(ctx);
        let child = S { id: dwallet::object::new(ctx) };
        ofield::add(&mut id, 0, child);
        dwallet::transfer::public_share_object(S { id })
    }

    public entry fun transfer(s: S, recipient: address) {
        dwallet::transfer::public_transfer(s, recipient)
    }

}

//
// Test share object allows non-zero child count
//

//# run test::m::mint_and_share --sender A

//# view-object 2,1

//
// Test transfer allows non-zero child count
//

//# run test::m::mint --sender A

//# run test::m::transfer --sender A --args object(4,2) @B

//# view-object 4,2

//
// Test TransferObject allows non-zero child count
//

//# run test::m::mint --sender A

//# transfer-object 7,1 --sender A --recipient B

//# view-object 7,1
