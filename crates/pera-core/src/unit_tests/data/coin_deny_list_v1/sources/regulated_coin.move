// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module coin_deny_list_v1::regulated_coin {
    use std::option;
    use pera::coin;
    use pera::object::UID;
    use pera::transfer;
    use pera::tx_context;
    use pera::tx_context::TxContext;

    public struct REGULATED_COIN has drop {}

    public struct Wallet has key {
        id: UID,
    }

    fun init(otw: REGULATED_COIN, ctx: &mut TxContext) {
        let (treasury_cap, deny_cap, metadata) = coin::create_regulated_currency(
            otw,
            9,
            b"RC",
            b"REGULATED_COIN",
            b"A new regulated coin",
            option::none(),
            ctx
        );
        transfer::public_transfer(deny_cap, tx_context::sender(ctx));
        transfer::public_freeze_object(treasury_cap);
        transfer::public_freeze_object(metadata);
    }
}
