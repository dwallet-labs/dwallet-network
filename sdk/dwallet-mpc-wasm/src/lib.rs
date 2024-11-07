// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Error;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use dwallet_mpc::{create_dkg_output, create_sign_output};
use log::debug;

#[wasm_bindgen]
pub fn create_dkg_centralized_output(dkg_first_round_output: Vec<u8>, session_id: String) -> Result<JsValue, JsErr> {
    let (public_key_share_and_proof, centralized_output) = create_dkg_output(dkg_first_round_output, session_id).map_err(to_js_err)?;
    Ok(serde_wasm_bindgen::to_value(&(public_key_share_and_proof, centralized_output)).unwrap())
}

#[wasm_bindgen]
pub fn create_sign_centralized_output(centralized_party_dkg_output: Vec<u8>, presign: Vec<u8>, message: Vec<u8>, hash: u8, session_id: String) -> Result<JsValue, JsErr> {
    let (sign_message, centralized_output) = create_sign_output(centralized_party_dkg_output, presign, message, hash, session_id).map_err(to_js_err)?;
    Ok(serde_wasm_bindgen::to_value(&(sign_message, centralized_output)).unwrap())
}

impl From<JsErr> for JsValue {
    fn from(err: JsErr) -> Self {
        serde_wasm_bindgen::to_value(&err).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
/// Error type for better JS handling and generalization
/// of Rust / WASM -> JS error conversion.
pub struct JsErr {
    // type_: String,
    message: String,
    display: String,
}

// There is no way to implement From<anyhow::Error> for JsErr
// since the current From<Error> is generic, and it results in a conflict.
fn to_js_err(e: Error) -> JsErr {
    JsErr {
        display: format!("{}", e),
        message: e.to_string(),
    }
}