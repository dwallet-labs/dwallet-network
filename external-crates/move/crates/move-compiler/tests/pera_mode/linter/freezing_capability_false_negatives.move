// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module a::test_false_negatives {
    use pera::object::UID;
    use pera::transfer;

    struct AdminRights has key {
       id: UID
    }

    struct PrivilegeToken has key {
       id: UID
    }

    struct AccessControl has key {
       id: UID
    }

    struct Capv0 has key {
        id: UID
    }

    public fun freeze_admin_rights(w: AdminRights) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_privilege_token(w: PrivilegeToken) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_access_control(w: AccessControl) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_cap_v(w: Capv0) {
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
