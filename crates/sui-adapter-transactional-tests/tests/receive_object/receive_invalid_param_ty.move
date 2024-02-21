// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses tto=0x0

//# publish
module tto::M1 {
    use dwallet::object::{Self, UID, ID};
    use dwallet::tx_context::{Self, TxContext};
    use dwallet::transfer::{Self, Receiving};

    struct A has key, store {
        id: UID,
    }

    struct B has key, store {
        id: UID,
    }

    struct Fake<phantom T> has drop { }

    struct FakeSameLayout<phantom T> has drop {
        id: ID, 
        version: u64,
    }

    public fun start(ctx: &mut TxContext) {
        let a = A { id: object::new(ctx) };
        let a_address = object::id_address(&a);
        let b = B { id: object::new(ctx) };
        transfer::public_transfer(a, tx_context::sender(ctx));
        transfer::public_transfer(b, a_address);
    }

    public entry fun receiver(_x: u64) { }
    public fun receiver2(_x: Fake<B>) { }
    public fun receiver3(_x: &Fake<B>) { }

    public fun receiver4(_x: FakeSameLayout<B>) { }
    public fun receiver5(_x: &FakeSameLayout<B>) { }

    public fun receiver6(_x: Receiving<B>) { }
}

//# run tto::M1::start

//# view-object 2,0

//# view-object 2,1

//# run tto::M1::receiver --args receiving(2,1)

//# run tto::M1::receiver2 --args receiving(2,1)

//# run tto::M1::receiver3 --args receiving(2,1)

//# run tto::M1::receiver4 --args receiving(2,1)

//# run tto::M1::receiver5 --args receiving(2,1)

//# run tto::M1::receiver6 --args object(2,1)

//# run tto::M1::receiver6 --args object(2,0)

//# run tto::M1::receiver6 --args receiving(2,0)

//# run tto::M1::receiver6 --args 0

//# run tto::M1::receiver6 --args vector[0,0,0,0,0,0,0,0,0,0]
