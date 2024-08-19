//! Ethereum Client Commands Module
//!
//! This module provides commands and functions to interact with an
//! Ethereum-based dWallet smart contracts in dWallet blockchain environment.
//! The primary functionalities include verifying Ethereum transactions,
//! connecting dWallets to Ethereum smart contracts, and initializing Ethereum state.
use anyhow::{anyhow, Result};
use clap::Subcommand;
use helios::consensus::nimbus_rpc::NimbusRpc;
use helios::consensus::ConsensusStateManager;
use helios::dwallet::light_client::{
    EthLightClientConfig, EthLightClientWrapper, ProofRequestParameters,
};
use helios::prelude::networks::Network;
use helios::prelude::Address;
use hex::encode;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use light_client_helpers::{
    get_object_bcs_by_id, get_object_from_transaction_changes, get_shared_object_input_by_id,
};
use shared_crypto::intent::Intent;
use sui_json::SuiJsonValue;
use sui_json_rpc_types::SuiData;
#[allow(unused_imports)]
// SuiTransactionBlockEffectsAPI is called in a macro; therefore, the IDE doesn't recognize it.
use sui_json_rpc_types::{ObjectChange, SuiExecutionStatus, SuiTransactionBlockEffectsAPI};
use sui_keys::keystore::AccountKeystore;
use sui_sdk::sui_client_config::SuiEnv;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::eth_dwallet::{
    EthereumDWalletCap, EthereumStateObject, LatestEthereumStateObject, APPROVE_MESSAGE_FUNC_NAME,
    CREATE_ETH_DWALLET_CAP_FUNC_NAME, ETHEREUM_STATE_MODULE_NAME, ETH_DWALLET_MODULE_NAME,
    ETH_STATE_STRUCT_NAME, INIT_STATE_FUNC_NAME, LATEST_ETH_STATE_STRUCT_NAME,
    VERIFY_ETH_STATE_FUNC_NAME,
};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{ObjectArg, SenderSignedData, Transaction, TransactionDataAPI};
use sui_types::SUI_SYSTEM_PACKAGE_ID;

use crate::client_commands::{construct_move_call_transaction, SuiClientCommandResult};
use crate::serialize_or_execute;

#[derive(Subcommand)]
pub enum EthClientCommands {
    /// Approve a TX with Eth contract for a given dWallet.
    #[command(name = "verify-message")]
    EthApproveMessage {
        /// Object of a [EthereumDWalletCap].
        #[clap(long)]
        eth_dwallet_cap_id: ObjectID,
        /// The Message (TX) to verify.
        #[clap(long)]
        message: String,
        /// The DWallet ID.
        #[clap(long)]
        dwallet_id: ObjectID,
        /// Gas object for gas payment.
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call.
        #[clap(long)]
        gas_budget: u64,
        /// Instead of executing the transaction, serialize the bcs bytes of
        /// the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,
        /// Instead of executing the transaction, serialize the bcs bytes of
        /// the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,
    },

    /// Connect dWallet to be controlled by Eth contract.
    #[command(name = "connect-eth-dwallet")]
    CreateEthDwallet {
        /// The ObjectID of the dWallet *cap*ability.
        #[clap(long)]
        dwallet_cap_id: ObjectID,
        /// The address of the gas object for gas payment.
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
        #[clap(long)]
        gas_budget: u64,
        /// Instead of executing the transaction,
        /// serialize the bcs bytes of the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,
        /// Instead of executing the transaction,
        /// serialize the bcs bytes of the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,
    },

    /// Initiate the LatestEthereumState struct in the DWallet module.
    #[command(name = "init-eth-state")]
    InitEthState {
        /// A Trusted checkpoint to initialize the state.
        #[clap(long)]
        checkpoint: String,
        /// The corresponding Ethereum network.
        #[clap(long)]
        network: String,
        /// The address of the contract.
        #[clap(long)]
        contract_address: String,
        /// The slot of the Data structure that holds approved transactions in eth smart contract.
        #[clap(long)]
        contract_approved_tx_slot: u64,
        /// The address of the gas object for gas payment.
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call.
        #[clap(long)]
        gas_budget: u64,
        /// Instead of executing the transaction,
        /// serialize the bcs bytes of the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,
        /// Instead of executing the transaction,
        /// serialize the bcs bytes of the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,
    },
}

