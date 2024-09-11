use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sui_types::{
    base_types::{ObjectID, SuiAddress},
    id::ID,
};

/// Rust version of the Move [`sui_system::dwallet::CreatedProofMPCSessionEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CreatedProofMPCSessionEvent {
    pub session_id: ID,
    pub sender: SuiAddress,
}
