use crate::crypto::default_hash;
use crate::crypto::AuthorityName;
use crate::digests::DWalletMPCOutputDigest;
use crate::dwallet_mpc_error::DwalletMPCError;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCMessageBuilder, MPCMessageSlice, MPCPublicInput, MessageState,
    NetworkDecryptionKeyShares, DWALLET_MPC_EVENT_STRUCT_NAME,
    START_DKG_FIRST_ROUND_EVENT_STRUCT_NAME, START_NETWORK_DKG_EVENT_STRUCT_NAME,
    START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME, START_SIGN_ROUND_EVENT_STRUCT_NAME,
};
use dwallet_mpc_types::dwallet_mpc::{
    MPCMessage, MPCPublicOutput, DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME, DWALLET_MODULE_NAME,
    START_DKG_SECOND_ROUND_EVENT_STRUCT_NAME,
};
use group::PartyID;
use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::{StructTag, TypeTag};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;
use std::collections::HashMap;
use sui_json_rpc_types::SuiEvent;
use sui_types::balance::Balance;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::collection_types::TableVec;
use sui_types::id::ID;
use sui_types::message_envelope::Message;
use sui_types::SUI_SYSTEM_ADDRESS;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MPCProtocolInitData {
    /// The first round of the DKG protocol.
    DKGFirst(DWalletMPCSuiEvent<StartDKGFirstRoundEvent>),
    /// The second round of the DKG protocol.
    /// Contains the data of the event that triggered the round,
    /// and the network key version of the first round.
    DKGSecond(DWalletMPCSuiEvent<StartDKGSecondRoundEvent>),
    /// The first round of the Presign protocol for each message in the Batch.
    /// Contains the `ObjectId` of the dWallet object,
    /// the DKG decentralized output, the batch session ID (same for each message in the batch),
    /// and the dWallets' network key version.
    Presign(DWalletMPCSuiEvent<StartPresignFirstRoundEvent>),
    /// The first and only round of the Sign protocol.
    /// Contains all the data needed to sign the message.
    Sign(DWalletMPCSuiEvent<StartSignEvent>),
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
    EncryptedShareVerification(DWalletMPCSuiEvent<StartEncryptedShareVerificationEvent>),
    PartialSignatureVerification(DWalletMPCSuiEvent<StartPartialSignaturesVerificationEvent>),
}

/// The session-specific state of the MPC session.
/// I.e., state needs to exist only in the sign protocol but is not required in the
/// presign protocol.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MPCSessionSpecificState {
    Sign(SignIASessionState),
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
    pub output: MPCMessageSlice,
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

/// The message a Validator can send to the other parties while
/// running a dWallet MPC session.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DWalletMPCMessage {
    /// The serialized message.
    pub message: MPCMessageSlice,
    /// The authority (Validator) that sent the message.
    pub authority: AuthorityName,
    pub session_id: ObjectID,
    /// The MPC round number, starts from 0.
    pub round_number: usize,
}

/// The message unique key in the consensus network.
/// Used to make sure no message is being processed twice.
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DWalletMPCMessageKey {
    /// The serialized message.
    pub message: MPCMessageSlice,
    /// The authority (Validator) that sent the message.
    pub authority: AuthorityName,
    pub session_id: ObjectID,
    /// The MPC round number, starts from 0.
    pub round_number: usize,
}

#[derive(Clone)]
pub struct MPCSessionMessagesCollector {
    pub messages: Vec<HashMap<PartyID, MPCMessageBuilder>>,
}

