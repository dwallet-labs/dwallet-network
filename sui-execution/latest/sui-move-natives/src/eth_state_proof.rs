// Copyright (c) 2024 dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use ethers::prelude::EIP1186ProofResponse;
use ethers::{
    types::U256,
    utils::{hex, keccak256},
};
use helios::config::networks::Network;
use helios::consensus::ConsensusStateManager;
use helios::consensus::{nimbus_rpc::NimbusRpc, Bytes32, ConsensusRpc};
use helios::dwallet::encode_account;
use helios::execution;
use helios::execution::{get_message_storage_slot, verify_proof};
use helios::prelude::{AggregateUpdates, FinalityUpdate, OptimisticUpdate, Update};
use helios::types::Address;
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
use tracing::log::error;

use crate::object_runtime::ObjectRuntime;
use crate::NativesCostTable;

/// The value of `True` in the contract's storage map.
const TRUE_VALUE: u8 = 1;

#[derive(Clone)]
pub struct EthDWalletCostParams {
    /// Base cost for invoking the `verify_eth_state` function.
    pub verify_eth_state_cost_base: InternalGas,
    /// Base cost for invoking the `verify_message_proof` function.
    pub verify_message_proof_cost_base: InternalGas,
    /// Base cost for invoking the `create_initial_eth_state_data` function.
    pub create_initial_eth_state_data_cost_base: InternalGas,
}

/***************************************************************************************************
* native fun verify_message_proof
* Implementation of the Move native function
* `verify_message_proof(p0: &mut NativeContext, p1: Vec<Type>, p2: VecDeque<Value>)
* -> PartialVMResult<NativeResult>`
* gas cost: verify_message_proof_cost_base | base cost for function call and fixed operations.
**************************************************************************************************/
pub fn verify_message_proof(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    // Load the cost parameters from the protocol config.
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

    let (contract_address, state_root, map_slot, dwallet_id, message, proof) = (
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, u64),
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
    );

    let proof = bcs::from_bytes::<EIP1186ProofResponse>(proof.as_slice())
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let message = String::from_utf8(message.clone())
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let message_map_index = get_message_storage_slot(message.clone(), dwallet_id.clone(), map_slot)
        .map_err(|_| PartialVMError::new(StatusCode::UNKNOWN_STATUS))?;

    let contract_address = Address::from_slice(contract_address.as_slice());
    let state_root = Bytes32::try_from(state_root.as_slice())
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let account_path = keccak256(contract_address.as_bytes()).to_vec();
    let account_encoded = encode_account(&proof);

    // Verify the account proof against the state root.
    let is_valid_account_proof = verify_proof(
        &proof.clone().account_proof,
        &state_root.as_slice().to_vec(),
        &account_path,
        &account_encoded,
    );

    if !is_valid_account_proof {
        return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)]));
    }

    // Get only the proof that matches the message_map_index.
    let msg_storage_proof = proof
        .storage_proof
        .iter()
        .find(|p| p.key == U256::from(message_map_index.as_bytes()))
        .ok_or_else(|| PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR))?;
    // Cast the storage key to a 32-byte array, and hash using keccak256 algorithm.
    let mut msg_storage_proof_key_bytes = [0u8; 32];
    msg_storage_proof
        .key
        .to_big_endian(&mut msg_storage_proof_key_bytes);

    let storage_key_hash = keccak256(msg_storage_proof_key_bytes);
    let storage_value = [TRUE_VALUE].to_vec();

    let is_valid = verify_proof(
        &msg_storage_proof.clone().proof,
        &proof.storage_hash.as_bytes().to_vec(),
        &storage_key_hash.to_vec(),
        &storage_value,
    );
    Ok(NativeResult::ok(cost, smallvec![Value::bool(is_valid)]))
}

/***************************************************************************************************
 * native fun verify_eth_state
 * Implementation of the Move native function
 * `eth_dwallet::verify_eth_state(updates: vector<u8>,
 * finality_update: vector<u8>,
 * optimistic_update: vector<u8>,
 * eth_state: vector<u8>) -> (vector<u8>, u64, vector<u8>, vector<u8>);`
 * gas cost: verify_eth_state_cost_base   | base cost for function call and fixed operations.
 **************************************************************************************************/
