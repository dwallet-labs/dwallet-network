use crate::base_types::{AuthorityName, ObjectID, PeraAddress};
use crate::crypto::default_hash;
use crate::digests::DWalletMPCOutputDigest;
use crate::event::Event;
use crate::id::ID;
use crate::message_envelope::Message;
use crate::PERA_SYSTEM_ADDRESS;
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, NetworkDecryptionKeyShares};
use dwallet_mpc_types::dwallet_mpc::{
    MPCPublicOutput, DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME, DWALLET_MODULE_NAME,
    START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME,
};
use move_core_types::ident_str;
use move_core_types::language_storage::StructTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;

// todo(zeev): move the events to mpc_events and the types to `dwallet-mpc-types` crate.

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MPCProtocolInitData {
    /// The first round of the DKG protocol.
    DKGFirst,
    /// The second round of the DKG protocol.
    DKGSecond(StartDKGSecondRoundEvent, u8),
    /// This is not a real round, but an indicator the Batches Manager to
    /// register a Presign Batch session.
    BatchedPresign(u64),
    /// The first round of the Presign protocol for each message in the Batch.
    /// Contains the `ObjectId` of the dWallet object,
    /// the DKG decentralized output, the batch session ID (same for each message in the batch),
    /// and the dWallets' network key version.
    // TODO (#543): Connect the two presign rounds to one.
    PresignFirst(ObjectID, MPCPublicOutput, ObjectID, u8),
    /// The second round of the Presign protocol.
    /// Contains the `ObjectId` of the dWallet object,
    /// the Presign first round output, and the batch session ID.
    PresignSecond(ObjectID, MPCPublicOutput, ObjectID),
    /// The first and only round of the Sign protocol.
    Sign(SingleSignSessionData),
    /// A batched sign session, contains the list of messages that are being signed.
    // TODO (#536): Store batch state and logic on Sui & remove this field.
    BatchedSign(Vec<Vec<u8>>),
    /// The only round of the network DKG protocol.
    NetworkDkg(
        DWalletMPCNetworkKeyScheme,
        Option<NetworkDecryptionKeyShares>,
    ),
    /// The round of verifying the encrypted share proof is valid and
    /// that the signature on it is valid.
    /// This is not a real MPC round,
    /// but we use it to start the verification process using the same events mechanism
    /// because the system does not support native functions.
    EncryptedShareVerification(StartEncryptedShareVerificationEvent),
    /// The round of verifying the public key that signed on the encryption key is
    /// matching the initiator address.
    /// todo(zeev): more docs, make it clearer.
    /// TODO (#544): Check if there's a way to convert the public key to an address in Move.
    /// This is not a real MPC round,
    /// but we use it to start the verification process using the same events mechanism
    /// because the system does not support native functions.
    EncryptionKeyVerification(StartEncryptionKeyVerificationEvent),
}

/// The session-specific state of the MPC session.
/// I.e., state needs to exist only in the sign protocol but is not required in the
/// presign protocol.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MPCSessionSpecificState {
    Sign(SignIASessionState),
}

/// The state of a sign-identifiable abort session.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SignIASessionState {
    pub malicious_report: MaliciousReport,
    pub initiating_ia_authority: AuthorityName,
}

/// The message and data for the Sign round.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SingleSignSessionData {
    pub batch_session_id: ObjectID,
    pub hashed_message: Vec<u8>,
    /// The dWallet ID that is used to sign, needed mostly for audit.
    pub dwallet_id: ObjectID,
    /// The DKG output of the dWallet, used to sign and verify the message.
    pub dwallet_decentralized_public_output: MPCPublicOutput,
    pub network_key_version: u8,
    /// Indicates whether the future sign feature was used to start the session.
    pub is_future_sign: bool,
}

impl MPCProtocolInitData {
    /// Returns `true` if the round is a single message, which is
    /// part of a batch, `false` otherwise.
    pub fn is_part_of_batch(&self) -> bool {
        matches!(
            self,
            MPCProtocolInitData::Sign(..) | MPCProtocolInitData::PresignSecond(..)
        )
    }

