// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[test_only]
/// Tests if normally illegal (in terms of Sui bytecode verification) code is allowed in tests.
module dwallet::verifier_tests {
    struct VERIFIER_TESTS has drop {}

    fun init(otw: VERIFIER_TESTS, _: &mut dwallet::tx_context::TxContext) {
        assert!(dwallet::types::is_one_time_witness(&otw), 0);
    }

    #[test]
    fun test_init() {
        use dwallet::test_scenario;
        let admin = @0xBABE;

        let scenario_val = test_scenario::begin(admin);
        let scenario = &mut scenario_val;
        let otw = VERIFIER_TESTS{};
        init(otw, test_scenario::ctx(scenario));
        test_scenario::end(scenario_val);
    }

    fun is_otw(witness: VERIFIER_TESTS): bool {
        dwallet::types::is_one_time_witness(&witness)
    }

    #[test]
    fun test_otw() {
        // we should be able to construct otw in test code
        let otw = VERIFIER_TESTS{};
        assert!(is_otw(otw), 0);
    }

}
