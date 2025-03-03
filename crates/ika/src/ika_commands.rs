// Copyright (c) dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::{anyhow, bail, ensure, Context};
use clap::*;
use colored::Colorize;
use fastcrypto::traits::KeyPair;
use ika_config::p2p::SeedPeer;
use ika_config::{
    ika_config_dir, network_config_exists, Config, PersistedConfig, FULL_NODE_DB_PATH,
    IKA_CLIENT_CONFIG, IKA_FULLNODE_CONFIG, IKA_NETWORK_CONFIG,
};
use ika_config::{
    IKA_BENCHMARK_GENESIS_GAS_KEYSTORE_FILENAME, IKA_GENESIS_FILENAME, IKA_KEYSTORE_FILENAME,
};
use move_analyzer::analyzer;
use move_binary_format::file_format::AddressIdentifierIndex;
use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use move_package::BuildConfig;
use rand::rngs::OsRng;
use shared_crypto::intent::Intent;
use std::io::{stderr, stdout, Write};
use std::net::{AddrParseError, IpAddr, Ipv4Addr, SocketAddr};
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fs, io, thread};
use sui::client_commands::{
    estimate_gas_budget_from_gas_cost, execute_dry_run, request_tokens_from_faucet,
    SuiClientCommandResult,
};
use sui_config::{sui_config_dir, SUI_CLIENT_CONFIG, SUI_KEYSTORE_FILENAME};
use sui_keys::keystore::{FileBasedKeystore, InMemKeystore, Keystore};
use sui_sdk::rpc_types::{ObjectChange, SuiObjectDataOptions, SuiTransactionBlockResponseOptions};
use sui_sdk::sui_client_config::{SuiClientConfig, SuiEnv};
use sui_sdk::SuiClient;

use crate::validator_commands::IkaValidatorCommand;
use ika_move_packages::IkaMovePackage;
use ika_swarm::memory::Swarm;
use ika_swarm_config::network_config::NetworkConfig;
use ika_swarm_config::network_config_builder::ConfigBuilder;
use ika_swarm_config::node_config_builder::FullnodeConfigBuilder;
use ika_swarm_config::validator_initialization_config::{
    ValidatorInitializationConfig, DEFAULT_NUMBER_OF_AUTHORITIES,
};
use ika_types::governance::{
    MIN_VALIDATOR_JOINING_STAKE_NIKA, VALIDATOR_LOW_STAKE_GRACE_PERIOD,
    VALIDATOR_LOW_STAKE_THRESHOLD_NIKA, VALIDATOR_VERY_LOW_STAKE_THRESHOLD_NIKA,
};
use ika_types::ika_coin::{IKACoin, IKA, TOTAL_SUPPLY_NIKA};
use ika_types::sui::System;
use sui_keys::keystore::AccountKeystore;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::{ObjectID, SequenceNumber, SuiAddress};
use sui_types::coin::{TreasuryCap, COIN_MODULE_NAME};
use sui_types::crypto::{SignatureScheme, SuiKeyPair, ToFromBytes};
use sui_types::move_package::PACKAGE_MODULE_NAME;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{Argument, CallArg, ObjectArg, TransactionData, TransactionKind};
use sui_types::SUI_FRAMEWORK_PACKAGE_ID;
use tempfile::tempdir;
use tokio::runtime::Runtime;
use tracing;
use tracing::{debug, info};

const DEFAULT_EPOCH_DURATION_MS: u64 = 1000000000000;

