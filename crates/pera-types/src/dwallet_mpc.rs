use serde::{Deserialize, Serialize};

/// Rust representation of the move struct `EncryptionOfNetworkDecryptionKeyShares`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptionOfNetworkDecryptionKeyShares {
    epoch: u64,
    pub current_epoch_shares: Vec<Vec<u8>>,
    previous_epoch_shares: Vec<Vec<u8>>,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum DWalletMPCNetworkKey {
    Secp256k1 = 1,
    Ristretto = 2,
}
