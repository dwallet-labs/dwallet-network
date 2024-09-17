// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses Test=0x0

//# publish
module Test::f {
    public enum F has drop {
        V,
    }

    public fun test() {
        assert!(!pera::types::is_one_time_witness(&F::V));
    }
}

//# run Test::f::test
