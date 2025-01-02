// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, bail, ensure, Context};
use clap::*;
use colored::Colorize;
use fastcrypto::traits::KeyPair;
use move_analyzer::analyzer;
use rand::rngs::OsRng;
use std::io::{stderr, stdout, Write};
use std::net::{AddrParseError, IpAddr, Ipv4Addr, SocketAddr};
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fs, io};
use shared_crypto::intent::Intent;
use sui::client_commands::{estimate_gas_budget_from_gas_cost, execute_dry_run, max_gas_budget, SuiClientCommandResult};
use sui_config::node::Genesis;
use sui_config::p2p::SeedPeer;
use sui_config::{
    genesis_blob_exists, sui_config_dir, Config, PersistedConfig, FULL_NODE_DB_PATH,
    SUI_CLIENT_CONFIG, SUI_FULLNODE_CONFIG, SUI_NETWORK_CONFIG,
};
use sui_config::{
    SUI_BENCHMARK_GENESIS_GAS_KEYSTORE_FILENAME, SUI_GENESIS_FILENAME, SUI_KEYSTORE_FILENAME,
};
use sui_faucet::{create_wallet_context, start_faucet, AppState, FaucetConfig, SimpleFaucet};
use sui_indexer::test_utils::{
    start_indexer_jsonrpc_for_testing, start_indexer_writer_for_testing,
};

use sui_graphql_rpc::{
    config::{ConnectionConfig, ServiceConfig},
    test_infra::cluster::start_graphql_server_with_fn_rpc,
};

use sui_keys::keypair_file::read_key;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_sdk::rpc_types::{SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions};
use sui_sdk::sui_client_config::{SuiClientConfig, SuiEnv};
use sui_sdk::wallet_context::WalletContext;
use sui_swarm::memory::Swarm;
use sui_swarm_config::genesis_config::{GenesisConfig, DEFAULT_NUMBER_OF_AUTHORITIES};
use sui_swarm_config::network_config::NetworkConfig;
use sui_swarm_config::network_config_builder::ConfigBuilder;
use sui_swarm_config::node_config_builder::FullnodeConfigBuilder;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::crypto::{SignatureScheme, SuiKeyPair, ToFromBytes};
use sui_types::digests::ChainIdentifier;
use sui_types::message_envelope::Message;
use sui_types::transaction::{ProgrammableTransaction, SenderSignedData, Transaction, TransactionDataAPI, TransactionKind};
use sui_sdk::rpc_types::SuiTransactionBlockEffectsAPI;
use tempfile::tempdir;
use tracing;
use tracing::info;

const CONCURRENCY_LIMIT: usize = 30;
const DEFAULT_EPOCH_DURATION_MS: u64 = 60_000;
const DEFAULT_FAUCET_NUM_COINS: usize = 5; // 5 coins per request was the default in sui-test-validator
const DEFAULT_FAUCET_MIST_AMOUNT: u64 = 200_000_000_000; // 200 SUI
const DEFAULT_FAUCET_PORT: u16 = 9123;

const DEFAULT_GRAPHQL_PORT: u16 = 9125;

const DEFAULT_INDEXER_PORT: u16 = 9124;

#[derive(Args)]
pub struct SuiIndexerArgs {
    /// Start an indexer with default host and port: 0.0.0.0:9124. This flag accepts also a port,
    /// a host, or both (e.g., 0.0.0.0:9124).
    /// When providing a specific value, please use the = sign between the flag and value:
    /// `--with-indexer=6124` or `--with-indexer=0.0.0.0`, or `--with-indexer=0.0.0.0:9124`
    /// The indexer will be started in writer mode and reader mode.
    #[clap(long,
            default_missing_value = "0.0.0.0:9124",
            num_args = 0..=1,
            require_equals = true,
            value_name = "INDEXER_HOST_PORT",
    )]
    pub sui_with_indexer: Option<String>,

    /// Start a GraphQL server with default host and port: 0.0.0.0:9125. This flag accepts also a
    /// port, a host, or both (e.g., 0.0.0.0:9125).
    /// When providing a specific value, please use the = sign between the flag and value:
    /// `--with-graphql=6124` or `--with-graphql=0.0.0.0`, or `--with-graphql=0.0.0.0:9125`
    /// Note that GraphQL requires a running indexer, which will be enabled by default if the
    /// `--with-indexer` flag is not set.
    #[clap(
            long,
            default_missing_value = "0.0.0.0:9125",
            num_args = 0..=1,
            require_equals = true,
            value_name = "GRAPHQL_HOST_PORT"
    )]
    pub sui_with_graphql: Option<String>,

    /// Port for the Indexer Postgres DB. Default port is 5432.
    #[clap(long, default_value = "5432")]
    pub sui_pg_port: u16,

    /// Hostname for the Indexer Postgres DB. Default host is localhost.
    #[clap(long, default_value = "localhost")]
    pub sui_pg_host: String,

    /// DB name for the Indexer Postgres DB. Default DB name is sui_indexer.
    #[clap(long, default_value = "sui_indexer")]
    pub sui_pg_db_name: String,

    /// DB username for the Indexer Postgres DB. Default username is postgres.
    #[clap(long, default_value = "postgres")]
    pub sui_pg_user: String,

    /// DB password for the Indexer Postgres DB. Default password is postgrespw.
    #[clap(long, default_value = "postgrespw")]
    pub sui_pg_password: String,
}

