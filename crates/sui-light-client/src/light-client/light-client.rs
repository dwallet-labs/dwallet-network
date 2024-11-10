// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

mod utils;

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use move_core_types::{account_address::AccountAddress, identifier::Identifier};
use object_store::path::Path;
use object_store::ObjectStore;
use sui_json_rpc_types::{
    Coin, SuiObjectDataOptions, SuiTransactionBlockEffectsAPI, SuiTransactionBlockResponse,
    SuiTransactionBlockResponseOptions,
};

use sui_json_rpc_types::{EventFilter, ObjectChange};

use sui_rest_api::CheckpointData;
use sui_types::base_types::{EpochId, SuiAddress};
use sui_types::transaction::{ObjectArg, ProgrammableTransaction};
use sui_types::{
    base_types::{ObjectID, ObjectRef},
    committee::Committee,
    crypto::AuthorityQuorumSignInfo,
    message_envelope::Envelope,
    messages_checkpoint::{CertifiedCheckpointSummary, CheckpointSummary, EndOfEpochData},
    object::{Object, Owner},
};

use sui_config::genesis::Genesis;

use sui_package_resolver::Result as ResolverResult;
use sui_package_resolver::{Package, PackageStore, Resolver};
use sui_sdk::{SuiClient, SuiClientBuilder};

use clap::{Parser, Subcommand};
use std::{fs, io::Write, path::PathBuf, str::FromStr};
use std::{io::Read, sync::Arc};

use move_core_types::language_storage::{StructTag, TypeTag};
use serde_json::Value;
use shared_crypto::intent::Intent;
use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::types::{
    programmable_transaction_builder::ProgrammableTransactionBuilder,
    quorum_driver_types::ExecuteTransactionRequestType,
    transaction::{Command, ProgrammableMoveCall, Transaction, TransactionData},
};

use crate::utils::fetch_or_request_coins;
use object_store::parse_url;
use reqwest::Client;
use serde_json::json;
use std::future::Future;
use std::time::Duration;
use std::{collections::HashMap, sync::Mutex};
use url::Url;

const DWALLET_MODULE_ADDR: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000003";
const SUI_DWALLET_MODULE_NAME: &str = "dwallet_cap";
const SUI_STATE_PROOF_MODULE_IN_DWALLET_NETWORK: &str = "sui_state_proof";
const GAS_BUDGET: u64 = 1_000_000_000;
const DWALLET_COIN_TYPE: &str = "0x2::dwlt::DWLT";

// todo(yuval): make this code work with .dwallet and not .sui.
// todo(yuval): rename in .move file: epoch_committee_id -> new_epoch_committee_id.

/// A light client for the Sui blockchain
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<SCommands>,
}

struct RemotePackageStore {
    config: Config,
    cache: Mutex<HashMap<AccountAddress, Arc<Package>>>,
}

impl RemotePackageStore {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            cache: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl PackageStore for RemotePackageStore {
    /// Read package contents. Fails if `id` is not an object, not a package, or is malformed in
    /// some way.
    async fn fetch(&self, id: AccountAddress) -> ResolverResult<Arc<Package>> {
        // Check if we have it in the cache
        if let Some(package) = self.cache.lock().unwrap().get(&id) {
            // info!("Fetch Package: {} cache hit", id);
            return Ok(package.clone());
        }

        println!("Fetch Package: {}", id);

        let object: Object = get_verified_object(&self.config, id.into()).await.unwrap();
        let package = Arc::new(Package::read_from_object(&object)?);

        // Add to the cache
        self.cache.lock().unwrap().insert(id, package.clone());

        Ok(package)
    }
}

#[derive(Subcommand, Debug)]
enum SCommands {
    /// Sync all end-of-epoch checkpoints
    Init {
        #[arg(short, long, value_name = "TID")]
        ckp_id: u64,
    },

    Sync {},

    /// Checks a specific transaction using the light client
    Transaction {
        /// Transaction hash
        #[arg(short, long, value_name = "TID")]
        tid: String,
    },
}

/// The config file for the Light Client.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct Config {
    /// SUI Full node url.
    sui_full_node_url: String,

    /// DWallet Full node url.
    dwallet_full_node_url: String,

    /// DWallet Faucet url.
    dwallet_faucet_url: String,

    /// Checkpoint summary directory
    checkpoint_summary_dir: PathBuf,

    /// Genesis file name.
    genesis_filename: PathBuf,

    /// Object store url
    object_store_url: String,

    /// GraphQL endpoint
    graphql_url: String,

    /// SUI deployed state proof package.
    sui_deployed_state_proof_package: String,

