use crate::crypto::AuthorityName;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, DWALLET_DECRYPTION_KEY_RESHARE_REQUEST_EVENT_NAME,
    DWALLET_DKG_FIRST_ROUND_REQUEST_EVENT_STRUCT_NAME,
    DWALLET_IMPORTED_KEY_VERIFICATION_REQUEST_EVENT,
    DWALLET_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_REQUEST_EVENT,
    DWALLET_MPC_EVENT_STRUCT_NAME, ENCRYPTED_SHARE_VERIFICATION_REQUEST_EVENT_NAME,
    FUTURE_SIGN_REQUEST_EVENT_NAME, PRESIGN_REQUEST_EVENT_STRUCT_NAME,
    SIGN_REQUEST_EVENT_STRUCT_NAME, START_NETWORK_DKG_EVENT_STRUCT_NAME,
};
use dwallet_mpc_types::dwallet_mpc::{
    DWALLET_DKG_SECOND_ROUND_REQUEST_EVENT_STRUCT_NAME, DWALLET_MODULE_NAME,
};
use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use move_core_types::language_storage::StructTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use sui_types::balance::Balance;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::collection_types::{Table, TableVec};

pub const DKG_FIRST_ROUND_PROTOCOL_FLAG: u32 = 0;
pub const DKG_SECOND_ROUND_PROTOCOL_FLAG: u32 = 1;
pub const RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG: u32 = 2;
pub const MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG: u32 = 3;
pub const IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG: u32 = 4;
pub const PRESIGN_PROTOCOL_FLAG: u32 = 5;
pub const SIGN_PROTOCOL_FLAG: u32 = 6;
pub const FUTURE_SIGN_PROTOCOL_FLAG: u32 = 7;
pub const SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG: u32 = 8;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MPCProtocolInitData {
    /// Make the dWallet user secret key shares public, so the network can control it.
    MakeDWalletUserSecretKeySharesPublicRequest(
        DWalletMPCSuiEvent<MakeDWalletUserSecretKeySharesPublicRequestEvent>,
    ),

    /// Import a secret key to a dWallet.
    DWalletImportedKeyVerificationRequest(
        DWalletMPCSuiEvent<DWalletImportedKeyVerificationRequestEvent>,
    ),
    /// The first round of the DKG protocol.
    DKGFirst(DWalletMPCSuiEvent<DWalletDKGFirstRoundRequestEvent>),
    /// The second round of the DKG protocol.
    /// Contains the data of the event that triggered the round,
    /// and the network key version of the first round.
    DKGSecond(DWalletMPCSuiEvent<DWalletDKGSecondRoundRequestEvent>),
    /// The first round of the Presign protocol for each message in the Batch.
    /// Contains the `ObjectId` of the dWallet object,
    /// the DKG decentralized output, the batch session ID (same for each message in the batch),
    /// and the dWallets' network key version.
    Presign(DWalletMPCSuiEvent<PresignRequestEvent>),
    /// The first and only round of the Sign protocol.
    /// Contains all the data needed to sign the message.
    Sign(DWalletMPCSuiEvent<SignRequestEvent>),
    /// The only round of the network DKG protocol.
    /// Contains the network key scheme, the dWallet network decryption key object ID
    /// and at the end of the session holds the new key version.
    NetworkDkg(
        DWalletMPCNetworkKeyScheme,
        DWalletMPCSuiEvent<StartNetworkDKGEvent>,
    ),
    /// The round of verifying the encrypted share proof is valid and
    /// that the signature on it is valid.
    /// This is not a real MPC round,
    /// but we use it to start the verification process using the same events mechanism
    /// because the system does not support native functions.
    EncryptedShareVerification(DWalletMPCSuiEvent<EncryptedShareVerificationRequestEvent>),
    PartialSignatureVerification(DWalletMPCSuiEvent<FutureSignRequestEvent>),
    DecryptionKeyReshare(DWalletMPCSuiEvent<DWalletEncryptionKeyReconfigurationRequestEvent>),
}

