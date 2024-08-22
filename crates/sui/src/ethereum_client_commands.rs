use anyhow::anyhow;
use clap::Subcommand;
use serde_json::{Number, Value};

use shared_crypto::intent::Intent;
use sui_json::SuiJsonValue;
#[allow(unused_imports)]
// SuiTransactionBlockEffectsAPI is called in a macro; therefore, the IDE doesn't recognize it.
use sui_json_rpc_types::{ObjectChange, SuiExecutionStatus, SuiTransactionBlockEffectsAPI};
use sui_keys::keystore::AccountKeystore;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::eth_dwallet::{
    CREATE_ETH_DWALLET_CAP_FUNC_NAME, ETHEREUM_STATE_MODULE_NAME, ETH_DWALLET_MODULE_NAME,
    INIT_STATE_FUNC_NAME, LATEST_ETH_STATE_STRUCT_NAME,
};
use sui_types::transaction::{SenderSignedData, Transaction, TransactionDataAPI};
use sui_types::SUI_SYSTEM_PACKAGE_ID;

use crate::client_commands::{construct_move_call_transaction, SuiClientCommandResult};
use crate::serialize_or_execute;

#[derive(Subcommand)]
pub enum EthClientCommands {
    /// Approve a TX with Eth contract for a given dWallet.
    #[command(name = "verify-message")]
    EthApproveMessage {
        /// Object of a [EthDwalletCap].
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
        /// Gas budget for this call
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
        /// The address of the contract.
        #[clap(long)]
        contract_address: String,
        /// The slot of the Data map that holds approved transactions in eth smart contract.
        #[clap(long)]
        contract_approved_tx_slot: u64,
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
        network: String,
        #[clap(long)]
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
    context: &mut WalletContext,
    gas: Option<ObjectID>,
    gas_budget: u64,
    serialize_unsigned_transaction: bool,
    serialize_signed_transaction: bool,
) -> Result<SuiClientCommandResult, anyhow::Error> {
    let args = vec![
        SuiJsonValue::new(Value::String(checkpoint))?,
        SuiJsonValue::new(Value::String(network.to_string()))?,
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
        return Err(anyhow!("Can't get response."));
    };

    let object_changes = state
        .object_changes
        .clone()
        .ok_or_else(|| anyhow!("Can't get object changes."))?;

    let latest_state_object_id = object_changes
        .iter()
        .find_map(|oc| {
            if let ObjectChange::Created {
                object_id,
                object_type,
                ..
            } = oc
            {
                if object_type.module == ETHEREUM_STATE_MODULE_NAME.into()
                    && object_type.name == LATEST_ETH_STATE_STRUCT_NAME.into()
                {
                    return Some(object_id);
                }
            }
            None
        })
        .ok_or_else(|| anyhow!("Can't get latest state object ID."))?;

    context
        .config
        .update_ethereum_state_object_id(latest_state_object_id.clone())?;
    context.config.save()?;

    Ok(SuiClientCommandResult::Call(state))
}

/// Creates an Ethereum-based dwallet using the provided parameters.
/// This function constructs a Move call transaction to create an Ethereum dWallet.
/// The transaction involves passing a smart contract address,
/// a transaction slot that was previously approved, and other necessary parameters.
/// The function serializes the smart contract address,
/// processes it into a format compatible with the Move
/// TX, and removes any leading '*' prefix from the address.
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
    let smart_contract_address = bcs::to_bytes(&smart_contract_address)?;
    let mut smart_contract_address: Vec<Value> = smart_contract_address
        .iter()
        .map(|v| Value::Number(Number::from(*v)))
        .collect();
    // Remove '*' prefix from the smart contract address.
    smart_contract_address.remove(0);

    let smart_contract_address = SuiJsonValue::new(Value::Array(smart_contract_address))?;

    let args = vec![
        SuiJsonValue::from_object_id(dwallet_cap_id),
        smart_contract_address,
        SuiJsonValue::new(Value::Number(Number::from(smart_contract_approved_tx_slot)))?,
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