impl MPCSessionMessagesCollector {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn add_message(
        &mut self,
        party_id: PartyID,
        message: DWalletMPCMessage,
        round_number: usize,
    ) -> Option<Vec<u8>> {
        let message_bytes = match self.messages.get_mut(message.round_number) {
            Some(party_to_msg) => {
                if let Some(a) = party_to_msg.get_mut(&party_id) {
                    // there is key
                    a.add_message(message.message.clone());
                    a.build_message();
                    match &a.messages {
                        MessageState::Complete(message) => {
                            println!("complete message: {:?}", message.len());
                            Some(message.clone())
                        }
                        MessageState::Incomplete(messages) => {
                            println!("incomplete message: {:?}", messages.len());
                            None
                        }
                    }
                } else {
                    // build the message here, but where do I store it?
                    let mut messages_builder = MPCMessageBuilder {
                        messages: MessageState::Incomplete(
                            vec![(message.message.sequence_number, message.message.clone())]
                                .into_iter()
                                .collect::<HashMap<_, _>>(),
                        ),
                    };
                    messages_builder.build_message();
                    party_to_msg.insert(party_id, messages_builder.clone());
                    match &messages_builder.messages {
                        MessageState::Complete(message) => {
                            println!("ccomplete message: {:?}", message.len());
                            Some(message.clone())
                        }
                        MessageState::Incomplete(messages) => {
                            println!("iincomplete message: {:?}", messages.len());
                            None
                        }
                    }
                }
            }
            // If next round.
            None if message.round_number == round_number => {
                let mut map = HashMap::new();
                // let mut messages = MPCMessageBuilder { messages: MessageState::Incomplete(HashMap::from(vec![(message.message.sequence_number, message.message.clone())])) };
                let mut messages = MPCMessageBuilder {
                    messages: MessageState::Incomplete(
                        vec![(message.message.sequence_number, message.message.clone())]
                            .into_iter()
                            .collect::<HashMap<_, _>>(),
                    ),
                };
                messages.build_message();
                map.insert(party_id, messages.clone());
                // Build the message
                self.messages.push(map);

                match &messages.messages {
                    MessageState::Complete(message) => {
                        println!("cccomplete message: {:?}", message.len());
                        Some(message.clone())
                    }
                    MessageState::Incomplete(messages) => {
                        println!("iiincomplete message: {:?}", messages.len());
                        None
                    }
                }
            }
            None => {
                // Unexpected round number; rounds should grow sequentially.
                // return Err(DwalletMPCError::MaliciousParties(vec![party_id]));
                return None;
            }
        };
        message_bytes
    }