#[allow(clippy::large_enum_variant)]
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum IkaCommand {
    /// Start a local network in two modes: saving state between re-runs and not saving state
    /// between re-runs. Please use (--help) to see the full description.
    ///
    /// By default, ika start will start a local network from the initiation blob that exists in
    /// the Ika config default dir or in the config_dir that was passed. If the default directory
    /// does not exist and the config_dir is not passed, it will generate a new default directory,
    /// generate the initiation blob, and start the network.
    ///
    /// Note that if you want to start an indexer, Postgres DB is required.
    ///
    /// ProtocolConfig parameters can be overridden individually by setting env variables as
    /// follows:
    /// - IKA_PROTOCOL_CONFIG_OVERRIDE_ENABLE=1
    /// - Then, to configure an override, use the prefix `IKA_PROTOCOL_CONFIG_OVERRIDE_`
    ///   along with the parameter name. For example, to increase the interval between
    ///   checkpoint creation to >1/s, you might set:
    ///   IKA_PROTOCOL_CONFIG_OVERRIDE_min_checkpoint_interval_ms=1000
    ///
    /// Note that ProtocolConfig parameters must match between all nodes, or the network
    /// may break. Changing these values outside local networks is very dangerous.
    #[clap(name = "start", verbatim_doc_comment)]
    Start {
        /// Config directory that will be used to store network config, node db, keystore
        /// ika initiation -f --with-faucet generates a initiation config that can be used to start this
        /// process. Use with caution as the `-f` flag will overwrite the existing config directory.
        /// We can use any config dir that is generated by the `ika initiation`.
        #[clap(long = "network.config")]
        config_dir: Option<std::path::PathBuf>,

        /// A new initiation is created each time this flag is set, and state is not persisted between
        /// runs. Only use this flag when you want to start the network from scratch every time you
        /// run this command.
        ///
        /// To run with persisted state, do not pass this flag and use the `ika initiation` command
        /// to generate a initiation that can be used to start the network with.
        #[clap(long)]
        force_reinitiation: bool,

        /// Sui full node rpc url. Default is http://127.0.0.1:9000.
        #[clap(
            long,
            default_value = "http://127.0.0.1:9000",
            value_name = "SUI_FULLNODE_RPC_URL"
        )]
        sui_fullnode_rpc_url: String,

        /// Sui faucet url. Default is http://127.0.0.1:9123/gas.
        #[clap(
            long,
            default_value = "http://127.0.0.1:9123/gas",
            value_name = "SUI_FAUCET_URL"
        )]
        sui_faucet_url: String,

        /// Set the epoch duration. Can only be used when `--force-reinitiation` flag is passed or if
        /// there's no initiation config and one will be auto-generated. When this flag is not set but
        /// `--force-reinitiation` is set, the epoch duration will be set to 60 seconds.
        #[clap(long)]
        epoch_duration_ms: Option<u64>,

        /// Start the network without a fullnode
        #[clap(long = "no-full-node")]
        no_full_node: bool,
    },
    #[clap(name = "network")]
    Network {
        #[clap(long = "network.config")]
        config: Option<PathBuf>,
        #[clap(short, long, help = "Dump the public keys of all authorities")]
        dump_addresses: bool,
    },

    /// A tool for validators and validator candidates.
    #[clap(name = "validator")]
    Validator {
        /// Sets the file storing the state of our user accounts (an empty one will be created if missing)
        #[clap(long = "client.config")]
        config: Option<PathBuf>,
        #[clap(subcommand)]
        cmd: Option<IkaValidatorCommand>,
        /// Return command outputs in json format.
        #[clap(long, global = true)]
        json: bool,
        #[clap(short = 'y', long = "yes")]
        accept_defaults: bool,
    },
}

impl IkaCommand {
    pub async fn execute(self) -> Result<(), anyhow::Error> {
        //move_package::package_hooks::register_package_hooks(Box::new(SuiPackageHooks));
        match self {
            IkaCommand::Network {
                config,
                dump_addresses,
            } => {
                let config_path = config.unwrap_or(ika_config_dir()?.join(IKA_NETWORK_CONFIG));
                let config: NetworkConfig = PersistedConfig::read(&config_path).map_err(|err| {
                    err.context(format!(
                        "Cannot open Ika network config file at {:?}",
                        config_path
                    ))
                })?;

                if dump_addresses {
                    println!("Validators:");
                    for validator in config.validator_configs() {
                        println!(
                            "{} - {}",
                            validator.network_address(),
                            validator.protocol_key_pair().public(),
                        );
                    }
                    println!("Fullnodes:");
                    for fullnode in config.fullnode_configs() {
                        println!(
                            "{} - {}",
                            fullnode.network_address(),
                            fullnode.protocol_key_pair().public(),
                        );
                    }
                }
                Ok(())
            }
            IkaCommand::Start {
                config_dir,
                force_reinitiation,
                sui_fullnode_rpc_url,
                sui_faucet_url,
                no_full_node,
                epoch_duration_ms,
            } => {
                let thread_builder = thread::Builder::new();
                const SIXTY_FOUR_MB: usize = 67108864;
                let thread_builder = thread_builder.stack_size(SIXTY_FOUR_MB);
                let thread_join_handle = thread_builder.spawn(move || {
                    let Ok(mut tokio_runtime) = Runtime::new() else {
                        eprintln!("{}", "[error] Failed to start tokio runtime".red().bold());
                        return;
                    };
                    tokio_runtime.block_on(async move {
                        if let Err(e) = start(
                            config_dir.clone(),
                            force_reinitiation,
                            epoch_duration_ms,
                            sui_fullnode_rpc_url,
                            sui_faucet_url,
                            no_full_node,
                        )
                        .await
                        {
                            eprintln!("{}", format!("[error] {e}").red().bold());
                        }
                    });
                })?;

                if let Err(e) = thread_join_handle.join() {
                    eprintln!("{}", format!("[error] {:?}", e).red().bold());
                }

                Ok(())
            }
            IkaCommand::Validator {
                config,
                cmd,
                json,
                accept_defaults,
            } => {
                let config_path = config.unwrap_or(sui_config_dir()?.join(SUI_CLIENT_CONFIG));
                // prompt_if_no_config(&config_path, accept_defaults).await?;
                let mut context = WalletContext::new(&config_path, None, None)?;
                if let Some(cmd) = cmd {
                    if let Ok(client) = context.get_client().await {
                        if let Err(e) = client.check_api_version() {
                            eprintln!("{}", format!("[warning] {e}").yellow().bold());
                        }
                    }
                    cmd.execute(&mut context).await?.print(!json);
                } else {
                    // Print help
                    let mut app: Command = IkaCommand::command();
                    app.build();
                    app.find_subcommand_mut("validator").unwrap().print_help()?;
                }
                Ok(())
            }
        }
    }
}

