use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use pera_types::{
    base_types::{ObjectID, PeraAddress},
    id::ID,
    PERA_SYSTEM_ADDRESS,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use pera_types::dwallet_mpc::DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME;

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

pub const CREATED_DKG_SESSION_EVENT_STRUCT_NAME: &IdentStr = ident_str!("CreatedDKGSessionEvent");
pub const COMPLETED_DKG_FIRST_ROUND_STRUCT_NAME: &IdentStr = ident_str!("CompletedDKGRoundEvent");
pub const INIT_DKG_SECOND_STRUCT_NAME: &IdentStr = ident_str!("StartDKGSecondRoundEvent");
pub const COMPLETED_DKG_SECOND_STRUCT_NAME: &IdentStr = ident_str!("CompletedSecondDKGRoundData");

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
pub struct CreatedDKGSessionEvent {
    pub session_id: ID,
    pub sender: PeraAddress,
    pub dwallet_cap_id: ID,
}

impl MPCEvent for CreatedDKGSessionEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`CreatedProofMPCEvent`] events from the chain and initiate the MPC session.
    fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: CREATED_DKG_SESSION_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CompletedDKGFirstRoundEvent {
    pub session_id: ID,
    pub sender: PeraAddress,
}

impl MPCEvent for CompletedDKGFirstRoundEvent {
    fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: COMPLETED_DKG_FIRST_ROUND_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct StartDKGSecondRoundEvent {
    pub session_id: ID,
    pub sender: PeraAddress,
    pub first_round_output: Vec<u8>,
    pub public_key_share_and_proof: Vec<u8>,
    pub dwallet_cap_id: ID,
}

impl MPCEvent for StartDKGSecondRoundEvent {
    fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: INIT_DKG_SECOND_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct CompletedDKGSecondRoundEvent {
    pub session_id: ID,
    pub sender: PeraAddress,
    pub first_round_output: Vec<u8>,
    pub public_key_share_and_proof: Vec<u8>
}

impl MPCEvent for CompletedDKGSecondRoundEvent {
    fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: COMPLETED_DKG_SECOND_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
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