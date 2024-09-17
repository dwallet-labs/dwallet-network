// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::workloads::Gas;
use crate::ValidatorProxy;
use anyhow::Result;
use pera_keys::keystore::{AccountKeystore, FileBasedKeystore};
use pera_test_transaction_builder::TestTransactionBuilder;
use pera_types::base_types::ObjectRef;
use pera_types::crypto::{AccountKeyPair, KeypairTraits};
use pera_types::object::Owner;
use pera_types::transaction::{Transaction, TransactionData, TEST_ONLY_GAS_UNIT_FOR_TRANSFER};
use pera_types::utils::to_sender_signed_transaction;
use pera_types::{base_types::PeraAddress, crypto::PeraKeyPair};
use std::path::PathBuf;
use std::sync::Arc;

// This is the maximum gas we will transfer from primary coin into any gas coin
// for running the benchmark

pub type UpdatedAndNewlyMintedGasCoins = Vec<Gas>;

pub fn get_ed25519_keypair_from_keystore(
    keystore_path: PathBuf,
    requested_address: &PeraAddress,
) -> Result<AccountKeyPair> {
    let keystore = FileBasedKeystore::new(&keystore_path)?;
    match keystore.get_key(requested_address) {
        Ok(PeraKeyPair::Ed25519(kp)) => Ok(kp.copy()),
        other => Err(anyhow::anyhow!("Invalid key type: {:?}", other)),
    }
}

pub fn make_pay_tx(
    input_coins: Vec<ObjectRef>,
    sender: PeraAddress,
    addresses: Vec<PeraAddress>,
    split_amounts: Vec<u64>,
    gas: ObjectRef,
    keypair: &AccountKeyPair,
    gas_price: u64,
) -> Result<Transaction> {
    let pay = TransactionData::new_pay(
        sender,
        input_coins,
        addresses,
        split_amounts,
        gas,
        TEST_ONLY_GAS_UNIT_FOR_TRANSFER * gas_price,
        gas_price,
    )?;
    Ok(to_sender_signed_transaction(pay, keypair))
}

pub async fn publish_basics_package(
    gas: ObjectRef,
    proxy: Arc<dyn ValidatorProxy + Sync + Send>,
    sender: PeraAddress,
    keypair: &AccountKeyPair,
    gas_price: u64,
) -> ObjectRef {
    let transaction = TestTransactionBuilder::new(sender, gas, gas_price)
        .publish_examples("basics")
        .build_and_sign(keypair);
    let effects = proxy.execute_transaction_block(transaction).await.unwrap();
    effects
        .created()
        .iter()
        .find(|(_, owner)| matches!(owner, Owner::Immutable))
        .map(|(reference, _)| *reference)
        .unwrap()
}
