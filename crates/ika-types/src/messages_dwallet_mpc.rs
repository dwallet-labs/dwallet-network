use crate::crypto::default_hash;
use crate::crypto::AuthorityName;
use crate::digests::DWalletMPCOutputDigest;
use crate::dwallet_mpc_error::DwalletMPCError;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCPublicInput, NetworkDecryptionKeyShares,
    DWALLET_MPC_EVENT_STRUCT_NAME, START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME,
    START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME,
};
use dwallet_mpc_types::dwallet_mpc::{
    MPCMessage, MPCPublicOutput, DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME, DWALLET_MODULE_NAME,
    START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME,
};
use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::{StructTag, TypeTag};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;
use sui_json_rpc_types::SuiEvent;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::id::ID;
use sui_types::message_envelope::Message;
use sui_types::SUI_SYSTEM_ADDRESS;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MPCProtocolInitData {
    /// The first round of the DKG protocol.
    DKGFirst(StartDKGFirstRoundEvent),
    /// The second round of the DKG protocol.
    /// Contains the data of the event that triggered the round,
    /// and the network key version of the first round.
    DKGSecond(StartDKGSecondRoundEvent, u8),
    /// This is not a real round, but an indicator the Batches Manager to
    /// register a Presign Batch session.
    /// Holds the number of messages in the batch.
    BatchedPresign(u64),
    /// The first round of the Presign protocol for each message in the Batch.
    /// Contains the `ObjectId` of the dWallet object,
    /// the DKG decentralized output, the batch session ID (same for each message in the batch),
    /// and the dWallets' network key version.
    Presign(StartPresignFirstRoundEvent),
    /// The first and only round of the Sign protocol.
    /// Contains all the data needed to sign the message.
    Sign(SingleSignSessionData),
    /// A batched sign session, contains the list of messages that are being signed.
    // TODO (#536): Store batch state and logic on Sui & remove this field.
    BatchedSign(Vec<Vec<u8>>),
    /// The only round of the network DKG protocol.
    /// Contains the network key scheme
    /// and at the end of the session holds the new key version.
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
    /// TODO (#544): Check if there's a way to convert the public key to an address in Move.
    /// This is not a real MPC round,
    /// but we use it to start the verification process using the same events mechanism
    /// because the system does not support native functions.
    EncryptionKeyVerification(StartEncryptionKeyVerificationEvent),
    PartialSignatureVerification(StartPartialSignaturesVerificationEvent<SignData>),
}

/// The session-specific state of the MPC session.
/// I.e., state needs to exist only in the sign protocol but is not required in the
/// presign protocol.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MPCSessionSpecificState {
    Sign(SignIASessionState),
    Presign(PresignSessionState),
}

/// The optional state of the Presign session, if the first round party was
/// completed and agreed on.
/// If the first presign round was completed and agreed on,
/// the [`DWalletMPCSession`] `session_specific_state` will hold
/// this state.
/// If the first round was not completed, the `session_specific_state` will be `None`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PresignSessionState {
    /// The verified output from the first party of the Presign protocol.
    pub first_presign_party_output: MPCPublicOutput,
    /// The public input for the second party of the Presign protocol.
    pub second_party_public_input: MPCPublicInput,
}

/// This is a wrapper type for the [`SuiEvent`] type that is being used to write it to the local RocksDB.
/// This is needed because the [`SuiEvent`] cannot be directly written to the RocksDB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBSuiEvent {
    pub type_: StructTag,
    pub contents: Vec<u8>,
}

/// The state of a sign-identifiable abort session.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SignIASessionState {
    /// The first report that triggered the beginning of the Sign-Identifiable Abort protocol,
    /// in which, instead of having only one validator run the last sign step, every validator runs
    /// the last step to agree on the malicious actors.
    pub start_ia_flow_malicious_report: MaliciousReport,
    /// The malicious report that has been agreed upon by a quorum of validators.
    /// If this report
    /// is different from the `start_ia_flow_malicious_report`, the authority that sent the
    /// `start_ia_flow_malicious_report` is being marked as malicious.
    pub verified_malicious_report: Option<MaliciousReport>,
    /// The first authority that sent a [`MaliciousReport`] in this sign session and triggered
    /// the beginning of the Sign-Identifiable Abort flow.
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
    pub presign_session_id: ObjectID,
}

