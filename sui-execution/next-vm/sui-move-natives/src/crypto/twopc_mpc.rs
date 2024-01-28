// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use crate::NativesCostTable;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::gas_algebra::InternalGas;
use move_vm_runtime::{native_charge_gas_early_exit, native_functions::NativeContext};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Value, Vector},
};
use smallvec::smallvec;
use std::collections::VecDeque;
use sui_types::messages_signature_mpc::{decentralized_party_dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share, DKGSignatureMPCCentralizedCommitment, DKGSignatureMPCCentralizedPublicKeyShareDecommitmentAndProof, DKGSignatureMPCSecretKeyShareEncryptionAndProof};
use crate::object_runtime::ObjectRuntime;

pub const INVALID_INPUT: u64 = 0;

#[derive(Clone)]
pub struct TwoPCMPCDKGCostParams {
    /// Base cost for invoking the `dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share` function
    pub dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base: InternalGas,
}
/***************************************************************************************************
 * native fun dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share
 * Implementation of the Move native function `dwallet_2pc_mpc_ecdsa_k1::dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(commitment_to_centralized_party_secret_key_share: vector<u8>, secret_key_share_encryption_and_proof: vector<u8>, centralized_party_public_key_share_decommitment_and_proofs: vector<u8>): (vector<u8>, vector<u8>, vector<u8>);`
 *   gas cost: dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base   | base cost for function call and fixed opers
 **************************************************************************************************/
pub fn dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 3);

    // Load the cost parameters from the protocol config
    let twopc_mpc_dkg_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .twopc_mpc_dkg_cost_params
        .clone();

    // Load the cost parameters from the protocol config
    let object_runtime = context
        .extensions()
        .get::<ObjectRuntime>();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        twopc_mpc_dkg_cost_params.dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base
    );

    let cost = context.gas_used();

    let centralized_party_public_key_share_decommitment_and_proof = pop_arg!(args, Vector);
    let centralized_party_public_key_share_decommitment_and_proof_ref = centralized_party_public_key_share_decommitment_and_proof.to_vec_u8()?;
    let Ok(centralized_party_public_key_share_decommitment_and_proof) = bcs::from_bytes::<DKGSignatureMPCCentralizedPublicKeyShareDecommitmentAndProof>(&centralized_party_public_key_share_decommitment_and_proof_ref) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };

    let secret_key_share_encryption_and_proof = pop_arg!(args, Vector);
    let secret_key_share_encryption_and_proof_ref = secret_key_share_encryption_and_proof.to_vec_u8()?;
    let Ok(secret_key_share_encryption_and_proof) = bcs::from_bytes::<DKGSignatureMPCSecretKeyShareEncryptionAndProof>(&secret_key_share_encryption_and_proof_ref) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };

    let commitment_to_centralized_party_secret_key_share = pop_arg!(args, Vector);
    let commitment_to_centralized_party_secret_key_share_ref = commitment_to_centralized_party_secret_key_share.to_vec_u8()?;
    let Ok(commitment_to_centralized_party_secret_key_share) = bcs::from_bytes::<DKGSignatureMPCCentralizedCommitment>(&commitment_to_centralized_party_secret_key_share_ref) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };

    let signature_mpc_paillier_public_parameters = object_runtime.local_config.signature_mpc_paillier_public_parameters.as_deref().unwrap();
    let output = decentralized_party_dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(signature_mpc_paillier_public_parameters, commitment_to_centralized_party_secret_key_share, centralized_party_public_key_share_decommitment_and_proof, secret_key_share_encryption_and_proof);

    Ok(NativeResult::ok(
        cost,
        smallvec![
            Value::vector_u8(bcs::to_bytes(&output).unwrap()),
            Value::vector_u8(bcs::to_bytes(&output.public_key).unwrap()),
            Value::vector_u8(bcs::to_bytes(&output.encrypted_secret_key_share).unwrap()),
        ],
    ))
}