/// Initializes a shared LatestEthereumState object in the dWallet network
/// with the given checkpoint.
/// This function should only be called once to initialize the Ethereum state.
/// After the state is initialized, the Ethereum state object ID is saved in the configuration,
/// and the state is updated whenever a new state is successfully verified.
pub(crate) async fn init_ethereum_state(
    checkpoint: String,
    network: String,
    contract_address: String,
    contract_slot: u64,
    context: &mut WalletContext,
    gas: Option<ObjectID>,
    gas_budget: u64,
    serialize_unsigned_transaction: bool,
    serialize_signed_transaction: bool,
) -> Result<SuiClientCommandResult> {
    let args = vec![
        SuiJsonValue::new(Value::String(checkpoint))?,
        SuiJsonValue::new(Value::String(network.to_string()))?,
        SuiJsonValue::new(Value::String(contract_address))?,
        SuiJsonValue::new(Value::Number(Number::from(contract_slot)))?,
    ];

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
        return Err(anyhow!("can't get response."));
    };

    let object_changes = state
        .object_changes
        .clone()
        .ok_or_else(|| anyhow!("can't get object changes."))?;

    let latest_state_object_id = get_object_from_transaction_changes(
        object_changes,
        ETHEREUM_STATE_MODULE_NAME.into(),
        LATEST_ETH_STATE_STRUCT_NAME.into(),
    )?;

    context
        .config
        .update_ethereum_state_object_id(latest_state_object_id.clone())?;
    context.config.save()?;

    Ok(SuiClientCommandResult::Call(state))
}