impl MPCProtocolInitData {
    /// Returns `true` if the round is a single message, which is
    /// part of a batch, `false` otherwise.
    pub fn is_part_of_batch(&self) -> bool {
        matches!(
            self,
            MPCProtocolInitData::Sign(..) | MPCProtocolInitData::Presign(..)
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
    pub event: DBSuiEvent,
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

/// The message a Validator can send to the other parties while
/// running a dWallet MPC session.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DWalletMPCMessage {
    /// The serialized message.
    pub message: MPCMessage,
    /// The authority (Validator) that sent the message.
    pub authority: AuthorityName,
    pub session_id: ObjectID,
    /// The MPC round number, starts from 0.
    pub round_number: usize,
}

/// Holds information about the current MPC session.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Unique identifier for the MPC session.
    pub session_id: ObjectID,
    /// The address of the user that initiated this session.
    pub initiating_user_address: SuiAddress,
    /// The current MPC round in the protocol.
    /// Contains extra parameters if needed.
    pub mpc_round: MPCProtocolInitData,
}

pub trait DWalletMPCEventTrait {
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag;
}

/// Represents the Rust version of the Move struct `ika_system::dwallet::DWalletMPCEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct DWalletMPCSuiEvent<E: DWalletMPCEventTrait> {
    pub epoch: u64,
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

/// The Rust representation of the `StartEncryptedShareVerificationEvent` Move struct.
/// Defined here so that we can use it in the [`MPCProtocolInitData`] enum,
/// as the inner data of the [`MPCProtocolInitData::EncryptedShareVerification`].
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartEncryptedShareVerificationEvent {
    /// Encrypted centralized secret key share and the associated
    /// cryptographic proof of encryption.
    pub encrypted_centralized_secret_share_and_proof: Vec<u8>,
    /// The signature of the dWallet `decentralized_public_output`,
    /// signed by the secret key that corresponds to `encryptor_ed25519_pubkey`.
    pub decentralized_public_output_signature: Vec<u8>,
    /// The public output of the decentralized party,
    /// belongs to the dWallet that its centralized secret share is being encrypted.
    pub decentralized_public_output: Vec<u8>,
    /// The ID of the dWallet that this encrypted secret key share belongs to.
    pub dwallet_id: ObjectID,
    /// The encryption key used to encrypt the secret key share with.
    pub encryption_key: Vec<u8>,
    /// The `EncryptionKey` Move object ID.
    pub encryption_key_id: ObjectID,
    pub session_id: ObjectID,
    /// The public key of the encryptor.
    /// Used to verify the signature on the `centralized_public_output`.
    /// Note that the "encryptor" is the entity that preformed the encryption,
    /// and the encryption can be done with another public key, so this is NOT
    /// the public key that was used for encryption.
    pub encryptor_ed25519_pubkey: Vec<u8>,
    pub initiator: SuiAddress,
}

impl DWalletMPCEventTrait for StartEncryptedShareVerificationEvent {
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_package_id,
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
    pub key_signer_public_key: Vec<u8>,
    pub initiator: SuiAddress,
    pub session_id: ObjectID,
}

impl DWalletMPCEventTrait for StartEncryptionKeyVerificationEvent {
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_package_id,
            name: ident_str!("StartEncryptionKeyVerificationEvent").to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust representation of the Move `StartPartialSignaturesVerificationEvent` Event.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartPartialSignaturesVerificationEvent<D> {
    pub session_id: ObjectID,
    pub messages: Vec<Vec<u8>>,
    pub hashed_messages: Vec<Vec<u8>>,
    pub dwallet_id: ObjectID,
    pub dwallet_decentralized_public_output: Vec<u8>,
    pub dwallet_cap_id: ObjectID,
    pub dwallet_mpc_network_decryption_key_version: u8,
    pub signature_data: Vec<D>,
    pub initiator: SuiAddress,
}

impl DWalletMPCEventTrait for StartPartialSignaturesVerificationEvent<SignData> {
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_package_id,
            name: ident_str!("StartPartialSignaturesVerificationEvent").to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![SignData::type_(packages_config).into()],
        }
    }
}

/// Represents the Rust version of the Move struct `pera_system::dwallet::StartDKGSecondRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartDKGSecondRoundEvent {
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
}

impl DWalletMPCEventTrait for StartDKGSecondRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartDKGSecondRoundEvent`] events from the chain
    /// and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME.to_owned(),
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
    pub advance_result: AdvanceResult,
    /// The unique identifier of the MPC session in which the malicious activity occurred.
    pub session_id: ObjectID,
}

impl MaliciousReport {
    /// Creates a new instance of a malicious report.
    pub fn new(
        malicious_actors: Vec<AuthorityName>,
        session_id: ObjectID,
        advance_result: AdvanceResult,
    ) -> Self {
        Self {
            malicious_actors,
            session_id,
            advance_result,
        }
    }
}

const SIGN_DATA_STRUCT_NAME: &IdentStr = ident_str!("SignData");

/// A representation of the Move object [`SignData`], which stores data specific to the
/// signing algorithm used in the MPC protocol.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct SignData {
    /// The presign object ID, which will be used as the sign MPC protocol ID.
    pub presign_id: ObjectID,
    /// The presign protocol output, serialized as bytes.
    pub presign_output: Vec<u8>,
    /// The centralized signature of a message.
    pub message_centralized_signature: Vec<u8>,
}

impl DWalletMPCEventTrait for SignData {
    /// This function returns the `StructTag` representation of the Move [`SignData`] object,
    /// allowing it to be compared with the corresponding Move object on the chain.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_package_id,
            name: SIGN_DATA_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Represents the Rust version of the Move struct `ika_system::dwallet_2pc_mpc_ecdsa_k1::StartPresignFirstRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartPresignFirstRoundEvent {
    /// Unique identifier for the MPC session.
    pub session_id: ObjectID,
    /// The address of the user that initiated this session.
    pub initiator: SuiAddress,
    /// The `DWallet` object's ID associated with the DKG output.
    pub dwallet_id: ObjectID,
    /// The DKG decentralized final output to use for the presign session.
    pub dkg_output: Vec<u8>,
    /// A unique identifier for the entire batch,
    /// used to collect all the presigns in the batch and complete it.
    pub batch_session_id: ObjectID,
    /// The dWallet mpc network key version
    pub dwallet_mpc_network_key_version: u8,
}

impl DWalletMPCEventTrait for StartPresignFirstRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartPresignFirstRoundEvent`] events
    /// from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_package_id,
            name: START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct IkaPackagesConfig {
    /// The move package ID of ika (IKA) on sui.
    pub ika_package_id: ObjectID,
    /// The move package ID of `ika_system` on sui.
    pub ika_system_package_id: ObjectID,
    /// The object ID of ika_system_state on sui.
    pub system_id: ObjectID,
}

/// Represents the Rust version of the Move struct `ika_system::dwallet::StartDKGFirstRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartDKGFirstRoundEvent {
    pub dwallet_id: ObjectID,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ObjectID,
    pub dwallet_network_decryption_key_id: ObjectID,
}

impl DWalletMPCEventTrait for StartDKGFirstRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartDKGFirstRoundEvent`] events from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}
