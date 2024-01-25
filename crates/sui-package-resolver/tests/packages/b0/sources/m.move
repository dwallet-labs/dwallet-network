// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_field)]
module b::m {
    use a::m::T2 as M;
    use a::n::T0 as N;

    struct T0 {
        m: M,
        n: N,
    }
}
