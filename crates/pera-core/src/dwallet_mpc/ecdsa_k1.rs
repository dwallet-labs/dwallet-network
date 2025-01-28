use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::StructTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use dwallet_mpc_types::dwallet_mpc::DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME;
use pera_types::id::ID;
use pera_types::PERA_SYSTEM_ADDRESS;

const ALGORITHM_SPECIFIC_DATA_STRUCT_NAME: &IdentStr = ident_str!("AlgorithmSpecificData");

/// A representation of the Move object `AlgorithmSpecificData`, which stores data specific to the
/// signing algorithm used in the MPC protocol.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct AlgorithmSpecificData {
    /// The presign object ID, which will be used as the sign MPC protocol ID.
    pub presign_id: ID,
    /// The presign protocol output, serialized as bytes.
    pub presign_output: Vec<u8>,
    /// The centralized signature of a message.
    pub message_centralized_signature: Vec<u8>,
}

impl AlgorithmSpecificData {
    /// This function returns the `StructTag` representation of the Move `AlgorithmSpecificData` object,
    /// allowing it to be compared with the corresponding Move object on the chain.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: ALGORITHM_SPECIFIC_DATA_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}
