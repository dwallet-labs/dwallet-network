// Copyright (c) 2024 dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use ethers::prelude::EIP1186ProofResponse;
use ethers::{
    types::U256,
    utils::{hex, keccak256},
};
use helios::config::networks::Network;
use helios::consensus::{
    nimbus_rpc::NimbusRpc, BeaconBlockBody, BeaconBlockBodyBellatrix, BeaconBlockBodyCapella,
    BeaconBlockBodyDeneb, BeaconBlockBodyWrapper, BeaconBlockType, Bytes32, ConsensusRpc,
    ExecutionPayload, ExecutionPayloadBellatrix, ExecutionPayloadCapella, ExecutionPayloadDeneb,
    ExecutionPayloadWrapper,
};
use helios::consensus::{BeaconBlock, ConsensusStateManager};
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
    pop_arg, values,
    values::{Value, Vector},
};
use smallvec::smallvec;
use ssz_rs::Merkleized;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::result;
use std::str::FromStr;
use tracing::log::error;
use tracing::Instrument;

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
* `verify_message_proof(
*  proof: vector<u8>,
*  message: vector<u8>,
*  dwallet_id: vector<u8>,
*  data_slot: u64,
*  contract_address: vector<u8>,
*  state_root: vector<u8>) -> bool;`
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

    let (state_root, contract_address, map_slot, dwallet_id, message, proof) = (
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, u64),
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
    );

    let proof: EIP1186ProofResponse = serde_json::from_slice(&proof)
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let message = String::from_utf8(message.clone())
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let message_map_index = get_message_storage_slot(message.clone(), dwallet_id.clone(), map_slot)
        .map_err(|_| PartialVMError::new(StatusCode::UNKNOWN_STATUS))?;

    let contract_address = String::from_utf8(contract_address.clone())
        .unwrap_or(hex::encode(contract_address.clone()));

    let contract_address: Address = contract_address
        .parse()
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let account_path = keccak256(contract_address.as_bytes()).to_vec();
    let account_encoded = encode_account(&proof);

    // Verify the account proof against the state root.
    let is_valid_account_proof = verify_proof(
        &proof.clone().account_proof,
        &state_root,
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
 * eth_state: vector<u8>,
 * beacon_block: vector<u8>,
 * beacon_block_body: vector<u8>,
 * beacon_block_execution_payload: vector<u8>,
 * beacon_block_type: vector<u8>) -> (vector<u8>, u64, vector<u8>, vector<u8>);`
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
    let (
        beacon_block_type,
        beacon_block_execution_payload,
        beacon_block_body,
        beacon_block,
        current_eth_state,
        optimistic_update,
        finality_update,
        updates_vec,
    ) = (
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
        pop_arg!(args, Vector).to_vec_u8()?,
    );

    let mut eth_state =
        bcs::from_bytes::<ConsensusStateManager<NimbusRpc>>(current_eth_state.as_slice())
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

    eth_state.advance_state(&updates).map_err(|e| {
        error!("failed to advance state: {:?}", e);
        PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
    })?;

    let beacon_block = verify_beacon_block(
        beacon_block_type,
        beacon_block_execution_payload,
        beacon_block_body,
        beacon_block,
        &mut eth_state,
    )?;

    // Get the Execution layer's state root from the verified beacon block.
    let state_root = beacon_block.body.execution_payload().state_root();
    let block_number = beacon_block.body.execution_payload().block_number();

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
        smallvec![Value::struct_(values::Struct::pack(vec![
            Value::vector_u8(new_state_bcs),
            Value::u64(slot),
            Value::vector_u8(network),
            Value::vector_u8(state_root.as_slice().to_vec()),
            Value::u64(block_number.as_u64())
        ]))],
    ))
}

/***************************************************************************************************
* native fun create_initial_eth_state_data
* Implementation of the Move native function
* `eth_dwallet::create_initial_eth_state_data(
* state_bytes: vector<u8>,
* network: vector<u8>,
* updates_vec: vector<u8>,
* finality_update: vector<u8>,
* optimistic_update: vector<u8>,
* beacon_block: vector<u8>,
* beacon_block_body: vector<u8>,
* beacon_block_execution_payload: vector<u8>,
* beacon_block_type: vector<u8>,
* ) -> (vector<u8>, u64, vector<u8>);`
* gas cost:
* create_initial_eth_state_data_cost_base | base cost for function call and fixed operations.
**************************************************************************************************/
pub(crate) fn create_initial_eth_state_data(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    // Load the cost parameters from the protocol config
    let create_initial_eth_state_data_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .eth_state_proof
        .clone();

    // Charge the base cost for this operation.
    native_charge_gas_early_exit!(
        context,
        create_initial_eth_state_data_cost_params.create_initial_eth_state_data_cost_base
    );

    // hardcoded hashes for verifying network states
    const MAINNET_STATE_HASH: &str =
        "37e9ce4f5764c4709724c256304e151e0af18662b22b1f3b5d916f878eb3ce0d";
    const HOLESKY_STATE_HASH: &str =
        "f0c6cc1e9a7d2516659fea19f84fc553ffda5067355a15dcacb78535bbaf7366";

    let cost = context.gas_used();

    let beacon_block_type = pop_arg!(args, Vector).to_vec_u8()?;
    let beacon_block_execution_payload = pop_arg!(args, Vector).to_vec_u8()?;
    let beacon_block_body = pop_arg!(args, Vector).to_vec_u8()?;
    let beacon_block = pop_arg!(args, Vector).to_vec_u8()?;
    let optimistic_update = pop_arg!(args, Vector).to_vec_u8()?;
    let finality_update = pop_arg!(args, Vector).to_vec_u8()?;
    let updates_vec = pop_arg!(args, Vector).to_vec_u8()?;
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

    let mut eth_state = bcs::from_bytes::<ConsensusStateManager<NimbusRpc>>(&state_bytes)
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

    eth_state
        .verify_and_apply_initial_updates(&updates)
        .map_err(|e| {
            error!("failed to verify updates: {:?}", e);
            PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
        })?;

    let beacon_block = verify_beacon_block(
        beacon_block_type,
        beacon_block_execution_payload,
        beacon_block_body,
        beacon_block,
        &eth_state,
    )?;

    // Get the Execution layer's state root from the verified beacon block.
    let state_root = beacon_block.body.execution_payload().state_root();
    let block_number = beacon_block.body.execution_payload().block_number();

    let state_bytes = bcs::to_bytes(&eth_state)
        .map_err(|_| PartialVMError::new(StatusCode::VALUE_SERIALIZATION_ERROR))?;
    let time_slot = eth_state.get_latest_slot();
    Ok(NativeResult::ok(
        cost,
        smallvec![Value::struct_(values::Struct::pack(vec![
            Value::vector_u8(state_bytes),
            Value::u64(time_slot.as_u64()),
            Value::vector_u8(state_root.as_slice().to_vec()),
            Value::u64(block_number.as_u64())
        ]))],
    ))
}

fn verify_beacon_block(
    beacon_block_type: Vec<u8>,
    beacon_block_execution_payload: Vec<u8>,
    beacon_block_body: Vec<u8>,
    beacon_block: Vec<u8>,
    eth_state: &ConsensusStateManager<NimbusRpc>,
) -> PartialVMResult<BeaconBlock> {
    let mut beacon_block: BeaconBlock =
        serde_json::from_str(&String::from_utf8(beacon_block).unwrap())
            .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let beacon_block_type = String::from_utf8(beacon_block_type.clone())
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let beacon_block_type: BeaconBlockType = BeaconBlockType::from_str(&beacon_block_type)
        .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;

    let mut beacon_block_body =
        BeaconBlockBodyWrapper::new_from_json(beacon_block_body, &beacon_block_type)
            .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;
    let beacon_block_body: BeaconBlockBody = beacon_block_body.inner();

    let beacon_block_execution_payload =
        ExecutionPayloadWrapper::new_from_json(beacon_block_execution_payload, &beacon_block_type)
            .map_err(|_| PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT))?;
    let beacon_block_execution_payload: ExecutionPayload = beacon_block_execution_payload.inner();

    let beacon_block_body = BeaconBlockBody::new_from_existing_with_execution_payload(
        beacon_block_body,
        beacon_block_execution_payload,
    );
    beacon_block.body = beacon_block_body;

    // Compute the root hash of the Merkle tree using the unverified (user-specified) beacon block.
    let beacon_block_hash = beacon_block
        .hash_tree_root()
        .map_err(|_| PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR))?;

    let mut verified_block_header = eth_state.clone().get_finalized_header();

    // Compute the root hash of the Merkle tree using the verified beacon header.
    let verified_block_hash = verified_block_header
        .hash_tree_root()
        .map_err(|_| PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR))?;

    // Verify the beacon block hash against the verified block hash.
    // If the hashes match, it means we can trust the beacon block, and use its state
    // root to verify the proof.
    // More info in [Ethereum Consensus Specs](https://github.com/ethereum/consensus-specs/blob/fa09d896484bbe240334fa21ffaa454bafe5842e/ssz/simple-serialize.md#summaries-and-expansions).
    if beacon_block_hash != verified_block_hash {
        return Err(PartialVMError::new(
            StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
        ));
    }

    // Verify that the `body_root` of the verified beacon block matches the actual root hash
    // of the beacon block body.
    let beacon_block_body_root = beacon_block
        .body
        .hash_tree_root()
        .map_err(|_| PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR))?;
    let beacon_block_body_root = Bytes32::try_from(beacon_block_body_root.as_ref())
        .map_err(|_| PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR))?;

    if verified_block_header.body_root != beacon_block_body_root {
        return Err(PartialVMError::new(
            StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
        ));
    }
    Ok(beacon_block)
}
