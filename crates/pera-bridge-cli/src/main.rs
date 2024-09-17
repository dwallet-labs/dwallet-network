// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use clap::*;
use ethers::providers::Middleware;
use ethers::types::Address as EthAddress;
use fastcrypto::encoding::{Encoding, Hex};
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentMessage;
use std::collections::HashMap;
use std::str::from_utf8;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use pera_bridge::client::bridge_authority_aggregator::BridgeAuthorityAggregator;
use pera_bridge::crypto::{BridgeAuthorityPublicKey, BridgeAuthorityPublicKeyBytes};
use pera_bridge::eth_transaction_builder::build_eth_transaction;
use pera_bridge::pera_client::PeraClient;
use pera_bridge::pera_transaction_builder::build_pera_transaction;
use pera_bridge::types::BridgeActionType;
use pera_bridge::utils::{
    examine_key, generate_bridge_authority_key_and_write_to_file,
    generate_bridge_client_key_and_write_to_file, generate_bridge_node_config_and_write_to_file,
};
use pera_bridge::utils::{get_eth_contracts, EthBridgeContracts};
use pera_bridge_cli::{
    make_action, select_contract_address, Args, BridgeCliConfig, BridgeCommand,
    LoadedBridgeCliConfig, Network, SEPOLIA_BRIDGE_PROXY_ADDR,
};
use pera_config::Config;
use pera_sdk::PeraClient as PeraSdkClient;
use pera_sdk::PeraClientBuilder;
use pera_types::base_types::PeraAddress;
use pera_types::bridge::BridgeChainId;
use pera_types::bridge::{MoveTypeCommitteeMember, MoveTypeCommitteeMemberRegistration};
use pera_types::committee::TOTAL_VOTING_POWER;
use pera_types::crypto::AuthorityPublicKeyBytes;
use pera_types::crypto::Signature;
use pera_types::crypto::ToFromBytes;
use pera_types::transaction::Transaction;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init logging
    let (_guard, _filter_handle) = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();
    let args = Args::parse();

    match args.command {
        BridgeCommand::CreateBridgeValidatorKey { path } => {
            generate_bridge_authority_key_and_write_to_file(&path)?;
            println!("Bridge validator key generated at {}", path.display());
        }
        BridgeCommand::CreateBridgeClientKey { path, use_ecdsa } => {
            generate_bridge_client_key_and_write_to_file(&path, use_ecdsa)?;
            println!("Bridge client key generated at {}", path.display());
        }
        BridgeCommand::ExamineKey {
            path,
            is_validator_key,
        } => {
            examine_key(&path, is_validator_key)?;
        }
        BridgeCommand::CreateBridgeNodeConfigTemplate { path, run_client } => {
            generate_bridge_node_config_and_write_to_file(&path, run_client)?;
            println!(
                "Bridge node config template generated at {}",
                path.display()
            );
        }

        BridgeCommand::Governance {
            config_path,
            chain_id,
            cmd,
            dry_run,
        } => {
            let chain_id = BridgeChainId::try_from(chain_id).expect("Invalid chain id");
            println!("Chain ID: {:?}", chain_id);
            let config = BridgeCliConfig::load(config_path).expect("Couldn't load BridgeCliConfig");
            let config = LoadedBridgeCliConfig::load(config).await?;
            let pera_bridge_client = PeraClient::<PeraSdkClient>::new(&config.pera_rpc_url).await?;

            let (pera_key, pera_address, gas_object_ref) = config
                .get_pera_account_info()
                .await
                .expect("Failed to get pera account info");
            let bridge_summary = pera_bridge_client
                .get_bridge_summary()
                .await
                .expect("Failed to get bridge summary");
            let bridge_committee = Arc::new(
                pera_bridge_client
                    .get_bridge_committee()
                    .await
                    .expect("Failed to get bridge committee"),
            );
            let agg = BridgeAuthorityAggregator::new(bridge_committee);

            // Handle Pera Side
            if chain_id.is_pera_chain() {
                let pera_chain_id = BridgeChainId::try_from(bridge_summary.chain_id).unwrap();
                assert_eq!(
                    pera_chain_id, chain_id,
                    "Chain ID mismatch, expected: {:?}, got from url: {:?}",
                    chain_id, pera_chain_id
                );
                // Create BridgeAction
                let pera_action = make_action(pera_chain_id, &cmd);
                println!("Action to execute on Pera: {:?}", pera_action);
                let certified_action = agg
                    .request_committee_signatures(pera_action)
                    .await
                    .expect("Failed to request committee signatures");
                if dry_run {
                    println!("Dryrun succeeded.");
                    return Ok(());
                }
                let bridge_arg = pera_bridge_client
                    .get_mutable_bridge_object_arg_must_succeed()
                    .await;
                let rgp = pera_bridge_client
                    .get_reference_gas_price_until_success()
                    .await;
                let id_token_map = pera_bridge_client.get_token_id_map().await.unwrap();
                let tx = build_pera_transaction(
                    pera_address,
                    &gas_object_ref,
                    certified_action,
                    bridge_arg,
                    &id_token_map,
                    rgp,
                )
                .expect("Failed to build pera transaction");
                let pera_sig = Signature::new_secure(
                    &IntentMessage::new(Intent::pera_transaction(), tx.clone()),
                    &pera_key,
                );
                let tx = Transaction::from_data(tx, vec![pera_sig]);
                let resp = pera_bridge_client
                    .execute_transaction_block_with_effects(tx)
                    .await
                    .expect("Failed to execute transaction block with effects");
                if resp.status_ok().unwrap() {
                    println!("Pera Transaction succeeded: {:?}", resp.digest);
                } else {
                    println!(
                        "Pera Transaction failed: {:?}. Effects: {:?}",
                        resp.digest, resp.effects
                    );
                }
                return Ok(());
            }

            // Handle eth side
            // TODO assert chain id returned from rpc matches chain_id
            let eth_signer_client = config.eth_signer();
            // Create BridgeAction
            let eth_action = make_action(chain_id, &cmd);
            println!("Action to execute on Eth: {:?}", eth_action);
            // Create Eth Signer Client
            // TODO if a validator is blocklisted on eth, ignore their signatures?
            let certified_action = agg
                .request_committee_signatures(eth_action)
                .await
                .expect("Failed to request committee signatures");
            if dry_run {
                println!("Dryrun succeeded.");
                return Ok(());
            }
            let contract_address = select_contract_address(&config, &cmd);
            let tx = build_eth_transaction(
                contract_address,
                eth_signer_client.clone(),
                certified_action,
            )
            .await
            .expect("Failed to build eth transaction");
            println!("sending Eth tx: {:?}", tx);
            match tx.send().await {
                Ok(tx_hash) => {
                    println!("Transaction sent with hash: {:?}", tx_hash);
                }
                Err(err) => {
                    let revert = err.as_revert();
                    println!("Transaction reverted: {:?}", revert);
                }
            };

            return Ok(());
        }

        BridgeCommand::ViewEthBridge {
            network,
            bridge_proxy,
            eth_rpc_url,
        } => {
            let bridge_proxy = match network {
                Some(Network::Testnet) => {
                    Ok(EthAddress::from_str(SEPOLIA_BRIDGE_PROXY_ADDR).unwrap())
                }
                None => bridge_proxy.ok_or(anyhow::anyhow!(
                    "Network or bridge proxy address must be provided"
                )),
            }?;
            let provider = Arc::new(
                ethers::prelude::Provider::<ethers::providers::Http>::try_from(eth_rpc_url)
                    .unwrap()
                    .interval(std::time::Duration::from_millis(2000)),
            );
            let chain_id = provider.get_chainid().await?;
            let EthBridgeContracts {
                bridge,
                committee,
                limiter,
                vault,
                config,
            } = get_eth_contracts(bridge_proxy, &provider).await?;
            let message_type = BridgeActionType::EvmContractUpgrade as u8;
            let bridge_upgrade_next_nonce: u64 = bridge.nonces(message_type).call().await?;
            let committee_upgrade_next_nonce: u64 = committee.nonces(message_type).call().await?;
            let limiter_upgrade_next_nonce: u64 = limiter.nonces(message_type).call().await?;
            let config_upgrade_next_nonce: u64 = config.nonces(message_type).call().await?;

            let token_transfer_next_nonce: u64 = bridge
                .nonces(BridgeActionType::TokenTransfer as u8)
                .call()
                .await?;
            let blocklist_update_nonce: u64 = committee
                .nonces(BridgeActionType::UpdateCommitteeBlocklist as u8)
                .call()
                .await?;
            let emergency_button_nonce: u64 = bridge
                .nonces(BridgeActionType::EmergencyButton as u8)
                .call()
                .await?;
            let limit_update_nonce: u64 = limiter
                .nonces(BridgeActionType::LimitUpdate as u8)
                .call()
                .await?;
            let asset_price_update_nonce: u64 = config
                .nonces(BridgeActionType::AssetPriceUpdate as u8)
                .call()
                .await?;
            let add_tokens_nonce: u64 = config
                .nonces(BridgeActionType::AddTokensOnEvm as u8)
                .call()
                .await?;

            let print = OutputEthBridge {
                chain_id: chain_id.as_u64(),
                bridge_proxy: bridge.address(),
                committee_proxy: committee.address(),
                limiter_proxy: limiter.address(),
                config_proxy: config.address(),
                vault: vault.address(),
                nonces: Nonces {
                    token_transfer: token_transfer_next_nonce,
                    blocklist_update: blocklist_update_nonce,
                    emergency_button: emergency_button_nonce,
                    limit_update: limit_update_nonce,
                    asset_price_update: asset_price_update_nonce,
                    add_evm_tokens: add_tokens_nonce,
                    contract_upgrade_bridge: bridge_upgrade_next_nonce,
                    contract_upgrade_committee: committee_upgrade_next_nonce,
                    contract_upgrade_limiter: limiter_upgrade_next_nonce,
                    contract_upgrade_config: config_upgrade_next_nonce,
                },
            };
            println!("{}", serde_json::to_string_pretty(&print).unwrap());
            return Ok(());
        }

        BridgeCommand::ViewBridgeRegistration { pera_rpc_url } => {
            let pera_bridge_client = PeraClient::<PeraSdkClient>::new(&pera_rpc_url).await?;
            let bridge_summary = pera_bridge_client
                .get_bridge_summary()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to get bridge summary: {:?}", e))?;
            let move_type_bridge_committee = bridge_summary.committee;
            let pera_client = PeraClientBuilder::default().build(pera_rpc_url).await?;
            let stakes = pera_client
                .governance_api()
                .get_committee_info(None)
                .await?
                .validators
                .into_iter()
                .collect::<HashMap<_, _>>();
            let names = pera_client
                .governance_api()
                .get_latest_pera_system_state()
                .await?
                .active_validators
                .into_iter()
                .map(|summary| {
                    let protocol_key =
                        AuthorityPublicKeyBytes::from_bytes(&summary.protocol_pubkey_bytes)
                            .unwrap();
                    (summary.pera_address, (protocol_key, summary.name))
                })
                .collect::<HashMap<_, _>>();
            let mut authorities = vec![];
            let mut output_wrapper = Output::<OutputPeraBridgeRegistration>::default();
            for (_, member) in move_type_bridge_committee.member_registration {
                let MoveTypeCommitteeMemberRegistration {
                    pera_address,
                    bridge_pubkey_bytes,
                    http_rest_url,
                } = member;
                let Ok(pubkey) = BridgeAuthorityPublicKey::from_bytes(&bridge_pubkey_bytes) else {
                    output_wrapper.add_error(format!(
                        "Invalid bridge pubkey for validator {}: {:?}",
                        pera_address, bridge_pubkey_bytes
                    ));
                    continue;
                };
                let eth_address = BridgeAuthorityPublicKeyBytes::from(&pubkey).to_eth_address();
                let Ok(url) = from_utf8(&http_rest_url) else {
                    output_wrapper.add_error(format!(
                        "Invalid bridge http url for validator: {}: {:?}",
                        pera_address, http_rest_url
                    ));
                    continue;
                };
                let url = url.to_string();

                let (protocol_key, name) = names.get(&pera_address).unwrap();
                let stake = stakes.get(protocol_key).unwrap();
                authorities.push((name, pera_address, pubkey, eth_address, url, stake));
            }
            let total_stake = authorities
                .iter()
                .map(|(_, _, _, _, _, stake)| **stake)
                .sum::<u64>();
            let mut output = OutputPeraBridgeRegistration {
                total_registered_stake: total_stake as f32 / TOTAL_VOTING_POWER as f32 * 100.0,
                ..Default::default()
            };
            for (name, pera_address, pubkey, eth_address, url, stake) in authorities {
                output.committee.push(OutputMember {
                    name: name.clone(),
                    pera_address,
                    eth_address,
                    pubkey: Hex::encode(pubkey.as_bytes()),
                    url,
                    stake: *stake,
                    blocklisted: None,
                    status: None,
                });
            }
            output_wrapper.inner = output;
            println!("{}", serde_json::to_string_pretty(&output_wrapper).unwrap());
        }

        BridgeCommand::ViewPeraBridge {
            pera_rpc_url,
            hex,
            ping,
        } => {
            let pera_bridge_client = PeraClient::<PeraSdkClient>::new(&pera_rpc_url).await?;
            let bridge_summary = pera_bridge_client
                .get_bridge_summary()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to get bridge summary: {:?}", e))?;
            let move_type_bridge_committee = bridge_summary.committee;
            let pera_client = PeraClientBuilder::default().build(pera_rpc_url).await?;
            let names = pera_client
                .governance_api()
                .get_latest_pera_system_state()
                .await?
                .active_validators
                .into_iter()
                .map(|summary| (summary.pera_address, summary.name))
                .collect::<HashMap<_, _>>();
            let mut authorities = vec![];
            let mut ping_tasks = vec![];
            let client = reqwest::Client::builder()
                .connect_timeout(Duration::from_secs(10))
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap();
            let mut output_wrapper = Output::<OutputPeraBridge>::default();
            for (_, member) in move_type_bridge_committee.members {
                let MoveTypeCommitteeMember {
                    pera_address,
                    bridge_pubkey_bytes,
                    voting_power,
                    http_rest_url,
                    blocklisted,
                } = member;
                let Ok(pubkey) = BridgeAuthorityPublicKey::from_bytes(&bridge_pubkey_bytes) else {
                    output_wrapper.add_error(format!(
                        "Invalid bridge pubkey for validator {}: {:?}",
                        pera_address, bridge_pubkey_bytes
                    ));
                    continue;
                };
                let eth_address = BridgeAuthorityPublicKeyBytes::from(&pubkey).to_eth_address();
                let Ok(url) = from_utf8(&http_rest_url) else {
                    output_wrapper.add_error(format!(
                        "Invalid bridge http url for validator: {}: {:?}",
                        pera_address, http_rest_url
                    ));
                    continue;
                };
                let url = url.to_string();

                let name = names.get(&pera_address).unwrap();
                if ping {
                    let client_clone = client.clone();
                    ping_tasks.push(client_clone.get(url.clone()).send());
                }
                authorities.push((
                    name,
                    pera_address,
                    pubkey,
                    eth_address,
                    url,
                    voting_power,
                    blocklisted,
                ));
            }
            let total_stake = authorities
                .iter()
                .map(|(_, _, _, _, _, stake, _)| *stake)
                .sum::<u64>();
            let mut output = OutputPeraBridge {
                total_stake: total_stake as f32 / TOTAL_VOTING_POWER as f32 * 100.0,
                ..Default::default()
            };
            let ping_tasks_resp = if !ping_tasks.is_empty() {
                futures::future::join_all(ping_tasks)
                    .await
                    .into_iter()
                    .map(|resp| {
                        Some(match resp {
                            Ok(resp) => resp.status().is_success(),
                            Err(_e) => false,
                        })
                    })
                    .collect::<Vec<_>>()
            } else {
                vec![None; authorities.len()]
            };
            let mut total_online_stake = 0;
            for ((name, pera_address, pubkey, eth_address, url, stake, blocklisted), ping_resp) in
                authorities.into_iter().zip(ping_tasks_resp)
            {
                let pubkey = if hex {
                    Hex::encode(pubkey.as_bytes())
                } else {
                    pubkey.to_string()
                };
                match ping_resp {
                    Some(resp) => {
                        if resp {
                            total_online_stake += stake;
                        }
                        output.committee.push(OutputMember {
                            name: name.clone(),
                            pera_address,
                            eth_address,
                            pubkey,
                            url,
                            stake,
                            blocklisted: Some(blocklisted),
                            status: Some(if resp {
                                "online".to_string()
                            } else {
                                "offline".to_string()
                            }),
                        });
                    }
                    None => {
                        output.committee.push(OutputMember {
                            name: name.clone(),
                            pera_address,
                            eth_address,
                            pubkey,
                            url,
                            stake,
                            blocklisted: Some(blocklisted),
                            status: None,
                        });
                    }
                }
            }
            if ping {
                output.total_online_stake =
                    Some(total_online_stake as f32 / TOTAL_VOTING_POWER as f32 * 100.0);
            }

            // sequence nonces
            for (type_, nonce) in bridge_summary.sequence_nums {
                output
                    .nonces
                    .insert(BridgeActionType::try_from(type_).unwrap(), nonce);
            }

            output_wrapper.inner = output;
            println!("{}", serde_json::to_string_pretty(&output_wrapper).unwrap());
        }
        BridgeCommand::Client { config_path, cmd } => {
            let config = BridgeCliConfig::load(config_path).expect("Couldn't load BridgeCliConfig");
            let config = LoadedBridgeCliConfig::load(config).await?;
            let pera_bridge_client = PeraClient::<PeraSdkClient>::new(&config.pera_rpc_url).await?;
            cmd.handle(&config, pera_bridge_client).await?;
            return Ok(());
        }
    }

    Ok(())
}

