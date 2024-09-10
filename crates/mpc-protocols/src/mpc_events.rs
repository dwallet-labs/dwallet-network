use sui_types::{
    base_types::{ObjectID, SuiAddress},
    id::ID,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Rust version of the Move sui_system::dwallet::CreatedProofMPCSessionEvent type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CreatedProofMPCSessionEvent<E> {
    pub session_id: ID,
    pub sender: SuiAddress,
}