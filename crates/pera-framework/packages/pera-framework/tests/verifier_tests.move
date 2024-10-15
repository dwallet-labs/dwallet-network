// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[test_only]
/// Tests if normally illegal (in terms of Pera bytecode verification) code is allowed in tests.
module pera::verifier_tests {
    public struct VERIFIER_TESTS has drop {}

    fun init(otw: VERIFIER_TESTS, _: &mut pera::tx_context::TxContext) {
        assert!(pera::types::is_one_time_witness(&otw));
    }

    #[test]
    fun test_init() {
        use pera::test_scenario;
        let admin = @0xBABE;

        let mut scenario = test_scenario::begin(admin);
        let otw = VERIFIER_TESTS{};
        init(otw, scenario.ctx());
        scenario.end();
    }

    fun is_otw(witness: VERIFIER_TESTS): bool {
        pera::types::is_one_time_witness(&witness)
    }

    #[test]
    fun test_otw() {
        // we should be able to construct otw in test code
        let otw = VERIFIER_TESTS{};
        assert!(is_otw(otw));
    }

}