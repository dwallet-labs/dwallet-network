// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::client_commands::SuiClientCommands;
use crate::console::start_console;
use crate::fire_drill::{run_fire_drill, FireDrill};
use crate::genesis_ceremony::{run, Ceremony};
use crate::keytool::KeyToolCommand;
use crate::validator_commands::SuiValidatorCommand;
use anyhow::{anyhow, bail};
use clap::*;
use fastcrypto::traits::KeyPair;
use move_package::BuildConfig;
use rand::rngs::OsRng;
use std::io::{stderr, stdout, Write};
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::{fs, io};
use std::time::Duration;
use sui_config::node::Genesis;
use sui_config::p2p::SeedPeer;
use sui_config::{
    sui_config_dir, Config, PersistedConfig, FULL_NODE_DB_PATH, SUI_CLIENT_CONFIG,
    SUI_FULLNODE_CONFIG, SUI_NETWORK_CONFIG,
};
use sui_config::{
    SUI_BENCHMARK_GENESIS_GAS_KEYSTORE_FILENAME, SUI_GENESIS_FILENAME, SUI_KEYSTORE_FILENAME,
};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_move::{self, execute_move_command};
use sui_move_build::SuiPackageHooks;
use sui_sdk::sui_client_config::{SuiClientConfig, SuiEnv};
use sui_sdk::wallet_context::WalletContext;
use sui_swarm::memory::Swarm;
use sui_swarm_config::genesis_config::{GenesisConfig, DEFAULT_NUMBER_OF_AUTHORITIES};
use sui_swarm_config::network_config::NetworkConfig;
use sui_swarm_config::network_config_builder::ConfigBuilder;
use sui_swarm_config::node_config_builder::FullnodeConfigBuilder;
use sui_types::crypto::{SignatureScheme, SuiKeyPair};
use tracing::info;

#[allow(clippy::large_enum_variant)]
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum SuiCommand {
    /// Start sui network.
    #[clap(name = "start")]
    Start {
        #[clap(long = "network.config")]
        config: Option<PathBuf>,
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
    /// Bootstrap and initialize a new sui network
    #[clap(name = "genesis")]
    Genesis {
        #[clap(long, help = "Start genesis with a given config file")]
        from_config: Option<PathBuf>,
        #[clap(
            long,
            help = "Build a genesis config, write it to the specified path, and exit"
        )]
        write_config: Option<PathBuf>,
        #[clap(long)]
        working_dir: Option<PathBuf>,
        #[clap(short, long, help = "Forces overwriting existing configuration")]
        force: bool,
        #[clap(long = "epoch-duration-ms")]
        epoch_duration_ms: Option<u64>,
        #[clap(
            long,
            value_name = "ADDR",
            num_args(1..),
            value_delimiter = ',',
            help = "A list of ip addresses to generate a genesis suitable for benchmarks"
        )]
        benchmark_ips: Option<Vec<String>>,
        #[clap(
            long,
            help = "Creates an extra faucet configuration for sui-test-validator persisted runs."
        )]
        with_faucet: bool,
    },
    GenesisCeremony(Ceremony),
    /// Sui keystore tool.
    #[clap(name = "keytool")]
    KeyTool {
        #[clap(long)]
        keystore_path: Option<PathBuf>,
        ///Return command outputs in json format
        #[clap(long, global = true)]
        json: bool,
        /// Subcommands.
        #[clap(subcommand)]
        cmd: KeyToolCommand,
    },
    /// Start Sui interactive console.
    #[clap(name = "console")]
    Console {
        /// Sets the file storing the state of our user accounts (an empty one will be created if missing)
        #[clap(long = "client.config")]
        config: Option<PathBuf>,
    },
    /// Client for interacting with the Sui network.
    #[clap(name = "client")]
    Client {
        /// Sets the file storing the state of our user accounts (an empty one will be created if missing)
        #[clap(long = "client.config")]
        config: Option<PathBuf>,
        #[clap(subcommand)]
        cmd: Option<SuiClientCommands>,
        /// Return command outputs in json format.
        #[clap(long, global = true)]
        json: bool,
        #[clap(short = 'y', long = "yes")]
        accept_defaults: bool,
    },
    /// A tool for validators and validator candidates.
    #[clap(name = "validator")]
    Validator {
        /// Sets the file storing the state of our user accounts (an empty one will be created if missing)
        #[clap(long = "client.config")]
        config: Option<PathBuf>,
        #[clap(subcommand)]
        cmd: Option<SuiValidatorCommand>,
        /// Return command outputs in json format.
        #[clap(long, global = true)]
        json: bool,
        #[clap(short = 'y', long = "yes")]
        accept_defaults: bool,
    },

    /// Tool to build and test Move applications.
    #[clap(name = "move")]
    Move {
        /// Path to a package which the command should be run with respect to.
        #[clap(long = "path", short = 'p', global = true)]
        package_path: Option<PathBuf>,
        /// Package build options
        #[clap(flatten)]
        build_config: BuildConfig,
        /// Subcommands.
        #[clap(subcommand)]
        cmd: sui_move::Command,
    },

    /// Tool for Fire Drill
    FireDrill {
        #[clap(subcommand)]
        fire_drill: FireDrill,
    },
}

