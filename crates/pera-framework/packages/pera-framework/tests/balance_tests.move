// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[test_only]
module pera::coin_balance_tests {
    use pera::test_scenario;
    use pera::pay;
    use pera::coin;
    use pera::balance;
    use pera::pera::PERA;

    #[test]
    fun type_morphing() {
        let mut scenario = test_scenario::begin(@0x1);

        let balance = balance::zero<PERA>();
        let coin = balance.into_coin(scenario.ctx());
        let balance = coin.into_balance();

        balance.destroy_zero();

        let mut coin = coin::mint_for_testing<PERA>(100, scenario.ctx());
        let balance_mut = coin::balance_mut(&mut coin);
        let sub_balance = balance_mut.split(50);

        assert!(sub_balance.value() == 50);
        assert!(coin.value() == 50);

        let mut balance = coin.into_balance();
        balance.join(sub_balance);

        assert!(balance.value() == 100);

        let coin = balance.into_coin(scenario.ctx());
        pay::keep(coin, scenario.ctx());
        scenario.end();
    }
}
