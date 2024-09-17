// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module move_test_code::init_with_event {
    public struct Event has drop, copy {}

    fun init(_ctx: &mut TxContext) {
        pera::event::emit(Event {});
    }
}