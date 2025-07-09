// Copyright (c) dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_common::address;

use sui::{address, hash};

// === Imports ===

// === Public Functions ===

public fun ed25519_address(public_key: vector<u8>): address {
    let address_bytes = hash::blake2b256(&public_key);
    address::from_bytes(address_bytes)
}
