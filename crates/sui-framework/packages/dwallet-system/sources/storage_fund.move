// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dwallet_system::storage_fund {
    use dwallet::balance::{Self, Balance};
    use dwallet::dwlt::DWLT;

    friend dwallet_system::dwallet_system_state_inner;

    /// Struct representing the storage fund, containing two `Balance`s:
    /// - `total_object_storage_rebates` has the invariant that it's the sum of `storage_rebate` of
    ///    all objects currently stored on-chain. To maintain this invariant, the only inflow of this
    ///    balance is storage charges collected from transactions, and the only outflow is storage rebates
    ///    of transactions, including both the portion refunded to the transaction senders as well as
    ///    the non-refundable portion taken out and put into `non_refundable_balance`.
    /// - `non_refundable_balance` contains any remaining inflow of the storage fund that should not
    ///    be taken out of the fund.
    struct StorageFund has store {
        total_object_storage_rebates: Balance<DWLT>,
        non_refundable_balance: Balance<DWLT>,
    }

    /// Called by `dwallet_system` at genesis time.
    public(friend) fun new(initial_fund: Balance<DWLT>) : StorageFund {
        StorageFund {
            // At the beginning there's no object in the storage yet
            total_object_storage_rebates: balance::zero(),
            non_refundable_balance: initial_fund,
        }
    }

    /// Called by `dwallet_system` at epoch change times to process the inflows and outflows of storage fund.
    public(friend) fun advance_epoch(
        self: &mut StorageFund,
        storage_charges: Balance<DWLT>,
        storage_fund_reinvestment: Balance<DWLT>,
        leftover_staking_rewards: Balance<DWLT>,
        storage_rebate_amount: u64,
        non_refundable_storage_fee_amount: u64,
    ) : Balance<DWLT> {
        // Both the reinvestment and leftover rewards are not to be refunded so they go to the non-refundable balance.
        balance::join(&mut self.non_refundable_balance, storage_fund_reinvestment);
        balance::join(&mut self.non_refundable_balance, leftover_staking_rewards);

        // The storage charges for the epoch come from the storage rebate of the new objects created
        // and the new storage rebates of the objects modified during the epoch so we put the charges
        // into `total_object_storage_rebates`.
        balance::join(&mut self.total_object_storage_rebates, storage_charges);

        // Split out the non-refundable portion of the storage rebate and put it into the non-refundable balance.
        let non_refundable_storage_fee = balance::split(&mut self.total_object_storage_rebates, non_refundable_storage_fee_amount);
        balance::join(&mut self.non_refundable_balance, non_refundable_storage_fee);

        // `storage_rebates` include the already refunded rebates of deleted objects and old rebates of modified objects and
        // should be taken out of the `total_object_storage_rebates`.
        let storage_rebate = balance::split(&mut self.total_object_storage_rebates, storage_rebate_amount);

        // The storage rebate has already been returned to individual transaction senders' gas coins
        // so we return the balance to be burnt at the very end of epoch change.
        storage_rebate
    }

    public fun total_object_storage_rebates(self: &StorageFund): u64 {
        balance::value(&self.total_object_storage_rebates)
    }

    public fun total_balance(self: &StorageFund): u64 {
        balance::value(&self.total_object_storage_rebates) + balance::value(&self.non_refundable_balance)
    }
}
