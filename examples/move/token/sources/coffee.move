// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// This example illustrates how to use the `Token` without a `TokenPolicy`. And
/// only rely on `TreasuryCap` for minting and burning tokens.
module examples::coffee {
    use dwallet::tx_context::{sender, TxContext};
    use dwallet::coin::{Self, TreasuryCap, Coin};
    use dwallet::balance::{Self, Balance};
    use dwallet::token::{Self, Token};
    use dwallet::object::{Self, UID};
    use dwallet::dwlt::DWLT;

    /// Error code for incorrect amount.
    const EIncorrectAmount: u64 = 0;
    /// Trying to claim a free coffee without enough points.
    /// Or trying to transfer but not enough points to pay the commission.
    const ENotEnoughPoints: u64 = 1;

    /// 10 SUI for a coffee.
    const COFFEE_PRICE: u64 = 10_000_000_000;

    /// OTW for the Token.
    struct COFFEE has drop {}

    /// The shop that sells Coffee and allows to buy a Coffee if the customer
    /// has 10 COFFEE points.
    struct CoffeeShop has key {
        id: UID,
        /// The treasury cap for the `COFFEE` points.
        coffee_points: TreasuryCap<COFFEE>,
        /// The SUI balance of the shop; the shop can sell Coffee for SUI.
        balance: Balance<DWLT>,
    }

    /// Event marking that a Coffee was purchased; transaction sender serves as
    /// the customer ID.
    struct CoffeePurchased has copy, store, drop {}

    // Create and share the `CoffeeShop` object.
    fun init(otw: COFFEE, ctx: &mut TxContext) {
        let (coffee_points, metadata) = coin::create_currency(
            otw, 0, b"COFFEE", b"Coffee Point",
            b"Buy 4 coffees and get 1 free",
            std::option::none(),
            ctx
        );

        dwallet::transfer::public_freeze_object(metadata);
        dwallet::transfer::share_object(CoffeeShop {
            coffee_points,
            id: object::new(ctx),
            balance: balance::zero(),
        });
    }

    /// Buy a coffee from the shop. Emitted event is tracked by the real coffee
    /// shop and the customer gets a free coffee after 4 purchases.
    public fun buy_coffee(app: &mut CoffeeShop, payment: Coin<DWLT>, ctx: &mut TxContext) {
        // Check if the customer has enough SUI to pay for the coffee.
        assert!(coin::value(&payment) > COFFEE_PRICE, EIncorrectAmount);

        let token = token::mint(&mut app.coffee_points, 1, ctx);
        let request = token::transfer(token, sender(ctx), ctx);

        token::confirm_with_treasury_cap(&mut app.coffee_points, request, ctx);
        coin::put(&mut app.balance, payment);
        dwallet::event::emit(CoffeePurchased {})
    }

    /// Claim a free coffee from the shop. Emitted event is tracked by the real
    /// coffee shop and the customer gets a free coffee after 4 purchases. The
    /// `COFFEE` tokens are spent.
    public fun claim_free(app: &mut CoffeeShop, points: Token<COFFEE>, ctx: &mut TxContext) {
        // Check if the customer has enough `COFFEE` points to claim a free one.
        assert!(token::value(&points) == 4, EIncorrectAmount);

        // While we could use `burn`, spend illustrates another way of doing this
        let request = token::spend(points, ctx);
        token::confirm_with_treasury_cap(&mut app.coffee_points, request, ctx);
        dwallet::event::emit(CoffeePurchased {})
    }

    /// We allow transfer of `COFFEE` points to other customers but we charge 1
    /// `COFFEE` point for the transfer.
    public fun transfer(
        app: &mut CoffeeShop,
        points: Token<COFFEE>,
        recipient: address,
        ctx: &mut TxContext
    ) {
        assert!(token::value(&points) > 1, ENotEnoughPoints);
        let commission = token::split(&mut points, 1, ctx);
        let request = token::transfer(points, recipient, ctx);

        token::confirm_with_treasury_cap(&mut app.coffee_points, request, ctx);
        token::burn(&mut app.coffee_points, commission);
    }
}
