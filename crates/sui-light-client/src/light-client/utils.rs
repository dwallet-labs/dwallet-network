// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Result;
use anyhow::{bail, Context};
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
use sui_sdk::types::{base_types::SuiAddress, crypto::SignatureScheme::ED25519};

use crate::{dwallet_client, DWALLET_COIN_TYPE};
use sui_sdk::SuiClient;

#[derive(serde::Deserialize, Debug)]
struct FaucetResponse {
    error: Option<String>,
    #[serde(rename = "transferredGasObjects")]
    transferred_gas_objects: Vec<TransferredGasObject>,
}

#[derive(serde::Deserialize, Debug)]
struct TransferredGasObject {
    amount: u64,
    // _id: String,
    // #[serde(skip)]
    // transfer_tx_digest: String,
}

/// Return a dwallet client to interact with the APIs,
/// and the active address of the local wallet.
///
/// By default, this function will set up a wallet locally if there isn't any, or reuse the
/// existing one and its active address.
pub async fn setup_for_write(
    conf: &crate::Config,
    required_balance: u64,
) -> Result<(SuiClient, Coin, SuiAddress), anyhow::Error> {
    let (client, active_address) = setup_for_read(conf).await?;
    // Make sure we have a minimum necessary DWLT on the active address.
    let coins = fetch_or_request_coins(&client, active_address, required_balance, conf).await?;
    Ok((client, coins, active_address))
}

/// Return a dwallet client to interact with the APIs and
/// an active address from the local wallet.
///
/// This function sets up a wallet if there is no wallet locally.
pub async fn setup_for_read(
    config: &crate::Config,
) -> Result<(SuiClient, SuiAddress), anyhow::Error> {
    let client = dwallet_client(config).await?;
    println!("Dwallet testnet version is: {}", client.api_version());
    let mut wallet = retrieve_wallet(config).await?;
    let active_address = wallet.active_address()?;
    println!("Wallet active address is: {active_address}");
    Ok((client, active_address))
}

/// Request tokens from the Faucet for the given address
pub async fn request_tokens_from_faucet(
    address: &SuiAddress,
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
    if let Some(err) = faucet_resp.error {
        bail!("Faucet request was unsuccessful. Error is {err:?}")
    }
    println!(
        "Received: {} coins from the faucet for addr: {}",
        faucet_resp.transferred_gas_objects[0].amount, address_str
    );
    Ok(())
}

/// Return the coin owned by the address that has at least `required_balance`
/// otherwise request coins from the faucet and return the coin.
pub async fn fetch_or_request_coins(
    dwallet_client: &SuiClient,
    sender: SuiAddress,
    required_balance: u64,
    config: &crate::Config,
) -> Result<Coin> {
    let coin = coins_by_required_balance(dwallet_client, sender, required_balance).await;
    if coin.is_none() {
        println!("Insufficient balance, requesting tokens from faucet");
        request_tokens_from_faucet(&sender, dwallet_client, config).await?;
    }
    let coin = coins_by_required_balance(dwallet_client, sender, required_balance).await;
    coin.context(format!(
        "Insufficient coin amount for operation, minium required balance: {}",
        required_balance
    ))
}

async fn coins_by_required_balance(
    dwallet_client: &SuiClient,
    sender: SuiAddress,
    required_balance: u64,
) -> Option<Coin> {
    let coins_stream = dwallet_client
        .coin_read_api()
        .get_coins_stream(sender, Some(DWALLET_COIN_TYPE.to_string()));
    let mut coins = coins_stream
        .skip_while(|c| future::ready(c.balance < required_balance))
        .boxed();
    coins.next().await
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

    client_config.active_address = Some(default_active_address);
    client_config.keystore = keystore.into();
    client_config.save(&wallet_conf)?;

    WalletContext::new(&wallet_conf, Some(std::time::Duration::from_secs(60)), None)
}
