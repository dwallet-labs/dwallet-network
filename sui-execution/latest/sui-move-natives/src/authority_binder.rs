// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::NativesCostTable;
use ethers::core::types::transaction::eip712::{EIP712WithDomain, Eip712};
use ethers::prelude::transaction::eip712::EIP712Domain;
use ethers::prelude::{Address, Eip712, EthAbiType, U256};
use ethers::utils::hex;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::gas_algebra::InternalGas;
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

/// Bind a `dwallet::DWalletCap` to an authority.
#[derive(Clone, Debug, Serialize, Deserialize, Eip712, EthAbiType)]
pub struct DWalletBinder {
    // #[serde(rename = "id")]
    pub id: Vec<u8>,
    // #[serde(rename = "dwalletCap")]
    pub dwallet_cap: Vec<u8>,
    // #[serde(rename = "bindToAuthority")]
    pub bind_to_authority: Vec<u8>,
    // #[serde(rename = "virginBound")]
    pub virgin_bound: bool,
    // #[serde(rename = "nonce")]
    pub nonce: u64,
}

#[derive(Debug)]
enum ChainIdError {
    InvalidFormat,
    UnknownInvariantViolation,
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
pub struct AuthorityBinderCostParams {
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
        chain_id_type,
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
        pop_arg!(args, u8),
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

    let chain_id_type = match chain_id_type {
        0 => ChainIdType::Number,
        1 => ChainIdType::HexString,
        _ => {
            return Err(PartialVMError::new(
                StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
            ))
        }
    };

    let chain_id = parse_chain_id(chain_id_type, chain_id)
        .map_err(|_| PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR))?;

    let domain_name = String::from_utf8(domain_name).unwrap();
    let domain_version = String::from_utf8(domain_version).unwrap();
    let contract_address = Address::from_slice(&contract_address);

    // todo(yuval): need to implement the differentiation between network types.
    // for example, SUI would use the chain_id as a string, while Ethereum would use it as a number.
    let chain_id_inner = match chain_id {
        ChainIdResult::U256(chain_id) => chain_id,
        ChainIdResult::String(_) => {
            return Err(PartialVMError::new(
                StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
            ))
        }
    };

    let domain = EIP712Domain {
        name: Some(domain_name),
        version: Some(domain_version),
        chain_id: Some(U256::from(chain_id)),
        verifying_contract: Some(contract_address),
        salt: None,
    };

    let dwallet_binder = DWalletBinder {
        id: binder_id,
        dwallet_cap: dwallet_cap_id,
        bind_to_authority: bind_to_authority_id,
        nonce: bind_to_authority_nonce,
        virgin_bound,
    };

    let binder_with_domain = EIP712WithDomain::<DWalletBinder>::new(dwallet_binder)
        .unwrap()
        .set_domain(domain);

    let domain_separator = binder_with_domain
        .domain_separator()
        .map_err(|_| PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR))?;
    let struct_hash = binder_with_domain
        .struct_hash()
        .map_err(|_| PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR))?;

    let digest_input = [&[0x19, 0x01], &domain_separator[..], &struct_hash[..]].concat();
    Ok(NativeResult::ok(
        cost,
        smallvec![Value::vector_u8(digest_input)],
    ))
}

fn parse_chain_id(
    chain_id_type: ChainIdType,
    chain_id: Vec<u8>,
) -> Result<ChainIdResult, ChainIdError> {
    match chain_id_type {
        ChainIdType::Number => Ok(ChainIdResult::U256(U256::from_big_endian(&chain_id))),
        ChainIdType::HexString => Ok(ChainIdResult::String(
            String::from_utf8(chain_id).map_err(|_| ChainIdError::InvalidFormat)?,
        )),
    }
}
