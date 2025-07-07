use crate::crypto::{keccak256_digest, AuthorityName};
use dwallet_mpc_types::dwallet_mpc::DWalletMPCNetworkKeyScheme;
use hex::FromHex;
use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::StructTag;
use rand::Rng;
use rand_chacha::rand_core::OsRng;
use schemars::JsonSchema;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;
use sui_types::balance::Balance;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::collection_types::{Table, TableVec};

// TODO (#650): Rename Move structs
pub const DWALLET_SESSION_EVENT_STRUCT_NAME: &IdentStr = ident_str!("DWalletSessionEvent");
pub const DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME: &IdentStr =
    ident_str!("dwallet_2pc_mpc_coordinator");
pub const VALIDATOR_SET_MODULE_NAME: &IdentStr = ident_str!("validator_set");
/// There's a wrapper and inner struct to support Move upgradable contracts. Read this doc for further explanations:
/// https://docs.sui.io/concepts/sui-move-concepts/packages/upgrade.
pub const DWALLET_MODULE_NAME: &IdentStr = ident_str!("dwallet_2pc_mpc_coordinator_inner");
pub const DWALLET_DKG_FIRST_ROUND_REQUEST_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("DWalletDKGFirstRoundRequestEvent");
pub const DWALLET_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_REQUEST_EVENT: &IdentStr =
    ident_str!("MakeDWalletUserSecretKeySharePublicRequestEvent");
pub const DWALLET_IMPORTED_KEY_VERIFICATION_REQUEST_EVENT: &IdentStr =
    ident_str!("DWalletImportedKeyVerificationRequestEvent");
// TODO (#650): Rename Move structs
pub const DWALLET_DKG_SECOND_ROUND_REQUEST_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("DWalletDKGSecondRoundRequestEvent");
// TODO (#650): Rename Move structs
pub const PRESIGN_REQUEST_EVENT_STRUCT_NAME: &IdentStr = ident_str!("PresignRequestEvent");
pub const SIGN_REQUEST_EVENT_STRUCT_NAME: &IdentStr = ident_str!("SignRequestEvent");
pub const LOCKED_NEXT_COMMITTEE_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("LockedNextEpochCommitteeEvent");
pub const VALIDATOR_DATA_FOR_SECRET_SHARE_STRUCT_NAME: &IdentStr =
    ident_str!("ValidatorDataForDWalletSecretShare");
pub const START_NETWORK_DKG_EVENT_STRUCT_NAME: &IdentStr =
    ident_str!("DWalletNetworkDKGEncryptionKeyRequestEvent");

pub const DKG_FIRST_ROUND_PROTOCOL_FLAG: u32 = 0;
pub const DKG_SECOND_ROUND_PROTOCOL_FLAG: u32 = 1;
pub const RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG: u32 = 2;
pub const MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG: u32 = 3;
pub const IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG: u32 = 4;
pub const PRESIGN_PROTOCOL_FLAG: u32 = 5;
pub const SIGN_PROTOCOL_FLAG: u32 = 6;
pub const FUTURE_SIGN_PROTOCOL_FLAG: u32 = 7;
pub const SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG: u32 = 8;

pub const NETWORK_ENCRYPTION_KEY_RECONFIGURATION_STR_KEY: &str =
    "NetworkEncryptionKeyReconfiguration";
