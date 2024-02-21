// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --addresses Test=0x0

//# publish

module Test::M {
    public entry fun create(_value: u64, _recipient: address) {}

}

// wrong number of args
//# run Test::M::create --args 10

// wrong arg types
//# run Test::M::create --args 10 10
