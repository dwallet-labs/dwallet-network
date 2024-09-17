// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module a::test_true_negatives {
    use pera::object::UID;
    use pera::transfer;

    struct NormalStruct has key {
       id: UID
    }

    struct Data has key {
       id: UID
    }

    struct Token has key {
       id: UID
    }

    struct Capture has key {
       id: UID
    }

    struct Handicap has key {
       id: UID
    }

    struct Recap has key {
       id: UID
    }

    struct MyCapybara has key {
       id: UID
    }

    public fun freeze_normal(w: NormalStruct) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_data(w: Data) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_token(w: Token) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_capture(w: Capture) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_handicap(w: Handicap) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_recap(w: Recap) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_capybara(w: MyCapybara) {
        transfer::public_freeze_object(w);
    }
}

module pera::object {
    struct UID has store {
        id: address,
    }
}

module pera::transfer {
    public fun public_freeze_object<T: key>(_: T) {
        abort 0
    }
}