pub const NETWORK_ENCRYPTION_KEY_DKG_STR_KEY: &str = "NetworkEncryptionKeyDkg";
pub const SIGN_STR_KEY: &str = "Sign";

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MPCRequestInput {
    /// Make the dWallet user secret key shares public, so the network can control it.
    MakeDWalletUserSecretKeySharesPublicRequest(
        DWalletSessionEvent<MakeDWalletUserSecretKeySharesPublicRequestEvent>,
    ),

    /// Import a secret key to a dWallet.
    DWalletImportedKeyVerificationRequest(
        DWalletSessionEvent<DWalletImportedKeyVerificationRequestEvent>,
    ),
    /// The first round of the DKG protocol.
    DKGFirst(DWalletSessionEvent<DWalletDKGFirstRoundRequestEvent>),
    /// The second round of the DKG protocol.
    /// Contains the data of the event that triggered the round,
    /// and the network key version of the first round.
    DKGSecond(DWalletSessionEvent<DWalletDKGSecondRoundRequestEvent>),
    /// The first round of the Presign protocol for each message in the Batch.
    /// Contains the `ObjectId` of the dWallet object,
    /// the DKG decentralized output, the batch session ID (same for each message in the batch),
    /// and the dWallets network key version.
    Presign(DWalletSessionEvent<PresignRequestEvent>),
    /// The first and only round of the Sign protocol.
    /// Contains all the data needed to sign the message.
    Sign(DWalletSessionEvent<SignRequestEvent>),
    /// The only round of the network DKG protocol.
    /// Contains the network key scheme, the dWallet network decryption key object ID
    /// and at the end of the session holds the new key version.
    NetworkEncryptionKeyDkg(
        DWalletMPCNetworkKeyScheme,
        DWalletSessionEvent<DWalletNetworkDKGEncryptionKeyRequestEvent>,
    ),
    /// The round of verifying the encrypted share proof is valid and
    /// that the signature on it is valid.
    /// This is not a real MPC round,
    /// but we use it to start the verification process using the same events mechanism
    /// because the system does not support native functions.
    EncryptedShareVerification(DWalletSessionEvent<EncryptedShareVerificationRequestEvent>),
    PartialSignatureVerification(DWalletSessionEvent<FutureSignRequestEvent>),
    NetworkEncryptionKeyReconfiguration(
        DWalletSessionEvent<DWalletEncryptionKeyReconfigurationRequestEvent>,
    ),
}

impl Display for MPCRequestInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MPCRequestInput::DKGFirst(_) => write!(f, "dWalletDKGFirstRound"),
            MPCRequestInput::DKGSecond(_) => write!(f, "dWalletDKGSecondRound"),
            MPCRequestInput::Presign(_) => write!(f, "Presign"),
            MPCRequestInput::Sign(_) => write!(f, "{}", SIGN_STR_KEY),
            MPCRequestInput::NetworkEncryptionKeyDkg(_, _) => {
                write!(f, "{}", NETWORK_ENCRYPTION_KEY_DKG_STR_KEY)
            }
            MPCRequestInput::EncryptedShareVerification(_) => {
                write!(f, "EncryptedShareVerification")
            }
            MPCRequestInput::PartialSignatureVerification(_) => {
                write!(f, "PartialSignatureVerification")
            }
            MPCRequestInput::NetworkEncryptionKeyReconfiguration(_) => {
                write!(f, "{}", NETWORK_ENCRYPTION_KEY_RECONFIGURATION_STR_KEY)
            }
            MPCRequestInput::MakeDWalletUserSecretKeySharesPublicRequest(_) => {
                write!(f, "MakeDWalletUserSecretKeySharesPublicRequest")
            }
            MPCRequestInput::DWalletImportedKeyVerificationRequest(_) => {
                write!(f, "DWalletImportedKeyVerificationRequestEvent")
            }
        }
    }
}

impl MPCRequestInput {
    pub fn get_curve(&self) -> String {
        let curve = match self {
            MPCRequestInput::DKGFirst(event) => Some(event.event_data.curve),
            MPCRequestInput::DKGSecond(event) => Some(event.event_data.curve),
            MPCRequestInput::Presign(event) => Some(event.event_data.curve),
            MPCRequestInput::Sign(event) => Some(event.event_data.curve),
            MPCRequestInput::NetworkEncryptionKeyDkg(_, _event) => None,
            MPCRequestInput::EncryptedShareVerification(event) => Some(event.event_data.curve),
            MPCRequestInput::PartialSignatureVerification(event) => Some(event.event_data.curve),
            MPCRequestInput::NetworkEncryptionKeyReconfiguration(_event) => None,
            MPCRequestInput::MakeDWalletUserSecretKeySharesPublicRequest(event) => {
                Some(event.event_data.curve)
            }

            MPCRequestInput::DWalletImportedKeyVerificationRequest(event) => {
                Some(event.event_data.curve)
            }
        };
        match &curve {
            None => "".to_string(),
            Some(curve) => {
                if curve == &0 {
                    "Secp256k1".to_string()
                } else {
                    "Unknown".to_string()
                }
            }
        }
    }

