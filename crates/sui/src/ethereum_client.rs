use anyhow::{anyhow, Error};
use clap::Subcommand;
use serde_json::{Number, Value};
use sui_json::SuiJsonValue;
use sui_json_rpc_types::{ObjectChange, SuiTransactionBlockEffectsAPI, SuiExecutionStatus};
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::eth_dwallet_cap::{APPROVE_MESSAGE_FUNC_NAME, CREATE_ETH_DWALLET_CAP_FUNC_NAME, ETH_DWALLET_MODULE_NAME, EthDWalletCap, ETHEREUM_STATE_MODULE_NAME, INIT_STATE_FUNC_NAME, LATEST_ETH_STATE_STRUCT_NAME, VERIFY_ETH_STATE_FUNC_NAME};
use sui_types::SUI_SYSTEM_PACKAGE_ID;
use sui_types::transaction::{TransactionDataAPI, Transaction, SenderSignedData, ObjectArg};
use sui_keys::keystore::AccountKeystore;
use shared_crypto::intent::Intent;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use eth_light_client::utils::{get_object_bcs_by_id, get_shared_object_input_by_id, serialize_object};
use eth_light_client::eth_state::{LatestEthStateObject, EthStateObject, EthState};
use eth_light_client::light_client::{get_data_from_eth_dwallet_cap, get_eth_config, init_light_client, fetch_consensus_updates, fetch_proofs};
use eth_light_client::config::ProofRequestParameters;
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
    let active_env = context
        .config
        .get_active_env()?;

    let eth_lc_config = match &active_env.eth_light_client {
        Some(eth_lc) => eth_lc.clone(),
        None => return Err(anyhow!("ETH Light Client configuration not found")),
    };

    let latest_state_object_id = match eth_lc_config.state_object_id
    {
        Some(state_object_id) => state_object_id,
        None => return Err(anyhow!("ETH State object ID configuration not found")),
    };

    let eth_dwallet_cap_data_bcs = get_object_bcs_by_id(context, eth_dwallet_cap_id).await?;
    let eth_dwallet_cap_obj: EthDWalletCap = eth_dwallet_cap_data_bcs.try_into()?;

    let latest_eth_state_data_bcs = get_object_bcs_by_id(context, latest_state_object_id).await?;
    let latest_eth_state_obj: LatestEthStateObject = latest_eth_state_data_bcs.try_into()?;
    let latest_eth_state_shared_object =
        get_shared_object_input_by_id(context, latest_state_object_id).await?;

    let eth_state_object_id = latest_eth_state_obj.eth_state_id;
    let eth_state_data_bcs = get_object_bcs_by_id(context, eth_state_object_id).await?;
    let eth_state_obj: EthStateObject = eth_state_data_bcs.try_into()?;

    // Desrialize Eth State object
    let mut eth_state = bcs::from_bytes::<EthState>(&eth_state_obj.data)
        .map_err(|e| anyhow!("error parsing eth state data: {e}"))?;

    let (data_slot, contract_addr) = get_data_from_eth_dwallet_cap(eth_dwallet_cap_obj)?;

    let mut eth_lc_config = get_eth_config(context)?;
    eth_lc_config.checkpoint = eth_state.last_checkpoint.clone();

    let proof_params = ProofRequestParameters {
        message: message.clone(),
        dwallet_id: dwallet_id.as_slice().to_vec(),
        data_slot,
    };

    let mut eth_lc = init_light_client(eth_lc_config.clone()).await?;
    let mut eth_state = eth_state
        .set_rpc(eth_lc_config.consensus_rpc.clone())
        .set_network(eth_lc_config.network.clone());

    // Fetch updates & proof from the consensus RPC
    let updates = fetch_consensus_updates(&mut eth_state)
        .await
        .map_err(|e| anyhow!("Could not fetch updates."))?;

    let proof = fetch_proofs(&mut eth_lc, &eth_state, &contract_addr, proof_params)
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
    let dwallet_id_json = serialize_object(&dwallet_id.as_slice().to_vec())
        .map_err(|e| anyhow!("Could not serialize dwallet_id. {e}"))?;

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