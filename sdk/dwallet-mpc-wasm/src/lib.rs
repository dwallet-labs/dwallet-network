// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Error;
use dwallet_mpc::{
    create_dkg_output, create_sign_output, generate_secp_cg_keypair_from_seed_internal,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn create_dkg_centralized_output(
    protocol_public_parameters: Vec<u8>,
    dkg_first_round_output: Vec<u8>,
    session_id: String,
) -> Result<JsValue, JsError> {
    let (public_key_share_and_proof, centralized_output, centralized_secret_output) =
        create_dkg_output(
            protocol_public_parameters,
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

/// Derives a Secp256k1 class groups keypair from a given seed.
///
/// The class groups key being used to encrypt a Secp256k1 keypair should be different from
/// the encryption key used to encrypt a Ristretto keypair, due to cryptographic reasons.
/// This function derives a class groups keypair to encrypt a Secp256k1 secret from the given seed.
#[wasm_bindgen]
pub fn generate_secp_cg_keypair_from_seed(seed: &[u8]) -> Result<JsValue, JsError> {
    let seed: [u8; 32] = seed
        .try_into()
        .map_err(|_| JsError::new("seed must be 32 bytes long"))?;
    let (public_key, private_key) =
        generate_secp_cg_keypair_from_seed_internal(seed).map_err(to_js_err)?;
    Ok(serde_wasm_bindgen::to_value(&(public_key, private_key))?)
}

#[wasm_bindgen]
pub fn create_sign_centralized_output(
    protocol_public_parameters: Vec<u8>,
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

// There is no way to implement From<anyhow::Error> for JsErr
// since the current From<Error> is generic, and it results in a conflict.
fn to_js_err(e: Error) -> JsError {
    JsError::new(format!("{}", e).as_str())
}
