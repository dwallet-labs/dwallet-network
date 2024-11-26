// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::NativesCostTable;
use ethers::core::types::transaction::eip712::{EIP712WithDomain, Eip712};
use ethers::prelude::transaction::eip712::EIP712Domain;
use ethers::prelude::{Address, Eip712, EthAbiType, H160, U256};
use ethers::utils::hex;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::account_address::AccountAddress;
use move_core_types::gas_algebra::InternalGas;
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::native_charge_gas_early_exit;
use move_vm_runtime::native_functions::NativeContext;
use move_vm_types::loaded_data::runtime_types::Type;
use move_vm_types::natives::function::NativeResult;
use move_vm_types::pop_arg;
use move_vm_types::values::{Value, Vector};
use serde::{Deserialize, Serialize};
use smallvec::smallvec;
use std::collections::VecDeque;
use std::str::FromStr;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{
    ProgrammableTransaction, TransactionData, TransactionDataAPI, TransactionDataV1,
    TransactionKind,
};

/// Bind a `dwallet::DWalletCap` to an authority.
#[derive(Clone, Debug, Serialize, Deserialize, Eip712, EthAbiType)]
pub struct EthDWalletBinder {
    pub id: Vec<u8>,
    pub dwallet_cap: Vec<u8>,
    pub bind_to_authority: Vec<u8>,
    pub nonce: u64,
    pub virgin_bound: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
// This struct is similar to `EthDWalletBinder` but it is used for SUI.
// Do not merge into one struct, this will ruin the EIP712 signature.
pub struct SuiDWalletBinder {
    pub id: Vec<u8>,
    pub dwallet_cap: Vec<u8>,
    pub bind_to_authority: Vec<u8>,
    pub nonce: u64,
    pub virgin_bound: bool,
    // pub chain_id:  Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct GasLessTransactionData {
    pub kind: TransactionKind,
    pub sender: SuiAddress,
}

#[derive(Debug)]
enum ChainIdType {
    Number,    // Represents a numeric input
    HexString, // Represents a hex string input
}

#[derive(Debug)]
enum ChainIdResult {
    U256(U256),
    String(String),
}

#[derive(Clone)]
pub struct DWalletBinderCostParams {
    /// Base cost for invoking the `verify_eth_state` function.
    pub create_authority_ack_transaction_cost_base: InternalGas,
}

/***************************************************************************************************
* native fun create_authority_ack_transaction
* Implementation of the Move native function
* `create_authority_ack_transaction(
*  binder_id: vector<u8>,
*  dwallet_cap_id: vector<u8>,
*  bind_to_authority_id: vector<u8>,
*  bind_to_authority_nonce: u64,
*  virgin_bound: bool,
*  chain_id: vector<u8>,
*  domain_name: vector<u8>,
*  domain_version: vector<u8>,
*  contract_address: vector<u8>,
*  chain_id_type: u8) -> vector<u8>;`
* gas cost: create_authority_ack_transaction_cost_base | base cost for function call and fixed operations.
**************************************************************************************************/
pub fn create_authority_ack_transaction(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    // Load the cost parameters from the protocol config.
    let create_authority_ack_transaction_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .authority_binder
        .clone();

    // Charge the base cost for this operation.
    native_charge_gas_early_exit!(
        context,
        create_authority_ack_transaction_cost_params.create_authority_ack_transaction_cost_base
    );

    let cost = context.gas_used();

    let (
        chain_type,
        contract_address,
        domain_version,
        domain_name,
        chain_id,
        virgin_bound,
        bind_to_authority_nonce,
        bind_to_authority_id,
        dwallet_cap_id,
        binder_id,
    ) = (
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, bool),
        pop_arg!(args, u64),
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
    );

    let domain_name = String::from_utf8(domain_name).unwrap();
    let domain_version = String::from_utf8(domain_version).unwrap();
    let contract_address = Address::from_slice(&contract_address);
    let chain_type = String::from_utf8(chain_type).unwrap();

    let chain_id = parse_chain_id(&chain_type, chain_id)?;

    let result_bytes = match chain_type.as_str() {
        "Ethereum" => create_ethereum_binder_ack(
            binder_id,
            dwallet_cap_id,
            bind_to_authority_id,
            bind_to_authority_nonce,
            virgin_bound,
            chain_id,
            domain_name,
            domain_version,
            contract_address,
        )?,
        "Sui" => create_sui_binder_ack(
            binder_id,
            dwallet_cap_id,
            bind_to_authority_id,
            bind_to_authority_nonce,
            virgin_bound,
            chain_id,
        )?,
        _ => {
            return Err(PartialVMError::new(
                StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
            ))
        }
    };

    Ok(NativeResult::ok(
        cost,
        smallvec![Value::vector_u8(result_bytes)],
    ))
}