    /// Sui Light Client Registry (State) object ID in dWallet Network.
    dwallet_network_registry_object_id: String,

    /// Sui Light Client Config object ID in dWallet Network.
    dwallet_network_config_object_id: String,
}

/// The list of checkpoints at the end of each epoch.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct CheckpointsList {
    // End of epoch checkpoints list.
    checkpoints: Vec<u64>,
}

fn read_checkpoint_list(config: &Config) -> Result<CheckpointsList> {
    let mut checkpoints_path = config.checkpoint_summary_dir.clone();
    checkpoints_path.push("checkpoints.yaml");
    // Read the resulting file and parse the YAML checkpoint list.
    let reader = fs::File::open(checkpoints_path.clone())?;
    Ok(serde_yaml::from_reader(reader)?)
}

fn read_checkpoint(
    config: &Config,
    seq: u64,
) -> Result<Envelope<CheckpointSummary, AuthorityQuorumSignInfo<true>>> {
    let mut checkpoint_path = config.checkpoint_summary_dir.clone();
    checkpoint_path.push(format!("{}.yaml", seq));
    let mut reader = fs::File::open(checkpoint_path.clone())?;
    let metadata = fs::metadata(&checkpoint_path)?;
    let mut buffer = vec![0; metadata.len() as usize];
    reader.read_exact(&mut buffer)?;
    bcs::from_bytes(&buffer).map_err(|_| anyhow!("Unable to parse checkpoint file"))
}

fn write_checkpoint(
    config: &Config,
    summary: &Envelope<CheckpointSummary, AuthorityQuorumSignInfo<true>>,
) -> Result<()> {
    write_checkpoint_general(config, summary, None)
}

fn write_checkpoint_general(
    config: &Config,
    summary: &Envelope<CheckpointSummary, AuthorityQuorumSignInfo<true>>,
    path: Option<&str>,
) -> Result<()> {
    // Write the checkpoint summary to a file
    let mut checkpoint_path = config.checkpoint_summary_dir.clone();
    if let Some(path) = path {
        checkpoint_path.push(path);
    }
    checkpoint_path.push(format!("{}.yaml", summary.sequence_number));
    let mut writer = fs::File::create(checkpoint_path.clone())?;
    let bytes =
        bcs::to_bytes(&summary).map_err(|_| anyhow!("Unable to serialize checkpoint summary"))?;
    writer.write_all(&bytes)?;
    Ok(())
}

fn write_checkpoint_list(config: &Config, checkpoints_list: &CheckpointsList) -> Result<()> {
    // Write the checkpoint list to a file
    let mut checkpoints_path = config.checkpoint_summary_dir.clone();
    checkpoints_path.push("checkpoints.yaml");
    let mut writer = fs::File::create(checkpoints_path.clone())?;
    let bytes = serde_yaml::to_vec(&checkpoints_list)?;
    writer
        .write_all(&bytes)
        .map_err(|_| anyhow!("Unable to serialize the checkpoint list"))
}

async fn download_checkpoint_summary(
    config: &Config,
    checkpoint_number: u64,
) -> Result<CertifiedCheckpointSummary> {
    const MAX_RETRIES: u32 = 5;
    const INITIAL_DELAY: u64 = 2000;

    let bytes = retry_with_backoff(MAX_RETRIES, INITIAL_DELAY, || {
        // Clone config for closure capture.
        let config = config.clone();

        async move {
            let url =
                Url::parse(&config.object_store_url).context("Failed to parse object store URL")?;

            let (dyn_store, _store_path) =
                parse_url(&url).context("Failed to parse URL into object store components")?;

            let path = Path::from(format!("{}.chk", checkpoint_number));
            let response = dyn_store.get(&path).await.context(format!(
                "Failed to download checkpoint data for {checkpoint_number}"
            ))?;

            let bytes = response
                .bytes()
                .await
                .context("Failed to read bytes from response")?;
            
            println!("Downloaded checkpoint #{} data", checkpoint_number);
            Ok(bytes)
        }
    })
    .await?;

    let (_, blob) = bcs::from_bytes::<(u8, CheckpointData)>(&bytes)
        .map_err(|e| {
            println!("Failed to deserialize checkpoint data; {}", e);
            e
        })
        .context(format!(
            "Failed to deserialize checkpoint #{} data",
            checkpoint_number
        ))?;

    println!(
        "Successfully deserialized checkpoint summary #{} ",
        checkpoint_number
    );
    Ok(blob.checkpoint_summary)
}

