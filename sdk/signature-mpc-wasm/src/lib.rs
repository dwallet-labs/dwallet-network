// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Error;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use signature_mpc::twopc_mpc_protocols::CentralizedPartyPresign;
use signature_mpc::twopc_mpc_protocols::DKGCentralizedPartyOutput;
use wasm_bindgen::prelude::*;

use signature_mpc::twopc_mpc_protocols::encrypt_user_share::{
    encryption_of_discrete_log_public_parameters, parse_and_verify_secret_share,
    EncryptedUserShareAndProof,
};
use signature_mpc::twopc_mpc_protocols::finalize_centralized_party_presign;
use signature_mpc::twopc_mpc_protocols::finalize_centralized_party_sign;
use signature_mpc::twopc_mpc_protocols::initiate_centralized_party_presign;
use signature_mpc::twopc_mpc_protocols::initiate_centralized_party_sign;
use signature_mpc::twopc_mpc_protocols::message_digest;
use signature_mpc::twopc_mpc_protocols::PresignDecentralizedPartyOutput;
use signature_mpc::twopc_mpc_protocols::PublicNonceEncryptedPartialSignatureAndProof;
use signature_mpc::twopc_mpc_protocols::Result as TwoPCMPCResult;
use signature_mpc::twopc_mpc_protocols::Scalar;
use signature_mpc::twopc_mpc_protocols::{
    affine_point_to_public_key, decommitment_round_centralized_party_dkg,
    initiate_centralized_party_dkg, recovery_id, DKGDecommitmentRoundState, Hash, ProtocolContext,
    PublicKeyValue, SecretKeyShareEncryptionAndProof, SignatureK256Secp256k1,
};