impl Display for MPCProtocolInitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MPCProtocolInitData::DKGFirst(_) => write!(f, "dWalletDKGFirstRound"),
            MPCProtocolInitData::DKGSecond(_) => write!(f, "dWalletDKGSecondRound"),
            MPCProtocolInitData::Presign(_) => write!(f, "Presign"),
            MPCProtocolInitData::Sign(_) => write!(f, "Sign"),
            MPCProtocolInitData::NetworkDkg(_, _) => write!(f, "NetworkDkg"),
            MPCProtocolInitData::EncryptedShareVerification(_) => {
                write!(f, "EncryptedShareVerification")
            }
            MPCProtocolInitData::PartialSignatureVerification(_) => {
                write!(f, "PartialSignatureVerification")
            }
            MPCProtocolInitData::DecryptionKeyReshare(_) => {
                write!(f, "DecryptionKeyReshare")
            }
            MPCProtocolInitData::MakeDWalletUserSecretKeySharesPublicRequest(_) => {
                write!(f, "MakeDWalletUserSecretKeySharesPublicRequest")
            }
            MPCProtocolInitData::DWalletImportedKeyVerificationRequest(_) => {
                write!(f, "DWalletImportedKeyVerificationRequestEvent")
            }
        }
    }
}

impl MPCProtocolInitData {
    pub fn get_curve(&self) -> String {
        let curve = match self {
            MPCProtocolInitData::DKGFirst(event) => Some(event.event_data.curve),
            MPCProtocolInitData::DKGSecond(event) => Some(event.event_data.curve),
            MPCProtocolInitData::Presign(event) => Some(event.event_data.curve),
            MPCProtocolInitData::Sign(event) => Some(event.event_data.curve),
            MPCProtocolInitData::NetworkDkg(_, _event) => None,
            MPCProtocolInitData::EncryptedShareVerification(event) => Some(event.event_data.curve),
            MPCProtocolInitData::PartialSignatureVerification(event) => {
                Some(event.event_data.curve)
            }
            MPCProtocolInitData::DecryptionKeyReshare(_event) => None,
            MPCProtocolInitData::MakeDWalletUserSecretKeySharesPublicRequest(event) => {
                Some(event.event_data.curve)
            }

            MPCProtocolInitData::DWalletImportedKeyVerificationRequest(event) => {
                Some(event.event_data.curve)
            }
        };
        let curve_str = match &curve {
            None => "".to_string(),
            Some(curve) => {
                if curve == &0 {
                    "Secp256k1".to_string()
                } else {
                    "Unknown".to_string()
                }
            }
        };
        curve_str
    }

    pub fn get_hash_scheme(&self) -> String {
        let hash_scheme = match self {
            MPCProtocolInitData::DKGFirst(_) => None,
            MPCProtocolInitData::DKGSecond(_) => None,
            MPCProtocolInitData::Presign(_) => None,
            MPCProtocolInitData::Sign(event) => Some(event.event_data.hash_scheme),
            MPCProtocolInitData::NetworkDkg(_, _event) => None,
            MPCProtocolInitData::EncryptedShareVerification(_) => None,
            MPCProtocolInitData::PartialSignatureVerification(event) => {
                Some(event.event_data.hash_scheme)
            }
            MPCProtocolInitData::DecryptionKeyReshare(_event) => None,
            MPCProtocolInitData::MakeDWalletUserSecretKeySharesPublicRequest(_) => None,
            MPCProtocolInitData::DWalletImportedKeyVerificationRequest(_) => None,
        };
        match &hash_scheme {
            None => "".to_string(),
            Some(hash_scheme) => {
                if hash_scheme == &0 {
                    "KECCAK256".to_string()
                } else if hash_scheme == &1 {
                    "SHA256".to_string()
                } else {
                    "Unknown".to_string()
                }
            }
        }
    }