impl SuiIndexerArgs {
    pub fn for_testing() -> Self {
        Self {
            sui_with_indexer: None,
            sui_with_graphql: None,
            sui_pg_port: 5432,
            sui_pg_host: "localhost".to_string(),
            sui_pg_db_name: "sui_indexer".to_string(),
            sui_pg_user: "postgres".to_string(),
            sui_pg_password: "postgrespw".to_string(),
        }
    }
}

#[derive(Args)]
pub struct SuiArgs {
    /// Config directory that will be used to store network config, node db, keystore
    /// sui genesis -f --with-faucet generates a genesis config that can be used to start this
    /// proces. Use with caution as the `-f` flag will overwrite the existing config directory.
    /// We can use any config dir that is generated by the `sui genesis`.
    #[clap(long = "sui-network.config")]
    pub sui_config_dir: Option<std::path::PathBuf>,

    /// A new genesis is created each time this flag is set, and state is not persisted between
    /// runs. Only use this flag when you want to start the network from scratch every time you
    /// run this command.
    ///
    /// To run with persisted state, do not pass this flag and use the `sui genesis` command
    /// to generate a genesis that can be used to start the network with.
    #[clap(long)]
    pub sui_force_regenesis: bool,

    /// Start a faucet with default host and port: 0.0.0.0:9123. This flag accepts also a
    /// port, a host, or both (e.g., 0.0.0.0:9123).
    /// When providing a specific value, please use the = sign between the flag and value:
    /// `--with-faucet=6124` or `--with-faucet=0.0.0.0`, or `--with-faucet=0.0.0.0:9123`
    #[clap(
            long,
            default_missing_value = "0.0.0.0:9123",
            num_args = 0..=1,
            require_equals = true,
            value_name = "FAUCET_HOST_PORT",
    )]
    pub sui_with_faucet: Option<String>,

    #[clap(flatten)]
    pub sui_indexer_feature_args: SuiIndexerArgs,

    /// Port to start the Fullnode RPC server on. Default port is 9000.
    #[clap(long, default_value = "9000")]
    pub sui_fullnode_rpc_port: u16,

    /// Set the epoch duration. Can only be used when `--force-regenesis` flag is passed or if
    /// there's no genesis config and one will be auto-generated. When this flag is not set but
    /// `--force-regenesis` is set, the epoch duration will be set to 60 seconds.
    #[clap(long)]
    pub sui_epoch_duration_ms: Option<u64>,

    /// Start the network without a fullnode
    #[clap(long)]
    pub sui_no_full_node: bool,
}

