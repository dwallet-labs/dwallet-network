use anyhow::Result;
use clap::{Parser, Subcommand};
use fastcrypto::traits::EncodeDecodeBase64;
use ika_config::initiation::InitiationParameters;
use ika_swarm_config::sui_client::{
    ika_system_initialize, ika_system_request_dwallet_network_encryption_key_dkg_by_cap,
    init_initialize, minted_ika, publish_ika_common_package_to_sui,
    publish_ika_dwallet_2pc_mpc_package_to_sui, publish_ika_package_to_sui,
    publish_ika_system_package_to_sui, setup_contract_paths,
    ika_system_set_witness_approving_advance_epoch, ika_system_add_upgrade_cap_by_cap,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use sui::client_commands::request_tokens_from_faucet;
use sui_config::SUI_KEYSTORE_FILENAME;
use sui_config::{sui_config_dir, Config, SUI_CLIENT_CONFIG};
use sui_keys::keystore::Keystore;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::sui_client_config::{SuiClientConfig, SuiEnv};
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::{ObjectID, SequenceNumber, SuiAddress};
use sui_types::crypto::SignatureScheme;
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
        /// Path to the configuration file (e.g., `ika_publish_config.json`) generated during publishing.
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
        /// Path to the configuration file (e.g., `ika_publish_config.json`).
        #[arg(long, value_parser = clap::value_parser!(PathBuf))]
        ika_config_path: PathBuf,
        /// The optional path for network configuration.
        #[clap(long, value_parser = clap::value_parser!(PathBuf))]
        sui_conf_dir: Option<PathBuf>,
        /// RPC URL for the Sui network.
        #[clap(long, default_value = "http://127.0.0.1:9000")]
        sui_rpc_addr: String,
        /// Epoch Duration
        #[clap(long)]
        epoch_duration_ms: Option<u64>,
        /// Protocol Version
        #[clap(long)]
        protocol_version: Option<u64>,
    },

    /// IKA system initialization.
    /// This command calls the functions to perform the system initialization and then
    /// requests the dwallet network decryption key.
    IkaSystemInitialize {
        /// Path to the configuration file (e.g., `ika_publish_config.json`).
        #[arg(long, value_parser = clap::value_parser!(PathBuf))]
        ika_config_path: PathBuf,
        /// The optional path for network configuration.
        #[clap(long, value_parser = clap::value_parser!(PathBuf))]
        sui_conf_dir: Option<PathBuf>,
        /// RPC URL for the Sui network.
        #[clap(long, default_value = "http://127.0.0.1:9000")]
        sui_rpc_addr: String,
    },

    /// Convert IKA configuration JSON to Terraform format.
    ConvertToTerraform {
        /// Path to the configuration file (e.g., `ika_publish_config.json`).
        #[arg(long, value_parser = clap::value_parser!(PathBuf))]
        ika_config_path: PathBuf,
        /// Sui chain identifier for Terraform.
        #[clap(long, default_value = "custom")]
        sui_chain_identifier: String,
        /// Output file path for the Terraform file.
        #[clap(long, value_parser = clap::value_parser!(PathBuf))]
        output_path: Option<PathBuf>,
    },
}

const ALIAS_PUBLISHER: &str = "publisher";

/// Configuration data that will be saved after publishing the IKA modules.
#[derive(Serialize, Deserialize)]
struct PublishIkaConfig {
    pub publisher_address: SuiAddress,
    pub ika_package_id: ObjectID,
    pub treasury_cap_id: ObjectID,
    pub ika_package_upgrade_cap_id: ObjectID,
    pub ika_common_package_id: Option<ObjectID>,
    pub ika_common_package_upgrade_cap_id: Option<ObjectID>,
    pub ika_system_package_id: ObjectID,
    pub init_cap_id: ObjectID,
    pub ika_system_package_upgrade_cap_id: ObjectID,
    pub ika_dwallet_2pc_mpc_package_id: Option<ObjectID>,
    pub ika_dwallet_2pc_mpc_init_id: Option<ObjectID>,
    pub ika_dwallet_2pc_mpc_package_upgrade_cap_id: Option<ObjectID>,
    pub ika_supply_id: Option<ObjectID>,
    pub ika_system_object_id: Option<ObjectID>,
    pub protocol_cap_id: Option<ObjectID>,
    pub init_system_shared_version: Option<u64>,
    pub ika_dwallet_coordinator_object_id: Option<ObjectID>,
}