    pub fn get_signature_algorithm(&self) -> String {
        let signature_alg = match self {
            MPCProtocolInitData::DKGFirst(event) => None,
            MPCProtocolInitData::DKGSecond(event) => None,
            MPCProtocolInitData::Presign(event) => Some(event.event_data.signature_algorithm),
            MPCProtocolInitData::Sign(event) => Some(event.event_data.signature_algorithm),
            MPCProtocolInitData::NetworkDkg(_, event) => None,
            MPCProtocolInitData::EncryptedShareVerification(_) => None,
            MPCProtocolInitData::PartialSignatureVerification(event) => {
                Some(event.event_data.signature_algorithm)
            }
            MPCProtocolInitData::DecryptionKeyReshare(_event) => None,
            MPCProtocolInitData::MakeDWalletUserSecretKeySharesPublicRequest(_) => None,
            MPCProtocolInitData::DWalletImportedKeyVerificationRequest(event) => None,
        };
        let signature_alg = match &signature_alg {
            None => "".to_string(),
            Some(curve) => {
                return if curve == &0 {
                    "ECDSA".to_string()
                } else {
                    "Unknown".to_string()
                }
            }
        };
        signature_alg
    }

    pub fn get_event_name(&self) -> String {
        match self {
            MPCProtocolInitData::MakeDWalletUserSecretKeySharesPublicRequest(_) => {
                DWALLET_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_REQUEST_EVENT.to_string()
            }
            MPCProtocolInitData::DWalletImportedKeyVerificationRequest(_) => {
                DWALLET_IMPORTED_KEY_VERIFICATION_REQUEST_EVENT.to_string()
            }
            MPCProtocolInitData::DKGFirst(_) => {
                DWALLET_DKG_FIRST_ROUND_REQUEST_EVENT_STRUCT_NAME.to_string()
            }
            MPCProtocolInitData::DKGSecond(_) => {
                DWALLET_DKG_SECOND_ROUND_REQUEST_EVENT_STRUCT_NAME.to_string()
            }
            MPCProtocolInitData::Presign(_) => PRESIGN_REQUEST_EVENT_STRUCT_NAME.to_string(),
            MPCProtocolInitData::Sign(_) => SIGN_REQUEST_EVENT_STRUCT_NAME.to_string(),
            MPCProtocolInitData::NetworkDkg(_, _) => {
                START_NETWORK_DKG_EVENT_STRUCT_NAME.to_string()
            }
            MPCProtocolInitData::EncryptedShareVerification(_) => {
                ENCRYPTED_SHARE_VERIFICATION_REQUEST_EVENT_NAME.to_string()
            }
            MPCProtocolInitData::PartialSignatureVerification(_) => {
                FUTURE_SIGN_REQUEST_EVENT_NAME.to_string()
            }
            MPCProtocolInitData::DecryptionKeyReshare(_) => {
                DWALLET_DECRYPTION_KEY_RESHARE_REQUEST_EVENT_NAME.to_string()
            }
        }
    }
}

impl Debug for MPCProtocolInitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MPCProtocolInitData::DKGFirst(_) => write!(f, "dWalletDKGFirstRound"),
            MPCProtocolInitData::DKGSecond(_) => write!(f, "dWalletDKGSecondRound"),
            MPCProtocolInitData::Presign(_) => write!(f, "Presign"),
            MPCProtocolInitData::Sign(_) => write!(f, "Sign"),
            MPCProtocolInitData::NetworkDkg(_, _) => write!(f, "NetworkDkg"),
            MPCProtocolInitData::EncryptedShareVerification(_) => {
                write!(f, "EncryptedShareVerification")
            }
            MPCProtocolInitData::PartialSignatureVerification(_) => {
                write!(f, "PartialSignatureVerification")
            }
            MPCProtocolInitData::DecryptionKeyReshare(_) => {
                write!(f, "DecryptionKeyReshare")
            }
            MPCProtocolInitData::MakeDWalletUserSecretKeySharesPublicRequest(_) => {
                write!(f, "MakeDWalletUserSecretKeySharesPublicRequest")
            }
            MPCProtocolInitData::DWalletImportedKeyVerificationRequest(_) => {
                write!(f, "DWalletImportedKeyVerificationRequestEvent")
            }
        }
    }
}

