use anyhow::{anyhow, Error};
use base64::Engine;
use ethers::types::Address;
use helios::config::networks::Network;
use serde::Serialize;
use serde_json::{Number, Value};
use shared_crypto::intent::Intent;
use sui_json::{call_args, SuiJsonValue};
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
use sui_json_rpc_types::{SuiData, SuiObjectDataOptions};
use sui_json_rpc_types::{SuiExecutionStatus, SuiObjectData, SuiRawData};
use sui_keys::keystore::AccountKeystore;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::eth_dwallet::config::EthLightClientConfig;
use sui_types::eth_dwallet::eth_state::{EthState, EthStateObject};
use sui_types::eth_dwallet::light_client::EthLightClient;
use sui_types::eth_dwallet::proof::ProofResponse;
use sui_types::eth_dwallet::update::UpdatesResponse;
use sui_types::eth_dwallet_cap::{
    EthDWalletCap, APPROVE_MESSAGE_FUNC_NAME, CREATE_ETH_DWALLET_CAP_FUNC_NAME,
    ETH_DWALLET_MODULE_NAME, VERIFY_ETH_STATE_FUNC_NAME,
};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::signature_mpc::{
    APPROVE_MESSAGES_FUNC_NAME, CREATE_DKG_SESSION_FUNC_NAME, DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME,
    DWALLET_MODULE_NAME,
};
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
    let state_object_id = context.config.get_active_env()?.state_object_id.clone().ok_or_else(|| anyhow!("ETH State object ID configuration not found"))?;

    let eth_dwallet_cap_data_bcs = fetch_object(context, eth_dwallet_cap_id).await?;
    let eth_dwallet_cap_obj: EthDWalletCap = eth_dwallet_cap_data_bcs.try_into()?;

    // todo(yuval): we need to decide how we implement this, maybe we should use constant address for the state object
    let eth_state_data_bcs = fetch_object(context, state_object_id).await?;
    let eth_state_obj: EthStateObject = eth_state_data_bcs.try_into()?;

    // Desrialize Eth State object
    let mut eth_state = bcs::from_bytes::<EthState>(&eth_state_obj.data)
        .map_err(|e| anyhow!("error parsing eth state data: {e}"))?;

    let (data_slot, contract_addr) = get_data_from_eth_dwallet_cap(eth_dwallet_cap_obj)?;

    let mut eth_lc_config = get_eth_config(context)?;
    // todo(yuval): make a separate object for func args
    eth_lc_config.data_slot = data_slot;
    eth_lc_config.message = message.clone();
    eth_lc_config.dwallet_id = dwallet_id.as_slice().to_vec();
    eth_lc_config.checkpoint = eth_state.last_checkpoint.clone();

    let mut eth_lc = init_light_client(eth_lc_config.clone()).await?;
    let mut eth_state = eth_state
        .set_rpc(eth_lc_config.consensus_rpc.clone())
        .set_network(eth_lc_config.network.clone());

    // Fetch updates & proof from the consensus RPC
    let Ok(updates) = fetch_consensus_updates(&mut eth_state).await else {
        return Err(anyhow!("Could not fetch updates."));
    };

    let Ok(proof) = fetch_proofs(&mut eth_lc, &eth_state, &contract_addr).await else {
        return Err(anyhow!("Could not fetch proof."));
    };

    let gas_owner = context.try_get_object_owner(&gas).await?;
    let sender = gas_owner.unwrap_or(context.active_address()?);

    // Serialize Move parameters
    let mut pt_builder = ProgrammableTransactionBuilder::new();
    let eth_state_arg = pt_builder.pure(bcs::to_bytes(&eth_state)?).unwrap();

    let updates_arg = pt_builder.pure(bcs::to_bytes(&updates)?).unwrap();

    pt_builder.programmable_move_call(
        SUI_SYSTEM_PACKAGE_ID,
        ETH_DWALLET_MODULE_NAME.into(),
        VERIFY_ETH_STATE_FUNC_NAME.into(),
        vec![],
        Vec::from([updates_arg, eth_state_arg]),
    );

    let Ok(proof_sui_json) = serialize_object(&proof) else {
        return Err(anyhow!("Could not serialize proof"));
    };

    let Ok(dwallet_id_json) = serialize_object(&dwallet_id.as_slice().to_vec()) else {
        return Err(anyhow!("Could not serialize dwallet_id"));
    };

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
                dwallet_id_json,
                proof_sui_json,
            ]),
        )
        .await?;

    let client = context.get_client().await?;
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

