// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use rand_core::OsRng;
use signature_mpc::twopc_mpc_protocols::initiate_centralized_party_dkg;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct InitiateDkgValue {
    pub commitment_to_centralized_party_secret_key_share: Vec<u8>,
    pub centralized_party_decommitment_round_party_state: Vec<u8>
}

#[wasm_bindgen]
pub fn initiate_dkg() -> Result<JsValue, JsErr> {
    let party = initiate_centralized_party_dkg().unwrap();
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