/// This is a wrapper type for the [`SuiEvent`] type that is being used to write it to the local RocksDB.
/// This is needed because the [`SuiEvent`] cannot be directly written to the RocksDB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBSuiEvent {
    pub type_: StructTag,
    pub contents: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DWalletMPCEvent {
    // TODO: remove event - do all parsing beforehand.
    pub event: DBSuiEvent,
    pub session_info: SessionInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DWalletMPCOutputMessage {
    /// The authority that sent the output.
    pub authority: AuthorityName,
    /// The session information of the MPC session.
    pub session_info: SessionInfo,
    /// The final value of the MPC session.
    pub output: Vec<u8>,
}

/// The content of the system transaction that stores the MPC session output on the chain.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DWalletMPCOutput {
    /// The session information of the MPC session.
    pub session_info: SessionInfo,
    /// The final value of the MPC session.
    pub output: Vec<u8>,
}

/// The message a Validator can send to the other parties while
/// running a dWallet MPC session.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DWalletMPCMessage {
    /// The serialized message.
    pub message: Vec<u8>,
    /// The authority (Validator) that sent the message.
    pub authority: AuthorityName,
    pub session_id: ObjectID,
    /// The MPC round number starts from 0.
    pub round_number: usize,
    pub mpc_protocol: String,
}

/// The message unique key in the consensus network.
/// Used to make sure no message is being processed twice.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DWalletMPCMessageKey {
    /// The authority (Validator) that sent the message.
    pub authority: AuthorityName,
    pub session_id: ObjectID,
    /// The MPC round number starts from 0.
    pub round_number: usize,
}

/// Holds information about the current MPC session.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_type: SessionType,
    /// Unique identifier for the MPC session.
    pub session_id: ObjectID,
    /// The current MPC round in the protocol.
    /// Contains extra parameters if needed.
    pub mpc_round: MPCProtocolInitData,
    pub epoch: u64,
}

pub trait DWalletMPCEventTrait {
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag;
}

/// The DWallet MPC session type
/// User initiated sessions have a sequence number, which is used to determine in which epoch the session will get
/// completed.
/// System sessions are guaranteed to always get completed in the epoch they were created in.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub enum SessionType {
    User { sequence_number: u64 },
    System,
}

/// Represents the Rust version of the Move struct `ika_system::dwallet::DWalletMPCEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct DWalletMPCSuiEvent<E: DWalletMPCEventTrait> {
    pub epoch: u64,
    pub session_type: SessionType,
    pub session_id: ObjectID,
    pub event_data: E,
}

impl<E: DWalletMPCEventTrait> DWalletMPCEventTrait for DWalletMPCSuiEvent<E> {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`DWalletMPCSuiEvent`] events from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: DWALLET_MPC_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![<E as DWalletMPCEventTrait>::type_(packages_config).into()],
        }
    }
}

impl<E: DWalletMPCEventTrait> DWalletMPCSuiEvent<E> {
    pub fn is_dwallet_mpc_event(event: StructTag, package_id: AccountAddress) -> bool {
        event.address == package_id
            && event.name == DWALLET_MPC_EVENT_STRUCT_NAME.to_owned()
            && event.module == DWALLET_MODULE_NAME.to_owned()
    }
}

