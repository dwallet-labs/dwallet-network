// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
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
// use sui_types::messages_signature_mpc::{decentralized_party_dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share, DKGSignatureMPCCentralizedCommitment, DKGSignatureMPCCentralizedPublicKeyShareDecommitmentAndProof, DKGSignatureMPCSecretKeyShareEncryptionAndProof};

use sui_types::{
    committee::{self, Committee}, 
    crypto::{AuthorityQuorumSignInfo, AuthoritySignInfoTrait},
    full_checkpoint_content::CheckpointData,
    messages_checkpoint::{CertifiedCheckpointSummary, CheckpointSummary, EndOfEpochData, CheckpointContents},
    full_checkpoint_content::CheckpointTransaction,
    event::Event,
    base_types::ObjectID,
};
use sui_json::SuiJsonValue;
use serde_json::Value as JsonValue;
// use sui_json::JsonValue;

use move_core_types::annotated_value::MoveTypeLayout;

use crate::object_runtime::ObjectRuntime;

pub const INVALID_INPUT: u64 = 0;
pub const INVALID_TX: u64 = 1;

#[derive(Clone)]
pub struct TwoPCMPCDKGCostParams {
    /// Base cost for invoking the `dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share` function
    pub sui_state_proof_verify: InternalGas,
}
/***************************************************************************************************
 * native fun dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share
 * Implementation of the Move native function `dwallet_2pc_mpc_ecdsa_k1::dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(commitment_to_centralized_party_secret_key_share: vector<u8>, secret_key_share_encryption_and_proof: vector<u8>, centralized_party_public_key_share_decommitment_and_proofs: vector<u8>): (vector<u8>, vector<u8>, vector<u8>);`
 *   gas cost: dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base   | base cost for function call and fixed opers
 **************************************************************************************************/
pub fn sui_state_proof_verify(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 3);

    // Load the cost parameters from the protocol config
    let sui_state_proof_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .twopc_mpc_dkg_cost_params // TODO
        .clone();

    // Load the cost parameters from the protocol config
    // let object_runtime = context
    //     .extensions()
    //     .get::<ObjectRuntime>();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        sui_state_proof_cost_params.dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base
    );

    let cost = context.gas_used();

    let prev_committee = pop_arg!(args, Vector);
    let prev_committee_ref = prev_committee.to_vec_u8()?;
    let Ok(prev_committee) = bcs::from_bytes::<Committee>(&prev_committee_ref) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };

    let checkpoint_summary = pop_arg!(args, Vector);
    let checkpoint_summary_ref = checkpoint_summary.to_vec_u8()?;
    let Ok(checkpoint_summary) = bcs::from_bytes::<CertifiedCheckpointSummary>(&checkpoint_summary_ref) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };


    match checkpoint_summary.clone().verify(&prev_committee) {
        Ok((_)) => (),
        Err(e) => return Ok(NativeResult::err(cost, INVALID_TX)),
    }


    let next_committee_epoch;
    // Extract the new committee information
    if let Some(EndOfEpochData {
        next_epoch_committee,
        ..
    }) = &checkpoint_summary.end_of_epoch_data
    {
        let next_committee = next_epoch_committee.iter().cloned().collect();
        next_committee_epoch =
            Committee::new(checkpoint_summary.epoch().checked_add(1).unwrap(), next_committee);
    } else {
        return Ok(NativeResult::err(cost, INVALID_TX))
    }

    Ok(NativeResult::ok(cost, smallvec![Value::vector_u8(bcs::to_bytes(&next_committee_epoch).unwrap())]))
}





/***************************************************************************************************
 * native fun dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share
 * Implementation of the Move native function `dwallet_2pc_mpc_ecdsa_k1::dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(commitment_to_centralized_party_secret_key_share: vector<u8>, secret_key_share_encryption_and_proof: vector<u8>, centralized_party_public_key_share_decommitment_and_proofs: vector<u8>): (vector<u8>, vector<u8>, vector<u8>);`
 *   gas cost: dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base   | base cost for function call and fixed opers
 **************************************************************************************************/
 pub fn sui_state_proof_process_dwallet_sign_request(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 3);

    // Load the cost parameters from the protocol config
    let sui_state_proof_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .twopc_mpc_dkg_cost_params // TODO
        .clone();

    // Load the cost parameters from the protocol config
    // let object_runtime = context
    //     .extensions()
    //     .get::<ObjectRuntime>();

    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        sui_state_proof_cost_params.dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base
    );

    let cost = context.gas_used();

    let committee = pop_arg!(args, Vector);
    let committee_ref = committee.to_vec_u8()?;
    let Ok(committee) = bcs::from_bytes::<Committee>(&committee_ref) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };

    let summary = pop_arg!(args, Vector);
    let summary_ref = summary.to_vec_u8()?;
    let Ok(summary) = bcs::from_bytes::<CertifiedCheckpointSummary>(&summary_ref) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };

    let checkpoint_contents = pop_arg!(args, Vector);
    let checkpoint_contents_ref = checkpoint_contents.to_vec_u8()?;
    let Ok(checkpoint_contents) = bcs::from_bytes::<CheckpointContents>(&checkpoint_contents_ref) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };

    let transaction: Vector = pop_arg!(args, Vector);
    let transaction_ref = transaction.to_vec_u8()?;
    let Ok(transaction) = bcs::from_bytes::<CheckpointTransaction>(&transaction_ref) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };


    let type_layout = pop_arg!(args, Vector);
    let type_layout_ref = type_layout.to_vec_u8()?;
    let Ok(type_layout) = bcs::from_bytes::<MoveTypeLayout>(&type_layout_ref) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };

    let package_id = pop_arg!(args, Vector);
    let package_id_ref = package_id.to_vec_u8()?;
    let Ok(package_id) = bcs::from_bytes::<ObjectID>(&package_id_ref) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };
    
    // Verify the checkpoint summary using the committee
    let res = summary.verify_with_contents(&committee, Some(&checkpoint_contents));
    if let Err(_) = res {
        return Ok(NativeResult::err(cost, INVALID_TX));
    }

    // Ensure the tx is part of the checkpoint
    let is_valid_checkpoint_tx = checkpoint_contents.iter().any(|&digest| digest == transaction.effects.execution_digests());
    if !is_valid_checkpoint_tx {
      return Ok(NativeResult::err(cost, INVALID_TX));
    };

    let tx_events = &transaction.events.as_ref().unwrap().data;

    let mut messages = Vec::new();
    for event in tx_events {

        if !event.package_id.eq(&package_id) {
            return Ok(NativeResult::err(cost, INVALID_TX));
        }

        let json_val = SuiJsonValue::from_bcs_bytes(Some(&type_layout), &event.contents).unwrap().to_json_value();
        
        // get signature from the json
        let approve_message = match json_val {
            JsonValue::Object(map) => {
                if let Some(msg_value) = map.get("message").and_then(|s| s.as_str()) { // TODO take String from config
                    msg_value.to_owned() 
                }
                else {
                    return Ok(NativeResult::err(cost, INVALID_TX));
                }
    
            },
            _ => return Ok(NativeResult::err(cost, INVALID_TX))
        };
        messages.push(approve_message);
    }
    
    Ok(NativeResult::ok(cost, smallvec![Value::vector_u8(bcs::to_bytes(&messages).unwrap())]))
}
