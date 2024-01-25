// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module type_params::m2 {
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::TxContext;
    use dwallet::transfer;

    struct AnotherObject has key, store {
        id: UID,
        value: u64,
    }

    public entry fun create_and_transfer(value: u64, recipient: address, ctx: &mut TxContext) {
        transfer::public_transfer(
            AnotherObject { id: object::new(ctx), value },
            recipient
        )
    }

    public fun create(value: u64, ctx: &mut TxContext): AnotherObject {
        AnotherObject { id: object::new(ctx), value }
    }


}
