//! Ethereum Client Commands Module
//!
//! This module provides commands and functions to interact with an Ethereum-based decentralized wallet
//! (dWallet) in a Sui blockchain environment.
//! The primary functionalities include verifying Ethereum transactions,
//! connecting dWallets to Ethereum smart contracts, and initializing Ethereum state.

use anyhow::{anyhow, Error};
use clap::Subcommand;
use serde_json::{Number, Value};
use helios::dwallet::eth_state::EthState;
use helios::dwallet::light_client::{EthLightClient, EthLightClientConfig, ProofRequestParameters};
use helios::prelude::networks::Network;
use helios::types::Address;
use serde::Serialize;
use shared_crypto::intent::Intent;
use sui_json::SuiJsonValue;
use sui_json_rpc_types::{ObjectChange, SuiData, SuiExecutionStatus, SuiObjectDataOptions, SuiRawData, SuiTransactionBlockEffectsAPI, SuiTransactionBlockResponse}; // todo!(ide keeps deleting SuiTransactionBlockEffectsAPI)
use sui_keys::keystore::AccountKeystore;
use sui_sdk::sui_client_config::SuiEnv;
use sui_sdk::wallet_context::WalletContext;

use sui_types::base_types::ObjectID;
use sui_types::eth_dwallet::{APPROVE_MESSAGE_FUNC_NAME, CREATE_ETH_DWALLET_CAP_FUNC_NAME, ETH_DWALLET_MODULE_NAME, EthDWalletCap, ETHEREUM_STATE_MODULE_NAME, EthStateObject, INIT_STATE_FUNC_NAME, LATEST_ETH_STATE_STRUCT_NAME, LatestEthStateObject, VERIFY_ETH_STATE_FUNC_NAME};
use sui_types::object::Owner;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::SUI_SYSTEM_PACKAGE_ID;
use sui_types::transaction::{ObjectArg, SenderSignedData, SharedInputObject, Transaction, TransactionDataAPI};

use crate::client_commands::{construct_move_call_transaction, SuiClientCommandResult};
use crate::serialize_or_execute;

#[derive(Subcommand)]
pub enum EthClientCommands {
    /// Approve a TX with Eth contract for a given dWallet.
    #[command(name = "dwallet-eth-verify")]
    EthApproveMessage {
        #[clap(long)]
        /// Object of a [EthDwalletCap].
        eth_dwallet_cap_id: ObjectID,
        /// The Message (TX) to verify.
        #[clap(long)]
        message: String,
        /// The DWallet ID.
        // todo(zeev): in the future we can fetch the dwallet ID.
        #[clap(long)]
        dwallet_id: ObjectID,
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
        #[clap(long)]
        gas_budget: u64,
        /// Instead of executing the transaction, serialize the bcs bytes of the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,
        /// Instead of executing the transaction, serialize the bcs bytes of the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,
    },

    /// Connect dWallet to be controlled by Eth contract.
    #[command(name = "dwallet-connect-eth")]
    CreateEthDwallet {
        /// The ObjectID of the dWallet *cap*ability.
        #[clap(long)]
        dwallet_cap_id: ObjectID,
        /// The address of the contract.
        #[clap(long)]
        smart_contract_address: String,
        /// The slot of the Data structure that holds approved transactions in eth smart contract.
        #[clap(long)]
        // todo(zeev): change the clap name to something shorter.
        smart_contract_approved_tx_slot: u64,
        /// The address of the gas object for gas payment.
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
        #[clap(long)]
        gas_budget: u64,
        /// Instead of executing the transaction, serialize the bcs bytes of the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,
        /// Instead of executing the transaction, serialize the bcs bytes of the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,
    },

    /// Initiate the LatestEthereumState struct in the DWallet module.
    #[command(name = "init-eth-state")]
    InitEthState {
        #[clap(long)]
        checkpoint: String,
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call.
        #[clap(long)]
        gas_budget: u64,
        /// Instead of executing the transaction, serialize the bcs bytes of the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,
        /// Instead of executing the transaction, serialize the bcs bytes of the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,
    },
}

