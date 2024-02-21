// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// tests cannot call init with programmable transactions

//# init --addresses test=0x0 --accounts A

//# publish
module test::m1 {
    fun init(_: &mut dwallet::tx_context::TxContext) {}
}

//# programmable
//> 0: test::m1::init();
