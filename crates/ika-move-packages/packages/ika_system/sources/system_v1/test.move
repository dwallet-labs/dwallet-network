// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::test;

use sui::event;

public struct TestEvent has copy, drop {
    sender: address,
    tx_hash: vector<u8>,
    epoch: u64,
    epoch_timestamp_ms: u64,
}

public entry fun test_event(ctx: &mut TxContext) {
    event::emit(TestEvent {
        sender: ctx.sender(),
        tx_hash: *ctx.digest(),
        epoch: ctx.epoch(),
        epoch_timestamp_ms: ctx.epoch_timestamp_ms(),
    });
}