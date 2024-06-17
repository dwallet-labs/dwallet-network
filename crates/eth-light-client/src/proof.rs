use ethers::prelude::Bytes;

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