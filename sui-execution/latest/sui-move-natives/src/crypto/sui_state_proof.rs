// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
use crate::NativesCostTable;
use move_binary_format::errors::PartialVMResult;
use move_core_types::gas_algebra::InternalGas;
use move_vm_runtime::{native_charge_gas_early_exit, native_functions::NativeContext};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};
use smallvec::smallvec;
use std::{collections::VecDeque, str::FromStr};
use sui_types::{
    base_types::{ObjectID, SuiAddress},
    committee::Committee,
    effects::TransactionEffectsAPI,
    full_checkpoint_content::CheckpointTransaction,
    messages_checkpoint::{CertifiedCheckpointSummary, CheckpointContents, EndOfEpochData},
};

use serde_json::Value as JsonValue;
use sui_json::SuiJsonValue;

use move_core_types::annotated_value::MoveTypeLayout;

pub const INVALID_TX: u64 = 0;
pub const INVALID_CHECKPOINT_SUMMARY: u64 = 1;
pub const INVALID_COMMITTEE: u64 = 2;
pub const INVALID_INPUT: u64 = 3;

#[derive(Clone)]
pub struct SuiStateProofCostParams {
    /// Base cost for invoking the `sui_state_proof_verify_committee` function.
    pub sui_state_proof_verify_committee_cost_base: InternalGas,
    /// Base cost for invoking the `sui_state_proof_verify_link_cap` function.
    pub sui_state_proof_verify_link_cap_base: InternalGas,
    /// Base cost for invoking the `sui_state_proof_verify_transaction` function.
    pub sui_state_proof_verify_transaction_base: InternalGas,
}


// Helper function
// to verify if the user's inputs are valid and were processed by the epoch committee.
fn verify_data(
    committee: &Committee,
    transaction: &CheckpointTransaction,
    checkpoint_contents: &CheckpointContents,
    summary: &CertifiedCheckpointSummary,
) -> bool {

    // Verify the checkpoint summary using the committee.
    let res = summary.verify_with_contents(committee, Some(checkpoint_contents));
    if res.is_err() {
        return false;
    }

    let digests = transaction.effects.execution_digests();

    // Check if transaction digest matches the execution digest.
    if transaction.transaction.digest() != &digests.transaction {
        return false;
    }

    // Ensure the digests are in the checkpoint contents.
    if !checkpoint_contents
        .enumerate_transactions(summary)
        .any(|x| x.1 == &digests) {
        return false;
    }

    let events_digest = transaction.events.as_ref().map(|events| events.digest());
    // Ensure that the execution digest matches the events digest of the passed transaction.
    if events_digest.as_ref() != transaction.effects.events_digest() {
        return false;
    }

    true
}

/***************************************************************************************************
 * native fun sui_state_proof_verify_committee
 * Implementation of the Move native function `sui_state_proofs::sui_state_proof_verify_committee(commitment_to_centralized_party_secret_key_share: vector<u8>, secret_key_share_encryption_and_proof: vector<u8>, centralized_party_public_key_share_decommitment_and_proofs: vector<u8>): (vector<u8>, vector<u8>, vector<u8>);`
 * gas cost: sui_state_proof_verify_committee_cost_base   | base cost for function call and fixed operations.
 **************************************************************************************************/
pub fn sui_state_proof_verify_committee(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 2);

    // Load the cost parameters from the protocol config.
    let sui_state_proof_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .sui_state_proof_cost_params
        .clone();

    // Charge the base cost for this operation.
    native_charge_gas_early_exit!(
        context,
        sui_state_proof_cost_params.sui_state_proof_verify_committee_cost_base
    );

    let cost = context.gas_used();

    let checkpoint_summary_bytes = pop_arg!(args, Vec<u8>);
    let prev_committee_bytes = pop_arg!(args, Vec<u8>);

    let Ok(prev_committee) = bcs::from_bytes::<Committee>(&prev_committee_bytes) else {
        return Ok(NativeResult::err(cost, INVALID_COMMITTEE));
    };

    let Ok(checkpoint_summary) =
        bcs::from_bytes::<CertifiedCheckpointSummary>(&checkpoint_summary_bytes)
    else {
        return Ok(NativeResult::err(cost, INVALID_CHECKPOINT_SUMMARY));
    };

    match checkpoint_summary.clone().verify(&prev_committee) {
        Ok(_) => (),
        Err(_) => return Ok(NativeResult::err(cost, INVALID_TX)),
    }

    let next_committee_epoch;
    // Extract the new committee information.
    if let Some(EndOfEpochData {
                    next_epoch_committee,
                    ..
                }) = &checkpoint_summary.end_of_epoch_data
    {
        let next_committee = next_epoch_committee.iter().cloned().collect();
        next_committee_epoch = Committee::new(
            checkpoint_summary.epoch().checked_add(1).unwrap(),
            next_committee,
        );
    } else {
        return Ok(NativeResult::err(cost, INVALID_TX));
    }

    Ok(NativeResult::ok(
        cost,
        smallvec![
            Value::vector_u8(bcs::to_bytes(&next_committee_epoch).unwrap()),
            Value::u64(prev_committee.epoch)
        ],
    ))
}

