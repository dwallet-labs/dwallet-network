use anyhow::{anyhow, Error};
use base64::Engine;
use helios::config::networks::Network;
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
use serde_json::{Number, Value};
use shared_crypto::intent::Intent;
use sui_json::{call_args, SuiJsonValue};
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
use sui_json_rpc_types::{SuiData, SuiObjectDataOptions};
use sui_json_rpc_types::{SuiExecutionStatus, SuiObjectData, SuiRawData};
use sui_keys::keystore::AccountKeystore;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::eth_dwallet::config::EthClientConfig;
use sui_types::eth_dwallet::eth_state::{EthState, EthStateObject};
use sui_types::eth_dwallet::light_client::EthLightClient;
use sui_types::eth_dwallet::proof::ProofResponse;
use sui_types::eth_dwallet::update::UpdatesResponse;
use sui_types::eth_dwallet_cap::{
    EthDWalletCap, ETH_DWALLET_MODULE_NAME, VERIFY_ETH_STATE_FUNC_NAME,
};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::Transaction;
use sui_types::transaction::TransactionDataAPI;
use sui_types::transaction::{CallArg, ObjectArg, SenderSignedData};
use sui_types::SUI_SYSTEM_PACKAGE_ID;

use crate::client_commands::{construct_move_call_transaction, SuiClientCommandResult};
use crate::serialize_or_execute;

struct SuiRawDataWrapper(SuiRawData);

impl TryFrom<SuiRawDataWrapper> for EthDWalletCap {
    type Error = anyhow::Error;
    fn try_from(wrapper: SuiRawDataWrapper) -> Result<Self, Error> {
        wrapper
            .0
            .try_as_move()
            .ok_or_else(|| anyhow!("Object is not a Move Object"))?
            .deserialize()
            .map_err(|e| anyhow!("Error deserializing object: {e}"))
    }
}

impl TryFrom<SuiRawDataWrapper> for EthStateObject {
    type Error = anyhow::Error;
    fn try_from(wrapper: SuiRawDataWrapper) -> Result<Self, Error> {
        wrapper
            .0
            .try_as_move()
            .ok_or_else(|| anyhow!("Object is not a Move Object"))?
            .deserialize()
            .map_err(|e| anyhow!("Error deserializing object: {e}"))
    }
}

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
    let (eth_execution_rpc, eth_consensus_rpc, state_object_id, provided_checkpoint) =
        get_sui_env_config(context)?;

    let eth_dwallet_cap_bcs_data = fetch_object(context, eth_dwallet_cap_id).await?;
    let eth_dwallet_cap_obj: EthDWalletCap = eth_dwallet_cap_bcs_data.try_into()?;

    // todo(yuval): we need to decide how we implement this, maybe we should use constant address for the state object
    let eth_state_data_bcs = fetch_object(context, state_object_id).await?;
    let eth_state_obj: EthStateObject = eth_state_data_bcs.try_into()?;

    let data_slot = eth_dwallet_cap_obj.eth_smart_contract_slot;
    // todo(yuval): check why the string has prefix and remove it the right way
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

    // Desrialize Eth State object
    //todo(yuval): deserialize from bcs, not from json
    let eth_state_data_str = String::from_utf8(eth_current_state_obj.data)
        .map_err(|e| anyhow!("error parsing eth state data: {e}"))?;
    let mut eth_state = EthState::build_from_json(&eth_state_data_str)?;
    let mut eth_state = eth_state.set_rpc(eth_consensus_rpc);

    // Fetch updates & proof from the consensus RPC
    let updates = match fetch_consensus_updates(&provided_checkpoint, &mut eth_state).await {
        Ok(value) => value,
        Err(value) => return value,
    };

    let _proof = match fetch_proofs(&mut eth_lc, &mut eth_state).await {
        Ok(value) => value,
        Err(value) => return value,
    };

    let gas_owner = context.try_get_object_owner(&gas).await?;
    let sender = gas_owner.unwrap_or(context.active_address()?);

    // Serialize Move parameters
    let message = bcs::to_bytes(&message)?
        .iter()
        .map(|v| Value::Number(Number::from(*v)))
        .collect();
    let message_sui_json = SuiJsonValue::new(Value::Array(message))?;

    let mut tx_builder = ProgrammableTransactionBuilder::new();

    let eth_state_arg = tx_builder
        .pure(bcs::to_bytes(&eth_state)?)
        .unwrap();
    let _res = eth_state.check_serialization();
    // eth_state.next_sync_committee = Some(eth_state.clone().current_sync_committee);
    // let _eth_state_des: EthState = bincode::deserialize(&eth_bytes)?;
    // match bcs::from_bytes(eth_bytes.clone().as_slice()){
    //     Ok(value) => value,
    //     Err(e) => return Err(anyhow!("error deserializing eth state: {e}")),
    // };

    let updates_bytes = updates.custom_serialize();

    let _updates_obj: UpdatesResponse = updates_bytes.custom_deserialize();
    //     match bcs::from_bytes(&updates_bytes){
    //     Ok(value) => value,
    //     Err(e) => return Err(anyhow!("error deserializing updates: {e}")),
    // };

    let updates_arg = tx_builder
        .pure(bcs::to_bytes(&updates)?)
        .unwrap();

    tx_builder.programmable_move_call(
        SUI_SYSTEM_PACKAGE_ID,
        ETH_DWALLET_MODULE_NAME.into(),
        VERIFY_ETH_STATE_FUNC_NAME.into(),
        vec![],
        Vec::from([updates_arg, eth_state_arg]),
    );

    let client = context.get_client().await?;
    let tx_data = client
        .transaction_builder()
        .finish_programmable_transaction(sender, tx_builder, gas, gas_budget)
        .await?;

    let session_response = serialize_or_execute!(
        tx_data,
        serialize_unsigned_transaction,
        serialize_signed_transaction,
        context,
        Call
    );
    Ok(session_response)

    // let message_sui_json = serialize_argument(&message)?;
    // let proof_sui_json = serialize_argument(&proof)?;
    //
    // let args = Vec::from([
    //     SuiJsonValue::from_object_id(eth_dwallet_cap_id),
    //     SuiJsonValue::from_object_id(dwallet_id),
    //     message_sui_json,
    //     proof_sui_json,
    // ]);
    //
    // let tx_data = construct_move_call_transaction(
    //     SUI_SYSTEM_PACKAGE_ID,
    //     "eth_dwallet",
    //     "approve_message",
    //     vec![],
    //     gas,
    //     gas_budget,
    //     args,
    //     context,
    // )
    // .await?;
    // Ok(serialize_or_execute!(
    //     tx_data,
    //     serialize_unsigned_transaction,
    //     serialize_signed_transaction,
    //     context,
    //     Call
    // ))
}

