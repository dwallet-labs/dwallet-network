// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::NativesCostTable;
use bytecode_interpreter_crypto::sha2_256_of;
use ethers::core::types::transaction::eip712::{EIP712WithDomain, Eip712};
use ethers::prelude::transaction::eip712::EIP712Domain;
use ethers::prelude::{Address, Eip712, EthAbiType, H160, U256};
use ethers::utils::hex;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
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
use sui_types::transaction::{TransactionDataAPI, TransactionKind};

/// Bind a `dwallet::DWalletCap` to an authority.
#[derive(Clone, Debug, Serialize, Deserialize, Eip712, EthAbiType)]
pub struct EthDWalletBinder {
    pub id: Vec<u8>,
    pub dwallet_cap: Vec<u8>,
    pub bind_to_authority: Vec<u8>,
    pub virgin_bound: bool,
    pub nonce: u64,
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
*  chain_type: u8) -> u8;`
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
        mut bind_to_authority_id,
        mut dwallet_cap_id,
        mut binder_id,
    ) = (
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, u64),
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

    let result_bytes = match chain_type.as_str() {
        "Ethereum" => create_ethereum_binder_ack(
            binder_id.clone(),
            dwallet_cap_id.clone(),
            bind_to_authority_id.clone(),
            bind_to_authority_nonce,
            virgin_bound,
            chain_id,
            domain_name,
            domain_version,
            contract_address,
        )?,
        "Sui" => create_sui_binder_ack(
            &mut binder_id,
            &mut dwallet_cap_id,
            &mut bind_to_authority_id,
            bind_to_authority_nonce,
            virgin_bound,
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
    chain_id: u64,
    domain_name: String,
    domain_version: String,
    contract_address: H160,
) -> PartialVMResult<Vec<u8>> {
    let domain = EIP712Domain {
        name: Some(domain_name),
        version: Some(domain_version),
        chain_id: Some(U256::from(chain_id)),
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

fn create_sui_binder_ack(
    binder_id: &mut Vec<u8>,
    dwallet_cap_id: &mut Vec<u8>,
    bind_to_authority_id: &mut Vec<u8>,
    bind_to_authority_nonce: u64,
    virgin_bound: bool,
) -> PartialVMResult<Vec<u8>> {
    let mut transaction_data = Vec::<u8>::new();
    transaction_data.append(binder_id);
    transaction_data.append(dwallet_cap_id);
    transaction_data.append(bind_to_authority_id);
    transaction_data.append(&mut bcs::to_bytes(&bind_to_authority_nonce).unwrap());
    transaction_data.append(&mut bcs::to_bytes(&virgin_bound).unwrap());

    Ok(sha2_256_of(&transaction_data))
}