    /// Is a special Round that indicates an initialization of a batch session.
    pub fn is_a_new_batch_session(&self) -> bool {
        matches!(self, |MPCProtocolInitData::BatchedSign(..))
            || matches!(self, MPCProtocolInitData::BatchedPresign(..))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DWalletMPCEvent {
    // TODO: remove event - do all parsing beforehand.
    pub event: Event,
    pub session_info: SessionInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DWalletMPCOutputMessage {
    pub output: Vec<u8>,
    pub authority: AuthorityName,
    pub session_info: SessionInfo,
}

/// Metadata for a local MPC computation.
/// Includes the session ID and the cryptographic round.
///
/// Used to remove a pending computation if a quorum of outputs for the session
/// is received before the computation is spawned, or if a quorum of messages
/// for the next round of the computation is received, making the old round redundant.
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct DWalletMPCLocalComputationMetadata {
    pub session_id: ObjectID,
    pub crypto_round_number: usize,
}

/// The content of the system transaction that stores the MPC session output on the chain.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DWalletMPCOutput {
    /// The session information of the MPC session.
    pub session_info: SessionInfo,
    /// The final value of the MPC session.
    pub output: Vec<u8>,
}

impl Message for DWalletMPCOutput {
    type DigestType = DWalletMPCOutputDigest;
    const SCOPE: IntentScope = IntentScope::DWalletMPCOutput;

    fn digest(&self) -> Self::DigestType {
        DWalletMPCOutputDigest::new(default_hash(self))
    }
}

/// Holds information about the current MPC session.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SessionInfo {
    /// The session ID of the first round in the flow — e.g.,
    /// in Presign we have two rounds, so the session ID of the first.
    pub flow_session_id: ObjectID,
    /// Unique identifier for the MPC session.
    pub session_id: ObjectID,
    /// The address of the user that initiated this session.
    pub initiating_user_address: PeraAddress,
    /// The current MPC round in the protocol.
    /// Contains extra parameters if needed.
    pub mpc_round: MPCProtocolInitData,
}

/// The Rust representation of the `StartEncryptedShareVerificationEvent` Move struct.
/// Defined here so that we can use it in the [`MPCProtocolInitData`] enum,
/// as the inner data of the [`MPCProtocolInitData::EncryptedShareVerification`].
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartEncryptedShareVerificationEvent {
    /// Encrypted centralized secret key share and the associated
    /// cryptographic proof of encryption.
    pub encrypted_centralized_secret_share_and_proof: Vec<u8>,
    /// The public output of the centralized party,
    /// belongs to the dWallet that its centralized secret share is being encrypted.
    pub centralized_public_output: Vec<u8>,
    /// The signature of the dWallet `centralized_public_output`,
    /// signed by the secret key that corresponds to `encryptor_ed25519_pubkey`.
    pub centralized_public_output_signature: Vec<u8>,
    /// The ID of the dWallet that this encrypted secret key share belongs to.
    pub dwallet_id: ID,
    /// The encryption key used to encrypt the secret key share with.
    pub encryption_key: Vec<u8>,
    /// The `EncryptionKey` Move object ID.
    pub encryption_key_id: ID,
    pub session_id: ID,
    /// The public key of the encryptor.
    /// Used to verify the signature on the `centralized_public_output`.
    /// Note that the "encryptor" is the entity that preformed the encryption,
    /// and the encryption can be done with another public key, so this is NOT
    /// the public key that was used for encryption.
    pub encryptor_ed25519_pubkey: Vec<u8>,
    pub initiator: PeraAddress,
}

impl StartEncryptedShareVerificationEvent {
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: ident_str!("StartEncryptedShareVerificationEvent").to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// An event emitted to start an encryption key verification process.
/// Ika does not support native functions, so an event is emitted and
/// caught by the blockchain, which then starts the verification process,
/// similar to the MPC processes.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartEncryptionKeyVerificationEvent {
    pub encryption_key_scheme: u8,
    pub encryption_key: Vec<u8>,
    pub encryption_key_signature: Vec<u8>,
    pub key_singer_public_key: Vec<u8>,
    pub initiator: PeraAddress,
    pub session_id: ID,
}

impl StartEncryptionKeyVerificationEvent {
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: ident_str!("StartEncryptionKeyVerificationEvent").to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Represents the Rust version of the Move struct `pera_system::dwallet::StartDKGSecondRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartDKGSecondRoundEvent {
    /// The unique identifier for the DKG session.
    pub session_id: PeraAddress,
    /// The address of the user who initiated the dWallet creation.
    pub initiator: PeraAddress,
    /// The output from the first round of the DKG process.
    pub first_round_output: Vec<u8>,
    /// A serialized vector containing the centralized public key share and its proof.
    pub centralized_public_key_share_and_proof: Vec<u8>,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
    /// The session ID associated with the first DKG round.
    pub first_round_session_id: ID,
    /// Encrypted centralized secret key share and the associated cryptographic proof of encryption.
    pub encrypted_centralized_secret_share_and_proof: Vec<u8>,
    /// The `EncryptionKey` object used for encrypting the secret key share.
    pub encryption_key: Vec<u8>,
    /// The unique identifier of the `EncryptionKey` object.
    pub encryption_key_id: ID,
    /// The public output of the centralized party in the DKG process.
    pub centralized_public_output: Vec<u8>,
    /// The signature for the public output of the centralized party in the DKG process.
    pub centralized_public_output_signature: Vec<u8>,
    /// The Ed25519 public key of the initiator,
    /// used to verify the signature on the centralized public output.
    pub initiator_public_key: Vec<u8>,
}

impl StartDKGSecondRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartDKGSecondRoundEvent`] events from the chain
    /// and initiate the MPC session.
    pub fn type_() -> StructTag {
        StructTag {
            address: PERA_SYSTEM_ADDRESS,
            name: START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
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

impl MaliciousReport {
    /// Creates a new instance of a malicious report.
    pub fn new(malicious_actors: Vec<AuthorityName>, session_id: ObjectID) -> Self {
        Self {
            malicious_actors,
            session_id,
        }
    }
}