    pub fn get_hash_scheme(&self) -> String {
        let hash_scheme = match self {
            MPCRequestInput::DKGFirst(_) => None,
            MPCRequestInput::DKGSecond(_) => None,
            MPCRequestInput::Presign(_) => None,
            MPCRequestInput::Sign(event) => Some(event.event_data.hash_scheme),
            MPCRequestInput::NetworkEncryptionKeyDkg(_, _event) => None,
            MPCRequestInput::EncryptedShareVerification(_) => None,
            MPCRequestInput::PartialSignatureVerification(event) => {
                Some(event.event_data.hash_scheme)
            }
            MPCRequestInput::NetworkEncryptionKeyReconfiguration(_event) => None,
            MPCRequestInput::MakeDWalletUserSecretKeySharesPublicRequest(_) => None,
            MPCRequestInput::DWalletImportedKeyVerificationRequest(_) => None,
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
            MPCRequestInput::DKGFirst(_event) => None,
            MPCRequestInput::DKGSecond(_event) => None,
            MPCRequestInput::Presign(event) => Some(event.event_data.signature_algorithm),
            MPCRequestInput::Sign(event) => Some(event.event_data.signature_algorithm),
            MPCRequestInput::NetworkEncryptionKeyDkg(_, _event) => None,
            MPCRequestInput::EncryptedShareVerification(_) => None,
            MPCRequestInput::PartialSignatureVerification(event) => {
                Some(event.event_data.signature_algorithm)
            }
            MPCRequestInput::NetworkEncryptionKeyReconfiguration(_event) => None,
            MPCRequestInput::MakeDWalletUserSecretKeySharesPublicRequest(_) => None,
            MPCRequestInput::DWalletImportedKeyVerificationRequest(_event) => None,
        };
        match &signature_alg {
            None => "".to_string(),
            Some(curve) => {
                if curve == &0 {
                    "ECDSA".to_string()
                } else {
                    "Unknown".to_string()
                }
            }
        }
    }

    pub fn get_network_encryption_key_id(&self) -> Option<ObjectID> {
        match self {
            MPCRequestInput::DKGFirst(event) => {
                Some(event.event_data.dwallet_network_encryption_key_id)
            }
            MPCRequestInput::DKGSecond(event) => {
                Some(event.event_data.dwallet_network_encryption_key_id)
            }
            MPCRequestInput::Presign(event) => {
                Some(event.event_data.dwallet_network_encryption_key_id)
            }
            MPCRequestInput::Sign(event) => {
                Some(event.event_data.dwallet_network_encryption_key_id)
            }
            MPCRequestInput::NetworkEncryptionKeyDkg(_, event) => {
                Some(event.event_data.dwallet_network_encryption_key_id)
            }
            MPCRequestInput::EncryptedShareVerification(event) => {
                Some(event.event_data.dwallet_network_encryption_key_id)
            }
            MPCRequestInput::PartialSignatureVerification(event) => {
                Some(event.event_data.dwallet_network_encryption_key_id)
            }
            MPCRequestInput::NetworkEncryptionKeyReconfiguration(event) => {
                Some(event.event_data.dwallet_network_encryption_key_id)
            }
            MPCRequestInput::MakeDWalletUserSecretKeySharesPublicRequest(event) => {
                Some(event.event_data.dwallet_network_encryption_key_id)
            }
            MPCRequestInput::DWalletImportedKeyVerificationRequest(event) => {
                Some(event.event_data.dwallet_network_encryption_key_id)
            }
        }
    }
}

impl Debug for MPCRequestInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MPCRequestInput::DKGFirst(_) => write!(f, "dWalletDKGFirstRound"),
            MPCRequestInput::DKGSecond(_) => write!(f, "dWalletDKGSecondRound"),
            MPCRequestInput::Presign(_) => write!(f, "Presign"),
            MPCRequestInput::Sign(_) => write!(f, "Sign"),
            MPCRequestInput::NetworkEncryptionKeyDkg(_, _) => write!(f, "NetworkDkg"),
            MPCRequestInput::EncryptedShareVerification(_) => {
                write!(f, "EncryptedShareVerification")
            }
            MPCRequestInput::PartialSignatureVerification(_) => {
                write!(f, "PartialSignatureVerification")
            }
            MPCRequestInput::NetworkEncryptionKeyReconfiguration(_) => {
                write!(f, "DecryptionKeyReconfiguration")
            }
            MPCRequestInput::MakeDWalletUserSecretKeySharesPublicRequest(_) => {
                write!(f, "MakeDWalletUserSecretKeySharesPublicRequest")
            }
            MPCRequestInput::DWalletImportedKeyVerificationRequest(_) => {
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
    // Set when the event was pulled from the state of the object, and unset when it was pushed as an event.
    pub pulled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DWalletMPCEvent {
    pub session_request: MPCSessionRequest,
    // Set when the event was pulled from the state of the object, and unset when it was pushed as an event.
    pub pulled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DWalletMPCOutputMessage {
    /// The authority that sent the output.
    pub authority: AuthorityName,
    /// The session information of the MPC session.
    pub session_request: MPCSessionRequest,
    /// The final value of the MPC session.
    pub output: Vec<u8>,
}

/// The content of the system transaction that stores the MPC session output on the chain.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DWalletMPCOutput {
    /// The session information of the MPC session.
    pub session_request: MPCSessionRequest,
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
    pub session_identifier: SessionIdentifier,
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
    pub session_identifier: SessionIdentifier,
    /// The MPC round number starts from 0.
    pub round_number: usize,
}

/// Holds information about the current MPC session.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MPCSessionRequest {
    pub session_type: SessionType,
    /// Unique identifier for the MPC session.
    pub session_identifier: SessionIdentifier,
    /// The input to the request MPC session.
    pub request_input: MPCRequestInput,
    pub epoch: u64,
    pub requires_next_active_committee: bool,
}

