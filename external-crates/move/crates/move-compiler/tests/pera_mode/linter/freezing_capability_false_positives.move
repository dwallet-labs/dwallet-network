// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module a::test_false_positives {
    use pera::object::UID;
    use pera::transfer;

    struct NoCap has key {
       id: UID
    }

    struct CapAndHat has key {
       id: UID
    }

    struct Recap has key {
       id: UID
    }

    struct MyCapybara has key {
       id: UID
    }

    public fun freeze_capture(w: NoCap) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_handicap(w: CapAndHat) {
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
