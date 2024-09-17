// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module e::e {
    use b::b::b;
    use b::b::c;
    
    public fun e() : u64 {
        b() + c()
    }
}
