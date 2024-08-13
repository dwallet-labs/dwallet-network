// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::{
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
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub const DWALLET_MODULE_NAME: &IdentStr = ident_str!("dwallet");
pub const NEW_SIGN_SESSION_EVENT_STRUCT_NAME: &IdentStr = ident_str!("NewSignSessionEvent");

pub const MESSAGE_APPROVAL_STRUCT_NAME: &IdentStr = ident_str!("MessageApproval");
pub const APPROVE_MESSAGES_FUNC_NAME: &IdentStr = ident_str!("approve_messages");
pub const SIGN_FUNC_NAME: &IdentStr = ident_str!("sign");

pub const DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME: &IdentStr = ident_str!("dwallet_2pc_mpc_ecdsa_k1");
pub const NEW_DKG_SESSION_EVENT_STRUCT_NAME: &IdentStr = ident_str!("NewDKGSessionEvent");
pub const NEW_PRESIGN_SESSION_EVENT_STRUCT_NAME: &IdentStr = ident_str!("NewPresignSessionEvent");
pub const NEW_SIGN_DATA_EVENT_STRUCT_NAME: &IdentStr = ident_str!("NewSignDataEvent");

pub const DWALLET_STRUCT_NAME: &IdentStr = ident_str!("DWallet");
pub const DKG_SESSION_STRUCT_NAME: &IdentStr = ident_str!("DKGSession");
pub const DKG_SESSION_OUTPUT_STRUCT_NAME: &IdentStr = ident_str!("DKGSessionOutput");
pub const PRESIGN_SESSION_STRUCT_NAME: &IdentStr = ident_str!("PresignSession");
pub const PRESIGN_SESSION_OUTPUT_STRUCT_NAME: &IdentStr = ident_str!("PresignSessionOutput");
pub const PRESIGN_STRUCT_NAME: &IdentStr = ident_str!("Presign");
pub const SIGN_DATA_STRUCT_NAME: &IdentStr = ident_str!("SignData");

pub const SIGN_SESSION_STRUCT_NAME: &IdentStr = ident_str!("SignSession");
pub const SIGN_OUTPUT_STRUCT_NAME: &IdentStr = ident_str!("SignOutput");
pub const CREATE_DKG_SESSION_FUNC_NAME: &IdentStr = ident_str!("create_dkg_session");
pub const CREATE_DKG_OUTPUT_FUNC_NAME: &IdentStr = ident_str!("create_dkg_output");
pub const CREATE_DWALLET_FUNC_NAME: &IdentStr = ident_str!("create_dwallet");
pub const CREATE_PRESIGN_SESSION_FUNC_NAME: &IdentStr = ident_str!("create_presign_session");
pub const CREATE_PRESIGN_OUTPUT_FUNC_NAME: &IdentStr = ident_str!("create_presign_output");
pub const CREATE_PRESIGN_FUNC_NAME: &IdentStr = ident_str!("create_presign");
pub const CREATE_PARTIAL_USER_SIGNED_MESSAGES_FUNC_NAME: &IdentStr =
    ident_str!("create_partial_user_signed_messages");
pub const CREATE_SIGN_OUTPUT_FUNC_NAME: &IdentStr = ident_str!("verify_and_create_sign_output");

// <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<

// Rust version of the Move sui_system::dwallet::NewSignSessionEvent type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct NewSignSessionEvent<E> {
    pub session_id: ID,
    pub dwallet_id: ID,
    pub dwallet_cap_id: ID,
    pub messages: Vec<Vec<u8>>,
    pub sender: SuiAddress,
    pub sign_data_event: E,
}

impl<E> NewSignSessionEvent<E> {
    pub fn type_(type_param: TypeTag) -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: NEW_SIGN_SESSION_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![type_param],
        }
    }
}

impl<E: Serialize + DeserializeOwned> NewSignSessionEvent<E> {
    /// Create from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

// Rust version of the Move sui_system::dwallet_2pc_mpc_ecdsa_k1::NewDKGSessionEvent type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct NewDKGSessionEvent {
    pub session_id: ID,
    pub dwallet_cap_id: ID,
    pub commitment_to_centralized_party_secret_key_share: Vec<u8>,
    pub sender: SuiAddress,
}

impl NewDKGSessionEvent {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: NEW_DKG_SESSION_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    /// Create from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

// Rust version of the Move sui_system::dwallet_2pc_mpc_ecdsa_k1::NewPresignSessionEvent type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct NewPresignSessionEvent {
    pub session_id: ID,
    pub dwallet_id: ID,
    pub dwallet_cap_id: ID,
    pub hash: u8,
    pub dkg_output: Vec<u8>,
    pub commitments_and_proof_to_centralized_party_nonce_shares: Vec<u8>,
    pub messages: Vec<Vec<u8>>,
    pub sender: SuiAddress,
}

impl NewPresignSessionEvent {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: NEW_PRESIGN_SESSION_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    /// Create from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

// Rust version of the Move sui_system::dwallet_2pc_mpc_ecdsa_k1::NewSignDataEvent type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct NewSignDataEvent {
    pub presign_session_id: ID,
    pub hash: u8,
    pub dkg_output: Vec<u8>,
    pub public_nonce_encrypted_partial_signature_and_proofs: Vec<u8>,
    pub presigns: Vec<u8>,
}