/// Starts a local Sui network with the given configuration.
pub(crate) async fn start_sui(
    config: Option<PathBuf>,
    with_faucet: Option<String>,
    indexer_feature_args: SuiIndexerArgs,
    force_regenesis: bool,
    epoch_duration_ms: Option<u64>,
    fullnode_rpc_port: u16,
    no_full_node: bool,
) -> Result<(), anyhow::Error> {
    if force_regenesis {
        ensure!(
            config.is_none(),
            "Cannot pass `--force-regenesis` and `--network.config` at the same time."
        );
    }

    let SuiIndexerArgs {
        mut sui_with_indexer,
        sui_with_graphql,
        sui_pg_port,
        sui_pg_host,
        sui_pg_db_name,
        sui_pg_user,
        sui_pg_password,
    } = indexer_feature_args;

    let pg_address = format!("postgres://{sui_pg_user}:{sui_pg_password}@{sui_pg_host}:{sui_pg_port}/{sui_pg_db_name}");

    if sui_with_graphql.is_some() {
        sui_with_indexer = Some(sui_with_indexer.unwrap_or_default());
    }

    if sui_with_indexer.is_some() {
        ensure!(
            !no_full_node,
            "Cannot start the indexer without a fullnode."
        );
    }

    if epoch_duration_ms.is_some() && genesis_blob_exists(config.clone()) && !force_regenesis {
        bail!(
            "Epoch duration can only be set when passing the `--force-regenesis` flag, or when \
            there is no genesis configuration in the default Sui configuration folder or the given \
            network.config argument.",
        );
    }

    let mut swarm_builder = Swarm::builder();
    // If this is set, then no data will be persisted between runs, and a new genesis will be
    // generated each run.
    if force_regenesis {
        swarm_builder =
            swarm_builder.committee_size(NonZeroUsize::new(DEFAULT_NUMBER_OF_AUTHORITIES).unwrap());
        let genesis_config = GenesisConfig::custom_genesis(1, 100);
        swarm_builder = swarm_builder.with_genesis_config(genesis_config);
        let epoch_duration_ms = epoch_duration_ms.unwrap_or(DEFAULT_EPOCH_DURATION_MS);
        swarm_builder = swarm_builder.with_epoch_duration_ms(epoch_duration_ms);
    } else {
        if config.is_none() && !sui_config_dir()?.join(SUI_NETWORK_CONFIG).exists() {
            genesis(None, None, None, false, epoch_duration_ms, None, false).await.map_err(|_| anyhow!("Cannot run genesis with non-empty Sui config directory: {}.\n\nIf you are trying to run a local network without persisting the data (so a new genesis that is randomly generated and will not be saved once the network is shut down), use --force-regenesis flag.\nIf you are trying to persist the network data and start from a new genesis, use sui genesis --help to see how to generate a new genesis.", sui_config_dir().unwrap().display()))?;
        }

        // Load the config of the Sui authority.
        // To keep compatibility with sui-test-validator where the user can pass a config
        // directory, this checks if the config is a file or a directory
        let network_config_path = if let Some(ref config) = config {
            if config.is_dir() {
                config.join(SUI_NETWORK_CONFIG)
            } else if config.is_file()
                && config
                .extension()
                .is_some_and(|ext| (ext == "yml" || ext == "yaml"))
            {
                config.clone()
            } else {
                config.join(SUI_NETWORK_CONFIG)
            }
        } else {
            config
                .clone()
                .unwrap_or(sui_config_dir()?)
                .join(SUI_NETWORK_CONFIG)
        };
        let network_config: NetworkConfig =
            PersistedConfig::read(&network_config_path).map_err(|err| {
                err.context(format!(
                    "Cannot open Sui network config file at {:?}",
                    network_config_path
                ))
            })?;

        swarm_builder = swarm_builder
            .dir(sui_config_dir()?)
            .with_network_config(network_config);
    }

    let data_ingestion_path = tempdir()?.into_path();

    // the indexer requires to set the fullnode's data ingestion directory
    // note that this overrides the default configuration that is set when running the genesis
    // command, which sets data_ingestion_dir to None.

    if sui_with_indexer.is_some() {
        swarm_builder = swarm_builder.with_data_ingestion_dir(data_ingestion_path.clone());
    }

    let mut fullnode_url = sui_config::node::default_json_rpc_address();
    fullnode_url.set_port(fullnode_rpc_port);

    if no_full_node {
        swarm_builder = swarm_builder.with_fullnode_count(0);
    } else {
        swarm_builder = swarm_builder
            .with_fullnode_count(1)
            .with_fullnode_rpc_addr(fullnode_url);
    }

    let mut swarm = swarm_builder.build();
    swarm.launch().await?;
    // Let nodes connect to one another
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    info!("Cluster started");

    // the indexer requires a fullnode url with protocol specified
    let fullnode_url = format!("http://{}", fullnode_url);
    info!("Fullnode URL: {}", fullnode_url);

    if let Some(input) = sui_with_indexer {
        let indexer_address = parse_host_port(input, DEFAULT_INDEXER_PORT)
            .map_err(|_| anyhow!("Invalid indexer host and port"))?;
        info!("Starting the indexer service at {indexer_address}");
        // Start in reader mode
        start_indexer_jsonrpc_for_testing(
            pg_address.clone(),
            fullnode_url.clone(),
            indexer_address.to_string(),
            None,
        )
            .await;
        info!("Indexer started in reader mode");
        start_indexer_writer_for_testing(
            pg_address.clone(),
            None,
            None,
            Some(data_ingestion_path.clone()),
            None,
            None, /* start_checkpoint */
            None, /* end_checkpoint */
        )
            .await;
        info!("Indexer started in writer mode");
    }

    if let Some(input) = sui_with_graphql {
        let graphql_address = parse_host_port(input, DEFAULT_GRAPHQL_PORT)
            .map_err(|_| anyhow!("Invalid graphql host and port"))?;
        tracing::info!("Starting the GraphQL service at {graphql_address}");
        let graphql_connection_config = ConnectionConfig {
            port: graphql_address.port(),
            host: graphql_address.ip().to_string(),
            db_url: pg_address,
            ..Default::default()
        };

        start_graphql_server_with_fn_rpc(
            graphql_connection_config,
            Some(fullnode_url.clone()),
            None, // it will be initialized by default
            ServiceConfig::test_defaults(),
        )
            .await;
        info!("GraphQL started");
    }

    if let Some(input) = with_faucet {
        let faucet_address = parse_host_port(input, DEFAULT_FAUCET_PORT)
            .map_err(|_| anyhow!("Invalid faucet host and port"))?;
        tracing::info!("Starting the faucet service at {faucet_address}");
        let config_dir = if force_regenesis {
            tempdir()?.into_path()
        } else {
            match config {
                Some(config) => config,
                None => sui_config_dir()?,
            }
        };

        let host_ip = match faucet_address {
            SocketAddr::V4(addr) => *addr.ip(),
            _ => bail!("Faucet configuration requires an IPv4 address"),
        };

        let config = FaucetConfig {
            host_ip,
            port: faucet_address.port(),
            num_coins: DEFAULT_FAUCET_NUM_COINS,
            amount: DEFAULT_FAUCET_MIST_AMOUNT,
            ..Default::default()
        };

        let prometheus_registry = prometheus::Registry::new();
        if force_regenesis {
            let kp = swarm.config_mut().account_keys.swap_remove(0);
            let keystore_path = config_dir.join(SUI_KEYSTORE_FILENAME);
            let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
            let address: SuiAddress = kp.public().into();
            keystore.add_key(None, SuiKeyPair::Ed25519(kp)).unwrap();
            SuiClientConfig {
                keystore,
                envs: vec![SuiEnv {
                    alias: "localnet".to_string(),
                    rpc: fullnode_url,
                    ws: None,
                    basic_auth: None,
                }],
                active_address: Some(address),
                active_env: Some("localnet".to_string()),
            }
                .persisted(config_dir.join(SUI_CLIENT_CONFIG).as_path())
                .save()
                .unwrap();
        }
        let faucet_wal = config_dir.join("faucet.wal");
        let simple_faucet = SimpleFaucet::new(
            create_wallet_context(config.wallet_client_timeout_secs, config_dir)?,
            &prometheus_registry,
            faucet_wal.as_path(),
            config.clone(),
        )
            .await
            .unwrap();

        let app_state = Arc::new(AppState {
            faucet: simple_faucet,
            config,
        });

        start_faucet(app_state, CONCURRENCY_LIMIT, &prometheus_registry).await?;
    }

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(3));
    let mut unhealthy_cnt = 0;
    loop {
        for node in swarm.validator_nodes() {
            if let Err(err) = node.health_check(true).await {
                unhealthy_cnt += 1;
                if unhealthy_cnt > 3 {
                    // The network could temporarily go down during reconfiguration.
                    // If we detect a failed validator 3 times in a row, give up.
                    return Err(err.into());
                }
                // Break the inner loop so that we could retry latter.
                break;
            } else {
                unhealthy_cnt = 0;
            }
        }

        interval.tick().await;
    }
}

