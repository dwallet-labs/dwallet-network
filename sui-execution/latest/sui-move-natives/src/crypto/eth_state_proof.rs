// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::VecDeque;

use ethers::utils::keccak256;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::gas_algebra::InternalGas;
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::{native_charge_gas_early_exit, native_functions::NativeContext};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Value, Vector},
};
use sha3::Digest;
use smallvec::smallvec;

use sui_types::eth_dwallet::{eth_state::EthState, proof::*, update::UpdatesResponse};

use crate::object_runtime::ObjectRuntime;
use crate::NativesCostTable;

pub const INVALID_INPUT: u64 = 0;
pub const DESERIALIZATION_ERROR: u64 = 1;
#[derive(Clone)]
pub struct EthDWalletCostParams {
    /// Base cost for invoking the `verify_eth_state` function
    pub verify_eth_state_cost_base: InternalGas,
}
/***************************************************************************************************
 * native fun verify_eth_state
 * Implementation of the Move native function `eth_dwallet::verify_eth_state(proof: vector<vector<u8>>, proof_len: u64, root: vector<u8>, eth_smart_contract_addr: vector<u8>, eth_smart_contract_slot: u64, message: vector<u8>): bool;`
 * gas cost: verify_eth_state_cost_base   | base cost for function call and fixed operations.
 **************************************************************************************************/
pub fn verify_eth_state(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 7);

    // Load the cost parameters from the protocol config
    let verify_eth_state_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .eth_state_proof
        .clone();

    // Load the cost parameters from the protocol config
    let object_runtime = context.extensions().get::<ObjectRuntime>();
    // Charge the base cost for this operation.
    native_charge_gas_early_exit!(
        context,
        verify_eth_state_cost_params.verify_eth_state_cost_base
    );

    let cost = context.gas_used();
    let current_eth_state = pop_arg!(args, Vector).to_vec_u8()?;
    let updates = pop_arg!(args, Vector).to_vec_u8()?;
    let message = pop_arg!(args, Vector).to_vec_u8()?;
    let eth_smart_contract_slot = pop_arg!(args, u64);
    let dwallet_id = pop_arg!(args, Vector).to_vec_u8()?;
    let proof = pop_arg!(args, Vector).to_vec_u8()?;

    let eth_state = String::from_utf8(current_eth_state.into())
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;
    let mut eth_state = EthState::build_from_json(&eth_state)
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let proof = String::from_utf8(proof)
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;
    let proof: ProofResponse = serde_json::from_str(&proof)
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let updates = String::from_utf8(updates)
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;
    let updates: UpdatesResponse = serde_json::from_str(&updates)
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let is_valid = match eth_state.verify_updates(&updates) {
        Ok(valid) => true,
        Err(e) => false,
    };

    let storage_key = get_storage_key(message, dwallet_id, eth_smart_contract_slot)
        .map_err(|_| PartialVMError::new(StatusCode::ARITHMETIC_ERROR))?;
    let storage_value = [1].to_vec();
    // todo(zeev): no urgent, but need to check the relation to to proof.
    let path = keccak256(storage_key.as_bytes());
    let is_valid =
        is_valid && verify_proof(&proof.proof.as_slice(), &proof.root, &path, &storage_value);
    Ok(NativeResult::ok(cost, smallvec![Value::bool(is_valid)]))
}