#[derive(serde::Serialize, Default)]
struct OutputEthBridge {
    chain_id: u64,
    bridge_proxy: EthAddress,
    committee_proxy: EthAddress,
    limiter_proxy: EthAddress,
    config_proxy: EthAddress,
    vault: EthAddress,
    nonces: Nonces,
}

#[derive(serde::Serialize, Default)]
struct Nonces {
    token_transfer: u64,
    blocklist_update: u64,
    emergency_button: u64,
    limit_update: u64,
    asset_price_update: u64,
    add_evm_tokens: u64,
    contract_upgrade_bridge: u64,
    contract_upgrade_committee: u64,
    contract_upgrade_limiter: u64,
    contract_upgrade_config: u64,
}

#[derive(serde::Serialize, Default)]
struct Output<P: Default> {
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<String>>,
    inner: P,
}

impl<P: Default> Output<P> {
    fn add_error(&mut self, error: String) {
        if self.errors.is_none() {
            self.errors = Some(vec![]);
        }
        self.errors.as_mut().unwrap().push(error);
    }
}

#[derive(serde::Serialize, Default)]
struct OutputPeraBridge {
    total_stake: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_online_stake: Option<f32>,
    committee: Vec<OutputMember>,
    nonces: HashMap<BridgeActionType, u64>,
}

#[derive(serde::Serialize)]
struct OutputMember {
    name: String,
    pera_address: PeraAddress,
    eth_address: EthAddress,
    pubkey: String,
    url: String,
    stake: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocklisted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
}

#[derive(serde::Serialize, Default)]
struct OutputPeraBridgeRegistration {
    total_registered_stake: f32,
    committee: Vec<OutputMember>,
}
