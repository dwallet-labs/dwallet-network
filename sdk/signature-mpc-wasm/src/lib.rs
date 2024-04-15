// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use rand_core::OsRng;
use signature_mpc::twopc_mpc_protocols::{initiate_centralized_party_dkg, decommitment_round_centralized_party_dkg, DKGDecommitmentRoundState, SecretKeyShareEncryptionAndProof, ProtocolContext};
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct InitiateDkgValue {
    pub commitment_to_centralized_party_secret_key_share: Vec<u8>,
    pub centralized_party_decommitment_round_party_state: Vec<u8>
}

#[derive(Serialize, Deserialize)]
pub struct FinalizeDkgValue {
    pub centralized_party_public_key_share_decommitment_and_proof: Vec<u8>,
    pub centralized_party_dkg_output: Vec<u8>
}

#[wasm_bindgen]
pub fn initiate_dkg() -> Result<JsValue, JsErr> {
    let party = initiate_centralized_party_dkg()?;
    let (
        commitment_to_centralized_party_secret_key_share,
        centralized_party_decommitment_round_party,
    ) = party
        .sample_commit_and_prove_secret_key_share(&mut OsRng)?;
    let centralized_party_decommitment_round_party_state = centralized_party_decommitment_round_party.to_state();
    let value = InitiateDkgValue {
        commitment_to_centralized_party_secret_key_share: bcs::to_bytes(&commitment_to_centralized_party_secret_key_share)?,
        centralized_party_decommitment_round_party_state: bcs::to_bytes(&centralized_party_decommitment_round_party_state)?,
    };
    Ok(serde_wasm_bindgen::to_value(&value)?)
}

#[wasm_bindgen]
pub fn finalize_dkg(centralized_party_decommitment_round_party_state: Vec<u8>, secret_key_share_encryption_and_proof: Vec<u8>) -> Result<JsValue, JsErr> {
    let centralized_party_decommitment_round_party_state: DKGDecommitmentRoundState<ProtocolContext> = bcs::from_bytes(&centralized_party_decommitment_round_party_state)?;
    let centralized_party_decommitment_round_party = decommitment_round_centralized_party_dkg(centralized_party_decommitment_round_party_state)?;

    let secret_key_share_encryption_and_proof = bcs::from_bytes::<SecretKeyShareEncryptionAndProof<ProtocolContext>>(&secret_key_share_encryption_and_proof)?;

    let (
        centralized_party_public_key_share_decommitment_and_proof,
        centralized_party_dkg_output,
    ) = centralized_party_decommitment_round_party
        .decommit_proof_public_key_share(secret_key_share_encryption_and_proof, &mut OsRng)?;

    let value = FinalizeDkgValue {
        centralized_party_public_key_share_decommitment_and_proof: bcs::to_bytes(&centralized_party_public_key_share_decommitment_and_proof)?,
        centralized_party_dkg_output: bcs::to_bytes(&centralized_party_dkg_output)?,
    };
    Ok(serde_wasm_bindgen::to_value(&value)?)
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
