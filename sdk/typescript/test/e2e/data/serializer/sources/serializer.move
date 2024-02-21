// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module serializer::serializer_tests {
    use dwallet::tx_context::{Self, TxContext};
    use dwallet::transfer;
    use dwallet::object::{Self, UID};
    use dwallet::clock::Clock;

    struct MutableShared has key {
        id: UID,
        value: u64,
    }

    fun init(ctx: &mut TxContext) {
        transfer::share_object(MutableShared {
            id: object::new(ctx),
            value: 1,
        })
    }

    public entry fun use_clock(_clock: &Clock) {}

    public entry fun list<T: key + store, C>(
        item: T,
        ctx: &mut TxContext
    ) {
        transfer::public_transfer(item, tx_context::sender(ctx))
    }

    public fun return_struct<T: key + store>(
        item: T,
    ): T {
        item
    }

    public entry fun value(clock: &MutableShared) {
        assert!(clock.value > 0, 1);
    }

    public entry fun set_value(clock: &mut MutableShared) {
        clock.value = 10;
    }

    public entry fun delete_value(clock: MutableShared) {
        let MutableShared { id, value: _ } = clock;
        object::delete(id);
    }

    public fun test_abort() {
        abort 0
    }
}
