// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module a::test {
    use pera::object::UID;
    use pera::transfer;
    use pera::tx_context::{Self, TxContext};

    #[allow(unused_field)]
    struct S1 has key, store {
        id: UID
    }

    #[allow(lint(self_transfer, prefer_mut_tx_context))]
    public fun custom_transfer_bad(o: S1, ctx: &TxContext) {
        transfer::transfer(o, tx_context::sender(ctx))
    }

    #[allow(lint(share_owned))]
    public fun custom_share_bad(o: S1) {
        transfer::share_object(o)
    }

    public fun custom_freeze_bad(o: S1) {
        transfer::freeze_object(o)
    }
}

module pera::object {
    struct UID has store {
        id: address,
    }
    public fun new(_: &mut pera::tx_context::TxContext): UID {
        abort 0
    }
}

module pera::tx_context {
    struct TxContext has drop {}
    public fun sender(_: &TxContext): address {
        @0
    }
}

module pera::transfer {
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