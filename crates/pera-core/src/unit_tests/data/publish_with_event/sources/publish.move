// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module examples::publish_with_event {
    use std::ascii::{Self, String};

    use pera::event;
    use pera::tx_context::TxContext;

    public struct PublishEvent has copy, drop {
        foo: String
    }

    fun init(_ctx: &mut TxContext) {
        event::emit(PublishEvent { foo: ascii::string(b"bar") })
    }
}
