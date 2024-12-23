use crate::dwallet_mpc_error::DwalletMPCError;
use serde::{Deserialize, Serialize};

/// Rust representation of the move struct `EncryptionOfNetworkDecryptionKeyShares`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptionOfNetworkDecryptionKeyShares {
    pub epoch: u64,
    pub current_epoch_shares: Vec<Vec<u8>>,
    pub previous_epoch_shares: Vec<Vec<u8>>,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash, Copy)]
pub enum DWalletMPCNetworkKey {
    Secp256k1 = 1,
    Ristretto = 2,
}

impl TryFrom<u8> for DWalletMPCNetworkKey {
    type Error = DwalletMPCError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DWalletMPCNetworkKey::Secp256k1),
            2 => Ok(DWalletMPCNetworkKey::Ristretto),
            _ => Err(DwalletMPCError::InvalidDWalletMPCNetworkKey),
        }
    }
}
