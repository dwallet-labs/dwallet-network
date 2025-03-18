use anyhow::Result;
use clap::{Parser, Subcommand};
use ika_move_packages::BuiltInIkaMovePackages;
use ika_swarm_config::sui_client::{publish_ika_package_to_sui, publish_ika_system_package_to_sui};
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use sui::client_commands::request_tokens_from_faucet;
use sui_config::SUI_CLIENT_CONFIG;
use sui_config::{sui_config_dir, Config, SUI_NETWORK_CONFIG};
use sui_keys::keystore::AccountKeystore;
use sui_keys::keystore::{InMemKeystore, Keystore};
use sui_rpc_api::client::reqwest::Url;
use sui_sdk::sui_client_config::{SuiClientConfig, SuiEnv};
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::crypto::SignatureScheme;
use tempfile;
use tokio::time::{sleep, Duration};

/// CLI for IKA operations on Sui.
#[derive(Parser)]
#[command(name = "ika-cli", about = "CLI for IKA operations on Sui")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Publish IKA modules.
    PublishIkaModules {
        /// RPC URL for the Sui network.
        #[arg(long)]
        rpc_addr: String,
        /// Faucet URL for requesting tokens.
        #[clap(
            long,
            default_value = "http://127.0.0.1:9123/gas",
        )]
        faucet_addr: String,
        /// The optional path for network configuration.
        #[clap(long = "network.config", value_parser = clap::value_parser!(PathBuf))]
        config: Option<PathBuf>,
    },

    /// Mint IKA tokens.
    MintIkaTokens {
        /// Path to the configuration file (e.g. `ika_publish_config.json`) generated during publish.
        #[arg(long, value_parser = clap::value_parser!(PathBuf))]
        config: PathBuf,
    },

    /// Initialize environment (calls the `INITIALIZE_FUNCTION_NAME` function).
    InitEnv {
        /// Path to the configuration file (e.g. `ika_publish_config.json`).
        #[arg(long, value_parser = clap::value_parser!(PathBuf))]
        config: PathBuf,
    },

    /// Create a validator candidate.
    CreateValidatorCandidate {
        /// Validator name.
        #[arg(long)]
        name: String,
        /// Protocol public key (as hex string or file path).
        #[arg(long)]
        protocol_key: String,
        /// Network public key.
        #[arg(long)]
        network_key: String,
        /// Consensus public key.
        #[arg(long)]
        consensus_key: String,
        /// Path to file with class groups public key and proof (raw bytes).
        #[arg(long, value_parser = clap::value_parser!(PathBuf))]
        class_groups_file: PathBuf,
        /// Proof of possession (as hex or string).
        #[arg(long)]
        proof_of_possession: String,
        /// Network address.
        #[arg(long)]
        network_address: String,
        /// P2P address.
        #[arg(long)]
        p2p_address: String,
        /// Current epoch consensus address.
        #[arg(long)]
        epoch_consensus_address: String,
        /// Computation price.
        #[arg(long)]
        computation_price: u64,
        /// Commission rate.
        #[arg(long)]
        commission_rate: u64,
        /// RPC URL for the Sui network.
        #[arg(long)]
        network: String,
    },

    /// Stake tokens for a given validator.
    StakeTokens {
        /// Validator address (as string).
        validator: String,
        /// Path to the configuration file (e.g. `ika_publish_config.json`).
        #[arg(long, value_parser = clap::value_parser!(PathBuf))]
        config: PathBuf,
    },
}