/// Starts a local network with the given configuration.
async fn start(
    config: Option<PathBuf>,
    force_reinitiation: bool,
    epoch_duration_ms: Option<u64>,
    sui_fullnode_rpc_url: String,
    sui_faucet_url: String,
    no_full_node: bool,
) -> Result<(), anyhow::Error> {
    if force_reinitiation {
        ensure!(
            config.is_none(),
            "Cannot pass `--force-reinitiation` and `--network.config` at the same time."
        );
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;

    if epoch_duration_ms.is_some() && network_config_exists(config.clone()) && !force_reinitiation {
        bail!(
            "Epoch duration can only be set when passing the `--force-reinitiation` flag, or when \
            there is no network.config in the default Ika configuration folder or the given \
            network.config argument.",
        );
    }

    let network_config_path = if let Some(ref config) = config {
        if config.is_dir() {
            config.join(IKA_NETWORK_CONFIG)
        } else if config.is_file()
            && config
                .extension()
                .is_some_and(|ext| (ext == "yml" || ext == "yaml"))
        {
            config.clone()
        } else {
            config.join(IKA_NETWORK_CONFIG)
        }
    } else {
        config
            .clone()
            .unwrap_or(ika_config_dir()?)
            .join(IKA_NETWORK_CONFIG)
    };
    let mut swarm_builder = Swarm::builder();
    // If this is set, then no data will be persisted between runs, and a new initiation will be
    // generated each run.
    let ika_network_config_not_exists =
        config.is_none() && !ika_config_dir()?.join(IKA_NETWORK_CONFIG).exists();
    if force_reinitiation {
        swarm_builder =
            swarm_builder.committee_size(NonZeroUsize::new(DEFAULT_NUMBER_OF_AUTHORITIES).unwrap());
        let epoch_duration_ms = epoch_duration_ms.unwrap_or(DEFAULT_EPOCH_DURATION_MS);
        swarm_builder = swarm_builder.with_epoch_duration_ms(epoch_duration_ms);
    } else {
        swarm_builder = swarm_builder.dir(ika_config_dir()?);
        if ika_network_config_not_exists {
            swarm_builder = swarm_builder
                .committee_size(NonZeroUsize::new(DEFAULT_NUMBER_OF_AUTHORITIES).unwrap());
            let epoch_duration_ms = epoch_duration_ms.unwrap_or(DEFAULT_EPOCH_DURATION_MS);
            swarm_builder = swarm_builder.with_epoch_duration_ms(epoch_duration_ms);
        } else {
            let network_config: NetworkConfig = PersistedConfig::read(&network_config_path)
                .map_err(|err| {
                    err.context(format!(
                        "Cannot open Ika network config file at {:?}",
                        network_config_path
                    ))
                })?;

            swarm_builder = swarm_builder.with_network_config(network_config);
        }
    }

    if no_full_node {
        swarm_builder = swarm_builder.with_fullnode_count(0);
    } else {
        swarm_builder = swarm_builder.with_fullnode_count(1);
    }

    let mut swarm = swarm_builder.build().await?;
    if ika_network_config_not_exists {
        swarm.network_config.save(&network_config_path)?;
    }

    swarm.launch().await?;
    // Let nodes connect to one another
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    info!("Cluster started");

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

fn read_line() -> Result<String, anyhow::Error> {
    let mut s = String::new();
    let _ = stdout().flush();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim_end().to_string())
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
