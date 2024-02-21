// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses Test=0x0 --accounts A

//# publish --upgradeable --sender A
module Test::M1 {
    use dwallet::tx_context::TxContext;
    fun init(_ctx: &mut TxContext) { }
    public fun f1() { }
}

//# upgrade --package Test --upgrade-capability 1,1 --sender A
module Test::M1 {
    use dwallet::tx_context::TxContext;
    fun init(_ctx: &mut TxContext) { }
}