async fn retry_with_backoff<F, T>(
    max_retries: u32,
    initial_delay: u64,
    operation: impl Fn() -> F,
) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    let mut attempt = 0;
    let mut delay = initial_delay;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                attempt += 1;
                if attempt > max_retries {
                    return Err(anyhow!(
                        "Operation failed after {} attempts: {}",
                        max_retries,
                        err
                    ));
                }

                println!(
                    "Attempt {} failed with error: {}. Retrying in {} ms...",
                    attempt, err, delay
                );

                tokio::time::sleep(Duration::from_millis(delay)).await;
                delay *= 2; // Exponential backoff
            }
        }
    }
}

async fn query_last_checkpoint_of_epoch(config: &Config, epoch_id: u64) -> Result<u64> {
    // GraphQL query to get the last checkpoint of an epoch.
    let query = json!({
        "query": "query ($epochID: Int) { epoch(id: $epochID) { checkpoints(last: 1) { nodes { sequenceNumber } } } }",
        "variables": { "epochID": epoch_id }
    });

    // Submit the query by POSTing to the GraphQL endpoint.
    let client = Client::builder()
        // Ignore SSL certificate errors (This is a temp until SUI fix their cert).
        .danger_accept_invalid_certs(true)
        .build()?;
    let resp = client
        .post(&config.graphql_url)
        .header("Content-Type", "application/json")
        .body(query.to_string())
        .send()
        .await
        .expect("Cannot connect to graphql")
        .text()
        .await
        .expect("Cannot parse response");

    // Parse the JSON response to get the last checkpoint of the epoch.
    let v: Value = serde_json::from_str(resp.as_str()).expect("Incorrect JSON response");
    let checkpoint_number = v["data"]["epoch"]["checkpoints"]["nodes"][0]["sequenceNumber"]
        .as_u64()
        .unwrap();

    Ok(checkpoint_number)
}

/// Run binary search to for each end of epoch checkpoint that is missing
/// between the latest on the list and the latest checkpoint.
async fn sync_checkpoint_list_to_latest(config: &Config) -> Result<()> {
    // Get the local checkpoint list.
    let mut checkpoints_list: CheckpointsList =
        read_checkpoint_list(config).context("Cannot read the checkpoint list file")?;
    let latest_in_list = checkpoints_list
        .checkpoints
        .last()
        .ok_or(anyhow!("Empty checkpoint list"))?;

    println!("Latest checkpoint in the list: {}", latest_in_list);
    // Download the latest in list checkpoint
    let summary = download_checkpoint_summary(config, *latest_in_list)
        .await
        .context("Failed to download checkpoint")?;
    let mut last_epoch_in_list = summary.epoch();

    // Download the very latest checkpoint.
    let sui_client = SuiClientBuilder::default()
        .build(&config.sui_full_node_url)
        .await?;

    let latest_seq = sui_client
        .read_api()
        .get_latest_checkpoint_sequence_number()
        .await?;
    println!("Latest Checkpoint Sequence in Sui network: {}", latest_seq);

    let latest_checkpoint_in_network = download_checkpoint_summary(config, latest_seq).await?;
    println!(
        "Latest Checkpoint Epoch in Sui network: {}",
        latest_checkpoint_in_network.epoch()
    );

    // Sequentially record all the missing end of epoch checkpoints numbers.
    while last_epoch_in_list + 1 < latest_checkpoint_in_network.epoch() {
        let target_epoch = last_epoch_in_list + 1;
        let target_last_checkpoint_number =
            query_last_checkpoint_of_epoch(config, target_epoch).await?;

        // Add to the list.
        checkpoints_list
            .checkpoints
            .push(target_last_checkpoint_number);
        write_checkpoint_list(config, &checkpoints_list)?;

        // Update.
        last_epoch_in_list = target_epoch;

        println!(
            "Last Epoch: {} Last Checkpoint: {}",
            target_epoch, target_last_checkpoint_number
        );
    }

    Ok(())
}

