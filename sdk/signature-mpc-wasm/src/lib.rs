// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Error;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use dwallet_mpc::create_centralized_output;
use log::debug;

#[wasm_bindgen]
pub fn hello_wasm(dkg_first_round_output: Vec<u8>) -> Result<Vec<u8>, JsErr> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    debug!("hello wasm {:?}", dkg_first_round_output);
    let output = match create_centralized_output(dkg_first_round_output) {
        Ok(output) => output,
        Err(e) => {debug!("{:?}", e);return Ok(vec![1,2]);},
    };
    Ok(output)
}


// impl<T: std::error::Error> From<T> for JsErr {
//     fn from(err: T) -> Self {
//         JsErr {
//             display: format!("{}", err),
//             message: err.to_string(),
//         }
//     }
// }

impl From<JsErr> for JsValue {
    fn from(err: JsErr) -> Self {
        serde_wasm_bindgen::to_value(&err).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
