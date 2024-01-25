// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// tests cannot call private with programmable transactions

//# init --addresses test=0x0 --accounts A

//# publish
module test::m1 {
    fun priv(_: &mut dwallet::tx_context::TxContext) {}
}

//# programmable
//> 0: test::m1::priv();