async fn genesis(
    from_config: Option<PathBuf>,
    write_config: Option<PathBuf>,
    working_dir: Option<PathBuf>,
    force: bool,
    epoch_duration_ms: Option<u64>,
    benchmark_ips: Option<Vec<String>>,
    with_faucet: bool,
) -> Result<(), anyhow::Error> {
    let sui_config_dir = &match working_dir {
        // if a directory is specified, it must exist (it
        // will not be created)
        Some(v) => v,
        // create default Sui config dir if not specified
        // on the command line and if it does not exist
        // yet
        None => {
            let config_path = sui_config_dir()?;
            fs::create_dir_all(&config_path)?;
            config_path
        }
    };

    // if Sui config dir is not empty then either clean it
    // up (if --force/-f option was specified or report an
    // error
    let dir = sui_config_dir.read_dir().map_err(|err| {
        anyhow!(err).context(format!("Cannot open Sui config dir {:?}", sui_config_dir))
    })?;
    let files = dir.collect::<Result<Vec<_>, _>>()?;

    let client_path = sui_config_dir.join(SUI_CLIENT_CONFIG);
    let keystore_path = sui_config_dir.join(SUI_KEYSTORE_FILENAME);

    if write_config.is_none() && !files.is_empty() {
        if force {
            // check old keystore and client.yaml is compatible
            let is_compatible = FileBasedKeystore::new(&keystore_path).is_ok()
                && PersistedConfig::<SuiClientConfig>::read(&client_path).is_ok();
            // Keep keystore and client.yaml if they are compatible
            if is_compatible {
                for file in files {
                    let path = file.path();
                    if path != client_path && path != keystore_path {
                        if path.is_file() {
                            fs::remove_file(path)
                        } else {
                            fs::remove_dir_all(path)
                        }
                            .map_err(|err| {
                                anyhow!(err).context(format!("Cannot remove file {:?}", file.path()))
                            })?;
                    }
                }
            } else {
                fs::remove_dir_all(sui_config_dir).map_err(|err| {
                    anyhow!(err)
                        .context(format!("Cannot remove Sui config dir {:?}", sui_config_dir))
                })?;
                fs::create_dir(sui_config_dir).map_err(|err| {
                    anyhow!(err)
                        .context(format!("Cannot create Sui config dir {:?}", sui_config_dir))
                })?;
            }
        } else if files.len() != 2 || !client_path.exists() || !keystore_path.exists() {
            bail!("Cannot run genesis with non-empty Sui config directory {}, please use the --force/-f option to remove the existing configuration", sui_config_dir.to_str().unwrap());
        }
    }

    let network_path = sui_config_dir.join(SUI_NETWORK_CONFIG);
    let genesis_path = sui_config_dir.join(SUI_GENESIS_FILENAME);

    let mut genesis_conf = match from_config {
        Some(path) => PersistedConfig::read(&path)?,
        None => {
            if let Some(ips) = benchmark_ips {
                // Make a keystore containing the key for the genesis gas object.
                let path = sui_config_dir.join(SUI_BENCHMARK_GENESIS_GAS_KEYSTORE_FILENAME);
                let mut keystore = FileBasedKeystore::new(&path)?;
                for gas_key in GenesisConfig::benchmark_gas_keys(ips.len()) {
                    keystore.add_key(None, gas_key)?;
                }
                keystore.save()?;

                // Make a new genesis config from the provided ip addresses.
                GenesisConfig::new_for_benchmarks(&ips)
            } else if keystore_path.exists() {
                let existing_keys = FileBasedKeystore::new(&keystore_path)?.addresses();
                GenesisConfig::for_local_testing_with_addresses(existing_keys)
            } else {
                GenesisConfig::for_local_testing()
            }
        }
    };

    // Adds an extra faucet account to the genesis
    if with_faucet {
        info!("Adding faucet account in genesis config...");
        genesis_conf = genesis_conf.add_faucet_account();
    }

    if let Some(path) = write_config {
        let persisted = genesis_conf.persisted(&path);
        persisted.save()?;
        return Ok(());
    }

    let validator_info = genesis_conf.validator_config_info.take();
    let ssfn_info = genesis_conf.ssfn_config_info.take();

    let builder = ConfigBuilder::new(sui_config_dir);
    if let Some(epoch_duration_ms) = epoch_duration_ms {
        genesis_conf.parameters.epoch_duration_ms = epoch_duration_ms;
    }
    let mut network_config = if let Some(validators) = validator_info {
        builder
            .with_genesis_config(genesis_conf)
            .with_validators(validators)
            .build()
    } else {
        builder
            .committee_size(NonZeroUsize::new(DEFAULT_NUMBER_OF_AUTHORITIES).unwrap())
            .with_genesis_config(genesis_conf)
            .build()
    };

    let mut keystore = FileBasedKeystore::new(&keystore_path)?;
    for key in &network_config.account_keys {
        keystore.add_key(None, SuiKeyPair::Ed25519(key.copy()))?;
    }
    let active_address = keystore.addresses().pop();

    network_config.genesis.save(&genesis_path)?;
    for validator in &mut network_config.validator_configs {
        validator.genesis = sui_config::node::Genesis::new_from_file(&genesis_path);
    }

    info!("Network genesis completed.");
    network_config.save(&network_path)?;
    info!("Network config file is stored in {:?}.", network_path);

    info!("Client keystore is stored in {:?}.", keystore_path);

    let fullnode_config = FullnodeConfigBuilder::new()
        .with_config_directory(FULL_NODE_DB_PATH.into())
        .with_rpc_addr(sui_config::node::default_json_rpc_address())
        .build(&mut OsRng, &network_config);

    fullnode_config.save(sui_config_dir.join(SUI_FULLNODE_CONFIG))?;
    let mut ssfn_nodes = vec![];
    if let Some(ssfn_info) = ssfn_info {
        for (i, ssfn) in ssfn_info.into_iter().enumerate() {
            let path =
                sui_config_dir.join(sui_config::ssfn_config_file(ssfn.p2p_address.clone(), i));
            // join base fullnode config with each SsfnGenesisConfig entry
            let ssfn_config = FullnodeConfigBuilder::new()
                .with_config_directory(FULL_NODE_DB_PATH.into())
                .with_p2p_external_address(ssfn.p2p_address)
                .with_network_key_pair(ssfn.network_key_pair)
                .with_p2p_listen_address("0.0.0.0:8084".parse().unwrap())
                .with_db_path(PathBuf::from("/opt/sui/db/authorities_db/full_node_db"))
                .with_network_address("/ip4/0.0.0.0/tcp/8080/http".parse().unwrap())
                .with_metrics_address("0.0.0.0:9184".parse().unwrap())
                .with_admin_interface_port(1337)
                .with_json_rpc_address("0.0.0.0:9000".parse().unwrap())
                .with_genesis(Genesis::new_from_file("/opt/sui/config/genesis.blob"))
                .build(&mut OsRng, &network_config);
            ssfn_nodes.push(ssfn_config.clone());
            ssfn_config.save(path)?;
        }

        let ssfn_seed_peers: Vec<SeedPeer> = ssfn_nodes
            .iter()
            .map(|config| SeedPeer {
                peer_id: Some(anemo::PeerId(
                    config.network_key_pair().public().0.to_bytes(),
                )),
                address: config.p2p_config.external_address.clone().unwrap(),
            })
            .collect();

        for (i, mut validator) in network_config
            .into_validator_configs()
            .into_iter()
            .enumerate()
        {
            let path = sui_config_dir.join(sui_config::validator_config_file(
                validator.network_address.clone(),
                i,
            ));
            let mut val_p2p = validator.p2p_config.clone();
            val_p2p.seed_peers = ssfn_seed_peers.clone();
            validator.p2p_config = val_p2p;
            validator.save(path)?;
        }
    } else {
        for (i, validator) in network_config
            .into_validator_configs()
            .into_iter()
            .enumerate()
        {
            let path = sui_config_dir.join(sui_config::validator_config_file(
                validator.network_address.clone(),
                i,
            ));
            validator.save(path)?;
        }
    }

    let mut client_config = if client_path.exists() {
        PersistedConfig::read(&client_path)?
    } else {
        SuiClientConfig::new(keystore.into())
    };

    if client_config.active_address.is_none() {
        client_config.active_address = active_address;
    }

    // On windows, using 0.0.0.0 will usually yield in an networking error. This localnet ip
    // address must bind to 127.0.0.1 if the default 0.0.0.0 is used.
    let localnet_ip =
        if fullnode_config.json_rpc_address.ip() == IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)) {
            "127.0.0.1".to_string()
        } else {
            fullnode_config.json_rpc_address.ip().to_string()
        };
    client_config.add_env(SuiEnv {
        alias: "localnet".to_string(),
        rpc: format!(
            "http://{}:{}",
            localnet_ip,
            fullnode_config.json_rpc_address.port()
        ),
        ws: None,
        basic_auth: None,
    });
    client_config.add_env(SuiEnv::devnet());

    if client_config.active_env.is_none() {
        client_config.active_env = client_config.envs.first().map(|env| env.alias.clone());
    }

    client_config.save(&client_path)?;
    info!("Client config file is stored in {:?}.", client_path);

    Ok(())
}

