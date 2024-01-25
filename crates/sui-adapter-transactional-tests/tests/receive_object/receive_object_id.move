// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses tto=0x0

//# publish
module tto::M1 {
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::{Self, TxContext};
    use dwallet::transfer::{Self, Receiving};

    struct A has key, store {
        id: UID,
    }

    struct B has key, store {
        id: UID,
    }

    public fun start(ctx: &mut TxContext) {
        let a = A { id: object::new(ctx) };
        let a_address = object::id_address(&a);
        let b = B { id: object::new(ctx) };
        transfer::public_transfer(a, tx_context::sender(ctx));
        transfer::public_transfer(b, a_address);
    }

    public entry fun receiver(x: Receiving<B>) {
        transfer::receiving_object_id(&x);
    }
}

//# run tto::M1::start

//# view-object 2,0

//# view-object 2,1

//# run tto::M1::receiver --args receiving(2,1)

//# programmable --inputs receiving(2,1) 
//> dwallet::transfer::receiving_object_id<tto::M1::B>(Input(0))

//# programmable --inputs receiving(2,1) 
//> tto::M1::receiver(Input(0))

//# view-object 2,0

//# view-object 2,1