    pub fn collect_completed_messages(&self, round_number: usize) -> Vec<MPCMessage> {
        self.messages
            .get(round_number)
            .map(|party_to_msg| {
                party_to_msg
                    .values()
                    .filter_map(|msg| match &msg.messages {
                        MessageState::Complete(message) => Some(message.clone()),
                        MessageState::Incomplete(_) => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Holds information about the current MPC session.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Unique identifier for the MPC session.
    pub session_id: ObjectID,
    /// The current MPC round in the protocol.
    /// Contains extra parameters if needed.
    pub mpc_round: MPCProtocolInitData,
}

pub trait DWalletMPCEventTrait {
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag;
}

/// Represents the Rust version of the Move struct `ika_system::dwallet::DWalletMPCEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct DWalletMPCSuiEvent<E: DWalletMPCEventTrait> {
    pub epoch: u64,
    pub session_sequence_number: u64,
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
    /// The public output of the decentralized party,
    /// belongs to the dWallet that its centralized secret share is being encrypted.
    pub decentralized_public_output: Vec<u8>,
    /// The ID of the dWallet that this encrypted secret key share belongs to.
    pub dwallet_id: ObjectID,
    /// The encryption key used to encrypt the secret key share with.
    pub encryption_key: Vec<u8>,
    /// The `EncryptionKey` Move object ID.
    pub encryption_key_id: ObjectID,
    pub encrypted_user_secret_key_share_id: ObjectID,
    pub source_encrypted_user_secret_key_share_id: ObjectID,
}

impl DWalletMPCEventTrait for StartEncryptedShareVerificationEvent {
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: ident_str!("EncryptedShareVerificationRequestEvent").to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust representation of the Move `StartPartialSignaturesVerificationEvent` Event.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartPartialSignaturesVerificationEvent {
    pub dwallet_id: ObjectID,
    pub partial_centralized_signed_message_id: ObjectID,
    pub message: Vec<u8>,
    pub presign: Vec<u8>,
    pub dkg_output: Vec<u8>,
    pub hash_scheme: u8,
    pub message_centralized_signature: Vec<u8>,
    pub dwallet_mpc_network_key_id: ObjectID,
}

impl DWalletMPCEventTrait for StartPartialSignaturesVerificationEvent {
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: ident_str!("ECDSAFutureSignRequestEvent").to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
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
    pub dwallet_mpc_network_key_id: ObjectID,
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

/// Represents the Rust version of the Move struct `ika_system::dwallet_2pc_mpc_ecdsa_k1::StartPresignFirstRoundEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartPresignFirstRoundEvent {
    /// The `DWallet` object's ID associated with the DKG output.
    pub dwallet_id: ObjectID,
    pub presign_id: ObjectID,
    /// The DKG decentralized final output to use for the presign session.
    pub dkg_output: Vec<u8>,
    pub dwallet_network_decryption_key_id: ObjectID,
}

impl DWalletMPCEventTrait for StartPresignFirstRoundEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartPresignFirstRoundEvent`] events
    /// from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: START_PRESIGN_FIRST_ROUND_EVENT_STRUCT_NAME.to_owned(),
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

/// Represents the Rust version of the Move
/// struct `ika_system::dwallet::StartSignEvent`.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct StartSignEvent {
    pub sign_id: ObjectID,
    /// The `DWallet` object's ObjectID associated with the DKG output.
    pub dwallet_id: ObjectID,
    /// The public output of the decentralized party in the dWallet DKG process.
    pub dwallet_decentralized_public_output: Vec<u8>,
    pub hash_scheme: u8,
    /// Hashed messages to Sign.
    pub message: Vec<u8>,
    /// The dWallet mpc network key version
    pub dwallet_mpc_network_key_id: ObjectID,
    pub presign_id: ObjectID,

    /// The presign protocol output as bytes.
    pub presign: Vec<u8>,

    /// The centralized party signature of a message.
    pub message_centralized_signature: Vec<u8>,

    /// Indicates whether the future sign feature was used to start the session.
    pub is_future_sign: bool,
}

impl DWalletMPCEventTrait for StartSignEvent {
    /// This function allows comparing this event with the Move event.
    /// It is used to detect [`StartSignEvent`]
    /// events from the chain and initiate the MPC session.
    fn type_(packages_config: &IkaPackagesConfig) -> StructTag {
        StructTag {
            address: *packages_config.ika_system_package_id,
            name: START_SIGN_ROUND_EVENT_STRUCT_NAME.to_owned(),
            module: DWALLET_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust version of the Move [`ika_system::dwallet_network_key::StartNetworkDKGEvent`] type.
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

/// Represents the Rust version of the Move struct `ika_system::dwallet_2pc_mpc_secp256k1_inner::DWalletNetworkDecryptionKey`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DWalletNetworkDecryptionKey {
    pub id: ObjectID,
    pub dwallet_network_decryption_key_cap_id: ObjectID,
    pub current_epoch: u64,
    pub current_epoch_shares: TableVec,
    pub next_epoch_shares: TableVec,
    pub previous_epoch_shares: TableVec,
    pub public_output: TableVec,
    /// The fees paid for computation in IKA.
    pub computation_fee_charged_ika: Balance,
    pub state: DWalletNetworkDecryptionKeyState,
}

/// Represents the Rust version of the Move enum `ika_system::dwallet_2pc_mpc_secp256k1_inner::DWalletNetworkDecryptionKeyShares`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DWalletNetworkDecryptionKeyState {
    AwaitingNetworkDKG,
    NetworkDKGCompleted,
}
