use ethers::utils::keccak256;
use helios::config::networks::Network;
use helios::consensus::nimbus_rpc::NimbusRpc;
use helios::consensus::{Bootstrap, ConsensusStateManager};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use ethers::prelude::EIP1186ProofResponse;
use helios::dwallet::light_client::ProofRequestParameters;

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

// There is no way to implement From<anyhow::Error> for JsErr
// since the current From<Error> is generic, and it results in a conflict.
fn from_anyhow_to_js_err(e: anyhow::Error) -> JsErr {
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

/// Retrieves the initial state of the Ethereum light client using BCS serialization.
///
/// This function takes in several JavaScript values, deserializes them into Rust types,
/// and initializes the Ethereum light client state. It then serializes the state back
/// into a JavaScript value for further use.
///
/// # Arguments
///
/// * `checkpoint` - A `JsValue` representing the checkpoint in hexadecimal format.
/// * `rpc` - A `JsValue` representing the RPC endpoint as a string.
/// * `network` - A `JsValue` representing the network as a string.
/// * `bootstrap` - A `JsValue` representing the bootstrap configuration.
///
/// # Returns
///
/// A `Result` containing a `JsValue` with the serialized Ethereum state value or a `JsErr` on failure.
#[wasm_bindgen]
pub fn get_initial_state_bcs(
    checkpoint: JsValue,
    rpc: JsValue,
    network: JsValue,
    bootstrap: JsValue,
) -> Result<JsValue, JsErr> {
    let checkpoint: String = serde_wasm_bindgen::from_value(checkpoint)?;
    let rpc: String = serde_wasm_bindgen::from_value(rpc)?;
    let network: String = serde_wasm_bindgen::from_value(network)?;
    let mut bootstrap: Bootstrap = serde_wasm_bindgen::from_value(bootstrap)?;

    let network = Network::from_str(&network)?;
    let checkpoint = hex::decode(checkpoint.strip_prefix("0x").unwrap())?;
    let state = ConsensusStateManager::<NimbusRpc>::new_from_checkpoint_and_bootstrap(
        checkpoint,
        network,
        rpc,
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

/// Calculates the current finalized period of the Ethereum light client state.
#[wasm_bindgen]
pub fn get_current_period(state_bytes: JsValue) -> Result<JsValue, JsErr> {
    let state_bytes: Vec<u8> = serde_wasm_bindgen::from_value(state_bytes)?;
    let mut eth_state = bcs::from_bytes::<ConsensusStateManager<NimbusRpc>>(&state_bytes)?;
    Ok(serde_wasm_bindgen::to_value(&eth_state.get_sync_period())?)
}

#[wasm_bindgen]
pub fn try_verify_proof(
    proof: JsValue,
    contract_address: JsValue,
    message: JsValue,
    dwallet_id: JsValue,
    data_slot: JsValue,
    state_root: JsValue,
) -> Result<bool, JsErr> {
    let proof: EIP1186ProofResponse = serde_wasm_bindgen::from_value(proof)?;
    let contract_address: helios::types::Address =
        serde_wasm_bindgen::from_value(contract_address)?;
    let data_slot: String = serde_wasm_bindgen::from_value(data_slot)?;
    let proof_params = ProofRequestParameters {
        message: serde_wasm_bindgen::from_value(message)?,
        dwallet_id: serde_wasm_bindgen::from_value(dwallet_id)?,
        data_slot: u64::from_str(&data_slot)?,
    };
    let state_root: Vec<u8> = serde_wasm_bindgen::from_value(state_root)?;

    let result =
        light_client_helpers::try_verify_proof(proof, &contract_address, proof_params, state_root)
            .map_err(from_anyhow_to_js_err)?;

    Ok(result)
}
