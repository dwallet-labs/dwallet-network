// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
use crate::NativesCostTable;
use move_binary_format::errors::{PartialVMResult};
use move_core_types::gas_algebra::InternalGas;
use move_vm_runtime::{native_charge_gas_early_exit, native_functions::NativeContext};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Value},
};
use smallvec::smallvec;
use std::{collections::VecDeque, ops::Add, str::FromStr};
use sui_types::{
    base_types::{ObjectID, SuiAddress}, committee::{self, Committee},  full_checkpoint_content::{CheckpointData, CheckpointTransaction}, messages_checkpoint::{CertifiedCheckpointSummary, CheckpointContents, CheckpointSummary, EndOfEpochData}, transaction
};


use sui_json::SuiJsonValue;
use serde_json::Value as JsonValue;

use move_core_types::annotated_value::MoveTypeLayout;

pub const INVALID_TX: u64 = 0;
pub const INVALID_CHECKPOINT_SUMMARY: u64 = 1;
pub const INVALID_COMMITTEE: u64 = 2;
pub const INVALID_INPUT: u64 = 3;

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
pub fn sui_state_proof_verify_committee(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 2);


    // Load the cost parameters from the protocol config
    let sui_state_proof_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .twopc_mpc_dkg_cost_params // TODO
        .clone();

    // Charge the base cost for this operation  
    native_charge_gas_early_exit!(
        context,
        sui_state_proof_cost_params.dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base
    );

    let cost = context.gas_used();


    let checkpoint_summary_bytes = pop_arg!(args, Vec<u8>);
    let prev_committee_bytes = pop_arg!(args, Vec<u8>);

    let Ok(prev_committee) = bcs::from_bytes::<Committee>(&prev_committee_bytes) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_COMMITTEE
        ));
    };

    
    let Ok(checkpoint_summary) = bcs::from_bytes::<CertifiedCheckpointSummary>(&checkpoint_summary_bytes) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_CHECKPOINT_SUMMARY
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

    Ok(NativeResult::ok(cost, smallvec![
            Value::vector_u8(bcs::to_bytes(&next_committee_epoch).unwrap()),
            Value::u64(prev_committee.epoch)
            ]))
}





/***************************************************************************************************
 * native fun dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share
 * Implementation of the Move native function `dwallet_2pc_mpc_ecdsa_k1::dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(commitment_to_centralized_party_secret_key_share: vector<u8>, secret_key_share_encryption_and_proof: vector<u8>, centralized_party_public_key_share_decommitment_and_proofs: vector<u8>): (vector<u8>, vector<u8>, vector<u8>);`
 *   gas cost: dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base   | base cost for function call and fixed opers
 **************************************************************************************************/
 pub fn sui_state_proof_verify_transaction(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 6);

    // Load the cost parameters from the protocol config
    let sui_state_proof_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .twopc_mpc_dkg_cost_params // TODO
        .clone();



    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        sui_state_proof_cost_params.dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share_cost_base
    );

    let cost = context.gas_used();


    let package_id_bytes = pop_arg!(args, Vec<u8>);
    let type_layout_bytes = pop_arg!(args, Vec<u8>);
    let transaction_bytes = pop_arg!(args, Vec<u8>);
    let checkpoint_contents_bytes = pop_arg!(args, Vec<u8>);
    let summary_bytes = pop_arg!(args, Vec<u8>);
    let committee_bytes = pop_arg!(args, Vec<u8>);

    let Ok(committee) = bcs::from_bytes::<Committee>(&committee_bytes) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_COMMITTEE
        ));
    };

    let Ok(summary) = bcs::from_bytes::<CertifiedCheckpointSummary>(&summary_bytes) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_CHECKPOINT_SUMMARY
        ));
    };

    let Ok(checkpoint_contents) = bcs::from_bytes::<CheckpointContents>(&checkpoint_contents_bytes) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };

    let Ok(transaction) = bcs::from_bytes::<CheckpointTransaction>(&transaction_bytes) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };


    let Ok(type_layout) = bcs::from_bytes::<MoveTypeLayout>(&type_layout_bytes) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };


    let Ok(package_id_target) = bcs::from_bytes::<ObjectID>(&package_id_bytes) else {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT
        ));
    };
    
    // Verify the checkpoint summary using the committee
    let res = summary.verify_with_contents(&committee, Some(&checkpoint_contents));
    if let Err(err) = res {
        return Ok(NativeResult::err(cost, INVALID_TX));
    }


    // Ensure the tx is part of the checkpoint
    let is_valid_checkpoint_tx = checkpoint_contents.iter().any(|&digest| digest == transaction.effects.execution_digests());
    if !is_valid_checkpoint_tx {
      return Ok(NativeResult::err(cost, INVALID_TX));
    };



    let tx_events = &transaction.events.as_ref().unwrap().data;


    let mut messages = Vec::new();
    let mut cap_id_final = SuiAddress::ZERO;

    for event in tx_events {
        if !(event.clone().package_id == package_id_target && event.clone().type_.module.into_string() == "dwallet_cap" && event.clone().type_.name.into_string() == "DWalletNetworkRequest") {
            continue;
        }

        let json_val = SuiJsonValue::from_bcs_bytes(Some(&type_layout), &event.contents).unwrap().to_json_value();


        let approve_message = match json_val.clone() {
            JsonValue::Object(map) => {
                if let Some(msg_value) = map.get("message").and_then(|s| s.as_array()) {
                    msg_value.to_owned() 
                    
                }
                else {
                    return Ok(NativeResult::err(cost, INVALID_TX));
                }
            },
            _ => return Ok(NativeResult::err(cost, INVALID_TX))
        };


        let approve_msg_vec: Vec<u8> = approve_message.iter().map(|msg| {
            msg.as_u64().unwrap() as u8
        }).collect();

        let cap_id = match json_val.clone() {
            JsonValue::Object(map) => {
                if let Some(msg_value) = map.get("cap_id").and_then(|s| s.as_str()) {
                    msg_value.to_owned() 
                }
                else {
                    return Ok(NativeResult::err(cost, INVALID_TX));
                }
            },
            _ => return Ok(NativeResult::err(cost, INVALID_TX))
        };

        let cap_id = match SuiAddress::from_str(&cap_id) {
            Ok(address) => address,
            Err(_) => return Ok(NativeResult::err(cost, INVALID_TX)),
        };


        // Ensure that the cap id is the same for all messages.
        if cap_id_final != SuiAddress::ZERO && cap_id_final != cap_id {
            return Ok(NativeResult::err(cost, INVALID_TX));
        }

        cap_id_final = cap_id;
        messages.push(approve_msg_vec);
    }

    if (cap_id_final != SuiAddress::ZERO && messages.len() > 0) {
        return Ok(NativeResult::ok(cost, smallvec![
            Value::vector_u8(bcs::to_bytes(&cap_id_final).unwrap()), 
            Value::vector_u8(bcs::to_bytes(&messages).unwrap())]));
    }
    else {
        return Ok(NativeResult::err(cost, INVALID_TX));
    }
}