/***************************************************************************************************
 * native fun sui_state_proof_verify_link_cap
 * Implementation of the Move native function `sui_state_proof::sui_state_proof_verify_link_cap(committee: vector<u8>, checkpoint_summary: vector<u8>, checkpoint_contents: vector<u8>, transaction: vector<u8>,  event_type_layout: vector<u8>,  package_id: vector<u8>): (vector<u8>, vector<u8>);`
 * gas cost: sui_state_proof_verify_link_cap_base   | base cost for function call and fixed operations.
 **************************************************************************************************/
pub fn sui_state_proof_verify_link_cap(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 6);

    // Load the cost parameters from the protocol config.
    let sui_state_proof_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .sui_state_proof_cost_params
        .clone();

    // Charge the base cost for this operation.
    native_charge_gas_early_exit!(
        context,
        sui_state_proof_cost_params.sui_state_proof_verify_link_cap_base
    );

    let cost = context.gas_used();

    let package_id_bytes = pop_arg!(args, Vec<u8>);
    let type_layout_bytes = pop_arg!(args, Vec<u8>);
    let transaction_bytes = pop_arg!(args, Vec<u8>);
    let checkpoint_contents_bytes = pop_arg!(args, Vec<u8>);
    let summary_bytes = pop_arg!(args, Vec<u8>);
    let committee_bytes = pop_arg!(args, Vec<u8>);

    // Trusted input from last state committee
    let Ok(committee) = bcs::from_bytes::<Committee>(&committee_bytes) else {
        return Ok(NativeResult::err(cost, INVALID_COMMITTEE));
    };

    // Untrusted input passed in by the user
    let Ok(summary) = bcs::from_bytes::<CertifiedCheckpointSummary>(&summary_bytes) else {
        return Ok(NativeResult::err(cost, INVALID_CHECKPOINT_SUMMARY));
    };

    // Untrusted input passed in by the user
    let Ok(checkpoint_contents) = bcs::from_bytes::<CheckpointContents>(&checkpoint_contents_bytes)
    else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    // Untrusted input passed in by the user.
    let Ok(transaction) = bcs::from_bytes::<CheckpointTransaction>(&transaction_bytes) else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    // Trusted input from config.
    let Ok(type_layout) = bcs::from_bytes::<MoveTypeLayout>(&type_layout_bytes) else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    // Trusted input from config.
    let Ok(package_id_target) = bcs::from_bytes::<ObjectID>(&package_id_bytes) else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    // Check if the user inputs are valid and were processed by the epoch committee.
    if !verify_data(&committee, &transaction, &checkpoint_contents, &summary) {
        return Ok(NativeResult::err(cost, INVALID_TX));
    }

    let tx_events = match transaction.events.as_ref() {
        Some(events) => &events.data,
        None => {
            return Ok(NativeResult::err(cost, INVALID_TX));
        }
    };

    let mut sui_cap_ids: Vec<SuiAddress> = Vec::new();
    let mut dwallet_cap_ids: Vec<SuiAddress> = Vec::new();

    // Iterate over each event to process
    for event in tx_events {
        // Check if the event matches the desired type
        if event.type_.address.to_hex() == package_id_target.to_hex()
            && event.type_.module.clone().into_string() == "dwallet_cap"
            && event.type_.name.clone().into_string() == "DWalletNetworkInitCapRequest"
        {
            let json_val = match SuiJsonValue::from_bcs_bytes(Some(&type_layout), &event.contents) {
                Ok(val) => val.to_json_value(),
                Err(_) => {
                    return Ok(NativeResult::err(cost, INVALID_TX));
                }
            };
            let sui_cap_id_str: Option<String> = match json_val.clone() {
                JsonValue::Object(map) => map
                    .get("cap_id")
                    .and_then(|s| s.as_str())
                    .map(|s| s.to_owned()),
                _ => None,
            };
            let sui_cap_id = match sui_cap_id_str.and_then(|s| SuiAddress::from_str(&s).ok()) {
                Some(id) => id,
                None => {
                    return Ok(NativeResult::err(cost, INVALID_TX));
                }
            };
            let dwallet_cap_id_str = match json_val.clone() {
                JsonValue::Object(map) => map
                    .get("dwallet_network_cap_id")
                    .and_then(|s| s.as_str())
                    .map(|s| s.to_owned()),
                _ => None,
            };
            let dwallet_cap_id =
                match dwallet_cap_id_str.and_then(|s| SuiAddress::from_str(&s).ok()) {
                    Some(id) => id,
                    None => {
                        return Ok(NativeResult::err(cost, INVALID_TX));
                    }
                };
            sui_cap_ids.push(sui_cap_id);
            dwallet_cap_ids.push(dwallet_cap_id);
        }
    }


    Ok(NativeResult::ok(
        cost,
        smallvec![
                Value::vector_u8(bcs::to_bytes(&sui_cap_ids).unwrap()),
                Value::vector_u8(bcs::to_bytes(&dwallet_cap_ids).unwrap())
            ],
    ))
}


