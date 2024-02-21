// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module base_addr::base {
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::{Self, TxContext};
    use dwallet::transfer;
    use base_addr::friend_module::{Self, X};

    struct A has store, drop {
        v: u16,
    }

    struct B has key {
        id: UID,
        field1: u32,
        field2: A,
    }

    struct C has key {
        id: UID,
        field1: u64,
        field2: X,
    }

    entry fun make_objs(ctx: &mut TxContext) {
        let field2 = A { v: 128 };
        transfer::transfer(
            B { id: object::new(ctx), field1: 256, field2 },
            tx_context::sender(ctx),
        );
        let field2 = friend_module::make_x(true);
        transfer::transfer(
            C { id: object::new(ctx), field1: 0, field2 },
            tx_context::sender(ctx),
        );
    }

    entry fun destroy_objs(b: B, c: C) {
        let B { id, field1: _, field2: _ }  = b;
        object::delete(id);
        let C { id, field1: _, field2: _ }  = c;
        object::delete(id);
    }
}
