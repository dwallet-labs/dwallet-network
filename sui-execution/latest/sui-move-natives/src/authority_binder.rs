// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// todo(yuval): doc all

use crate::NativesCostTable;
use ethers::core::types::transaction::eip712::{EIP712WithDomain, Eip712};
use ethers::prelude::transaction::eip712::EIP712Domain;
use ethers::prelude::{Address, Eip712, EthAbiType, U256};
use move_binary_format::errors::PartialVMResult;
use move_core_types::gas_algebra::InternalGas;
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
#[eip712(
    // These are placeholders; actual domain values will be supplied dynamically.
    name = "Placeholder",
    version = "1",
    chain_id = 1,
    verifying_contract = "0x0000000000000000000000000000000000000000"
)]
pub struct DWalletBinder {
    #[serde(rename = "id")]
    pub id: Vec<u8>,
    #[serde(rename = "dwalletCap")]
    pub dwallet_cap: Vec<u8>,
    #[serde(rename = "bindToAuthority")]
    pub bind_to_authority: Vec<u8>,
    #[serde(rename = "virginBound")]
    pub virgin_bound: bool,
    #[serde(rename = "nonce")]
    pub nonce: u64,
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
*  state_root: vector<u8>) -> vector<u8>;`
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

    let typed_data_hash = binder_with_domain.encode_eip712().unwrap();
    Ok(NativeResult::ok(
        cost,
        smallvec![Value::vector_u8(typed_data_hash)],
    ))
}
