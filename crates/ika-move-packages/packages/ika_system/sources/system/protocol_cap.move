// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::protocol_cap;

public struct ProtocolCap has key, store {
    id: UID,
}

public(package) fun new_protocol_cap(
    ctx: &mut TxContext,
): ProtocolCap {
    let cap = ProtocolCap {
        id: object::new(ctx),
    };
    cap
}