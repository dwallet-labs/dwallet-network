use anyhow::anyhow;
use ethers::prelude::H256;
use ethers::utils::__serde_json::{Number, Value};
use sha3::{Keccak256, Digest};
use sui_sdk::json::SuiJsonValue;
use sui_sdk::rpc_types::{SuiData, SuiObjectDataOptions, SuiRawData};
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::types::object::Owner;
use sui_sdk::types::transaction::SharedInputObject;
use sui_sdk::wallet_context::WalletContext;
use sui_types::eth_dwallet_cap::EthDWalletCap;
use eyre::Error;

// todo(shay): maybe put this struct in sui-types
pub struct SuiRawDataWrapper(pub SuiRawData);

impl TryFrom<SuiRawDataWrapper> for EthDWalletCap {
    type Error = anyhow::Error;
    fn try_from(wrapper: SuiRawDataWrapper) -> Result<Self, anyhow::Error> {
        wrapper
            .0
            .try_as_move()
            .ok_or_else(|| anyhow::anyhow!("Object is not a Move Object"))?
            .deserialize()
            .map_err(|e| anyhow::anyhow!("Error deserializing object: {e}"))
    }
}

/// This function standardizes the input slot for a given unsigned 64-bit integer.
/// It first converts the integer into a hexadecimal string representation.
/// Then, it pads the hexadecimal string to ensure it has a length of 64 characters.
/// We pad the string because in solidity, the slot is a 256-bit hash (H256).
/// Finally,
/// it decodes the padded hexadecimal string back into bytes and converts it into a 256-bit hash
/// (H256).
/// # Arguments
/// * `input` - An unsigned 64-bit integer that represents the input slot.
/// # Returns
/// * A 256-bit hash (H256) that represents the standardized input slot.
pub fn standardize_slot_input(input: u64) -> H256 {
    let hex_str = format!("{:x}", input);
    let padded_hex_str = format!("{:0>64}", hex_str);
    H256::from_slice(&hex::decode(padded_hex_str).unwrap_or_default())
}

/// This function standardizes the input key for a given 256-bit hash (H256).
/// It first converts the hash into a hexadecimal string representation.
/// Then, it pads the hexadecimal string to ensure it has a length of 64 characters.
/// We pad the string because in solidity, the slot is a 256-bit hash (H256).
/// Finally,
/// it decodes the padded hexadecimal string back into bytes and converts it into a 256-bit hash
/// (H256).
/// # Arguments
/// * `input` - A 256-bit hash (H256) that represents the input key.
/// # Returns
/// * A 256-bit hash (H256) that represents the standardized input key.
pub fn standardize_key_input(input: H256) -> H256 {
    let hex_str = format!("{:x}", input);
    let padded_hex_str = format!("{:0>64}", hex_str);
    H256::from_slice(&hex::decode(padded_hex_str).unwrap_or_default())
}

/// Calculates the mapping slot for a given key and storage slot (in the contract's storage layout).
/// First initializes a new `Keccak256` hasher, then standardizes the input slot and key.
/// The standardized key and slot are then hashed together to produce a new `H256` hash.
/// The result hash will be used to get the location of the
/// (key, value) pair in the contract's storage.
/// # Arguments
/// * `key` - A H256 hash that represents the key for which the mapping slot is to be calculated.
/// The Key is `Keccak256(message + dwallet_id)`.
/// * `Mapping_slot` - A `u64` value that represents the mapping slot in the contract storage layout.
/// For more info:
/// https://docs.soliditylang.org/en/v0.8.24/internals/layout_in_storage.html#mappings-and-dynamic-arrays
pub fn calculate_mapping_slot(key: H256, mapping_slot: u64) -> H256 {
    let mut hasher = Keccak256::new();
    let slot_padded = standardize_slot_input(mapping_slot);
    let key_padded = standardize_key_input(key);
    hasher.update(key_padded.as_bytes());
    hasher.update(slot_padded.as_bytes());
    H256::from_slice(&hasher.finalize())
}

/// Calculates the key for a given message and dWallet ID.
/// In the smart contract, the key is calculated by hashing the message and the dWallet id together.
/// The result is a H256 hash that represents the key.
pub fn calculate_key(mut message: Vec<u8>, dwallet_id: Vec<u8>) -> H256 {
    let mut hasher = Keccak256::new();
    message.extend_from_slice(dwallet_id.as_slice());
    hasher.update(message);
    H256::from_slice(&hasher.finalize())
}

pub fn serialize_object<T>(object: &T) -> Result<SuiJsonValue, anyhow::Error>
    where
        T: ?std::marker::Sized + serde::Serialize,
{
    let object_bytes = bcs::to_bytes(&object)?;
    let object_json = object_bytes
        .iter()
        .map(|v| Value::Number(Number::from(*v)))
        .collect();
    Ok(SuiJsonValue::new(Value::Array(object_json))?)
}

pub async fn get_object_bcs_by_id(
    context: &mut WalletContext,
    object_id: ObjectID,
) -> Result<SuiRawDataWrapper, anyhow::Error> {
    let object_resp = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(
            object_id,
            SuiObjectDataOptions::default().with_bcs().with_owner(),
        )
        .await?;

    match object_resp.data {
        Some(data) => Ok(SuiRawDataWrapper(
            data.bcs.ok_or_else(|| anyhow!("missing object data"))?,
        )),
        None => Err(anyhow!("Could not find object with ID: {:?}", object_id)),
    }
}

pub async fn get_shared_object_input_by_id(
    context: &mut WalletContext,
    object_id: ObjectID,
) -> Result<SharedInputObject, anyhow::Error> {
    let object_resp = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(object_id, SuiObjectDataOptions::default().with_owner())
        .await?;

    match object_resp.data {
        Some(_) => {
            let owner = object_resp
                .owner()
                .ok_or_else(|| anyhow!("missing object owner"))?;
            let initial_shared_version = match owner {
                Owner::Shared {
                    initial_shared_version,
                } => initial_shared_version,
                _ => return Err(anyhow!("Object is not shared")),
            };
            Ok(SharedInputObject {
                id: object_id,
                initial_shared_version,
                mutable: true,
            })
        }
        None => Err(anyhow!("Could not find object with ID: {:?}", object_id)),
    }
}

pub fn get_message_storage_slot(
    message: String,
    dwallet_id: Vec<u8>,
    data_slot: u64,
) -> Result<H256, Error> {
    // Calculate memory slot.
    // Each mapping slot is calculated by concatenating of the msg and dWalletID.
    let key = calculate_key(
        message.clone().as_bytes().to_vec(),
        dwallet_id.as_slice().to_vec(),
    );
    Ok(calculate_mapping_slot(key, data_slot))
}