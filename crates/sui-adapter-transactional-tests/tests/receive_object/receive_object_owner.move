// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses tto=0x0 --accounts A

//# publish
module tto::M1 {
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::{Self, TxContext};
    use dwallet::transfer::{Self, Receiving};
    use dwallet::dynamic_object_field as dof;

    const KEY: u64 = 0;

    struct A has key, store {
        id: UID,
        value: u64,
    }

    public fun start(ctx: &mut TxContext) {
        let a = A { id: object::new(ctx), value: 0 };
        dof::add(&mut a.id, KEY, A { id: object::new(ctx), value: 0 });
        transfer::public_transfer(a, tx_context::sender(ctx));
    }

    public entry fun receive(parent: &mut A, x: Receiving<A>) {
        let b = transfer::receive(&mut parent.id, x);
        dof::add(&mut parent.id, KEY, b);
    }
}

//# run tto::M1::start --sender A

//# view-object 2,0

//# view-object 2,1

//# view-object 2,2

// Try to receive an object with an object
//# run tto::M1::receive --args object(2,2) receiving(2,1) --sender A

// Try to receive another object with an object owner
//# run tto::M1::receive --args object(2,2) receiving(2,0) --sender A
