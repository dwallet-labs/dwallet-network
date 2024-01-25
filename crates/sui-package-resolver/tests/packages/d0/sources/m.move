// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_field)]
module d::m {
    use dwallet::object::UID;

    struct O<T, phantom U> has key, store {
        id: UID,
        xs: vector<T>,
    }


    struct T<U, V> has copy, drop, store {
        u: U,
        v: V,
    }

    struct P has key { id: UID }
    struct Q { x: u32 }
    struct R has copy, drop { x: u16 }
    struct S has drop, store { x: u8 }
}
