use anyhow::anyhow;
use clap::Error;
use helios::config::networks::Network;
use serde_json::{Number, Value};

use sui_json::SuiJsonValue;
use sui_json_rpc_types::{SuiData, SuiObjectDataOptions};
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::dwallet_eth::config::EthClientConfig;
use sui_types::dwallet_eth::eth_state::EthState;
use sui_types::dwallet_eth::light_client::EthLightClient;
use sui_types::eth_current_state::EthStateSuiObject;
use sui_types::eth_dwallet::EthDWalletCap;
use sui_types::SUI_SYSTEM_PACKAGE_ID;

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
) -> SuiClientCommandResult {
    let sui_env_config = context.config.get_active_env()?;
    let eth_execution_rpc = sui_env_config.eth_execution_rpc.clone()?;
    let eth_consensus_rpc = sui_env_config.eth_consensus_rpc.clone()?;
    let state_object_id = sui_env_config.state_object_id.clone()?;
    let provided_checkpoint = sui_env_config.checkpoint.clone()?;

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
            anyhow!("Could not find object with ID: {:?}", eth_dwallet_cap_id)
        }
    };

    let bcs_data = data.bcs.ok_or_else(|| anyhow!("missing object data"))?;

    let eth_dwallet_cap_obj: EthDWalletCap = bcs_data
        .try_as_move()
        .ok_or_else(|| anyhow!("Object is not a Move Object"))?
        .deserialize()?;

    let data_slot = eth_dwallet_cap_obj.eth_smart_contract_slot;
    let contract_addr = hex::encode(eth_dwallet_cap_obj.eth_smart_contract_addr)?;

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
        None => {
            anyhow!("Could not find object with ID: {:?}", state_object_id);
        }
    };

    let bcs_data = data.bcs.ok_or_else(|| anyhow!("missing object data"))?;

    let eth_current_state_obj: EthStateSuiObject = bcs_data
        .try_as_move()
        .ok_or_else(|| anyhow!("Object is not a Move Object"))?
        .deserialize()?;

    let mut eth_state = EthState::eth_state_from_json(&eth_current_state_obj.data.into())?;
    eth_state.set_rpc(eth_consensus_rpc);
    let current_state_checkpoint = hex::encode(eth_state.last_checkpoint);

    let Ok(updates) = eth_state.get_updates(&current_state_checkpoint, &provided_checkpoint).await else {
        anyhow!("error fetching updates from Consensus RPC: {e}")
    };
    let Ok(proof) = eth_lc.get_proof().await else {
        anyhow!("Error getting proof from Consensus RPC: {e}")
    };

    let message = bcs::to_bytes(&message)?
        .iter()
        .map(|v| Value::Number(Number::from(*v)))
        .collect();
    let message_json_val = SuiJsonValue::new(Value::Array(message))?;

    let updates_json_bytes = serde_json::to_string(&updates)?.into_bytes().to_vec();
    let updates = bcs::to_bytes(&updates_json_bytes)
        .iter()
        .map(|v| Value::Number(Number::from(*v.clone())))
        .collect();
    let updates_json_val = SuiJsonValue::new(Value::Array(updates))?;

    let proof_json_bytes = serde_json::to_string(&proof)?.into_bytes().to_vec();
    let proof = bcs::to_bytes(&proof_json_bytes)
        .iter()
        .map(|v| Value::Number(Number::from(*v.clone())))
        .collect();
    let proof_json_val = SuiJsonValue::new(Value::Array(proof));

    let current_state_json_bytes = serde_json::to_string(&eth_state)?.into_bytes().to_vec();
    let current_state = bcs::to_bytes(&current_state_json_bytes)
        .iter()
        .map(|v| Value::Number(Number::from(*v.clone())))
        .collect();
    let current_state_json_val = SuiJsonValue::new(Value::Array(current_state));

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
    serialize_or_execute!(
        tx_data,
        serialize_unsigned_transaction,
        serialize_signed_transaction,
        context,
        Call
    )
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
) -> Result<SuiClientCommandResult, Error> {
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
