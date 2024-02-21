// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// test unwrapping an object in a dynamic field

//# init --addresses a=0x0 --accounts A

//# publish
module a::m {

use dwallet::dynamic_field;
use dwallet::dynamic_object_field;
use dwallet::object;
use dwallet::tx_context::{sender, TxContext};

struct Obj has key, store {
    id: object::UID,
}

entry fun mint(ctx: &mut TxContext) {
    let parent = object::new(ctx);
    dynamic_field::add(&mut parent, 0, Obj { id: object::new(ctx) });
    dwallet::transfer::public_transfer(Obj { id: parent }, sender(ctx))
}

entry fun take_and_wrap(obj: &mut Obj) {
    let v = dynamic_field::remove<u64, Obj>(&mut obj.id, 0);
    dynamic_object_field::add(&mut obj.id, 0, v)
}

entry fun take_and_destroy(obj: &mut Obj) {
    let Obj { id } = dynamic_field::remove(&mut obj.id, 0);
    object::delete(id)
}

entry fun take_and_take(obj: &mut Obj, ctx: &mut TxContext) {
    let v = dynamic_field::remove<u64, Obj>(&mut obj.id, 0);
    dwallet::transfer::public_transfer(v, sender(ctx))
}

}

//# run a::m::mint --sender A

//# run a::m::take_and_wrap --sender A --args object(2,0)

//# view-object 3,0


//# run a::m::mint --sender A

//# run a::m::take_and_destroy --sender A --args object(5,0)


//# run a::m::mint --sender A


//# run a::m::take_and_take --sender A --args object(7,0)

//# view-object 7,0