async fn check_and_sync_checkpoints(config: &Config, active_addr: SuiAddress) -> Result<()> {
    println!("Syncing checkpoints to the latest");
    sync_checkpoint_list_to_latest(config)
        .await
        .context("Failed to sync checkpoints")?;
    println!("Synced checkpoints list to latest");

    // Get the local checkpoint list.
    let checkpoints_list: CheckpointsList = read_checkpoint_list(config)?;
    println!("Checkpoints: {:?}", checkpoints_list.checkpoints);

    let genesis_committee = load_genesis_committee(config)?;

    // Retrieve the highest epoch ID that was registered on dWallet network.
    let latest_registered_epoch_in_dwallet = retrieve_highest_epoch_from_dwallet_network(config)
        .await
        .unwrap_or(0);
    println!(
        "Latest registered epoch in dwallet network: {}",
        latest_registered_epoch_in_dwallet
    );

    // Check the signatures of all checkpoints, and download any missing ones.
    let mut prev_committee = genesis_committee;
    for ckp_id in &checkpoints_list.checkpoints {
        println!("Processing checkpoint: {}", ckp_id);

        // Check if there is a file with this name ckp_id.yaml in the `checkpoint_summary_dir`.
        let mut checkpoint_path = config.checkpoint_summary_dir.clone();
        checkpoint_path.push(format!("{}.yaml", ckp_id));

        // If the file exists read the file otherwise download it from the server.
        let summary = if checkpoint_path.exists() {
            let checkpoint = read_checkpoint(config, *ckp_id)?;
            verify_checkpoint(&mut prev_committee, ckp_id, &checkpoint)?;
            checkpoint
        } else {
            // Download the checkpoint from the server.
            println!("Downloading checkpoint: {}", ckp_id);
            let summary = download_checkpoint_summary(config, *ckp_id)
                .await
                .context("Failed to download checkpoint")?;
            verify_checkpoint(&mut prev_committee, ckp_id, &summary)?;
            // Write the checkpoint summary to a file.
            write_checkpoint(config, &summary)?;
            summary
        };
        // Print the ID of the checkpoint, and the epoch number.
        println!(
            "Sequence: {} Epoch: {} Checkpoint ID: {}",
            summary.sequence_number,
            summary.epoch(),
            summary.digest()
        );

        // Check if the checkpoint needs to be submitted to the dwallet network.
        if latest_registered_epoch_in_dwallet < summary.epoch() {
            update_state_proof_registry_in_dwallet_network(config, &summary, active_addr)
                .await
                .context("Failed to update sui state proof registry in dwallet network")?;
        }

        // Extract the new committee information.
        if let Some(EndOfEpochData {
            next_epoch_committee,
            ..
        }) = &summary.end_of_epoch_data
        {
            let next_committee = next_epoch_committee.iter().cloned().collect();
            prev_committee = Committee::new(summary.epoch().saturating_add(1), next_committee);
        } else {
            return Err(anyhow!(
                "Expected all checkpoints to be end-of-epoch checkpoints"
            ));
        }
    }
    Ok(())
}

fn verify_checkpoint(
    prev_committee: &mut Committee,
    ckp_id: &u64,
    checkpoint: &Envelope<CheckpointSummary, AuthorityQuorumSignInfo<true>>,
) -> Result<()> {
    // There is a bug with try_into_verified() where if the committee is genesis committee,
    // it will take the correct epoch number (==0), but it will fail against the checkpoint
    // (==1).
    if (prev_committee.epoch == 0) && (checkpoint.epoch() == 1) {
        prev_committee.epoch = 1;
    }
    checkpoint
        .clone()
        .try_into_verified(prev_committee)
        .context("Failed to verify checkpoint")?;
    println!("Verified checkpoint: {}", ckp_id);
    Ok(())
}

async fn update_state_proof_registry_in_dwallet_network(
    config: &Config,
    checkpoint_summary: &Envelope<CheckpointSummary, AuthorityQuorumSignInfo<true>>,
    active_addr: SuiAddress,
) -> Result<()> {
    let dwallet_client = dwallet_client(config).await?;
    let pt =
        build_tx_submit_new_state_committee(config, &dwallet_client, checkpoint_summary).await?;

    let gas_price = gas_price(&dwallet_client).await?;
    let coins = fetch_or_request_coins(&dwallet_client, active_addr, GAS_BUDGET, config).await?;
    println!("Coin Balance: {}", coins.balance);

    let tx_data = TransactionData::new_programmable(
        active_addr,
        vec![coins.object_ref()],
        pt,
        GAS_BUDGET,
        gas_price,
    );

    let keystore = get_keystore()?;
    let signature = keystore
        .sign_secure(&active_addr, &tx_data, Intent::sui_transaction())
        .context("Failed to sign transaction")?;

    println!("Executing the transaction...");
    let transaction_response = dwallet_client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    assert_transaction(&transaction_response).context("submit_new_state_committee tx error")?;

    let object_changes = transaction_response
        .object_changes
        .context("Missing object changes in transaction response")?;

    for (index, object_change) in object_changes.iter().enumerate() {
        println!("Object Change {}: {}", index + 1, object_change);
    }
    for event in transaction_response.events.iter() {
        println!("Event Emitted: {}", event);
    }

    let _committee_object_change = object_changes
        .iter()
        .find(|object| matches!(object, ObjectChange::Created { object_type, .. } if object_type.to_string().contains("EpochCommittee")));

    tokio::time::sleep(Duration::from_secs(5)).await;
    Ok(())
}

