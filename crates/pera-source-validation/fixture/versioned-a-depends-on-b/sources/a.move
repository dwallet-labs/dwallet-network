// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module a::a {
    use b::b::b;
    use b::b::c;
    
    public fun a() : u64 {
        b() + c()
    }
}
