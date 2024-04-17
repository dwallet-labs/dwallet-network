// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::VecDeque;

use ethers::utils::keccak256;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::{gas_algebra::InternalGas, vm_status::StatusCode};
use move_vm_runtime::{native_charge_gas_early_exit, native_functions::NativeContext};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Value, Vector},
};
use sha3::Digest;
use smallvec::smallvec;
use tracing::log::info;

use sui_types::eth_dwallet::{eth_state::EthState, proof::*, update::UpdatesResponse};

use crate::NativesCostTable;
use crate::object_runtime::ObjectRuntime;
use crate::NativesCostTable;

pub const INVALID_INPUT: u64 = 0;
pub const DESERIALIZATION_ERROR: u64 = 1;
#[derive(Clone)]
pub struct EthDWalletCostParams {
    /// Base cost for invoking the `verify_eth_state` function
    pub verify_eth_state_cost_base: InternalGas,
    /// Base cost for invoking the `verify_message_proof` function
    pub verify_message_proof_cost_base: InternalGas,
}

/***************************************************************************************************
 * native fun verify_message_proof
 * Implementation of the Move native function `verify_message_proof(p0: &mut NativeContext, p1: Vec<Type>, p2: VecDeque<Value>) -> PartialVMResult<NativeResult>`
 * gas cost: verify_message_proof_cost_base   | base cost for function call and fixed operations.
 **************************************************************************************************/
pub fn verify_message_proof(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {

    // Load the cost parameters from the protocol config
    let verify_message_proof_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .eth_state_proof
        .clone();

    // Load the cost parameters from the protocol config
    let object_runtime = context.extensions().get::<ObjectRuntime>();
    // Charge the base cost for this operation.
    native_charge_gas_early_exit!(
        context,
        verify_message_proof_cost_params.verify_message_proof_cost_base
    );

    let cost = context.gas_used();
    let (message, eth_smart_contract_slot, dwallet_id, proof) = (
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, u64),
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
    );

    let proof: ProofResponse = bcs::from_bytes(proof.as_slice())
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let storage_key = get_storage_key(message, dwallet_id, eth_smart_contract_slot)
        .map_err(|_| PartialVMError::new(StatusCode::ARITHMETIC_ERROR))?;

    // Verify the account proof against the state root.
    let is_valid_account_proof = verify_proof(
        &proof.account_proof.proof.as_slice(),
        &proof.execution_state_root,
        &proof.account_proof.path,
        &proof.account_proof.value,
    );

    if !is_valid_account_proof {
        return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)]));
    }

    // Verify the storage proof, after making sure the account exist on the current state root.
    let storage_value = [1].to_vec();
    let path = keccak256(storage_key.as_bytes());
    let is_valid = verify_proof(
        &proof.storage_proof.proof.as_slice(),
        &proof.storage_proof.root,
        &path,
        &storage_value,
    );
    Ok(NativeResult::ok(cost, smallvec![Value::bool(is_valid)]))
}

/***************************************************************************************************
 * native fun verify_eth_state
 * Implementation of the Move native function `eth_dwallet::verify_eth_state(proof: vector<vector<u8>>, proof_len: u64, root: vector<u8>, eth_smart_contract_addr: vector<u8>, eth_smart_contract_slot: u64, message: vector<u8>): bool;`
 * gas cost: verify_eth_state_cost_base   | base cost for function call and fixed operations.
 **************************************************************************************************/
pub(crate) fn verify_eth_state(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    // Load the cost parameters from the protocol config
    let verify_message_proof_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .eth_state_proof
        .clone();

    // Charge the base cost for this operation.
    native_charge_gas_early_exit!(
        context,
        verify_message_proof_cost_params.verify_message_proof_cost_base
    );

    let cost = context.gas_used();
    let (current_eth_state, updates) = (
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
    );

    let (new_state_bcs, slot) = process_eth_state_updates(current_eth_state, updates)?;

    Ok(NativeResult::ok(
        cost,
        smallvec![Value::u64(slot), Value::vector_u8(new_state_bcs)],
    ))

}

fn process_eth_state_updates(
    current_eth_state: Vec<u8>,
    updates: Vec<u8>,
) -> Result<(Vec<u8>, u64), PartialVMError> {
    let mut eth_state: EthState = bcs::from_bytes(current_eth_state.as_slice())
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let updates: UpdatesResponse = bcs::from_bytes(updates.as_slice())
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let is_valid = eth_state.verify_updates(&updates).unwrap_or(false);

    let mut new_state_bcs = if is_valid {
        bcs::to_bytes(&eth_state)
            .map_err(|_| PartialVMError::new(StatusCode::VALUE_SERIALIZATION_ERROR))?
    } else {
        Vec::new()
    };

    let mut slot = if is_valid {
        u64::from(eth_state.finalized_header.slot)
    } else {
        0u64
    };

    Ok((new_state_bcs, slot))
}
