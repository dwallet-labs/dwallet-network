use jsonrpsee::core::Serialize;
use serde::Deserialize;

/// Rust representation of the move struct `EncryptedNetworkDecryptionKeyShares`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedNetworkDecryptionKeyShares {
    epoch: u64,
    current_epoch_shares: Vec<Vec<u8>>,
    previous_epoch_shares: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum KeyType {
    Secp256k1,
    Ristretto,
}
