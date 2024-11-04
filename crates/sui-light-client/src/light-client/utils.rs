// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::bail;
use futures::{future, stream::StreamExt};
use sui_config::{
    sui_config_dir, Config, PersistedConfig, SUI_CLIENT_CONFIG, SUI_KEYSTORE_FILENAME,
};
use sui_json_rpc_types::Coin;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    sui_client_config::{SuiClientConfig, SuiEnv},
    wallet_context::WalletContext,
};
use tracing::info;

use reqwest::Client;
use serde_json::json;
use shared_crypto::intent::Intent;
use sui_sdk::types::{
    base_types::SuiAddress,
    crypto::SignatureScheme::ED25519,
    digests::TransactionDigest,
    programmable_transaction_builder::ProgrammableTransactionBuilder,
    quorum_driver_types::ExecuteTransactionRequestType,
    transaction::{Argument, Command, Transaction, TransactionData},
};

use sui_sdk::{rpc_types::SuiTransactionBlockResponseOptions, SuiClient, SuiClientBuilder};

#[derive(serde::Deserialize)]
struct FaucetResponse {
    task: String,
    error: Option<String>,
}

/// Return a dwallet client to interact with the APIs,
/// the active address of the local wallet, and another address that can be used as a recipient.
///
/// By default, this function will set up a wallet locally if there isn't any, or reuse the
/// existing one and its active address. This function should be used when two addresses are needed,
/// e.g., transferring objects from one address to another.
pub async fn setup_for_write(
    conf: &crate::Config,
) -> Result<(SuiClient, SuiAddress, SuiAddress), anyhow::Error> {
    let (client, active_address) = setup_for_read(conf).await?;
    // Make sure we have some DWLT on this address.
    let coin = fetch_coin(&client, &active_address).await?;
    if coin.is_none() {
        request_tokens_from_faucet(active_address, &client, conf).await?;
    }
    let wallet = retrieve_wallet(conf).await?;
    let addresses = wallet.get_addresses();
    let addresses = addresses
        .into_iter()
        .filter(|address| address != &active_address)
        .collect::<Vec<_>>();
    let recipient = addresses
        .first()
        .expect("Cannot get the recipient address needed for writing operations. Aborting");

    Ok((client, active_address, *recipient))
}

/// Return a dwallet client to interact with the APIs and an active address from the local wallet.
///
/// This function sets up a wallet in case there is no wallet locally,
/// and ensures that the active address of the wallet has SUI on it.
/// If there is no SUI owned by the active address, then it will request
/// SUI from the faucet.
pub async fn setup_for_read(
    conf: &crate::Config,
) -> Result<(SuiClient, SuiAddress), anyhow::Error> {
    let client = SuiClientBuilder::default()
        .build(&conf.dwallet_full_node_url)
        .await?;
    println!("Dwallet testnet version is: {}", client.api_version());
    let mut wallet = retrieve_wallet(conf).await?;
    println!("{}", wallet.get_addresses().len());
    assert!(wallet.get_addresses().len() >= 2);
    let active_address = wallet.active_address()?;

    println!("Wallet active address is: {active_address}");
    Ok((client, active_address))
}

/// Request tokens from the Faucet for the given address
pub async fn request_tokens_from_faucet(
    address: SuiAddress,
    _sui_client: &SuiClient,
    conf: &crate::Config,
) -> Result<(), anyhow::Error> {
    let address_str = address.to_string();
    let json_body = json![{
        "FixedAmountRequest": {
            "recipient": &address_str
        }
    }];

    // Make the request to the faucet JSON RPC API for DWLT.
    let rest_client = Client::new();
    let resp = rest_client
        .post(&conf.dwallet_faucet_url)
        .header("Content-Type", "application/json")
        .json(&json_body)
        .send()
        .await?;
    println!(
        "Faucet request for address {address_str} has status: {}",
        resp.status()
    );
    println!("Waiting for the faucet to complete the gas request...");
    let faucet_resp: FaucetResponse = resp.json().await?;
    println!("Faucet request task id: {}", faucet_resp.task);

    if let Some(err) = faucet_resp.error {
        bail!("Faucet request was unsuccessful. Error is {err:?}")
    } else {
        Ok(())
    }

    // println!("Faucet request task ID: {task_id}");
    //
    // let json_body = json![{
    //     "GetBatchSendStatusRequest": {
    //         "task_id": &task_id
    //     }
    // }];
    //
    // let mut coin_id = "".to_string();
    //
    // // Wait for the faucet to finish the batch of token requests.
    // loop {
    //     let resp = rest_client
    //         .get("https://faucet.testnet.sui.io/v1/status")
    //         .header("Content-Type", "application/json")
    //         .json(&json_body)
    //         .send()
    //         .await?;
    //     let text = resp.text().await?;
    //     if text.contains("SUCCEEDED") {
    //         let resp_json: serde_json::Value = serde_json::from_str(&text)?;
    //
    //         coin_id = <&str>::clone(
    //             &resp_json
    //                 .pointer("/status/transferred_gas_objects/sent/0/id")
    //                 .unwrap()
    //                 .as_str()
    //                 .unwrap(),
    //         )
    //         .to_string();
    //
    //         break;
    //     } else {
    //         tokio::time::sleep(Duration::from_secs(1)).await;
    //     }
    // }
    //
    // // wait until the fullnode has the coin object, and check if it has the same owner
    // loop {
    //     let owner = sui_client
    //         .read_api()
    //         .get_object_with_options(
    //             ObjectID::from_str(&coin_id)?,
    //             SuiObjectDataOptions::new().with_owner(),
    //         )
    //         .await?;
    //
    //     if owner.owner().is_some() {
    //         let owner_address = owner.owner().unwrap().get_owner_address()?;
    //         if owner_address == address {
    //             break;
    //         }
    //     } else {
    //         tokio::time::sleep(Duration::from_secs(1)).await;
    //     }
    // }
    // Ok(())
}

