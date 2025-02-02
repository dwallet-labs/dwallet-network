
// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// Coin<IKA> is the token used to pay for gas in Ika.
/// It has 9 decimals, and the smallest unit (10^-9) is called "nika".
/// Module: ika
module ika::ika;

use sui::coin;

/// The OTW for the `IKA` coin.
public struct IKA has drop {}

#[allow(unused_const)]
/// The amount of NIka per Ika token based on the fact that nika is
/// 10^-9 of a Ika token
const NIKA_PER_IKA: u64 = 1_000_000_000;

#[allow(lint(share_owned))]
fun init(otw: IKA, ctx: &mut TxContext) {
    let (treasury_cap, coin_metadata) = coin::create_currency(
        otw,
        9, // decimals,
        b"IKA", // symbol,
        b"Ika", // name,
        b"IKA Token", // description,
        option::none(), // url (currently, empty)
        ctx,
    );

    transfer::public_transfer(treasury_cap, ctx.sender());
    transfer::public_share_object(coin_metadata);
}

// === Test only ===

#[test_only]
use sui::test_scenario;

#[test_only]
public fun init_for_testing(ctx: &mut TxContext) {
    init(IKA {}, ctx);
}

#[test]
fun test_init() {
    let user = @0xa11ce;
    let mut test = test_scenario::begin(user);
    init(IKA {}, test.ctx());
    test.next_tx(user);

    let treasury_cap = test.take_from_address<coin::TreasuryCap<IKA>>(user);
    assert!(treasury_cap.total_supply() == 0);
    test.return_to_sender(treasury_cap);

    let coin_metadata = test.take_shared<coin::CoinMetadata<IKA>>();

    assert!(coin_metadata.get_decimals() == 9);
    assert!(coin_metadata.get_symbol() == b"IKA".to_ascii_string());
    assert!(coin_metadata.get_name() == b"IKA".to_string());
    assert!(coin_metadata.get_description() == b"IKA Token".to_string());
    assert!(coin_metadata.get_icon_url() == option::none());

    test_scenario::return_shared(coin_metadata);
    test.end();
}