use anyhow::anyhow;
use ethers::prelude::H256;
use eyre::Error;
use helios::consensus::types::{Bytes32, Header};
use serde_json::{Number, Value};
use sha3::{Digest, Keccak256};
use ssz_rs::prelude::*;

use sui_json_rpc_types::{SuiData, SuiObjectDataOptions, SuiRawData};
use sui_sdk::json::SuiJsonValue;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::eth_dwallet_cap::EthDWalletCap;
use sui_types::object::Owner;
use sui_types::transaction::SharedInputObject;

use crate::eth_state::{EthStateObject, LatestEthStateObject};

#[derive(SimpleSerialize, Default, Debug)]
struct SigningData {
    object_root: Bytes32,
    domain: Bytes32,
}

pub struct SuiRawDataWrapper(pub SuiRawData);

impl TryFrom<SuiRawDataWrapper> for EthDWalletCap {
    type Error = anyhow::Error;
    fn try_from(wrapper: SuiRawDataWrapper) -> std::result::Result<Self, anyhow::Error> {
        wrapper
            .0
            .try_as_move()
            .ok_or_else(|| anyhow::anyhow!("Object is not a Move Object"))?
            .deserialize()
            .map_err(|e| anyhow::anyhow!("Error deserializing object: {e}"))
    }
}

impl TryFrom<SuiRawDataWrapper> for EthStateObject {
    type Error = anyhow::Error;
    fn try_from(wrapper: SuiRawDataWrapper) -> std::result::Result<Self, anyhow::Error> {
        wrapper
            .0
            .try_as_move()
            .ok_or_else(|| anyhow::anyhow!("Object is not a Move Object"))?
            .deserialize()
            .map_err(|e| anyhow::anyhow!("Error deserializing object: {e}"))
    }
}

impl TryFrom<SuiRawDataWrapper> for LatestEthStateObject {
    type Error = anyhow::Error;
    fn try_from(wrapper: SuiRawDataWrapper) -> std::result::Result<Self, anyhow::Error> {
        wrapper
            .0
            .try_as_move()
            .ok_or_else(|| anyhow::anyhow!("Object is not a Move Object"))?
            .deserialize()
            .map_err(|e| anyhow::anyhow!("Error deserializing object: {e}"))
    }
}

