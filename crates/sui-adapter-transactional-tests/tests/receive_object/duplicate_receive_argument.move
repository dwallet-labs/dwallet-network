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

    public entry fun send_back(parent: &mut A, x: Receiving<B>) {
        let b = transfer::receive(&mut parent.id, x);
        let parent_address = object::id_address(parent);
        transfer::public_transfer(b, parent_address);
    }
}

//# run tto::M1::start

//# programmable --inputs object(2,0) receiving(2,1) 
//> tto::M1::send_back(Input(0), Input(1))

// Duplicate object ref in input
//# programmable --inputs object(2,0) receiving(2,1) receiving(2,1)
//> tto::M1::send_back(Input(0), Input(1))

// Invalid signature for the receiving object since we try to use it as a normal input
//# programmable --inputs object(2,1) receiving(2,1)
//> tto::M1::send_back(Input(0), Input(1))