/// Connects a dWallet to be controlled by an Ethereum smart contract.
pub(crate) async fn create_eth_dwallet(
    context: &mut WalletContext,
    dwallet_cap_id: ObjectID,
    smart_contract_address: String,
    smart_contract_approved_tx_slot: u64,
    gas: Option<ObjectID>,
    gas_budget: u64,
    serialize_unsigned_transaction: bool,
    serialize_signed_transaction: bool,
) -> Result<SuiClientCommandResult, Error> {
    // Serialize to the Move TX format.
    let smart_contract_address = bcs::to_bytes(&smart_contract_address).unwrap();
    let mut smart_contract_address: Vec<Value> = smart_contract_address
        .iter()
        .map(|v| Value::Number(Number::from(*v)))
        .collect();
    smart_contract_address.remove(0);
    let smart_contract_address = SuiJsonValue::new(Value::Array(smart_contract_address)).unwrap();

    let args = vec![
        SuiJsonValue::from_object_id(dwallet_cap_id),
        smart_contract_address,
        SuiJsonValue::new(Value::Number(Number::from(smart_contract_approved_tx_slot))).unwrap(),
    ];

    let tx_data = construct_move_call_transaction(
        SUI_SYSTEM_PACKAGE_ID,
        ETH_DWALLET_MODULE_NAME.as_str(),
        CREATE_ETH_DWALLET_CAP_FUNC_NAME.as_str(),
        vec![],
        gas,
        gas_budget,
        args,
        context,
    )
        .await?;
    Ok(serialize_or_execute!(
        tx_data,
        serialize_unsigned_transaction,
        serialize_signed_transaction,
        context,
        Call
    ))
}

/// Initializes a shared LatestEthereumState object in the dWallet network
/// with the given checkpoint.
/// This function should only be called once to initialize the Ethereum state.
/// After the state is initialized, the Ethereum state object ID is saved in the configuration,
/// and the state is updated whenever a new state is successfully verified.
// todo(yuval): in future, we should load also the sync committee from binary data file.
pub(crate) async fn init_ethereum_state(
    checkpoint: String,
    context: &mut WalletContext,
    gas: Option<ObjectID>,
    gas_budget: u64,
    serialize_unsigned_transaction: bool,
    serialize_signed_transaction: bool,
) -> Result<SuiClientCommandResult, Error> {
    let args = vec![SuiJsonValue::new(Value::String(checkpoint))?];

    let tx_data = construct_move_call_transaction(
        SUI_SYSTEM_PACKAGE_ID,
        ETHEREUM_STATE_MODULE_NAME.as_str(),
        INIT_STATE_FUNC_NAME.as_str(),
        vec![],
        gas,
        gas_budget,
        args,
        context,
    )
        .await?;

    let latest_state = serialize_or_execute!(
        tx_data,
        serialize_unsigned_transaction,
        serialize_signed_transaction,
        context,
        Call
    );

    let SuiClientCommandResult::Call(state) = latest_state else {
        return Err(anyhow!(
            "Can't get response."
        ));
    };

    let latest_state_object_id = state.object_changes.clone().unwrap().iter().find_map(|oc| {
        if let ObjectChange::Created {
            object_id,
            object_type,
            ..
        } = oc {
            if object_type.module == ETHEREUM_STATE_MODULE_NAME.into() && object_type.name == LATEST_ETH_STATE_STRUCT_NAME.into() {
                return Some(object_id);
            }
        }
        None
    }).unwrap().clone();

    context.config.update_ethereum_state_object_id(latest_state_object_id);
    context.config.save()?;

    Ok(SuiClientCommandResult::Call(state))
}

