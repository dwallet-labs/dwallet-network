// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module a::test1 {
    use pera::transfer;
    use pera::object::UID;

    struct Obj has key, store {
        id: UID
    }

    public entry fun arg_object(o: Obj) {
        let arg = o;
        transfer::public_share_object(arg);
    }
}

module a::test2 {
    use pera::transfer;
    use pera::object::{Self, UID};

    struct Obj has key, store {
        id: UID
    }

    struct Wrapper has key, store {
        id: UID,
        i: u32,
        o: Obj,
    }

    public entry fun unpack_obj(w: Wrapper) {
        let Wrapper { id, i: _, o } = w;
        transfer::public_share_object(o);
        object::delete(id);
    }

    #[allow(lint(share_owned))]
    public entry fun unpack_obj_suppressed(w: Wrapper) {
        let Wrapper { id, i: _, o } = w;
        transfer::public_share_object(o);
        object::delete(id);
    }
}

module pera::object {
    struct UID has store {
        id: address,
    }
    public fun delete(_: UID) {
        abort 0
    }
}

module pera::transfer {
    public fun public_share_object<T: key>(_: T) {
        abort 0
    }
}
