// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[test_only]
/// Template bytecode to use when working with (de)serialized bytecode.
module kiosk::template {
    use std::option::{some, none};
    use dwallet::tx_context::TxContext;
    use kiosk::collectible;

    struct TEMPLATE has drop {}
    struct Template has store {}

    const TOTAL_SUPPLY: u32 = 10;

    fun init(otw: TEMPLATE, ctx: &mut TxContext) {
        let supply = if (TOTAL_SUPPLY == 0) {
            none()
        } else {
            some(TOTAL_SUPPLY)
        };

        collectible::claim_ticket<
            TEMPLATE,
            Template
        >(otw, supply, ctx)
    }
}
