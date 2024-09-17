// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_field)]
module e::m {
    use std::ascii::String as ASCII;
    use std::option::Option;
    use std::string::String as UTF8;
    use pera::object::UID;

    public struct O has key { id: UID }

    public native fun foo<T>(
        o: &O,
        x: u64,
        p: &mut O,
        y: T,
        q: O,
        z: Option<UTF8>,
        w: vector<Option<ASCII>>,
    );
}
