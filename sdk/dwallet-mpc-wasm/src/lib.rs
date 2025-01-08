// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use dwallet_mpc::{create_dkg_output, create_sign_output};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn create_dkg_centralized_output(
    protocol_public_parameters: Vec<u8>,
    key_scheme: u8,
    dkg_first_round_output: Vec<u8>,
    session_id: String,
) -> Result<JsValue, JsError> {
    let (public_key_share_and_proof, centralized_output, centralized_secret_output) =
        create_dkg_output(
            protocol_public_parameters,
            key_scheme,
            dkg_first_round_output,
            session_id,
        )
        .map_err(|e| JsError::new(&e.to_string()))?;

    // Serialize the result to JsValue and handle potential errors.
    serde_wasm_bindgen::to_value(&(
        public_key_share_and_proof,
        centralized_output,
        centralized_secret_output,
    ))
    .map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen]
pub fn create_sign_centralized_output(
    protocol_public_parameters: Vec<u8>,
    key_scheme: u8,
    centralized_party_dkg_output: Vec<u8>,
    centralized_party_dkg_secret_output: Vec<u8>,
    presigns: Vec<u8>,
    messages: Vec<u8>,
    hash_type: u8,
    session_ids: Vec<u8>,
) -> Result<JsValue, JsError> {
    let messages: Vec<Vec<u8>> =
        bcs::from_bytes(&messages).map_err(|e| JsError::new(&e.to_string()))?;
    let presigns: Vec<Vec<u8>> =
        bcs::from_bytes(&presigns).map_err(|e| JsError::new(&e.to_string()))?;
    let session_ids: Vec<String> =
        bcs::from_bytes(&session_ids).map_err(|e| JsError::new(&e.to_string()))?;
    let res = create_sign_output(
        protocol_public_parameters,
        key_scheme,
        centralized_party_dkg_output,
        centralized_party_dkg_secret_output,
        presigns,
        messages,
        hash_type,
        session_ids,
    )
    .map_err(|e| JsError::new(&e.to_string()))?;

    serde_wasm_bindgen::to_value(&res).map_err(|e| JsError::new(&e.to_string()))
}