impl PublishIkaConfig {
    /// Convert the configuration to Terraform format
    pub fn to_terraform(&self, sui_chain_identifier: &str) -> String {
        let mut tf_content = String::new();
        tf_content.push_str("locals {\n");
        tf_content.push_str("  ika_chain_config = {\n");
        
        // Add sui_chain_identifier
        tf_content.push_str(&format!("    sui_chain_identifier              = \"{}\"\n", sui_chain_identifier));
        
        // Add required package IDs
        tf_content.push_str(&format!("    ika_package_id                    = \"{}\"\n", self.ika_package_id));
        
        let ika_common_package_id = self.ika_common_package_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "placeholder".to_string());
        tf_content.push_str(&format!("    ika_common_package_id             = \"{}\"\n", ika_common_package_id));
        
        let ika_dwallet_2pc_mpc_package_id = self.ika_dwallet_2pc_mpc_package_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "placeholder".to_string());
        tf_content.push_str(&format!("    ika_dwallet_2pc_mpc_package_id    = \"{}\"\n", ika_dwallet_2pc_mpc_package_id));
        
        tf_content.push_str(&format!("    ika_system_package_id             = \"{}\"\n", self.ika_system_package_id));
        
        let ika_system_object_id = self.ika_system_object_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "placeholder".to_string());
        tf_content.push_str(&format!("    ika_system_object_id              = \"{}\"\n", ika_system_object_id));
        
        let ika_dwallet_coordinator_object_id = self.ika_dwallet_coordinator_object_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "placeholder".to_string());
        tf_content.push_str(&format!("    ika_dwallet_coordinator_object_id = \"{}\"\n", ika_dwallet_coordinator_object_id));
        
        tf_content.push_str("  }\n");
        tf_content.push_str("}\n");
        
        tf_content
    }
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

            let (keystore, publisher_address, sui_config_path) = init_sui_keystore(sui_conf_dir)?;
            inti_sui_client_conf(&sui_rpc_addr, keystore, publisher_address, &sui_config_path)?;
            request_tokens_from_faucet(publisher_address, sui_faucet_addr.clone()).await?;

            let mut context = WalletContext::new(&sui_config_path)?;

            // Setup contract paths.
            let contract_paths = setup_contract_paths()?;

            // Publish the "ika" package.
            let (ika_package_id, treasury_cap_id, ika_package_upgrade_cap_id) =
                publish_ika_package_to_sui(&mut context, contract_paths.ika_contract_path).await?;
            println!("Published IKA package:");
            println!("  ika_package_id: {ika_package_id}");
            println!("  treasury_cap_id: {treasury_cap_id}");
            println!("  ika_package_upgrade_cap_id: {ika_package_upgrade_cap_id}");

            // Allow a short delay between publishing calls.
            sleep(Duration::from_secs(2)).await;

            // Publish the "ika_common" package.
            let (ika_common_package_id, ika_common_package_upgrade_cap_id) =
                publish_ika_common_package_to_sui(
                    &mut context,
                    contract_paths.ika_common_contract_path,
                )
                .await?;
            println!("Published IKA common package:");
            println!("  ika_common_package_id: {ika_common_package_id}");
            println!("  ika_common_package_upgrade_cap_id: {ika_common_package_upgrade_cap_id}");

            sleep(Duration::from_secs(2)).await;

            // Publish the "ika_system" package.
            let (ika_system_package_id, init_cap_id, ika_system_package_upgrade_cap_id) =
                publish_ika_system_package_to_sui(
                    &mut context,
                    contract_paths.ika_system_contract_path,
                )
                .await?;
            println!("Published IKA system package:");
            println!("  ika_system_package_id: {ika_system_package_id}");
            println!("  init_cap_id: {init_cap_id}");
            println!("  ika_system_package_upgrade_cap_id: {ika_system_package_upgrade_cap_id}");

            sleep(Duration::from_secs(2)).await;

            // Publish the "ika_dwallet_2pc_mpc" package.
            let (
                ika_dwallet_2pc_mpc_package_id,
                ika_dwallet_2pc_mpc_init_id,
                ika_dwallet_2pc_mpc_package_upgrade_cap_id,
            ) = publish_ika_dwallet_2pc_mpc_package_to_sui(
                &mut context,
                contract_paths.ika_dwallet_2pc_mpc_contract_path,
            )
            .await?;
            println!("Published IKA dwallet 2pc mpc package:");
            println!("  ika_dwallet_2pc_mpc_package_id: {ika_dwallet_2pc_mpc_package_id}");
            println!("  ika_dwallet_2pc_mpc_init_id: {ika_dwallet_2pc_mpc_init_id}");
            println!("  ika_dwallet_2pc_mpc_package_upgrade_cap_id: {ika_dwallet_2pc_mpc_package_upgrade_cap_id}");

            // Save the published package IDs into a configuration file.
            let publish_config = PublishIkaConfig {
                publisher_address,
                ika_package_id,
                treasury_cap_id,
                ika_package_upgrade_cap_id,
                ika_common_package_id: Some(ika_common_package_id),
                ika_common_package_upgrade_cap_id: Some(ika_common_package_upgrade_cap_id),
                ika_system_package_id,
                init_cap_id,
                ika_system_package_upgrade_cap_id,
                ika_dwallet_2pc_mpc_package_id: Some(ika_dwallet_2pc_mpc_package_id),
                ika_dwallet_2pc_mpc_init_id: Some(ika_dwallet_2pc_mpc_init_id),
                ika_dwallet_2pc_mpc_package_upgrade_cap_id: Some(
                    ika_dwallet_2pc_mpc_package_upgrade_cap_id,
                ),
                ika_supply_id: None,
                ika_system_object_id: None,
                protocol_cap_id: None,
                init_system_shared_version: None,
                ika_dwallet_coordinator_object_id: None,
            };

            let config_file_path = contract_paths
                .current_working_dir
                .join("ika_publish_config.json");
            let mut file = File::create(&config_file_path)?;
            let json = serde_json::to_string_pretty(&publish_config)?;
            file.write_all(json.as_bytes())?;
            println!(
                "Published IKA modules configuration saved to {:?}:\n {:?}",
                config_file_path,
                json
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

            let (keystore, publisher_address, sui_config_path) = init_sui_keystore(sui_conf_dir)?;
            println!("Using SUI configuration from: {:?}", sui_config_path);
            inti_sui_client_conf(&sui_rpc_addr, keystore, publisher_address, &sui_config_path)?;
            println!("Using SUI faucet address: {}", sui_faucet_addr);
            request_tokens_from_faucet(publisher_address, sui_faucet_addr.clone()).await?;

            // Load the published IKA configuration from the file.
            let ika_config = fs::read_to_string(&ika_config_path)?;
            let mut publish_config: PublishIkaConfig = serde_json::from_str(&ika_config)?;

            println!("Using publisher address: {}", publisher_address);

            // Create a WalletContext using the persisted SuiClientConfig.
            let context = WalletContext::new(&sui_config_path)?;
            let client = context.get_client().await?;

            println!("Using SUI client configuration from: {:?}", sui_config_path);

            // Call `mint_ika` with the publisher address, context,
            // client, IKA package ID, and treasury cap ID.
            let ika_supply_id = minted_ika(
                publisher_address,
                client.clone(),
                publish_config.ika_package_id,
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

        Commands::InitEnv {
            ika_config_path,
            sui_conf_dir,
            sui_rpc_addr,
            epoch_duration_ms,
            protocol_version,
        } => {
            println!(
                "Initializing environment using configuration at {:?}",
                ika_config_path
            );

            let config_content = fs::read_to_string(&ika_config_path)?;
            let mut publish_config: PublishIkaConfig = serde_json::from_str(&config_content)?;

            let (keystore, publisher_address, sui_config_path) = init_sui_keystore(sui_conf_dir)?;
            inti_sui_client_conf(&sui_rpc_addr, keystore, publisher_address, &sui_config_path)?;
            println!("Using SUI configuration from: {:?}", sui_config_path);

            // Create a WalletContext and obtain a Sui client.
            let mut context = WalletContext::new(&sui_config_path)?;
            let client = context.get_client().await?;

            let mut initiation_parameters = InitiationParameters::new();
            if let Some(epoch_duration_ms) = epoch_duration_ms {
                initiation_parameters.epoch_duration_ms = epoch_duration_ms;
            }
            if let Some(protocol_version) = protocol_version {
                initiation_parameters.protocol_version = protocol_version;
            }

            let (ika_system_object_id, protocol_cap_id, init_system_shared_version) =
                init_initialize(
                    publisher_address,
                    &mut context,
                    client.clone(),
                    publish_config.ika_system_package_id,
                    publish_config.init_cap_id,
                    publish_config.ika_package_upgrade_cap_id,
                    publish_config.ika_system_package_upgrade_cap_id,
                    publish_config.treasury_cap_id,
                    initiation_parameters,
                )
                .await
                .expect("Failed to initialize the IKA system");
            println!(
                "Environment initialized successfully with ika_system_object_id: {ika_system_object_id},\
                 protocol_cap_id: {protocol_cap_id},\
                  init_system_shared_version: {init_system_shared_version}",
            );

            // Call the witness-related functions that were missing from the CLI
            let ika_dwallet_2pc_mpc_package_id = publish_config.ika_dwallet_2pc_mpc_package_id.ok_or_else(|| {
                anyhow::Error::msg(
                    "`ika_dwallet_2pc_mpc_package_id` not found in configuration. Please run publish-ika-modules first.",
                )
            })?;

            ika_system_set_witness_approving_advance_epoch(
                publisher_address,
                &mut context,
                client.clone(),
                publish_config.ika_system_package_id,
                ika_system_object_id,
                init_system_shared_version,
                protocol_cap_id,
                ika_dwallet_2pc_mpc_package_id,
            )
            .await?;
            println!("Running `system::set_witness_approving_advance_epoch` done.");

            let ika_common_package_upgrade_cap_id = publish_config.ika_common_package_upgrade_cap_id.ok_or_else(|| {
                anyhow::Error::msg(
                    "`ika_common_package_upgrade_cap_id` not found in configuration. Please run publish-ika-modules first.",
                )
            })?;

            let ika_dwallet_2pc_mpc_package_upgrade_cap_id = publish_config.ika_dwallet_2pc_mpc_package_upgrade_cap_id.ok_or_else(|| {
                anyhow::Error::msg(
                    "`ika_dwallet_2pc_mpc_package_upgrade_cap_id` not found in configuration. Please run publish-ika-modules first.",
                )
            })?;

            ika_system_add_upgrade_cap_by_cap(
                publisher_address,
                &mut context,
                client.clone(),
                publish_config.ika_system_package_id,
                ika_system_object_id,
                init_system_shared_version,
                protocol_cap_id,
                ika_common_package_upgrade_cap_id,
                ika_dwallet_2pc_mpc_package_upgrade_cap_id,
            )
            .await?;
            println!("Running `system::add_upgrade_cap_by_cap` done.");

            // Update the configuration with the new fields
            publish_config.ika_system_object_id = Some(ika_system_object_id);
            publish_config.protocol_cap_id = Some(protocol_cap_id);
            publish_config.init_system_shared_version = Some(init_system_shared_version.into());

            // Write the updated configuration back to the file
            let json = serde_json::to_string_pretty(&publish_config)?;
            let mut file = File::create(&ika_config_path)?;
            file.write_all(json.as_bytes())?;
            println!(
                "Updated IKA modules configuration saved to {:?}",
                ika_config_path
            );
        }

        Commands::IkaSystemInitialize {
            ika_config_path,
            sui_conf_dir,
            sui_rpc_addr,
        } => {
            println!(
                "Starting IKA system initialization using configuration at {:?}",
                ika_config_path
            );

            // Load the published config.
            let config_content = fs::read_to_string(&ika_config_path)?;
            let mut publish_config: PublishIkaConfig =
                serde_json::from_str(&config_content).expect("Failed to parse IKA configuration");

            // Check that the required fields are present.
            let ika_system_object_id = publish_config.ika_system_object_id.ok_or_else(|| {
                anyhow::Error::msg(
                    "`ika_system_object_id` not found in configuration. Please run init-env first.",
                )
            })?;
            let init_system_shared_version = publish_config.init_system_shared_version.ok_or_else(|| {
                anyhow::Error::msg("`init_system_shared_version` not found in configuration. Please run init-env first.")
            })?;
            let protocol_cap_id = publish_config.protocol_cap_id.ok_or_else(|| {
                anyhow::Error::msg(
                    "`protocol_cap_id` not found in configuration. Please run init-env first.",
                )
            })?;
            let ika_system_package_id = publish_config.ika_system_package_id;
            let ika_dwallet_2pc_mpc_package_id = publish_config.ika_dwallet_2pc_mpc_package_id.ok_or_else(|| {
                anyhow::Error::msg(
                    "`ika_dwallet_2pc_mpc_package_id` not found in configuration. Please run publish-ika-modules first.",
                )
            })?;
            let ika_dwallet_2pc_mpc_init_id = publish_config.ika_dwallet_2pc_mpc_init_id.ok_or_else(|| {
                anyhow::Error::msg(
                    "`ika_dwallet_2pc_mpc_init_id` not found in configuration. Please run publish-ika-modules first.",
                )
            })?;

            // Initialize the SUI configuration.
            let (keystore, publisher_address, sui_config_path) = init_sui_keystore(sui_conf_dir)?;
            inti_sui_client_conf(&sui_rpc_addr, keystore, publisher_address, &sui_config_path)?;
            println!("Using SUI configuration from: {:?}", sui_config_path);

            // Create a WalletContext and Sui client.
            let mut context = WalletContext::new(&sui_config_path)?;
            let client = context.get_client().await?;

            let initiation_parameters = InitiationParameters::new();

            // Call ika_system_initialize.
            let (dwallet_coordinator_object_id, dwallet_initial_shared_version) = ika_system_initialize(
                publisher_address,
                &mut context,
                client.clone(),
                ika_system_package_id,
                ika_system_object_id,
                SequenceNumber::from(init_system_shared_version),
                protocol_cap_id,
                ika_dwallet_2pc_mpc_package_id,
                ika_dwallet_2pc_mpc_init_id,
                initiation_parameters.max_validator_change_count,
            )
            .await?;
            println!(
                "system::initialize done. `dwallet_id`: {}, `initial_shared_version`: {}",
                dwallet_coordinator_object_id, dwallet_initial_shared_version
            );

            // object_id = 0xacdb9188b62bea2201a836361f5f20374d8402cd5f200d6f92e06a604d4fb2a8
            // 1

            // Call ika_system_request_dwallet_network_decryption_key_dkg_by_cap
            ika_system_request_dwallet_network_encryption_key_dkg_by_cap(
                publisher_address,
                &mut context,
                client.clone(),
                ika_system_package_id,
                ika_dwallet_2pc_mpc_package_id,
                ika_system_object_id,
                SequenceNumber::from(init_system_shared_version),
                dwallet_coordinator_object_id,
                dwallet_initial_shared_version,
                protocol_cap_id,
            )
            .await?;
            println!("system::request_dwallet_network_decryption_key_dkg_by_cap done.");

            // Update the configuration with the dwallet_coordinator_object_id
            publish_config.ika_dwallet_coordinator_object_id = Some(dwallet_coordinator_object_id);

            // Write the updated configuration back to the file
            let json = serde_json::to_string_pretty(&publish_config)?;
            let mut file = File::create(&ika_config_path)?;
            file.write_all(json.as_bytes())?;
            println!(
                "Updated IKA modules configuration saved to {:?}",
                ika_config_path
            );

            // Optionally, update the configuration file if needed.
            // For example, you might want to store dwallet_id or other values.
            // Here, we simply print a success message.
            println!("IKA system initialization completed successfully.");
        }

        Commands::ConvertToTerraform {
            ika_config_path,
            sui_chain_identifier,
            output_path,
        } => {
            println!(
                "Converting IKA configuration from {:?} to Terraform format for chain: {}",
                ika_config_path, sui_chain_identifier
            );

                         let config_content = fs::read_to_string(&ika_config_path)?;
             let publish_config: PublishIkaConfig = serde_json::from_str(&config_content)?;

             let terraform_content = publish_config.to_terraform(&sui_chain_identifier);

            let output_path = output_path.unwrap_or_else(|| {
                ika_config_path.parent().unwrap().join("terraform.tf")
            });
            let mut file = File::create(&output_path)?;
            file.write_all(terraform_content.as_bytes())?;
            println!(
                "Terraform configuration saved to {:?}:\n {}",
                output_path,
                terraform_content
            );
        }
    }

    Ok(())
}