/// Assert that the transaction executed successfully, exit on errors.
fn assert_transaction(transaction_response: &SuiTransactionBlockResponse) -> Result<()> {
    if let Some(true) = transaction_response.status_ok() {
        println!("Transaction executed successfully");
        return Ok(());
    }

    // Collect all error messages from `transaction_response.errors`,
    // `effects.status`, and `effects.errors`
    let mut error_messages = Vec::new();

    if !transaction_response.errors.is_empty() {
        error_messages.push(format!(
            "Transaction errors: {:?}",
            transaction_response.errors
        ));
    }

    if let Some(effects) = transaction_response.effects.as_ref() {
        if effects.status().is_err() {
            error_messages.push(format!("Effects status error: {:?}", effects.status()));
        }
    }
    Err(anyhow!(error_messages.join("; ")))
}

async fn gas_price(dwallet_client: &SuiClient) -> Result<u64> {
    dwallet_client
        .read_api()
        .get_reference_gas_price()
        .await
        .context("Failed to retrieve gas price")
}

fn get_keystore() -> Result<FileBasedKeystore> {
    let keystore_path = sui_config_dir()
        .context("Failed to get SUI config directory")?
        .join(SUI_KEYSTORE_FILENAME);
    println!("Keystore loaded, path: {:?}", keystore_path);
    FileBasedKeystore::new(&keystore_path).context("Failed to load keystore")
}

async fn build_tx_submit_new_state_committee(
    config: &Config,
    dwallet_client: &SuiClient,
    summary: &Envelope<CheckpointSummary, AuthorityQuorumSignInfo<true>>,
) -> Result<ProgrammableTransaction> {
    let mut ptb = ProgrammableTransactionBuilder::new();
    // We subtract 1 from the epoch to get the highest epoch that appeared in an event emitted
    // by the Move module.
    // The event is named: `EpochCommitteeSubmitted`,
    // and it contains the committee for the next epoch,
    // which is corresponding to the currently processed checkpoint.
    let last_epoch_number_in_event = summary.epoch().checked_sub(1).unwrap();
    let current_committee_object_id =
        retrieve_committee_id_by_target_epoch(config, last_epoch_number_in_event).await?;
    let current_committee_object_ref_dwltn =
        object_ref_by_id(config, current_committee_object_id).await?;

    let dwallet_registry_object_id =
        ObjectID::from_hex_literal(&config.dwallet_network_registry_object_id)?;

    // Retrieve the highest shared version of the registry.
    let registry_obj = dwallet_client
        .read_api()
        .get_object_with_options(
            dwallet_registry_object_id,
            SuiObjectDataOptions::full_content().with_bcs(),
        )
        .await?;
    let registry_initial_shared_version = match registry_obj
        .owner()
        .context("Failed to retrieve owner data")?
    {
        Owner::Shared {
            initial_shared_version,
        } => initial_shared_version,
        _ => return Err(anyhow!("Expected a Shared owner")),
    };

    let registry_arg = ptb.obj(ObjectArg::SharedObject {
        id: dwallet_registry_object_id,
        initial_shared_version: registry_initial_shared_version,
        mutable: true,
    })?;

    let current_committee_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(
        current_committee_object_ref_dwltn,
    ))?;
    let new_checkpoint_summary_arg = ptb.pure(bcs::to_bytes(&summary)?)?;

    let call = ProgrammableMoveCall {
        package: ObjectID::from_hex_literal(DWALLET_MODULE_ADDR)?,
        module: Identifier::new(SUI_STATE_PROOF_MODULE_IN_DWALLET_NETWORK)?,
        function: Identifier::new("submit_new_state_committee")?,
        type_arguments: vec![],
        arguments: vec![
            registry_arg,
            current_committee_arg,
            new_checkpoint_summary_arg,
        ],
    };
    ptb.command(Command::MoveCall(Box::new(call)));
    Ok(ptb.finish())
}

fn load_genesis_committee(config: &Config) -> Result<Committee> {
    let mut genesis_path = config.checkpoint_summary_dir.clone();
    genesis_path.push(&config.genesis_filename);
    Genesis::load(&genesis_path)?
        .committee()
        .context("Failed to retrieve committee from genesis file data")
}

