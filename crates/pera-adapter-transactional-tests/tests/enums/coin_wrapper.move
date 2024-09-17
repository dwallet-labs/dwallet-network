// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses Test=0x0 --accounts A

//# publish --upgradeable --sender A
module Test::f {
    use pera::coin::Coin;
    use pera::pera::PERA;

    public struct Other { }

    public enum CoinWrapper has store {
        Pera(Coin<PERA>),
        Other(Coin<Other>),
    }

    public struct CoinObject has key, store {
        id: UID,
        coin: CoinWrapper,
    }

    public fun split_off(coin: &mut CoinObject, amount: u64, ctx: &mut TxContext): CoinObject {
        match (&mut coin.coin) {
            CoinWrapper::Pera(c) => {
                let new_coin = CoinObject {
                    id: object::new(ctx),
                    coin: CoinWrapper::Pera(c.split(amount, ctx)),
                };
                new_coin
            },
            CoinWrapper::Other(c) => {
                let new_coin = CoinObject {
                    id: object::new(ctx),
                    coin: CoinWrapper::Other(c.split(amount, ctx)),
                };
                new_coin
            },
        }
    }

    public fun create_pera(coin: &mut Coin<PERA>, amount: u64, ctx: &mut TxContext): CoinObject {
        CoinObject {
            id: object::new(ctx),
            coin: CoinWrapper::Pera(coin.split(amount, ctx)),
        }
    }
}

//# programmable --sender A --inputs 10 @A
//> 0: Test::f::create_pera(Gas, Input(0));
//> 1: TransferObjects([Result(0)], Input(1))
