// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[test_only]
/// Dummy policy which showcases all of the methods.
module dwallet::dummy_policy {
    use dwallet::coin::Coin;
    use dwallet::dwlt::DWLT;
    use dwallet::transfer_policy::{
        Self as policy,
        TransferPolicy,
        TransferPolicyCap,
        TransferRequest
    };

    struct Rule has drop {}
    struct Config has store, drop {}

    public fun set<T>(
        policy: &mut TransferPolicy<T>,
        cap: &TransferPolicyCap<T>
    ) {
        policy::add_rule(Rule {}, policy, cap, Config {})
    }

    public fun pay<T>(
        policy: &mut TransferPolicy<T>,
        request: &mut TransferRequest<T>,
        payment: Coin<DWLT>
    ) {
        policy::add_to_balance(Rule {}, policy, payment);
        policy::add_receipt(Rule {}, request);
    }
}