pub(crate) fn verify_eth_state(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    // Load the cost parameters from the protocol config
    let verify_eth_state_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .eth_state_proof
        .clone();

    // Charge the base cost for this operation.
    native_charge_gas_early_exit!(
        context,
        verify_eth_state_cost_params.verify_eth_state_cost_base
    );

    let cost = context.gas_used();
    let (current_eth_state, optimistic_update, finality_update, updates_vec) = (
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
    );

    let mut eth_state =
        serde_json::from_slice::<ConsensusStateManager<NimbusRpc>>(current_eth_state.as_slice())
            .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let updates_vec = serde_json::from_slice::<Vec<Update>>(updates_vec.as_slice())
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let finality_update = serde_json::from_slice::<FinalityUpdate>(finality_update.as_slice())
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let optimistic_update =
        serde_json::from_slice::<OptimisticUpdate>(optimistic_update.as_slice())
            .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let updates = AggregateUpdates {
        updates: updates_vec,
        finality_update,
        optimistic_update,
    };

    eth_state.verify_and_apply_updates(&updates).map_err(|e| {
        error!("failed to verify updates: {:?}", e);
        PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
    })?;

    let state_root = eth_state.clone().get_finalized_state_root().to_vec();
    let new_state_bcs = bcs::to_bytes(&eth_state)
        .map_err(|_| PartialVMError::new(StatusCode::VALUE_SERIALIZATION_ERROR))?;
    let slot = u64::from(eth_state.clone().get_latest_slot());
    let network = eth_state
        .get_network()
        .map_err(|_| PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR))?
        .to_string()
        .as_bytes()
        .to_vec();

    Ok(NativeResult::ok(
        cost,
        smallvec![
            Value::vector_u8(new_state_bcs),
            Value::u64(slot),
            Value::vector_u8(state_root),
            Value::vector_u8(network),
        ],
    ))
}

/***************************************************************************************************
* native fun create_initial_eth_state_data
* Implementation of the Move native function
* `eth_dwallet::create_initial_eth_state_data(checkpoint: vector<u8>): vector<u8>;`
* gas cost:
* create_initial_eth_state_data_cost_base | base cost for function call and fixed operations.
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

    // hardcoded hashes for verifying network states
    const MAINNET_STATE_HASH: &str =
        "9fb325c6f66a0f98b57f4b8117c193982c622ee4eb0f6373c84cfc46821091de";
    const HOLESKY_STATE_HASH: &str =
        "e418e4c236fcb1b13282f23346f8c4b14af29cce5ad843f27ed565fd00d49269";

    let cost = context.gas_used();

    let network = pop_arg!(args, Vector).to_vec_u8()?;
    let state_bytes = pop_arg!(args, Vector).to_vec_u8()?;
    let state_bytes_hash = hex::encode(keccak256(state_bytes.clone()));
    let is_valid = match network.as_slice() {
        b"mainnet" => state_bytes_hash == MAINNET_STATE_HASH,
        b"holesky" => state_bytes_hash == HOLESKY_STATE_HASH,
        _ => false,
    };

    if !is_valid {
        return Ok(NativeResult::ok(cost, smallvec![Value::vector_u8(vec![])]));
    }

    let eth_state = bcs::from_bytes::<ConsensusStateManager<NimbusRpc>>(&state_bytes)
        .map_err(|_| PartialVMError::new(StatusCode::VALUE_SERIALIZATION_ERROR))?;

    let state_root = eth_state.clone().get_finalized_state_root().to_vec();
    let time_slot = eth_state.get_latest_slot();
    Ok(NativeResult::ok(
        cost,
        smallvec![
            Value::vector_u8(state_bytes),
            Value::vector_u8(state_root),
            Value::u64(time_slot.as_u64())
        ],
    ))
}