async fn get_verified_object(config: &Config, id: ObjectID) -> Result<Object> {
    let sui_client: Arc<SuiClient> = Arc::new(
        SuiClientBuilder::default()
            .build(config.sui_full_node_url.as_str())
            .await?,
    );

    println!("Getting object: {}", id);

    let read_api = sui_client.read_api();
    let object_json = read_api
        .get_object_with_options(id, SuiObjectDataOptions::bcs_lossless())
        .await
        .expect("cannot get object");
    let object = object_json
        .into_object()
        .expect("cannot make into object data");
    let object: Object = object.try_into().expect("Cannot reconstruct object");
    Ok(object)
}

async fn retrieve_highest_epoch_from_dwallet_network(config: &Config) -> Result<u64> {
    let dwallet_client = SuiClientBuilder::default()
        .build(&config.dwallet_full_node_url)
        .await
        .context("Failed to build dwallet client")?;

    let query = sui_state_proof_event_filter()?;

    let events = dwallet_client
        .event_api()
        .query_events(query, None, None, true)
        .await
        .context("Failed to query events from dwallet network")?;

    let highest_epoch = events
        .data
        .iter()
        .filter_map(|event| {
            let registry_id = event.parsed_json.get("registry_id")?.as_str()?;
            if registry_id == config.dwallet_network_registry_object_id {
                let epoch_str = event.parsed_json.get("epoch")?.as_str()?;
                u64::from_str(epoch_str).ok()
            } else {
                None
            }
        })
        .max()
        .context("No valid epoch events found")?;

    Ok(highest_epoch)
}

fn sui_state_proof_event_filter() -> Result<EventFilter> {
    Ok(EventFilter::MoveModule {
        package: ObjectID::from_hex_literal(DWALLET_MODULE_ADDR)
            .context("Failed to parse DWALLET_MODULE_ADDR as ObjectID")?,
        module: Identifier::from_str(SUI_STATE_PROOF_MODULE_IN_DWALLET_NETWORK)
            .context("Failed to parse SUI_STATE_PROOF_MODULE_IN_DWALLET_NETWORK as Identifier")?,
    })
}

/// This code is fetching the event emitted by `init_module`
///     struct EpochCommitteeSubmitted has copy, drop {
///         epoch: u64,
///         registry_id: ID,
///         epoch_committee_id: ID, -> This should be renamed to next_committee_id
///     }
async fn retrieve_committee_id_by_target_epoch(
    config: &Config,
    target_epoch: u64,
) -> Result<ObjectID> {
    let dwallet_client = dwallet_client(config).await?;
    let query = sui_state_proof_event_filter()?;

    let mut has_next = true;
    let mut cursor = None;

    while has_next {
        let events = dwallet_client
            .event_api()
            .query_events(query.clone(), cursor, None, true)
            .await
            .context("Failed to query events")?;

        if let Some(epoch_committee_id) = events.data.iter().find_map(|event| {
            // Check for registry_id and match it with the config.
            let registry_id = event.parsed_json.get("registry_id")?.as_str()?;
            if registry_id != config.dwallet_network_registry_object_id {
                return None;
            }
            println!("Previous Event: {}", event);
            // Get the event for the previous processed epoch,
            // since it contains the committee for currently processed checkpoint.
            event
                .parsed_json
                .get("epoch")
                .and_then(|epoch| epoch.as_str())
                .and_then(|epoch_str| u64::from_str(epoch_str).ok())
                .filter(|&epoch| epoch == target_epoch)
                // NOTE:
                // epoch_committee_id actually the next committee ID
                // (corresponding to the currently processed checkpoint).
                .and_then(|_| event.parsed_json.get("epoch_committee_id"))
                .and_then(|id| id.as_str())
        }) {
            return ObjectID::from_hex_literal(epoch_committee_id)
                .map_err(|e| anyhow!(format!("failed get epoch committee ID: {}", e)));
        }
        cursor = events.next_cursor;
        has_next = events.has_next_page;
    }
    Err(anyhow!("Epoch {} not found", target_epoch))
}

