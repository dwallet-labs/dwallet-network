// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_field)]
module a::n {
    struct T0 {
        t: a::m::T1<u16, u32>,
        u: a::m::T2,
    }
}
