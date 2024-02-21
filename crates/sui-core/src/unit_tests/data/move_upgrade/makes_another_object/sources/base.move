// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module base_addr::base {
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::{Self, TxContext};
    use dwallet::transfer;
    use dwallet::event;

    struct A<T> {
        f1: bool,
        f2: T
    }

    struct B has key {
        id: UID,
        x: u64,
    }

    struct BModEvent has copy, drop {
        old: u64,
        new: u64,
    }

    struct C has key {
        id: UID,
        x: u64,
    }

    struct CModEvent has copy, drop {
        old: u64,
        new: u64,
    }

    friend base_addr::friend_module;

    public fun return_0(): u64 { abort 42 }

    public fun plus_1(x: u64): u64 { x + 1 }

    public(friend) fun friend_fun(x: u64): u64 { x }

    fun non_public_fun(y: bool): u64 { if (y) 0 else 1 }

    entry fun makes_b(ctx: &mut TxContext) {
        transfer::transfer(
            B { id: object::new(ctx), x: 42 },
            tx_context::sender(ctx),
        )
    }

    entry fun destroys_b(b: B) {
        let B { id, x: _ }  = b;
        object::delete(id);
    }

    entry fun modifies_b(b: B, ctx: &mut TxContext) {
        event::emit(BModEvent{ old: b.x, new: 7 });
        b.x = 7;
        transfer::transfer(b, tx_context::sender(ctx))
    }

    entry fun makes_c(ctx: &mut TxContext) {
        transfer::transfer(
            C { id: object::new(ctx), x: 42 },
            tx_context::sender(ctx),
        )
    }

    entry fun modifies_c(c: C, ctx: &mut TxContext) {
        event::emit(CModEvent{ old: c.x, new: 7 });
        c.x = 7;
        transfer::transfer(c, tx_context::sender(ctx))
    }


}
