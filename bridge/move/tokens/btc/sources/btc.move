// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module bridged_btc::btc {
    use std::option;

    use pera::coin;
    use pera::transfer;
    use pera::tx_context;
    use pera::tx_context::TxContext;

    struct BTC has drop {}

    const DECIMAL: u8 = 8;

    fun init(otw: BTC, ctx: &mut TxContext) {
        let (treasury_cap, metadata) = coin::create_currency(
            otw,
            DECIMAL,
            b"BTC",
            b"Bitcoin",
            b"Bridged Bitcoin token",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
    }
}
