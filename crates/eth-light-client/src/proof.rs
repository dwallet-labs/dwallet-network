use ethers::prelude::Bytes;
use ethers::utils::{hex, keccak256};
use ethers::utils::rlp::{decode_list, RlpStream};
use sha3::Digest;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Proof {
    // Array of rlp-serialized MerkleTree-Nodes, starting with the storageHash-Node,
    // following the path of the SHA3 (key) as a path.
    pub proof: Vec<Bytes>,
    //  32 Bytes - SHA3 of the StorageRoot.
    // All storage will deliver a MerkleProof starting with this rootHash.
    pub root: Vec<u8>,
    // The requested storage key hash.
    pub path: Vec<u8>,
    // The storage value.
    pub value: Vec<u8>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ProofResponse {
    pub account_proof: Proof,
    pub storage_proof: Proof,
}

/// Verifies the proof of a given storage value in a Merkle Patricia Tree.
/// It takes a proof, storage root, path, and value as arguments and verifies the proof.
/// The function checks for exclusion and inclusion proofs by iterating over each node in the proof.
/// If the proof is valid, the function returns true. Otherwise, it returns false.
/// # Arguments
/// * `proof` - A reference to a slice of Bytes that represents the proof.
/// * `root` - A reference to a slice of bytes that represents the root of the Merkle Patricia Tree (of the contract's storage).
/// * `path` - A reference to a slice of bytes that represents the path to the value in the tree. it is built from hashing the key.
/// * `value` - A reference to a Vec<u8> that represents the value to be verified.
pub fn verify_proof(proof: &[Bytes], root: &[u8], path: &[u8], value: &Vec<u8>) -> bool {
    let mut expected_hash = root.to_vec();
    let mut path_offset = 0;

    for (i, node) in proof.iter().enumerate() {
        if expected_hash != keccak256(node).to_vec() {
            return false;
        }

        let node_list: Vec<Vec<u8>> = decode_list(node);

        if node_list.len() == 17 {
            let nibble = get_nibble(path, path_offset);
            if i == proof.len() - 1 {
                // exclusion proof
                let node = &node_list[nibble as usize];

                if node.is_empty() && is_empty_value(value) {
                    return true;
                }
            } else {
                expected_hash = node_list[nibble as usize].clone();
                path_offset += 1;
            }
        } else if node_list.len() == 2 {
            if i == proof.len() - 1 {
                // exclusion proof
                if !paths_match(&node_list[0], skip_length(&node_list[0]), path, path_offset)
                    && is_empty_value(value)
                {
                    return true;
                }

                // inclusion proof
                if &node_list[1] == value {
                    return paths_match(
                        &node_list[0],
                        skip_length(&node_list[0]),
                        path,
                        path_offset,
                    );
                }
            } else {
                let node_path = &node_list[0];
                let prefix_length = shared_prefix_length(path, path_offset, node_path);
                if prefix_length < node_path.len() * 2 - skip_length(node_path) {
                    // The proof shows a divergent path, but we're not
                    // at the end of the proof, so something's wrong.
                    return false;
                }
                path_offset += prefix_length;
                expected_hash = node_list[1].clone();
            }
        } else {
            return false;
        }
    }

    false
}

fn paths_match(p1: &[u8], s1: usize, p2: &[u8], s2: usize) -> bool {
    let len1 = p1.len() * 2 - s1;
    let len2 = p2.len() * 2 - s2;

    if len1 != len2 {
        return false;
    }

    for offset in 0..len1 {
        let n1 = get_nibble(p1, s1 + offset);
        let n2 = get_nibble(p2, s2 + offset);

        if n1 != n2 {
            return false;
        }
    }

    true
}

fn is_empty_value(value: &Vec<u8>) -> bool {
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

fn shared_prefix_length(path: &[u8], path_offset: usize, node_path: &[u8]) -> usize {
    let skip_length = skip_length(node_path);

    let len = std::cmp::min(
        node_path.len() * 2 - skip_length,
        path.len() * 2 - path_offset,
    );
    let mut prefix_len = 0;

    for i in 0..len {
        let path_nibble = get_nibble(path, i + path_offset);
        let node_path_nibble = get_nibble(node_path, i + skip_length);

        if path_nibble == node_path_nibble {
            prefix_len += 1;
        } else {
            break;
        }
    }

    prefix_len
}

fn skip_length(node: &[u8]) -> usize {
    if node.is_empty() {
        return 0;
    }

    let nibble = get_nibble(node, 0);
    match nibble {
        0 => 2,
        1 => 1,
        2 => 2,
        3 => 1,
        _ => 0,
    }
}

/// Gets the nibble at a given offset in the path.
/// A nibble is a four-bit aggregation, which indicates a single hexadecimal digit.
/// The nibble helps you navigate the Merkle Patricia Tree.
fn get_nibble(path: &[u8], offset: usize) -> u8 {
    let byte = path[offset / 2];
    if offset % 2 == 0 {
        byte >> 4
    } else {
        byte & 0xF
    }
}
