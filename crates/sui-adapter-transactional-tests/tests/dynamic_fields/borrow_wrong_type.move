// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// similar to dynamic_field_tests but over multiple transactions, as this uses a different code path
// test borrow with the wrong value type

//# init --addresses a=0x0 --accounts A

//# publish
module a::m {

use dwallet::dynamic_field::{add, borrow, borrow_mut};
use dwallet::object;
use dwallet::tx_context::{sender, TxContext};

struct Obj has key {
    id: object::UID,
}

entry fun t1(ctx: &mut TxContext) {
    let id = object::new(ctx);
    add<u64, u64>(&mut id, 0, 0);
    dwallet::transfer::transfer(Obj { id }, sender(ctx))
}

entry fun t2(obj: &mut Obj) {
    borrow<u64, u8>(&mut obj.id, 0);
}

entry fun t3(obj: &mut Obj) {
    borrow_mut<u64, u8>(&mut obj.id, 0);
}

}

//# run a::m::t1 --sender A

//# run a::m::t2 --sender A --args object(2,0)

//# run a::m::t3 --sender A --args object(2,0)