#[derive(Serialize, Deserialize)]
pub struct InitiateDKGValue {
    pub commitment_to_secret_key_share: Vec<u8>,
    pub decommitment_round_party_state: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct FinalizeDKGValue {
    pub public_key_share_decommitment_and_proof: Vec<u8>,
    pub secret_key_share: Vec<u8>,
    pub dkg_output: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct InitiatePresignValue {
    pub nonce_shares_commitments_and_batched_proof: Vec<u8>,
    pub signature_nonce_shares_and_commitment_randomnesses: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct InitiateSignValue {
    pub public_nonce_encrypted_partial_signature_and_proofs: Vec<u8>,
    pub signature_nonce_shares_and_commitment_randomnesses: Vec<u8>,
}

#[wasm_bindgen]
pub fn initiate_dkg() -> Result<JsValue, JsErr> {
    let party = initiate_centralized_party_dkg()?;
    let (commitment_to_secret_key_share, decommitment_round_party) =
        party.sample_commit_and_prove_secret_key_share(&mut OsRng)?;
    let decommitment_round_party_state = decommitment_round_party.to_state();
    let value = InitiateDKGValue {
        commitment_to_secret_key_share: bcs::to_bytes(&commitment_to_secret_key_share)?,
        decommitment_round_party_state: bcs::to_bytes(&decommitment_round_party_state)?,
    };
    Ok(serde_wasm_bindgen::to_value(&value)?)
}

#[wasm_bindgen]
pub fn finalize_dkg(
    decommitment_round_party_state: Vec<u8>,
    secret_key_share_encryption_and_proof: Vec<u8>,
) -> Result<JsValue, JsErr> {
    let decommitment_round_party_state: DKGDecommitmentRoundState<ProtocolContext> =
        bcs::from_bytes(&decommitment_round_party_state)?;
    let decommitment_round_party =
        decommitment_round_centralized_party_dkg(decommitment_round_party_state)?;

    let secret_key_share_encryption_and_proof = bcs::from_bytes::<
        SecretKeyShareEncryptionAndProof<ProtocolContext>,
    >(&secret_key_share_encryption_and_proof)?;

    let (public_key_share_decommitment_and_proof, dkg_output) = decommitment_round_party
        .decommit_proof_public_key_share(secret_key_share_encryption_and_proof, &mut OsRng)?;

    let value = FinalizeDKGValue {
        public_key_share_decommitment_and_proof: bcs::to_bytes(
            &public_key_share_decommitment_and_proof,
        )?,
        secret_key_share: bcs::to_bytes(&dkg_output.secret_key_share)?,
        dkg_output: bcs::to_bytes(&dkg_output)?,
    };
    Ok(serde_wasm_bindgen::to_value(&value)?)
}

#[wasm_bindgen]
pub fn initiate_presign(dkg_output: Vec<u8>, batch_size: usize) -> Result<JsValue, JsErr> {
    let dkg_output: DKGCentralizedPartyOutput = bcs::from_bytes(&dkg_output)?;
    let commitment_round_party = initiate_centralized_party_presign(dkg_output.clone())?;

    let (nonce_shares_commitments_and_batched_proof, proof_verification_round_party) =
        commitment_round_party
            .sample_commit_and_prove_signature_nonce_share(batch_size, &mut OsRng)?;

    let value = InitiatePresignValue {
        nonce_shares_commitments_and_batched_proof: bcs::to_bytes(
            &nonce_shares_commitments_and_batched_proof,
        )?,
        signature_nonce_shares_and_commitment_randomnesses: bcs::to_bytes(
            &proof_verification_round_party.signature_nonce_shares_and_commitment_randomnesses,
        )?,
    };

    Ok(serde_wasm_bindgen::to_value(&value)?)
}

#[wasm_bindgen]
pub fn finalize_presign(
    dkg_output: Vec<u8>,
    signature_nonce_shares_and_commitment_randomnesses: Vec<u8>,
    presign_output: Vec<u8>,
) -> Result<JsValue, JsErr> {
    let dkg_output: DKGCentralizedPartyOutput = bcs::from_bytes(&dkg_output)?;
    let presign_output: PresignDecentralizedPartyOutput<ProtocolContext> =
        bcs::from_bytes(&presign_output)?;
    let signature_nonce_shares_and_commitment_randomnesses: Vec<(Scalar, Scalar)> =
        bcs::from_bytes(&signature_nonce_shares_and_commitment_randomnesses)?;
    let commitment_round_party = finalize_centralized_party_presign(
        dkg_output.clone(),
        signature_nonce_shares_and_commitment_randomnesses,
    )?;

    let presigns = commitment_round_party.verify_presign_output(presign_output, &mut OsRng)?;

    let presigns = bcs::to_bytes(&presigns)?;

    Ok(serde_wasm_bindgen::to_value(&presigns)?)
}

#[wasm_bindgen]
pub fn initiate_sign(
    dkg_output: Vec<u8>,
    presigns: Vec<u8>,
    messages: Vec<u8>,
    hash: u8,
) -> Result<JsValue, JsErr> {
    let messages: Vec<Vec<u8>> = bcs::from_bytes(&messages)?;
    let presigns: Vec<CentralizedPartyPresign> = bcs::from_bytes(&presigns)?;
    let dkg_output: DKGCentralizedPartyOutput = bcs::from_bytes(&dkg_output)?;
    let commitment_round_parties = initiate_centralized_party_sign(dkg_output.clone(), presigns)?;

    let (
        public_nonce_encrypted_partial_signature_and_proofs,
        _signature_verification_round_parties,
    ): (Vec<_>, Vec<_>) = messages
        .into_iter()
        .zip(commitment_round_parties.into_iter())
        .map(|(message, party)| {
            let m = message_digest(&message, &hash.into());
            party.evaluate_encrypted_partial_signature_prehash(m, &mut OsRng)
        })
        .collect::<TwoPCMPCResult<Vec<_>>>()?
        .into_iter()
        .unzip();

    let public_nonce_encrypted_partial_signature_and_proofs =
        bcs::to_bytes(&public_nonce_encrypted_partial_signature_and_proofs)?;

    Ok(serde_wasm_bindgen::to_value(
        &public_nonce_encrypted_partial_signature_and_proofs,
    )?)
}

#[wasm_bindgen]
pub fn finalize_sign(
    dkg_output: Vec<u8>,
    messages: Vec<u8>,
    public_nonce_encrypted_partial_signature_and_proofs: Vec<u8>,
    signatures_s: Vec<u8>,
    hash: u8,
) -> Result<(), JsErr> {
    let messages: Vec<Vec<u8>> = bcs::from_bytes(&messages)?;
    let messages = messages
        .into_iter()
        .map(|message| message_digest(&message, &hash.into()))
        .collect();
    let dkg_output: DKGCentralizedPartyOutput = bcs::from_bytes(&dkg_output)?;
    let public_nonce_encrypted_partial_signature_and_proofs: Vec<
        PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>,
    > = bcs::from_bytes(&public_nonce_encrypted_partial_signature_and_proofs)?;
    let signatures_s: Vec<Scalar> = bcs::from_bytes(&signatures_s)?;

    finalize_centralized_party_sign(
        messages,
        dkg_output,
        public_nonce_encrypted_partial_signature_and_proofs,
        signatures_s,
    )
    .map_err(JsErr::from)
}

#[wasm_bindgen]
pub fn recovery_id_keccak256(
    public_key: Vec<u8>,
    message: Vec<u8>,
    signature: Vec<u8>,
) -> Result<u8, JsErr> {
    let public_key: PublicKeyValue = affine_point_to_public_key(&public_key).ok_or(JsErr {
        display: "cannot serialize public key".to_string(),
        message: "cannot serialize public key".to_string(),
    })?;
    let signature: SignatureK256Secp256k1 = bcs::from_bytes(&signature)?;

    Ok(
        recovery_id(message, public_key, signature, &Hash::KECCAK256)
            .map_err(|_| JsErr {
                message: "Can't generate RecoveryId".to_string(),
                display: "Can't generate RecoveryId".to_string(),
            })?
            .into(),
    )
}

#[wasm_bindgen]
pub fn recovery_id_sha256(
    public_key: Vec<u8>,
    message: Vec<u8>,
    signature: Vec<u8>,
) -> Result<u8, JsErr> {
    let public_key: PublicKeyValue = affine_point_to_public_key(&public_key).ok_or(JsErr {
        display: "cannot serialize public key".to_string(),
        message: "cannot serialize public key".to_string(),
    })?;
    let signature: SignatureK256Secp256k1 = bcs::from_bytes(&signature)?;

    Ok(recovery_id(message, public_key, signature, &Hash::SHA256)
        .map_err(|_| JsErr {
            message: "Can't generate RecoveryId".to_string(),
            display: "Can't generate RecoveryId".to_string(),
        })?
        .into())
}

#[wasm_bindgen]
pub fn generate_keypair() -> Result<JsValue, JsErr> {
    let (public_key, private_key) =
        signature_mpc::twopc_mpc_protocols::encrypt_user_share::generate_keypair()
            .map_err(to_js_err)?;
    Ok(serde_wasm_bindgen::to_value(&(public_key, private_key))?)
}

#[wasm_bindgen]
pub fn generate_proof(secret_share: Vec<u8>, public_key: Vec<u8>) -> Result<JsValue, JsErr> {
    let language_public_parameters =
        encryption_of_discrete_log_public_parameters(public_key.clone()).map_err(to_js_err)?;
    let proof_public_output =
        signature_mpc::twopc_mpc_protocols::encrypt_user_share::generate_proof(
            public_key,
            secret_share,
            language_public_parameters,
        )
        .map_err(to_js_err)?;
    let proof_public_output = bcs::to_bytes(&proof_public_output)?;

    Ok(serde_wasm_bindgen::to_value(&proof_public_output)?)
}

#[wasm_bindgen]
pub fn decrypt_user_share(
    encryption_key: Vec<u8>,
    decryption_key: Vec<u8>,
    encrypted_user_share_and_proof: Vec<u8>,
) -> Result<Vec<u8>, JsErr> {
    let encrypted_user_share_and_proof: EncryptedUserShareAndProof =
        bcs::from_bytes(&encrypted_user_share_and_proof)?;
    let user_share = signature_mpc::twopc_mpc_protocols::encrypt_user_share::decrypt_user_share(
        encryption_key,
        decryption_key,
        encrypted_user_share_and_proof,
    )
    .map_err(to_js_err)?;
    Ok(user_share)
}

#[wasm_bindgen]
pub fn verify_user_share(secret_share: Vec<u8>, dkg_output: Vec<u8>) -> Result<JsValue, JsErr> {
    let is_matching =
        parse_and_verify_secret_share(&secret_share, &dkg_output).map_err(to_js_err)?;
    Ok(JsValue::from(is_matching))
}

#[derive(Serialize, Deserialize)]
/// Error type for better JS handling and generalization
/// of Rust / WASM -> JS error conversion.
pub struct JsErr {
    // type_: String,
    message: String,
    display: String,
}

impl<T: std::error::Error> From<T> for JsErr {
    fn from(err: T) -> Self {
        JsErr {
            display: format!("{}", err),
            message: err.to_string(),
        }
    }
}

impl From<JsErr> for JsValue {
    fn from(err: JsErr) -> Self {
        serde_wasm_bindgen::to_value(&err).unwrap()
    }
}

// There is no way to implement From<anyhow::Error> for JsErr
// since the current From<Error> is generic, and it results in a conflict.
fn to_js_err(e: Error) -> JsErr {
    JsErr {
        display: format!("{}", e),
        message: e.to_string(),
    }
}