/// The Rust representation of the `EncryptedShareVerificationRequestEvent` Move struct.
/// Defined here so that we can use it in the [`MPCProtocolInitData`] enum,
/// as the inner data of the [`MPCProtocolInitData::EncryptedShareVerification`].
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct EncryptedShareVerificationRequestEvent {
    /// Encrypted centralized secret key share and the associated
    /// cryptographic proof of encryption.
    pub encrypted_centralized_secret_share_and_proof: Vec<u8>,
    /// The public output of the decentralized party.
    /// Belongs to the dWallet that its centralized secret share is being encrypted.
    pub decentralized_public_output: Vec<u8>,
    /// The ID of the dWallet that this encrypted secret key share belongs to.
    pub dwallet_id: ObjectID,
    /// The encryption key used to encrypt the secret key share with.
    pub encryption_key: Vec<u8>,
    /// The `EncryptionKey` Move object ID.
    pub encryption_key_id: ObjectID,
    pub encrypted_user_secret_key_share_id: ObjectID,
    pub source_encrypted_user_secret_key_share_id: ObjectID,
    pub dwallet_network_decryption_key_id: ObjectID,
    pub curve: u32,
}

impl DWalletMPCEventTrait for EncryptedShareVerificationRequestEvent {
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: ident_str!("EncryptedShareVerificationRequestEvent").to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust representation of the Move `FutureSignRequestEvent` Event.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct FutureSignRequestEvent {
    pub dwallet_id: ObjectID,
    pub partial_centralized_signed_message_id: ObjectID,
    pub message: Vec<u8>,
    pub presign: Vec<u8>,
    pub dkg_output: Vec<u8>,
    pub curve: u32,
    pub signature_algorithm: u32,
    pub hash_scheme: u32,
    pub message_centralized_signature: Vec<u8>,
    pub dwallet_network_decryption_key_id: ObjectID,
}

impl DWalletMPCEventTrait for FutureSignRequestEvent {
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: ident_str!("FutureSignRequestEvent").to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Represents the Rust version of the Move struct `ika_system::dwallet_2pc_mpc_coordinator_inner::DWalletDKGSecondRoundRequestEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct DWalletDKGSecondRoundRequestEvent {
    pub encrypted_user_secret_key_share_id: ObjectID,
    pub dwallet_id: ObjectID,
    /// The output from the first round of the DKG process.
    pub first_round_output: Vec<u8>,
    /// A serialized vector containing the centralized public key share and its proof.
    pub centralized_public_key_share_and_proof: Vec<u8>,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ObjectID,
    /// Encrypted centralized secret key share and the associated cryptographic proof of encryption.
    pub encrypted_centralized_secret_share_and_proof: Vec<u8>,
    /// The `EncryptionKey` object used for encrypting the secret key share.
    pub encryption_key: Vec<u8>,
    /// The unique identifier of the `EncryptionKey` object.
    pub encryption_key_id: ObjectID,
    pub encryption_key_address: SuiAddress,
    pub user_public_output: Vec<u8>,
    /// The Ed25519 public key of the initiator,
    /// used to verify the signature on the centralized public output.
    pub signer_public_key: Vec<u8>,
    pub dwallet_network_decryption_key_id: ObjectID,
    pub curve: u32,
}

impl DWalletMPCEventTrait for DWalletDKGSecondRoundRequestEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`DWalletDKGSecondRoundRequestEvent`] events from the chain
    /// and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: DWALLET_DKG_SECOND_ROUND_REQUEST_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// The possible result of advancing the MPC protocol.
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AdvanceResult {
    Success,
    Failure,
}

/// Represents a report of malicious behavior in the dWallet MPC process.
///
/// This struct is used to record instances where validators identify malicious actors
/// attempting to disrupt the protocol.
/// It links the malicious actors to a specific MPC session.
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaliciousReport {
    /// A list of authority names that have been identified as malicious actors.
    pub malicious_actors: Vec<AuthorityName>,
    /// The unique identifier of the MPC session in which the malicious activity occurred.
    pub session_id: ObjectID,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ThresholdNotReachedReport {
    pub session_id: ObjectID,
    pub attempt: usize,
}

impl MaliciousReport {
    /// Creates a new instance of a malicious report.
    pub fn new(malicious_actors: Vec<AuthorityName>, session_id: ObjectID) -> Self {
        Self {
            malicious_actors,
            session_id,
        }
    }
}

/// Represents the Rust version of the Move struct `ika_system::dwallet_2pc_mpc_coordinator_inner::PresignRequestEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct PresignRequestEvent {
    /// The `DWallet` object's ID associated with the DKG output.
    pub dwallet_id: Option<ObjectID>,
    pub presign_id: ObjectID,
    /// The DKG decentralized final output to use for the presign session.
    pub dwallet_public_output: Option<Vec<u8>>,
    pub dwallet_network_decryption_key_id: ObjectID,
    pub curve: u32,
    pub signature_algorithm: u32,
}

impl DWalletMPCEventTrait for PresignRequestEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`PresignRequestEvent`] events
    /// from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: PRESIGN_REQUEST_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IkaPackagesConfig {
    /// The move package ID of ika (IKA) on sui.
    pub ika_package_id: ObjectID,
    /// The move package ID of `ika_system` on sui.
    pub ika_system_package_id: ObjectID,
    /// The object ID of ika_system_state on sui.
    pub ika_system_object_id: ObjectID,
}

