// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::anyhow;
use clap::Parser;
use fastcrypto::encoding::{Encoding, Hex};
use fastcrypto::traits::EncodeDecodeBase64;
use serde_json::{json, Value};
use pera_config::{pera_config_dir, Config, NodeConfig, PERA_FULLNODE_CONFIG, PERA_KEYSTORE_FILENAME};
use pera_node::PeraNode;
use pera_rosetta::types::{CurveType, PrefundedAccount, PeraEnv};
use pera_rosetta::{RosettaOfflineServer, RosettaOnlineServer, PERA};
use pera_sdk::{PeraClient, PeraClientBuilder};
use pera_types::base_types::PeraAddress;
use pera_types::crypto::{KeypairTraits, PeraKeyPair, ToFromBytes};
use tracing::info;
use tracing::log::warn;

#[derive(Parser)]
#[clap(name = "pera-rosetta", rename_all = "kebab-case", author, version)]
pub enum RosettaServerCommand {
    GenerateRosettaCLIConfig {
        #[clap(long)]
        keystore_path: Option<PathBuf>,
        #[clap(long, default_value = "localnet")]
        env: PeraEnv,
        #[clap(long, default_value = "http://rosetta-online:9002")]
        online_url: String,
        #[clap(long, default_value = "http://rosetta-offline:9003")]
        offline_url: String,
    },
    StartOnlineRemoteServer {
        #[clap(long, default_value = "localnet")]
        env: PeraEnv,
        #[clap(long, default_value = "0.0.0.0:9002")]
        addr: SocketAddr,
        #[clap(long)]
        full_node_url: String,
        #[clap(long, default_value = "/data")]
        data_path: PathBuf,
    },
    StartOnlineServer {
        #[clap(long, default_value = "localnet")]
        env: PeraEnv,
        #[clap(long, default_value = "0.0.0.0:9002")]
        addr: SocketAddr,
        #[clap(long)]
        node_config: Option<PathBuf>,
        #[clap(long, default_value = "/data")]
        data_path: PathBuf,
    },
    StartOfflineServer {
        #[clap(long, default_value = "localnet")]
        env: PeraEnv,
        #[clap(long, default_value = "0.0.0.0:9003")]
        addr: SocketAddr,
    },
}

impl RosettaServerCommand {
    async fn execute(self) -> Result<(), anyhow::Error> {
        match self {
            RosettaServerCommand::GenerateRosettaCLIConfig {
                keystore_path,
                env,
                online_url,
                offline_url,
            } => {
                let path = keystore_path
                    .unwrap_or_else(|| pera_config_dir().unwrap().join(PERA_KEYSTORE_FILENAME));

                let prefunded_accounts = read_prefunded_account(&path)?;

                info!(
                    "Retrieved {} Pera address from keystore file {:?}",
                    prefunded_accounts.len(),
                    &path
                );

                let mut config: Value =
                    serde_json::from_str(include_str!("../resources/rosetta_cli.json"))?;

                config
                    .as_object_mut()
                    .unwrap()
                    .insert("online_url".into(), json!(online_url));

                // Set network.
                let network = config.pointer_mut("/network").ok_or_else(|| {
                    anyhow!("Cannot find construction config in default config file.")
                })?;
                network
                    .as_object_mut()
                    .unwrap()
                    .insert("network".into(), json!(env));

                // Add prefunded accounts.
                let construction = config.pointer_mut("/construction").ok_or_else(|| {
                    anyhow!("Cannot find construction config in default config file.")
                })?;

                let construction = construction.as_object_mut().unwrap();
                construction.insert("prefunded_accounts".into(), json!(prefunded_accounts));
                construction.insert("offline_url".into(), json!(offline_url));

                let config_path = PathBuf::from(".").join("rosetta_cli.json");
                fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;
                info!(
                    "Rosetta CLI configuration file is stored in {:?}",
                    config_path
                );

                let dsl_path = PathBuf::from(".").join("pera.ros");
                let dsl = include_str!("../resources/pera.ros");
                fs::write(
                    &dsl_path,
                    dsl.replace("{{pera.env}}", json!(env).as_str().unwrap()),
                )?;
                info!("Rosetta DSL file is stored in {:?}", dsl_path);
            }
            RosettaServerCommand::StartOfflineServer { env, addr } => {
                info!("Starting Rosetta Offline Server.");
                let server = RosettaOfflineServer::new(env);
                server.serve(addr).await;
            }
            RosettaServerCommand::StartOnlineRemoteServer {
                env,
                addr,
                full_node_url,
                data_path,
            } => {
                info!(
                    "Starting Rosetta Online Server with remove Pera full node [{full_node_url}]."
                );
                let pera_client = wait_for_pera_client(full_node_url).await;
                let rosetta_path = data_path.join("rosetta_db");
                info!("Rosetta db path : {rosetta_path:?}");
                let rosetta = RosettaOnlineServer::new(env, pera_client);
                rosetta.serve(addr).await;
            }

            RosettaServerCommand::StartOnlineServer {
                env,
                addr,
                node_config,
                data_path,
            } => {
                info!("Starting Rosetta Online Server with embedded Pera full node.");
                info!("Data directory path: {data_path:?}");

                let node_config = node_config.unwrap_or_else(|| {
                    let path = pera_config_dir().unwrap().join(PERA_FULLNODE_CONFIG);
                    info!("Using default node config from {path:?}");
                    path
                });

                let mut config = NodeConfig::load(&node_config)?;
                config.db_path = data_path.join("pera_db");
                info!("Overriding Pera db path to : {:?}", config.db_path);

                let registry_service =
                    mysten_metrics::start_prometheus_server(config.metrics_address);
                // Staring a full node for the rosetta server.
                let rpc_address = format!("http://127.0.0.1:{}", config.json_rpc_address.port());
                let _node = PeraNode::start(config, registry_service, None).await?;

                let pera_client = wait_for_pera_client(rpc_address).await;

                let rosetta_path = data_path.join("rosetta_db");
                info!("Rosetta db path : {rosetta_path:?}");
                let rosetta = RosettaOnlineServer::new(env, pera_client);
                rosetta.serve(addr).await;
            }
        };
        Ok(())
    }
}

