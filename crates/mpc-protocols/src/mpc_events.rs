use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use pera_types::{
    base_types::{ObjectID, PeraAddress},
    id::ID,
};

/// Rust version of the Move [`pera_system::dwallet::CreatedProofMPCSessionEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CreatedProofMPCSessionEvent {
    pub session_id: ID,
    pub sender: PeraAddress,
}
