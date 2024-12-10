use jsonrpsee::core::Serialize;
use move_core_types::{ident_str, identifier::IdentStr};
use serde::Deserialize;

pub const DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME: &IdentStr = ident_str!("dwallet_2pc_mpc_ecdsa_k1");

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