impl sui_config::Config for IkaPackagesConfig {}

/// Represents the Rust version of the Move struct `ika_system::dwallet_2pc_mpc_coordinator_inner::DWalletDKGFirstRoundRequestEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct DWalletDKGFirstRoundRequestEvent {
    pub dwallet_id: ObjectID,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ObjectID,
    pub dwallet_network_decryption_key_id: ObjectID,
    pub curve: u32,
}

impl DWalletMPCEventTrait for DWalletDKGFirstRoundRequestEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`DWalletDKGFirstRoundRequestEvent`] events from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: DWALLET_DKG_FIRST_ROUND_REQUEST_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DWalletImportedKeyVerificationRequestEvent {
    /// The unique session identifier for the DWallet.
    pub dwallet_id: ObjectID,

    /// The Encrypted user secret key share object ID.
    pub encrypted_user_secret_key_share_id: ObjectID,

    /// The message delivered to the decentralized party from a centralized party.
    /// Includes the encrypted decentralized secret key share and
    /// the associated cryptographic proof of encryption.
    pub centralized_party_message: Vec<u8>,

    /// The unique identifier of the dWallet capability associated with this session.
    pub dwallet_cap_id: ObjectID,

    /// Encrypted centralized secret key share and the associated cryptographic proof of encryption.
    pub encrypted_centralized_secret_share_and_proof: Vec<u8>,

    /// The user `EncryptionKey` object used for encrypting the user secret key share.
    pub encryption_key: Vec<u8>,

    /// The unique identifier of the `EncryptionKey` object.
    pub encryption_key_id: ObjectID,

    pub encryption_key_address: SuiAddress,

    /// The public output of the centralized party in the DKG process.
    pub user_public_output: Vec<u8>,

    /// The Ed25519 public key of the initiator,
    /// used to verify the signature on the centralized public output.
    pub signer_public_key: Vec<u8>,

    /// The MPC network decryption key id that is used to decrypt associated dWallet.
    pub dwallet_network_encryption_key_id: ObjectID,

    /// The elliptic curve used for the dWallet.
    pub curve: u32,
}

/// Represents the Rust version of the Move struct `ika_system::dwallet_2pc_mpc_coordinator_inner::DWalletDKGFirstRoundRequestEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct MakeDWalletUserSecretKeySharesPublicRequestEvent {
    pub public_user_secret_key_shares: Vec<u8>,
    pub public_output: Vec<u8>,
    pub curve: u32,
    pub dwallet_id: ObjectID,
    pub dwallet_network_decryption_key_id: ObjectID,
}

impl DWalletMPCEventTrait for MakeDWalletUserSecretKeySharesPublicRequestEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`DWalletDKGFirstRoundRequestEvent`] events from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: DWALLET_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_REQUEST_EVENT.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

