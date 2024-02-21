// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses Test_V0=0x0 Test_V1=0x0 --accounts A

//# publish --upgradeable --sender A
module Test_V0::base {
    use dwallet::object::UID;
    struct Foo {
        id: UID,
    }
}

//# upgrade --package Test_V0 --upgrade-capability 1,1 --sender A
module Test_V1::base {
    use dwallet::object::UID;
    struct Foo {
        id: UID,
    }
    struct Bar has key {
        id: UID,
    }
}