async fn wait_for_pera_client(rpc_address: String) -> PeraClient {
    loop {
        match PeraClientBuilder::default()
            .max_concurrent_requests(usize::MAX)
            .build(&rpc_address)
            .await
        {
            Ok(client) => return client,
            Err(e) => {
                warn!("Error connecting to Pera RPC server [{rpc_address}]: {e}, retrying in 5 seconds.");
                tokio::time::sleep(Duration::from_millis(5000)).await;
            }
        }
    }
}

/// This method reads the keypairs from the Pera keystore to create the PrefundedAccount objects,
/// PrefundedAccount will be written to the rosetta-cli config file for testing.
///
fn read_prefunded_account(path: &Path) -> Result<Vec<PrefundedAccount>, anyhow::Error> {
    let reader = BufReader::new(File::open(path).unwrap());
    let kp_strings: Vec<String> = serde_json::from_reader(reader).unwrap();
    let keys = kp_strings
        .iter()
        .map(|kpstr| {
            let key = PeraKeyPair::decode_base64(kpstr);
            key.map(|k| (PeraAddress::from(&k.public()), k))
        })
        .collect::<Result<BTreeMap<_, _>, _>>()
        .unwrap();

    Ok(keys
        .into_iter()
        .map(|(address, key)| {
            let (privkey, curve_type) = match key {
                PeraKeyPair::Ed25519(k) => {
                    (Hex::encode(k.private().as_bytes()), CurveType::Edwards25519)
                }
                PeraKeyPair::Secp256k1(k) => {
                    (Hex::encode(k.private().as_bytes()), CurveType::Secp256k1)
                }
                PeraKeyPair::Secp256r1(k) => {
                    (Hex::encode(k.private().as_bytes()), CurveType::Secp256r1)
                }
            };
            PrefundedAccount {
                privkey,
                account_identifier: address.into(),
                curve_type,
                currency: PERA.clone(),
            }
        })
        .collect())
}

#[test]
fn test_read_keystore() {
    use pera_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
    use pera_types::crypto::SignatureScheme;

    let temp_dir = tempfile::tempdir().unwrap();
    let path = temp_dir.path().join("pera.keystore");
    let mut ks = Keystore::from(FileBasedKeystore::new(&path).unwrap());
    let key1 = ks
        .generate_and_add_new_key(SignatureScheme::ED25519, None, None, None)
        .unwrap();
    let key2 = ks
        .generate_and_add_new_key(SignatureScheme::Secp256k1, None, None, None)
        .unwrap();

    let accounts = read_prefunded_account(&path).unwrap();
    let acc_map = accounts
        .into_iter()
        .map(|acc| (acc.account_identifier.address, acc))
        .collect::<BTreeMap<_, _>>();

    assert_eq!(2, acc_map.len());
    assert!(acc_map.contains_key(&key1.0));
    assert!(acc_map.contains_key(&key2.0));

    let acc1 = acc_map[&key1.0].clone();
    let acc2 = acc_map[&key2.0].clone();

    let schema1: SignatureScheme = acc1.curve_type.into();
    let schema2: SignatureScheme = acc2.curve_type.into();
    assert!(matches!(schema1, SignatureScheme::ED25519));
    assert!(matches!(schema2, SignatureScheme::Secp256k1));
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cmd: RosettaServerCommand = RosettaServerCommand::parse();

    let (_guard, _) = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    cmd.execute().await
}
