// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses Test=0x0 --accounts A

//# publish
module Test::M {
    public struct Obj has key {
        id: pera::object::UID,
        value: u64
    }

    public entry fun mint(ctx: &mut TxContext) {
        pera::transfer::transfer(
            Obj { id: pera::object::new(ctx), value: 0 },
            tx_context::sender(ctx),
        )
    }

    public entry fun incr(obj: &mut Obj) {
        obj.value = obj.value + 1
    }
}

//# run Test::M::mint --sender A

//# run Test::M::incr --sender A --args object(2,0)
