// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// Coin<PERA> is the token used to pay for gas in Pera.
/// It has 9 decimals, and the smallest unit (10^-9) is called "npera".
module pera::pera {
    use pera::balance::Balance;
    use pera::coin;

    const EAlreadyMinted: u64 = 0;
    /// Sender is not @0x0 the system address.
    const ENotSystemAddress: u64 = 1;

    #[allow(unused_const)]
    /// The amount of NPera per Pera token based on the fact that npera is
    /// 10^-9 of a Pera token
    const NPERA_PER_PERA: u64 = 1_000_000_000;

    #[allow(unused_const)]
    /// The total supply of Pera denominated in whole Pera tokens (10 Billion)
    const TOTAL_SUPPLY_PERA: u64 = 10_000_000_000;

    /// The total supply of Pera denominated in NPera (10 Billion * 10^9)
    const TOTAL_SUPPLY_NPERA: u64 = 10_000_000_000_000_000_000;

    /// Name of the coin
    public struct PERA has drop {}

    #[allow(unused_function)]
    /// Register the `PERA` Coin to acquire its `Supply`.
    /// This should be called only once during genesis creation.
    fun new(ctx: &mut TxContext): Balance<PERA> {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);
        assert!(ctx.epoch() == 0, EAlreadyMinted);

        let (treasury, metadata) = coin::create_currency(
            PERA {},
            9,
            b"PERA",
            b"Pera",
            // TODO: add appropriate description and logo url
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        let mut supply = treasury.treasury_into_supply();
        let total_pera = supply.increase_supply(TOTAL_SUPPLY_NPERA);
        supply.destroy_supply();
        total_pera
    }

    public entry fun transfer(c: coin::Coin<PERA>, recipient: address) {
        transfer::public_transfer(c, recipient)
    }
}
