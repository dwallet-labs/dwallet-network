// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Coin<IKA> is the token used to pay for gas in Ika.
/// It has 9 decimals, and the smallest unit (10^-9) is called "nika".
module ika::ika;

use ika::balance::Balance;
use ika::coin;

const EAlreadyMinted: u64 = 0;
/// Sender is not @0x0 the system address.
const ENotSystemAddress: u64 = 1;

#[allow(unused_const)]
/// The amount of NIka per Ika token based on the fact that nika is
/// 10^-9 of a Ika token
const NIKA_PER_IKA: u64 = 1_000_000_000;

#[allow(unused_const)]
/// The total supply of Ika denominated in whole Ika tokens (10 Billion)
const TOTAL_SUPPLY_IKA: u64 = 10_000_000_000;

/// The total supply of Ika denominated in NIka (10 Billion * 10^9)
const TOTAL_SUPPLY_NIKA: u64 = 10_000_000_000_000_000_000;

/// Name of the coin
public struct IKA has drop {}

#[allow(unused_function)]
/// Register the `IKA` Coin to acquire its `Supply`.
/// This should be called only once during genesis creation.
fun new(ctx: &mut TxContext): Balance<IKA> {
    assert!(ctx.sender() == @0x0, ENotSystemAddress);
    assert!(ctx.epoch() == 0, EAlreadyMinted);

    let (treasury, metadata) = coin::create_currency(
        IKA {},
        9,
        b"IKA",
        b"Ika",
        // TODO: add appropriate description and logo url
        b"",
        option::none(),
        ctx,
    );
    transfer::public_freeze_object(metadata);
    let mut supply = treasury.treasury_into_supply();
    let total_ika = supply.increase_supply(TOTAL_SUPPLY_NIKA);
    supply.destroy_supply();
    total_ika
}

public entry fun transfer(c: coin::Coin<IKA>, recipient: address) {
    transfer::public_transfer(c, recipient)
}