fn serialize_argument<T: serde::Serialize>(object: &T) -> Result<SuiJsonValue, Error> {
    let serialized_numbers = bcs::to_bytes(object)?
        .iter()
        .map(|v| Value::Number(Number::from(*v)))
        .collect();
    let object_as_sui_json = SuiJsonValue::new(Value::Array(serialized_numbers))?;
    Ok(object_as_sui_json)
}

async fn fetch_proofs(
    eth_lc: &mut EthLightClient,
    eth_state: &mut &mut EthState,
) -> Result<ProofResponse, Result<SuiClientCommandResult, Error>> {
    let proof = match eth_lc.get_proofs(&eth_state.execution_state_root).await {
        Ok(proof) => proof,
        Err(e) => return Err(Err(anyhow!("error fetching proof from Consensus RPC: {e}"))),
    };
    Ok(proof)
}

async fn fetch_consensus_updates(
    provided_checkpoint: &String,
    eth_state: &mut &mut EthState,
) -> Result<UpdatesResponse, Result<SuiClientCommandResult, Error>> {
    let updates = match eth_state
        .get_updates(
            &eth_state.clone().last_checkpoint,
            &provided_checkpoint.clone(),
        )
        .await
    {
        Ok(updates) => updates,
        Err(e) => {
            return Err(Err(anyhow!(
                "error fetching updates from Consensus RPC: {e}"
            )))
        }
    };
    Ok(updates)
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

async fn fetch_object(
    context: &mut WalletContext,
    object_id: ObjectID,
) -> Result<SuiRawDataWrapper, Error> {
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
        Some(data) => Ok(SuiRawDataWrapper(
            data.bcs.ok_or_else(|| anyhow!("missing object data"))?,
        )),
        None => Err(anyhow!("Could not find object with ID: {:?}", object_id)),
    }
}

fn get_sui_env_config(
    context: &mut WalletContext,
) -> Result<(String, String, ObjectID, String), Error> {
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
    Ok((
        eth_execution_rpc,
        eth_consensus_rpc,
        state_object_id,
        provided_checkpoint,
    ))
}