/// Approves an Ethereum transaction for a given dWallet.
///
/// Interacts with the Ethereum light client to verify and approve a transaction message
/// using an Ethereum smart contract linked to a dWallet within the dWallet blockchain context.
/// The verification of the state & message is done offline, inside the dWallet module.
/// # Logic
/// 1. **Retrieve Configuration**: It starts by retrieving the Ethereum state object ID from
///      the active environment.
/// 2. **Fetch Ethereum Objects**: It retrieves and deserializes the `EthDWalletCap`,
///      `LatestEthStateObject`, and `EthStateObject` to collect the latest Ethereum state data.
/// 3. **Initialize Light Client**: Initializes the Ethereum light client with the deserialized
///      Ethereum state.
/// 4. **Prepare Proof Parameters**: Constructs proof request parameters using the message,
///      dWallet ID, and data slot obtained from the `EthDWalletCap`.
/// 5. **Fetch Updates and Proofs**: Retrieves the necessary updates and cryptographic proofs from
///      the Ethereum light client.
/// 6. **Build Transaction**: Uses the Sui programmable transaction builder to serialize transaction
///      parameters, including the Ethereum state, updates, and shared state object,
///      and prepares the transaction to call the `VERIFY_ETH_STATE_FUNC_NAME` function in the
///      Ethereum state module.
/// 7. **Send Transaction**: Constructs the transaction data, including the proof and dWallet ID,
///      and executes or serializes it based on the provided flags.
/// # Arguments
/// * `eth_dwallet_cap_id` - The `ObjectID` of the Ethereum dWallet capability,
///     representing the link between the dWallet and Ethereum.
/// * `message` - The Ethereum transaction message to be approved.
/// * `dwallet_id` - The `ObjectID` of the dWallet to which the transaction belongs.
/// # Returns
/// * `Result<SuiClientCommandResult, Error>` -
/// Result containing either the transaction execution result or an error.
/// # Errors
/// The function returns an error if any of the following occur:
/// - Missing Ethereum state configuration.
/// - Errors during object deserialization.
/// - Failure in fetching updates or proofs from the Ethereum light client.
/// - Transaction construction or execution failures.
pub(crate) async fn eth_approve_message(
    context: &mut WalletContext,
    eth_dwallet_cap_id: ObjectID,
    message: String,
    dwallet_id: ObjectID,
    gas: Option<ObjectID>,
    gas_budget: u64,
    serialize_unsigned_transaction: bool,
    serialize_signed_transaction: bool,
) -> Result<SuiClientCommandResult, Error> {
    let active_env = context
        .config
        .get_active_env()?
        .clone();

    let latest_state_object_id = match active_env
        .eth_client_settings
    {
        Some(eth_client_settings) => eth_client_settings
            .state_object_id
            .ok_or_else(|| anyhow!("ETH State object ID configuration not found"))?,
        None => return Err(anyhow!("ETH State object ID configuration not found"))
    };

    let eth_dwallet_cap_data_bcs = get_object_bcs_by_id(context, eth_dwallet_cap_id).await?;
    let eth_dwallet_cap_obj: EthDWalletCap = eth_dwallet_cap_data_bcs
        .try_as_move()
        .ok_or_else(|| anyhow!("Object is not a Move Object"))?
        .deserialize()
        .map_err(|e| anyhow!("Error deserializing object: {e}"))?;

    let latest_eth_state_data_bcs = get_object_bcs_by_id(context, latest_state_object_id).await?;
    let latest_eth_state_obj: LatestEthStateObject = latest_eth_state_data_bcs
        .try_as_move()
        .ok_or_else(|| anyhow!("Object is not a Move Object"))?
        .deserialize()
        .map_err(|e| anyhow!("Error deserializing object: {e}"))?;

    let eth_state_object_id = latest_eth_state_obj.eth_state_id;
    let eth_state_data_bcs = get_object_bcs_by_id(context, eth_state_object_id).await?;
    let eth_state_obj: EthStateObject = eth_state_data_bcs
        .try_as_move()
        .ok_or_else(|| anyhow!("Object is not a Move Object"))?
        .deserialize()
        .map_err(|e| anyhow!("Error deserializing object: {e}"))?;

    let latest_eth_state_shared_object =
        get_shared_object_input_by_id(context, latest_state_object_id).await?;

    let mut eth_state = bcs::from_bytes::<EthState>(&eth_state_obj.data)
        .map_err(|e| anyhow!("error parsing eth state data: {e}"))?;

    let (data_slot, contract_addr) = get_data_from_eth_dwallet_cap(eth_dwallet_cap_obj)?;

    let eth_lc_config = get_eth_config(context)?;

    // We need to set the network in the eth_state object for the state verification process.
    let eth_state = eth_state
        .set_network(eth_lc_config.network.clone());

    let mut eth_lc = EthLightClient::init_new_light_client(eth_lc_config.clone(), eth_state.clone()).await?;

    let proof_params = ProofRequestParameters {
        message: message.clone(),
        dwallet_id: dwallet_id.as_slice().to_vec(),
        data_slot,
    };

    let updates = eth_lc.get_updates()
        .await
        .map_err(|e| anyhow!("Could not fetch updates: {e}"))?;

    let proof = eth_lc.get_proofs(&contract_addr, proof_params)
        .await
        .map_err(|e| anyhow!("Could not fetch proof: {e}"))?;

    let gas_owner = context.try_get_object_owner(&gas).await?;
    let sender = gas_owner.unwrap_or(context.active_address()?);

    // Serialize Move parameters
    let mut pt_builder = ProgrammableTransactionBuilder::new();
    let eth_state_arg = pt_builder
        .pure(bcs::to_bytes(&eth_state)?)
        .map_err(|e| anyhow!("Could not serialize eth_state. {e}"))?;

    let updates_arg = pt_builder
        .pure(bcs::to_bytes(&updates)?)
        .map_err(|e| anyhow!("Could not serialize updates. {e}"))?;

    let latest_eth_state_shared_object_arg = ObjectArg::SharedObject {
        id: latest_eth_state_shared_object.id,
        initial_shared_version: latest_eth_state_shared_object.initial_shared_version,
        mutable: true,
    };

    let latest_eth_state_arg = pt_builder
        .obj(latest_eth_state_shared_object_arg)
        .map_err(|e| anyhow!("Could not serialize latest_eth_state_id. {e}"))?;

    pt_builder.programmable_move_call(
        SUI_SYSTEM_PACKAGE_ID,
        ETHEREUM_STATE_MODULE_NAME.into(),
        VERIFY_ETH_STATE_FUNC_NAME.into(),
        vec![],
        Vec::from([updates_arg, eth_state_arg, latest_eth_state_arg]),
    );

    let proof_sui_json =
        serialize_object(&proof).map_err(|e| anyhow!("Could not serialize proof. {e}"))?;

    let client = context.get_client().await?;

    client
        .transaction_builder()
        .single_move_call(
            &mut pt_builder,
            SUI_SYSTEM_PACKAGE_ID,
            ETH_DWALLET_MODULE_NAME.as_str(),
            APPROVE_MESSAGE_FUNC_NAME.as_str(),
            Vec::new(),
            Vec::from([
                SuiJsonValue::from_object_id(eth_dwallet_cap_id),
                SuiJsonValue::from_object_id(dwallet_id),
                proof_sui_json,
            ]),
        )
        .await?;

    let tx_data = client
        .transaction_builder()
        .finish_programmable_transaction(sender, pt_builder, gas, gas_budget)
        .await?;

    let session_response = serialize_or_execute!(
        tx_data,
        serialize_unsigned_transaction,
        serialize_signed_transaction,
        context,
        Call
    );
    Ok(session_response)
}

