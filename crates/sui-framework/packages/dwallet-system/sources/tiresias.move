// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_const)]
module dwallet_system::tiresias {
    use dwallet::object::{UID, ID};

    #[allow(unused_field)]
    struct NetworkPublicKey has key, store {
        id: UID,
        value: vector<u8>,
    }

    #[allow(unused_field)]
    struct UserPublicKey has key, store {
        id: UID,
        value: vector<u8>,
    }

    #[allow(unused_field)]
    struct EncryptedUserShare has key, store {
        id: UID,
        dwallet_id: ID,
        user_public_key_id: ID,
        value: vector<u8>,
    }
}
