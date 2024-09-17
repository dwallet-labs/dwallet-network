// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module c::c {
    use a::a;
    use b::b;

    public fun c() {
        a::a();
        b::b();
    }
}