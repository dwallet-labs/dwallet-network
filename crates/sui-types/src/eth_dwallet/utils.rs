use ethers::prelude::{H256, H512};
use ethers::types::H160;
use ethers::utils::rlp::RlpStream;
use eyre::Error;
use helios::consensus::types::{Bytes32, Header};
use sha3::{Digest, Keccak256};
use ssz_rs::prelude::*;

#[derive(SimpleSerialize, Default, Debug)]
struct SigningData {
    object_root: Bytes32,
    domain: Bytes32,
}

#[derive(SimpleSerialize, Default, Debug)]
struct ForkData {
    current_version: Vector<u8, 4>,
    genesis_validator_root: Bytes32,
}

/// Check if a given value is an empty string.
pub fn is_empty_value(value: &Vec<u8>) -> bool {
    let mut stream = RlpStream::new();
    stream.begin_list(4);
    stream.append_empty_data();
    stream.append_empty_data();
    let empty_storage_hash = "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421";
    stream.append(&hex::decode(empty_storage_hash).unwrap());
    let empty_code_hash = "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470";
    stream.append(&hex::decode(empty_code_hash).unwrap());
    let empty_account = stream.out();

    let is_empty_slot = value.len() == 1 && value[0] == 0x80;
    let is_empty_account = value == &empty_account;
    is_empty_slot || is_empty_account
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
