// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use dwallet_mpc::create_dkg_output;
use wasm_bindgen::prelude::*;

// Define the function to be exposed to JavaScript.
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
