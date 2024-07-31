// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::VecDeque;

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

use signature_mpc::twopc_mpc_protocols::{
    affine_point_to_public_key,
    decentralized_party_dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share,
    decentralized_party_sign_verify_encrypted_signature_parts_prehash, verify_signature,
    Commitment, DKGDecentralizedPartyOutput, DecentralizedPartyPresign, Hash, ProtocolContext,
    PublicKeyShareDecommitmentAndProof, PublicKeyValue,
    PublicNonceEncryptedPartialSignatureAndProof, SecretKeyShareEncryptionAndProof,
};

use crate::object_runtime::ObjectRuntime;
use crate::NativesCostTable;
use k256::AffinePoint;

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

#[derive(Clone)]
pub struct SignCostParams {
    /// Base cost for invoking the `verify_signatures` function
    pub verify_signatures_cost_base: InternalGas,
}
/***************************************************************************************************
 * native fun dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share
 * Implementation of the Move native function `dwallet_2pc_mpc_ecdsa_k1::dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(commitment_to_centralized_party_secret_key_share: vector<u8>, secret_key_share_encryption_and_proof: vector<u8>, centralized_party_public_key_share_decommitment_and_proofs: vector<u8>): (vector<u8>, vector<u8>);`
 * gas cost: dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base  | base cost for function call and fixed operators.
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
    let object_runtime = context.extensions().get::<ObjectRuntime>();
    // Charge the base cost for this operation.
    native_charge_gas_early_exit!(
        context,
        twopc_mpc_dkg_cost_params
            .dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base
    );

    let cost = context.gas_used();

    let centralized_party_public_key_share_decommitment_and_proof = pop_arg!(args, Vector);
    let centralized_party_public_key_share_decommitment_and_proof_ref =
        centralized_party_public_key_share_decommitment_and_proof.to_vec_u8()?;
    let Ok(centralized_party_public_key_share_decommitment_and_proof) =
        bcs::from_bytes::<PublicKeyShareDecommitmentAndProof<ProtocolContext>>(
            &centralized_party_public_key_share_decommitment_and_proof_ref,
        )
    else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    let secret_key_share_encryption_and_proof = pop_arg!(args, Vector);
    let secret_key_share_encryption_and_proof_ref =
        secret_key_share_encryption_and_proof.to_vec_u8()?;
    let Ok(secret_key_share_encryption_and_proof) =
        bcs::from_bytes::<SecretKeyShareEncryptionAndProof<ProtocolContext>>(
            &secret_key_share_encryption_and_proof_ref,
        )
    else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    let commitment_to_centralized_party_secret_key_share = pop_arg!(args, Vector);
    let commitment_to_centralized_party_secret_key_share_ref =
        commitment_to_centralized_party_secret_key_share.to_vec_u8()?;
    let Ok(commitment_to_centralized_party_secret_key_share) =
        bcs::from_bytes::<Commitment>(&commitment_to_centralized_party_secret_key_share_ref)
    else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    let signature_mpc_tiresias_public_parameters = object_runtime
        .protocol_config
        .signature_mpc_tiresias_public_parameters()
        .unwrap();

    let res =
        decentralized_party_dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(
            signature_mpc_tiresias_public_parameters,
            commitment_to_centralized_party_secret_key_share,
            centralized_party_public_key_share_decommitment_and_proof,
            secret_key_share_encryption_and_proof,
        );
    match res {
        Ok((output, public_key)) => Ok(NativeResult::ok(
            cost,
            smallvec![
                Value::vector_u8(bcs::to_bytes(&output).unwrap()),
                Value::vector_u8(public_key),
            ],
        )),
        Err(_) => Ok(NativeResult::err(cost, INVALID_INPUT)),
    }
}

/***************************************************************************************************
 * native fun sign_verify_encrypted_signature_parts_prehash
 * Implementation of the Move native function `dwallet_2pc_mpc_ecdsa_k1::sign_verify_encrypted_signature_parts_prehash(commitment_to_centralized_party_secret_key_share: vector<u8>, secret_key_share_encryption_and_proof: vector<u8>, centralized_party_public_key_share_decommitment_and_proofs: vector<u8>): (vector<u8>, vector<u8>, vector<u8>);`
 *   gas cost: sign_verify_encrypted_signature_parts_prehash_cost_base   | base cost for function call and fixed opers.
 **************************************************************************************************/
