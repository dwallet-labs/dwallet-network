// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use dwallet_mpc_centralized_party::{
    advance_centralized_sign_party, centralized_public_share_from_decentralized_output_inner,
    create_dkg_output, decrypt_user_share_inner, encrypt_secret_key_share_and_prove,
    generate_secp256k1_cg_keypair_from_seed_internal, public_keys_from_dwallet_output,
    verify_secret_share,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn create_dkg_centralized_output(
    network_decryption_key_public_output: Vec<u8>,
    key_scheme: u8,
    decentralized_first_round_public_output: Vec<u8>,
    session_id: String,
) -> Result<JsValue, JsError> {
    let dkg_centralized_result = &create_dkg_output(
        network_decryption_key_public_output,
        key_scheme,
        decentralized_first_round_public_output,
        session_id,
    )
    .map_err(|e| JsError::new(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&(
        dkg_centralized_result.public_key_share_and_proof.clone(),
        dkg_centralized_result.public_output.clone(),
        dkg_centralized_result.centralized_secret_output.clone(),
    ))
    .map_err(|e| JsError::new(&e.to_string()))
}

/// Derives a Secp256k1 class groups keypair from a given seed.
///
/// The class groups public encryption key being used to encrypt a Secp256k1 keypair will be
/// different from the encryption key used to encrypt a Ristretto keypair.
/// The plaintext space/fundamental group will correspond to the order
/// of the respective elliptic curve.
/// The secret decryption key may be the same in terms of correctness,
/// but to simplify security analysis and implementation current version maintain distinct key-pairs.
#[wasm_bindgen]
pub fn generate_secp_cg_keypair_from_seed(seed: &[u8]) -> Result<JsValue, JsError> {
    let seed: [u8; 32] = seed
        .try_into()
        .map_err(|_| JsError::new("seed must be 32 bytes long"))?;
    let (public_key, private_key) =
        generate_secp256k1_cg_keypair_from_seed_internal(seed).map_err(to_js_err)?;
    Ok(serde_wasm_bindgen::to_value(&(public_key, private_key))?)
}

/// Encrypts the given secret share to the given encryption key.
/// Returns a tuple of the encryption key and proof of encryption.
#[wasm_bindgen]
pub fn encrypt_secret_share(
    secret_key_share: Vec<u8>,
    encryption_key: Vec<u8>,
) -> Result<JsValue, JsError> {
    let encryption_and_proof =
        encrypt_secret_key_share_and_prove(secret_key_share, encryption_key).map_err(to_js_err)?;
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

/// Verifies that the given secret key share matches the given dWallet public key share.
/// DKG output->centralized_party_public_key_share.
#[wasm_bindgen]
pub fn verify_user_share(secret_share: Vec<u8>, dkg_output: Vec<u8>) -> Result<JsValue, JsError> {
    Ok(JsValue::from(
        verify_secret_share(secret_share, dkg_output).map_err(to_js_err)?,
    ))
}

/// Derives the DWallet's public keys from the given DKG output.
#[wasm_bindgen]
pub fn public_keys_from_dkg_output(dkg_output: Vec<u8>) -> Result<JsValue, JsError> {
    Ok(JsValue::from(
        public_keys_from_dwallet_output(dkg_output).map_err(to_js_err)?,
    ))
}

#[wasm_bindgen]
pub fn create_sign_centralized_output(
    network_decryption_key_public_output: Vec<u8>,
    key_scheme: u8,
    decentralized_party_dkg_public_output: Vec<u8>,
    centralized_party_dkg_secret_output: Vec<u8>,
    presign: Vec<u8>,
    message: Vec<u8>,
    hash_type: u8,
) -> Result<JsValue, JsError> {
    let signed_message = advance_centralized_sign_party(
        network_decryption_key_public_output,
        key_scheme,
        decentralized_party_dkg_public_output,
        centralized_party_dkg_secret_output,
        presign,
        message,
        hash_type,
    )
    .map_err(|e| JsError::new(&e.to_string()))?;

    serde_wasm_bindgen::to_value(&signed_message).map_err(|e| JsError::new(&e.to_string()))
}

// There is no way to implement From<anyhow::Error> for JsErr
// since the current From<Error> is generic, and it results in a conflict.
fn to_js_err(e: anyhow::Error) -> JsError {
    JsError::new(format!("{}", e).as_str())
}
