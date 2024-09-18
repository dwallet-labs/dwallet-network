use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::StructTag;
use pera_types::{
    base_types::{ObjectID, PeraAddress},
    id::ID,
    PERA_SYSTEM_ADDRESS,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Rust version of the Move [`pera_system::dwallet::CreatedProofMPCSessionEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CreatedProofMPCEvent {
    pub session_id: ID,
    pub sender: PeraAddress,
}

pub const PROOF_MODULE_NAME: &IdentStr = ident_str!("proof");
pub const CREATED_PROOF_STRUCT_NAME: &IdentStr = ident_str!("CreatedProofMPCSessionEvent");

pub trait MPCEvent {
    fn type_() -> StructTag;
}

impl MPCEvent for CreatedProofMPCEvent {
    fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: CREATED_PROOF_STRUCT_NAME.to_owned(),
            module: PROOF_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}
