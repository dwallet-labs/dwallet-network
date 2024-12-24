use serde::{Deserialize, Serialize};
use pera_types::dwallet_mpc_error::DwalletMPCResult;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DwalletMPCKeyParser {

}

impl DwalletMPCKeyParser {
    pub fn get_protocol_public_parameters(&self) -> DwalletMPCResult<Vec<u8>> {
        todo!()
    }

    pub fn new(dkg_output: Vec<u8>, ) -> DwalletMPCResult<Self> {
        todo!()
    }
}