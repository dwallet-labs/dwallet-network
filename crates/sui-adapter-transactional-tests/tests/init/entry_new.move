// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// init with entry is no longer allowed

//# init --addresses test=0x0

//# publish
module test::m {
    use dwallet::tx_context::TxContext;
    entry fun init(_: &mut TxContext) {
    }
}

// TODO double check this error
//# run test::m::init
