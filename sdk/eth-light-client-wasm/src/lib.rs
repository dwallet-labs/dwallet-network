use ethers::utils::keccak256;
use helios::config::networks::Network;
use helios::consensus::nimbus_rpc::NimbusRpc;
use helios::consensus::{Bootstrap, ConsensusStateManager};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
/// Error type for better JS handling and generalization
/// of Rust / WASM -> JS error conversion.
pub struct JsErr {
    pub message: String,
    pub display: String,
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

// There is no way to implement From<eyre::Error> for JsErr
// since the current From<Error> is generic, and it results in a conflict.
fn from_eyre_to_js_err(e: eyre::Report) -> JsErr {
    JsErr {
        display: format!("{}", e),
        message: e.to_string(),
    }
}

#[derive(Deserialize, Serialize)]
pub struct EthereumStateValue {
    pub object: ConsensusStateManager<NimbusRpc>,
    pub bytes: Vec<u8>,
    pub hash: String,
}

#[wasm_bindgen]
pub fn get_initial_state_bcs(
    checkpoint: JsValue,
    rpc: JsValue,
    network: JsValue,
    bootstrap: JsValue,
) -> Result<JsValue, JsErr> {
    let checkpoint_value: String = serde_wasm_bindgen::from_value(checkpoint)?;
    let rpc_value: String = serde_wasm_bindgen::from_value(rpc)?;
    let network_value: String = serde_wasm_bindgen::from_value(network)?;
    let mut bootstrap: Bootstrap = serde_wasm_bindgen::from_value(bootstrap)?;

    let network = Network::from_str(&network_value)?;
    let checkpoint = hex::decode(checkpoint_value.strip_prefix("0x").unwrap())?;
    let state = ConsensusStateManager::<NimbusRpc>::new_from_checkpoint_and_bootstrap(
        checkpoint,
        network,
        rpc_value,
        &mut bootstrap,
    )
    .map_err(from_eyre_to_js_err)?;

    let state_bytes = bcs::to_bytes(&state)?;
    let ethereum_state_value = EthereumStateValue {
        object: state,
        bytes: state_bytes.clone(),
        hash: hex::encode(keccak256(&state_bytes)),
    };

    Ok(serde_wasm_bindgen::to_value(&ethereum_state_value)?)
}

#[wasm_bindgen]
pub fn get_current_period(state_bytes: JsValue) -> Result<JsValue, JsErr> {
    let state_bytes: Vec<u8> = serde_wasm_bindgen::from_value(state_bytes)?;
    let mut eth_state = bcs::from_bytes::<ConsensusStateManager<NimbusRpc>>(&state_bytes)?;
    Ok(serde_wasm_bindgen::to_value(&eth_state.get_sync_period())?)
}