/***************************************************************************************************
 * native fun sui_state_proof_verify_transaction
 * Implementation of the Move native function `dwallet_2pc_mpc_ecdsa_k1::sui_state_proof_verify_transaction(committee: vector<u8>, checkpoint_summary: vector<u8>, checkpoint_contents: vector<u8>, transaction: vector<u8>,  event_type_layout: vector<u8>,  package_id: vector<u8>): (vector<u8>, vector<u8>);`
 * gas cost: sui_state_proof_verify_transaction_base   | base cost for function call and fixed operations.
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
        .sui_state_proof_cost_params
        .clone();

    // Charge the base cost for this operation.
    native_charge_gas_early_exit!(
        context,
        sui_state_proof_cost_params.sui_state_proof_verify_transaction_base
    );

    let cost = context.gas_used();

    let package_id_bytes = pop_arg!(args, Vec<u8>);
    let type_layout_bytes = pop_arg!(args, Vec<u8>);
    let transaction_bytes = pop_arg!(args, Vec<u8>);
    let checkpoint_contents_bytes = pop_arg!(args, Vec<u8>);
    let summary_bytes = pop_arg!(args, Vec<u8>);
    let committee_bytes = pop_arg!(args, Vec<u8>);

    // Trusted input from last state committee
    let Ok(committee) = bcs::from_bytes::<Committee>(&committee_bytes) else {
        return Ok(NativeResult::err(cost, INVALID_COMMITTEE));
    };

    // Untrusted input passed in by the user
    let Ok(summary) = bcs::from_bytes::<CertifiedCheckpointSummary>(&summary_bytes) else {
        return Ok(NativeResult::err(cost, INVALID_CHECKPOINT_SUMMARY));
    };

    // Untrusted input passed in by the user
    let Ok(checkpoint_contents) = bcs::from_bytes::<CheckpointContents>(&checkpoint_contents_bytes)
    else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    // Untrusted input passed in by the user
    let Ok(transaction) = bcs::from_bytes::<CheckpointTransaction>(&transaction_bytes) else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    // Trusted input from config
    let Ok(type_layout) = bcs::from_bytes::<MoveTypeLayout>(&type_layout_bytes) else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    // Trusted input from config
    let Ok(package_id_target) = bcs::from_bytes::<ObjectID>(&package_id_bytes) else {
        return Ok(NativeResult::err(cost, INVALID_INPUT));
    };

    // Check if the user inputs are valid and were processed by the epoch committee.
    if !verify_data(&committee, &transaction, &checkpoint_contents, &summary) {
        return Ok(NativeResult::err(cost, INVALID_TX));
    }

    let tx_events = match transaction.events.as_ref() {
        Some(events) => &events.data,
        None => {
            return Ok(NativeResult::err(cost, INVALID_TX));
        }
    };

    let results: Vec<(SuiAddress, Vec<Vec<u8>>)> = tx_events
        .iter()
        .filter_map(|event| {
            if event.type_.address.to_hex() == package_id_target.to_hex()
                && event.type_.module.clone().into_string() == "dwallet_cap"
                && event.type_.name.clone().into_string() == "DWalletNetworkApproveRequest"
            {
                let json_val = SuiJsonValue::from_bcs_bytes(Some(&type_layout), &event.contents)
                    .unwrap()
                    .to_json_value();

                let cap_id_str = json_val.get("cap_id").and_then(JsonValue::as_str);
                let cap_id = cap_id_str.and_then(|s| SuiAddress::from_str(s).ok());

                let approve_messages = json_val.get("messages").and_then(JsonValue::as_array);
                let approve_msgs_vec: Option<Vec<Vec<u8>>> = approve_messages.map(|msgs_array| {
                    msgs_array
                        .iter()
                        .filter_map(|msg| {
                            msg.as_array().map(|inner| {
                                inner
                                    .iter()
                                    .filter_map(|byte| byte.as_u64().map(|b| b as u8))
                                    .collect()
                            })
                        })
                        .collect()
                });
                cap_id.and_then(|cap_id| approve_msgs_vec.map(|messages| (cap_id, messages)))
            } else {
                None
            }
        })
        .collect();

    if results.is_empty() {
        return Ok(NativeResult::err(cost, INVALID_TX));
    }

    let (cap_ids, messages): (Vec<SuiAddress>, Vec<Vec<Vec<u8>>>) = results.into_iter().unzip();

    Ok(NativeResult::ok(
        cost,
        smallvec![
            Value::vector_u8(bcs::to_bytes(&cap_ids).unwrap()),
            Value::vector_u8(bcs::to_bytes(&messages).unwrap())
        ],
    ))
}
