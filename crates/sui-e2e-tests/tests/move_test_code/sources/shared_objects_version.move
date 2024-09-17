// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module move_test_code::shared_objects_version {
    use pera::object::{Self, UID};
    use pera::transfer;
    use pera::tx_context::{Self, TxContext};

    public struct Counter has key {
        id: UID,
        value: u64,
    }

    public entry fun create_counter(ctx: &mut TxContext) {
        transfer::transfer(
            Counter {
                id: object::new(ctx),
                value: 0,
            },
            tx_context::sender(ctx),
        )
    }

    public entry fun create_shared_counter(ctx: &mut TxContext) {
        transfer::share_object(Counter {
            id: object::new(ctx),
            value: 0,
        })
    }

    public entry fun share_counter(counter: Counter) {
        transfer::share_object(counter)
    }

    public entry fun increment_counter(counter: &mut Counter) {
        counter.value = counter.value + 1
    }
}