fn create_ethereum_binder_ack(
    binder_id: Vec<u8>,
    dwallet_cap_id: Vec<u8>,
    bind_to_authority_id: Vec<u8>,
    bind_to_authority_nonce: u64,
    virgin_bound: bool,
    chain_id: ChainIdResult,
    domain_name: String,
    domain_version: String,
    contract_address: H160,
) -> PartialVMResult<Vec<u8>> {
    let chain_id_inner = match chain_id {
        ChainIdResult::U256(chain_id) => chain_id,
        _ => {
            return Err(
                PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                    .with_message("chain_id is not U256".to_string()),
            );
        }
    };

    let domain = EIP712Domain {
        name: Some(domain_name),
        version: Some(domain_version),
        chain_id: Some(chain_id_inner),
        verifying_contract: Some(contract_address),
        salt: None,
    };

    let dwallet_binder = EthDWalletBinder {
        id: binder_id,
        dwallet_cap: dwallet_cap_id,
        bind_to_authority: bind_to_authority_id,
        nonce: bind_to_authority_nonce,
        virgin_bound,
    };

    let binder_with_domain = EIP712WithDomain::<EthDWalletBinder>::new(dwallet_binder)
        .unwrap()
        .set_domain(domain);

    let domain_separator = binder_with_domain.domain_separator().map_err(|_| {
        PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
            .with_message("domain separator error".to_string())
    })?;
    let struct_hash = binder_with_domain.struct_hash().map_err(|_| {
        PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
            .with_message("struct hash error".to_string())
    })?;

    let digest = [&[0x19, 0x01], &domain_separator[..], &struct_hash[..]].concat();
    Ok(digest)
}

fn parse_chain_id(chain_type: &String, chain_id: Vec<u8>) -> Result<ChainIdResult, PartialVMError> {
    let chain_id_type = match chain_type.as_str() {
        "Ethereum" => ChainIdType::Number,
        "Sui" => ChainIdType::HexString,
        _ => {
            return Err(
                PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                    .with_message("chain_id type is not supported".to_string()),
            );
        }
    };
    match chain_id_type {
        ChainIdType::Number => Ok(ChainIdResult::U256(U256::from_big_endian(&chain_id))),
        ChainIdType::HexString => Ok(ChainIdResult::String(
            String::from_utf8(chain_id.clone()).unwrap_or(hex::encode(chain_id)),
        )),
    }
}

fn create_sui_binder_ack(
    binder_id: Vec<u8>,
    dwallet_cap_id: Vec<u8>,
    bind_to_authority_id: Vec<u8>,
    bind_to_authority_nonce: u64,
    virgin_bound: bool,
    chain_id: ChainIdResult,
) -> PartialVMResult<Vec<u8>> {
    let mut tx_builder = ProgrammableTransactionBuilder::new();

    let package_id = "0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec";

    pub const SUI_DWALLET_CAP_MODULE_NAME: &IdentStr = ident_str!("dwallet_cap");
    pub const SUI_DWALLET_CAP_BIND_FUNCTION_NAME: &IdentStr = ident_str!("approve_dwallet_binder");
    pub const SUI_FULLNODE_URL: &str = "sui testnet url - update";

    // todo(yuval): is there a way to verify this in the contract? like domain in eth
    let chain_id_inner = match chain_id {
        ChainIdResult::String(chain_id) => chain_id,
        _ => {
            return Err(
                PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                    .with_message("chain_id is not String".to_string()),
            );
        }
    };

    let binder_id_arg = tx_builder
        .pure(bcs::to_bytes(&binder_id).unwrap())
        .map_err(|_| {
            PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                .with_message("binder_id c error".to_string())
        })?;
    let dwallet_cap_id_arg = tx_builder
        .pure(bcs::to_bytes(&dwallet_cap_id).unwrap())
        .map_err(|_| {
            PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                .with_message("dwallet_cap_id serialization error".to_string())
        })?;
    let bind_to_authority_id_arg = tx_builder
        .pure(bcs::to_bytes(&bind_to_authority_id).unwrap())
        .map_err(|_| {
            PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                .with_message("bind_to_authority_id serialization error".to_string())
        })?;
    let nonce_arg = tx_builder
        .pure(bcs::to_bytes(&bind_to_authority_nonce).unwrap())
        .map_err(|_| {
            PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                .with_message("nonce serialization error".to_string())
        })?;

    let virgin_bound_arg = tx_builder
        .pure(bcs::to_bytes(&virgin_bound).unwrap())
        .map_err(|_| {
            PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                .with_message("virgin_bound serialization error".to_string())
        })?;

    tx_builder.programmable_move_call(
        ObjectID::from_address(AccountAddress::from_str(package_id).unwrap()),
        SUI_DWALLET_CAP_MODULE_NAME.into(),
        SUI_DWALLET_CAP_BIND_FUNCTION_NAME.into(),
        vec![], // type args
        vec![
            binder_id_arg,
            dwallet_cap_id_arg,
            bind_to_authority_id_arg,
            nonce_arg,
            virgin_bound_arg,
        ],
    );

    let programmable_transaction = tx_builder.finish();

    let gasless_tx_data = GasLessTransactionData {
        kind: TransactionKind::ProgrammableTransaction(programmable_transaction.clone()),
        //todo(yuval): put the real sender address here
        sender: SuiAddress::from_str(
            "0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec",
        )
        .unwrap(),
    };
    // let tx = TransactionDataV1 {
    //     kind: TransactionKind::ProgrammableTransaction(programmable_transaction),
    //     expiration: 0.into(),
    //     sender: SuiAddress::from_str("0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec").unwrap(),
    //     gas_data: None,
    // };
    // let serialized = tx.is_sponsored_tx()
    // todo(yuval): make sure that this can be serde correctly.
    // let gasless_tx_bytes = bcs::to_bytes(&gasless_tx_data).unwrap();
    // let gasless_tx_data: GasLessTransactionData = bcs::from_bytes(&gasless_tx_bytes).unwrap();
    //
    // let serialized_tx_kind = bcs::to_bytes(&gasless_tx_data.kind).unwrap();
    Ok(bcs::to_bytes(&gasless_tx_data.kind).unwrap())
}
