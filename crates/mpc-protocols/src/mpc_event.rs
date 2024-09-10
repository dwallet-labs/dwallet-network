use sui_types::{
    base_types::{ObjectID, SuiAddress},
    id::{ID, UID},
    SUI_SYSTEM_ADDRESS,
};
use move_core_types::{
    ident_str,
    identifier::IdentStr,
    language_storage::{StructTag, TypeTag},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Rust version of the Move sui_system::dwallet::CreatedProofMPCSessionEvent type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CreatedProofMPCSessionEvent<E> {
    pub session_id: ID,
    pub sender: SuiAddress,
    }