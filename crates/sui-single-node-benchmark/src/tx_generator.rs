// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::mock_account::Account;
pub use move_tx_generator::MoveTxGenerator;
pub use non_move_tx_generator::NonMoveTxGenerator;
pub use root_object_create_tx_generator::RootObjectCreateTxGenerator;
use sui_types::transaction::Transaction;

mod move_tx_generator;
mod non_move_tx_generator;
mod root_object_create_tx_generator;

pub(crate) trait TxGenerator: Send + Sync {
    /// Given an account that contains a sender address, a keypair for that address,
    /// and a list of gas objects owned by this address, generate a single transaction.
    fn generate_tx(&self, account: Account) -> Transaction;

    fn name(&self) -> &'static str;
}
