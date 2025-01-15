// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Error;
use dwallet_mpc::{
    centralized_public_share_from_decentralized_output_inner, create_dkg_output,
    create_sign_output, decrypt_user_share_inner, encrypt_secret_share_and_prove,
    generate_secp_cg_keypair_from_seed_internal, verify_secret_share,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn create_dkg_centralized_output(protocol_public_parameters: Vec<u8>,
    key_scheme: u8,
    dkg_first_round_output: Vec<u8>,
    session_id: String,
) -> Result<JsValue, JsError> {
    serde_wasm_bindgen::to_value(
        &create_dkg_output(
            protocol_public_parameters,
            key_scheme,
            dkg_first_round_output,
            session_id,
        )
        .map_err(|e| JsError::new(&e.to_string()))?,
    )
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

/// Encrypts the given secret share to the given encryption key.
/// Returns a tuple of the encryption key and proof of encryption.
#[wasm_bindgen]
pub fn encrypt_secret_share(secret: Vec<u8>, encryption_key: Vec<u8>) -> Result<JsValue, JsError> {
    let encryption_and_proof =
        encrypt_secret_share_and_prove(secret, encryption_key).map_err(to_js_err)?;
    Ok(serde_wasm_bindgen::to_value(&encryption_and_proof)?)
}

/// Get the centralized party public share out of the decentralized dkg output.
#[wasm_bindgen]
pub fn centralized_public_share_from_decentralized_output(
    decentralized_output: Vec<u8>,
) -> Result<JsValue, JsError> {
    let encryption_and_proof =
        centralized_public_share_from_decentralized_output_inner(decentralized_output)
            .map_err(to_js_err)?;
    Ok(serde_wasm_bindgen::to_value(&encryption_and_proof)?)
}

/// Decrypts the given encrypted user share using the given decryption key.
#[wasm_bindgen]
pub fn decrypt_user_share(
    encryption_key: Vec<u8>,
    decryption_key: Vec<u8>,
    encrypted_user_share_and_proof: Vec<u8>,
) -> Result<JsValue, JsError> {
    let decrypted_secret_share = decrypt_user_share_inner(
        encryption_key,
        decryption_key,
        encrypted_user_share_and_proof,
    )
    .map_err(to_js_err)?;
    Ok(serde_wasm_bindgen::to_value(&decrypted_secret_share)?)
}

/// Verifies the given secret share matches the given DWallet's DKG output centralized_party_public_key_share.
#[wasm_bindgen]
pub fn verify_user_share(secret_share: Vec<u8>, dkg_output: Vec<u8>) -> Result<JsValue, JsError> {
    Ok(JsValue::from(
        verify_secret_share(secret_share, dkg_output).map_err(to_js_err)?,
    ))
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

// There is no way to implement From<anyhow::Error> for JsErr
// since the current From<Error> is generic, and it results in a conflict.
fn to_js_err(e: anyhow::Error) -> JsError {
    JsError::new(format!("{}", e).as_str())
}
