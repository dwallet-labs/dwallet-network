use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::StructTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sui_types::{
    base_types::{ObjectID, SuiAddress},
    id::ID,
    SUI_SYSTEM_ADDRESS,
};

pub const PROOF_MODULE_NAME: &IdentStr = ident_str!("proof");
pub const CREATED_PROOF_STRUCT_NAME: &IdentStr = ident_str!("CreatedProofMPCSessionEvent");

pub trait MPCEvent {
    fn type_() -> StructTag;
}

/// Rust version of the Move [`sui_system::dwallet::CreatedProofMPCSessionEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CreatedProofMPCEvent {
    pub session_id: ID,
    pub sender: SuiAddress,
}

impl MPCEvent for CreatedProofMPCEvent {
    fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: CREATED_PROOF_STRUCT_NAME.to_owned(),
            module: PROOF_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}