fn serialize_object<T>(object: &T) -> Result<SuiJsonValue, Error>
where
    T: ?Sized + Serialize,
{
    let object_bytes = bcs::to_bytes(&object)?;
    let object_json = object_bytes
        .iter()
        .map(|v| Value::Number(Number::from(*v)))
        .collect();
    Ok(SuiJsonValue::new(Value::Array(object_json))?)
}

async fn get_object_bcs_by_id(
    context: &mut WalletContext,
    object_id: ObjectID,
) -> Result<SuiRawData, Error> {
    let object_resp = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(
            object_id,
            SuiObjectDataOptions::default().with_bcs().with_owner(),
        )
        .await?;

    match object_resp.data {
        Some(data) => Ok(
            data.bcs.ok_or_else(|| anyhow!("missing object data"))?,
        ),
        None => Err(anyhow!("Could not find object with ID: {:?}", object_id)),
    }
}

async fn get_shared_object_input_by_id(
    context: &mut WalletContext,
    object_id: ObjectID,
) -> Result<SharedInputObject, Error> {
    let object_resp = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(object_id, SuiObjectDataOptions::default().with_owner())
        .await?;

    match object_resp.data {
        Some(_) => {
            let owner = object_resp
                .owner()
                .ok_or_else(|| anyhow!("missing object owner"))?;
            let initial_shared_version = match owner {
                Owner::Shared {
                    initial_shared_version,
                } => initial_shared_version,
                _ => return Err(anyhow!("Object is not shared")),
            };
            Ok(SharedInputObject {
                id: object_id,
                initial_shared_version,
                mutable: true,
            })
        }
        None => Err(anyhow!("Could not find object with ID: {:?}", object_id)),
    }
}