async fn object_ref_by_id(config: &Config, object_id: ObjectID) -> Result<ObjectRef> {
    let dwallet_client = dwallet_client(config).await?;

    let obj = dwallet_client
        .read_api()
        .get_object_with_options(object_id, SuiObjectDataOptions::full_content().with_bcs())
        .await
        .context(format!("Failed to retrieve object with ID: {}", object_id))?;

    let object_ref = obj
        .data
        .context("Missing object data in the response")?
        .object_ref();

    Ok(object_ref)
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // Command line arguments and config loading.
    let args = Args::parse();

    let path = args.config.context("Config file path is required")?;
    let reader = fs::File::open(&path)
        .with_context(|| format!("Unable to load config from {}", path.display()))?;
    let mut config: Config =
        serde_yaml::from_reader(reader).context("Failed to parse configuration file")?;

    println!("Config: {:?}", config);
    println!(
        "Checkpoint Dir: {}",
        config.checkpoint_summary_dir.display()
    );

    // Initialize pacakge resolver and client.
    let remote_package_store = RemotePackageStore::new(config.clone());
    let resolver = Resolver::new(remote_package_store);

    let (dwallet_client, coins, active_addr) = utils::setup_for_write(&config, GAS_BUDGET)
        .await
        .context("Wallet setup failed")?;

    match args.command {
        Some(SCommands::Init { ckp_id }) => {
            init_light_client(
                &mut config,
                resolver,
                dwallet_client,
                ckp_id,
                coins,
                active_addr,
            )
            .await
            .context("Init Command Failed")?;
        }
        Some(SCommands::Sync {}) => {
            check_and_sync_checkpoints(&config, active_addr)
                .await
                .context("Sync Command Failed")?;
        }
        _ => {
            panic!("No valid command provided");
        }
    }

    // Write updated config back to file.
    let file = fs::File::create(&path)
        .with_context(|| format!("Unable to open config file for writing: {}", path.display()))?;
    serde_yaml::to_writer(file, &config)
        .with_context(|| format!("Failed to write config to file: {}", path.display()))?;

    Ok(())
}

async fn dwallet_client(config: &Config) -> Result<SuiClient> {
    SuiClientBuilder::default()
        .build(&config.dwallet_full_node_url)
        .await
        .context("Failed to create dwallet client")
}

async fn init_light_client(
    config: &mut Config,
    resolver: Resolver<RemotePackageStore>,
    dwallet_client: SuiClient,
    ckp_id: u64,
    coins: Coin,
    active_addr: SuiAddress,
) -> Result<()> {
    let (next_committee, current_epoch) = init_by_checkpoint_id(&config, ckp_id).await?;
    println!(
        "[Init] — next epoch from committee: {}, current_epoch: {}",
        next_committee.epoch, current_epoch
    );

    let pt = build_init_module_tx(&config, resolver, &next_committee, current_epoch)
        .await
        .context("init_module transaction failed")?;

    let keystore = get_keystore()?;

    let tx_data = TransactionData::new_programmable(
        active_addr,
        vec![coins.object_ref()],
        pt,
        GAS_BUDGET,
        gas_price(&dwallet_client).await?,
    );

    let signature = keystore
        .sign_secure(&active_addr, &tx_data, Intent::sui_transaction())
        .context("Failed to sign transaction")?;

    println!("Executing the transaction...");
    let transaction_response = dwallet_client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await
        .context("Failed to execute `init_module` transaction")?;

    assert_transaction(&transaction_response).context("submit_new_state_committee tx error")?;

    println!(
        "Transaction executed with {} object changes:",
        transaction_response
            .object_changes
            .as_ref()
            .map(|oc| oc.len())
            .unwrap_or(0)
    );

    transaction_response.events.iter().for_each(|event| {
        println!("Event Emitted: {}", event);
    });

    let object_changes = transaction_response
        .object_changes
        .context("Missing object changes in transaction response")?;

    for (index, object_change) in object_changes.iter().enumerate() {
        println!("Object Change {}: {}", index + 1, object_change);
    }

    let registry_object_change = object_changes
        .iter()
        .find(|object| matches!(object, ObjectChange::Created { object_type, .. } if object_type.to_string().contains("Registry")))
        .context("Registry object not found in transaction response")?;

    let config_object_change = object_changes
        .iter()
        .find(|object| matches!(object, ObjectChange::Created { object_type, .. } if object_type.to_string().contains("StateProofConfig")))
        .context("Config object not found in transaction response")?;

    // Update YAML config.
    config.dwallet_network_config_object_id = config_object_change.object_ref().0.to_string();
    config.dwallet_network_registry_object_id = registry_object_change.object_ref().0.to_string();
    Ok(())
}

