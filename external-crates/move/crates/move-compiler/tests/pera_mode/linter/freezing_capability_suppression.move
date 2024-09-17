// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module a::test_suppression {
    use pera::object::UID;
    use pera::transfer;

    struct SuperAdminCap has key {
       id: UID
    }

    struct MasterCapability has key {
       id: UID
    }

    struct RootCapV3 has key {
       id: UID
    }

    #[allow(lint(freezing_capability))]
    public fun freeze_super_admin(w: SuperAdminCap) {
        transfer::public_freeze_object(w);
    }

    #[allow(lint(freezing_capability))]
    public fun freeze_master_cap(w: MasterCapability) {
        transfer::public_freeze_object(w);
    }

    #[allow(lint(freezing_capability))]
    public fun freeze_root_cap(w: RootCapV3) {
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
