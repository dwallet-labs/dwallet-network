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

pub const PROOF_MODULE_NAME: &IdentStr = ident_str!("proof");
pub const DWALLET_MODULE_NAME: &IdentStr = ident_str!("dwallet");
pub const CREATED_PROOF_STRUCT_NAME: &IdentStr = ident_str!("CreatedProofMPCSessionEvent");
pub const COMPLETED_PROOF_STRUCT_NAME: &IdentStr = ident_str!("CompletedProofMPCSessionEvent");

pub const INIT_DKG_STRUCT_NAME: &IdentStr = ident_str!("InitiateDKGSessionEvent");

/// Rust version of the Move [`pera_system::dwallet::CreatedProofMPCSessionEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CreatedProofMPCEvent {
    pub session_id: ID,
    pub sender: PeraAddress,
}

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

/// Rust version of the Move [`pera_system::dwallet::CreatedProofMPCSessionEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct InitDKGMPCEvent {
    pub session_id: ID,
    pub sender: PeraAddress,
}

impl MPCEvent for InitDKGMPCEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`CreatedProofMPCEvent`] events from the chain and initiate the MPC session.
    fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: INIT_DKG_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
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

/// Rust version of the Move [`pera_system::dwallet::CompletedProofMPCSessionEvent`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CompletedProofMPCSessionEvent {
    session_id: ID,
    sender: PeraAddress,
}

impl MPCEvent for CompletedProofMPCSessionEvent {
    /// It is used to detect [`CompletedProofMPCSessionEvent`] events from the chain and finalize the MPC session.
    fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: COMPLETED_PROOF_STRUCT_NAME.to_owned(),
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