impl NewSignDataEvent {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: NEW_SIGN_DATA_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    /// Create from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

// <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<

// <<<<<<<<<<<<<<<<<<<<<<<< Objects <<<<<<<<<<<<<<<<<<<<<<<<

// Rust version of the Move sui_system::dwallet::SignSession type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct SignSession<S> {
    pub id: UID,
    pub dwallet_id: ID,
    pub dwallet_cap_id: ID,
    pub messages: Vec<Vec<u8>>,
    pub sender: SuiAddress,
    pub sign_data: S,
}

impl<S: Serialize + DeserializeOwned> SignSession<S> {
    pub fn type_(type_param: TypeTag) -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: SIGN_SESSION_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![type_param],
        }
    }

    /// Create from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn id(&self) -> &ObjectID {
        self.id.object_id()
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

// Rust version of the Move sui_system::dwallet::SignOutput type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct SignOutput {
    pub id: UID,
    pub session_id: ID,
    pub dwallet_id: ID,
    pub dwallet_cap_id: ID,
    pub signatures: Vec<Vec<u8>>,
    pub sender: SuiAddress,
}

impl SignOutput {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: SIGN_OUTPUT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    /// Create from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn id(&self) -> &ObjectID {
        self.id.object_id()
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

// Rust version of the Move sui_system::dwallet_2pc_mpc_ecdsa_k1::DWallet type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct DWallet {
    pub id: UID,
    pub session_id: ID,
    pub dwallet_cap_id: ID,
    pub output: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl DWallet {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: DWALLET_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    /// Create from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn id(&self) -> &ObjectID {
        self.id.object_id()
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

// Rust version of the Move sui_system::dwallet_2pc_mpc_ecdsa_k1::DKGSession type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct DKGSession {
    pub id: UID,
    pub dwallet_cap_id: ID,
    pub commitment_to_centralized_party_secret_key_share: Vec<u8>,
    pub sender: SuiAddress,
}

impl DKGSession {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: DKG_SESSION_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    /// Create from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn id(&self) -> &ObjectID {
        self.id.object_id()
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

// Rust version of the Move sui_system::dwallet_2pc_mpc_ecdsa_k1::DKGSessionOutput type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct DKGSessionOutput {
    pub id: UID,
    pub session_id: ID,
    pub dwallet_cap_id: ID,
    pub commitment_to_centralized_party_secret_key_share: Vec<u8>,
    pub secret_key_share_encryption_and_proof: Vec<u8>,
}

impl DKGSessionOutput {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: DKG_SESSION_OUTPUT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    pub fn is_type(other: &StructTag) -> bool {
        other.address == SUI_SYSTEM_ADDRESS
            && other.module.as_ident_str() == DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME
            && other.name.as_ident_str() == DKG_SESSION_OUTPUT_STRUCT_NAME
    }

    /// Create from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn id(&self) -> &ObjectID {
        self.id.object_id()
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

// Rust version of the Move sui_system::dwallet_2pc_mpc_ecdsa_k1::PresignSession type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct PresignSession {
    pub id: UID,
    pub dwallet_id: ID,
    pub dwallet_cap_id: ID,
    pub hash: u8,
    pub commitments_and_proof_to_centralized_party_nonce_shares: Vec<u8>,
    pub messages: Vec<Vec<u8>>,
    pub sender: SuiAddress,
}

impl PresignSession {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: PRESIGN_SESSION_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    /// Create from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn id(&self) -> &ObjectID {
        self.id.object_id()
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

// Rust version of the Move sui_system::dwallet_2pc_mpc_ecdsa_k1::PresignSessionOutput type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct PresignSessionOutput {
    pub id: UID,
    pub session_id: ID,
    pub dwallet_id: ID,
    pub dwallet_cap_id: ID,
    pub output: Vec<u8>,
}

impl PresignSessionOutput {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: PRESIGN_SESSION_OUTPUT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    /// Create from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn id(&self) -> &ObjectID {
        self.id.object_id()
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

// Rust version of the Move sui_system::dwallet_2pc_mpc_ecdsa_k1::Presign type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct Presign {
    pub id: UID,
    pub session_id: ID,
    pub dwallet_id: ID,
    pub dwallet_cap_id: ID,
    pub presigns: Vec<u8>,
}

impl Presign {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: PRESIGN_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    /// Create from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn id(&self) -> &ObjectID {
        self.id.object_id()
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

// Rust version of the Move sui_system::dwallet_2pc_mpc_ecdsa_k1::SignData type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct SignData {
    pub id: UID,
    pub session_id: ID,
    pub hash: u8,
    pub public_nonce_encrypted_partial_signature_and_proofs: Vec<u8>,
    pub presigns: Vec<u8>,
}

impl SignData {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            name: SIGN_DATA_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    /// Create from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn id(&self) -> &ObjectID {
        self.id.object_id()
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

// <<<<<<<<<<<<<<<<<<<<<<<< Objects <<<<<<<<<<<<<<<<<<<<<<<<