/// Connects a dWallet to be controlled by an Ethereum smart contract.
pub(crate) async fn create_eth_dwallet(
    context: &mut WalletContext,
    dwallet_cap_id: ObjectID,
    gas: Option<ObjectID>,
    gas_budget: u64,
    serialize_unsigned_transaction: bool,
    serialize_signed_transaction: bool,
) -> Result<SuiClientCommandResult> {
    let args = vec![SuiJsonValue::from_object_id(dwallet_cap_id)];

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

/// Approves an Ethereum transaction for a given dWallet.
///
/// Interacts with the Ethereum light client to verify and approve a transaction message
/// using an Ethereum smart contract linked to a dWallet within the dWallet blockchain context.
/// The verification of the state & message is done offline, inside the dWallet module.
/// # Logic
/// 1. **Retrieve Configuration**: It starts by retrieving the Ethereum state object ID from
///      the active environment.
/// 2. **Fetch Ethereum Objects**: It retrieves and deserializes the [`EthereumDWalletCap`],
///     [`LatestEthereumStateObject`], and [`EthereumStateObject`] to collect the
///     latest Ethereum state data.
/// 3. **Initialize Light Client**: Initializes the Ethereum light client with the deserialized
///      Ethereum state.
/// 4. **Prepare Proof Parameters**: Constructs proof request parameters using the message,
///      dWallet ID, and data slot get from the [`EthereumDWalletCap`].
/// 5. **Fetch Updates and Proofs**: Retrieves the necessary updates and cryptographic proofs from
///      the Ethereum light client.
/// 6. **Build Transaction**: Uses the Sui programmable transaction builder to serialize transaction
///      parameters, including the Ethereum state, updates, and shared state object,
///      and prepares the transaction to call the [`VERIFY_ETH_STATE_FUNC_NAME`] function in the
///      Ethereum state module.
/// 7. **Send Transaction**: Constructs the transaction data, including the proof and dWallet ID,
///      and executes or serializes it based on the provided flags.
/// # Arguments
/// * `eth_dwallet_cap_id` – The `ObjectID` of the Ethereum dWallet capability,
///     representing the link between the dWallet and Ethereum.
/// * `message` – The Ethereum transaction message to be approved.
/// * `dwallet_id` – The `ObjectID` of the dWallet to which the transaction belongs.
/// # Returns
/// * `Result<SuiClientCommandResult>` –
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
) -> Result<SuiClientCommandResult> {
    let active_env = context.config.get_active_env()?.clone();
    let mut eth_lc_config = get_eth_config(context)?;

    let latest_state_object_id = match active_env.eth_client_settings {
        Some(eth_client_settings) => eth_client_settings
            .state_object_id
            .ok_or_else(|| anyhow!("ETH State object ID configuration not found"))?,
        None => return Err(anyhow!("ETH State object ID configuration not found")),
    };

    let latest_eth_state_data_bcs = get_object_bcs_by_id(context, latest_state_object_id).await?;
    let latest_eth_state_obj: LatestEthereumStateObject = latest_eth_state_data_bcs
        .try_as_move()
        .ok_or_else(|| anyhow!("object is not a Move Object"))?
        .deserialize()
        .map_err(|e| anyhow!("error deserializing object: {e}"))?;

    let eth_state_object_id = latest_eth_state_obj.eth_state_id;
    let eth_state_data_bcs = get_object_bcs_by_id(context, eth_state_object_id).await?;
    let eth_state_obj: EthereumStateObject = eth_state_data_bcs
        .try_as_move()
        .ok_or_else(|| anyhow!("object is not a Move Object"))?
        .deserialize()
        .map_err(|e| anyhow!("error deserializing object: {e}"))?;

    let mut eth_state = bcs::from_bytes::<ConsensusStateManager<NimbusRpc>>(&eth_state_obj.data)
        .map_err(|e| anyhow!("error parsing eth state data: {e}"))?;
    let mut eth_state = eth_state.set_rpc(&eth_lc_config.consensus_rpc);

    let latest_eth_state_shared_object =
        get_shared_object_input_by_id(context, latest_state_object_id).await?;

    if let Some(checkpoint) = eth_state.last_checkpoint.clone() {
        eth_lc_config.checkpoint = format!("0x{}", encode(checkpoint));
    } else {
        return Err(anyhow!("checkpoint is missing in the Ethereum state"));
    }

    let data_slot = latest_eth_state_obj.eth_smart_contract_slot;
    let contract_addr: String = latest_eth_state_obj.eth_smart_contract_address;
    let contract_addr = contract_addr.clone().parse::<Address>()?;

    let mut eth_lc = EthLightClientWrapper::init_new_light_client(eth_lc_config.clone()).await?;

    let updates_response = eth_state
        .get_updates_since_checkpoint()
        .await
        .map_err(|e| anyhow!("could not fetch updates: {e}"))?;

    let gas_owner = context.try_get_object_owner(&gas).await?;
    let sender = gas_owner.unwrap_or(context.active_address()?);

    // Serialize Move parameters
    let mut pt_builder = ProgrammableTransactionBuilder::new();
    let eth_state_arg = pt_builder
        .pure(serde_json::to_vec(&eth_state)?)
        .map_err(|e| anyhow!("could not serialize eth_state: {e}"))?;

    let updates_vec_arg = pt_builder
        .pure(serde_json::to_vec(&updates_response.updates)?)
        .map_err(|e| anyhow!("could not serialize updates: {e}"))?;

    let finality_update_arg = pt_builder
        .pure(serde_json::to_vec(&updates_response.finality_update)?)
        .map_err(|e| anyhow!("could not serialize `finality_updates`: {e}"))?;

    let optimistic_update_arg = pt_builder
        .pure(serde_json::to_vec(&updates_response.optimistic_update)?)
        .map_err(|e| anyhow!("could not serialize `optimistic_updates`: {e}"))?;

    let latest_eth_state_shared_object_arg = ObjectArg::SharedObject {
        id: latest_eth_state_shared_object.id,
        initial_shared_version: latest_eth_state_shared_object.initial_shared_version,
        mutable: true,
    };

    let latest_eth_state_arg = pt_builder
        .obj(latest_eth_state_shared_object_arg)
        .map_err(|e| anyhow!("could not serialize `latest_eth_state_id`: {e}"))?;

    pt_builder.programmable_move_call(
        SUI_SYSTEM_PACKAGE_ID,
        ETHEREUM_STATE_MODULE_NAME.into(),
        VERIFY_ETH_STATE_FUNC_NAME.into(),
        vec![],
        Vec::from([
            updates_vec_arg,
            finality_update_arg,
            optimistic_update_arg,
            eth_state_arg,
            latest_eth_state_arg,
        ]),
    );

    let client = context.get_client().await?;
    let tx_data = client
        .transaction_builder()
        .finish_programmable_transaction(sender, pt_builder, gas, gas_budget)
        .await?;

    let verify_state_session_response = serialize_or_execute!(
        tx_data,
        serialize_unsigned_transaction,
        serialize_signed_transaction,
        context,
        Call
    );

    let SuiClientCommandResult::Call(state) = verify_state_session_response else {
        return Err(anyhow!("can't get response"));
    };

    let object_changes = state
        .object_changes
        .clone()
        .ok_or_else(|| anyhow!("can't get object changes"))?;

    let verified_state_object_id = get_object_from_transaction_changes(
        object_changes,
        ETHEREUM_STATE_MODULE_NAME.into(),
        ETH_STATE_STRUCT_NAME.into(),
    )?;

    let verified_eth_state_data_bcs =
        get_object_bcs_by_id(context, verified_state_object_id).await?;
    let verified_eth_state_obj: EthereumStateObject = verified_eth_state_data_bcs
        .try_as_move()
        .ok_or_else(|| anyhow!("object is not a Move Object"))?
        .deserialize()
        .map_err(|e| anyhow!("error deserializing object: {e}"))?;

    let mut verified_eth_state =
        bcs::from_bytes::<ConsensusStateManager<NimbusRpc>>(&verified_eth_state_obj.data)
            .map_err(|e| anyhow!("error parsing eth state data: {e}"))?;
    let mut verified_eth_state = verified_eth_state.set_rpc(&eth_lc_config.consensus_rpc);

    let latest_slot = updates_response
        .finality_update
        .finalized_header
        .slot
        .as_u64();
    let latest_execution_payload = verified_eth_state
        .get_execution_payload(&Some(latest_slot))
        .await
        .map_err(|e| anyhow!("could not fetch execution payload: {e}"))?;

    let proof_params = ProofRequestParameters {
        message: message.clone(),
        dwallet_id: dwallet_id.as_slice().to_vec(),
        data_slot,
    };

    let proof = eth_lc
        .get_proofs(&contract_addr, proof_params, &latest_execution_payload)
        .await
        .map_err(|e| anyhow!("could not fetch proof: {e}"))?;

    let proof_sui_json =
        serialize_object(&proof).map_err(|e| anyhow!("could not serialize proof: {e}"))?;

    let mut pt_builder = ProgrammableTransactionBuilder::new();
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

// todo(zeev): check if we can add a type validation in here.
fn serialize_object<T>(object: &T) -> Result<SuiJsonValue>
where
    T: ?Sized + Serialize,
{
    let object_json = bcs::to_bytes(&object)?
        .iter()
        .map(|v| Value::Number(Number::from(*v)))
        .collect();
    SuiJsonValue::new(Value::Array(object_json))
}

fn get_eth_config(context: &mut WalletContext) -> Result<EthLightClientConfig> {
    let sui_env_config = context.config.get_active_env()?;

    let eth_client_settings = sui_env_config
        .eth_client_settings
        .clone()
        .ok_or_else(|| anyhow!("ETH client settings configuration not found"))?;

    let eth_execution_rpc = eth_client_settings
        .eth_execution_rpc
        .clone()
        .ok_or_else(|| anyhow!("ETH execution RPC configuration not found"))?;
    let eth_consensus_rpc = eth_client_settings
        .eth_consensus_rpc
        .clone()
        .ok_or_else(|| anyhow!("ETH consensus RPC configuration not found"))?;

    let mut eth_lc_config = EthLightClientConfig::default();
    eth_lc_config.network = get_network_by_sui_env(sui_env_config)?;
    eth_lc_config.execution_rpc = eth_execution_rpc;
    eth_lc_config.consensus_rpc = eth_consensus_rpc;

    Ok(eth_lc_config)
}

fn get_network_by_sui_env(sui_env_config: &SuiEnv) -> Result<Network> {
    let network = match sui_env_config.alias.as_str() {
        "mainnet" => Network::MAINNET,
        "testnet" => Network::HOLESKY,
        "localnet" => Network::LOCAL,
        _ => Network::MAINNET,
    };
    Ok(network)
}
