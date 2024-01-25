// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// tests that gas-by-value rules come after taken/borrow rules

//# init --addresses test=0x0 --accounts A

//# publish
module test::m1 {
    public fun take<T: key>(_: T) { abort 0 }
    public fun imm<T: key>(_: &T, _: T) { abort 0 }
    public fun mut<T: key>(_: &mut T, _: T) { abort 0 }
}

//# programmable --sender A --inputs @A
//> TransferObjects([Gas], Input(0));
//> test::m1::take<dwallet::coin::Coin<dwallet::dwlt::DWLT>>(Gas)

//# programmable
//> test::m1::imm<dwallet::coin::Coin<dwallet::dwlt::DWLT>>(Gas, Gas)

//# programmable
//> test::m1::mut<dwallet::coin::Coin<dwallet::dwlt::DWLT>>(Gas, Gas)