pub trait DWalletSessionEventTrait {
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

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
pub struct SessionIdentifier([u8; SessionIdentifier::LENGTH]);

impl SessionIdentifier {
    pub const fn new(address: [u8; Self::LENGTH]) -> Self {
        Self(address)
    }

    /// The number of bytes in an address.
    pub const LENGTH: usize = 32;

    /// Hex address: 0x0
    pub const ZERO: Self = Self([0u8; Self::LENGTH]);

    /// Hex address: 0x1
    pub const ONE: Self = Self::get_hex_address_one();

    /// Hex address: 0x2
    pub const TWO: Self = Self::get_hex_address_two();

    pub const fn from_suffix(suffix: u16) -> SessionIdentifier {
        let mut addr = [0u8; SessionIdentifier::LENGTH];
        let [hi, lo] = suffix.to_be_bytes();
        addr[SessionIdentifier::LENGTH - 2] = hi;
        addr[SessionIdentifier::LENGTH - 1] = lo;
        SessionIdentifier::new(addr)
    }

    const fn get_hex_address_one() -> Self {
        let mut addr = [0u8; SessionIdentifier::LENGTH];
        addr[SessionIdentifier::LENGTH - 1] = 1u8;
        Self(addr)
    }

    const fn get_hex_address_two() -> Self {
        let mut addr = [0u8; SessionIdentifier::LENGTH];
        addr[SessionIdentifier::LENGTH - 1] = 2u8;
        Self(addr)
    }

    pub fn random() -> Self {
        let mut rng = OsRng;
        let buf: [u8; Self::LENGTH] = rng.r#gen();
        Self(buf)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.0
    }

    pub fn from_hex_literal(literal: &str) -> Result<Self, SessionIdentifierParseError> {
        if !literal.starts_with("0x") {
            return Err(SessionIdentifierParseError);
        }

        let hex_len = literal.len() - 2;

        // If the string is too short, pad it
        if hex_len < Self::LENGTH * 2 {
            let mut hex_str = String::with_capacity(Self::LENGTH * 2);
            for _ in 0..Self::LENGTH * 2 - hex_len {
                hex_str.push('0');
            }
            hex_str.push_str(&literal[2..]);
            SessionIdentifier::from_hex(hex_str)
        } else {
            SessionIdentifier::from_hex(&literal[2..])
        }
    }

    pub fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, SessionIdentifierParseError> {
        <[u8; Self::LENGTH]>::from_hex(hex)
            .map_err(|_| SessionIdentifierParseError)
            .map(Self)
    }

    pub fn to_hex(&self) -> String {
        format!("{:x}", self)
    }

    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, SessionIdentifierParseError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| SessionIdentifierParseError)
            .map(Self)
    }
}

impl AsRef<[u8]> for SessionIdentifier {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::ops::Deref for SessionIdentifier {
    type Target = [u8; Self::LENGTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for SessionIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl fmt::Debug for SessionIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl fmt::LowerHex for SessionIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

impl fmt::UpperHex for SessionIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }

        Ok(())
    }
}

impl From<[u8; SessionIdentifier::LENGTH]> for SessionIdentifier {
    fn from(bytes: [u8; SessionIdentifier::LENGTH]) -> Self {
        Self::new(bytes)
    }
}

impl TryFrom<&[u8]> for SessionIdentifier {
    type Error = SessionIdentifierParseError;

