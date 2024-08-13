// Copyright (c) 2024 dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::VecDeque;
use std::str::FromStr;

use ethers::utils::{hex, keccak256};
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
    todo!();
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
    todo!();
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
    todo!();
}

fn process_eth_state_updates(
    eth_state: &mut ConsensusStateManager<NimbusRpc>,
    updates: AggregateUpdates,
) -> Result<(Vec<u8>, u64), PartialVMError> {
    todo!();
}