pub fn sign_verify_encrypted_signature_parts_prehash(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 5);

    // Load the cost parameters from the protocol config
    let twopc_mpc_dkg_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .twopc_mpc_dkg_cost_params
        .clone();

    // Load the cost parameters from the protocol config
    let object_runtime = context.extensions().get::<ObjectRuntime>();
    // Charge the base cost for this operation.
    native_charge_gas_early_exit!(
        context,
        twopc_mpc_dkg_cost_params.sign_verify_encrypted_signature_parts_prehash_cost_base
    );

    let cost = context.gas_used();

    let hash = pop_arg!(args, u8);

    let presigns = pop_arg!(args, Vector);
    let presigns = presigns.to_vec_u8()?;
    let Ok(presigns) = bcs::from_bytes::<Vec<DecentralizedPartyPresign>>(&presigns) else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    let public_nonce_encrypted_partial_signature_and_proofs = pop_arg!(args, Vector);
    let public_nonce_encrypted_partial_signature_and_proofs =
        public_nonce_encrypted_partial_signature_and_proofs.to_vec_u8()?;
    let Ok(public_nonce_encrypted_partial_signature_and_proofs) =
        bcs::from_bytes::<Vec<PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>>>(
            &public_nonce_encrypted_partial_signature_and_proofs,
        )
    else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    let dkg_output = pop_arg!(args, Vector);
    let dkg_output = dkg_output.to_vec_u8()?;
    let Ok(dkg_output) = bcs::from_bytes::<DKGDecentralizedPartyOutput>(&dkg_output) else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    let messages = pop_arg!(args, Vec<Value>);
    let messages = messages
        .into_iter()
        .map(|m| m.value_as::<Vec<u8>>())
        .collect::<PartialVMResult<Vec<_>>>()?;

    let signature_mpc_tiresias_public_parameters = object_runtime
        .protocol_config
        .signature_mpc_tiresias_public_parameters()
        .unwrap();
    let valid = decentralized_party_sign_verify_encrypted_signature_parts_prehash(
        signature_mpc_tiresias_public_parameters,
        messages,
        public_nonce_encrypted_partial_signature_and_proofs,
        dkg_output,
        presigns,
        hash.into(),
    )
    .is_ok();

    Ok(NativeResult::ok(cost, smallvec![Value::bool(valid),]))
}

pub fn verify_signatures_native(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 4);
    let sign_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .sign_cost_params
        .clone();
    native_charge_gas_early_exit!(context, sign_cost_params.verify_signatures_cost_base);

    let cost = context.gas_used();

    let public_key = pop_arg!(args, Vec<u8>);
    let public_key = affine_point_to_public_key(&public_key).expect("Invalid public key");
    let hash = pop_arg!(args, u8);
    let hash = Hash::from(hash);
    let signatures = pop_arg!(args, Vec<Value>);
    let signatures = signatures
        .into_iter()
        .map(|m| m.value_as::<Vec<u8>>())
        .collect::<PartialVMResult<Vec<_>>>()?;
    let messages = pop_arg!(args, Vec<Value>);
    let messages = messages
        .into_iter()
        .map(|m| m.value_as::<Vec<u8>>())
        .collect::<PartialVMResult<Vec<_>>>()?;
    let is_valid = verify_signature(messages, &hash, public_key, signatures);
    Ok(NativeResult::ok(cost, smallvec![Value::bool(is_valid),]))
}