fn get_data_from_eth_dwallet_cap(
    eth_dwallet_cap_obj: EthDWalletCap,
) -> Result<(u64, Address), Error> {
    let data_slot = eth_dwallet_cap_obj.eth_smart_contract_slot;
    let mut contract_addr: String = eth_dwallet_cap_obj.eth_smart_contract_addr;
    let contract_addr = contract_addr.clone().parse::<Address>()?;
    Ok((data_slot, contract_addr))
}

async fn init_light_client(eth_client_config: EthLightClientConfig) -> Result<EthLightClient, Error> {
    let mut eth_lc = EthLightClient::new(eth_client_config.clone()).await?;
    eth_lc.start().await?;
    Ok(eth_lc)
}

async fn fetch_proofs(
    eth_lc: &mut EthLightClient,
    eth_state: &EthState,
    contract_addr: &Address,
) -> Result<ProofResponse, Result<SuiClientCommandResult, Error>> {
    let proof = match eth_lc
        .get_proofs(
            contract_addr,
            eth_state.last_update_execution_block_number,
            &eth_state.last_update_execution_state_root,
        )
        .await
    {
        Ok(proof) => proof,
        Err(e) => return Err(Err(anyhow!("error fetching proof from Consensus RPC: {e}"))),
    };
    Ok(proof)
}

async fn fetch_consensus_updates(
    eth_state: &mut EthState,
) -> Result<UpdatesResponse, Result<SuiClientCommandResult, Error>> {
    let Ok(updates) = eth_state
        .get_updates(&eth_state.clone().last_checkpoint)
        .await
    else {
        return Err(Err(anyhow!(
            "error fetching updates from Consensus RPC"
        )));
    };
    Ok(updates)
}

pub(crate) async fn create_eth_dwallet(
    context: &mut WalletContext,
    dwallet_cap_id: ObjectID,
    smart_contract_address: String,
    smart_contract_approved_tx_slot: u64,
    gas: Option<ObjectID>,
    gas_budget: u64,
    serialize_unsigned_transaction: bool,
    serialize_signed_transaction: bool,
) -> Result<SuiClientCommandResult, anyhow::Error> {
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

fn get_eth_config(context: &mut WalletContext) -> Result<EthLightClientConfig, Error> {
    let mut eth_lc_config = EthLightClientConfig::default();

    let sui_env_config = context.config.get_active_env()?;
    let eth_execution_rpc = sui_env_config
        .eth_execution_rpc
        .clone()
        .ok_or_else(|| anyhow!("ETH execution RPC configuration not found"))?;
    let eth_consensus_rpc = sui_env_config
        .eth_consensus_rpc
        .clone()
        .ok_or_else(|| anyhow!("ETH consensus RPC configuration not found"))?;
    let eth_chain_id = sui_env_config
        .eth_chain_id
        .clone()
        .ok_or_else(|| anyhow!("ETH Chain ID configuration not found"))?;
    let eth_genesis_time = sui_env_config
        .eth_genesis_time
        .clone()
        .ok_or_else(|| anyhow!("ETH Genesis Time configuration not found"))?;
    let eth_genesis_validators_root = sui_env_config
        .eth_genesis_validators_root
        .clone()
        .ok_or_else(|| anyhow!("ETH Genesis Validators Root configuration not found"))?;

    let chain_config = helios::config::types::ChainConfig {
        chain_id: eth_chain_id,
        genesis_time: eth_genesis_time,
        genesis_root: hex_str_to_bytes(&eth_genesis_validators_root).unwrap(),
    };

    eth_lc_config.network = Network::DEVNET(chain_config);
    eth_lc_config.execution_rpc = eth_execution_rpc;
    eth_lc_config.consensus_rpc = eth_consensus_rpc;
    Ok(eth_lc_config)
}

pub fn hex_str_to_bytes(s: &str) -> eyre::Result<Vec<u8>> {
    let stripped = s.strip_prefix("0x").unwrap_or(s);
    Ok(hex::decode(stripped)?)
}
