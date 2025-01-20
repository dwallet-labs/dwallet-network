use crate::base_types::{ObjectID, PeraAddress};
use crate::crypto::default_hash;
use crate::digests::DWalletMPCOutputDigest;
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MPCRound {
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
    PresignFirst(ObjectID, MPCPublicOutput, ObjectID, u8),
    /// The second round of the Presign protocol.
    /// Contains the `ObjectId` of the dWallet object,
    /// the Presign first round output, and the batch session ID.
    PresignSecond(ObjectID, MPCPublicOutput, ObjectID),
    /// This is not a real round, but an indicator the Batches Manager to
    /// register a Sign Batch session.
    BatchedSign(Vec<Vec<u8>>),
    /// The first and only round of the Sign protocol for each message in the Batch.
    Sign(SignMessageData),
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
    /// This is not a real MPC round,
    /// but we use it to start the verification process using the same events mechanism
    /// because the system does not support native functions.
    EncryptionKeyVerification(StartEncryptionKeyVerificationEvent),
}

/// The message and data for the Sign round.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SignMessageData {
    pub batch_session_id: ObjectID,
    pub message: Vec<u8>,
    /// The dWallet ID that is used to sign, needed mostly for audit.
    pub dwallet_id: ObjectID,
}

impl MPCRound {
    /// Returns `true` if the round is a single message, which is
    /// part of a batch, `false` otherwise.
    pub fn is_part_of_batch(&self) -> bool {
        matches!(self, MPCRound::Sign(..) | MPCRound::PresignSecond(..))
    }

    /// Is a special Round that indicates an initialization of a batch session.
    pub fn is_a_new_batch_session(&self) -> bool {
        matches!(self, |MPCRound::BatchedSign(..)| MPCRound::BatchedPresign(
            ..
        ))
    }
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
    pub mpc_round: MPCRound,
}

/// The Rust representation of the `StartEncryptedShareVerificationEvent` Move struct.
/// Defined here so that we can use it in the [`MPCRound`] enum,
/// as the inner data of the [`MPCRound::EncryptedShareVerification`].
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartEncryptedShareVerificationEvent {
    pub encrypted_secret_share_and_proof: Vec<u8>,
    pub dwallet_centralized_public_output: Vec<u8>,
    pub dwallet_id: ID,
    pub encryption_key: Vec<u8>,
    pub encryption_key_id: ID,
    pub session_id: ID,
    pub signed_public_share: Vec<u8>,
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
/// Since we cannot use native functions if we depend on Sui to hold our state,
/// we need to emit an event to start the verification process, like we start the other MPC processes.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartEncryptionKeyVerificationEvent {
    pub scheme: u8,
    pub encryption_key: Vec<u8>,
    pub key_owner_address: PeraAddress,
    pub encryption_key_signature: Vec<u8>,
    pub sender_sui_pubkey: Vec<u8>,
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
    /// Unique identifier for the MPC session.
    pub session_id: PeraAddress,
    /// The address of the user that initiated this session.
    pub initiator: PeraAddress,
    /// The DKG first decentralized round output.
    pub first_round_output: Vec<u8>,
    /// The DKG centralized round output.
    pub public_key_share_and_proof: Vec<u8>,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ID,
    pub first_round_session_id: ID,
    pub encrypted_secret_share_and_proof: Vec<u8>,
    pub encryption_key: Vec<u8>,
    pub encryption_key_id: ID,
    pub signed_public_share: Vec<u8>,
    pub encryptor_ed25519_pubkey: Vec<u8>,
    pub dkg_centralized_public_output: Vec<u8>,
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