fn inti_sui_client_conf(
    sui_rpc_addr: &str,
    keystore: Keystore,
    active_addr: SuiAddress,
    sui_config_path: &PathBuf,
) -> Result<()> {
    // Parse the RPC URL to extract the host for naming the environment.
    let parsed_url = url::Url::parse(sui_rpc_addr)?;
    let rpc_host = parsed_url.host_str().unwrap_or_default();
    let mut config =
        SuiClientConfig::load(sui_config_path).unwrap_or_else(|_| SuiClientConfig::new(keystore));
    if config.get_env(&Some(rpc_host.to_string())).is_none() {
        config.add_env(SuiEnv {
            alias: rpc_host.to_string(),
            rpc: sui_rpc_addr.to_string(),
            ws: None,
            basic_auth: None,
        });
    }
    config.active_address = Some(active_addr);
    config.active_env = Some(rpc_host.to_string());
    config.persisted(sui_config_path).save()?;
    Ok(())
}

fn init_sui_keystore(sui_conf_dir: Option<PathBuf>) -> Result<(Keystore, SuiAddress, PathBuf)> {
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
        Keystore::File(file_ks) => {
            if !file_ks.alias_exists(ALIAS_PUBLISHER) {
                println!("Creating publisher alias: {}", ALIAS_PUBLISHER);
                file_ks.create_alias(Option::from(ALIAS_PUBLISHER.to_string()))?;
            }

            match file_ks.get_address_by_alias(ALIAS_PUBLISHER.to_string()) {
                Ok(address) => *address,
                Err(_) => {
                    // Generate a new key if not found
                    let (address, phrase, _) = file_ks.generate_and_add_new_key(
                        SignatureScheme::ED25519,
                        Some(ALIAS_PUBLISHER.to_string()),
                        None,
                        Some("word24".to_string()),
                    )?;

                    println!("Generated a new publisher key with address: {}", address);
                    println!("Secret Recovery Phrase: {}", phrase);

                    let publisher_keypair = file_ks.get_key(&address)?.copy();
                    let encoded = publisher_keypair.encode_base64();
                    let publisher_key_path = sui_conf_dir.join("publisher.key");
                    let mut file = File::create(&publisher_key_path)?;
                    writeln!(file, "{}", encoded)?;
                    println!("Saved key to {:?}", publisher_key_path);

                    // Write the phrase to publisher.seed
                    let seed_path = sui_conf_dir.join("publisher.seed");
                    let mut file = File::create(&seed_path)?;
                    writeln!(file, "{}", phrase)?;
                    println!("Saved recovery phrase to {:?}", seed_path);
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