    /// Tries to convert the provided byte array into Address.
    fn try_from(bytes: &[u8]) -> Result<SessionIdentifier, SessionIdentifierParseError> {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<Vec<u8>> for SessionIdentifier {
    type Error = SessionIdentifierParseError;

    /// Tries to convert the provided byte buffer into Address.
    fn try_from(bytes: Vec<u8>) -> Result<SessionIdentifier, SessionIdentifierParseError> {
        Self::from_bytes(bytes)
    }
}

impl From<SessionIdentifier> for Vec<u8> {
    fn from(addr: SessionIdentifier) -> Vec<u8> {
        addr.0.to_vec()
    }
}

impl From<&SessionIdentifier> for Vec<u8> {
    fn from(addr: &SessionIdentifier) -> Vec<u8> {
        addr.0.to_vec()
    }
}

impl From<SessionIdentifier> for [u8; SessionIdentifier::LENGTH] {
    fn from(addr: SessionIdentifier) -> Self {
        addr.0
    }
}

impl From<&SessionIdentifier> for [u8; SessionIdentifier::LENGTH] {
    fn from(addr: &SessionIdentifier) -> Self {
        addr.0
    }
}

impl From<&SessionIdentifier> for String {
    fn from(addr: &SessionIdentifier) -> String {
        ::hex::encode(addr.as_ref())
    }
}

impl TryFrom<String> for SessionIdentifier {
    type Error = SessionIdentifierParseError;

    fn try_from(s: String) -> Result<SessionIdentifier, SessionIdentifierParseError> {
        Self::from_hex(s)
    }
}

impl FromStr for SessionIdentifier {
    type Err = SessionIdentifierParseError;

    fn from_str(s: &str) -> Result<Self, SessionIdentifierParseError> {
        // Accept 0xADDRESS or ADDRESS
        if let Ok(address) = SessionIdentifier::from_hex_literal(s) {
            Ok(address)
        } else {
            Self::from_hex(s)
        }
    }
}

impl<'de> Deserialize<'de> for SessionIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = <String>::deserialize(deserializer)?;
            SessionIdentifier::from_str(&s).map_err(Error::custom)
        } else {
            // In order to preserve the Serde data model and help analysis tools,
            // make sure to wrap our value in a container with the same name
            // as the original type.
            #[derive(::serde::Deserialize)]
            #[serde(rename = "SessionIdentifier")]
            struct Value([u8; SessionIdentifier::LENGTH]);

            let value = Value::deserialize(deserializer)?;
            Ok(SessionIdentifier::new(value.0))
        }
    }
}

impl Serialize for SessionIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            self.to_hex().serialize(serializer)
        } else {
            // See comment in deserialize.
            serializer.serialize_newtype_struct("SessionIdentifier", &self.0)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SessionIdentifierParseError;

impl fmt::Display for SessionIdentifierParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse SessionIdentifier (must be hex string of length {})",
            SessionIdentifier::LENGTH
        )
    }
}

impl std::error::Error for SessionIdentifierParseError {}

pub type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;

/// Represents the Rust version of the Move struct `ika_system::dwallet_2pc_mpc_coordinator_inner::DWalletSessionEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct DWalletSessionEvent<E: DWalletSessionEventTrait> {
    pub epoch: u64,
    pub session_object_id: ObjectID,
    pub session_type: SessionType,
    // DO NOT MAKE THIS PUBLIC! ONLY CALL `session_identifier_digest`
    session_identifier_preimage: Vec<u8>,
    pub event_data: E,
}

impl<E: DWalletSessionEventTrait> DWalletSessionEventTrait for DWalletSessionEvent<E> {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`DWalletSessionEvent`] events from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: DWALLET_SESSION_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![<E as DWalletSessionEventTrait>::type_(packages_config).into()],
        }
    }
}

impl<E: DWalletSessionEventTrait> DWalletSessionEvent<E> {
    pub fn is_dwallet_mpc_event(event: StructTag, package_id: AccountAddress) -> bool {
        event.address == package_id
            && event.name == DWALLET_SESSION_EVENT_STRUCT_NAME.to_owned()
            && event.module == DWALLET_MODULE_NAME.to_owned()
    }

