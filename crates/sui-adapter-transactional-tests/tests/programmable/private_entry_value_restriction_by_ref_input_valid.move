// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// tests that object values can be used private entry functions if they have been used
// by other entry functions or primitive commands

//# init --addresses test=0x0 --accounts A

//# publish
module test::m1 {
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::TxContext;
    use dwallet::coin::Coin;
    use dwallet::dwlt::DWLT;

    struct R has key, store { id: UID }
    public fun r(ctx: &mut TxContext): R { R { id: object::new(ctx) } }

    public fun v(): u64 { 100 }

    public entry fun clean(_: &mut R, _extra_arg: u64) {}
    entry fun priv(_: &mut R) { }

    entry fun coin(_: &mut Coin<SUI>) {}
}

//# programmable --sender A --inputs @A
//> 0: test::m1::r();
//> TransferObjects([Result(0)], Input(0))

//# programmable --sender A --inputs object(2,0) 200
//> 0: test::m1::v();
//> test::m1::clean(Input(0), Result(0));
//> test::m1::priv(Input(0));
//> test::m1::clean(Input(0), Input(1));
//> test::m1::priv(Input(0));
//> test::m1::priv(Input(0));
//> test::m1::priv(Input(0));

//# programmable --sender A --inputs @A  --gas-budget 10000000000
//> 0: test::m1::v();
//> 1: SplitCoins(Gas, [Result(0)]);
//> test::m1::coin(Gas);
//> test::m1::coin(Result(1));
//> TransferObjects([Result(1)], Input(0))
