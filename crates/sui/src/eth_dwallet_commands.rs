use anyhow::anyhow;
use base64::Engine;
use helios::config::networks::Network;
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
use serde_json::{Number, Value};
use shared_crypto::intent::Intent;
use sui_json::SuiJsonValue;
use sui_json_rpc_types::SuiExecutionStatus;
use sui_json_rpc_types::{SuiData, SuiObjectDataOptions};
use sui_keys::keystore::AccountKeystore;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::eth_dwallet::config::EthClientConfig;
use sui_types::eth_dwallet::eth_state::{EthState, EthStateObject};
use sui_types::eth_dwallet::light_client::EthLightClient;
use sui_types::eth_dwallet_cap::EthDWalletCap;
use sui_types::transaction::SenderSignedData;
use sui_types::transaction::Transaction;
use sui_types::transaction::TransactionDataAPI;
use sui_types::SUI_SYSTEM_PACKAGE_ID;

use crate::client_commands::{construct_move_call_transaction, SuiClientCommandResult};
use crate::serialize_or_execute;

pub(crate) async fn eth_approve_message(
    context: &mut WalletContext,
    eth_dwallet_cap_id: ObjectID,
    message: String,
    dwallet_id: ObjectID,
    gas: Option<ObjectID>,
    gas_budget: u64,
    serialize_unsigned_transaction: bool,
    serialize_signed_transaction: bool,
) -> Result<SuiClientCommandResult, anyhow::Error> {
    let sui_env_config = context.config.get_active_env()?;
    let eth_execution_rpc = sui_env_config
        .eth_execution_rpc
        .clone()
        .ok_or_else(|| anyhow!("ETH execution RPC configuration not found"))?;
    let eth_consensus_rpc = sui_env_config
        .eth_consensus_rpc
        .clone()
        .ok_or_else(|| anyhow!("ETH consensus RPC configuration not found"))?;
    let state_object_id = sui_env_config
        .state_object_id
        .clone()
        .ok_or_else(|| anyhow!("ETH State object ID configuration not found"))?;
    let provided_checkpoint = sui_env_config
        .checkpoint
        .clone()
        .ok_or_else(|| anyhow!("Checkpoint configuration not found"))?;

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
        None => {
            return Err(anyhow!(
                "Could not find object with ID: {:?}",
                eth_dwallet_cap_id
            ))
        }
    };

    let bcs_data = data.bcs.ok_or_else(|| anyhow!("missing object data"))?;

    let eth_dwallet_cap_obj: EthDWalletCap = bcs_data
        .try_as_move()
        .ok_or_else(|| anyhow!("Object is not a Move Object"))?
        .deserialize()?;

    let data_slot = eth_dwallet_cap_obj.eth_smart_contract_slot;
    let mut contract_addr = String::from_utf8(eth_dwallet_cap_obj.eth_smart_contract_addr)?;
    contract_addr.remove(0);

    let eth_client_config = EthClientConfig::new(
        Network::SEPOLIA,
        eth_execution_rpc.clone(),
        contract_addr,
        eth_consensus_rpc.clone(),
        data_slot,
        dwallet_id.to_hex().into_bytes(),
        message.clone(),
        0,
        provided_checkpoint.clone(),
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
        None => {
            return Err(anyhow!(
                "Could not find state object with ID: {:?}",
                state_object_id,
            ))
        }
    };

    let eth_state_data_bcs = data.bcs.ok_or_else(|| anyhow!("missing object data"))?;

    let eth_current_state_obj: EthStateObject = eth_state_data_bcs
        .try_as_move()
        .ok_or_else(|| anyhow!("Object is not a Move Object"))?
        .deserialize()?;

    // Desrialize Eth State object
    //todo(yuval): deserialize from bcs, not from json
    let eth_state_data_str = String::from_utf8(eth_current_state_obj.data)
        .map_err(|e| anyhow!("error parsing eth state data: {e}"))?;
    let mut eth_state = EthState::build_from_json(&eth_state_data_str)?;
    let mut eth_state = eth_state.set_rpc(eth_consensus_rpc);

    // Fetch updates & proof from the consensus RPC
    let updates = match eth_state
        .get_updates(
            &eth_state.clone().last_checkpoint,
            &provided_checkpoint.clone(),
        )
        .await
    {
        Ok(updates) => updates,
        Err(e) => return Err(anyhow!("error fetching updates from Consensus RPC: {e}")),
    };

    let proof = match eth_lc.get_proof().await {
        Ok(proof) => proof,
        Err(e) => return Err(anyhow!("error fetching proof from Consensus RPC: {e}")),
    };

    // Serialize Move parameters
    let message = bcs::to_bytes(&message)?
        .iter()
        .map(|v| Value::Number(Number::from(*v)))
        .collect();
    let message_sui_json = SuiJsonValue::new(Value::Array(message))?;

    let updates_bcs = bcs::to_bytes(&updates)?
        .iter()
        .map(|&v| Value::Number(Number::from(v)))
        .collect();
    let updates_sui_json = SuiJsonValue::new(Value::Array(updates_bcs))?;

    let proof_bcs = bcs::to_bytes(&proof)?
        .iter()
        .map(|&v| Value::Number(Number::from(v)))
        .collect();
    let proof_sui_json = SuiJsonValue::new(Value::Array(proof_bcs))?;


    let eth_state_bcs = bcs::to_bytes(&eth_state)?
        .iter()
        .map(|&v| Value::Number(Number::from(v)))
        .collect();
    let eth_state_sui_json = SuiJsonValue::new(Value::Array(eth_state_bcs))?;

    let args = Vec::from([eth_state_sui_json.clone(), updates_sui_json.clone()]);

    let tx_data = construct_move_call_transaction(
        SUI_SYSTEM_PACKAGE_ID,
        "eth_dwallet",
        "verify_new_eth_state",
        vec![],
        gas,
        gas_budget,
        args.clone(),
        context,
    )
    .await?;
    serialize_or_execute!(
        tx_data,
        serialize_unsigned_transaction,
        serialize_signed_transaction,
        context,
        Call
    );

    // todo(yuval): this might be a base64?
    let args = Vec::from([
        SuiJsonValue::from_object_id(eth_dwallet_cap_id),
        SuiJsonValue::from_object_id(dwallet_id),
        message_sui_json,
        proof_sui_json,
        updates_sui_json,
        eth_state_sui_json,
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
