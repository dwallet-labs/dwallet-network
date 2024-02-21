// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses test=0x0 --accounts A --simulator

//# publish --sender A
module test::fake {
    use std::option;
    use dwallet::coin;
    use dwallet::transfer;
    use dwallet::tx_context::{Self, TxContext};

    struct FAKE has drop {}

    fun init(witness: FAKE, ctx: &mut TxContext){
        let (treasury_cap, metadata) = coin::create_currency(witness, 2, b"FAKE", b"", b"", option::none(), ctx);
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
    }
}

//# create-checkpoint

//# run-graphql
{
  coinMetadata(coinType: "@{test}::fake::FAKE") {
    decimals
    name
    symbol
    description
    iconUrl
    supply
    asMoveObject {
      hasPublicTransfer
    }
  }
}


//# programmable --sender A --inputs object(1,2) 100 @A
//> 0: dwallet::coin::mint<test::fake::FAKE>(Input(0), Input(1));
//> TransferObjects([Result(0)], Input(2))

//# create-checkpoint

//# run-graphql
{
  coinMetadata(coinType: "@{test}::fake::FAKE") {
    decimals
    name
    symbol
    description
    iconUrl
    supply
    asMoveObject {
      hasPublicTransfer
    }
  }
}
