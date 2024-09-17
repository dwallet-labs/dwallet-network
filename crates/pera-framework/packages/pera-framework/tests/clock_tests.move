// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[test_only]
module pera::clock_tests {
    use pera::clock;

    #[test]
    fun creating_a_clock_and_incrementing_it() {
        let mut ctx = tx_context::dummy();
        let mut clock = clock::create_for_testing(&mut ctx);

        clock.increment_for_testing(42);
        assert!(clock.timestamp_ms() == 42);

        clock.set_for_testing(50);
        assert!(clock.timestamp_ms() == 50);

        clock.destroy_for_testing();
    }
}
