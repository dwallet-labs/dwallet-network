// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module a::test1 {
    use dwallet::transfer;
    use dwallet::object::UID;

    #[allow(unused_field)]
    struct Obj has key, store {
        id: UID
    }

    public entry fun arg_object(o: Obj) {
        let arg = o;
        transfer::public_share_object(arg);
    }
}

module a::test2 {
    use dwallet::transfer;
    use dwallet::object::{Self, UID};

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

    #[lint_allow(share_owned)]
    public entry fun unpack_obj_suppressed(w: Wrapper) {
        let Wrapper { id, i: _, o } = w;
        transfer::public_share_object(o);
        object::delete(id);
    }

    // a linter suppression should not work for regular compiler warnings
    #[linter_allow(code_suppression_should_not_work)]
    fun private_fun_should_not_be_suppressed() {}

    // a linter suppression should not work for regular compiler warnings
    #[linter_allow(category_suppression_should_not_work)]
    fun another_private_fun_should_not_be_suppressed() {}
}

module dwallet::object {
    struct UID has store {
        id: address,
    }
    public fun delete(_: UID) {
        abort 0
    }
}

module dwallet::transfer {
    public fun public_share_object<T: key>(_: T) {
        abort 0
    }
}
