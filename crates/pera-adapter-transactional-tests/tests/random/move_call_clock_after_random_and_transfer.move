// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --accounts A B --addresses test=0x0

//# publish --sender A
module test::random {
    use pera::clock::Clock;
    use pera::random::Random;

    public fun use_clock(_clock: &Clock) {}
    public fun use_random(_random: &Random) {}
}

// bad tx - use_random, transfer, use_clock
//# programmable --sender A --inputs 10 immshared(8) immshared(6) @B
//> SplitCoins(Gas, [Input(0)]);
//> test::random::use_random(Input(1));
//> TransferObjects([Result(0)], Input(3));
//> test::random::use_clock(Input(0))
