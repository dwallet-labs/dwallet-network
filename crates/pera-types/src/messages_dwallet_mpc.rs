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
use group::PartyID;
use move_core_types::ident_str;
use move_core_types::language_storage::StructTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;
use std::collections::HashSet;

// todo(zeev): move the events to mpc_events and the types to `dwallet-mpc-types` crate.

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MPCProtocolInitData {
    /// The first round of the DKG protocol.
    DKGFirst,
    /// The second round of the DKG protocol.
    DKGSecond(StartDKGSecondRoundEvent, u8),
    /// The first round of the Presign protocol.
    /// Contains the `ObjectId` of the dWallet object,
    /// the DKG decentralized output, the batch session ID,
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
    BatchedPresign(u64),
    /// The round of the network DKG protocol.
    NetworkDkg(
        DWalletMPCNetworkKeyScheme,
        Option<NetworkDecryptionKeyShares>,
    ),
    /// The round of verifying the encrypted share proof is valid and
    /// that the signature on it is valid.
    EncryptedShareVerification(StartEncryptedShareVerificationEvent),
    /// The round of verifying the public key that signed on the encryption key is
    /// matching the initiator address.
    /// todo(zeev): more docs, make it clearer.
    /// TODO (#544): Check if there's a way to convert the public key to an address in Move.
    EncryptionKeyVerification(StartEncryptionKeyVerificationEvent),
    SignIdentifiableAbort(SignIASessionData),
}

/// The message and data for the Sign round.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SingleSignSessionData {
    pub batch_session_id: ObjectID,
    pub message: Vec<u8>,
    /// The dWallet ID that is used to sign, needed mostly for audit.
    pub dwallet_id: ObjectID,
    /// The DKG output of the dWallet, used to sign and verify the message.
    pub dkg_output: MPCPublicOutput,
    pub network_key_version: u8,
}

/// The message and data for the Sign round.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SignIASessionData {
    pub initiating_authority: AuthorityName,
    pub claimed_malicious_actors: Vec<AuthorityName>,
    pub sign_session_id: ObjectID,
    pub parties_used_for_last_step: Vec<PartyID>,
}

impl MPCProtocolInitData {
    /// Returns `true` if the round output is part of a batch, `false` otherwise.
    pub fn is_part_of_batch(&self) -> bool {
        matches!(
            self,
            MPCProtocolInitData::Sign(..)
                | MPCProtocolInitData::PresignSecond(..)
                | MPCProtocolInitData::BatchedSign(..)
                | MPCProtocolInitData::BatchedPresign(..)
                | MPCProtocolInitData::SignIdentifiableAbort(..)
        )
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
    pub involved_parties: Vec<PartyID>,
}

impl MaliciousReport {
    /// Creates a new instance of a malicious report.
    pub fn new(
        malicious_actors: Vec<AuthorityName>,
        session_id: ObjectID,
        involved_parties: Vec<PartyID>,
    ) -> Self {
        Self {
            malicious_actors,
            session_id,
            involved_parties,
        }
    }
}