impl DWalletMPCEventTrait for DWalletImportedKeyVerificationRequestEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`DWalletDKGFirstRoundRequestEvent`] events from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: DWALLET_IMPORTED_KEY_VERIFICATION_REQUEST_EVENT.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Represents the Rust version of the Move
/// struct `ika_system::dwallet_2pc_mpc_coordinator_inner::SignRequestEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct SignRequestEvent {
    pub sign_id: ObjectID,
    /// The `DWallet` object's ObjectID associated with the DKG output.
    pub dwallet_id: ObjectID,
    /// The public output of the decentralized party in the dWallet DKG process.
    pub dwallet_decentralized_public_output: Vec<u8>,
    pub curve: u32,
    pub signature_algorithm: u32,
    pub hash_scheme: u32,
    /// Hashed messages to Sign.
    pub message: Vec<u8>,
    /// The dWallet mpc network key version
    pub dwallet_network_decryption_key_id: ObjectID,
    pub presign_id: ObjectID,

    /// The presign protocol output as bytes.
    pub presign: Vec<u8>,

    /// The centralized party signature of a message.
    pub message_centralized_signature: Vec<u8>,

    /// Indicates whether the future sign feature was used to start the session.
    pub is_future_sign: bool,
}

impl DWalletMPCEventTrait for SignRequestEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`SignRequestEvent`]
    /// events from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: SIGN_REQUEST_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust version of the Move [`ika_system::dwallet_2pc_mpc_coordinator_inner::StartNetworkDKGEvent`] type.
/// It is used to trigger the start of the network DKG process.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartNetworkDKGEvent {
    pub dwallet_network_decryption_key_id: ObjectID,
}

impl DWalletMPCEventTrait for StartNetworkDKGEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartNetworkDKGEvent`] events from the chain and initiate the MPC session.
    /// It is used to trigger the start of the network DKG process.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: START_NETWORK_DKG_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Represents the Rust version of the Move struct `ika_system::dwallet_2pc_mpc_coordinator_inner::DWalletNetworkDecryptionKey`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DWalletNetworkDecryptionKey {
    pub id: ObjectID,
    pub dwallet_network_decryption_key_cap_id: ObjectID,
    pub current_epoch: u64,
    /// key -> epoch, value -> reconfiguration public output (TableVec).
    pub reconfiguration_public_outputs: Table,
    pub network_dkg_public_output: TableVec,
    /// The fees paid for computation in IKA.
    pub computation_fee_charged_ika: Balance,
    pub state: DWalletNetworkEncryptionKeyState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DWalletNetworkDecryptionKeyData {
    pub id: ObjectID,
    pub dwallet_network_decryption_key_cap_id: ObjectID,
    pub current_epoch: u64,
    pub current_reconfiguration_public_output: Vec<u8>,
    pub network_dkg_public_output: Vec<u8>,
    pub state: DWalletNetworkEncryptionKeyState,
}

/// Represents the Rust version of the Move enum `ika_system::dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKeyState`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DWalletNetworkEncryptionKeyState {
    AwaitingNetworkDKG,
    NetworkDKGCompleted,
    /// Reconfiguration request was sent to the network, but didn't finish yet.
    /// `is_first` is true if this is the first reconfiguration request, false otherwise.
    AwaitingNetworkReconfiguration {
        is_first: bool,
    },
    /// Reconfiguration request finished, but we didn't switch an epoch yet.
    /// We need to wait for the next epoch to update the reconfiguration public outputs.
    AwaitingNextEpochToUpdateReconfiguration,
    NetworkReconfigurationCompleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct DWalletEncryptionKeyReconfigurationRequestEvent {
    pub dwallet_network_decryption_key_id: ObjectID,
}

impl DWalletMPCEventTrait for DWalletEncryptionKeyReconfigurationRequestEvent {
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: ident_str!("DWalletEncryptionKeyReconfigurationRequestEvent").to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}
