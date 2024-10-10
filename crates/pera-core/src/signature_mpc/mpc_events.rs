use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use pera_types::{
    base_types::{ObjectID, PeraAddress},
    id::ID,
    PERA_SYSTEM_ADDRESS,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Generic trait for all MPC events.
pub trait MPCEvent {
    /// This function allows comparing this event with the Move event.
    fn type_() -> StructTag;
    /// The session ID of the MPC session.
    fn session_id(&self) -> ID;
    /// The address of the event emitter.
    fn event_emitter(&self) -> PeraAddress;
}

/// Rust version of the Move [`pera_system::dwallet::CreatedProofMPCSessionEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CreatedProofMPCEvent {
    pub session_id: ID,
    pub sender: PeraAddress,
}

pub const PROOF_MODULE_NAME: &IdentStr = ident_str!("proof");
pub const CREATED_PROOF_STRUCT_NAME: &IdentStr = ident_str!("CreatedProofMPCSessionEvent");

impl MPCEvent for CreatedProofMPCEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`CreatedProofMPCEvent`] events from the chain and initiate the MPC session.
    fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: CREATED_PROOF_STRUCT_NAME.to_owned(),
            module: PROOF_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    fn session_id(&self) -> ID {
        self.session_id.clone()
    }

    fn event_emitter(&self) -> PeraAddress {
        self.sender.clone()
    }
}
