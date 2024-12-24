use crate::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use serde::{Deserialize, Serialize};

/// Rust representation of the move struct `DwalletMPCNetworkKey`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DwalletMPCNetworkKey {
    pub epoch: u64,
    pub current_epoch_shares: Vec<Vec<u8>>,
    pub previous_epoch_shares: Vec<Vec<u8>>,
    pub protocol_public_parameters: Vec<u8>,
    pub decryption_public_parameters: Vec<u8>,

}

impl DwalletMPCNetworkKey {
    pub fn get_protocol_public_parameters(&self) -> DwalletMPCResult<Vec<u8>> {
        todo!()
    }

    pub fn new(dkg_output: Vec<u8>, key_scheme: DWalletMPCNetworkKeyScheme, epoch: u64) -> DwalletMPCResult<Self> {
        todo!()
    }
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash, Copy)]
pub enum DWalletMPCNetworkKeyScheme {
    Secp256k1 = 1,
    Ristretto = 2,
}

impl TryFrom<u8> for DWalletMPCNetworkKeyScheme {
    type Error = DwalletMPCError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DWalletMPCNetworkKeyScheme::Secp256k1),
            2 => Ok(DWalletMPCNetworkKeyScheme::Ristretto),
            _ => Err(DwalletMPCError::InvalidDWalletMPCNetworkKey),
        }
    }
}
