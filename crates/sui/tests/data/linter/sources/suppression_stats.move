// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// This file is used to test linter suppression stats output (the test itself is part of CLI tests
// in the sui crate)

#[lint_allow(custom_state_change)]
module linter::suppression_stats {
    use dwallet::object::UID;
    use dwallet::transfer;
    use dwallet::tx_context::{Self, TxContext};

    #[allow(unused_field)]
    struct S1 has key, store {
        id: UID
    }

    #[lint_allow(self_transfer)]
    public fun custom_transfer_bad(o: S1, ctx: &mut TxContext) {
        transfer::transfer(o, tx_context::sender(ctx))
    }

    #[lint_allow(share_owned)]
    public fun custom_share_bad(o: S1) {
        transfer::share_object(o)
    }

    public fun custom_freeze_bad(o: S1) {
        transfer::freeze_object(o)
    }
}
