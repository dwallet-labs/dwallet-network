use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
#[wasm_bindgen]
pub fn hello_wasm() -> Result<JsValue, JsErr> {
    println!("Hello, wasm!");
    Ok(JsValue::from_bool(false))
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
