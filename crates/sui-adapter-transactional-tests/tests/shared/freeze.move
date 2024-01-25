// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// tests that shared objects cannot be freezed

//# init --addresses t1=0x0 t2=0x0 --shared-object-deletion true

//# publish


module t2::o2 {
    use dwallet::object::{Self, UID};
    use dwallet::transfer;
    use dwallet::tx_context::TxContext;

    struct Obj2 has key, store {
        id: UID,
    }

    public entry fun create(ctx: &mut TxContext) {
        let o = Obj2 { id: object::new(ctx) };
        transfer::public_share_object(o)
    }

    public entry fun freeze_o2(o2: Obj2) {
        transfer::freeze_object(o2);
    }

}

//# run t2::o2::create

//# view-object 2,0

//# run t2::o2::freeze_o2 --args object(2,0)
