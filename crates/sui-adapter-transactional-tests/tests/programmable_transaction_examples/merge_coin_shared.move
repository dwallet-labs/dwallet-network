// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// test invalid usages of shared coin

//# init --addresses test=0x0 --accounts A --shared-object-deletion true

//# publish

module test::m1 {
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::TxContext;
    use dwallet::transfer;

    // not a native coin, but same type structure and BCS layout
    struct Coin has key, store {
        id: UID,
        value: u64,
    }

    public fun mint_shared(ctx: &mut TxContext) {
        transfer::public_share_object(
            Coin {
                id: object::new(ctx),
                value: 1000000,
            }
        )
    }

}

//# run test::m1::mint_shared

//# view-object 2,0

//# programmable --sender A --inputs object(2,0)
//> MergeCoins(Gas, [Input(0)])
