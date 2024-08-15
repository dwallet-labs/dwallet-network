// Copyright (c) 2024 dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::VecDeque;
use std::str::FromStr;

use ethers::{
    types::U256,
    utils::{hex, keccak256},
};
use helios::config::networks::Network;
use helios::consensus::ConsensusStateManager;
use helios::consensus::{nimbus_rpc::NimbusRpc, ConsensusRpc};
use helios::dwallet::light_client::ProofResponse;
use helios::execution::verify_proof;
use helios::prelude::{AggregateUpdates, FinalityUpdate, OptimisticUpdate, Update};
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_binary_format::file_format::StructFieldInformation::Native;
use move_core_types::{gas_algebra::InternalGas, vm_status::StatusCode};
use move_vm_runtime::{native_charge_gas_early_exit, native_functions::NativeContext};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Value, Vector},
};
use smallvec::smallvec;
use tracing::log::info;

use crate::object_runtime::ObjectRuntime;
use crate::NativesCostTable;

#[derive(Clone)]
pub struct EthDWalletCostParams {
    /// Base cost for invoking the `verify_eth_state` function
    pub verify_eth_state_cost_base: InternalGas,
    /// Base cost for invoking the `verify_message_proof` function
    pub verify_message_proof_cost_base: InternalGas,
    /// Base cost for invoking the `create_initial_eth_state_data` function
    pub create_initial_eth_state_data_cost_base: InternalGas,
}

/***************************************************************************************************
 * native fun verify_message_proof
 * Implementation of the Move native function `verify_message_proof(p0: &mut NativeContext, p1: Vec<Type>, p2: VecDeque<Value>) -> PartialVMResult<NativeResult>`
 * gas cost: verify_message_proof_cost_base   | base cost for function call and fixed operations.
 **************************************************************************************************/
pub fn verify_message_proof(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
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
    let map_slot = pop_arg!(args, u64);
    let dwallet_id = pop_arg!(args, Vector).to_vec_u8()?;
    let message = pop_arg!(args, Vector).to_vec_u8()?;
    let proof = pop_arg!(args, Vector).to_vec_u8()?;

    let Ok(proof) = bcs::from_bytes::<ProofResponse>(proof.as_slice()) else {
        return Ok(NativeResult::err(
            cost,
            StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT.into(),
        ));
    };

    // Verify the account proof against the state root.
    let is_valid_account_proof = verify_proof(
        &proof.account_proof.proof.as_slice(),
        &proof.account_proof.root,
        &proof.account_proof.path,
        &proof.account_proof.value,
    );

    if !is_valid_account_proof {
        return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)]));
    }

    let Ok(message) = String::from_utf8(message.clone()) else {
        return Ok(NativeResult::err(
            cost,
            StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT.into(),
        ));
    };
    let Ok(message_map_index) =
        get_message_storage_slot(message, dwallet_id.clone(), map_slot.clone())
    else {
        return Ok(NativeResult::err(
            cost,
            StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR.into(),
        ));
    };

    let message_map_index = U256::from(message_map_index.as_bytes());
    let mut msg_storage_proof_key_bytes = [0u8; 32];
    message_map_index.to_big_endian(&mut msg_storage_proof_key_bytes);
    let storage_key_hash = keccak256(msg_storage_proof_key_bytes);

    if storage_key_hash != proof.storage_proof.path.as_slice() {
        return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)]));
    }

    let is_valid = verify_proof(
        &proof.storage_proof.proof,
        &proof.storage_proof.root,
        &proof.storage_proof.path,
        &proof.storage_proof.value,
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
    _ty_args: Vec<Type>,
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
        verify_message_proof_cost_params.verify_eth_state_cost_base
    );

    let cost = context.gas_used();
    let (current_eth_state, optimistic_update, finality_update, updates_vec) = (
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
    );

    let Ok(mut eth_state) =
        serde_json::from_slice::<ConsensusStateManager<NimbusRpc>>(current_eth_state.as_slice())
    else {
        return Ok(NativeResult::err(
            cost,
            StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT.into(),
        ));
    };

    let Ok(updates_vec) = serde_json::from_slice::<Vec<Update>>(updates_vec.as_slice()) else {
        return Ok(NativeResult::err(
            cost,
            StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT.into(),
        ));
    };

    let Ok(finality_update) = serde_json::from_slice::<FinalityUpdate>(finality_update.as_slice())
    else {
        return Ok(NativeResult::err(
            cost,
            StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT.into(),
        ));
    };

    let Ok(optimistic_update) =
        serde_json::from_slice::<OptimisticUpdate>(optimistic_update.as_slice())
    else {
        return Ok(NativeResult::err(
            cost,
            StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT.into(),
        ));
    };

    let updates = AggregateUpdates {
        updates: updates_vec,
        finality_update,
        optimistic_update,
    };

    let Ok((new_state_bcs, slot)) = process_eth_state_updates(&mut eth_state, updates) else {
        return Ok(NativeResult::err(
            cost,
            StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR.into(),
        ));
    };

    Ok(NativeResult::ok(
        cost,
        smallvec![Value::vector_u8(new_state_bcs), Value::u64(slot)],
    ))
}

/***************************************************************************************************
 * native fun create_initial_eth_state_data
 * Implementation of the Move native function `eth_dwallet::create_initial_eth_state_data(checkpoint: vector<u8>): vector<u8>;`
 * gas cost: create_initial_eth_state_data_cost_base   | base cost for function call and fixed operations.
 **************************************************************************************************/
pub(crate) fn create_initial_eth_state_data(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
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
        verify_message_proof_cost_params.create_initial_eth_state_data_cost_base
    );

    let cost = context.gas_used();

    let (network, checkpoint) = (
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
    );

    let network = String::from_utf8(network)
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;
    let network = Network::from_str(&network)
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let eth_state = ConsensusStateManager::<NimbusRpc>::new_from_checkpoint(checkpoint, network);

    let Ok(eth_state_bytes) = bcs::to_bytes(&eth_state) else {
        return Ok(NativeResult::err(
            cost,
            StatusCode::VALUE_SERIALIZATION_ERROR.into(),
        ));
    };

    Ok(NativeResult::ok(
        cost,
        smallvec![Value::vector_u8(eth_state_bytes)],
    ))
}

fn process_eth_state_updates(
    eth_state: &mut ConsensusStateManager<NimbusRpc>,
    updates: AggregateUpdates,
) -> Result<(Vec<u8>, u64), PartialVMError> {
    eth_state.verify_and_apply_updates(&updates).map_err(|e| {
        info!("Failed to verify updates: {:?}", e);
        PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
    })?;

    let new_state_bytes = bcs::to_bytes(&eth_state)
        .map_err(|_| PartialVMError::new(StatusCode::VALUE_SERIALIZATION_ERROR))?;
    let slot = u64::from(eth_state.get_latest_slot());

    Ok((new_state_bytes, slot))
}