async fn build_init_module_tx(
    config: &&mut Config,
    resolver: Resolver<RemotePackageStore>,
    next_committee: &Committee,
    current_epoch_id: EpochId,
) -> Result<ProgrammableTransaction> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    let next_committee_bytes =
        bcs::to_bytes(&next_committee).context("Failed to serialize genesis committee")?;
    let init_committee_arg = ptb
        .pure(next_committee_bytes)
        .context("Failed to create init_committee_arg")?;

    let sui_package_id = ObjectID::from_hex_literal(&config.sui_deployed_state_proof_package)
        .context("Failed to parse `sui_deployed_state_proof_package` as ObjectID")?;
    let sui_package_id_bytes =
        bcs::to_bytes(&sui_package_id).context("Failed to serialize sui_package_id")?;
    let sui_package_id_arg = ptb
        .pure(sui_package_id_bytes)
        .context("Failed to create sui_package_id_arg")?;

    let init_tag = StructTag {
        address: AccountAddress::from_hex_literal(&config.sui_deployed_state_proof_package)
            .context("Failed to parse sui_deployed_state_proof_package as AccountAddress")?,
        module: Identifier::new(SUI_DWALLET_MODULE_NAME)
            .context("Failed to create Identifier for module dwallet_cap")?,
        name: Identifier::new("DWalletNetworkInitCapRequest")
            .context("Failed to create Identifier for DWalletNetworkInitCapRequest")?,
        type_params: vec![],
    };
    println!("Init Tag: {}", init_tag);

    let init_type_layout = resolver
        .type_layout(TypeTag::Struct(Box::new(init_tag)))
        .await
        .context("Failed to resolve init_type_layout")?;
    let init_type_layout_bytes =
        bcs::to_bytes(&init_type_layout).context("Failed to serialize init_type_layout")?;
    let init_event_type_layout_arg = ptb
        .pure(init_type_layout_bytes)
        .context("Failed to create init_event_type_layout_arg")?;

    let approve_tag = StructTag {
        address: AccountAddress::from_hex_literal(&config.sui_deployed_state_proof_package)
            .context("Failed to parse sui_deployed_state_proof_package for approve_tag")?,
        module: Identifier::new(SUI_DWALLET_MODULE_NAME)
            .context("Failed to create Identifier for module dwallet_cap")?,
        name: Identifier::new("DWalletNetworkApproveRequest")
            .context("Failed to create Identifier for DWalletNetworkApproveRequest")?,
        type_params: vec![],
    };

    let approve_type_layout = resolver
        .type_layout(TypeTag::Struct(Box::new(approve_tag)))
        .await
        .context("Failed to resolve `approve_type_layout`")?;
    let approve_type_layout_bytes =
        bcs::to_bytes(&approve_type_layout).context("Failed to serialize `approve_type_layout`")?;
    let approve_event_type_layout_arg = ptb
        .pure(approve_type_layout_bytes)
        .context("Failed to create `approve_event_type_layout_arg`")?;

    let epoch_id_committee_arg = ptb
        .pure(current_epoch_id)
        .context("Failed to create `epoch_id_committee_arg`")?;

    let init_module_call = ProgrammableMoveCall {
        package: ObjectID::from_hex_literal(DWALLET_MODULE_ADDR)
            .context("Failed to parse DWALLET_MODULE_ADDR as ObjectID")?,
        module: Identifier::new(SUI_STATE_PROOF_MODULE_IN_DWALLET_NETWORK).context(format!(
            "Failed to create Identifier for module: {SUI_STATE_PROOF_MODULE_IN_DWALLET_NETWORK}"
        ))?,
        function: Identifier::new("init_module")
            .context("Failed to create Identifier for function init_module")?,
        type_arguments: vec![],
        arguments: vec![
            init_committee_arg,
            sui_package_id_arg,
            init_event_type_layout_arg,
            approve_event_type_layout_arg,
            epoch_id_committee_arg,
        ],
    };

    ptb.command(Command::MoveCall(Box::new(init_module_call)));
    Ok(ptb.finish())
}

async fn init_by_checkpoint_id(
    config: &&mut Config,
    checkpoint_id: u64,
) -> Result<(Committee, EpochId)> {
    if checkpoint_id == 0 {
        // Load the genesis committee.
        let genesis_committee = load_genesis_committee(config)?;
        return Ok((genesis_committee, 0));
    }
    let checkpoint_summary = download_checkpoint_summary(config, checkpoint_id)
        .await
        .context("Failed to download checkpoint summary")?;

    let end_of_epoch_data = checkpoint_summary.end_of_epoch_data.as_ref().context(
        "Missing end_of_epoch_data in checkpoint summary, must provide end of epoch checkpoint ID",
    )?;

    let next_epoch_committee = end_of_epoch_data
        .next_epoch_committee
        .iter()
        .cloned()
        .collect();
    let current_epoch = checkpoint_summary.epoch();

    let next_committee = Committee::new(current_epoch + 1, next_epoch_committee);
    Ok((next_committee, current_epoch))
}