impl SuiCommand {
    pub async fn execute(self) -> Result<(), anyhow::Error> {
        move_package::package_hooks::register_package_hooks(Box::new(SuiPackageHooks));
        match self {
            SuiCommand::Start {
                config,
                no_full_node,
            } => {
                // Auto genesis if path is none and sui directory doesn't exists.
                if config.is_none() && !sui_config_dir()?.join(SUI_NETWORK_CONFIG).exists() {
                    genesis(None, None, None, false, None, None, false).await?;
                }

                // Load the config of the Sui authority.
                let network_config_path = config
                    .clone()
                    .unwrap_or(sui_config_dir()?.join(SUI_NETWORK_CONFIG));
                let network_config: NetworkConfig = PersistedConfig::read(&network_config_path)
                    .map_err(|err| {
                        err.context(format!(
                            "Cannot open Sui network config file at {:?}",
                            network_config_path
                        ))
                    })?;
                let mut swarm_builder = Swarm::builder()
                    .dir(sui_config_dir()?)
                    .with_network_config(network_config);
                if no_full_node {
                    swarm_builder = swarm_builder.with_fullnode_count(0);
                } else {
                    swarm_builder = swarm_builder
                        .with_fullnode_count(1)
                        .with_fullnode_rpc_addr(sui_config::node::default_json_rpc_address());
                }
                let mut swarm = swarm_builder.build();
                swarm.launch().await?;

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
            SuiCommand::Network {
                config,
                dump_addresses,
            } => {
                let config_path = config.unwrap_or(sui_config_dir()?.join(SUI_NETWORK_CONFIG));
                let config: NetworkConfig = PersistedConfig::read(&config_path).map_err(|err| {
                    err.context(format!(
                        "Cannot open Sui network config file at {:?}",
                        config_path
                    ))
                })?;

                if dump_addresses {
                    for validator in config.validator_configs() {
                        println!(
                            "{} - {}",
                            validator.network_address(),
                            validator.protocol_key_pair().public(),
                        );
                    }
                }
                Ok(())
            }
            SuiCommand::Genesis {
                working_dir,
                force,
                from_config,
                write_config,
                epoch_duration_ms,
                benchmark_ips,
                with_faucet,
            } => {
                genesis(
                    from_config,
                    write_config,
                    working_dir,
                    force,
                    epoch_duration_ms,
                    benchmark_ips,
                    with_faucet,
                )
                .await
            }
            SuiCommand::GenesisCeremony(cmd) => run(cmd),
            SuiCommand::KeyTool {
                keystore_path,
                json,
                cmd,
            } => {
                let keystore_path =
                    keystore_path.unwrap_or(sui_config_dir()?.join(SUI_KEYSTORE_FILENAME));
                let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
                cmd.execute(&mut keystore).await?.print(!json);
                Ok(())
            }
            SuiCommand::Console { config } => {
                let config = config.unwrap_or(sui_config_dir()?.join(SUI_CLIENT_CONFIG));
                prompt_if_no_config(&config, false).await?;
                let context = WalletContext::new(&config, None, None).await?;
                start_console(context, &mut stdout(), &mut stderr()).await
            }
            SuiCommand::Client {
                config,
                cmd,
                json,
                accept_defaults,
            } => {
                let config_path = config.unwrap_or(sui_config_dir()?.join(SUI_CLIENT_CONFIG));
                prompt_if_no_config(&config_path, accept_defaults).await?;
                let mut context = WalletContext::new(&config_path, Some(Duration::new(60, 0)), None).await?;
                if let Some(cmd) = cmd {
                    cmd.execute(&mut context).await?.print(!json);
                } else {
                    // Print help
                    let mut app: Command = SuiCommand::command();
                    app.build();
                    app.find_subcommand_mut("client").unwrap().print_help()?;
                }
                Ok(())
            }
            SuiCommand::Validator {
                config,
                cmd,
                json,
                accept_defaults,
            } => {
                let config_path = config.unwrap_or(sui_config_dir()?.join(SUI_CLIENT_CONFIG));
                prompt_if_no_config(&config_path, accept_defaults).await?;
                let mut context = WalletContext::new(&config_path, None, None).await?;
                if let Some(cmd) = cmd {
                    cmd.execute(&mut context).await?.print(!json);
                } else {
                    // Print help
                    let mut app: Command = SuiCommand::command();
                    app.build();
                    app.find_subcommand_mut("validator").unwrap().print_help()?;
                }
                Ok(())
            }
            SuiCommand::Move {
                package_path,
                build_config,
                cmd,
            } => execute_move_command(package_path, build_config, cmd),
            SuiCommand::FireDrill { fire_drill } => run_fire_drill(fire_drill).await,
        }
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
    client_config.add_env(SuiEnv {
        alias: "localnet".to_string(),
        rpc: format!("http://{}", fullnode_config.json_rpc_address),
        ws: None,
        eth_execution_rpc: None,
        eth_consensus_rpc: None,
        eth_genesis_time: None,
        eth_chain_id: None,
        eth_genesis_validators_root: None,
        state_object_id: None,
    });
    client_config.add_env(SuiEnv::devnet());

    if client_config.active_env.is_none() {
        client_config.active_env = client_config.envs.first().map(|env| env.alias.clone());
    }

    client_config.save(&client_path)?;
    info!("Client config file is stored in {:?}.", client_path);

    Ok(())
}

async fn prompt_if_no_config(
    wallet_conf_path: &Path,
    accept_defaults: bool,
) -> Result<(), anyhow::Error> {
    // Prompt user for connect to devnet fullnode if config does not exist.
    if !wallet_conf_path.exists() {
        let env = match std::env::var_os("SUI_CONFIG_WITH_RPC_URL") {
            Some(v) => Some(SuiEnv {
                alias: "custom".to_string(),
                rpc: v.into_string().unwrap(),
                ws: None,
                eth_execution_rpc: None,
                eth_consensus_rpc: None,
                eth_genesis_time: None,
                eth_chain_id: None,
                eth_genesis_validators_root: None,
                state_object_id: None,
            }),
            None => {
                if accept_defaults {
                    print!("Creating config file [{:?}] with default (devnet) Full node server and ed25519 key scheme.", wallet_conf_path);
                } else {
                    print!(
                        "Config file [{:?}] doesn't exist, do you want to connect to a dWallet Full node server [y/N]?",
                        wallet_conf_path
                    );
                }
                if accept_defaults
                    || matches!(read_line(), Ok(line) if line.trim().to_lowercase() == "y")
                {
                    let url = if accept_defaults {
                        String::new()
                    } else {
                        print!(
                            "dWallet Full node server URL (Defaults to dWallet Testnet if not specified) : "
                        );
                        read_line()?
                    };
                    Some(if url.trim().is_empty() {
                        SuiEnv::devnet()
                    } else {
                        print!("Environment alias for [{url}] : ");
                        let alias = read_line()?;
                        let alias = if alias.trim().is_empty() {
                            "custom".to_string()
                        } else {
                            alias
                        };
                        SuiEnv {
                            alias,
                            rpc: url,
                            ws: None,
                            eth_execution_rpc: None,
                            eth_consensus_rpc: None,
                            eth_genesis_time: None,
                            eth_chain_id: None,
                            eth_genesis_validators_root: None,
                            state_object_id: None,
                        }
                    })
                } else {
                    None
                }
            }
        };

        if let Some(env) = env {
            let keystore_path = wallet_conf_path
                .parent()
                .unwrap_or(&sui_config_dir()?)
                .join(SUI_KEYSTORE_FILENAME);
            let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
            let key_scheme = if accept_defaults {
                SignatureScheme::ED25519
            } else {
                println!("Select key scheme to generate keypair (0 for ed25519, 1 for secp256k1, 2: for secp256r1):");
                match SignatureScheme::from_flag(read_line()?.trim()) {
                    Ok(s) => s,
                    Err(e) => return Err(anyhow!("{e}")),
                }
            };
            let (new_address, phrase, scheme) =
                keystore.generate_and_add_new_key(key_scheme, None, None, None)?;
            let alias = keystore.get_alias_by_address(&new_address)?;
            println!(
                "Generated new keypair and alias for address with scheme {:?} [{alias}: {new_address}]",
                scheme.to_string()
            );
            println!("Secret Recovery Phrase : [{phrase}]");
            let alias = env.alias.clone();
            SuiClientConfig {
                keystore,
                envs: vec![env],
                active_address: Some(new_address),
                active_env: Some(alias),
                dwallets: vec![],
                active_dwallet: None,
            }
            .persisted(wallet_conf_path)
            .save()?;
        }
    }
    Ok(())
}

fn read_line() -> Result<String, anyhow::Error> {
    let mut s = String::new();
    let _ = stdout().flush();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim_end().to_string())
}