/// Configuration data that will be saved after publishing the IKA modules.
#[derive(Serialize)]
struct PublishIkaConfig {
    pub ika_package_id: ObjectID,
    pub treasury_cap_id: ObjectID,
    pub ika_package_upgrade_cap_id: ObjectID,
    pub ika_system_package_id: ObjectID,
    pub init_cap_id: ObjectID,
    pub ika_system_package_upgrade_cap_id: ObjectID,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Common initialization: you may want to create a wallet context here.
    // For example, create a temporary config directory or load an existing one.
    // (Adjust this to suit how you set up your WalletContext and key management.)
    match cli.command {
        Commands::PublishIkaModules { rpc_addr, faucet_addr, config } => {
            println!("Publishing IKA modules on network: {}", rpc_addr);

            // Determine the configuration directory.
            let config_dir = match config {
                Some(cfg) => cfg,
                None => tempfile::tempdir()?.into_path(),
            };
            let config_path = config_dir.join(SUI_CLIENT_CONFIG);

            // Create an in-memory keystore and generate a new publisher keypair.
            let mut keystore = Keystore::InMem(InMemKeystore::default());
            let alias = "publisher";
            let (publisher_address, phrase, _scheme, _publisher_keypair) = match &mut keystore {
                Keystore::InMem(k) => {
                    let _ = k.update_alias(alias, None);
                    let (publisher_address, phrase, scheme) = k.generate_and_add_new_key(
                        SignatureScheme::ED25519,
                        Some(alias.to_string()),
                        None,
                        None,
                    )?;

                    let publisher_keypair = k.get_key(&publisher_address)?.copy();

                    (publisher_address, phrase, scheme, publisher_keypair)
                }
                _ => {
                    panic!("Keystore is not in memory");
                }
            };
            println!(
                "Generated publisher keypair for address {} with alias \"{}\"",
                publisher_address, alias
            );
            println!("Secret Recovery Phrase: {}", phrase);

            // Parse the RPC URL to extract the host for naming the environment.
            let parsed_url = Url::parse(&rpc_addr)?;
            let rpc_host = parsed_url.host_str().unwrap_or_default();

            // Build the SuiClientConfig.
            let sui_client_config = SuiClientConfig {
                keystore,
                envs: vec![SuiEnv {
                    alias: rpc_host.to_string(),
                    rpc: rpc_addr.clone(),
                    ws: None,
                    basic_auth: None,
                }],
                active_address: Some(publisher_address),
                active_env: Some(rpc_host.to_string()),
            };
            sui_client_config.persisted(&config_path).save()?;

            // Request tokens from the faucet for the publisher.
            let faucet_future = request_tokens_from_faucet(publisher_address, faucet_addr.clone());
            // Await the faucet request; if needed, handle errors here.
            faucet_future.await?;

            // Create a WalletContext and obtain a SuiClient.
            let mut context = WalletContext::new(&config_path, None, None)?;
            let client = context.get_client().await?;

            // Load the IKA Move packages.
            let ika_package = BuiltInIkaMovePackages::get_package_by_name("ika");
            let ika_system_package = BuiltInIkaMovePackages::get_package_by_name("ika_system");

            // Publish the "ika" package.
            let (ika_package_id, treasury_cap_id, ika_package_upgrade_cap_id) =
                publish_ika_package_to_sui(
                    publisher_address,
                    &mut context,
                    client.clone(),
                    ika_package,
                )
                .await?;
            println!("Published IKA package:");
            println!("  ika_package_id: {}", ika_package_id);
            println!("  treasury_cap_id: {}", treasury_cap_id);
            println!(
                "  ika_package_upgrade_cap_id: {}",
                ika_package_upgrade_cap_id
            );

            // Allow a short delay between publishing calls.
            sleep(Duration::from_secs(2)).await;

            // Publish the "ika_system" package (which depends on the IKA package).
            let (ika_system_package_id, init_cap_id, ika_system_package_upgrade_cap_id) =
                publish_ika_system_package_to_sui(
                    publisher_address,
                    &mut context,
                    client.clone(),
                    ika_system_package,
                    ika_package_id,
                )
                .await?;
            println!("Published IKA system package:");
            println!("  ika_system_package_id: {}", ika_system_package_id);
            println!("  init_cap_id: {}", init_cap_id);
            println!(
                "  ika_system_package_upgrade_cap_id: {}",
                ika_system_package_upgrade_cap_id
            );

            // Save the published package IDs into a configuration file.
            let publish_config = PublishIkaConfig {
                ika_package_id,
                treasury_cap_id,
                ika_package_upgrade_cap_id,
                ika_system_package_id,
                init_cap_id,
                ika_system_package_upgrade_cap_id,
            };

            let config_file_path = PathBuf::from("ika_publish_config.json");
            let mut file = File::create(&config_file_path)?;
            let json = serde_json::to_string_pretty(&publish_config)?;
            file.write_all(json.as_bytes())?;
            println!(
                "Published IKA modules configuration saved to {:?}",
                config_file_path
            );
        }

        Commands::MintIkaTokens { config } => {
            println!("Minting IKA tokens using configuration at {:?}", config);
            // Load the configuration (e.g. deserialize ika_config.json to get ika_package_id and treasury_cap_id)
            // Create a WalletContext and SuiClient.
            // Call the mint_ika function:
            // let ika_supply_id = mint_ika(publisher_address, &mut context, client.clone(), ika_package_id, treasury_cap_id).await?;
            println!("(Pseudocode) IKA tokens minted. Supply id: ...");
        }

        Commands::InitEnv { config } => {
            println!(
                "Initializing environment using configuration at {:?}",
                config
            );
            // Load the configuration to get ika_system_package_id, system_id, etc.
            // Create a WalletContext.
            // Call the function that wraps the move call to INITIALIZE_FUNCTION_NAME:
            //   e.g. ika_system_initialize(publisher_address, &mut context, ika_system_package_id, system_id, init_system_shared_version).await?;
            println!("(Pseudocode) Environment initialized.");
        }

        Commands::CreateValidatorCandidate {
            name,
            protocol_key,
            network_key,
            consensus_key,
            class_groups_file,
            proof_of_possession,
            network_address,
            p2p_address,
            epoch_consensus_address,
            computation_price,
            commission_rate,
            network,
        } => {
            println!("Creating validator candidate: {}", name);
            // Load the file containing the class groups public key and proof bytes.
            let class_groups_bytes = std::fs::read(class_groups_file)?;
            // Parse or load other keys as needed. Here we assume the keys are provided as strings.
            //
            // Build a ValidatorInfo struct (or equivalent) with the provided parameters.
            // For example:
            // let validator_info = ValidatorInfo {
            //    name: name.clone(),
            //    protocol_public_key: parse_key(protocol_key)?,
            //    network_public_key: parse_key(network_key)?,
            //    consensus_public_key: parse_key(consensus_key)?,
            //    class_groups_public_key_and_proof: class_groups_bytes,
            //    proof_of_possession: hex::decode(proof_of_possession)?,
            //    network_address: network_address.clone(),
            //    p2p_address: p2p_address.clone(),
            //    current_epoch_consensus_address: epoch_consensus_address.clone(),
            //    computation_price: *computation_price,
            //    commission_rate: *commission_rate,
            // };
            //
            // Create a WalletContext using the provided network RPC URL.
            // Call request_add_validator_candidate(...) with the above info.
            //
            // (Again, below is pseudocode.)
            println!("(Pseudocode) Validator candidate created with the provided parameters on network {}.", network);
        }

        Commands::StakeTokens { validator, config } => {
            println!(
                "Staking tokens for validator: {} using configuration at {:?}",
                validator, config
            );
            // Load configuration to retrieve ika_system_package_id, system_id, init_system_shared_version, and ika_supply_id.
            // Create a WalletContext.
            // Convert the provided validator string into the appropriate SuiAddress.
            // Call stake_ika with the publisher address, context, and a vector containing the validator’s ID.
            println!("(Pseudocode) Tokens staked for validator {}.", validator);
        }
    }

    Ok(())
}
