// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module test_coin::test {
    use std::option;
    use ika::coin;
    use ika::transfer;
    use ika::url;
    use ika::tx_context::{Self, TxContext};

    public struct TEST has drop {}

    fun init(witness: TEST, ctx: &mut TxContext) {
        let (mut treasury_cap, metadata) = coin::create_currency<TEST>(
            witness,
            2,
            b"TEST",
            b"Test Coin",
            b"Test coin metadata",
            option::some(url::new_unsafe_from_bytes(b"http://ika.io")),
            ctx
        );

        coin::mint_and_transfer<TEST>(&mut treasury_cap, 1000, tx_context::sender(ctx), ctx);

        transfer::public_share_object(metadata);
        transfer::public_share_object(treasury_cap)
    }
}
