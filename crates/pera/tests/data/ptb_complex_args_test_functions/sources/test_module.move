// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module test_functions::test_module {
    use pera::object::{Self, UID};
    use pera::tx_context::TxContext;
    use pera::transfer;
    use std::ascii::String as AS;
    use std::string::String as US;

    public struct Shared has key, store {
        id: UID,
    }

    public fun new_shared(ctx: &mut TxContext) {
        transfer::share_object(Shared { id: object::new(ctx) })
    }

    public fun use_immut(_: &Shared) {
        // do nothing
    }

    public fun use_mut(_: &mut Shared) {
        // do nothing
    }

    public fun use_ascii_string(_: AS) {
        // do nothing
    }

    public fun use_utf8_string(_: US) {
        // do nothing
    }

    public fun delete_shared_object(shared: Shared) {
        let Shared { id } = shared;
        object::delete(id);
    }
}