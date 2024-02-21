// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// Test the change of APY with heavy transactions

//# init --simulator --accounts A --addresses P0=0x0

//# advance-epoch

//# create-checkpoint

//# publish --sender A --gas-budget 9999999999
module P0::m {
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::{sender, TxContext};
    use std::vector;

    struct Big has key, store {
        id: UID,
        weight: vector<u8>,
    }

    fun weight(): vector<u8> {
        let i = 0;
        let v = vector[];
        while (i < 248 * 1024) {
            vector::push_back(&mut v, 42);
            i = i + 1;
        };
        v
    }
    
    public entry fun new(ctx: &mut TxContext){
        let id = object::new(ctx);
        let w = weight();
        dwallet::transfer::public_transfer(
            Big { id, weight: w }, 
            sender(ctx)
        )
    }
}

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# create-checkpoint

//# advance-epoch

//# run-graphql
{
  epoch(id: 1) {
    validatorSet {
      activeValidators {
        apy
        name
      }
    }
  }
}

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# run P0::m::new --sender A

//# create-checkpoint

//# advance-epoch

// check the epoch metrics

//# run-graphql
{
  epoch(id: 2) {
    validatorSet {
      activeValidators {
        apy
        name
      }
    }
  }
}
