use anyhow::anyhow;
use helios::config::networks::Network;
use serde_json::{Number, Value};
use serde_json::Value::Array;
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
use sui_types::transaction::TransactionDataAPI;
use sui_keys::keystore::AccountKeystore;
use shared_crypto::intent::Intent;
use sui_json::SuiJsonValue;
use sui_json_rpc_types::{SuiData, SuiObjectDataOptions};
use sui_json_rpc_types::SuiExecutionStatus;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::eth_state::EthStateObject;
use sui_types::eth_dwallet::config::EthClientConfig;
use sui_types::eth_dwallet::eth_state::EthState;
use sui_types::eth_dwallet::light_client::EthLightClient;
use sui_types::eth_dwallet_cap::EthDWalletCap;
use sui_types::SUI_SYSTEM_PACKAGE_ID;
use sui_types::transaction::SenderSignedData;
use sui_types::transaction::Transaction;

use crate::client_commands::{construct_move_call_transaction, SuiClientCommandResult};
use crate::serialize_or_execute;

pub(crate) async fn eth_approve_message(
    context: &mut WalletContext,
    eth_dwallet_cap_id: ObjectID,
    message: &Vec<u8>,
    dwallet_id: ObjectID,
    gas: Option<ObjectID>,
    gas_budget: u64,
    serialize_unsigned_transaction: bool,
    serialize_signed_transaction: bool,
) -> Result<SuiClientCommandResult, anyhow::Error> {
    let sui_env_config = context.config.get_active_env()?;
    let eth_execution_rpc = sui_env_config.eth_execution_rpc.clone().ok_or_else(|| anyhow!("ETH execution RPC configuration not found"))?;
    let eth_consensus_rpc = sui_env_config.eth_consensus_rpc.clone().ok_or_else(|| anyhow!("ETH consensus RPC configuration not found"))?;
    let state_object_id = sui_env_config.state_object_id.clone().ok_or_else(|| anyhow!("ETH State object ID configuration not found"))?;
    let provided_checkpoint = sui_env_config.checkpoint.clone().ok_or_else(|| anyhow!("Checkpoint configuration not found"))?;

    let resp = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(
            eth_dwallet_cap_id,
            SuiObjectDataOptions::default().with_bcs().with_owner(),
        )
        .await?;

    let data = match resp.data {
        Some(data) => data,
        None => return Err(anyhow!("Could not find object with ID: {:?}", eth_dwallet_cap_id))
    };

    let bcs_data = data.bcs.ok_or_else(|| anyhow!("missing object data"))?;

    let eth_dwallet_cap_obj: EthDWalletCap = bcs_data
        .try_as_move()
        .ok_or_else(|| anyhow!("Object is not a Move Object"))?
        .deserialize()?;

    let data_slot = eth_dwallet_cap_obj.eth_smart_contract_slot;
    let contract_addr = hex::encode(eth_dwallet_cap_obj.eth_smart_contract_addr);

    let eth_client_config = EthClientConfig::new(
        Network::MAINNET,
        eth_execution_rpc,
        contract_addr,
        eth_consensus_rpc.clone(),
        data_slot,
        dwallet_id.to_hex().into_bytes(),
        message.clone(),
        0,
    )?;

    let mut eth_lc = EthLightClient::new(eth_client_config).await?;
    eth_lc.start().await?;

    //Fetch Current Eth State object
    let resp = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(
            state_object_id,
            SuiObjectDataOptions::default().with_bcs().with_owner(),
        )
        .await?;

    let data = match resp.data {
        Some(data) => data,
        None => return Err(anyhow!("Could not find object with ID: {:?}", state_object_id))
    };

    let bcs_data = data.bcs.ok_or_else(|| anyhow!("missing object data"))?;

    let eth_current_state_obj: EthStateObject = bcs_data
        .try_as_move()
        .ok_or_else(|| anyhow!("Object is not a Move Object"))?
        .deserialize()?;

    let eth_state_data_str = std::str::from_utf8(&eth_current_state_obj.data)
        .map_err(|e| anyhow!("error parsing eth state data: {e}"))?;
    let mut eth_state = EthState::build_from_json(&eth_state_data_str)?;
    let mut eth_state = eth_state.set_rpc(eth_consensus_rpc);

    let current_state_checkpoint = hex::encode(eth_state.clone().last_checkpoint);

    let updates = match eth_state.get_updates(&current_state_checkpoint, &provided_checkpoint).await {
        Ok(updates) => updates,
        Err(e) => return Err(anyhow!("error fetching updates from Consensus RPC: {e}")),
    };

    let proof = match eth_lc.get_proof().await {
        Ok(proof) => proof,
        Err(e) => return Err(anyhow!("error fetching proof from Consensus RPC: {e}")),
    };

    let message = bcs::to_bytes(&message)?
        .iter()
        .map(|v| Value::Number(Number::from(*v)))
        .collect();
    let message_json_val = SuiJsonValue::new(Value::Array(message))?;

    let updates_json_bytes = serde_json::to_string(&updates)?.into_bytes();
    let updates = bcs::to_bytes(&updates_json_bytes)?
        .iter()
        .map(|&v| Value::Number(Number::from(v)))
        .collect();
    let updates_json_val = SuiJsonValue::new(Value::Array(updates))?;

    let proof_json_bytes = serde_json::to_string(&proof)?.into_bytes();
    let proof = bcs::to_bytes(&proof_json_bytes)?
        .iter()
        .map(|&v| Value::Number(Number::from(v)))
        .collect();
    let proof_json_val = SuiJsonValue::new(Value::Array(proof))?;

    let current_state_json_bytes = serde_json::to_string(&eth_state)?.into_bytes();
    let current_state = bcs::to_bytes(&current_state_json_bytes)?
        .iter()
        .map(|&v| Value::Number(Number::from(v)))
        .collect();
    let current_state_json_val = SuiJsonValue::new(Value::Array(current_state))?;

    // todo(yuval): this might be a base64?
    let args = Vec::from([
        SuiJsonValue::from_object_id(eth_dwallet_cap_id),
        SuiJsonValue::from_object_id(dwallet_id),
        message_json_val,
        // todo(yuval): make sure this is the correct way to send these.
        proof_json_val,
        updates_json_val,
        current_state_json_val,
    ]);

    let tx_data = construct_move_call_transaction(
        SUI_SYSTEM_PACKAGE_ID,
        "eth_dwallet",
        "approve_message",
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

pub(crate) async fn create_eth_dwallet(
    context: &mut WalletContext,
    dwallet_cap_id: ObjectID,
    smart_contract_address: &String,
    smart_contract_approved_tx_slot: u64,
    gas: Option<ObjectID>,
    gas_budget: u64,
    serialize_unsigned_transaction: bool,
    serialize_signed_transaction: bool,
) -> Result<SuiClientCommandResult, anyhow::Error> {
    // Serialize to the Move TX format.
    // todo(zeev): check this, might just use Value::String()
    let smart_contract_address = bcs::to_bytes(&smart_contract_address).unwrap();
    let smart_contract_address = smart_contract_address
        .iter()
        .map(|v| Value::Number(Number::from(*v)))
        .collect();
    let smart_contract_address = SuiJsonValue::new(Value::Array(smart_contract_address)).unwrap();

    let args = vec![
        SuiJsonValue::from_object_id(dwallet_cap_id),
        smart_contract_address,
        SuiJsonValue::new(Value::Number(Number::from(smart_contract_approved_tx_slot))).unwrap(),
    ];

    let tx_data = construct_move_call_transaction(
        SUI_SYSTEM_PACKAGE_ID,
        // todo(zeev): make it use some Consts.
        "eth_dwallet",
        "create_eth_dwallet_cap",
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
