// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// tests that object-by-value rules come after taken/borrow rules

//# init --addresses test=0x0 --accounts A

//# publish
module test::m1 {
    use dwallet::object::{Self, UID};
    use dwallet::transfer;
    use dwallet::tx_context::TxContext;

    struct R has key {
        id: UID,
    }

    public fun r(ctx: &mut TxContext): R {
        R { id: object::new(ctx) }
    }

    public fun share_r(ctx: &mut TxContext) {
        transfer::share_object(r(ctx))
    }

    public fun freeze_r(ctx: &mut TxContext) {
        transfer::share_object(r(ctx))
    }

    public fun imm(_: &R, _: R) { abort 0 }
    public fun mut(_: &mut R, _: R) { abort 0 }
}

//# programmable
//> test::m1::share_r();

//# programmable --inputs object(2,0)
//> test::m1::imm(Input(0), Input(0));

//# programmable --inputs object(2,0)
//> test::m1::mut(Input(0), Input(0));

//# programmable
//> test::m1::freeze_r();

//# programmable --inputs object(5,0)
//> test::m1::imm(Input(0), Input(0));

//# programmable --inputs object(5,0)
//> test::m1::mut(Input(0), Input(0));
