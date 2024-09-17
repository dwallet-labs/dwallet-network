// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use proptest::arbitrary::*;
use proptest::test_runner::TestCaseError;
use pera_types::base_types::dbg_addr;
use pera_types::crypto::KeypairTraits;
use pera_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use pera_types::transaction::TransactionData;
use pera_types::transaction::TransactionKind;
use pera_types::utils::to_sender_signed_transaction;
use tracing::debug;
use transaction_fuzzer::executor::Executor;
use transaction_fuzzer::run_proptest;
use transaction_fuzzer::GasDataGenConfig;
use transaction_fuzzer::GasDataWithObjects;

/// Send transfer pera txn with provided random gas data and gas objects to an authority.
fn test_with_random_gas_data(
    gas_data_test: GasDataWithObjects,
    executor: &mut Executor,
) -> Result<(), TestCaseError> {
    let gas_data = gas_data_test.gas_data;
    let objects = gas_data_test.objects;
    let sender = gas_data_test.sender_key.public().into();

    // Insert the random gas objects into genesis.
    executor.add_objects(&objects);
    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        let recipient = dbg_addr(2);
        builder.transfer_pera(recipient, None);
        builder.finish()
    };
    let kind = TransactionKind::ProgrammableTransaction(pt);
    let tx_data = TransactionData::new_with_gas_data(kind, sender, gas_data);
    let tx = to_sender_signed_transaction(tx_data, &gas_data_test.sender_key);

    let result = executor.execute_transaction(tx);
    debug!("result: {:?}", result);
    Ok(())
}

#[test]
#[cfg_attr(msim, ignore)]
fn test_gas_data_owned_or_immut() {
    let strategy = any_with::<GasDataWithObjects>(GasDataGenConfig::owned_by_sender_or_immut());
    run_proptest(1000, strategy, |gas_data_test, mut executor| {
        test_with_random_gas_data(gas_data_test, &mut executor)
    });
}

#[test]
#[cfg_attr(msim, ignore)]
fn test_gas_data_any_owner() {
    let strategy = any_with::<GasDataWithObjects>(GasDataGenConfig::any_owner());
    run_proptest(1000, strategy, |gas_data_test, mut executor| {
        test_with_random_gas_data(gas_data_test, &mut executor)
    });
}
