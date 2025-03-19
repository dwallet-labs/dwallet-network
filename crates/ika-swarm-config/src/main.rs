use anyhow::Result;
use clap::{Parser, Subcommand};
use ika_move_packages::BuiltInIkaMovePackages;
use ika_swarm_config::sui_client::{
    mint_ika, publish_ika_package_to_sui, publish_ika_system_package_to_sui,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use sui::client_commands::request_tokens_from_faucet;
use sui_config::{sui_config_dir, Config, SUI_NETWORK_CONFIG};
use sui_config::{SUI_CLIENT_CONFIG, SUI_KEYSTORE_FILENAME};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_keys::keystore::{InMemKeystore, Keystore};
use sui_rpc_api::client::reqwest::Url;
use sui_sdk::sui_client_config::{SuiClientConfig, SuiEnv};
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::{ObjectID, SuiAddress};
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
        #[clap(long, default_value = "http://127.0.0.1:9000")]
        sui_rpc_addr: String,
        /// Faucet URL for requesting tokens.
        #[clap(long, default_value = "http://127.0.0.1:9123/gas")]
        sui_faucet_addr: String,
        /// The optional path for network configuration.
        #[clap(long, value_parser = clap::value_parser!(PathBuf))]
        sui_conf_dir: Option<PathBuf>,
    },

    /// Mint IKA tokens.
    MintIkaTokens {
        /// The optional path for network configuration.
        #[clap(long, value_parser = clap::value_parser!(PathBuf))]
        sui_conf_dir: Option<PathBuf>,
        /// Path to the configuration file (e.g., `ika_publish_config.json`) generated during publish.
        #[arg(long, value_parser = clap::value_parser!(PathBuf))]
        ika_config_path: PathBuf,
        /// Faucet URL for requesting tokens.
        #[clap(long, default_value = "http://127.0.0.1:9123/gas")]
        sui_faucet_addr: String,
        /// RPC URL for the Sui network.
        #[clap(long, default_value = "http://127.0.0.1:9000")]
        sui_rpc_addr: String,
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

const ALIAS_PUBLISHER: &str = "publisher";

/// Configuration data that will be saved after publishing the IKA modules.
#[derive(Serialize, Deserialize)]
struct PublishIkaConfig {
    pub ika_package_id: ObjectID,
    pub treasury_cap_id: ObjectID,
    pub ika_package_upgrade_cap_id: ObjectID,
    pub ika_system_package_id: ObjectID,
    pub init_cap_id: ObjectID,
    pub ika_system_package_upgrade_cap_id: ObjectID,
    pub ika_supply_id: Option<ObjectID>, // Add this field
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::PublishIkaModules {
            sui_rpc_addr,
            sui_faucet_addr,
            sui_conf_dir,
        } => {
            println!("Publishing IKA modules on network: {}", sui_rpc_addr);

            let (keystore, publisher_address, sui_config_path) = init_sui_conf(sui_conf_dir)?;
            inti_sui_env(&sui_rpc_addr, keystore, publisher_address, &sui_config_path)?;
            request_tokens_from_faucet(publisher_address, sui_faucet_addr.clone()).await?;

            let mut context = WalletContext::new(&sui_config_path, None, None)?;
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
            println!("  ika_package_id: {ika_package_id}");
            println!("  treasury_cap_id: {treasury_cap_id}");
            println!("  ika_package_upgrade_cap_id: {ika_package_upgrade_cap_id}");

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
            println!("  ika_system_package_id: {ika_system_package_id}",);
            println!("  init_cap_id: {init_cap_id}",);
            println!("  ika_system_package_upgrade_cap_id: {ika_system_package_upgrade_cap_id}",);

            // Save the published package IDs into a configuration file.
            let publish_config = PublishIkaConfig {
                ika_package_id,
                treasury_cap_id,
                ika_package_upgrade_cap_id,
                ika_system_package_id,
                init_cap_id,
                ika_system_package_upgrade_cap_id,
                ika_supply_id: None,
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

        Commands::MintIkaTokens {
            ika_config_path,
            sui_conf_dir,
            sui_faucet_addr,
            sui_rpc_addr,
        } => {
            println!(
                "Minting IKA tokens using configuration from: {:?}",
                ika_config_path
            );
        
            let (keystore, publisher_address, sui_config_path) = init_sui_conf(sui_conf_dir)?;
            inti_sui_env(&sui_rpc_addr, keystore, publisher_address, &sui_config_path)?;
            request_tokens_from_faucet(publisher_address, sui_faucet_addr.clone()).await?;
            
            // Load the published IKA configuration from the file.
            let ika_config = std::fs::read_to_string(&ika_config_path)?;
            let mut publish_config: PublishIkaConfig = serde_json::from_str(&ika_config)?;
        
            // Create a WalletContext using the persisted SuiClientConfig.
            let mut context = WalletContext::new(&sui_config_path, None, None)?;
            let client = context.get_client().await?;
        
            // Call `mint_ika` with the publisher address, context,
            // client, IKA package ID, and treasury cap ID.
            let ika_supply_id = mint_ika(
                publisher_address,
                &mut context,
                client.clone(),
                publish_config.ika_package_id,
                publish_config.treasury_cap_id,
            )
            .await?;
            println!("Minting done: ika_supply_id: {}", ika_supply_id);
        
            // Update the configuration with the new ika_supply_id
            publish_config.ika_supply_id = Some(ika_supply_id);
        
            // Write the updated configuration back to the file
            let json = serde_json::to_string_pretty(&publish_config)?;
            let mut file = File::create(&ika_config_path)?;
            file.write_all(json.as_bytes())?;
            println!(
                "Updated IKA modules configuration saved to {:?}",
                ika_config_path
            );
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
            // let class_groups_bytes = std::fs::read(class_groups_file)?;
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

fn inti_sui_env(
    sui_rpc_addr: &String,
    keystore: Keystore,
    active_addr: SuiAddress,
    sui_config_path: &PathBuf,
) -> Result<()> {
    // // Parse the RPC URL to extract the host for naming the environment.
    let parsed_url = Url::parse(&sui_rpc_addr)?;
    let rpc_host = parsed_url.host_str().unwrap_or_default();
    let config = SuiClientConfig::load(sui_config_path).expect("Failed to load SuiClientConfig");
    if config.get_env(&Some(rpc_host.to_string())).is_none() {
        let mut config = SuiClientConfig::new(keystore);
        config.add_env(SuiEnv {
            alias: rpc_host.to_string(),
            rpc: sui_rpc_addr.clone(),
            ws: None,
            basic_auth: None,
        });
        config.active_address = Some(active_addr);
        config.active_env = Some(rpc_host.to_string());
        config.persisted(sui_config_path).save()?;
    }
    Ok(())
}

/// Initializes a keystore and returns the necessary components for SUI client configuration.
///
/// This function sets up a keystore based on the provided configuration directory or creates
/// a temporary one if none is provided. It ensures that a publisher key exists and is properly
/// configured.
///
/// # Arguments
///
/// * `sui_config_dir` — Optional path to a SUI configuration directory
///
/// # Returns
///
/// A tuple containing:
/// * `Keystore` — The initialized keystore (either file-based or in-memory)
/// * `SuiAddress` — The publisher's address
/// * `PathBuf` — The path to the SUI client configuration file
///
/// # Details
///
/// If `sui_config_dir` is provided:
/// * Uses a file-based keystore initialized from the given directory
/// * Ensures the "publisher" alias exists, creating it if necessary
/// * Retrieves the existing publisher address or generates a new one if not found
///
/// If `sui_config_dir` is None:
/// * Creates a temporary directory
/// * Uses an in-memory keystore
/// * Always generates a new key for the publisher.
///
/// # Errors
///
/// Returns an error if:
/// * Unable to create or access the keystore.
/// * Unable to create or update the publisher alias.
/// * Unable to generate a new key when needed.
fn init_sui_conf(sui_conf_dir: Option<PathBuf>) -> Result<(Keystore, SuiAddress, PathBuf)> {
    let sui_conf_dir = match sui_conf_dir {
        Some(dir) => dir,
        None => sui_config_dir()?,
    };
    let keystore_path = sui_conf_dir.join(SUI_KEYSTORE_FILENAME);

    let mut keystore = Keystore::File(FileBasedKeystore::new(&keystore_path)?);
    let sui_client_config_path = sui_conf_dir.join(SUI_CLIENT_CONFIG);
    println!(
        "Using SUI client configuration at: {:?}",
        sui_client_config_path
    );
    println!("Using keystore at: {:?}", keystore_path);

    let publisher_address = match &mut keystore {
        Keystore::File(fks) => {
            if !fks.alias_exists(ALIAS_PUBLISHER) {
                println!("Creating publisher alias: {}", ALIAS_PUBLISHER);
                fks.create_alias(Option::from(ALIAS_PUBLISHER.to_string()))?;
            }
            // Get the address by alias
            match fks.get_address_by_alias(ALIAS_PUBLISHER.to_string()) {
                Ok(address) => *address,
                Err(_) => {
                    // If getting the address fails, generate a new key
                    let (address, phrase, _) = fks.generate_and_add_new_key(
                        SignatureScheme::ED25519,
                        Some(ALIAS_PUBLISHER.to_string()),
                        None,
                        None,
                    )?;
                    println!("Generated a new publisher key with address: {}", address);
                    println!("Secret Recovery Phrase: {}", phrase);
                    address
                }
            }
        }
        _ => {
            unreachable!("In-memory keystore should not be used for the publisher key");
        }
    };

    Ok((keystore, publisher_address, sui_client_config_path))
}
