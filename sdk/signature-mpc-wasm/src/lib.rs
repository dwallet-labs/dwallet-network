// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Error;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use dwallet_mpc::create_centralized_output;

#[wasm_bindgen]
pub fn hello_wasm() -> Result<Vec<u8>, JsErr> {
    let output = create_centralized_output(Vec::new()).map_err(to_js_err)?;;
    Ok(output)
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
