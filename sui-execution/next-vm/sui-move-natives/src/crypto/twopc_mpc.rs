// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::VecDeque;

use move_binary_format::errors::PartialVMResult;
use move_core_types::gas_algebra::InternalGas;
use move_vm_runtime::native_functions::NativeContext;
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, values::Value,
};

pub const INVALID_INPUT: u64 = 0;

#[derive(Clone)]
pub struct TwoPCMPCDKGCostParams {
    /// Base cost
    /// for invoking the `dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share`
    /// function.
    pub dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base:
        InternalGas,
    /// Base cost for invoking the `sign_verify_encrypted_signature_parts_prehash` function.
    pub sign_verify_encrypted_signature_parts_prehash_cost_base: InternalGas,
}
/***************************************************************************************************
 * native fun dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share
 * Implementation of the Move native function `dwallet_2pc_mpc_ecdsa_k1::dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(commitment_to_centralized_party_secret_key_share: vector<u8>, secret_key_share_encryption_and_proof: vector<u8>, centralized_party_public_key_share_decommitment_and_proofs: vector<u8>): (vector<u8>, vector<u8>);`
 *   gas cost: dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base   | base cost for function call and fixed operations.
 **************************************************************************************************/
pub fn dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut _args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    todo!()
}

/***************************************************************************************************
 * native fun sign_verify_encrypted_signature_parts_prehash
 * Implementation of the Move native function `dwallet_2pc_mpc_ecdsa_k1::sign_verify_encrypted_signature_parts_prehash(commitment_to_centralized_party_secret_key_share: vector<u8>, secret_key_share_encryption_and_proof: vector<u8>, centralized_party_public_key_share_decommitment_and_proofs: vector<u8>): (vector<u8>, vector<u8>, vector<u8>);`
 *   gas cost: sign_verify_encrypted_signature_parts_prehash_cost_base   | base cost for function call and fixed operations.
 **************************************************************************************************/
pub fn sign_verify_encrypted_signature_parts_prehash(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut _args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    todo!()
}