    /// Convert the pre-image session identifier to the session ID by hashing it together with its distinguisher.
    /// Guarantees same values of `self.session_identifier_preimage` yield different output for `User` and `System`
    pub fn session_identifier_digest(&self) -> SessionIdentifier {
        // We are adding a string distinguisher between
        // the `User` and `System` sessions, so that when it is hashed, the same inner value
        // in the two different options will yield a different output, thus guaranteeing
        // user-initiated sessions can never block or reuse session IDs for system sessions.
        let session_type = match self.session_type {
            SessionType::User { .. } => {
                [b"USER", self.session_identifier_preimage.as_slice()].concat()
            }
            SessionType::System => {
                [b"SYSTEM", self.session_identifier_preimage.as_slice()].concat()
            }
        };
        SessionIdentifier(keccak256_digest(&session_type))
    }
}

/// The Rust representation of the `EncryptedShareVerificationRequestEvent` Move struct.
/// Defined here so that we can use it in the [`MPCRequestInput`] enum,
/// as the inner data of the [`MPCRequestInput::EncryptedShareVerification`].
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
    pub dwallet_network_encryption_key_id: ObjectID,
    pub curve: u32,
}

impl DWalletSessionEventTrait for EncryptedShareVerificationRequestEvent {
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
    pub dwallet_network_encryption_key_id: ObjectID,
}

impl DWalletSessionEventTrait for FutureSignRequestEvent {
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
    pub dwallet_network_encryption_key_id: ObjectID,
    pub curve: u32,
}

impl DWalletSessionEventTrait for DWalletDKGSecondRoundRequestEvent {
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
    pub session_identifier: SessionIdentifier,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ThresholdNotReachedReport {
    pub session_identifier: SessionIdentifier,
    pub attempt: usize,
}

impl MaliciousReport {
    /// Creates a new instance of a malicious report.
    pub fn new(
        malicious_actors: Vec<AuthorityName>,
        session_identifier: SessionIdentifier,
    ) -> Self {
        Self {
            malicious_actors,
            session_identifier,
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
    pub dwallet_network_encryption_key_id: ObjectID,
    pub curve: u32,
    pub signature_algorithm: u32,
}

impl DWalletSessionEventTrait for PresignRequestEvent {
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
    pub dwallet_network_encryption_key_id: ObjectID,
    pub curve: u32,
}

impl DWalletSessionEventTrait for DWalletDKGFirstRoundRequestEvent {
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
    pub dwallet_network_encryption_key_id: ObjectID,
}

impl DWalletSessionEventTrait for MakeDWalletUserSecretKeySharesPublicRequestEvent {
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

impl DWalletSessionEventTrait for DWalletImportedKeyVerificationRequestEvent {
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
    pub dwallet_network_encryption_key_id: ObjectID,
    pub presign_id: ObjectID,

    /// The presign protocol output as bytes.
    pub presign: Vec<u8>,

    /// The centralized party signature of a message.
    pub message_centralized_signature: Vec<u8>,

    /// Indicates whether the future sign feature was used to start the session.
    pub is_future_sign: bool,
}

impl DWalletSessionEventTrait for SignRequestEvent {
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
pub struct DWalletNetworkDKGEncryptionKeyRequestEvent {
    pub dwallet_network_encryption_key_id: ObjectID,
    pub params_for_network: Vec<u8>,
}

impl DWalletSessionEventTrait for DWalletNetworkDKGEncryptionKeyRequestEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`DWalletNetworkDKGEncryptionKeyRequestEvent`] events from the chain and initiate the MPC session.
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
    pub dkg_params_for_network: Vec<u8>,
    pub supported_curves: Vec<u32>,
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
    // Network DKG is completed, but we didn't do Reconfiguration yet.
    NetworkDKGCompleted,
    /// Reconfiguration request was sent to the network, but didn't finish yet.
    /// `is_first` is true if this is the first reconfiguration request, false otherwise.
    AwaitingNetworkReconfiguration {
        is_first: bool,
    },
    /// Reconfiguration request finished, but we didn't switch an epoch yet.
    /// We need to wait for the next epoch to update the reconfiguration of public outputs.
    AwaitingNextEpochToUpdateReconfiguration {
        is_first: bool,
    },
    NetworkReconfigurationCompleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct DWalletEncryptionKeyReconfigurationRequestEvent {
    pub dwallet_network_encryption_key_id: ObjectID,
}

impl DWalletSessionEventTrait for DWalletEncryptionKeyReconfigurationRequestEvent {
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: ident_str!("DWalletEncryptionKeyReconfigurationRequestEvent").to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}