fn get_data_from_eth_dwallet_cap(
    eth_dwallet_cap_obj: EthDWalletCap,
) -> Result<(u64, Address), Error> {
    let data_slot = eth_dwallet_cap_obj.eth_smart_contract_slot;
    let contract_addr: String = eth_dwallet_cap_obj.eth_smart_contract_addr;
    let contract_addr = contract_addr.clone().parse::<Address>()?;
    Ok((data_slot, contract_addr))
}

fn get_eth_config(context: &mut WalletContext) -> Result<EthLightClientConfig, Error> {
    let mut eth_lc_config = EthLightClientConfig::default();
    let sui_env_config = context.config.get_active_env()?;

    let eth_client_settings = sui_env_config.eth_client_settings.clone().ok_or_else(|| {
        anyhow!("ETH client settings configuration not found")
    })?;

    let eth_execution_rpc = eth_client_settings
        .eth_execution_rpc
        .clone()
        .ok_or_else(|| anyhow!("ETH execution RPC configuration not found"))?;
    let eth_consensus_rpc = eth_client_settings
        .eth_consensus_rpc
        .clone()
        .ok_or_else(|| anyhow!("ETH consensus RPC configuration not found"))?;

    eth_lc_config.network = get_network_by_sui_env(sui_env_config)?;
    eth_lc_config.execution_rpc = eth_execution_rpc;
    eth_lc_config.consensus_rpc = eth_consensus_rpc;

    Ok(eth_lc_config)
}

fn get_network_by_sui_env(sui_env_config: &SuiEnv) -> Result<Network, Error> {
    let network = match sui_env_config.alias.as_str() {
        "mainnet" => Network::MAINNET,
        "testnet" => Network::HOLESKY,
        "localnet" => get_eth_devnet_network_config(sui_env_config)?,
        _ => Network::MAINNET,
    };
    Ok(network)
}

fn get_eth_devnet_network_config(sui_env_config: &SuiEnv) -> Result<Network, Error> {
    let eth_client_settings = sui_env_config.eth_client_settings.clone().ok_or_else(|| {
        anyhow!("ETH client settings configuration not found")
    })?;

    let path_to_config = eth_client_settings
        .eth_devnet_network_config_filename
        .clone()
        .ok_or_else(|| anyhow!("eth_devnet_network_config_filename not found in sui configuration."))?;

    Ok(Network::DEVNET(path_to_config))
}
