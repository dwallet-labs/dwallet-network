// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses Test=0x0

//# publish
module Test::M1 {
   use dwallet::tx_context::TxContext;
   fun init(_ctx: &mut TxContext) { }
}

module Test::M2 {
    use dwallet::tx_context::TxContext;
    fun init(_ctx: &mut TxContext) { }
}

//# view-object 1,0
 
