// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use dwallet_mpc::{create_dkg_output, create_sign_output};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn create_dkg_centralized_output(
    dkg_first_round_output: Vec<u8>,
    session_id: String,
) -> Result<JsValue, JsError> {
    let (public_key_share_and_proof, centralized_output) =
        create_dkg_output(dkg_first_round_output, session_id)
            .map_err(|e| JsError::new(&e.to_string()))?;

    // Serialize the result to JsValue and handle potential errors.
    serde_wasm_bindgen::to_value(&(public_key_share_and_proof, centralized_output))
        .map_err(|e| JsError::new(&e.to_string()))
}

// todo(zeev): why sign output was moved?
#[wasm_bindgen]
pub fn create_sign_centralized_output(
    centralized_party_dkg_output: Vec<u8>,
    presign_first_round_output: Vec<u8>,
    presign_second_round_output: Vec<u8>,
    // todo(zeev): Vec<Vec<u8>> here.
    messages: Vec<u8>,
    hash: u8,
    session_id: String,
) -> Result<JsValue, JsError> {
    let messages: Vec<Vec<u8>> =
        bcs::from_bytes(&messages).map_err(|e| JsError::new(&e.to_string()))?;
    let res = create_sign_output(
        centralized_party_dkg_output,
        presign_first_round_output,
        presign_second_round_output,
        messages,
        hash,
        session_id,
    )
    .map_err(|e| JsError::new(&e.to_string()))?;

    serde_wasm_bindgen::to_value(&res).map_err(|e| JsError::new(&e.to_string()))
}