/// Parse the input string into a SocketAddr, with a default port if none is provided.
pub fn parse_host_port(
    input: String,
    default_port_if_missing: u16,
) -> Result<SocketAddr, AddrParseError> {
    let default_host = "0.0.0.0";
    let mut input = input;
    if input.contains("localhost") {
        input = input.replace("localhost", "127.0.0.1");
    }
    if input.contains(':') {
        input.parse::<SocketAddr>()
    } else if input.contains('.') {
        format!("{input}:{default_port_if_missing}").parse::<SocketAddr>()
    } else if input.is_empty() {
        format!("{default_host}:{default_port_if_missing}").parse::<SocketAddr>()
    } else if !input.is_empty() {
        format!("{default_host}:{input}").parse::<SocketAddr>()
    } else {
        format!("{default_host}:{default_port_if_missing}").parse::<SocketAddr>()
    }
}

pub async fn sui_chain_identifier_after_genesis() -> Result<ChainIdentifier, anyhow::Error> {
    let sui_config_dir = sui_config_dir()?;

    let genesis_path = sui_config_dir.join(SUI_GENESIS_FILENAME);

    let genesis = Genesis::new_from_file(&genesis_path);
    let genesis = genesis.genesis()?;
    Ok(ChainIdentifier::from(genesis.checkpoint().digest().clone()))
}

