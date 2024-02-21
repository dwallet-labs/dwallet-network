// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module a::test {
    use dwallet::object::UID;
    use dwallet::transfer;
    use dwallet::tx_context::{Self, TxContext};

    #[allow(unused_field)]
    struct S1 has key, store {
        id: UID
    }

    #[lint_allow(self_transfer)]
    public fun custom_transfer_bad(o: S1, ctx: &TxContext) {
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

module dwallet::object {
    struct UID has store {
        id: address,
    }
    public fun new(_: &mut dwallet::tx_context::TxContext): UID {
        abort 0
    }
}

module dwallet::tx_context {
    struct TxContext has drop {}
    public fun sender(_: &TxContext): address {
        @0
    }
}

module dwallet::transfer {
    public fun transfer<T: key>(_: T, _: address) {
        abort 0
    }

    public fun freeze_object<T: key>(_: T) {
        abort 0
    }

    public fun share_object<T: key>(_: T) {
        abort 0
    }
}
