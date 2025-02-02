// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridged_usdt::usdt {
    use std::option;

    use ika::coin;
    use ika::transfer;
    use ika::tx_context;
    use ika::tx_context::TxContext;

    struct USDT has drop {}

    const DECIMAL: u8 = 6;

    fun init(otw: USDT, ctx: &mut TxContext) {
        let (treasury_cap, metadata) = coin::create_currency(
            otw,
            DECIMAL,
            b"USDT",
            b"Tether",
            b"Bridged Tether token",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
    }
}
