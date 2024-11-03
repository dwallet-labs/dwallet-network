// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Error;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use dwallet_mpc::create_dkg_output;
use log::debug;

#[wasm_bindgen]
pub fn create_dkg_centralized_output(dkg_first_round_output: Vec<u8>, session_id: String) -> Result<JsValue, JsErr> {
    let (public_key_share_and_proof, centralized_output) = create_dkg_output(dkg_first_round_output, session_id).map_err(to_js_err)?;
    // Ok(serde_wasm_bindgen::to_value(&(public_key_share_and_proof, centralized_output)).map_err(to_js_err)?)
Ok(serde_wasm_bindgen::to_value(&(public_key_share_and_proof, centralized_output)).unwrap())
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

// impl<T: std::error::Error> From<T> for JsErr {
//     fn from(err: T) -> Self {
//         JsErr {
//             display: format!("{}", err),
//             message: err.to_string(),
//         }
//     }
// }

// impl From<JsErr> for JsValue {
//     fn from(err: JsErr) -> Self {
//         serde_wasm_bindgen::to_value(&err).unwrap()
//     }
// }

// There is no way to implement From<anyhow::Error> for JsErr
// since the current From<Error> is generic, and it results in a conflict.
fn to_js_err(e: Error) -> JsErr {
    JsErr {
        display: format!("{}", e),
        message: e.to_string(),
    }
}