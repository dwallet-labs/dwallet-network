// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module a::test_true_positives {
    use pera::object::UID;
    use pera::transfer;

    struct AdminCap has key {
       id: UID
    }

    struct UserCapability has key {
       id: UID
    }

    struct OwnerCapV2 has key {
       id: UID
    }

    public fun freeze_cap1(w: AdminCap) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_cap2(w: UserCapability) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_cap3(w: OwnerCapV2) {
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