pub(crate) async fn create_sui_transaction(
    signer: SuiAddress,
    tx_kind: TransactionKind,
    context: &mut WalletContext,
) -> Result<Transaction, anyhow::Error> {
    let gas_price = context.get_reference_gas_price().await?;

    let client = context.get_client().await?;

    let gas_budget = max_gas_budget(&client).await?;
    // let gas_budget = estimate_gas_budget(
    //     context,
    //     signer,
    //     tx_kind.clone(),
    //     gas_price,
    //     None,
    //     None,
    // ).await?;

    let tx_data = client
        .transaction_builder()
        .tx_data(
            signer,
            tx_kind,
            gas_budget,
            gas_price,
            vec![],
            None,
        )
        .await?;

    let signature = context.config.keystore.sign_secure(
        &tx_data.sender(),
        &tx_data,
        Intent::sui_transaction(),
    )?;
    let sender_signed_data = SenderSignedData::new_from_sender_signature(tx_data, signature);

    let transaction = Transaction::new(sender_signed_data);

    Ok(transaction)
}

pub(crate) async fn execute_sui_transaction(
    signer: SuiAddress,
    tx_kind: TransactionKind,
    context: &mut WalletContext,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let transaction = create_sui_transaction(signer, tx_kind, context).await?;

    let response = context
        .execute_transaction_may_fail(transaction.clone())
        .await?;
    Ok(response)
}

