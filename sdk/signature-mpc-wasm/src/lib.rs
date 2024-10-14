use wasm_bindgen::JsValue;

// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
#[wasm_bindgen]
pub fn hello_wasm() -> Result<JsValue, JsErr> {
    println!("Hello, wasm!");
    Ok(JsValue::from_serde(&true).unwrap())
}
