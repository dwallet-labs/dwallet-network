use anyhow::{anyhow, Error};
use serde_json::{Number, Value};

use sui_json::SuiJsonValue;
use sui_json_rpc_types::ObjectChange;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::eth_types::eth_dwallet_cap::{CREATE_ETH_DWALLET_CAP_FUNC_NAME, ETH_DWALLET_MODULE_NAME, ETHEREUM_STATE_MODULE_NAME, INIT_STATE_FUNC_NAME, LATEST_ETH_STATE_STRUCT_NAME};
use sui_types::SUI_SYSTEM_PACKAGE_ID;

use crate::client_commands::{construct_move_call_transaction, SuiClientCommandResult};
use crate::serialize_or_execute;

//todo(yuval): in future, we should load also the sync committee from binary data file.
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