// Right now execute_sui_transaction crashes for somme txs, this is an hack to make it work for now.
pub(crate) async fn execute_sui_transaction_no_events(
    signer: SuiAddress,
    tx_kind: TransactionKind,
    context: &mut WalletContext,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let transaction = create_sui_transaction(signer, tx_kind, context).await?;

    let client = context.get_client().await?;

    let response = client
        .quorum_driver_api()
        .execute_transaction_block(
            transaction,
            SuiTransactionBlockResponseOptions::new()
                .with_effects()
                .with_input()
                .with_object_changes()
                .with_balance_changes(),
            Some(sui_types::quorum_driver_types::ExecuteTransactionRequestType::WaitForEffectsCert)).await?;
    Ok(response)
}

pub async fn estimate_gas_budget(
    context: &mut WalletContext,
    signer: SuiAddress,
    kind: TransactionKind,
    gas_price: u64,
    gas_payment: Option<Vec<ObjectID>>,
    sponsor: Option<SuiAddress>,
) -> Result<u64, anyhow::Error> {
    let client = context.get_client().await?;
    let SuiClientCommandResult::DryRun(dry_run) =
        execute_dry_run(context, signer, kind, None, gas_price, gas_payment, sponsor).await?
    else {
        bail!("Wrong SuiClientCommandResult. Should be SuiClientCommandResult::DryRun.")
    };

    let rgp = client.read_api().get_reference_gas_price().await?;

    Ok(estimate_gas_budget_from_gas_cost(
        dry_run.effects.gas_cost_summary(),
        rgp,
    ))
}