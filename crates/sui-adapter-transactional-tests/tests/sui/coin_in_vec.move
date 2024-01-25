// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses test=0x0 --accounts A

//# publish --sender A

module test::coin_in_vec {
    use std::vector;
    use dwallet::coin::Coin;
    use dwallet::object::{Self, UID};
    use dwallet::dwlt::DWLT;
    use dwallet::transfer;
    use dwallet::tx_context::{Self, TxContext};

    struct Wrapper has key {
        id: UID,
        coins: vector<Coin<SUI>>,
    }

    fun init(ctx: &mut TxContext) {
        transfer::transfer(Wrapper { id: object::new(ctx), coins: vector[] }, tx_context::sender(ctx));
    }

    public fun deposit(wrapper: &mut Wrapper, c: Coin<SUI>) {
        vector::push_back(&mut wrapper.coins, c)
    }

    public fun withdraw(wrapper: &mut Wrapper, ctx: &mut TxContext) {
        transfer::public_transfer(vector::pop_back(&mut wrapper.coins), tx_context::sender(ctx))
    }
}

//# programmable --sender A --inputs 10 @A
//> SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

//# run test::coin_in_vec::deposit --args object(1,0) object(2,0) --sender A

//# run test::coin_in_vec::withdraw --args object(1,0) --sender A
