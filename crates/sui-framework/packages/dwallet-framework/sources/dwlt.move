// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// Coin<SUI> is the token used to pay for gas in Sui.
/// It has 9 decimals, and the smallest unit (10^-9) is called "mist".
/// todo: rename to dwallet.
module dwallet::dwlt {
    use std::option;
    use dwallet::tx_context::{Self, TxContext};
    use dwallet::balance::{Balance};
    use dwallet::transfer;
    use dwallet::coin::{Self, TreasuryCap};

    const EAlreadyMinted: u64 = 0;
    /// Sender is not @0x0 the system address.
    const ENotSystemAddress: u64 = 1;

    #[allow(unused_const)]
    /// The amount of Mist per Sui token based on the the fact that mist is
    /// 10^-9 of a Sui token
    const MIST_PER_SUI: u64 = 1_000_000_000;

    #[allow(unused_const)]
    /// The total supply of Sui denominated in whole Sui tokens (10 Billion)
    const TOTAL_SUPPLY_SUI: u64 = 10_000_000_000;

    /// The total supply of Sui denominated in Mist (10 Billion * 10^9)
    const TOTAL_SUPPLY_MIST: u64 = 10_000_000_000_000_000_000;

    /// Name of the coin
    struct DWLT has drop {}

    #[allow(unused_function)]
    /// Register the `SUI` Coin to acquire its `Supply`.
    /// This should be called only once during genesis creation.
    fun new(ctx: &mut TxContext): (TreasuryCap<DWLT>, Balance<DWLT>) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        assert!(tx_context::epoch(ctx) == 0, EAlreadyMinted);

        let (treasury, metadata) = coin::create_currency(
            DWLT {},
            9,
            b"DWLT",
            b"dWallet",
            // TODO: add appropriate description and logo url
            b"",
            option::none(),
            ctx
        );
        transfer::public_share_object(metadata);

        let total = coin::mint_balance(&mut treasury, TOTAL_SUPPLY_MIST);
        (treasury, total)
    }

    public entry fun transfer(c: coin::Coin<DWLT>, recipient: address) {
        transfer::public_transfer(c, recipient)
    }
}