#[derive(SimpleSerialize, Default, Debug)]
struct ForkData {
    current_version: Vector<u8, 4>,
    genesis_validator_root: Bytes32,
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

/// Performs a Merkle Proof on the given parameters.
/// Checks whether the path that is given as `branch`, and the `leaf_object`
/// constructs the correct merkle tree.
/// Ultimately, the hash of the merkle tree that is constructed should
/// be equal to the attested header's state root.
pub fn is_proof_valid<L: Merkleized>(
    attested_header: &Header,
    leaf_object: &mut L,
    branch: &[Bytes32],
    depth: usize,
    index: usize,
) -> bool {
    let res: Result<bool, eyre::Error> = (move || {
        let leaf_hash = leaf_object.hash_tree_root()?;
        let state_root = bytes32_to_node(&attested_header.state_root)?;
        let branch = branch_to_nodes(branch.to_vec())?;

        let is_valid = is_valid_merkle_branch(&leaf_hash, branch.iter(), depth, index, &state_root);
        Ok(is_valid)
    })();

    if let Ok(is_valid) = res {
        is_valid
    } else {
        false
    }
}

fn serialize_object<T>(object: &T) -> Result<SuiJsonValue, anyhow::Error>
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

async fn get_object_bcs_by_id(
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

async fn get_shared_object_input_by_id(
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

pub fn compute_signing_root(object_root: Bytes32, domain: Bytes32) -> Result<Node, Error> {
    let mut data = SigningData {
        object_root,
        domain,
    };
    Ok(data.hash_tree_root()?)
}

pub fn branch_to_nodes(branch: Vec<Bytes32>) -> Result<Vec<Node>, eyre::Error> {
    branch
        .iter()
        .map(bytes32_to_node)
        .collect::<Result<Vec<Node>, eyre::Error>>()
}

pub fn bytes32_to_node(bytes: &Bytes32) -> Result<Node, eyre::Error> {
    Ok(Node::try_from(bytes.as_slice())?)
}

pub fn calc_sync_period(slot: u64) -> u64 {
    let epoch = slot / 32; // 32 slots per epoch
    epoch / 256 // 256 epochs per sync committee
}

pub fn compute_domain(
    domain_type: &[u8],
    fork_version: Vector<u8, 4>,
    genesis_root: Bytes32,
) -> Result<Bytes32, Error> {
    let fork_data_root = compute_fork_data_root(fork_version, genesis_root)?;
    let start = domain_type;
    let end = &fork_data_root.as_ref()[..28];
    let d = [start, end].concat();
    Ok(d.to_vec().try_into().unwrap())
}

fn compute_fork_data_root(
    current_version: Vector<u8, 4>,
    genesis_validator_root: Bytes32,
) -> Result<Node, Error> {
    let mut fork_data = ForkData {
        current_version,
        genesis_validator_root,
    };
    Ok(fork_data.hash_tree_root()?)
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

mod tests {
    use helios::consensus::types::primitives::ByteVector;
    use super::*;

    #[test]
    fn standardize_slot_input_valid() {
        let input_zero = 0u64;
        let expected = [0u8; 32];
        assert_eq!(standardize_slot_input(input_zero), H256::from_slice(&expected));

        let input_one = 1u64;
        let expected: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

        assert_eq!(standardize_slot_input(input_one), H256::from_slice(&expected));

        let input = u64::MAX;
        let expected: [u8; 32] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255
        ];
        assert_eq!(standardize_slot_input(input), H256::from_slice(&expected));
    }

    #[test]
    fn standardize_key_input_valid() {
        let input_zero = H256::from_slice(&[0u8; 32]);
        let expected_zero = H256::from_slice(&[0u8; 32]);
        assert_eq!(standardize_key_input(input_zero), expected_zero);

        let input = H256::from_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
        ]);
        let expected = H256::from_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
        ]);
        assert_eq!(standardize_key_input(input), expected);
    }

    #[test]
    fn calculate_mapping_slot_valid() {
        let key = H256::from_slice(&[0u8; 32]);
        let slot = 0;

        let expected_hash = {
            let mut hasher = Keccak256::new();
            hasher.update(&[0u8; 32]);
            hasher.update(&[0u8; 32]);
            H256::from_slice(&hasher.finalize())
        };

        assert_eq!(calculate_mapping_slot(key, slot), expected_hash);

        let key = H256::from_slice(&[1u8; 32]);
        let slot = u64::MAX;

        let expected_hash = {
            let mut hasher = Keccak256::new();
            hasher.update(&[1u8; 32]);
            hasher.update([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255
            ]);
            H256::from_slice(&hasher.finalize())
        };

        assert_eq!(calculate_mapping_slot(key, slot), expected_hash);
    }

    #[test]
    fn calculate_key_valid() {
        let message: Vec<u8> = vec![];
        let dwallet_id: Vec<u8> = vec![];
        let expected_hash = {
            let mut hasher = Keccak256::new();
            hasher.update(&[]);
            H256::from_slice(&hasher.finalize())
        };

        assert_eq!(calculate_key(message, dwallet_id), expected_hash);

        let dwallet_id = "be344ddffaa7a8c9c5ae7f2d09a77f20ed54f93bf5e567659feca5c3422ae7a6";
        let byte_vec_dwallet_id = hex::decode(dwallet_id).expect("Invalid hex string");
        let mut message = [1u8; 32].to_vec();

        let expected_hash = {
            let mut hasher = Keccak256::new();
            let mut combined = message.clone();
            combined.extend_from_slice(&byte_vec_dwallet_id);
            hasher.update(combined);
            H256::from_slice(&hasher.finalize())
        };

        assert_eq!(calculate_key(message, byte_vec_dwallet_id), expected_hash)
    }

    #[test]
    fn compute_signing_root_valid() {
        let object_root = ByteVector::<32>::default();
        let domain = ByteVector::<32>::default();

        let mut data = SigningData {
            object_root: object_root.clone(),
            domain: domain.clone(),
        };

        let result = compute_signing_root(object_root, domain);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), data.hash_tree_root().unwrap());
    }
}