/// Return the coin owned by the address that has at least `5_000_000`
/// otherwise returns None.
pub async fn fetch_coin(
    sui: &SuiClient,
    sender: &SuiAddress,
) -> Result<Option<Coin>, anyhow::Error> {
    let coin_type = "0x2::dwlt::DWLT".to_string();
    let coins_stream = sui
        .coin_read_api()
        .get_coins_stream(*sender, Some(coin_type));

    let mut coins = coins_stream
        .skip_while(|c| future::ready(c.balance < 5_000_000))
        .boxed();
    let coin = coins.next().await;
    Ok(coin)
}

/// Return a transaction digest from a split coin + merge coins transaction
pub async fn split_coin_digest(
    sui: &SuiClient,
    sender: &SuiAddress,
    conf: &crate::Config,
) -> Result<TransactionDigest, anyhow::Error> {
    let coin = match fetch_coin(sui, sender).await? {
        None => {
            request_tokens_from_faucet(*sender, sui, conf).await?;
            fetch_coin(sui, sender)
                .await?
                .expect("Supposed to get a coin with SUI, but didn't. Aborting")
        }
        Some(c) => c,
    };

    println!(
        "Address: {sender}. The selected coin for split is {} and has a balance of {}\n",
        coin.coin_object_id, coin.balance
    );

    // set the maximum gas budget
    let max_gas_budget = 5_000_000;

    // get the reference gas price from the network
    let gas_price = sui.read_api().get_reference_gas_price().await?;

    // now we programmatically build the transaction through several commands
    let mut ptb = ProgrammableTransactionBuilder::new();
    // first, we want to split the coin, and we specify how much SUI (in MIST) we want
    // for the new coin
    let split_coin_amount = ptb.pure(1000u64)?; // note that we need to specify the u64 type here
    ptb.command(Command::SplitCoins(
        Argument::GasCoin,
        vec![split_coin_amount],
    ));
    // Now we want to merge the coins (so that we don't have many coins with very small values)
    // observe here that we pass Argument::Result(0), which instructs the PTB to get
    // the result from the previous command.
    ptb.command(Command::MergeCoins(
        Argument::GasCoin,
        vec![Argument::Result(0)],
    ));

    // We finished constructing our PTB, and we need to call finish.
    let builder = ptb.finish();

    // Using the PTB that we just constructed, create the transaction data
    // that we will submit to the network.
    let tx_data = TransactionData::new_programmable(
        *sender,
        vec![coin.object_ref()],
        builder,
        max_gas_budget,
        gas_price,
    );

    // sign & execute the transaction
    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let signature = keystore.sign_secure(sender, &tx_data, Intent::sui_transaction())?;

    let transaction_response = sui
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            SuiTransactionBlockResponseOptions::new(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    Ok(transaction_response.digest)
}

pub async fn retrieve_wallet(conf: &crate::Config) -> Result<WalletContext, anyhow::Error> {
    let wallet_conf = sui_config_dir()?.join(SUI_CLIENT_CONFIG);
    let keystore_path = sui_config_dir()?.join(SUI_KEYSTORE_FILENAME);

    // Check if a wallet exists and if not, create a wallet and a sui client config.
    if !keystore_path.exists() {
        let keystore = FileBasedKeystore::new(&keystore_path)?;
        keystore.save()?;
    }

    if !wallet_conf.exists() {
        let keystore = FileBasedKeystore::new(&keystore_path)?;
        let mut client_config = SuiClientConfig::new(keystore.into());

        client_config.add_env(SuiEnv {
            alias: "testnet".to_string(),
            rpc: conf.dwallet_full_node_url.clone(),
            ws: None,
            basic_auth: None,
        });

        if client_config.active_env.is_none() {
            client_config.active_env = client_config.envs.first().map(|env| env.alias.clone());
        }

        client_config.save(&wallet_conf)?;
        info!("Client config file is stored in {:?}.", &wallet_conf);
    }

    let mut keystore = FileBasedKeystore::new(&keystore_path)?;
    let mut client_config: SuiClientConfig = PersistedConfig::read(&wallet_conf)?;

    let default_active_address = if let Some(address) = keystore.addresses().first() {
        *address
    } else {
        keystore
            .generate_and_add_new_key(ED25519, None, None, None)?
            .0
    };

    if keystore.addresses().len() < 2 {
        keystore.generate_and_add_new_key(ED25519, None, None, None)?;
    }

    client_config.active_address = Some(default_active_address);
    client_config.keystore = keystore.into();
    client_config.save(&wallet_conf)?;

    let wallet = WalletContext::new(&wallet_conf, Some(std::time::Duration::from_secs(60)), None)?;

    Ok(wallet)
}
