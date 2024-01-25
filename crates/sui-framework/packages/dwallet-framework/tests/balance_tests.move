// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[test_only]
module dwallet::coin_balance_tests {
    use dwallet::test_scenario::{Self, ctx};
    use dwallet::pay;
    use dwallet::coin;
    use dwallet::balance;
    use dwallet::dwlt::DWLT;

    #[test]
    fun type_morphing() {
        let scenario = test_scenario::begin(@0x1);
        let test = &mut scenario;

        let balance = balance::zero<DWLT>();
        let coin = coin::from_balance(balance, ctx(test));
        let balance = coin::into_balance(coin);

        balance::destroy_zero(balance);

        let coin = coin::mint_for_testing<DWLT>(100, ctx(test));
        let balance_mut = coin::balance_mut(&mut coin);
        let sub_balance = balance::split(balance_mut, 50);

        assert!(balance::value(&sub_balance) == 50, 0);
        assert!(coin::value(&coin) == 50, 0);

        let balance = coin::into_balance(coin);
        balance::join(&mut balance, sub_balance);

        assert!(balance::value(&balance) == 100, 0);

        let coin = coin::from_balance(balance, ctx(test));
        pay::keep(coin, ctx(test));
        test_scenario::end(scenario);
    }
}
