// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[test_only]
module dwallet::clock_tests {
    use dwallet::clock;
    use dwallet::tx_context;

    #[test]
    fun creating_a_clock_and_incrementing_it() {
        let ctx = tx_context::dummy();
        let clock = clock::create_for_testing(&mut ctx);

        clock::increment_for_testing(&mut clock, 42);
        assert!(clock::timestamp_ms(&clock) == 42, 1);

        clock::set_for_testing(&mut clock, 50);
        assert!(clock::timestamp_ms(&clock) == 50, 1);

        clock::destroy_for_testing(clock);
    }
}
