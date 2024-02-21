// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// tests various valid operations involving TransferObjects

//# init --addresses test=0x0 --accounts A B

//# publish
module test::m1 {
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::TxContext;

    struct Pub has key, store {
        id: UID,
        value: u64,
    }

    struct Cup<phantom T> has key, store {
        id: UID,
    }

    public fun new(ctx: &mut TxContext): Pub {
        Pub { id: object::new(ctx), value: 112 }
    }

    public fun cup<T>(ctx: &mut TxContext): Cup<T> {
        Cup { id: object::new(ctx) }
    }

    public fun addr(a1: address, cond: bool): address {
        if (cond) a1 else @0x0
    }
}

// simple
//# programmable --sender A --inputs @A
//> 0: test::m1::new();
//> TransferObjects([Result(0)], Input(0));

//# view-object 0,0

// cast using a Move function
//# programmable --sender A --inputs 0u256
//> 0: dwallet::address::from_u256(Input(0));
//> 1: test::m1::new();
//> TransferObjects([Result(1)], Result(0));

//# view-object 4,0

// compilicated Move logic
//# programmable --sender A --inputs @B true
//> 0: dwallet::address::to_u256(Input(0));
//> 1: dwallet::address::from_u256(Result(0));
//> 2: test::m1::new();
//> 3: test::m1::addr(Result(1), Input(1));
//> TransferObjects([Result(2)], Result(3));

//# view-object 6,0

// many object types
//# programmable --sender A --inputs @B true
//> 0: dwallet::address::to_u256(Input(0));
//> 1: dwallet::address::from_u256(Result(0));
//> 2: test::m1::new();
//> 3: test::m1::addr(Result(1), Input(1));
//> 4: test::m1::cup<dwallet::object::ID>();
//> 5: test::m1::cup<test::m1::Pub>();
//> TransferObjects([Result(4), Result(2), Result(5)], Result(3));

//# view-object 8,0

//# view-object 8,1

//# view-object 8,2
