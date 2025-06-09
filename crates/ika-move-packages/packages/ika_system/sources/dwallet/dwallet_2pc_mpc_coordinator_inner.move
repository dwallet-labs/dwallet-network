// Copyright (c) dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// # dWallet 2PC-MPC Coordinator Inner Module
/// 
/// This module implements the core logic for creating and managing dWallets using 
/// Multi-Party Computation (MPC) protocols. It provides a trustless and decentralized
/// approach to wallet creation and key management through distributed key generation (DKG)
/// and threshold signing protocols.
/// 
/// ## Key Features
/// - Distributed Key Generation (DKG) for secure key creation
/// - Threshold signing with presign optimization
/// - Network encryption key management and reconfiguration
/// - User encryption key registration and management
/// - Session-based MPC protocol coordination
/// - Epoch-based validator committee transitions
/// - Comprehensive pricing and fee management
/// - Support for multiple cryptographic curves and algorithms
/// 
/// ## Architecture
/// The module is organized around the `DWalletCoordinatorInner` struct which manages:
/// - dWallet lifecycle and state transitions
/// - MPC session coordination and scheduling
/// - Validator committee management
/// - Cryptographic algorithm support and emergency controls
/// - Economic incentives through pricing and fee collection

module ika_system::dwallet_2pc_mpc_coordinator_inner;

// === Imports ===

use sui::{
    table_vec::{Self, TableVec},
    object_table::{Self, ObjectTable},
    table::{Self, Table},
    balance::{Self, Balance},
    bcs,
    coin::Coin,
    bag::{Self, Bag},
    event,
    ed25519::ed25519_verify,
    vec_map::VecMap,
    sui::SUI,
};

use ika::ika::IKA;

use ika_system::{
    address,
    bls_committee::{Self, BlsCommittee},
    dwallet_pricing::{Self, DWalletPricing, DWalletPricingValue, DWalletPricingCalculationVotes},
};

// === Constants ===

/// Intent bytes for checkpoint message verification to prevent replay attacks
const CHECKPOINT_MESSAGE_INTENT: vector<u8> = vector[1, 0, 0];

// Protocol flags for different MPC operations
// Used for pricing configuration and protocol identification

/// DKG first round protocol identifier
const DKG_FIRST_ROUND_PROTOCOL_FLAG: u32 = 0;
/// DKG second round protocol identifier  
const DKG_SECOND_ROUND_PROTOCOL_FLAG: u32 = 1;
/// User share re-encryption protocol identifier
const RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG: u32 = 2;
/// Make user secret key share public protocol identifier
const MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG: u32 = 3;
/// Imported key dWallet verification protocol identifier
const IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG: u32 = 4;
/// Presign generation protocol identifier
const PRESIGN_PROTOCOL_FLAG: u32 = 5;
/// Standard signing protocol identifier
const SIGN_PROTOCOL_FLAG: u32 = 6;
/// Future/conditional signing protocol identifier
const FUTURE_SIGN_PROTOCOL_FLAG: u32 = 7;
/// Signing with partial user signature protocol identifier
const SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG: u32 = 8;

// Message data type constants corresponding to MessageKind enum variants (in ika-types/src/message.rs)
const RESPOND_DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE: u64 = 0;
const RESPOND_DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE: u64 = 1;
const RESPOND_DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE: u64 = 2;
const RESPOND_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE: u64 = 3;
const RESPOND_DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE: u64 = 4;
const RESPOND_DWALLET_PRESIGN_MESSAGE_TYPE: u64 = 5;
const RESPOND_DWALLET_SIGN_MESSAGE_TYPE: u64 = 6;
const RESPOND_DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE: u64 = 7;
const RESPOND_DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE: u64 = 8;
const RESPOND_DWALLET_MPC_NETWORK_RECONFIGURATION_OUTPUT_MESSAGE_TYPE: u64 = 9;
const SET_MAX_ACTIVE_SESSIONS_BUFFER_MESSAGE_TYPE: u64 = 10;
const SET_GAS_FEE_REIMBURSEMENT_SUI_SYSTEM_CALL_VALUE_MESSAGE_TYPE: u64 = 11;

// === Errors ===

/// dWallet parameters do not match expected values
const EDWalletMismatch: u64 = 1;
/// dWallet is not in active state for requested operation
const EDWalletInactive: u64 = 2;
/// Referenced dWallet does not exist
const EDWalletNotExists: u64 = 3;
/// Object is in wrong state for requested operation
const EWrongState: u64 = 4;
/// Referenced network encryption key does not exist
const EDWalletNetworkEncryptionKeyNotExist: u64 = 5;
/// Encryption key signature verification failed
const EInvalidEncryptionKeySignature: u64 = 6;
/// Message approval parameters do not match partial signature
const EMessageApprovalMismatch: u64 = 7;
/// Specified hash scheme is not supported
const EInvalidHashScheme: u64 = 8;
/// Signing session is in wrong state
const ESignWrongState: u64 = 9;
/// Referenced presign does not exist
const EPresignNotExist: u64 = 10;
/// Capability does not match expected object
const EIncorrectCap: u64 = 11;
/// Capability has not been verified
const EUnverifiedCap: u64 = 12;
/// Invalid source for re-encryption operation
const EInvalidSource: u64 = 13;
/// Network encryption key is not in active state
const EDWalletNetworkEncryptionKeyNotActive: u64 = 14;
/// Presign is invalid or incomplete
const EInvalidPresign: u64 = 15;
/// Cannot advance epoch due to incomplete sessions
const ECannotAdvanceEpoch: u64 = 16;
/// Specified cryptographic curve is not supported
const EInvalidCurve: u64 = 17;
/// Specified signature algorithm is not supported
const EInvalidSignatureAlgorithm: u64 = 18;
/// Cryptographic curve is temporarily paused
const ECurvePaused: u64 = 19;
/// Signature algorithm is temporarily paused
const ESignatureAlgorithmPaused: u64 = 20;
/// dWallet user secret key shares are already public
const EDWalletUserSecretKeySharesAlreadyPublic: u64 = 21;
/// Cryptographic curve mismatch between objects
const EMismatchCurve: u64 = 22;
/// Operation not allowed on imported key dWallet
const EImportedKeyDWallet: u64 = 23;
/// Operation requires imported key dWallet
const ENotImportedKeyDWallet: u64 = 24;
/// Hash scheme is temporarily paused
const EHashSchemePaused: u64 = 25;
/// Referenced encryption key does not exist
const EEncryptionKeyNotExist: u64 = 26;
/// Pricing configuration missing for protocol
const EMissingProtocolPricing: u64 = 27;
/// Pricing calculation votes have not been initiated
const EPricingCalculationVotesHasNotBeenStarted: u64 = 28;
/// Pricing calculation votes must complete before epoch advance
const EPricingCalculationVotesMustBeCompleted: u64 = 29;
/// Cannot modify settings during active pricing calculation
const ECannotSetDuringVotesCalculation: u64 = 30;
/// Insufficient IKA payment
const EInsufficientIKAPayment: u64 = 31;
/// Insufficient SUI payment
const EInsufficientSUIPayment: u64 = 32;

#[error]
const EIncorrectEpochInCheckpoint: vector<u8> = b"The checkpoint epoch is incorrect.";

#[error]
const EWrongCheckpointSequenceNumber: vector<u8> = b"The checkpoint sequence number should be the expected next one.";

#[error]
const EActiveBlsCommitteeMustInitialize: vector<u8> = b"First active committee must initialize.";

// === Structs ===

/// Session management data for the dWallet coordinator.
public struct SessionManagement has store {
    /// Active sessions indexed by sequence number
    sessions: ObjectTable<u64, DWalletSession>,
    /// Events for user-requested sessions, keyed by session ID
    user_requested_sessions_events: Bag,
    /// Number of completed user-initiated sessions
    number_of_completed_user_initiated_sessions: u64,
    /// Count of started system sessions
    started_system_sessions_count: u64,
    /// Count of completed system sessions
    completed_system_sessions_count: u64,
    /// The sequence number to assign to the next user-requested session.
    /// Initialized to `1` and incremented at every new session creation.
    next_session_sequence_number: u64,
    /// The last MPC session to process in the current epoch.
    /// The validators of the Ika network must always begin sessions,
    /// when they become available to them, so long their sequence number is lesser or equal to this value.
    /// Initialized to `0`, as when the system is initialized no user-requested session exists so none should be started
    /// and we shouldn't wait for any to complete before advancing epoch (until the first session is created),
    /// and updated at every new session creation or completion, and when advancing epochs,
    /// to the latest session whilst assuring a maximum of `max_active_sessions_buffer` sessions to be completed in the current epoch.
    /// Validators should complete every session they start before switching epochs.
    last_user_initiated_session_to_complete_in_current_epoch: u64,
    /// Denotes whether the `last_user_initiated_session_to_complete_in_current_epoch` field is locked or not.
    /// This field gets locked before performing the epoch switch.
    locked_last_user_initiated_session_to_complete_in_current_epoch: bool,
    /// The maximum number of active MPC sessions Ika nodes may run during an epoch.
    /// Validators should complete every session they start before switching epochs.
    max_active_sessions_buffer: u64,
}

/// Support data for the dWallet coordinator, including curve and algorithm configurations.
public struct SupportConfig has store {
    /// A nested map of supported curves to signature algorithms to hash schemes.
    /// e.g. secp256k1 -> [(ecdsa -> [sha256, keccak256]), (schnorr -> [sha256])]
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap<u32, VecMap<u32, vector<u32>>>,
    /// List of paused curves in case of emergency (e.g. [secp256k1, ristretto])
    paused_curves: vector<u32>,
    /// List of paused signature algorithms in case of emergency (e.g. [ecdsa, schnorr])
    paused_signature_algorithms: vector<u32>,
    /// List of paused hash schemes in case of emergency (e.g. [sha256, keccak256])
    paused_hash_schemes: vector<u32>,
    /// Signature algorithms that are allowed for global presign
    signature_algorithms_allowed_global_presign: vector<u32>,
}

/// Pricing and fee management data for the dWallet coordinator.
public struct PricingAndFeeManagement has store {
    /// Pricing for the current epoch
    current: DWalletPricing,
    /// Default pricing configuration
    default: DWalletPricing,
    /// Validator votes for pricing (validator ID -> pricing vote)
    validator_votes: Table<ID, DWalletPricing>,
    /// Pricing calculation votes - if set, must complete before epoch advance
    calculation_votes: Option<DWalletPricingCalculationVotes>,
    /// Gas fee reimbursement value for system calls
    gas_fee_reimbursement_sui_system_call_value: u64,
    /// SUI balance for gas fee reimbursement to fund network tx responses
    gas_fee_reimbursement_sui: Balance<SUI>,
    /// IKA fees charged for consensus validation
    consensus_validation_fee_charged_ika: Balance<IKA>,
}

/// Core coordinator for dWallet 2PC-MPC operations.
/// 
/// This shared object manages all aspects of dWallet creation and operation:
/// - dWallet lifecycle (DKG, signing, presigning)
/// - Network encryption keys and user encryption
/// - Session management and epoch transitions
/// - Pricing and fee collection
/// - Committee management and consensus
///
/// Most importantly, the `dwallets` themselves, which holds the public key and public key shares,
/// and the encryption of the network's share under the network's threshold encryption key.
/// The encryption of the network's secret key share for every dWallet points to an encryption key in `dwallet_network_encryption_keys`,
/// which also stores the encrypted encryption key shares of each validator and their public verification keys.
/// 
/// For the user side, the secret key share is stored encrypted to the user encryption key (in `encryption_keys`) inside the dWallet,
/// together with a signature on the public key (shares).
/// Together, these constitute the necessary information to create a signature with the user.
///
/// Next, `presign_sessions` holds the outputs of the Presign protocol which are later used for the signing protocol,
/// and `partial_centralized_signed_messages` holds the partial signatures of users awaiting for a future sign once a `MessageApproval` is presented.
///
/// Additionally, this structure holds management information, like the `previous_committee` and `active_committee` committees,
/// information regarding `pricing_and_fee_management`, all the `session_management` and the `next_session_sequence_number` that will be used for the next session,
/// and various other fields, like the supported and paused curves, signing algorithms and hashes.
/// 
/// ## Key Components:
/// - `dwallets`: Core dWallet objects with public keys and encrypted shares
/// - `dwallet_network_encryption_keys`: Network threshold encryption keys
/// - `encryption_keys`: User encryption keys for secure share storage
/// - `presign_sessions`: Precomputed signing materials
/// - `partial_centralized_signed_messages`: Future sign capabilities
/// - `session_management`: MPC session coordination
/// - `pricing_and_fee_management`: Economic incentives and fee collection
/// - `active_committee`/`previous_committee`: Validator consensus groups
/// - `support_config`: Cryptographic algorithm support and emergency controls
public struct DWalletCoordinatorInner has store {
    /// Current epoch number
    current_epoch: u64,
    /// Session management and coordination
    session_management: SessionManagement,
    // TODO: change to versioned
    /// All dWallet instances (DWallet ID -> DWallet)
    dwallets: ObjectTable<ID, DWallet>,
    // TODO: change to versioned
    /// Network encryption keys (Network encryption key ID -> DWalletNetworkEncryptionKey)
    dwallet_network_encryption_keys: ObjectTable<ID, DWalletNetworkEncryptionKey>,
    // TODO: change to versioned
    /// User encryption keys (User encryption key address -> EncryptionKey)
    encryption_keys: ObjectTable<address, EncryptionKey>,
    /// Presign sessions for signature optimization (Presign session ID -> PresignSession)
    presign_sessions: ObjectTable<ID, PresignSession>,
    /// Partial user signatures for future signing (Partial user signature ID -> PartialUserSignature)
    partial_centralized_signed_messages: ObjectTable<ID, PartialUserSignature>,
    /// Pricing and fee management
    pricing_and_fee_management: PricingAndFeeManagement,
    /// Current active validator committee
    active_committee: BlsCommittee,
    /// Previous validator committee
    previous_committee: BlsCommittee,
    /// Total number of messages processed
    total_messages_processed: u64,
    /// Last processed checkpoint sequence number
    last_processed_checkpoint_sequence_number: Option<u64>,
    /// Last checkpoint sequence number from previous epoch
    previous_epoch_last_checkpoint_sequence_number: u64,
    /// Cryptographic algorithm support configuration
    support_config: SupportConfig,
    /// Any extra fields that's not defined statically
    extra_fields: Bag,
}

/// Represents an active MPC session in the Ika network.
/// 
/// Each session tracks fees and is associated with a network encryption key.
/// Sessions are sequentially numbered for epoch management.
public struct DWalletSession has key, store {
    id: UID,
    /// Sequential number for session ordering
    session_sequence_number: u64,
    /// Associated network encryption key
    dwallet_network_encryption_key_id: ID,
    /// IKA fees for consensus validation
    consensus_validation_fee_charged_ika: Balance<IKA>,
    /// IKA fees for computation
    computation_fee_charged_ika: Balance<IKA>,
    /// SUI balance for gas reimbursement
    gas_fee_reimbursement_sui: Balance<SUI>,
}

/// Capability granting control over a specific dWallet.
/// 
/// This capability allows the holder to perform operations on the associated dWallet,
/// such as requesting signatures, managing encryption keys, and approving messages.
public struct DWalletCap has key, store {
    id: UID,
    /// ID of the controlled dWallet
    dwallet_id: ID,
}

/// Capability granting control over a specific imported key dWallet.
/// 
/// Similar to DWalletCap but specifically for dWallets created from imported keys
/// rather than through the DKG process.
public struct ImportedKeyDWalletCap has key, store {
    id: UID,
    /// ID of the controlled imported key dWallet
    dwallet_id: ID,
}

/// Capability granting control over a specific dWallet network encryption key.
/// 
/// This capability allows management of network-level encryption keys used
/// for threshold encryption in the MPC protocols.
public struct DWalletNetworkEncryptionKeyCap has key, store {
    id: UID,
    /// ID of the controlled network encryption key
    dwallet_network_encryption_key_id: ID,
}

/// Network-owned threshold encryption key for dWallet MPC protocols.
/// 
/// This key enables the validator network to securely store and manage encrypted
/// shares of dWallet secret keys. It supports reconfiguration across epochs to
/// maintain security as the validator set changes.
/// 
/// ## Lifecycle Phases
/// 
/// ### Initial Creation
/// - Network DKG generates the initial threshold encryption key
/// - `network_dkg_public_output` contains the key and validator shares
/// 
/// ### Reconfiguration
/// - Triggered before epoch transitions when validator set changes
/// - `reconfiguration_public_outputs` stores updated keys per epoch
/// - Ensures continuous security across validator set changes
/// 
/// ## Data Storage Strategy
/// - Large cryptographic outputs are chunked due to storage limitations
/// - Chunked data is reconstructed during verification and usage
/// - Supports both initial DKG and ongoing reconfiguration outputs
/// 
/// ## Security Properties
/// - Threshold encryption protects against individual validator compromise
/// - Reconfiguration maintains security across validator set changes
/// - Cryptographic proofs ensure data integrity
public struct DWalletNetworkEncryptionKey has key, store {
    id: UID,
    /// ID of the capability that controls this encryption key
    dwallet_network_encryption_key_cap_id: ID,
    /// Current epoch for this encryption key
    current_epoch: u64,
    /// Reconfiguration outputs indexed by epoch (Epoch -> Chunked Output)
    reconfiguration_public_outputs: sui::table::Table<u64, TableVec<vector<u8>>>,
    /// Initial network DKG output (chunked for storage efficiency)
    network_dkg_public_output: TableVec<vector<u8>>,
    /// IKA fees accumulated for computation services
    computation_fee_charged_ika: Balance<IKA>,
    /// Current operational state
    state: DWalletNetworkEncryptionKeyState,
}

/// State of a dWallet network encryption key throughout its lifecycle
public enum DWalletNetworkEncryptionKeyState has copy, drop, store {
    /// Waiting for network DKG to complete
    AwaitingNetworkDKG,
    /// Network DKG has completed successfully
    NetworkDKGCompleted,
    /// Reconfiguration request was sent to the network, but didn't finish yet.
    /// `is_first` is true if this is the first reconfiguration request, false otherwise.
    AwaitingNetworkReconfiguration {
        is_first: bool,
    },
    /// Reconfiguration request finished, but we didn't switch an epoch yet.
    /// We need to wait for the next epoch to update the reconfiguration public outputs.
    AwaitingNextEpochToUpdateReconfiguration,
    /// Network reconfiguration has completed successfully
    NetworkReconfigurationCompleted,
}

/// User encryption key for secure dWallet secret key share storage.
/// 
/// Encryption keys enable secure transfer and storage of encrypted user secret key shares
/// between accounts. Each user address has an associated encryption key that allows
/// others to encrypt data specifically for that user to ensure sensitive information 
/// remains confidential during transmission.
/// 
/// Each address on the Ika is associated with a unique encryption key.
/// When a user intends to send encrypted data (i.e. when sharing the secret key share to grant access and/or transfer a dWallet) to another user,
/// they use the recipient's encryption key to encrypt the data.
/// The recipient is then the sole entity capable of decrypting and accessing this information, ensuring secure, end-to-end encryption.
/// 
/// ## Security Model
/// - Keys are Ed25519-signed to prove authenticity
/// - Each address maintains one active encryption key
/// - Keys support various cryptographic curves
/// - Encrypted shares can only be decrypted by the key owner
/// 
/// ## Use Cases
/// - Encrypting user secret key shares during dWallet creation
/// - Re-encrypting shares for access transfer or dWallet sharing
public struct EncryptionKey has key, store {
    /// Unique identifier for this encryption key
    id: UID,
    /// Epoch when this key was created
    created_at_epoch: u64,
    /// Cryptographic curve this key supports
    curve: u32,
    // TODO: Include class group type and version in the bytes (Rust side)
    /// Serialized encryption key data
    encryption_key: vector<u8>,
    /// Ed25519 signature proving encryption key authenticity, signed by the `signer_public_key`.
    /// Used to verify the data originated from the `signer_address`.
    encryption_key_signature: vector<u8>,
    /// Ed25519 public key used to create the signature
    signer_public_key: vector<u8>,
    /// Address of the encryption key owner
    signer_address: address,
}

/// Encrypted user secret key share with cryptographic verification.
/// 
/// Represents a user's secret key share that has been encrypted to a specific
/// user's encryption key. Includes zero-knowledge proofs that the encryption
/// is valid and corresponds to the dWallet's public key share.
/// 
/// ## Verification Process
/// 1. Network verifies the encryption proof
/// 2. User decrypts and verifies the share matches the public output
/// 3. User signs the public output to accept the share
/// 
/// ## Creation Methods
/// - **Direct**: Created during DKG second round
/// - **Re-encryption**: Created when transferring access to another user
/// 
/// ## Security Properties
/// - Zero-knowledge proof ensures encryption correctness
/// - Only the target user can decrypt the share
/// - Cryptographically linked to the associated dWallet
public struct EncryptedUserSecretKeyShare has key, store {
    /// Unique identifier for this encrypted share
    id: UID,
    /// Epoch when this share was created
    created_at_epoch: u64,
    /// ID of the dWallet this share belongs to
    dwallet_id: ID,
    /// Encrypted secret share with zero-knowledge proof of correctness
    /// for the dWallet's secret key share (of `dwallet_id`).
    encrypted_centralized_secret_share_and_proof: vector<u8>,
    /// ID of the encryption key used for encryption
    encryption_key_id: ID,
    /// Address of the encryption key owner
    encryption_key_address: address,
    /// Source share ID if this was created via re-encryption (None for DKG-created)
    source_encrypted_user_secret_key_share_id: Option<ID>,
    /// Current verification and acceptance state
    state: EncryptedUserSecretKeyShareState,
}
/// State of an encrypted user secret key share throughout verification and acceptance
public enum EncryptedUserSecretKeyShareState has copy, drop, store {
    /// Waiting for network to verify the encryption proof
    AwaitingNetworkVerification,
    /// Network has successfully verified the encryption
    NetworkVerificationCompleted,
    /// Network has rejected the encryption verification
    NetworkVerificationRejected,
    /// Key holder has signed and accepted the share
    KeyHolderSigned {
        /// The signed public share corresponding to the encrypted secret key share,
        /// used to verify its authenticity.
        user_output_signature: vector<u8>,
    }
}

/// Unverified capability for a partial user signature requiring network validation.
/// 
/// This capability is issued when a user creates a partial signature but must be
/// verified by the network before it can be used for conditional signing.
/// 
/// ## Verification Process
/// 1. Network validates the user's partial signature
/// 2. Network verifies the signature matches the message and dWallet
/// 3. Network confirms the presign material is valid
/// 4. Capability becomes verified and ready for use
/// 
/// ## Security Properties
/// - Prevents use of invalid partial signatures
/// - Ensures network validation before conditional signing
/// - Capability-based authorization for future signing
public struct UnverifiedPartialUserSignatureCap has key, store {
    /// Unique identifier for this capability
    id: UID,
    /// ID of the associated partial user signature
    partial_centralized_signed_message_id: ID,
}

/// Verified capability for a network-validated partial user signature.
/// 
/// This capability proves that:
/// - The user's partial signature has been validated by the network
/// - The signature matches the intended message and dWallet
/// - The associated presign material is valid and reserved
/// - The holder is authorized to request signature completion
/// 
/// ## Usage in Conditional Signing
/// - Can be combined with `MessageApproval` to complete signatures
/// - Enables conditional execution when multiple conditions are met
/// - Supports atomic multi-party transactions
/// 
/// ## Security Guarantees
/// - Network has verified the partial signature authenticity
/// - Presign material is reserved and cannot be double-spent
/// - Only the capability holder can trigger signature completion
public struct VerifiedPartialUserSignatureCap has key, store {
    /// Unique identifier for this capability
    id: UID,
    /// ID of the associated verified partial user signature
    partial_centralized_signed_message_id: ID,
}

/// Partial user signature for future/conditional signing scenarios.
/// 
/// Represents a message that has been signed by the user (centralized party) but not
/// yet by the network. This enables conditional signing patterns where user consent
/// is obtained first, and network signing occurs later when conditions are met.
/// 
/// ## Use Cases
/// 
/// ### Decentralized Exchange (DEX)
/// 1. User A creates a partial signature to buy BTC with ETH at price X
/// 2. User B creates a matching partial signature to sell BTC for ETH at price X
/// 3. When both conditions are met, the network completes both signatures
/// 4. Atomic swap is executed
/// 
/// ### Conditional Payments
/// - Pre-authorize payments that execute when specific conditions are met
/// - Escrow-like functionality with delayed execution
/// - Multi-party agreement protocols
/// 
/// ## Security Properties
/// - User signature proves intent and authorization
/// - Presign capability ensures single-use semantics
/// - Network verification prevents malicious signatures
/// - Capability-based access control for completion
public struct PartialUserSignature has key, store {
    /// Unique identifier for this partial signature
    id: UID,
    /// Epoch when this partial signature was created
    created_at_epoch: u64,
    /// Presign capability (consumed to prevent reuse)
    presign_cap: VerifiedPresignCap,
    /// ID of the dWallet that will complete the signature
    dwallet_id: ID,
    /// ID of the capability that controls completion
    cap_id: ID,
    /// Cryptographic curve for the signature
    curve: u32,
    /// Signature algorithm to be used
    signature_algorithm: u32,
    /// Hash scheme to apply to the message
    hash_scheme: u32,
    /// Raw message bytes to be signed
    message: vector<u8>,
    /// User's partial signature on the message
    message_centralized_signature: vector<u8>,
    /// Current verification state
    state: PartialUserSignatureState,
}

public enum PartialUserSignatureState has copy, drop, store {
    AwaitingNetworkVerification,
    NetworkVerificationCompleted,
    NetworkVerificationRejected
}

/// Represents a decentralized wallet (dWallet) created through DKG or key import.
/// 
/// A dWallet encapsulates cryptographic key material and provides secure signing
/// capabilities through Multi-Party Computation. It can operate in two security models:
/// 
/// 1. **Zero-trust mode**: User secret key share remains encrypted, requiring user
///    participation for every signature. Maximum security.
/// 2. **Trust-minimized mode**: User secret key share is made public, allowing
///    network-only signing. Reduced security but improved UX.
/// 
/// ## Security Models
/// - **DKG dWallets**: Created through distributed key generation
/// - **Imported Key dWallets**: Created from existing private keys
/// 
/// ## State Lifecycle
/// The dWallet progresses through various states from creation to active use,
/// with different paths for DKG and imported key variants.
public struct DWallet has key, store {
    /// Unique identifier for the dWallet
    id: UID,
    /// Epoch when this dWallet was created
    created_at_epoch: u64,
    /// Elliptic curve used for cryptographic operations
    curve: u32,
    /// Public user secret key share (if trust-minimized mode is enabled)
    /// 
    /// - `None`: Zero-trust mode - user participation required for signing
    /// - `Some(share)`: Trust-minimized mode - network can sign independently
    public_user_secret_key_share: Option<vector<u8>>,
    /// ID of the capability that controls this dWallet
    dwallet_cap_id: ID,
    /// Network encryption key used for securing this dWallet's network share
    dwallet_network_encryption_key_id: ID,
    /// Whether this dWallet was created from an imported key
    is_imported_key_dwallet: bool,
    /// Encrypted user secret key shares (Encryption user secret key share ID -> EncryptedUserSecretKeyShare)
    encrypted_user_secret_key_shares: ObjectTable<ID, EncryptedUserSecretKeyShare>,
    /// Signing sessions (Sign ID -> SignSession)
    sign_sessions: ObjectTable<ID, SignSession>,
    /// Current state of the dWallet
    state: DWalletState,
}

/// State of a dWallet throughout its creation and operational lifecycle.
/// 
/// dWallets can be created through two paths:
/// 1. **DKG Path**: Distributed Key Generation with validator participation
/// 2. **Import Path**: Importing existing private keys with network verification
/// 
/// Both paths converge to the `Active` state where signing operations can be performed.
public enum DWalletState has copy, drop, store {
    // === DKG Creation Path ===

    /// DKG first round has been requested from the network
    DKGRequested,
    /// Network rejected the DKG first round request
    NetworkRejectedDKGRequest,
    /// DKG first round completed, waiting for user to initiate second round
    AwaitingUserDKGVerificationInitiation {
        /// Output from the first round of DKG
        first_round_output: vector<u8>,
    },
    /// DKG second round has been requested, waiting for network verification
    AwaitingNetworkDKGVerification,
    /// Network rejected the DKG second round verification
    NetworkRejectedDKGVerification,

    // === Imported Key Creation Path ===
    
    /// Imported key dWallet created, waiting for user to initiate verification
    AwaitingUserImportedKeyInitiation,
    /// Imported key verification requested, waiting for network verification
    AwaitingNetworkImportedKeyVerification,
    /// Network rejected the imported key verification
    NetworkRejectedImportedKeyVerification,

    // === Common Completion Path ===
    
    /// DKG/Import completed, waiting for key holder to sign and accept
    AwaitingKeyHolderSignature {
        /// Public output from DKG or import verification
        public_output: vector<u8>,
    },

    /// dWallet is fully operational and ready for signing
    Active {
        /// The verified public output
        public_output: vector<u8>,
    }
}

/// Unverified capability for a presign session requiring validation.
/// 
/// This capability is issued when a presign is requested but must be verified
/// as completed before it can be used for signing operations.
/// 
/// ## Verification Process
/// 1. Check that the referenced presign session is completed
/// 2. Validate capability ID matches the session
/// 3. Convert to `VerifiedPresignCap` for actual use
/// 
/// ## Security Model
/// - Cannot be used for signing until verified
/// - Prevents use of incomplete or invalid presigns
/// - Capability-based access control
public struct UnverifiedPresignCap has key, store {
    id: UID,
    /// Target dWallet ID for dWallet-specific presigns
    /// 
    /// - `Some(id)`: Can only be used with the specified dWallet (e.g. ECDSA requirement)
    /// - `None`: Global presign, can be used with any compatible dWallet (e.g. Schnorr and EdDSA)
    dwallet_id: Option<ID>,
    /// ID of the associated presign session
    presign_id: ID,
}

/// Verified capability for a completed presign session ready for signing.
/// 
/// This capability proves that:
/// - The associated presign session has completed successfully
/// - The capability holder has authorization to use the presign
/// - The presign matches the cryptographic requirements
/// 
/// ## Usage Constraints
/// - Single-use: Consumed during signature generation
/// - Algorithm-specific: Must match the target signature algorithm
/// - Expiration: May have epoch-based validity limits
/// 
/// ## Security Properties
/// - Cryptographically bound to specific presign output
/// - Prevents double-spending of presign material
/// - Enforces proper authorization flow
public struct VerifiedPresignCap has key, store {
    id: UID,
    /// Target dWallet ID for dWallet-specific presigns
    /// 
    /// - `Some(id)`: Can only be used with the specified dWallet (e.g. ECDSA requirement)
    /// - `None`: Global presign, can be used with any compatible dWallet (e.g. Schnorr and EdDSA)
    dwallet_id: Option<ID>,
    /// ID of the associated presign session
    presign_id: ID,
}

/// Presign session for optimized signature generation.
/// 
/// Presigns are cryptographic precomputations that enable faster online signing
/// by performing expensive computations offline, before the message is known.
/// This significantly reduces signing latency in real-time applications.
/// 
/// ## Types of Presigns
/// 
/// ### dWallet-Specific Presigns
/// - Bound to a specific dWallet ID
/// - Required for algorithms like ECDSA
/// - Higher security isolation
/// 
/// ### Global Presigns
/// - Can be used with any dWallet under the same network key
/// - Supported by algorithms like Schnorr and EdDSA
/// - Better resource efficiency
/// 
/// ## Performance Benefits
/// - Reduces online full signing flow time significantly
/// - Enables high-frequency trading and real-time applications
/// - Improves user experience with instant signatures
/// 
/// ## Security Properties
/// - Single-use: Each presign can only be used once
/// - Algorithm-specific: Tailored to the signature algorithm
/// - Network-secured: Protected by threshold cryptography
public struct PresignSession has key, store {
    /// Unique identifier for this presign session
    id: UID,
    /// Epoch when this presign was created
    created_at_epoch: u64,
    /// Elliptic curve used for the presign
    curve: u32,
    /// Signature algorithm this presign supports
    signature_algorithm: u32,
    /// Target dWallet ID (None for global presigns)
    /// 
    /// - `Some(id)`: dWallet-specific presign (e.g. required for ECDSA)
    /// - `None`: Global presign (e.g. available for Schnorr, EdDSA)
    dwallet_id: Option<ID>,
    /// ID of the capability that controls this presign
    cap_id: ID,
    /// Current state of the presign computation
    state: PresignState,
}

/// State progression of a presign session through its lifecycle.
/// 
/// Presign sessions follow a linear progression from request to completion,
/// with potential rejection at the network validation stage.
public enum PresignState has copy, drop, store {
    /// Presign has been requested and is awaiting network processing
    Requested,
    /// Network rejected the presign request (invalid parameters, insufficient resources, etc.)
    NetworkRejected,
    /// Presign completed successfully with cryptographic material ready for use
    Completed {
        /// Precomputed cryptographic material for accelerated signing
        presign: vector<u8>,
    }
}

/// Signing session for generating dWallet signatures.
/// 
/// Represents an ongoing or completed signature generation process using
/// the 2PC-MPC protocol. Combines user and network contributions to create
/// a complete signature.
/// 
/// ## Signing Process
/// 1. User provides message approval and presign capability
/// 2. Network validates the request and user's partial signature
/// 3. Network combines with its share to generate the full signature
/// 4. Session transitions to completed state with the final signature
/// 
/// ## Types of Signing
/// - **Standard**: Direct signing with immediate user participation
/// - **Future**: Conditional signing using pre-validated partial signatures
/// - **Imported Key**: Signing with imported key dWallets
/// 
/// ## Performance Optimization
/// - Uses presign material to accelerate the online signing process
/// - Reduces latency from seconds to milliseconds for real-time applications
/// - Enables high-frequency trading and interactive applications
public struct SignSession has key, store {
    id: UID,
    /// Epoch when this signing session was initiated
    created_at_epoch: u64,
    /// ID of the dWallet performing the signature
    dwallet_id: ID,
    /// ID of the MPC session coordinating this signature
    session_id: ID,
    /// Current state of the signing process
    state: SignState,
}

/// State progression of a signing session through its lifecycle.
/// 
/// Signing sessions combine user authorization with network cryptographic operations
/// to produce final signatures.
public enum SignState has copy, drop, store {
    /// Signature has been requested and is awaiting network processing
    Requested,
    /// Network rejected the signature request (invalid presign, unauthorized message, etc.)
    NetworkRejected,
    /// Signature completed successfully and ready for use
    Completed {
        /// Final cryptographic signature that can be verified against the public key
        signature: vector<u8>,
    }
}

/// Type of dWallet MPC session for scheduling and epoch management.
/// 
/// User-initiated sessions have sequence numbers for multi-epoch completion scheduling.
/// System sessions are guaranteed to complete within their creation epoch.
public enum SessionType has copy, drop, store {
    /// User-initiated session with sequence number for epoch scheduling
    User {
        sequence_number: u64,
    },
    /// System-initiated session (always completes in current epoch)
    System
}

/// Authorization to sign a specific message with a dWallet.
/// 
/// This approval object grants permission to sign a message using a dWallet's
/// secret key material. It specifies the exact cryptographic parameters and
/// message content that has been authorized.
/// 
/// ## Security Properties
/// - Single-use: Consumed during signature generation to prevent replay
/// - Cryptographically bound: Specifies exact algorithm and hash scheme
/// - Message-specific: Tied to specific message content
/// - dWallet-specific: Can only be used with the designated dWallet
/// 
/// ## Usage Pattern
/// 1. User creates approval for specific message and dWallet
/// 2. Approval is combined with presign capability
/// 3. Network validates and generates signature
/// 4. Approval is consumed and cannot be reused
public struct MessageApproval has store, drop {
    /// ID of the dWallet authorized to sign this message
    dwallet_id: ID,
    /// Cryptographic signature algorithm to use
    signature_algorithm: u32,
    /// Hash scheme to apply to the message before signing
    hash_scheme: u32,
    /// Raw message bytes to be signed
    message: vector<u8>,
}

/// Authorization to sign a specific message with an imported key dWallet.
/// 
/// Similar to `MessageApproval` but specifically for dWallets created from
/// imported private keys rather than through distributed key generation.
/// 
/// ## Differences from Standard MessageApproval
/// - Used with `ImportedKeyDWalletCap` instead of `DWalletCap`
/// - May have different security assumptions due to key import process
/// - Supports the same cryptographic algorithms and operations
/// 
/// ## Security Considerations
/// - Imported key dWallets may have different trust models
/// - Users should understand the provenance of imported keys
/// - Same single-use and message-binding properties apply
public struct ImportedKeyMessageApproval has store, drop {
    /// ID of the imported key dWallet authorized to sign this message
    dwallet_id: ID,
    /// Cryptographic signature algorithm to use
    signature_algorithm: u32,
    /// Hash scheme to apply to the message before signing
    hash_scheme: u32,
    /// Raw message bytes to be signed
    message: vector<u8>,
}

// === Events ===

/// Generic wrapper for dWallet-related events with session context.
/// 
/// Provides standardized metadata for all dWallet operations including
/// epoch information, session type, and session ID for tracking and debugging.
public struct DWalletEvent<E: copy + drop + store> has copy, drop, store {
    /// Epoch when the event occurred
    epoch: u64,
    /// Type of session (User or System)
    session_type: SessionType,
    /// Unique session identifier
    session_id: ID,
    /// Event-specific data
    event_data: E,
}


// === Network Encryption Key DKG Events ===

/// Event requesting network DKG for a new encryption key.
/// 
/// Initiates the distributed key generation process for creating a new
/// network threshold encryption key used by the validator committee.
public struct DWalletNetworkDKGEncryptionKeyRequestEvent has copy, drop, store {
    /// ID of the network encryption key to be generated
    dwallet_network_encryption_key_id: ID,
}

/// Event emitted when network DKG for an encryption key completes successfully.
/// 
/// Signals that the validator network has successfully generated a new
/// threshold encryption key and it's ready for use in securing dWallet shares.
/// 
/// ## Next Steps
/// The encryption key can now be used for:
/// - Encrypting dWallet network shares
/// - Securing validator committee communications
/// - Supporting MPC protocol operations
public struct CompletedDWalletNetworkDKGEncryptionKeyEvent has copy, drop, store {
    /// ID of the successfully generated network encryption key
    dwallet_network_encryption_key_id: ID,
}

/// Event emitted when network DKG for an encryption key is rejected.
/// 
/// Indicates that the validator network could not complete the DKG process
/// for the requested encryption key, typically due to insufficient participation
/// or validation failures.
public struct RejectedDWalletNetworkDKGEncryptionKeyEvent has copy, drop, store {
    /// ID of the rejected network encryption key
    dwallet_network_encryption_key_id: ID,
}

// === Network Encryption Key Reconfiguration Events ===

/// Event requesting reconfiguration of a network encryption key.
/// 
/// Initiates the process to update a network encryption key for a new
/// validator committee, ensuring continuity of service across epoch transitions.
public struct DWalletEncryptionKeyReconfigurationRequestEvent has copy, drop, store {
    /// ID of the network encryption key to be reconfigured
    dwallet_network_encryption_key_id: ID,
}

/// Event emitted when encryption key reconfiguration completes successfully.
/// 
/// Signals that the network encryption key has been successfully updated
/// for the new validator committee and is ready for the next epoch.
public struct CompletedDWalletEncryptionKeyReconfigurationEvent has copy, drop, store {
    /// ID of the successfully reconfigured network encryption key
    dwallet_network_encryption_key_id: ID,
}

/// Event emitted when encryption key reconfiguration is rejected.
/// 
/// Indicates that the validator network could not complete the reconfiguration
/// process, potentially requiring retry or manual intervention.
public struct RejectedDWalletEncryptionKeyReconfigurationEvent has copy, drop, store {
    /// ID of the network encryption key that failed reconfiguration
    dwallet_network_encryption_key_id: ID,
}

// === DKG First Round Events ===

/// Event requesting the start of DKG first round from the validator network.
/// 
/// Initiates the distributed key generation process for a new dWallet.
/// Validators respond by executing the first round of the DKG protocol.
public struct DWalletDKGFirstRoundRequestEvent has copy, drop, store {
    /// ID of the dWallet being created
    dwallet_id: ID,
    /// ID of the capability that controls the dWallet
    dwallet_cap_id: ID,
    /// Network encryption key for securing the dWallet's network share
    dwallet_network_encryption_key_id: ID,
    /// Elliptic curve for the dWallet's cryptographic operations
    curve: u32,
}

/// Event emitted when DKG first round completes successfully.
/// 
/// Signals that the validator network has completed the first round of DKG
/// and provides the output needed for the user to proceed with the second round.
/// 
/// ## Next Steps
/// Users should:
/// 1. Process the `first_round_output`
/// 2. Generate their contribution to the DKG
/// 3. Call `request_dwallet_dkg_second_round()` to continue
public struct CompletedDWalletDKGFirstRoundEvent has copy, drop, store {
    /// ID of the dWallet being created
    dwallet_id: ID,
    /// Public output from the first round of DKG
    first_round_output: vector<u8>,
}

/// Event emitted when DKG first round is rejected by the network.
/// 
/// Indicates that the validator network could not complete the first round
/// of DKG for the requested dWallet, typically due to validation failures
/// or insufficient validator participation.
public struct RejectedDWalletDKGFirstRoundEvent has copy, drop, store {
    /// ID of the dWallet whose DKG first round was rejected
    dwallet_id: ID,
}

// === DKG Second Round Events ===

/// Event requesting the second round of DKG from the validator network.
/// 
/// This event initiates the final phase of distributed key generation where
/// the user's contribution is combined with the network's first round output
/// to complete the dWallet creation process.
/// 
/// ## Process Flow
/// 1. User processes the first round output from validators
/// 2. User generates their cryptographic contribution
/// 3. User encrypts their secret key share
/// 4. Network validates and completes the DKG process
/// 
/// ## Security Properties
/// - User contribution ensures the user controls part of the key
/// - Network validation prevents malicious key generation
/// - Encrypted shares ensure proper key distribution
public struct DWalletDKGSecondRoundRequestEvent has copy, drop, store {
    /// ID of the encrypted user secret key share being created
    encrypted_user_secret_key_share_id: ID,
    /// ID of the dWallet being created through DKG
    dwallet_id: ID,
    /// Cryptographic output from the network's first round of DKG
    first_round_output: vector<u8>,
    /// User's public key share with cryptographic proof of correctness
    centralized_public_key_share_and_proof: vector<u8>,
    /// ID of the dWallet capability that authorizes this operation
    dwallet_cap_id: ID,
    /// User's encrypted secret key share with zero-knowledge proof
    encrypted_centralized_secret_share_and_proof: vector<u8>,
    /// Serialized encryption key used to encrypt the user's secret share
    encryption_key: vector<u8>,
    /// ID of the encryption key object
    encryption_key_id: ID,
    /// Address of the encryption key owner
    encryption_key_address: address,
    /// User's contribution to the DKG public output
    user_public_output: vector<u8>,
    /// Ed25519 public key for verifying the user's signature
    signer_public_key: vector<u8>,
    /// ID of the network encryption key for securing network shares
    dwallet_network_encryption_key_id: ID,
    /// Elliptic curve for the dWallet's cryptographic operations
    curve: u32,
}

/// Event emitted when DKG second round completes successfully.
/// 
/// Signals the successful completion of the distributed key generation process.
/// The dWallet is now ready for user acceptance and can begin signing operations
/// once the user validates and accepts their encrypted key share.
/// 
/// ## Next Steps for Users
/// 1. Validate the public output matches expected values
/// 2. Decrypt and verify the received encrypted key share
/// 3. Sign the public output to accept the dWallet
/// 4. Begin using the dWallet for signing operations
/// 
/// ## Security Verification
/// Users should verify that the public key corresponds to their expected
/// contribution and that the encrypted share can be properly decrypted.
public struct CompletedDWalletDKGSecondRoundEvent has copy, drop, store {
    /// ID of the successfully created dWallet
    dwallet_id: ID,
    /// Complete public output from the DKG process (public key and metadata)
    public_output: vector<u8>,
    /// ID of the user's encrypted secret key share
    encrypted_user_secret_key_share_id: ID,
    /// ID of the MPC session that completed the DKG
    session_id: ID
}

/// Event emitted when DKG second round is rejected by the network.
/// 
/// Indicates that the validator network rejected the user's contribution
/// to the DKG process, typically due to invalid proofs or malformed data.
/// 
/// ## Common Rejection Reasons
/// - Invalid cryptographic proofs
/// - Malformed user contribution
/// - Encryption verification failures
/// - Network consensus issues
public struct RejectedDWalletDKGSecondRoundEvent has copy, drop, store {
    /// ID of the dWallet whose DKG second round was rejected
    dwallet_id: ID,
    /// Public output that was being processed when rejection occurred
    public_output: vector<u8>,
}

// === Imported Key Events ===

/// Event emitted when a new imported key dWallet is created.
/// 
/// This event signals that a new imported key dWallet has been created and is
/// ready for user acceptance and subsequent signing operations.
/// 
/// ## Next Steps for Users
/// 1. Verify the public output matches the imported key
/// 2. Validate the encrypted key share can be properly decrypted
/// 3. Sign the public output to accept the dWallet
/// 4. Begin using the imported key dWallet for signatures
public struct NewImportedKeyDWalletEvent has copy, drop {
    dwallet_id: ID,
    dwallet_cap_id: ID,
}

/// Event requesting verification of an imported key dWallet from the network.
/// 
/// This event initiates the validation process for a dWallet created from an
/// existing private key rather than through distributed key generation.
/// 
/// ## Imported Key Flow
/// 1. User creates an imported key dWallet object
/// 2. User provides cryptographic proof of key ownership
/// 3. Network validates the proof and key authenticity
/// 4. If valid, the dWallet becomes active for signing
/// 
/// ## Security Considerations
/// - Imported keys may have different security assumptions than DKG keys
/// - Network validates proof of ownership but cannot verify key generation process
/// - Users should understand the provenance and security of imported keys
public struct DWalletImportedKeyVerificationRequestEvent has copy, drop, store {
    /// ID of the imported key dWallet being verified
    dwallet_id: ID,
    /// ID of the encrypted user secret key share being created
    encrypted_user_secret_key_share_id: ID,
    /// User's cryptographic message for importing computation
    centralized_party_message: vector<u8>,
    /// ID of the imported key dWallet capability
    dwallet_cap_id: ID,
    /// User's encrypted secret key share with proof of correctness
    encrypted_centralized_secret_share_and_proof: vector<u8>,
    /// Serialized encryption key used for user share encryption
    encryption_key: vector<u8>,
    /// ID of the encryption key object
    encryption_key_id: ID,
    /// Address of the encryption key owner
    encryption_key_address: address,
    /// User's public key contribution and verification data
    user_public_output: vector<u8>,
    /// Ed25519 public key for signature verification, used to verify the user's signature on the public output
    signer_public_key: vector<u8>,
    /// ID of the network encryption key for securing network shares
    dwallet_network_encryption_key_id: ID,
    /// Elliptic curve for the imported key dWallet
    curve: u32,
}

/// Event emitted when imported key verification completes successfully.
/// 
/// Signals that the network has validated the user's imported key and the
/// dWallet is ready for user acceptance and subsequent signing operations.
/// 
/// ## Next Steps for Users
/// 1. Verify the public output matches the imported key
/// 2. Validate the encrypted key share can be properly decrypted
/// 3. Sign the public output to accept the dWallet
/// 4. Begin using the imported key dWallet for signatures
public struct CompletedDWalletImportedKeyVerificationEvent has copy, drop, store {
    /// ID of the successfully verified imported key dWallet
    dwallet_id: ID,
    /// Public output from the verification process
    public_output: vector<u8>,
    /// ID of the user's encrypted secret key share
    encrypted_user_secret_key_share_id: ID,
    /// ID of the MPC session that completed the verification
    session_id: ID
}

/// Event emitted when imported key verification is rejected by the network.
/// 
/// Indicates that the validator network could not validate the imported key,
/// typically due to invalid proofs or malformed verification data.
/// 
/// ## Common Rejection Reasons
/// - Invalid cryptographic proofs of key ownership
/// - Malformed imported key data
/// - Verification signature failures
/// - Incompatible curve parameters
public struct RejectedDWalletImportedKeyVerificationEvent has copy, drop, store {
    /// ID of the imported key dWallet that failed verification
    dwallet_id: ID,
}


// === Encrypted User Share Events ===

/// Event emitted when an encryption key is successfully created and registered.
/// 
/// This event signals that a new encryption key has been validated and is available
/// for use in encrypting user secret key shares.
public struct CreatedEncryptionKeyEvent has copy, drop, store {
    /// ID of the newly created encryption key
    encryption_key_id: ID,
    /// Address of the encryption key owner
    signer_address: address,
}

/// Event requesting verification of an encrypted user secret key share.
/// 
/// This event initiates the validation process for re-encrypted user shares,
/// typically used when transferring dWallet access to another user or when
/// creating additional encrypted copies for backup purposes.
/// 
/// ## Re-encryption Use Cases
/// - **Access Transfer**: Share dWallet access with another user
/// - **Access Granting**: Allow multiple users to control the same dWallet  
/// - **Backup Creation**: Create additional encrypted copies for redundancy
/// - **Key Recovery**: Re-encrypt shares for recovery scenarios
/// 
/// ## Verification Process
/// 1. User re-encrypts their secret key share to a new encryption key
/// 2. User provides zero-knowledge proof of correct re-encryption
/// 3. Network validates the proof against the dWallet's public output
/// 4. If valid, the new encrypted share becomes available for use
/// 
/// ## Security Properties
/// - Zero-knowledge proofs ensure re-encryption correctness
/// - Original share remains secure during the process
/// - Network cannot learn the secret key material
/// - Destination user must decrypt and validate the share
public struct EncryptedShareVerificationRequestEvent has copy, drop, store {
    /// User's encrypted secret key share with zero-knowledge proof of correctness
    encrypted_centralized_secret_share_and_proof: vector<u8>,
    /// Public output of the dWallet (used for verification), this is the 
    /// public output of the dWallet that the user's share is being encrypted to.
    /// This value is taken from the the dWallet object during event creation, and
    /// we cannot get it from the user's side.
    public_output: vector<u8>,
    /// ID of the dWallet this encrypted share belongs to
    dwallet_id: ID,
    /// Serialized encryption key used for the re-encryption
    encryption_key: vector<u8>,
    /// ID of the encryption key object
    encryption_key_id: ID,
    /// ID of the new encrypted user secret key share being created
    encrypted_user_secret_key_share_id: ID,
    /// ID of the source encrypted share (if this is a re-encryption)
    source_encrypted_user_secret_key_share_id: ID,
    /// ID of the network encryption key securing network shares
    dwallet_network_encryption_key_id: ID,
    /// Elliptic curve for the dWallet
    curve: u32,
}

/// Event emitted when encrypted share verification completes successfully.
/// 
/// Signals that the network has validated the re-encryption proof and the
/// new encrypted share is ready for the destination user to accept.
/// 
/// ## Next Steps for Recipient
/// 1. Decrypt the encrypted share using their private encryption key
/// 2. Verify the decrypted share matches the dWallet's public output
/// 3. Sign the public output to accept and activate the share
/// 4. Use the share for dWallet operations
public struct CompletedEncryptedShareVerificationEvent has copy, drop, store {
    /// ID of the successfully verified encrypted user secret key share
    encrypted_user_secret_key_share_id: ID,
    /// ID of the dWallet associated with this encrypted share
    dwallet_id: ID,
}

/// Event emitted when encrypted share verification is rejected.
/// 
/// Indicates that the network could not validate the re-encryption proof,
/// typically due to invalid cryptographic proofs or verification failures.
/// 
/// ## Common Rejection Reasons
/// - Invalid zero-knowledge proof of re-encryption
/// - Mismatch between encrypted share and public output
/// - Corrupted or malformed encryption data
/// - Incompatible encryption key parameters
public struct RejectedEncryptedShareVerificationEvent has copy, drop, store {
    /// ID of the encrypted user secret key share that failed verification
    encrypted_user_secret_key_share_id: ID,
    /// ID of the dWallet associated with the failed share
    dwallet_id: ID,
}

/// Event emitted when a user accepts an encrypted secret key share.
/// 
/// This event signals the final step in the share transfer process where
/// the recipient has validated and accepted their encrypted share, making
/// the dWallet fully accessible to them.
/// 
/// ## Acceptance Process
/// 1. User decrypts the share with their private encryption key
/// 2. User verifies the share produces the correct public key
/// 3. User signs the public output to prove acceptance
/// 4. Share becomes active and usable for signing operations
/// 
/// ## Security Verification
/// The user's signature on the public output serves as cryptographic proof that:
/// - They successfully decrypted the share
/// - The share is mathematically correct
/// - They accept responsibility for the dWallet
public struct AcceptEncryptedUserShareEvent has copy, drop, store {
    /// ID of the accepted encrypted user secret key share
    encrypted_user_secret_key_share_id: ID,
    /// ID of the dWallet associated with this share
    dwallet_id: ID,
    /// User's signature on the public output proving acceptance
    user_output_signature: vector<u8>,
    /// ID of the encryption key used for this share
    encryption_key_id: ID,
    /// Address of the user who accepted the share
    encryption_key_address: address,
}

// === Make User Secret Key Share Public Events ===

/// Event requesting to make a dWallet's user secret key share public.
/// 
/// This event initiates the transition from zero-trust mode to trust-minimized mode,
/// where the user's secret key share becomes publicly visible, allowing the network
/// to sign independently without user participation.
/// 
/// ## ⚠️ CRITICAL SECURITY WARNING
/// **This operation is IRREVERSIBLE and reduces security!**
/// 
/// ### Security Trade-offs
/// - **Before**: Zero-trust - user participation required for every signature
/// - **After**: Trust-minimized - network can sign independently
/// - **Risk**: Compromised validators could potentially misuse the dWallet
/// 
/// ### When to Consider This
/// - High-frequency automated trading where latency is critical
/// - Applications requiring instant signature generation
/// - When convenience outweighs the security reduction
/// - Smart contract automation that needs independent signing
/// 
/// ### Use Cases
/// - DeFi protocols with automated rebalancing
/// - Gaming applications with instant transactions
/// - IoT devices requiring autonomous signing
/// - Bot trading with microsecond latency requirements
public struct MakeDWalletUserSecretKeySharePublicRequestEvent has copy, drop, store {
    /// The user's secret key share to be made public
    public_user_secret_key_share: vector<u8>,
    /// dWallet's public output for verification
    public_output: vector<u8>,
    /// Elliptic curve for the dWallet
    curve: u32,
    /// ID of the dWallet being transitioned to trust-minimized mode
    dwallet_id: ID,
    /// ID of the network encryption key
    dwallet_network_encryption_key_id: ID,
}

/// Event emitted when user secret key share is successfully made public.
/// 
/// Signals that the dWallet has transitioned to trust-minimized mode where
/// the network can now sign independently without user participation.
/// 
/// ## Post-Transition Capabilities
/// - Network can generate signatures autonomously
/// - Reduced latency for signing operations
/// - No user interaction required for each signature
/// - Suitable for high-frequency automated applications
/// 
/// ## ⚠️ Security Reminder
/// The dWallet now operates in trust-minimized mode. Monitor validator
/// behavior and consider the implications for your security model.
public struct CompletedMakeDWalletUserSecretKeySharePublicEvent has copy, drop, store {
    /// ID of the dWallet that successfully transitioned to trust-minimized mode
    dwallet_id: ID,
}

/// Event emitted when the request to make user secret key share public is rejected.
/// 
/// Indicates that the network could not validate or complete the transition
/// to trust-minimized mode.
/// 
/// ## Common Rejection Reasons
/// - Invalid user secret key share provided
/// - Mismatch between share and public output
/// - dWallet already in trust-minimized mode
/// - Network validation failures
public struct RejectedMakeDWalletUserSecretKeySharePublicEvent has copy, drop, store {
    /// ID of the dWallet that failed to transition to trust-minimized mode
    dwallet_id: ID,
}

// === Presign Events ===

/// Event requesting the generation of a presign from the validator network.
/// 
/// This event initiates the precomputation of cryptographic material that will
/// be used to accelerate future signature generation. Presigns are a key
/// optimization in the 2PC-MPC protocol, reducing online signing time by 80-90%.
/// 
/// ## Presign Types
/// 
/// ### dWallet-Specific Presigns
/// - Required for algorithms like ECDSA that need key-specific precomputation
/// - Bound to a specific dWallet and cannot be used elsewhere
/// - Higher security isolation but less resource efficiency
/// 
/// ### Global Presigns  
/// - Supported by algorithms like Schnorr and EdDSA
/// - Can be used with any compatible dWallet under the same network key
/// - Better resource utilization and batching efficiency
/// 
/// ## Performance Benefits
/// - **Latency Reduction**: From seconds to milliseconds for signing
/// - **Throughput Increase**: Enables high-frequency trading applications
/// - **User Experience**: Near-instant signature generation
/// - **Scalability**: Batch presign generation during low activity periods
public struct PresignRequestEvent has copy, drop, store {
    /// Target dWallet ID for dWallet-specific presigns
    /// 
    /// - `Some(id)`: dWallet-specific presign (required for ECDSA)
    /// - `None`: Global presign (available for Schnorr, EdDSA)
    dwallet_id: Option<ID>,
    /// Unique identifier for this presign session
    presign_id: ID,
    /// dWallet's public output for verification (None for global presigns)
    dwallet_public_output: Option<vector<u8>>,
    /// ID of the network encryption key securing the presign
    dwallet_network_encryption_key_id: ID,
    /// Elliptic curve for the presign computation
    curve: u32,
    /// Signature algorithm for the presign (determines presign type)
    signature_algorithm: u32,
}

/// Event emitted when a presign generation completes successfully.
/// 
/// Signals that the validator network has successfully generated the
/// cryptographic precomputation material and it's ready for use in
/// accelerated signature generation.
/// 
/// ## Next Steps
/// 1. User receives a `VerifiedPresignCap` capability
/// 2. Presign can be combined with message approval for fast signing
/// 3. Single-use: Each presign can only be used once
/// 4. Expiration: Presigns may have validity time limits
/// 
/// ## Security Properties
/// - Cryptographically bound to specific algorithm and curve
/// - Cannot be used for different signature types
/// - Single-use prevents double-spending of presign material
/// - Network validation ensures correctness
public struct CompletedPresignEvent has copy, drop, store {
    /// Target dWallet ID (None for global presigns)
    dwallet_id: Option<ID>,
    /// ID of the MPC session that generated the presign
    session_id: ID,
    /// Unique identifier for the completed presign
    presign_id: ID,
    /// Precomputed cryptographic material for signature acceleration
    presign: vector<u8>,
}

/// Event emitted when presign generation is rejected by the network.
/// 
/// Indicates that the validator network could not complete the presign
/// generation, typically due to validation failures or resource constraints.
/// 
/// ## Common Rejection Reasons
/// - Insufficient validator participation
/// - Invalid cryptographic parameters
/// - Network resource constraints
/// - Validation failures during precomputation
/// - Incompatible algorithm/curve combinations
public struct RejectedPresignEvent has copy, drop, store {
    /// Target dWallet ID (None for global presigns)
    dwallet_id: Option<ID>,
    /// ID of the MPC session that failed
    session_id: ID,
    /// ID of the presign that failed generation
    presign_id: ID
}

// === Sign Events ===

/// Event requesting signature generation from the validator network.
/// 
/// This event initiates the final phase of the 2PC-MPC signing protocol where
/// the network combines user authorization with precomputed material to generate
/// a complete cryptographic signature.
/// 
/// ## Signing Process Flow
/// 1. User provides message approval and presign capability
/// 2. Network validates the user's authorization
/// 3. Network combines presign with user's partial signature
/// 4. Complete signature is generated and returned
/// 
/// ## Signature Types
/// 
/// ### Standard Signing (`is_future_sign: false`)
/// - Immediate user participation required
/// - User signature computed in real-time
/// - Highest security with fresh user authorization
/// 
/// ### Future Signing (`is_future_sign: true`)
/// - Uses pre-validated partial user signatures
/// - Enables conditional and delayed execution
/// - Supports complex multi-party transaction patterns
/// 
/// ## Performance Optimization
/// - Presign material enables sub-second signature generation
/// - Critical for high-frequency trading and real-time applications
/// - Reduces network round-trips and computational overhead
public struct SignRequestEvent has copy, drop, store {
    /// Unique identifier for this signing session
    sign_id: ID,
    /// ID of the dWallet performing the signature
    dwallet_id: ID,
    /// dWallet's public output for signature verification
    dwallet_public_output: vector<u8>,
    /// Elliptic curve for the signature
    curve: u32,
    /// Cryptographic signature algorithm
    signature_algorithm: u32,
    /// Hash scheme applied to the message
    hash_scheme: u32,
    /// Raw message bytes to be signed
    message: vector<u8>,
    /// ID of the network encryption key securing network shares
    dwallet_network_encryption_key_id: ID,
    /// ID of the presign used for acceleration
    presign_id: ID,
    /// Precomputed cryptographic material for fast signing
    presign: vector<u8>,
    /// User's partial signature on the message
    message_centralized_signature: vector<u8>,
    /// Whether this uses future sign capabilities
    is_future_sign: bool,
}

/// Event emitted when signature generation completes successfully.
/// 
/// This event signals the successful completion of the 2PC-MPC signing protocol
/// and provides the final cryptographic signature that can be used in transactions.
/// 
/// ## Signature Properties
/// - **Mathematically Valid**: Verifiable against the dWallet's public key
/// - **Cryptographically Secure**: Generated using threshold cryptography
/// - **Single-Use Presign**: Associated presign material is consumed
/// - **User Authorized**: Includes validated user consent
/// 
/// ## Next Steps
/// 1. Extract the signature from the event
/// 2. Combine with transaction data for blockchain submission
/// 3. Verify signature matches expected format for target blockchain
/// 4. Submit transaction to the destination network
/// 
/// ## Performance Metrics
/// With presigns, signature generation typically completes in:
/// - **Standard Networks**: 100-500ms
/// - **High-Performance Setup**: 50-100ms
/// - **Without Presigns**: 2-5 seconds
public struct CompletedSignEvent has copy, drop, store {
    /// Unique identifier for the completed signing session
    sign_id: ID,
    /// ID of the MPC session that generated the signature
    session_id: ID,
    /// Complete cryptographic signature ready for use
    signature: vector<u8>,
    /// Whether this signature used future sign capabilities
    is_future_sign: bool,
}

/// Event emitted when signature generation is rejected by the network.
/// 
/// Indicates that the validator network could not complete the signature
/// generation, typically due to validation failures or protocol errors.
/// 
/// ## Common Rejection Reasons
/// - **Invalid Presign**: Presign material is corrupted or expired
/// - **Authorization Failure**: User signature validation failed
/// - **Network Issues**: Insufficient validator participation
/// - **Protocol Errors**: Cryptographic validation failures
/// - **Resource Constraints**: Network overload or rate limiting
/// 
/// ## Recovery Steps
/// 1. Check presign validity and obtain new presign if needed
/// 2. Verify message approval is correctly formatted
/// 3. Ensure dWallet is in active state
/// 4. Retry with fresh authorization if temporary failure
public struct RejectedSignEvent has copy, drop, store {
    /// ID of the signing session that failed
    sign_id: ID,
    /// ID of the MPC session that rejected the request
    session_id: ID,
    /// Whether this rejection involved future sign capabilities
    is_future_sign: bool,
}

// === Future Sign Events ===

/// Event requesting validation of a partial user signature for future signing.
/// 
/// This event initiates the creation of a conditional signature capability where
/// the user's authorization is validated upfront but the network signature is
/// deferred until specific conditions are met.
/// 
/// ## Future Sign Use Cases
/// 
/// ### Decentralized Exchange (DEX) Orders
/// ```
/// 1. User A: "I'll sell 1 BTC for 50,000 USDC"
/// 2. User B: "I'll buy 1 BTC for 50,000 USDC"  
/// 3. When both conditions match → automatic execution
/// ```
/// 
/// ### Conditional Payments
/// ```
/// 1. User: "Pay 1000 USDC to Alice when she delivers the goods"
/// 2. Oracle confirms delivery → automatic payment
/// ```
/// 
/// ### Multi-Party Atomic Swaps
/// ```
/// 1. Multiple users create conditional signatures
/// 2. When all conditions are met → atomic execution
/// ```
/// 
/// ## Security Benefits
/// - User authorization is cryptographically committed upfront
/// - Network validation prevents invalid partial signatures
/// - Conditions can be verified before execution
/// - Atomic execution reduces counterparty risk
public struct FutureSignRequestEvent has copy, drop, store {
    /// ID of the dWallet that will complete the future signature
    dwallet_id: ID,
    /// ID of the partial user signature being validated
    partial_centralized_signed_message_id: ID,
    /// Message that will be signed when conditions are met
    message: vector<u8>,
    /// Precomputed cryptographic material for the future signature
    presign: vector<u8>,
    /// dWallet's public output for verification
    dwallet_public_output: vector<u8>,
    /// Elliptic curve for the signature
    curve: u32,
    /// Signature algorithm for the future signature
    signature_algorithm: u32,
    /// Hash scheme to be applied to the message
    hash_scheme: u32,
    /// User's partial signature proving authorization
    message_centralized_signature: vector<u8>,
    /// ID of the network encryption key
    dwallet_network_encryption_key_id: ID,
}

/// Event emitted when future sign validation completes successfully.
/// 
/// Signals that the network has validated the user's partial signature and
/// the future sign capability is ready for conditional execution.
/// 
/// ## Next Steps
/// 1. User receives a `VerifiedPartialUserSignatureCap`
/// 2. Capability can be combined with `MessageApproval` for execution
/// 3. Network will complete the signature when both are presented
/// 4. Enables complex conditional signing workflows
public struct CompletedFutureSignEvent has copy, drop, store {
    /// ID of the MPC session that validated the partial signature
    session_id: ID,
    /// ID of the dWallet associated with the future signature
    dwallet_id: ID,
    /// ID of the validated partial user signature
    partial_centralized_signed_message_id: ID,
}

/// Event emitted when future sign validation is rejected.
/// 
/// Indicates that the network could not validate the user's partial signature,
/// preventing the creation of the conditional signing capability.
/// 
/// ## Common Rejection Reasons
/// - Invalid user partial signature
/// - Mismatch between signature and message
/// - Incompatible presign material
/// - dWallet validation failures
public struct RejectedFutureSignEvent has copy, drop, store {
    /// ID of the MPC session that failed validation
    session_id: ID,
    /// ID of the dWallet associated with the failed request
    dwallet_id: ID,
    /// ID of the partial user signature that failed validation
    partial_centralized_signed_message_id: ID,
}

// === Operational Events ===


/// Event containing dwallet 2pc-mpc checkpoint information, emitted during
/// the checkpoint submission message.
public struct DWalletCheckpointInfoEvent has copy, drop, store {
    epoch: u64,
    sequence_number: u64,
    timestamp_ms: u64,
}

/// Event requesting to set the maximum number of active sessions buffer.
/// 
/// This event is used to configure the maximum number of active sessions that
/// can be created at any given time. This is used to prevent the network from
/// creating too many sessions and causing the validators to become out of sync.
public struct SetMaxActiveSessionsBufferEvent has copy, drop {
    max_active_sessions_buffer: u64,
}

/// Event requesting to set the gas fee reimbursement SUI system call value.
/// 
/// This event is used to configure the gas fee reimbursement SUI system call value.
public struct SetGasFeeReimbursementSuiSystemCallValueEvent has copy, drop {
    gas_fee_reimbursement_sui_system_call_value: u64,
}

// === Package Functions ===

/// Creates a new DWalletCoordinatorInner instance with initial configuration.
/// 
/// Validates that pricing exists for all supported protocols and curves before creation.
/// Initializes all internal data structures with default values.
/// 
/// ### Parameters
/// - `current_epoch`: Starting epoch number
/// - `active_committee`: Initial validator committee
/// - `pricing`: Default pricing configuration
/// - `supported_curves_to_signature_algorithms_to_hash_schemes`: Supported cryptographic configurations
/// - `ctx`: Transaction context for object creation
/// 
/// ### Returns
/// A new DWalletCoordinatorInner instance ready for use
public(package) fun create_dwallet_coordinator_inner(
    current_epoch: u64,
    active_committee: BlsCommittee,
    pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap<u32, VecMap<u32, vector<u32>>>,
    ctx: &mut TxContext
): DWalletCoordinatorInner {
    verify_pricing_exists_for_all_protocols(&supported_curves_to_signature_algorithms_to_hash_schemes, &pricing);
    DWalletCoordinatorInner {
        current_epoch,
        session_management: SessionManagement {
            sessions: object_table::new(ctx),
            user_requested_sessions_events: bag::new(ctx),
            number_of_completed_user_initiated_sessions: 0,
            started_system_sessions_count: 0,
            completed_system_sessions_count: 0,
            next_session_sequence_number: 1,
            last_user_initiated_session_to_complete_in_current_epoch: 0,
            locked_last_user_initiated_session_to_complete_in_current_epoch: true,
            max_active_sessions_buffer: 100,
        },
        dwallets: object_table::new(ctx),
        dwallet_network_encryption_keys: object_table::new(ctx),
        encryption_keys: object_table::new(ctx),
        presign_sessions: object_table::new(ctx),
        partial_centralized_signed_messages: object_table::new(ctx),
        pricing_and_fee_management: PricingAndFeeManagement {
            current: pricing,
            default: pricing,
            validator_votes: table::new(ctx),
            calculation_votes: option::none(),
            gas_fee_reimbursement_sui_system_call_value: 0,
            gas_fee_reimbursement_sui: balance::zero(),
            consensus_validation_fee_charged_ika: balance::zero(),
        },
        active_committee,
        previous_committee: bls_committee::empty(),
        total_messages_processed: 0,
        last_processed_checkpoint_sequence_number: option::none(),
        previous_epoch_last_checkpoint_sequence_number: 0,
        support_config: SupportConfig {
            supported_curves_to_signature_algorithms_to_hash_schemes,
            paused_curves: vector[],
            paused_signature_algorithms: vector[],
            paused_hash_schemes: vector[],
            signature_algorithms_allowed_global_presign: vector[],
        },
        extra_fields: bag::new(ctx),
    }
}

/// Locks the last active session sequence number to prevent further updates.
/// 
/// This function is called before epoch transitions to ensure session scheduling
/// stability during the epoch switch process.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// 
/// ### Effects
/// - Prevents further updates to `last_user_initiated_session_to_complete_in_current_epoch`
/// - Ensures session completion targets remain stable during epoch transitions
public(package) fun lock_last_active_session_sequence_number(self: &mut DWalletCoordinatorInner) {
    self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch = true;
}

/// Starts a Distributed Key Generation (DKG) session for the network (threshold) encryption key.
/// 
/// Creates a new network encryption key and initiates the DKG process through the validator network.
/// Returns a capability that grants control over the created encryption key.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// - `ctx`: Transaction context for object creation
/// 
/// ### Returns
/// A capability granting control over the new network encryption key
public(package) fun request_dwallet_network_encryption_key_dkg(
    self: &mut DWalletCoordinatorInner,
    ctx: &mut TxContext
): DWalletNetworkEncryptionKeyCap {
    // Create a new capability to control this encryption key.
    let id = object::new(ctx);
    let dwallet_network_encryption_key_id = id.to_inner();
    let cap = DWalletNetworkEncryptionKeyCap {
        id: object::new(ctx),
        dwallet_network_encryption_key_id,
    };

    // Create a new network encryption key and add it to the shared state.
    self.dwallet_network_encryption_keys.add(dwallet_network_encryption_key_id, DWalletNetworkEncryptionKey {
        id,
        dwallet_network_encryption_key_cap_id: object::id(&cap),
        current_epoch: self.current_epoch,
        reconfiguration_public_outputs: sui::table::new(ctx),
        network_dkg_public_output: table_vec::empty(ctx),
        computation_fee_charged_ika: balance::zero(),
        state: DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG,
    });

    self.initiate_system_dwallet_session(
        DWalletNetworkDKGEncryptionKeyRequestEvent {
            dwallet_network_encryption_key_id
        },
        ctx,
    );

    // Return the capability.
    cap
}

/// Charges gas fee reimbursement for system-initiated operations.
/// 
/// Allocates SUI from the coordinator's gas reimbursement pool to cover
/// transaction costs for system operations like network DKG and reconfiguration.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// 
/// ### Returns
/// SUI balance to reimburse gas costs for system operations
/// 
/// ### Logic
/// - Returns zero if no reimbursement funds or value configured
/// - Takes the minimum of available funds and configured system call value
/// - Ensures system operations don't exhaust the entire reimbursement pool
fun charge_gas_fee_reimbursement_sui_for_system_calls(
    self: &mut DWalletCoordinatorInner,
): Balance<SUI> {
    let gas_fee_reimbursement_sui_value = self.pricing_and_fee_management.gas_fee_reimbursement_sui.value();
    let gas_fee_reimbursement_sui_system_call_value = self.pricing_and_fee_management.gas_fee_reimbursement_sui_system_call_value;
    if(gas_fee_reimbursement_sui_value > 0 && gas_fee_reimbursement_sui_system_call_value > 0) {
        if(gas_fee_reimbursement_sui_value > gas_fee_reimbursement_sui_system_call_value) {
            self.pricing_and_fee_management.gas_fee_reimbursement_sui.split(gas_fee_reimbursement_sui_system_call_value)
        } else {
            self.pricing_and_fee_management.gas_fee_reimbursement_sui.split(gas_fee_reimbursement_sui_value)
        }
    } else {
        balance::zero()
    }
}

/// Complete the Distributed Key Generation (DKG) session
/// and store the public output corresponding to the newly created network (threshold) encryption key.
///
/// Note: assumes the public output is divided into chunks and each `network_public_output_chunk` is delivered in order,
/// with `is_last_chunk` set for the last call.
public(package) fun respond_dwallet_network_encryption_key_dkg(
    self: &mut DWalletCoordinatorInner,
    dwallet_network_encryption_key_id: ID,
    network_public_output_chunk: vector<u8>,
    is_last_chunk: bool,
    rejected: bool,
    ctx: &mut TxContext,
): Balance<SUI> {
    if (is_last_chunk) {
        self.session_management.completed_system_sessions_count = self.session_management.completed_system_sessions_count + 1;
    };
    let dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(
        dwallet_network_encryption_key_id
    );
    if (rejected) {
        dwallet_network_encryption_key.state = DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG;
        // TODO(@scaly): should we empty dwallet_network_encryption_key.network_dkg_public_output?
        event::emit(RejectedDWalletNetworkDKGEncryptionKeyEvent {
            dwallet_network_encryption_key_id,
        });
        self.initiate_system_dwallet_session(
            DWalletNetworkDKGEncryptionKeyRequestEvent {
                dwallet_network_encryption_key_id,
            },
            ctx,
        );
    } else {
        dwallet_network_encryption_key.network_dkg_public_output.push_back(network_public_output_chunk);
        dwallet_network_encryption_key.state = match (&dwallet_network_encryption_key.state) {
            DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG => {
            if (is_last_chunk) {
                event::emit(CompletedDWalletNetworkDKGEncryptionKeyEvent {
                    dwallet_network_encryption_key_id,
                });
                DWalletNetworkEncryptionKeyState::NetworkDKGCompleted
            } else {
                DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG
            }
        },
            _ => abort EWrongState
        };
    };
    self.charge_gas_fee_reimbursement_sui_for_system_calls()
}

/// Complete the Reconfiguration session
/// and store the public output corresponding to the reconfigured network (threshold) encryption key.
///
/// Note: assumes the public output is divided into chunks and each `network_public_output_chunk` is delivered in order,
/// with `is_last_chunk` set for the last call.
public(package) fun respond_dwallet_network_encryption_key_reconfiguration(
    self: &mut DWalletCoordinatorInner,
    dwallet_network_encryption_key_id: ID,
    public_output: vector<u8>,
    is_last_chunk: bool,
    rejected: bool,
    ctx: &mut TxContext,
): Balance<SUI> {
    // The Reconfiguration output can be large, so it is seperated into chunks.
    // We should only update the count once, so we check it is the last chunk before we do.
    if (is_last_chunk) {
        self.session_management.completed_system_sessions_count = self.session_management.completed_system_sessions_count + 1;
    };

    // Store this chunk as the last chunk in the chunks vector corresponding to the upcoming's epoch in the public outputs map.
    let dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(dwallet_network_encryption_key_id);
    if (rejected) {
        dwallet_network_encryption_key.state = match (&dwallet_network_encryption_key.state) {
            DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first } => {
                DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first: *is_first }
            },
            _ => DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first: false }
        };
        // TODO(@scaly): should we empty next_reconfiguration_public_output?
        event::emit(RejectedDWalletEncryptionKeyReconfigurationEvent {
            dwallet_network_encryption_key_id,
        });
        self.initiate_system_dwallet_session(
            DWalletEncryptionKeyReconfigurationRequestEvent {
                dwallet_network_encryption_key_id,
            },
            ctx,
        );
    } else {
        let next_reconfiguration_public_output = dwallet_network_encryption_key.reconfiguration_public_outputs.borrow_mut(dwallet_network_encryption_key.current_epoch + 1);
        // Change state to complete and emit an event to signify that only if it is the last chunk.
        next_reconfiguration_public_output.push_back(public_output);
        dwallet_network_encryption_key.state = match (&dwallet_network_encryption_key.state) {
            DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first } => {
                if (is_last_chunk) {
                        event::emit(CompletedDWalletEncryptionKeyReconfigurationEvent {
                            dwallet_network_encryption_key_id,
                        });
                        DWalletNetworkEncryptionKeyState::AwaitingNextEpochToUpdateReconfiguration
                    } else {
                        DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first: *is_first }
                    }
                },
            _ => abort EWrongState
        };
    };
    self.charge_gas_fee_reimbursement_sui_for_system_calls()
}

/// Advance the `current_epoch` and `state` of the network encryption key corresponding to `cap`,
/// finalizing the reconfiguration of that key, and readying it for use in the next epoch.
fun advance_epoch_dwallet_network_encryption_key(
    self: &mut DWalletCoordinatorInner,
    cap: &DWalletNetworkEncryptionKeyCap,
): Balance<IKA> {
    // Get the corresponding network encryption key.
    let dwallet_network_encryption_key = self.get_active_dwallet_network_encryption_key(
        cap.dwallet_network_encryption_key_id
    );

    // Sanity checks: check the capability is the right one, and that the key is in the right state.
    assert!(dwallet_network_encryption_key.dwallet_network_encryption_key_cap_id == cap.id.to_inner(), EIncorrectCap);
    assert!(dwallet_network_encryption_key.state == DWalletNetworkEncryptionKeyState::AwaitingNextEpochToUpdateReconfiguration, EWrongState);

    // Advance the current epoch and state.
    dwallet_network_encryption_key.current_epoch = dwallet_network_encryption_key.current_epoch + 1;
    dwallet_network_encryption_key.state = DWalletNetworkEncryptionKeyState::NetworkReconfigurationCompleted;

    // Return the fees.
    let mut epoch_computation_fee_charged_ika = sui::balance::zero<IKA>();
    epoch_computation_fee_charged_ika.join(dwallet_network_encryption_key.computation_fee_charged_ika.withdraw_all());
    return epoch_computation_fee_charged_ika
}

public(package) fun mid_epoch_reconfiguration(
    self: &mut DWalletCoordinatorInner,
    next_epoch_active_committee: BlsCommittee,
    dwallet_network_encryption_key_caps: &vector<DWalletNetworkEncryptionKeyCap>,
    ctx: &mut TxContext,
) {
    let pricing_calculation_votes = dwallet_pricing::new_pricing_calculation(next_epoch_active_committee, self.pricing_and_fee_management.default);
    self.pricing_and_fee_management.calculation_votes = option::some(pricing_calculation_votes);
    dwallet_network_encryption_key_caps.do_ref!(|cap| self.emit_start_reconfiguration_event(cap, ctx));
}

public(package) fun calculate_pricing_votes(
    self: &mut DWalletCoordinatorInner,
    curve: u32,
    signature_algorithm: Option<u32>,
    protocol: u32,
) {
    let pricing_and_fee_management = &mut self.pricing_and_fee_management;
    assert!(pricing_and_fee_management.calculation_votes.is_some(), EPricingCalculationVotesHasNotBeenStarted);
    let pricing_calculation_votes = pricing_and_fee_management.calculation_votes.borrow_mut();
    let pricing_votes = pricing_calculation_votes.committee_members_for_pricing_calculation_votes().map!(|id| {
        if (pricing_and_fee_management.validator_votes.contains(id)) {
            pricing_and_fee_management.validator_votes[id]
        } else {
            pricing_and_fee_management.default
        }
    });
    pricing_calculation_votes.calculate_pricing_quorum_below(pricing_votes, curve, signature_algorithm, protocol);
    if(pricing_calculation_votes.is_calculation_completed()) {
        pricing_and_fee_management.current = pricing_calculation_votes.calculated_pricing();
        pricing_and_fee_management.calculation_votes = option::none();
    }
}

/// Emit an event to the Ika network to request a reconfiguration session for the network encryption key corresponding to `cap`.
fun emit_start_reconfiguration_event(
    self: &mut DWalletCoordinatorInner,
    cap: &DWalletNetworkEncryptionKeyCap,
    ctx: &mut TxContext
) {
    assert!(self.dwallet_network_encryption_keys.contains(cap.dwallet_network_encryption_key_id), EDWalletNetworkEncryptionKeyNotExist);

    let dwallet_network_encryption_key = self.get_active_dwallet_network_encryption_key(cap.dwallet_network_encryption_key_id);

    dwallet_network_encryption_key.state = match (&dwallet_network_encryption_key.state) {
        DWalletNetworkEncryptionKeyState::NetworkDKGCompleted => {
            DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first: true }
        },
        DWalletNetworkEncryptionKeyState::NetworkReconfigurationCompleted => {
            DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first: false }
        },
        _ => return, // TODO(@scaly): should not happen, what do you think?
    };

    // Initialize the chunks vector corresponding to the upcoming's epoch in the public outputs map.
    dwallet_network_encryption_key.reconfiguration_public_outputs.add(dwallet_network_encryption_key.current_epoch + 1, table_vec::empty(ctx));

    self.initiate_system_dwallet_session(
        DWalletEncryptionKeyReconfigurationRequestEvent {
            dwallet_network_encryption_key_id: cap.dwallet_network_encryption_key_id
        },
        ctx,
    );
}

fun get_active_dwallet_network_encryption_key(
    self: &mut DWalletCoordinatorInner,
    dwallet_network_encryption_key_id: ID,
): &mut DWalletNetworkEncryptionKey {
    let dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(dwallet_network_encryption_key_id);

    assert!(dwallet_network_encryption_key.state != DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG, EDWalletNetworkEncryptionKeyNotActive);

    dwallet_network_encryption_key
}

/// Advances the coordinator to the next epoch with comprehensive state transitions.
/// 
/// Performs a complete epoch transition including session management updates,
/// committee transitions, and network encryption key advancement. This is a
/// critical operation that must be executed atomically.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// - `next_committee`: New validator committee for the upcoming epoch
/// - `dwallet_network_encryption_key_caps`: Capabilities for network encryption keys to advance
/// 
/// ### Returns
/// Combined IKA balance from fees collected during the epoch
/// 
/// ### Effects
/// - Validates all current epoch sessions are completed
/// - Updates session management metadata for the next epoch
/// - Transitions validator committees (current -> previous, next -> current)
/// - Advances network encryption key epochs
/// - Unlocks session sequence number management
/// - Increments the current epoch counter
/// - Collects and returns accumulated fees
/// 
/// ### Aborts
/// - `EPricingCalculationVotesMustBeCompleted`: If pricing votes are still in progress
/// - `ECannotAdvanceEpoch`: If not all current epoch sessions are completed
/// - Various network encryption key related errors from capability validation
public(package) fun advance_epoch(
    self: &mut DWalletCoordinatorInner,
    next_committee: BlsCommittee,
    dwallet_network_encryption_key_caps: &vector<DWalletNetworkEncryptionKeyCap>,
): Balance<IKA> {
    assert!(self.pricing_and_fee_management.calculation_votes.is_none(), EPricingCalculationVotesMustBeCompleted);
    assert!(self.all_current_epoch_sessions_completed(), ECannotAdvanceEpoch);

    if (self.last_processed_checkpoint_sequence_number.is_some()) {
        let last_processed_checkpoint_sequence_number = *self.last_processed_checkpoint_sequence_number.borrow();
        self.previous_epoch_last_checkpoint_sequence_number = last_processed_checkpoint_sequence_number;
    };

    self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch = false;
    self.update_last_user_initiated_session_to_complete_in_current_epoch();

    self.current_epoch = self.current_epoch + 1;

    self.previous_committee = self.active_committee;
    self.active_committee = next_committee;

    let mut balance = balance::zero<IKA>();
    dwallet_network_encryption_key_caps.do_ref!(|cap| {
        balance.join(self.advance_epoch_dwallet_network_encryption_key(cap));
    });
    balance.join(self.pricing_and_fee_management.consensus_validation_fee_charged_ika.withdraw_all());
    balance
}

/// Gets an immutable reference to a dWallet by ID.
/// 
/// ### Parameters
/// - `self`: Reference to the coordinator
/// - `dwallet_id`: ID of the dWallet to retrieve
/// 
/// ### Returns
/// Immutable reference to the dWallet
/// 
/// ### Aborts
/// - `EDWalletNotExists`: If the dWallet doesn't exist
fun get_dwallet(
    self: &DWalletCoordinatorInner,
    dwallet_id: ID,
): &DWallet {
    assert!(self.dwallets.contains(dwallet_id), EDWalletNotExists);
    self.dwallets.borrow(dwallet_id)
}

/// Gets a mutable reference to a dWallet by ID.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// - `dwallet_id`: ID of the dWallet to retrieve
/// 
/// ### Returns
/// Mutable reference to the dWallet
/// 
/// ### Aborts
/// - `EDWalletNotExists`: If the dWallet doesn't exist
fun get_dwallet_mut(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
): &mut DWallet {
    assert!(self.dwallets.contains(dwallet_id), EDWalletNotExists);
    self.dwallets.borrow_mut(dwallet_id)
}

/// Validates that a dWallet is in active state and returns its public output.
/// 
/// This function ensures that a dWallet has completed its creation process
/// (either DKG or imported key verification) and is ready for cryptographic
/// operations like signing.
/// 
/// ### Parameters
/// - `self`: Reference to the dWallet to validate
/// 
/// ### Returns
/// Reference to the dWallet's public output
/// 
/// ### Aborts
/// - `EDWalletInactive`: If the dWallet is not in the `Active` state
/// 
/// ### Active State Requirements
/// A dWallet is considered active when:
/// - DKG process has completed successfully, OR
/// - Imported key verification has completed successfully
/// - User has accepted their encrypted key share
/// - Public output is available for cryptographic operations
fun validate_active_and_get_public_output(
    self: &DWallet,
): &vector<u8> {
    match (&self.state) {
        DWalletState::Active {
            public_output,
        } => {
            public_output
        },
        DWalletState::DKGRequested |
        DWalletState::NetworkRejectedDKGRequest |
        DWalletState::AwaitingUserDKGVerificationInitiation { .. } |
        DWalletState::AwaitingNetworkDKGVerification |
        DWalletState::NetworkRejectedDKGVerification |
        DWalletState::AwaitingUserImportedKeyInitiation |
        DWalletState::AwaitingNetworkImportedKeyVerification |
        DWalletState::NetworkRejectedImportedKeyVerification |
        DWalletState::AwaitingKeyHolderSignature { .. } => abort EDWalletInactive,
    }
}

/// Creates and charges a user-initiated MPC session for the current epoch.
/// 
/// This function implements the core session creation and payment logic for all
/// user-initiated dWallet operations. It handles fee collection, session sequencing,
/// and epoch management in a unified manner.
/// 
/// ### Fee Structure
/// - **Computation IKA**: Paid to validators for MPC computation
/// - **Consensus Validation IKA**: Paid for validator consensus on results
/// - **Gas Reimbursement SUI**: Covers blockchain transaction costs
/// - **System Call SUI**: Reserved for internal system operations
/// 
/// ### Session Management
/// 1. Assigns sequential session number for epoch ordering
/// 2. Creates session object with collected fees
/// 3. Updates session completion tracking for epoch transitions
/// 4. Stores event for retrieval during session completion
/// 
/// ### Epoch Coordination
/// - Sessions are sequentially numbered for deterministic epoch management
/// - Last session completion target is updated to manage epoch transitions
/// - Fee distribution occurs only upon successful session completion
/// 
/// ### Security Properties
/// - Fees are escrowed until session completion
/// - Session sequence numbers prevent replay attacks
/// - Network encryption key validation ensures proper key usage
fun charge_and_create_current_epoch_dwallet_event<E: copy + drop + store>(
    self: &mut DWalletCoordinatorInner,
    dwallet_network_encryption_key_id: ID,
    pricing_value: DWalletPricingValue,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    event_data: E,
    ctx: &mut TxContext,
): DWalletEvent<E> {
    assert!(self.dwallet_network_encryption_keys.contains(dwallet_network_encryption_key_id), EDWalletNetworkEncryptionKeyNotExist);


    assert!(payment_ika.value() >= pricing_value.computation_ika() + pricing_value.consensus_validation_ika(), EInsufficientIKAPayment);
    assert!(payment_sui.value() >= pricing_value.gas_fee_reimbursement_sui() + pricing_value.gas_fee_reimbursement_sui_for_system_calls(), EInsufficientSUIPayment);

    let computation_fee_charged_ika = payment_ika.split(pricing_value.computation_ika(), ctx).into_balance();
    let consensus_validation_fee_charged_ika = payment_ika.split(pricing_value.consensus_validation_ika(), ctx).into_balance();
    let gas_fee_reimbursement_sui = payment_sui.split(pricing_value.gas_fee_reimbursement_sui(), ctx).into_balance();
    self.pricing_and_fee_management.gas_fee_reimbursement_sui.join(payment_sui.split(pricing_value.gas_fee_reimbursement_sui_for_system_calls(), ctx).into_balance());

    let session_sequence_number = self.session_management.next_session_sequence_number;
    let session = DWalletSession {
        id: object::new(ctx),
        session_sequence_number,
        dwallet_network_encryption_key_id,
        consensus_validation_fee_charged_ika,
        computation_fee_charged_ika,
        gas_fee_reimbursement_sui,
    };

    let event = DWalletEvent {
        epoch: self.current_epoch,
        session_type: {
            SessionType::User {
                sequence_number: session_sequence_number,
            }
        },
        session_id: object::id(&session),
        event_data,
    };

    self.session_management.user_requested_sessions_events.add(session.id.to_inner(), event);
    self.session_management.sessions.add(session_sequence_number, session);
    self.session_management.next_session_sequence_number = session_sequence_number + 1;
    self.update_last_user_initiated_session_to_complete_in_current_epoch();

    event
}

/// Initiates a system-managed MPC session for network operations.
/// 
/// System sessions are initiated by the protocol itself for critical
/// network maintenance operations that don't involve direct user interaction.
/// These sessions are essential for network health and security.
/// 
/// ### Supported System Operations
/// - **Network DKG**: Distributed Key Generation for encryption keys
/// - **Key Reconfiguration**: Updating existing network encryption keys
/// - **Network Maintenance**: Other validator network coordination tasks
/// 
/// ### Key Differences from User Sessions
/// - **No Payment Required**: System operations don't charge users
/// - **No Sequential Numbering**: System sessions use generated IDs
/// - **Immediate Emission**: Events are emitted immediately rather than stored
/// - **Network Priority**: These sessions have priority in validator processing
/// 
/// ### Session Tracking
/// - Increments `started_system_sessions_count` for network monitoring
/// - Uses fresh object addresses for unique session identification
/// - Maintains epoch association for proper network coordination
/// 
/// ### Security Properties
/// - System sessions cannot be initiated by external users
/// - Session IDs are cryptographically unique to prevent conflicts
/// - Epoch tracking ensures proper network state consistency
fun initiate_system_dwallet_session<E: copy + drop + store>(
    self: &mut DWalletCoordinatorInner,
    event_data: E,
    ctx: &mut TxContext,
) {
    self.session_management.started_system_sessions_count = self.session_management.started_system_sessions_count + 1;

    let event = DWalletEvent {
        epoch: self.current_epoch,
        session_type: SessionType::System,
        session_id: object::id_from_address(tx_context::fresh_object_address(ctx)),
        event_data,
    };

    event::emit(event);
}

/// Retrieves an active dWallet and its public output for read-only operations.
/// 
/// This helper function safely accesses a dWallet ensuring it exists and is in
/// an active state suitable for cryptographic operations. The public output
/// represents the cryptographic public key material.
/// 
/// ### Parameters
/// - `self`: Reference to the coordinator
/// - `dwallet_id`: Unique identifier of the target dWallet
/// 
/// ### Returns
/// A tuple containing:
/// - Reference to the validated dWallet object
/// - Copy of the public output (cryptographic public key data)
/// 
/// ### Validation Performed
/// - Confirms dWallet exists in the coordinator's registry
/// - Validates dWallet is in `Active` state (DKG completed)
/// - Ensures public output is available for cryptographic operations
/// 
/// ### Aborts
/// - `EDWalletNotExists`: If the dWallet ID is not found
/// - `EDWalletNotActive`: If the dWallet is not in active state
fun get_active_dwallet_and_public_output(
    self: &DWalletCoordinatorInner,
    dwallet_id: ID,
): (&DWallet, vector<u8>) {
    assert!(self.dwallets.contains(dwallet_id), EDWalletNotExists);
    let dwallet = self.dwallets.borrow(dwallet_id);
    let public_output = dwallet.validate_active_and_get_public_output();
    (dwallet, *public_output)
}

/// Retrieves an active dWallet and its public output for mutable operations.
/// 
/// Similar to `get_active_dwallet_and_public_output` but returns a mutable reference
/// to the dWallet for operations that need to modify the dWallet state, such as
/// updating session counts or state transitions.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// - `dwallet_id`: Unique identifier of the target dWallet
/// 
/// ### Returns
/// A tuple containing:
/// - Mutable reference to the validated dWallet object
/// - Copy of the public output (cryptographic public key data)
/// 
/// ### Common Use Cases
/// - Updating presign session counters
/// - Modifying dWallet state during operations
/// - Recording operational history or metrics
/// - Managing active session associations
/// 
/// ### Validation Performed
/// - Confirms dWallet exists in the coordinator's registry
/// - Validates dWallet is in `Active` state (DKG completed)
/// - Ensures public output is available for cryptographic operations
/// 
/// ### Aborts
/// - `EDWalletNotExists`: If the dWallet ID is not found
/// - `EDWalletNotActive`: If the dWallet is not in active state
fun get_active_dwallet_and_public_output_mut(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
): (&mut DWallet, vector<u8>) {
    assert!(self.dwallets.contains(dwallet_id), EDWalletNotExists);
    let dwallet = self.dwallets.borrow_mut(dwallet_id);
    let public_output = dwallet.validate_active_and_get_public_output();
    (dwallet, *public_output)
}

/// Get the active encryption key ID by its address.
public(package) fun get_active_encryption_key(
    self: &DWalletCoordinatorInner,
    address: address,
): ID {
    assert!(self.encryption_keys.contains(address), EEncryptionKeyNotExist);
    self.encryption_keys.borrow(address).id.to_inner()
}

/// Validates that a curve is supported and not paused.
/// 
/// ### Parameters
/// - `self`: Reference to the coordinator
/// - `curve`: Curve identifier to validate
/// 
/// ### Aborts
/// - `EInvalidCurve`: If the curve is not supported
/// - `ECurvePaused`: If the curve is currently paused
fun validate_curve(
    self: &DWalletCoordinatorInner,
    curve: u32,
) {
    assert!(self.support_config.supported_curves_to_signature_algorithms_to_hash_schemes.contains(&curve), EInvalidCurve);
    assert!(!self.support_config.paused_curves.contains(&curve), ECurvePaused);
}

/// Validates that a curve and signature algorithm combination is supported and not paused.
/// 
/// ### Parameters
/// - `self`: Reference to the coordinator
/// - `curve`: Curve identifier to validate
/// - `signature_algorithm`: Signature algorithm to validate
/// 
/// ### Aborts
/// - `EInvalidCurve`: If the curve is not supported
/// - `ECurvePaused`: If the curve is currently paused
/// - `EInvalidSignatureAlgorithm`: If the signature algorithm is not supported for this curve
/// - `ESignatureAlgorithmPaused`: If the signature algorithm is currently paused
fun validate_curve_and_signature_algorithm(
    self: &DWalletCoordinatorInner,
    curve: u32,
    signature_algorithm: u32,
) {
    self.validate_curve(curve);
    let supported_curve_to_signature_algorithms = self.support_config.supported_curves_to_signature_algorithms_to_hash_schemes[&curve];
    assert!(supported_curve_to_signature_algorithms.contains(&signature_algorithm), EInvalidSignatureAlgorithm);
    assert!(!self.support_config.paused_signature_algorithms.contains(&signature_algorithm), ESignatureAlgorithmPaused);
}

/// Validates that a curve, signature algorithm, and hash scheme combination is supported and not paused.
/// 
/// ### Parameters
/// - `self`: Reference to the coordinator
/// - `curve`: Curve identifier to validate
/// - `signature_algorithm`: Signature algorithm to validate
/// - `hash_scheme`: Hash scheme to validate
/// 
/// ### Aborts
/// - `EInvalidCurve`: If the curve is not supported
/// - `ECurvePaused`: If the curve is currently paused
/// - `EInvalidSignatureAlgorithm`: If the signature algorithm is not supported for this curve
/// - `ESignatureAlgorithmPaused`: If the signature algorithm is currently paused
/// - `EInvalidHashScheme`: If the hash scheme is not supported for this combination
/// - `EHashSchemePaused`: If the hash scheme is currently paused
fun validate_curve_and_signature_algorithm_and_hash_scheme(
    self: &DWalletCoordinatorInner,
    curve: u32,
    signature_algorithm: u32,
    hash_scheme: u32,
) {
    self.validate_curve_and_signature_algorithm(curve, signature_algorithm);
    let supported_hash_schemes = self.support_config.supported_curves_to_signature_algorithms_to_hash_schemes[&curve][&signature_algorithm];
    assert!(supported_hash_schemes.contains(&hash_scheme), EInvalidHashScheme);
    assert!(!self.support_config.paused_hash_schemes.contains(&hash_scheme), EHashSchemePaused);
}

/// Registers an encryption key for secure dWallet share storage.
/// 
/// Creates and validates a new encryption key that can be used to encrypt
/// centralized secret key shares. The key signature is verified before registration.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// - `curve`: Cryptographic curve for the encryption key
/// - `encryption_key`: Serialized encryption key data
/// - `encryption_key_signature`: Ed25519 signature of the encryption key
/// - `signer_public_key`: Public key used to create the signature
/// - `ctx`: Transaction context for object creation
/// 
/// ### Effects
/// - Creates a new `EncryptionKey` object
/// - Emits a `CreatedEncryptionKeyEvent`
/// 
/// ### Aborts
/// - `EInvalidCurve`: If the curve is not supported
/// - `ECurvePaused`: If the curve is currently paused
/// - `EInvalidEncryptionKeySignature`: If the signature verification fails
public(package) fun register_encryption_key(
    self: &mut DWalletCoordinatorInner,
    curve: u32,
    encryption_key: vector<u8>,
    encryption_key_signature: vector<u8>,
    signer_public_key: vector<u8>,
    ctx: &mut TxContext
) {
    self.validate_curve(curve);
    assert!(
        ed25519_verify(&encryption_key_signature, &signer_public_key, &encryption_key),
        EInvalidEncryptionKeySignature
    );
    let signer_address = address::ed25519_address(signer_public_key);

    let id = object::new(ctx);

    let encryption_key_id = id.to_inner();

    self.encryption_keys.add(signer_address, EncryptionKey {
        id,
        created_at_epoch: self.current_epoch,
        curve,
        encryption_key,
        encryption_key_signature,
        signer_public_key,
        signer_address,
    });

    // Emit an event to signal the creation of the encryption key
    event::emit(CreatedEncryptionKeyEvent {
        encryption_key_id,
        signer_address,
    });
}

/// Approves a message for signing by a dWallet.
/// 
/// Creates a message approval that authorizes the specified message to be signed
/// using the given signature algorithm and hash scheme. This approval can later
/// be used to initiate a signing session.
/// 
/// ### Parameters
/// - `self`: Reference to the coordinator
/// - `dwallet_cap`: Capability proving control over the dWallet
/// - `signature_algorithm`: Algorithm to use for signing
/// - `hash_scheme`: Hash scheme to apply to the message
/// - `message`: Raw message bytes to be signed
/// 
/// ### Returns
/// A `MessageApproval` that can be used to request signing
/// 
/// ### Aborts
/// - `EImportedKeyDWallet`: If this is an imported key dWallet (use `approve_imported_key_message` instead)
/// - `EDWalletNotExists`: If the dWallet doesn't exist
/// - `EDWalletInactive`: If the dWallet is not in active state
/// - Various validation errors for unsupported/paused algorithms
public(package) fun approve_message(
    self: &DWalletCoordinatorInner,
    dwallet_cap: &DWalletCap,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector<u8>
): MessageApproval {
    let dwallet_id = dwallet_cap.dwallet_id;

    let is_imported_key_dwallet = self.validate_approve_message(dwallet_id, signature_algorithm, hash_scheme);
    assert!(!is_imported_key_dwallet, EImportedKeyDWallet);

    let approval = MessageApproval {
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message,
    };

    approval
}

/// Approves a message for signing by an imported key dWallet.
/// 
/// Creates a message approval that authorizes the specified message to be signed
/// using the given signature algorithm and hash scheme. This approval can later
/// be used to initiate a signing session.
/// 
/// ### Parameters
/// - `self`: Reference to the coordinator
/// - `imported_key_dwallet_cap`: Capability proving control over the dWallet
/// - `signature_algorithm`: Algorithm to use for signing
/// - `hash_scheme`: Hash scheme to apply to the message
/// - `message`: Raw message bytes to be signed
/// 
/// ### Returns
/// A `ImportedKeyMessageApproval` that can be used to request signing
/// 
/// ### Aborts
/// - `ENotImportedKeyDWallet`: If this is not an imported key dWallet (use `approve_message` instead)
/// - `EDWalletNotExists`: If the dWallet doesn't exist
/// - `EDWalletInactive`: If the dWallet is not in active state
/// - Various validation errors for unsupported/paused algorithms
public(package) fun approve_imported_key_message(
    self: &DWalletCoordinatorInner,
    imported_key_dwallet_cap: &ImportedKeyDWalletCap,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector<u8>
): ImportedKeyMessageApproval {
    let dwallet_id = imported_key_dwallet_cap.dwallet_id;

    let is_imported_key_dwallet = self.validate_approve_message(dwallet_id, signature_algorithm, hash_scheme);
    assert!(is_imported_key_dwallet, ENotImportedKeyDWallet);

    let approval = ImportedKeyMessageApproval {
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message,
    };

    approval
}

/// Perform shared validation for both the dWallet and imported key dWallet's variants of `approve_message()`.
/// Verify the `curve`, `signature_algorithm` and `hash_scheme` choice, and that the dWallet exists.
/// Returns whether this is an imported key dWallet, to be verified by the caller.
fun validate_approve_message(
    self: &DWalletCoordinatorInner,
    dwallet_id: ID,
    signature_algorithm: u32,
    hash_scheme: u32,
): bool {
    let (dwallet, _) = self.get_active_dwallet_and_public_output(dwallet_id);

    self.validate_curve_and_signature_algorithm_and_hash_scheme(dwallet.curve, signature_algorithm, hash_scheme);

    dwallet.is_imported_key_dwallet
}

/// Starts the first round of Distributed Key Generation (DKG) for a new dWallet.
///
/// Creates a new dWallet in the DKG requested state and initiates the first round
/// of the DKG protocol through the validator network. Returns a capability that
/// grants control over the newly created dWallet.
///
/// ### Parameters
/// - `self`: Mutable reference to the DWallet coordinator
/// - `dwallet_network_encryption_key_id`: ID of the network encryption key to use
/// - `curve`: Elliptic curve to use for the dWallet
/// - `payment_ika`: IKA payment for computation fees
/// - `payment_sui`: SUI payment for gas reimbursement
/// - `ctx`: Transaction context
///
/// ### Returns
/// A new `DWalletCap` object granting control over the created dWallet
///
/// ### Effects
/// - Creates a new `DWallet` object in DKG requested state
/// - Creates and returns a `DWalletCap` for the new dWallet
/// - Charges fees and creates a session for the DKG process
/// - Emits a `DWalletDKGFirstRoundRequestEvent` to start the protocol
///
/// ### Aborts
/// - `EInvalidCurve`: If the curve is not supported or is paused
/// - `EDWalletNetworkEncryptionKeyNotExist`: If the network encryption key doesn't exist
/// - `EMissingProtocolPricing`: If pricing is not configured for DKG first round
/// - Various payment-related errors if insufficient funds provided
public(package) fun request_dwallet_dkg_first_round(
    self: &mut DWalletCoordinatorInner,
    dwallet_network_encryption_key_id: ID,
    curve: u32,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): DWalletCap {
    self.validate_curve(curve);

    let mut pricing_value = self.pricing_and_fee_management.default.try_get_dwallet_pricing_value(curve, option::none(), DKG_FIRST_ROUND_PROTOCOL_FLAG);
    assert!(pricing_value.is_some(), EMissingProtocolPricing);

    // TODO(@Omer): check the state of the dWallet (i.e., not waiting for dkg.)
    // TODO(@Omer): I believe the best thing would be to always use the latest key. I'm not sure why the user should even supply the id.
    assert!(self.dwallet_network_encryption_keys.contains(dwallet_network_encryption_key_id), EDWalletNetworkEncryptionKeyNotExist);

    // Create a new `DWalletCap` object.
    let id = object::new(ctx);
    let dwallet_id = id.to_inner();
    let dwallet_cap = DWalletCap {
        id: object::new(ctx),
        dwallet_id,
    };
    let dwallet_cap_id = object::id(&dwallet_cap);

    // Create a new `DWallet` object,
    // link it to the `dwallet_cap` we just created by id,
    // and insert it into the `dwallets` map.
    self.dwallets.add(dwallet_id, DWallet {
        id,
        created_at_epoch: self.current_epoch,
        curve,
        public_user_secret_key_share: option::none(),
        dwallet_cap_id,
        dwallet_network_encryption_key_id,
        is_imported_key_dwallet: false,
        encrypted_user_secret_key_shares: object_table::new(ctx),
        sign_sessions: object_table::new(ctx),
        state: DWalletState::DKGRequested,
    });


    // Emit an event to request the Ika network to start DKG for this dWallet.
    event::emit(self.charge_and_create_current_epoch_dwallet_event(
                dwallet_network_encryption_key_id,
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        DWalletDKGFirstRoundRequestEvent {
            dwallet_id,
            dwallet_cap_id,
            dwallet_network_encryption_key_id,
            curve,
        },
        ctx,
    ));

    dwallet_cap
}
/// Updates the last session sequence number that should complete in the current epoch.
/// 
/// Implements session flow control by limiting the number of active sessions per epoch.
/// This ensures validators don't become overloaded and can complete sessions before
/// epoch transitions.
/// 
/// ### Algorithm
/// 1. Skip update if session management is locked (during epoch transition)
/// 2. Calculate target: completed sessions + max buffer, capped by latest session
/// 3. Only update if the new target is higher (prevents regression)
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// 
/// ### Effects
/// - Updates `last_user_initiated_session_to_complete_in_current_epoch` if appropriate
/// - Maintains session flow control within the configured buffer limits
/// - Ensures session completion targets only increase, never decrease
fun update_last_user_initiated_session_to_complete_in_current_epoch(self: &mut DWalletCoordinatorInner) {
    // Don't update during epoch transitions when session management is locked
    if (self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch) {
        return
    };

    // Calculate new target: completed + buffer, but don't exceed latest session
    let new_last_user_initiated_session_to_complete_in_current_epoch = (
        self.session_management.number_of_completed_user_initiated_sessions + self.session_management.max_active_sessions_buffer
    ).min(
        self.session_management.next_session_sequence_number - 1
    );

    // Sanity check: Only update if the new target is higher (prevent regression)
    if (self.session_management.last_user_initiated_session_to_complete_in_current_epoch >= new_last_user_initiated_session_to_complete_in_current_epoch) {
        return
    };
    self.session_management.last_user_initiated_session_to_complete_in_current_epoch = new_last_user_initiated_session_to_complete_in_current_epoch;
}

/// Validates that all required sessions for the current epoch have completed.
/// 
/// This function performs a comprehensive check to ensure the system is ready
/// for epoch advancement by verifying that all scheduled sessions have finished.
/// 
/// ### Parameters
/// - `self`: Reference to the coordinator
/// 
/// ### Returns
/// `true` if all required sessions are completed and epoch can advance, `false` otherwise
/// 
/// ### Validation Criteria
/// 1. **Session Management Locked**: `last_user_initiated_session_to_complete_in_current_epoch` must be locked
/// 2. **User Sessions Complete**: All user-initiated sessions up to the target sequence number must be completed
/// 3. **System Sessions Complete**: All started system sessions must be completed
/// 
/// ### Why This Matters
/// - Prevents epoch transitions with incomplete operations
/// - Ensures validator consensus on session completion
/// - Maintains system consistency across epoch boundaries
/// - Prevents resource leaks from abandoned sessions
/// 
/// ### Session Types
/// - **User Sessions**: Have sequence numbers for multi-epoch scheduling
/// - **System Sessions**: Must complete within their creation epoch
public(package) fun all_current_epoch_sessions_completed(self: &DWalletCoordinatorInner): bool {
    return (self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch &&
        (self.session_management.number_of_completed_user_initiated_sessions == self.session_management.last_user_initiated_session_to_complete_in_current_epoch) &&
        (self.session_management.completed_system_sessions_count == self.session_management.started_system_sessions_count))
}

/// Completes a user-initiated session and processes its associated fees.
/// 
/// This function handles the critical session completion workflow, including fee
/// distribution, state cleanup, and session accounting. It's called when the
/// validator network has finished processing a user's MPC request.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// - `session_sequence_number`: Sequential number of the session to complete
/// 
/// ### Returns
/// Gas reimbursement balance to be distributed to the user
/// 
/// ### Session Completion Process
/// 1. **Session Accounting**: Increments completed session counter
/// 2. **Buffer Management**: Updates session completion target based on new buffer availability
/// 3. **Fee Distribution**: Distributes collected fees to appropriate recipients
/// 4. **Resource Cleanup**: Removes session objects and events from storage
/// 5. **Network Key Updates**: Credits computation fees to the network encryption key
/// 
/// ### Fee Distribution
/// - **Computation Fees (IKA)**: Transferred to network encryption key for validator rewards
/// - **Consensus Validation Fees (IKA)**: Added to coordinator's fee pool for consensus rewards
/// - **Gas Reimbursement (SUI)**: Returned to caller for user refund
/// 
/// ### Security Properties
/// - Only called for successful session completions
/// - Fees are distributed atomically to prevent partial distributions
/// - Session sequence numbers ensure proper ordering
/// - Resource cleanup prevents memory leaks
/// 
/// ### System Sessions
/// This function is never called for system sessions, which handle their own
/// completion workflow without user fee management.
fun remove_user_initiated_session_and_charge<E: copy + drop + store>(self: &mut DWalletCoordinatorInner, session_sequence_number: u64): Balance<SUI> {
    self.session_management.number_of_completed_user_initiated_sessions = self.session_management.number_of_completed_user_initiated_sessions + 1;

    self.update_last_user_initiated_session_to_complete_in_current_epoch();
    let session = self.session_management.sessions.remove(session_sequence_number);

    // Unpack and delete the `DWalletSession` object.
    let DWalletSession {
        computation_fee_charged_ika,
        gas_fee_reimbursement_sui,
        consensus_validation_fee_charged_ika,
        dwallet_network_encryption_key_id,
        id,
        ..
    } = session;

    // Remove the corresponding event.
    let dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(dwallet_network_encryption_key_id);
    let _: DWalletEvent<E> = self.session_management.user_requested_sessions_events.remove(id.to_inner());

    id.delete();

    dwallet_network_encryption_key.computation_fee_charged_ika.join(computation_fee_charged_ika);
    self.pricing_and_fee_management.consensus_validation_fee_charged_ika.join(consensus_validation_fee_charged_ika);
    //self.gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
    gas_fee_reimbursement_sui
}

/// Processes validator network response to dWallet DKG first round.
/// 
/// This function handles the validator network's response to a user's DKG first round
/// request, advancing the dWallet through its initialization lifecycle. It represents
/// the completion of the first phase of distributed cryptographic key generation.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// - `dwallet_id`: ID of the dWallet undergoing DKG
/// - `first_round_output`: Cryptographic output from validators' first round computation
/// - `rejected`: Whether the validator network rejected the DKG request
/// - `session_sequence_number`: Session identifier for fee processing
/// 
/// ### Returns
/// Gas reimbursement balance for user refund
/// 
/// ### DKG First Round Process
/// 1. **Session Completion**: Processes session fees and cleanup
/// 2. **State Validation**: Ensures dWallet is in correct state for first round completion
/// 3. **Output Processing**: Handles validator output or rejection appropriately
/// 4. **Event Emission**: Notifies ecosystem of DKG progress or failure
/// 5. **State Transition**: Updates dWallet to next appropriate state
/// 
/// ### Success Path
/// - **Input**: Valid first round output from validator network
/// - **State Transition**: `DKGRequested` → `AwaitingUserDKGVerificationInitiation`
/// - **Event**: `CompletedDWalletDKGFirstRoundEvent` with cryptographic output
/// - **Next Step**: User must verify output and initiate second round
/// 
/// ### Rejection Path  
/// - **Input**: Network rejection signal (computational or consensus failure)
/// - **State Transition**: `DKGRequested` → `NetworkRejectedDKGRequest`
/// - **Event**: `RejectedDWalletDKGFirstRoundEvent` signaling failure
/// - **Next Step**: User must create new dWallet or retry operation
/// 
/// ### Security Properties
/// - Only processes sessions in correct DKG state
/// - Validator consensus ensures output authenticity
/// - State transitions are atomic and irreversible
/// - Fees are processed regardless of success/failure
/// 
/// ### Network Integration
/// This function is exclusively called by the Ika validator network as part
/// of the consensus protocol, never directly by users.
public(package) fun respond_dwallet_dkg_first_round(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    first_round_output: vector<u8>,
    rejected: bool,
    session_sequence_number: u64,
): Balance<SUI> {
    let gas_fee_reimbursement_sui = self.remove_user_initiated_session_and_charge<DWalletDKGFirstRoundRequestEvent>(session_sequence_number);

    let dwallet = self.get_dwallet_mut(dwallet_id);
    dwallet.state = match (dwallet.state) {
        DWalletState::DKGRequested => {
            if (rejected) {
                event::emit(RejectedDWalletDKGFirstRoundEvent {
                    dwallet_id,
                });
                DWalletState::NetworkRejectedDKGRequest
            } else {
                event::emit(CompletedDWalletDKGFirstRoundEvent {
                    dwallet_id,
                    first_round_output,
                });
                DWalletState::AwaitingUserDKGVerificationInitiation {
                    first_round_output
                }
            }
        },
        _ => abort EWrongState
    };

    gas_fee_reimbursement_sui
}

/// Initiates the second round of Distributed Key Generation (DKG) with encrypted user shares.
/// 
/// This function represents the user's contribution to the DKG second round, where they
/// provide their encrypted secret share and request validator network verification.
/// It creates the encrypted share object and transitions the dWallet to network verification.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// - `dwallet_cap`: User's capability proving dWallet ownership
/// - `centralized_public_key_share_and_proof`: User's public key contribution with ZK proof
/// - `encrypted_centralized_secret_share_and_proof`: User's encrypted secret share with proof
/// - `encryption_key_address`: Address of the encryption key for securing the share
/// - `user_public_output`: User's contribution to the final public key
/// - `signer_public_key`: Ed25519 key for signature verification
/// - `payment_ika`: User's IKA payment for computation
/// - `payment_sui`: User's SUI payment for gas reimbursement
/// - `ctx`: Transaction context
/// 
/// ### DKG Second Round Process
/// 1. **Validation**: Verifies encryption key compatibility and dWallet state
/// 2. **Share Creation**: Creates `EncryptedUserSecretKeyShare` with verification pending
/// 3. **Payment Processing**: Charges user for validator computation and consensus
/// 4. **Event Emission**: Requests validator network to verify encrypted share
/// 5. **State Transition**: Updates dWallet to `AwaitingNetworkDKGVerification`
/// 
/// ### Cryptographic Security
/// - **Zero-Knowledge Proofs**: User provides proofs of correct share encryption
/// - **Encryption Key Validation**: Ensures proper key curve compatibility
/// - **Share Verification**: Network will validate encrypted share correctness
/// - **Threshold Security**: Maintains distributed key generation properties
/// 
/// ### Network Integration
/// Emits `DWalletDKGSecondRoundRequestEvent` for validator processing,
/// triggering network verification of the encrypted share.
/// 
/// ### Aborts
/// - `EImportedKeyDWallet`: If called on imported key dWallet
/// - `EMismatchCurve`: If encryption key curve doesn't match dWallet curve
/// - `EWrongState`: If dWallet not in correct state for second round
/// - Various validation and payment errors
public(package) fun request_dwallet_dkg_second_round(
    self: &mut DWalletCoordinatorInner,
    dwallet_cap: &DWalletCap,
    centralized_public_key_share_and_proof: vector<u8>,
    encrypted_centralized_secret_share_and_proof: vector<u8>,
    encryption_key_address: address,
    user_public_output: vector<u8>,
    signer_public_key: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    let encryption_key = self.encryption_keys.borrow(encryption_key_address);
    let encryption_key_curve = encryption_key.curve;
    let encryption_key_id = encryption_key.id.to_inner();
    let encryption_key = encryption_key.encryption_key;
    let created_at_epoch: u64 = self.current_epoch;
    let dwallet_id = dwallet_cap.dwallet_id;
    let dwallet = self.get_dwallet(dwallet_id);
    let curve = dwallet.curve;

    assert!(!dwallet.is_imported_key_dwallet, EImportedKeyDWallet);
    assert!(encryption_key_curve == curve, EMismatchCurve);
    self.validate_curve(curve);

    let first_round_output = match (&dwallet.state) {
        DWalletState::AwaitingUserDKGVerificationInitiation {
            first_round_output,
        } => {
            *first_round_output
        },
        _ => abort EWrongState
    };

    let dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;

    let encrypted_user_share = EncryptedUserSecretKeyShare {
        id: object::new(ctx),
        created_at_epoch,
        dwallet_id,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_id,
        encryption_key_address,
        source_encrypted_user_secret_key_share_id: option::none(),
        state: EncryptedUserSecretKeyShareState::AwaitingNetworkVerification
    };
    let encrypted_user_secret_key_share_id = object::id(&encrypted_user_share);

    let mut pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::none(), DKG_SECOND_ROUND_PROTOCOL_FLAG);
    assert!(pricing_value.is_some(), EMissingProtocolPricing);


    let emit_event = self.charge_and_create_current_epoch_dwallet_event(
        dwallet_network_encryption_key_id,
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        DWalletDKGSecondRoundRequestEvent {
            encrypted_user_secret_key_share_id,
            dwallet_id,
            first_round_output,
            centralized_public_key_share_and_proof,
            dwallet_cap_id: object::id(dwallet_cap),
            encrypted_centralized_secret_share_and_proof,
            encryption_key,
            encryption_key_id,
            encryption_key_address,
            user_public_output,
            signer_public_key,
            dwallet_network_encryption_key_id,
            curve,
        },
        ctx,
    );

    event::emit(emit_event);

    let dwallet = self.get_dwallet_mut(dwallet_cap.dwallet_id);
    dwallet.encrypted_user_secret_key_shares.add(encrypted_user_secret_key_share_id, encrypted_user_share);
    dwallet.state = DWalletState::AwaitingNetworkDKGVerification;
}

/// This function is called by the Ika network to respond to the dWallet DKG second round request made by the user.
///
/// Completes the second round of the Distributed Key Generation (DKG) process and
/// advances the [`DWallet`] state to `AwaitingKeyHolderSignature` with the DKG public output registered in it.
///
/// Advances the `EncryptedUserSecretKeyShareState` to `NetworkVerificationCompleted`.
///
/// Also emits an event with the public output.
public(package) fun respond_dwallet_dkg_second_round(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    public_output: vector<u8>,
    encrypted_user_secret_key_share_id: ID,
    session_id: ID,
    rejected: bool,
    session_sequence_number: u64,
): Balance<SUI> {
    let gas_fee_reimbursement_sui = self.remove_user_initiated_session_and_charge<DWalletDKGSecondRoundRequestEvent>(session_sequence_number);
    let dwallet = self.get_dwallet_mut(dwallet_id);

    dwallet.state = match (&dwallet.state) {
        DWalletState::AwaitingNetworkDKGVerification => {
            if (rejected) {
                event::emit(RejectedDWalletDKGSecondRoundEvent {
                    dwallet_id,
                    public_output,
                });
                DWalletState::NetworkRejectedDKGVerification
            } else {
                let encrypted_user_share = dwallet.encrypted_user_secret_key_shares.borrow_mut(encrypted_user_secret_key_share_id);
                encrypted_user_share.state = EncryptedUserSecretKeyShareState::NetworkVerificationCompleted;

                event::emit(CompletedDWalletDKGSecondRoundEvent {
                    dwallet_id,
                    public_output,
                    encrypted_user_secret_key_share_id,
                    session_id,
                });
                DWalletState::AwaitingKeyHolderSignature {
                    public_output
                }
            }
        },
        _ => abort EWrongState
    };
    gas_fee_reimbursement_sui
}

/// Requests a re-encryption of the user share of the dWallet by having the Ika network
/// verify a zk-proof that the encryption matches the public share of the dWallet.
///
/// This can be used as part of granting access or transferring the dWallet.
///
/// Creates a new `EncryptedUserSecretKeyShare` object, with the state awaiting the network verification.
/// Emits an event to request the verification by the network.
public(package) fun request_re_encrypt_user_share_for(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    destination_encryption_key_address: address,
    encrypted_centralized_secret_share_and_proof: vector<u8>,
    source_encrypted_user_secret_key_share_id: ID,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext,
) {
    let created_at_epoch = self.current_epoch;
    let destination_encryption_key = self.encryption_keys.borrow(destination_encryption_key_address);
    let destination_encryption_key_id = destination_encryption_key.id.to_inner();
    let destination_encryption_key = destination_encryption_key.encryption_key;

    let dwallet = self.get_dwallet_mut(dwallet_id);
    let public_output = *dwallet.validate_active_and_get_public_output();
    let dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;
    let curve = dwallet.curve;

    assert!(dwallet.encrypted_user_secret_key_shares.contains(source_encrypted_user_secret_key_share_id), EInvalidSource);

    let encrypted_user_share = EncryptedUserSecretKeyShare {
        id: object::new(ctx),
        created_at_epoch,
        dwallet_id,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_id: destination_encryption_key_id,
        encryption_key_address: destination_encryption_key_address,
        source_encrypted_user_secret_key_share_id: option::some(source_encrypted_user_secret_key_share_id),
        state: EncryptedUserSecretKeyShareState::AwaitingNetworkVerification
    };
    let encrypted_user_secret_key_share_id = object::id(&encrypted_user_share);
    dwallet.encrypted_user_secret_key_shares.add(encrypted_user_secret_key_share_id, encrypted_user_share);

    let mut pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::none(), RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG);
    assert!(pricing_value.is_some(), EMissingProtocolPricing);

    event::emit(
        self.charge_and_create_current_epoch_dwallet_event(
            dwallet_network_encryption_key_id,
            pricing_value.extract(),
            payment_ika,
            payment_sui,
            EncryptedShareVerificationRequestEvent {
                encrypted_centralized_secret_share_and_proof,
                public_output,
                dwallet_id,
                encryption_key: destination_encryption_key,
                encryption_key_id: destination_encryption_key_id,
                encrypted_user_secret_key_share_id,
                source_encrypted_user_secret_key_share_id,
                dwallet_network_encryption_key_id,
                curve,
            },
            ctx,
        )
    );
}

/// This function is called by the Ika network to respond to a re-encryption request of the user share of the dWallet
/// by setting the `EncryptedUserSecretKeyShareState` object's state according to the verification result.
public(package) fun respond_re_encrypt_user_share_for(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    encrypted_user_secret_key_share_id: ID,
    rejected: bool,
    session_sequence_number: u64
): Balance<SUI> {
    let gas_fee_reimbursement_sui = self.remove_user_initiated_session_and_charge<EncryptedShareVerificationRequestEvent>(session_sequence_number);
    let (dwallet, _) = self.get_active_dwallet_and_public_output_mut(dwallet_id);

    let encrypted_user_secret_key_share = dwallet.encrypted_user_secret_key_shares.borrow_mut(encrypted_user_secret_key_share_id);

    encrypted_user_secret_key_share.state = match(encrypted_user_secret_key_share.state) {
        EncryptedUserSecretKeyShareState::AwaitingNetworkVerification => {
            if(rejected) {
                event::emit(
                    RejectedEncryptedShareVerificationEvent {
                        encrypted_user_secret_key_share_id,
                        dwallet_id,
                    }
                );
                EncryptedUserSecretKeyShareState::NetworkVerificationRejected
            } else {
                event::emit(
                    CompletedEncryptedShareVerificationEvent {
                        encrypted_user_secret_key_share_id,
                        dwallet_id,
                    }
                );
                EncryptedUserSecretKeyShareState::NetworkVerificationCompleted
            }
        },
        _ => abort EWrongState
    };
    gas_fee_reimbursement_sui
}

/// Accept the encryption of the user share of a dWallet.
///
/// Called after the user verified the signature of the sender (who re-encrypted the user share for them)
/// on the public output of the dWallet, and that the decrypted share matches the public key share of the dWallet.
///
/// Register the user's own signature on the public output `user_output_signature` for an easy way to perform self-verification in the future.
///
/// Finalizes the `EncryptedUserSecretKeyShareState` object's state as `KeyHolderSigned`.
public(package) fun accept_encrypted_user_share(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    encrypted_user_secret_key_share_id: ID,
    user_output_signature: vector<u8>,
) {
    let dwallet = self.get_dwallet_mut(dwallet_id);
    match(&dwallet.state) {
        DWalletState::AwaitingKeyHolderSignature {
            public_output
        } => {
            dwallet.state = DWalletState::Active {
                public_output: *public_output
            };
        },
        DWalletState::Active { .. } => { },
        _ => abort EWrongState
    };
    let public_output = *dwallet.validate_active_and_get_public_output();
    let encrypted_user_secret_key_share = dwallet.encrypted_user_secret_key_shares.borrow(encrypted_user_secret_key_share_id);
    let encryption_key = self.encryption_keys.borrow(encrypted_user_secret_key_share.encryption_key_address);
    let encryption_key_id = encrypted_user_secret_key_share.encryption_key_id;
    let encryption_key_address = encrypted_user_secret_key_share.encryption_key_address;
    assert!(
        ed25519_verify(&user_output_signature, &encryption_key.signer_public_key, &public_output),
        EInvalidEncryptionKeySignature
    );
    let dwallet = self.get_dwallet_mut(dwallet_id);

    let encrypted_user_secret_key_share = dwallet.encrypted_user_secret_key_shares.borrow_mut(encrypted_user_secret_key_share_id);
    encrypted_user_secret_key_share.state = match (encrypted_user_secret_key_share.state) {
        EncryptedUserSecretKeyShareState::NetworkVerificationCompleted => EncryptedUserSecretKeyShareState::KeyHolderSigned {
            user_output_signature
        },
        _ => abort EWrongState
    };
    event::emit(
        AcceptEncryptedUserShareEvent {
            encrypted_user_secret_key_share_id,
            dwallet_id,
            user_output_signature,
            encryption_key_id,
            encryption_key_address,
        }
    );
}

/// Creates a new imported key dWallet, by creating a new `DWallet` object with `is_imported_key_dwallet` set and the state at `AwaitingUserImportedKeyInitiation`,
/// alongside a corresponding `ImportedKeyDWalletCap`.
///
/// Required as a first step before the user can call `request_imported_key_dwallet_verification()`,
/// which requires the user to know the `dwallet_id` for a unique identifier used by the user to prove the imported key is valid.
public(package) fun new_imported_key_dwallet(
    self: &mut DWalletCoordinatorInner,
    dwallet_network_encryption_key_id: ID,
    curve: u32,
    ctx: &mut TxContext
): ImportedKeyDWalletCap {
    self.validate_curve(curve);
    assert!(self.dwallet_network_encryption_keys.contains(dwallet_network_encryption_key_id), EDWalletNetworkEncryptionKeyNotExist);

    let id = object::new(ctx);
    let dwallet_id = id.to_inner();

    let dwallet_cap = ImportedKeyDWalletCap {
        id: object::new(ctx),
        dwallet_id,
    };

    let dwallet_cap_id = object::id(&dwallet_cap);
    self.dwallets.add(dwallet_id, DWallet {
        id,
        created_at_epoch: self.current_epoch,
        curve,
        public_user_secret_key_share: option::none(),
        dwallet_cap_id,
        dwallet_network_encryption_key_id,
        is_imported_key_dwallet: true,
        encrypted_user_secret_key_shares: object_table::new(ctx),
        sign_sessions: object_table::new(ctx),
        state: DWalletState::AwaitingUserImportedKeyInitiation,
    });
    event::emit(NewImportedKeyDWalletEvent {
        dwallet_id,
        dwallet_cap_id,
    });
    dwallet_cap
}

/// Request verification of the imported key dWallet from the Ika network.
///
/// Sets the state of the dWallet to `AwaitingNetworkImportedKeyVerification` and creates a new `EncryptedUserSecretKeyShare` object, with the state awaiting the network verification
/// that the user encrypted its user share correctly (the network will verify it as part of the second round).
///
/// Emits an event with the user's message and encrypted user share proof to the Ika network.
public(package) fun request_imported_key_dwallet_verification(
    self: &mut DWalletCoordinatorInner,
    dwallet_cap: &ImportedKeyDWalletCap,
    centralized_party_message: vector<u8>,
    encrypted_centralized_secret_share_and_proof: vector<u8>,
    encryption_key_address: address,
    user_public_output: vector<u8>,
    signer_public_key: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    let encryption_key = self.encryption_keys.borrow(encryption_key_address);
    let encryption_key_id = encryption_key.id.to_inner();
    let encryption_key = encryption_key.encryption_key;
    let created_at_epoch: u64 = self.current_epoch;
    let dwallet_id = dwallet_cap.dwallet_id;

    let dwallet = self.get_dwallet_mut(dwallet_cap.dwallet_id);
    assert!(dwallet.is_imported_key_dwallet, ENotImportedKeyDWallet);

    dwallet.state = match (&dwallet.state) {
        DWalletState::AwaitingUserImportedKeyInitiation => {
            DWalletState::AwaitingNetworkImportedKeyVerification
        },
        _ => abort EWrongState
    };
    let dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;
    let curve = dwallet.curve;

    let encrypted_user_share = EncryptedUserSecretKeyShare {
        id: object::new(ctx),
        created_at_epoch,
        dwallet_id,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_id,
        encryption_key_address,
        source_encrypted_user_secret_key_share_id: option::none(),
        state: EncryptedUserSecretKeyShareState::AwaitingNetworkVerification
    };

    let encrypted_user_secret_key_share_id = object::id(&encrypted_user_share);
    dwallet.encrypted_user_secret_key_shares.add(encrypted_user_secret_key_share_id, encrypted_user_share);

    let mut pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::none(), IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG);
    assert!(pricing_value.is_some(), EMissingProtocolPricing);

    let emit_event = self.charge_and_create_current_epoch_dwallet_event(
        dwallet_network_encryption_key_id,
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        DWalletImportedKeyVerificationRequestEvent {
            dwallet_id,
            encrypted_user_secret_key_share_id,
            centralized_party_message,
            dwallet_cap_id: object::id(dwallet_cap),
            encrypted_centralized_secret_share_and_proof,
            encryption_key,
            encryption_key_id,
            encryption_key_address,
            user_public_output,
            signer_public_key,
            dwallet_network_encryption_key_id,
            curve,
        },
        ctx,
    );

    event::emit(emit_event);
}

/// This function is called by the Ika network to respond to the import key dWallet verification request made by the user.
///
/// Completes the verification of an imported key dWallet and
/// advances the [`DWallet`] state to `AwaitingKeyHolderSignature` with the DKG public output registered in it.
/// Also emits an event with the public output.
///
/// Advances the `EncryptedUserSecretKeyShareState` to `NetworkVerificationCompleted`.
public(package) fun respond_imported_key_dwallet_verification(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    public_output: vector<u8>,
    encrypted_user_secret_key_share_id: ID,
    session_id: ID,
    rejected: bool,
    session_sequence_number: u64,
): Balance<SUI> {
    let gas_fee_reimbursement_sui = self.remove_user_initiated_session_and_charge<DWalletImportedKeyVerificationRequestEvent>(session_sequence_number);
    let dwallet = self.get_dwallet_mut(dwallet_id);

    dwallet.state = match (&dwallet.state) {
        DWalletState::AwaitingNetworkImportedKeyVerification => {
            if (rejected) {
                event::emit(RejectedDWalletImportedKeyVerificationEvent {
                    dwallet_id,
                });
                DWalletState::NetworkRejectedImportedKeyVerification
            } else {
                let encrypted_user_share = dwallet.encrypted_user_secret_key_shares.borrow_mut(encrypted_user_secret_key_share_id);
                encrypted_user_share.state = EncryptedUserSecretKeyShareState::NetworkVerificationCompleted;


                event::emit(CompletedDWalletImportedKeyVerificationEvent {
                    dwallet_id,
                    public_output,
                    encrypted_user_secret_key_share_id,
                    session_id,
                });
                DWalletState::AwaitingKeyHolderSignature {
                    public_output
                }
            }
        },
        _ => abort EWrongState
    };
    gas_fee_reimbursement_sui
}

/// Requests to make the user secret key shares of a dWallet public.
/// *IMPORTANT*: If you make the dWallet user secret key shares public, you remove
/// the zero trust security of the dWallet and you can't revert it.
///
/// This function emits a `MakeDWalletUserSecretKeySharePublicRequestEvent` event to initiate the
/// process of making the user secret key shares of a dWallet public. It charges the initiator for
/// the operation and creates a new event to record the request.
///
/// ### Parameters
/// - `dwallet_id`: The ID of the dWallet to make the user secret key shares public.
/// - `public_user_secret_key_share`: The public user secret key shares to be made public.
/// - `payment_ika`: The IKA payment for the operation.
/// - `payment_sui`: The SUI payment for the operation.
/// - `ctx`: The transaction context.
public(package) fun request_make_dwallet_user_secret_key_share_public(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    public_user_secret_key_share: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext,
) {
    let (dwallet, public_output) = self.get_active_dwallet_and_public_output(dwallet_id);
    let dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;
    let curve = dwallet.curve;
    assert!(dwallet.public_user_secret_key_share.is_none(), EDWalletUserSecretKeySharesAlreadyPublic);

    let mut pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::none(), MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG);
    assert!(pricing_value.is_some(), EMissingProtocolPricing);

    event::emit(
        self.charge_and_create_current_epoch_dwallet_event(
            dwallet_network_encryption_key_id,
            pricing_value.extract(),
            payment_ika,
            payment_sui,
            MakeDWalletUserSecretKeySharePublicRequestEvent {
                public_user_secret_key_share,
                public_output,
                curve,
                dwallet_id,
                dwallet_network_encryption_key_id,
            },
            ctx,
        )
    );
}

/// This function is called by the Ika network to respond to the request to make the dWallet's user share public.
/// Sets `public_user_secret_key_share` to the verified value.
public(package) fun respond_make_dwallet_user_secret_key_share_public(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    public_user_secret_key_share: vector<u8>,
    rejected: bool,
    session_sequence_number: u64,
): Balance<SUI> {
    let gas_fee_reimbursement_sui = self.remove_user_initiated_session_and_charge<MakeDWalletUserSecretKeySharePublicRequestEvent>(session_sequence_number);
    let dwallet = self.get_dwallet_mut(dwallet_id);
    if (rejected) {
        event::emit(RejectedMakeDWalletUserSecretKeySharePublicEvent {
            dwallet_id,
        });
    } else {
        dwallet.public_user_secret_key_share.fill(public_user_secret_key_share);
        event::emit(CompletedMakeDWalletUserSecretKeySharePublicEvent {
            dwallet_id,
        });
    };
    gas_fee_reimbursement_sui
}

/// Requests generation of a dWallet-specific presign for accelerated signing.
/// 
/// Presigns are precomputed cryptographic material that dramatically reduce online
/// signing latency from seconds to milliseconds. This function creates a dWallet-specific
/// presign that can only be used with the specified dWallet.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// - `dwallet_id`: Target dWallet for the presign generation
/// - `signature_algorithm`: Algorithm requiring presign material (e.g., ECDSA)
/// - `payment_ika`: User's IKA payment for computation
/// - `payment_sui`: User's SUI payment for gas reimbursement
/// - `ctx`: Transaction context
/// 
/// ### Returns
/// `UnverifiedPresignCap` that must be verified before use in signing
/// 
/// 
/// ### Security Properties
/// - **Single Use**: Each presign can only be consumed once
/// - **Cryptographic Binding**: Tied to specific dWallet public key
/// - **Validator Consensus**: Generated through secure MPC protocol
/// - **Expiration**: Presigns have limited validity period
/// 
/// ### Next Steps
/// 1. Wait for validator network to process the presign request
/// 2. Call `is_presign_valid()` to check completion status
/// 3. Use `verify_presign_cap()` to convert to verified capability
/// 4. Combine with message approval for actual signing
/// 
/// ### Aborts
/// - `EInvalidSignatureAlgorithm`: If the signature algorithm is not allowed for dWallet-specific presigns
/// - `EInvalidCurve`: If the curve is not supported
/// - `EInvalidNetworkEncryptionKey`: If the network encryption key is not supported
/// - `EInsufficientFunds`: If the user does not have enough funds to pay for the presign
public(package) fun request_presign(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    signature_algorithm: u32,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): UnverifiedPresignCap {
    let created_at_epoch = self.current_epoch;

    let (dwallet, public_output) = self.get_active_dwallet_and_public_output(dwallet_id);

    let curve = dwallet.curve;

    self.validate_curve_and_signature_algorithm(curve, signature_algorithm);
    
    assert!(!self.support_config.signature_algorithms_allowed_global_presign.contains(&signature_algorithm), EInvalidSignatureAlgorithm);

    let dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;

    let id = object::new(ctx);
    let presign_id = id.to_inner();

    let cap = UnverifiedPresignCap {
        id: object::new(ctx),
        dwallet_id: option::some(dwallet_id),
        presign_id,
    };

    self.presign_sessions.add(presign_id, PresignSession {
        id,
        created_at_epoch,
        signature_algorithm,
        curve,
        dwallet_id: option::some(dwallet_id),
        cap_id: object::id(&cap),
        state: PresignState::Requested,
    });

    let mut pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), PRESIGN_PROTOCOL_FLAG);
    assert!(pricing_value.is_some(), EMissingProtocolPricing);

    event::emit(
        self.charge_and_create_current_epoch_dwallet_event(
            dwallet_network_encryption_key_id,
            pricing_value.extract(),
            payment_ika,
            payment_sui,
            PresignRequestEvent {
                dwallet_id: option::some(dwallet_id),
                presign_id,
                dwallet_public_output: option::some(public_output),
                dwallet_network_encryption_key_id,
                curve,
                signature_algorithm,
            },
            ctx,
        )
    );
    cap
}

/// Requests generation of a global presign for flexible cross-dWallet use.
/// 
/// Global presigns provide computational efficiency by creating precomputed material
/// that can be used with any compatible dWallet under the same network encryption key.
/// This enables better resource utilization and batch processing optimization.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// - `dwallet_network_encryption_key_id`: Network encryption key for presign compatibility
/// - `curve`: Cryptographic curve for presign generation
/// - `signature_algorithm`: Algorithm requiring presign material
/// - `payment_ika`: User's IKA payment for computation
/// - `payment_sui`: User's SUI payment for gas reimbursement
/// - `ctx`: Transaction context
/// 
/// ### Returns
/// `UnverifiedPresignCap` that can be used with any compatible dWallet
/// 
/// ### Security Considerations
/// - Global presigns maintain cryptographic security properties
/// - Network encryption key provides isolation between key epochs
/// - Validator consensus ensures presign authenticity
/// - Single-use property prevents replay attacks
/// 
/// ### Next Steps
/// 1. Wait for validator network to process the global presign request
/// 2. Verify presign completion using `is_presign_valid()`
/// 3. Convert to `VerifiedPresignCap` with `verify_presign_cap()`
/// 4. Use with any compatible dWallet for signing operations
/// 
/// ### Aborts
/// - `EInvalidSignatureAlgorithm`: If the signature algorithm is not allowed for global presigns
/// - `EInvalidCurve`: If the curve is not supported
/// - `EInvalidNetworkEncryptionKey`: If the network encryption key is not supported
/// - `EInsufficientFunds`: If the user does not have enough funds to pay for the presign
public(package) fun request_global_presign(
    self: &mut DWalletCoordinatorInner,
    dwallet_network_encryption_key_id: ID,
    curve: u32,
    signature_algorithm: u32,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): UnverifiedPresignCap {
    let created_at_epoch = self.current_epoch;

    self.validate_curve_and_signature_algorithm(curve, signature_algorithm);

    assert!(self.support_config.signature_algorithms_allowed_global_presign.contains(&signature_algorithm), EInvalidSignatureAlgorithm);

    let id = object::new(ctx);
    let presign_id = id.to_inner();
    let cap = UnverifiedPresignCap {
        id: object::new(ctx),
        dwallet_id: option::none(),
        presign_id,
    };
    self.presign_sessions.add(presign_id, PresignSession {
        id,
        created_at_epoch,
        signature_algorithm,
        curve,
        dwallet_id: option::none(),
        cap_id: object::id(&cap),
        state: PresignState::Requested,
    });

    let mut pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), PRESIGN_PROTOCOL_FLAG);
    assert!(pricing_value.is_some(), EMissingProtocolPricing);

    event::emit(
        self.charge_and_create_current_epoch_dwallet_event(
            dwallet_network_encryption_key_id,
            pricing_value.extract(),
            payment_ika,
            payment_sui,
            PresignRequestEvent {
                dwallet_id: option::none(),
                presign_id,
                dwallet_public_output: option::none(),
                dwallet_network_encryption_key_id,
                curve,
                signature_algorithm,
            },
            ctx,
        )
    );
    cap
}

/// Processes validator network response to presign generation request.
/// 
/// This function handles the completion or rejection of presign generation by the
/// validator network, updating the presign session state and emitting appropriate events.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// - `dwallet_id`: Target dWallet ID for dWallet-specific presigns (None for global)
/// - `presign_id`: Unique identifier of the presign session
/// - `session_id`: MPC session ID that processed the presign
/// - `presign`: Generated cryptographic presign material (if successful)
/// - `rejected`: Whether the validator network rejected the presign request
/// - `session_sequence_number`: Session sequence for fee processing
/// 
/// ### Returns
/// Gas reimbursement balance for user refund
/// 
/// ### Success Path
/// - **State Transition**: `Requested` → `Completed`
/// - **Presign Storage**: Cryptographic material is stored in session
/// - **Event**: `CompletedPresignEvent` with presign data
/// - **Capability**: Associated capability can now be verified and used
/// 
/// ### Rejection Path
/// - **State Transition**: `Requested` → `NetworkRejected`
/// - **Event**: `RejectedPresignEvent` indicating failure
/// - **Capability**: Associated capability becomes unusable
/// - **Common Causes**: Insufficient validator participation, computation errors
/// 
/// 
/// ### Security Properties
/// - Presign material is cryptographically secure and verifiable
/// - Single-use property enforced through session consumption
/// - Validator consensus ensures authenticity of generated material
/// - Rejection handling prevents use of incomplete presigns
/// 
/// ### Network Integration
/// This function is exclusively called by the Ika validator network as part
/// of the distributed presign generation protocol.
public(package) fun respond_presign(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: Option<ID>,
    presign_id: ID,
    session_id: ID,
    presign: vector<u8>,
    rejected: bool,
    session_sequence_number: u64
): Balance<SUI> {
    let gas_fee_reimbursement_sui = self.remove_user_initiated_session_and_charge<PresignRequestEvent>(session_sequence_number);

    let presign_obj = self.presign_sessions.borrow_mut(presign_id);

    presign_obj.state = match(presign_obj.state) {
        PresignState::Requested => {
            if(rejected) {
                event::emit(RejectedPresignEvent {
                    dwallet_id,
                    session_id,
                    presign_id
                });
                PresignState::NetworkRejected
            } else {
                event::emit(CompletedPresignEvent {
                    dwallet_id,
                    session_id,
                    presign_id,
                    presign
                });
                PresignState::Completed {
                    presign
                }
            }
        },
        _ => abort EWrongState
    };
    gas_fee_reimbursement_sui
}

/// Validates that a presign capability corresponds to a completed presign session.
/// 
/// Checks both the completion state and capability ID matching to ensure
/// the capability is authentic and the presign is ready for use.
/// 
/// ### Parameters
/// - `self`: Reference to the coordinator
/// - `cap`: Unverified presign capability to validate
/// 
/// ### Returns
/// `true` if the presign is completed and the capability is valid, `false` otherwise
/// 
/// ### Validation Criteria
/// - Presign session must be in `Completed` state
/// - Capability ID must match the session's recorded capability ID
/// - Presign session must exist in the coordinator
public(package) fun is_presign_valid(
    self: &DWalletCoordinatorInner,
    cap: &UnverifiedPresignCap,
): bool {
    let presign = self.presign_sessions.borrow(cap.presign_id);
    match(&presign.state) {
        PresignState::Completed { .. } => {
            cap.id.to_inner() == presign.cap_id
        },
        _ => false
    }
}

/// Verify `cap` by deleting the `UnverifiedPresignCap` object and replacing it with a new `VerifiedPresignCap`,
/// if `is_presign_valid()`.
public(package) fun verify_presign_cap(
    self: &mut DWalletCoordinatorInner,
    cap: UnverifiedPresignCap,
    ctx: &mut TxContext
): VerifiedPresignCap {
    let UnverifiedPresignCap {
        id,
        dwallet_id,
        presign_id
    } = cap;

    let cap_id = id.to_inner();
    id.delete();

    let presign = self.presign_sessions.borrow_mut(presign_id);
    assert!(presign.cap_id == cap_id, EIncorrectCap);
        match(&presign.state) {
        PresignState::Completed { .. } => {},
        _ => abort EUnverifiedCap
    };

    let cap = VerifiedPresignCap {
        id: object::new(ctx),
        dwallet_id,
        presign_id,
    };
    presign.cap_id = cap.id.to_inner();

    cap
}

/// This function is a shared logic for both the standard and future sign flows.
///
/// It checks the presign is valid and deletes it (and its `presign_cap`), thus assuring it is not used twice.
///
/// Creates a `SignSession` object and register it in `sign_sessions`.
///
/// Finally it emits the sign event.
fun validate_and_initiate_sign(
    self: &mut DWalletCoordinatorInner,
    pricing_value: DWalletPricingValue,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    dwallet_id: ID,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector<u8>,
    presign_cap: VerifiedPresignCap,
    message_centralized_signature: vector<u8>,
    is_future_sign: bool,
    ctx: &mut TxContext
): bool {
    let created_at_epoch = self.current_epoch;

    assert!(self.presign_sessions.contains(presign_cap.presign_id), EPresignNotExist);
    let presign = self.presign_sessions.remove(presign_cap.presign_id);

    let (dwallet, dwallet_public_output) = self.get_active_dwallet_and_public_output_mut(dwallet_id);

    let VerifiedPresignCap {
        id,
        dwallet_id: presign_cap_dwallet_id,
        presign_id: presign_cap_presign_id,
    } = presign_cap;

    let presign_cap_id = id.to_inner();
    id.delete();

    let PresignSession {
        id,
        created_at_epoch: _,
        dwallet_id: presign_dwallet_id,
        cap_id,
        state,
        curve,
        signature_algorithm: presign_signature_algorithm,
    } = presign;

    let presign = match(state) {
        PresignState::Completed { presign } => {
            presign
        },
        _ => abort EInvalidPresign
    };

    let presign_id = id.to_inner();
    id.delete();

    // Check that the presign is global, or that it belongs to this dWallet.
    assert!(presign_dwallet_id.is_none() || presign_dwallet_id.is_some_and!(|id| id == dwallet_id), EMessageApprovalMismatch);

    // Sanity checks: check that the IDs of the capability and presign match, and that they point to this dWallet.
    assert!(presign_cap_id == cap_id, EPresignNotExist);
    assert!(presign_id == presign_cap_presign_id, EPresignNotExist);
    assert!(presign_cap_dwallet_id == presign_dwallet_id, EPresignNotExist);

    // Check that the curve of the dWallet matches that of the presign, and that the signature algorithm matches.
    assert!(dwallet.curve == curve, EDWalletMismatch);
    assert!(presign_signature_algorithm == signature_algorithm, EMessageApprovalMismatch);

    // Emit a `SignRequestEvent` to request the Ika network to sign `message`.
    let id = object::new(ctx);
    let sign_id = id.to_inner();
    let dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;
    let emit_event = self.charge_and_create_current_epoch_dwallet_event(
        dwallet_network_encryption_key_id,
        pricing_value,
        payment_ika,
        payment_sui,
        SignRequestEvent {
            sign_id,
            dwallet_id,
            dwallet_public_output,
            curve,
            signature_algorithm,
            hash_scheme,
            message,
            dwallet_network_encryption_key_id,
            presign_id,
            presign,
            message_centralized_signature,
            is_future_sign,
        },
        ctx,
    );

    // Create a `SignSession` object and register it in `sign_sessions`.
    let session_id = emit_event.session_id;
    let dwallet = self.get_dwallet_mut(dwallet_id);
    dwallet.sign_sessions.add(sign_id, SignSession {
        id,
        created_at_epoch,
        dwallet_id,
        session_id,
        state: SignState::Requested,
    });
    let is_imported_key_dwallet = dwallet.is_imported_key_dwallet;
    self.validate_curve_and_signature_algorithm_and_hash_scheme(curve, signature_algorithm, hash_scheme);

    event::emit(emit_event);
    is_imported_key_dwallet
}

/// Initiates the Sign protocol for this dWallet.
/// Requires a `MessageApproval`, which approves a message for signing and is unpacked and deleted to ensure it is never used twice.
public(package) fun request_sign(
    self: &mut DWalletCoordinatorInner,
    message_approval: MessageApproval,
    presign_cap: VerifiedPresignCap,
    message_centralized_signature: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    let MessageApproval {
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message
    } = message_approval;

    let (dwallet, _) = self.get_active_dwallet_and_public_output(dwallet_id);

    let curve = dwallet.curve;
    let mut pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), SIGN_PROTOCOL_FLAG);
    assert!(pricing_value.is_some(), EMissingProtocolPricing);

    let is_imported_key_dwallet = self.validate_and_initiate_sign(
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message,
        presign_cap,
        message_centralized_signature,
        false,
        ctx
    );

    assert!(!is_imported_key_dwallet, EImportedKeyDWallet);
}

/// Initiates the Sign protocol for this imported key dWallet.
/// Requires an `ImportedKeyMessageApproval`, which approves a message for signing and is unpacked and deleted to ensure it is never used twice.
public(package) fun request_imported_key_sign(
    self: &mut DWalletCoordinatorInner,
    message_approval: ImportedKeyMessageApproval,
    presign_cap: VerifiedPresignCap,
    message_centralized_signature: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    let ImportedKeyMessageApproval {
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message
    } = message_approval;

    let (dwallet, _) = self.get_active_dwallet_and_public_output(dwallet_id);
    let curve = dwallet.curve;
    let mut pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), SIGN_PROTOCOL_FLAG);
    assert!(pricing_value.is_some(), EMissingProtocolPricing);

    let is_imported_key_dwallet = self.validate_and_initiate_sign(
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message,
        presign_cap,
        message_centralized_signature,
        false,
        ctx
    );

    assert!(is_imported_key_dwallet, ENotImportedKeyDWallet);
}

/// Request the Ika network verify the user-side sign protocol (in other words, that `message` is partially signed by the user),
/// without (yet) executing the network side sign-protocol.
///
/// Used for future sign use-cases, in which the user share isn't required to sign `message`;
/// instead, anyone that holds a `VerifiedPartialUserSignatureCap` capability and a `MessageApproval` can sign `message` by calling `request_sign_with_partial_user_signature()` at any time.
///
/// Creates a new `PartialUserSignature` in the `AwaitingNetworkVerification` state and registered it into `partial_centralized_signed_messages`. Moves `presign_cap` to it,
/// ensuring it can be used for anything other than signing this `message` using `request_sign_with_partial_user_signature()` (which will in turn ensure it can only be signed once).
///
/// Creates a new `UnverifiedPartialUserSignatureCap` object and returns it to the caller.
///
/// See the doc of [`PartialUserSignature`] for
/// more details on when this may be used.
public(package) fun request_future_sign(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    presign_cap: VerifiedPresignCap,
    message: vector<u8>,
    hash_scheme: u32,
    message_centralized_signature: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): UnverifiedPartialUserSignatureCap {
    // Check that the presign is global, or that it belongs to this dWallet.
    assert!(presign_cap.dwallet_id.is_none() || presign_cap.dwallet_id.is_some_and!(|id| id == dwallet_id), EMessageApprovalMismatch);

    let (dwallet, dwallet_public_output) = self.get_active_dwallet_and_public_output_mut(dwallet_id);
    let dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;
    let curve = dwallet.curve;

    assert!(self.presign_sessions.contains(presign_cap.presign_id), EPresignNotExist);

    let presign_obj = self.presign_sessions.borrow(presign_cap.presign_id);
    assert!(presign_obj.curve == curve, EDWalletMismatch);

    let presign = match(presign_obj.state) {
        PresignState::Completed { presign } => {
            presign
        },
        _ => abort EInvalidPresign
    };

    let id = object::new(ctx);
    let partial_centralized_signed_message_id = id.to_inner();
    let cap = UnverifiedPartialUserSignatureCap {
        id: object::new(ctx),
        partial_centralized_signed_message_id,
    };

    let signature_algorithm = presign_obj.signature_algorithm;

    self.validate_curve_and_signature_algorithm_and_hash_scheme(curve, signature_algorithm, hash_scheme);

    let mut pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), FUTURE_SIGN_PROTOCOL_FLAG);
    assert!(pricing_value.is_some(), EMissingProtocolPricing);
    let emit_event = self.charge_and_create_current_epoch_dwallet_event(
        dwallet_network_encryption_key_id,
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        FutureSignRequestEvent {
                dwallet_id,
                partial_centralized_signed_message_id,
                message,
                presign: presign,
                dwallet_public_output,
                curve,
                signature_algorithm,
                hash_scheme,
                message_centralized_signature,
                dwallet_network_encryption_key_id,
        },
        ctx,
    );

    // Create a new `PartialUserSignature` that wraps around `presign_cap` to ensure it can't be used twice.
    self.partial_centralized_signed_messages.add(partial_centralized_signed_message_id, PartialUserSignature {
        id: id,
        created_at_epoch: self.current_epoch,
        presign_cap,
        dwallet_id,
        cap_id: object::id(&cap),
        hash_scheme,
        message,
        message_centralized_signature,
        state: PartialUserSignatureState::AwaitingNetworkVerification,
        curve,
        signature_algorithm,
    });

    event::emit(emit_event);

    cap
}

/// Called by the Ika network to respond with the verification result of the user-side sign protocol (in other words, whether `message` is partially signed by the user).
///
/// Advances the `PartialUserSignature` state to `NetworkVerificationCompleted`.
///
/// See the doc of [`PartialUserSignature`] for
/// more details on when this may be used.
public(package) fun respond_future_sign(
    self: &mut DWalletCoordinatorInner,
    session_id: ID,
    dwallet_id: ID,
    partial_centralized_signed_message_id: ID,
    rejected: bool,
    session_sequence_number: u64
): Balance<SUI> {
    let gas_fee_reimbursement_sui = self.remove_user_initiated_session_and_charge<FutureSignRequestEvent>(session_sequence_number);
    let partial_centralized_signed_message = self.partial_centralized_signed_messages.borrow_mut(partial_centralized_signed_message_id);

    // Check that the presign is global, or that it belongs to this dWallet.
    assert!(partial_centralized_signed_message.presign_cap.dwallet_id.is_none() || partial_centralized_signed_message.presign_cap.dwallet_id.is_some_and!(|id| id == dwallet_id), EDWalletMismatch);

    partial_centralized_signed_message.state = match(partial_centralized_signed_message.state) {
        PartialUserSignatureState::AwaitingNetworkVerification => {
            if(rejected) {
                event::emit(RejectedFutureSignEvent {
                    session_id,
                    dwallet_id,
                    partial_centralized_signed_message_id
                });
                PartialUserSignatureState::NetworkVerificationRejected
            } else {
                event::emit(CompletedFutureSignEvent {
                    session_id,
                    dwallet_id,
                    partial_centralized_signed_message_id
                });
                PartialUserSignatureState::NetworkVerificationCompleted
            }
        },
        _ => abort EWrongState
    };
    gas_fee_reimbursement_sui
}

/// Checks that the partial user signature corresponding to `cap` is valid, by assuring it is in the `NetworkVerificationCompleted` state.
public(package) fun is_partial_user_signature_valid(
    self: &DWalletCoordinatorInner,
    cap: &UnverifiedPartialUserSignatureCap,
): bool {
    let partial_centralized_signed_message = self.partial_centralized_signed_messages.borrow(cap.partial_centralized_signed_message_id);
    partial_centralized_signed_message.cap_id == cap.id.to_inner() && partial_centralized_signed_message.state == PartialUserSignatureState::NetworkVerificationCompleted
}

/// Verifies that the partial user signature corresponding to `cap` is valid,
/// deleting the `UnverifiedPartialUserSignatureCap` object and returning a new `VerifiedPartialUserSignatureCap` in its place.
public(package) fun verify_partial_user_signature_cap(
    self: &mut DWalletCoordinatorInner,
    cap: UnverifiedPartialUserSignatureCap,
    ctx: &mut TxContext
): VerifiedPartialUserSignatureCap {
    let UnverifiedPartialUserSignatureCap {
        id,
        partial_centralized_signed_message_id
    } = cap;

    let cap_id = id.to_inner();
    id.delete();

    let partial_centralized_signed_message = self.partial_centralized_signed_messages.borrow_mut(partial_centralized_signed_message_id);
    assert!(partial_centralized_signed_message.cap_id == cap_id, EIncorrectCap);
    assert!(partial_centralized_signed_message.state == PartialUserSignatureState::NetworkVerificationCompleted, EUnverifiedCap);
    let cap = VerifiedPartialUserSignatureCap {
        id: object::new(ctx),
        partial_centralized_signed_message_id,
    };
    partial_centralized_signed_message.cap_id = cap.id.to_inner();
    cap
}

/// Requests the Ika network to complete the signing session on a message that was already partially-signed by the user (i.e. a message with a verified [`PartialUserSignature`]).
/// Useful is `message_approval` was only acquired after `PartialUserSignature` was created, and the caller does not own the user-share of this dWallet.
///
/// Takes the `presign_cap` from the `PartialUserSignature` object, and destroys it in `validate_and_initiate_sign()`,
/// ensuring the presign was not used for any other purpose than signing this message once.
///
/// See the doc of [`PartialUserSignature`] for
/// more details on when this may be used.
public(package) fun request_sign_with_partial_user_signature(
    self: &mut DWalletCoordinatorInner,
    partial_user_signature_cap: VerifiedPartialUserSignatureCap,
    message_approval: MessageApproval,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    // Ensure that each partial user signature has a corresponding message approval; otherwise, abort.
    let is_match = self.match_partial_user_signature_with_message_approval(&partial_user_signature_cap, &message_approval);
    assert!(is_match, EMessageApprovalMismatch);

    let VerifiedPartialUserSignatureCap {
        id,
        partial_centralized_signed_message_id,
    } = partial_user_signature_cap;
    let verified_cap_id = id.to_inner();
    id.delete();

    let PartialUserSignature {
        id,
        created_at_epoch: _,
        presign_cap,
        dwallet_id: _,
        cap_id,
        curve,
        signature_algorithm: _,
        hash_scheme: _,
        message: _,
        message_centralized_signature,
        state
    } = self.partial_centralized_signed_messages.remove(partial_centralized_signed_message_id);

    id.delete();
    assert!(cap_id == verified_cap_id && state == PartialUserSignatureState::NetworkVerificationCompleted, EIncorrectCap);

    let MessageApproval {
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message
    } = message_approval;

    let mut pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG);
    assert!(pricing_value.is_some(), EMissingProtocolPricing);

    // Emit signing events to finalize the signing process.
    let is_imported_key_dwallet = self.validate_and_initiate_sign(
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message,
        presign_cap,
        message_centralized_signature,
        true,
        ctx
    );
    assert!(!is_imported_key_dwallet, EImportedKeyDWallet);
}

/// The imported key variant of [`request_sign_with_partial_user_signature()`] (see for documentation).
public(package) fun request_imported_key_sign_with_partial_user_signature(
    self: &mut DWalletCoordinatorInner,
    partial_user_signature_cap: VerifiedPartialUserSignatureCap,
    message_approval: ImportedKeyMessageApproval,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    // Ensure that each partial user signature has a corresponding imported key message approval; otherwise, abort.
    let is_match = self.match_partial_user_signature_with_imported_key_message_approval(&partial_user_signature_cap, &message_approval);
    assert!(is_match, EMessageApprovalMismatch);

    let VerifiedPartialUserSignatureCap {
        id,
        partial_centralized_signed_message_id,
    } = partial_user_signature_cap;
    let verified_cap_id = id.to_inner();
    id.delete();

    let PartialUserSignature {
        id,
        created_at_epoch: _,
        presign_cap,
        dwallet_id: _,
        cap_id,
        curve,
        signature_algorithm: _,
        hash_scheme: _,
        message: _,
        message_centralized_signature,
        state
    } = self.partial_centralized_signed_messages.remove(partial_centralized_signed_message_id);
    id.delete();
    assert!(cap_id == verified_cap_id && state == PartialUserSignatureState::NetworkVerificationCompleted, EIncorrectCap);

    let ImportedKeyMessageApproval {
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message
    } = message_approval;

    let mut pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG);
    assert!(pricing_value.is_some(), EMissingProtocolPricing);


    // Emit signing events to finalize the signing process.
    let is_imported_key_dwallet = self.validate_and_initiate_sign(
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message,
        presign_cap,
        message_centralized_signature,
        true,
        ctx
    );
    assert!(is_imported_key_dwallet, ENotImportedKeyDWallet);
}

/// Matches partial user signature with message approval to ensure they are consistent.
/// This function can be called by the user to verify before calling
/// the `request_sign_with_partial_user_signature` function.
/// It is also called before requesting the Ika network to complete the signing.
public(package) fun match_partial_user_signature_with_message_approval(
    self: &DWalletCoordinatorInner,
    partial_user_signature_cap: &VerifiedPartialUserSignatureCap,
    message_approval: &MessageApproval,
): bool {
    let partial_signature = self.partial_centralized_signed_messages.borrow(partial_user_signature_cap.partial_centralized_signed_message_id);

    partial_signature.dwallet_id == message_approval.dwallet_id &&
    partial_signature.message == message_approval.message &&
    partial_signature.signature_algorithm == message_approval.signature_algorithm &&
    partial_signature.hash_scheme == message_approval.hash_scheme
}

/// Matches partial user signature with imported key message approval to ensure they are consistent.
/// This function can be called by the user to verify before calling
/// the `request_imported_key_sign_with_partial_user_signatures` function.
public(package) fun match_partial_user_signature_with_imported_key_message_approval(
    self: &DWalletCoordinatorInner,
    partial_user_signature_cap: &VerifiedPartialUserSignatureCap,
    message_approval: &ImportedKeyMessageApproval,
): bool {
    let partial_signature = self.partial_centralized_signed_messages.borrow(partial_user_signature_cap.partial_centralized_signed_message_id);

    partial_signature.dwallet_id == message_approval.dwallet_id &&
    partial_signature.message == message_approval.message &&
    partial_signature.signature_algorithm == message_approval.signature_algorithm &&
    partial_signature.hash_scheme == message_approval.hash_scheme
}

/// Called by the Ika network to respond to (and complete) a Sign protocol request.
///
/// Sets the `SignSession` to `Completed` and stores in it the `signature`.
/// Also emits an event with the `signature`.
public(package) fun respond_sign(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    sign_id: ID,
    session_id: ID,
    signature: vector<u8>,
    is_future_sign: bool,
    rejected: bool,
    session_sequence_number: u64
): Balance<SUI> {
    let gas_fee_reimbursement_sui = self.remove_user_initiated_session_and_charge<SignRequestEvent>(session_sequence_number);
    let (dwallet, _) = self.get_active_dwallet_and_public_output_mut(dwallet_id);

    let sign = dwallet.sign_sessions.borrow_mut(sign_id);

    sign.state = match(sign.state) {
        SignState::Requested => {
            if(rejected) {
                event::emit(RejectedSignEvent {
                    sign_id,
                    session_id,
                    is_future_sign,
                });
                SignState::NetworkRejected
            } else {
                event::emit(CompletedSignEvent {
                    sign_id,
                    session_id,
                    signature,
                    is_future_sign,
                });
                SignState::Completed { signature }
            }
        },
        _ => abort ESignWrongState
    };
    gas_fee_reimbursement_sui
}

/// Processes a checkpoint message that has been signed by a validator quorum.
/// 
/// Verifies the BLS multi-signature from the active validator committee before
/// processing the checkpoint contents. This ensures only valid, consensus-approved
/// checkpoints are processed.
/// 
/// ### Parameters
/// - `self`: Mutable reference to the coordinator
/// - `signature`: BLS multi-signature from validators
/// - `signers_bitmap`: Bitmap indicating which validators signed
/// - `message`: Checkpoint message content to process
/// - `ctx`: Transaction context for coin creation
/// 
/// ### Returns
/// SUI coin containing gas fee reimbursements from processed operations
/// 
/// ### Effects
/// - Verifies the signature against the active committee
/// - Processes all operations contained in the checkpoint
/// - Updates session states and emits relevant events
/// - Collects and returns gas fee reimbursements
/// 
/// ### Aborts
/// - BLS verification errors if signature is invalid
/// - Various operation-specific errors during checkpoint processing
public(package) fun process_checkpoint_message_by_quorum(
    self: &mut DWalletCoordinatorInner,
    signature: vector<u8>,
    signers_bitmap: vector<u8>,
    message: vector<u8>,
    ctx: &mut TxContext,
): Coin<SUI> {
    let mut intent_bytes = CHECKPOINT_MESSAGE_INTENT;
    intent_bytes.append(message);
    intent_bytes.append(bcs::to_bytes(&self.current_epoch));

    self.active_committee.verify_certificate(self.current_epoch, &signature, &signers_bitmap, &intent_bytes);

    self.process_checkpoint_message(message, ctx)
}

fun process_checkpoint_message(
    self: &mut DWalletCoordinatorInner,
    message: vector<u8>,
    ctx: &mut TxContext,
): Coin<SUI> {
    assert!(!self.active_committee.members().is_empty(), EActiveBlsCommitteeMustInitialize);

    let mut bcs_body = bcs::new(copy message);

    let epoch = bcs_body.peel_u64();
    assert!(epoch == self.current_epoch, EIncorrectEpochInCheckpoint);

    let sequence_number = bcs_body.peel_u64();

    if(self.last_processed_checkpoint_sequence_number.is_none()) {
        assert!(sequence_number == 0, EWrongCheckpointSequenceNumber);
        self.last_processed_checkpoint_sequence_number.fill(sequence_number);
    } else {
        assert!(sequence_number > 0 && *self.last_processed_checkpoint_sequence_number.borrow() + 1 == sequence_number, EWrongCheckpointSequenceNumber);
        self.last_processed_checkpoint_sequence_number.swap(sequence_number);
    };

    let timestamp_ms = bcs_body.peel_u64();

    event::emit(DWalletCheckpointInfoEvent {
        epoch,
        sequence_number,
        timestamp_ms,
    });

    let len = bcs_body.peel_vec_length();
    let mut i = 0;
    let mut total_gas_fee_reimbursement_sui = balance::zero();
    while (i < len) {
        let message_data_type = bcs_body.peel_vec_length();
        // Parses checkpoint BCS bytes directly.
        // Messages with `message_data_type` 1 & 2 are handled by the system module,
        // but their bytes must be extracted here to allow correct parsing of types 3 and above.
        // This step only extracts the bytes without further processing.
        match (message_data_type) {
            RESPOND_DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE => {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let first_round_output = bcs_body.peel_vec_u8();
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                let gas_fee_reimbursement_sui = self.respond_dwallet_dkg_first_round(dwallet_id, first_round_output, rejected, session_sequence_number);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            RESPOND_DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE => {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_output = bcs_body.peel_vec_u8();
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                let gas_fee_reimbursement_sui = self.respond_dwallet_dkg_second_round(
                    dwallet_id,
                    public_output,
                    encrypted_user_secret_key_share_id,
                    session_id,
                    rejected,
                    session_sequence_number,
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            RESPOND_DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE => {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                let gas_fee_reimbursement_sui = self.respond_re_encrypt_user_share_for(
                    dwallet_id,
                    encrypted_user_secret_key_share_id,
                    rejected,
                    session_sequence_number,
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            RESPOND_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE => {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_user_secret_key_shares = bcs_body.peel_vec_u8();
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                let gas_fee_reimbursement_sui = self.respond_make_dwallet_user_secret_key_share_public(dwallet_id, public_user_secret_key_shares, rejected, session_sequence_number);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            RESPOND_DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE => {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_output = bcs_body.peel_vec_u8();
                let encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                let gas_fee_reimbursement_sui = self.respond_imported_key_dwallet_verification(
                    dwallet_id,
                    public_output,
                    encrypted_user_secret_key_share_id,
                    session_id,
                    rejected,
                    session_sequence_number
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            RESPOND_DWALLET_PRESIGN_MESSAGE_TYPE => {
                let dwallet_id = bcs_body.peel_option!(|bcs_option| object::id_from_bytes(bcs_option.peel_vec_u8()));
                let presign_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let presign = bcs_body.peel_vec_u8();
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                let gas_fee_reimbursement_sui = self.respond_presign(
                    dwallet_id,
                    presign_id,
                    session_id,
                    presign,
                    rejected,
                    session_sequence_number);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            RESPOND_DWALLET_SIGN_MESSAGE_TYPE => {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let sign_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let signature = bcs_body.peel_vec_u8();
                let is_future_sign = bcs_body.peel_bool();
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                let gas_fee_reimbursement_sui = self.respond_sign(
                    dwallet_id,
                    sign_id,
                    session_id,
                    signature,
                    is_future_sign,
                    rejected,
                    session_sequence_number
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            RESPOND_DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE => {
                let session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let partial_centralized_signed_message_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                let gas_fee_reimbursement_sui = self.respond_future_sign(
                    session_id,
                    dwallet_id,
                    partial_centralized_signed_message_id,
                    rejected,
                    session_sequence_number
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            RESPOND_DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE => {
                let dwallet_network_encryption_key_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_output = bcs_body.peel_vec_u8();
                let is_last = bcs_body.peel_bool();
                let rejected = bcs_body.peel_bool();
                let gas_fee_reimbursement_sui = self.respond_dwallet_network_encryption_key_dkg(dwallet_network_encryption_key_id, public_output, is_last, rejected, ctx);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            RESPOND_DWALLET_MPC_NETWORK_RECONFIGURATION_OUTPUT_MESSAGE_TYPE => {
                let dwallet_network_encryption_key_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_output = bcs_body.peel_vec_u8();
                let is_last = bcs_body.peel_bool();
                let rejected = bcs_body.peel_bool();
                let gas_fee_reimbursement_sui = self.respond_dwallet_network_encryption_key_reconfiguration(dwallet_network_encryption_key_id, public_output, is_last, rejected, ctx);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            SET_MAX_ACTIVE_SESSIONS_BUFFER_MESSAGE_TYPE => {
                let max_active_sessions_buffer = bcs_body.peel_u64();
                self.set_max_active_sessions_buffer(max_active_sessions_buffer);
            },
            SET_GAS_FEE_REIMBURSEMENT_SUI_SYSTEM_CALL_VALUE_MESSAGE_TYPE => {
                let gas_fee_reimbursement_sui_system_call_value = bcs_body.peel_u64();
                self.set_gas_fee_reimbursement_sui_system_call_value(gas_fee_reimbursement_sui_system_call_value);
            },
            _ => {},
        };
        i = i + 1;
    };
    self.total_messages_processed = self.total_messages_processed + i;
    total_gas_fee_reimbursement_sui.into_coin(ctx)
}

fun set_max_active_sessions_buffer(
    self: &mut DWalletCoordinatorInner,
    max_active_sessions_buffer: u64,
) {
    self.session_management.max_active_sessions_buffer = max_active_sessions_buffer;
    event::emit(SetMaxActiveSessionsBufferEvent {
        max_active_sessions_buffer
    });
}

fun set_gas_fee_reimbursement_sui_system_call_value(
    self: &mut DWalletCoordinatorInner,
    gas_fee_reimbursement_sui_system_call_value: u64,
) {
    self.pricing_and_fee_management.gas_fee_reimbursement_sui_system_call_value = gas_fee_reimbursement_sui_system_call_value;
    event::emit(SetGasFeeReimbursementSuiSystemCallValueEvent {
        gas_fee_reimbursement_sui_system_call_value
    });
}

/// Sets the supported curves, signature algorithms and hash schemes, and the default pricing.
/// 
/// This function is used to set the supported curves, signature algorithms and hash schemes, and the default pricing.
/// Default pricing is used to set the pricing for a protocol or curve if pricing is missing for a protocol or curve
/// and it has to contain the default pricing for all protocols and curves as set in the `supported_curves_to_signature_algorithms_to_hash_schemes` parameter.
/// 
/// ### Parameters
/// - **`default_pricing`**: The default pricing to use if pricing is missing for a protocol or curve.
/// - **`supported_curves_to_signature_algorithms_to_hash_schemes`**: A map of curves to signature algorithms to hash schemes.
///
/// ### Errors
/// - **`EMissingProtocolPricing`**: If pricing is missing for any protocol or curve.
public(package) fun set_supported_and_pricing(
    self: &mut DWalletCoordinatorInner,
    default_pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap<u32, VecMap<u32, vector<u32>>>,
) {
    verify_pricing_exists_for_all_protocols(&supported_curves_to_signature_algorithms_to_hash_schemes, &default_pricing);
    self.pricing_and_fee_management.default = default_pricing;
    self.support_config.supported_curves_to_signature_algorithms_to_hash_schemes = supported_curves_to_signature_algorithms_to_hash_schemes;
}

/// Verifies that pricing exists for all protocols for all curves.
/// Aborts if pricing is missing for any protocol or curve.
/// IMPORTANT: every time a new protocol is added, this function must be updated with verifying the new protocol pricing.
///
/// ### Parameters
/// - **`supported_curves_to_signature_algorithms_to_hash_schemes`**: A map of curves to signature algorithms to hash schemes.
/// - **`default_pricing`**: The default pricing to use if pricing is missing for a protocol or curve.
///
/// ### Errors
/// - **`EMissingProtocolPricing`**: If pricing is missing for any protocol or curve.
fun verify_pricing_exists_for_all_protocols(supported_curves_to_signature_algorithms_to_hash_schemes: &VecMap<u32, VecMap<u32, vector<u32>>>, default_pricing: &DWalletPricing) {
    let curves = supported_curves_to_signature_algorithms_to_hash_schemes.keys();
    curves.do_ref!(|curve| {
        let mut is_missing_pricing = false;
        let signature_algorithms = &supported_curves_to_signature_algorithms_to_hash_schemes[curve];
        let signature_algorithms = signature_algorithms.keys();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::none(), DKG_FIRST_ROUND_PROTOCOL_FLAG).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::none(), DKG_SECOND_ROUND_PROTOCOL_FLAG).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::none(), RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::none(), MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::none(), IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG).is_none();
        // Add here pricing validation for new protocols per curve.
        signature_algorithms.do_ref!(|signature_algorithm| {
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::some(*signature_algorithm), PRESIGN_PROTOCOL_FLAG).is_none();
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::some(*signature_algorithm), SIGN_PROTOCOL_FLAG).is_none();
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::some(*signature_algorithm), FUTURE_SIGN_PROTOCOL_FLAG).is_none();
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::some(*signature_algorithm), SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG).is_none();
            // Add here pricing validation for new protocols per curve per signature algorithm.
        });
        assert!(!is_missing_pricing, EMissingProtocolPricing);
    });
}

/// Sets the paused curves, signature algorithms and hash schemes.
/// 
/// This function is used to set the paused curves, signature algorithms and hash schemes.
/// 
/// ### Parameters
/// - **`paused_curves`**: The curves to pause.
/// - **`paused_signature_algorithms`**: The signature algorithms to pause.
/// - **`paused_hash_schemes`**: The hash schemes to pause.
public(package) fun set_paused_curves_and_signature_algorithms(
    self: &mut DWalletCoordinatorInner,
    paused_curves: vector<u32>,
    paused_signature_algorithms: vector<u32>,
    paused_hash_schemes: vector<u32>,
) {
    self.support_config.paused_curves = paused_curves;
    self.support_config.paused_signature_algorithms = paused_signature_algorithms;
    self.support_config.paused_hash_schemes = paused_hash_schemes;
}

/// Sets the pricing vote for a validator.
/// 
/// This function is used to set the pricing vote for a validator.
/// Cannot be called during the votes calculation.
/// 
/// ### Parameters
/// - **`validator_id`**: The ID of the validator.
/// - **`pricing_vote`**: The pricing vote for the validator.
///
/// ### Errors
/// - **`ECannotSetDuringVotesCalculation`**: If the pricing vote is set during the votes calculation.
public(package) fun set_pricing_vote(
    self: &mut DWalletCoordinatorInner,
    validator_id: ID,
    pricing_vote: DWalletPricing,
) {
    assert!(self.pricing_and_fee_management.calculation_votes.is_none(), ECannotSetDuringVotesCalculation);
    if(self.pricing_and_fee_management.validator_votes.contains(validator_id)) {
        let vote = self.pricing_and_fee_management.validator_votes.borrow_mut(validator_id);
        *vote = pricing_vote;
    } else {
        self.pricing_and_fee_management.validator_votes.add(validator_id, pricing_vote);
    }
}

public(package) fun subsidize_coordinator_with_sui(
    self: &mut DWalletCoordinatorInner,
    sui: Coin<SUI>,
) {
    self.pricing_and_fee_management.gas_fee_reimbursement_sui.join(sui.into_balance());
}

public(package) fun subsidize_coordinator_with_ika(
    self: &mut DWalletCoordinatorInner,
    ika: Coin<IKA>,
) {
    self.pricing_and_fee_management.consensus_validation_fee_charged_ika.join(ika.into_balance());
}

public(package) fun dwallet_network_encryption_key_id(self: &DWalletNetworkEncryptionKeyCap): ID {
    self.dwallet_network_encryption_key_id
}

public(package) fun current_pricing(self: &DWalletCoordinatorInner): DWalletPricing {
    self.pricing_and_fee_management.current
}

/// === Public Functions ===

public fun dwallet_id(self: &DWalletCap): ID {
    self.dwallet_id
}

public fun imported_key_dwallet_id(self: &ImportedKeyDWalletCap): ID {
    self.dwallet_id
}

// === Test Functions ===

#[test_only]
public fun last_processed_checkpoint_sequence_number(
    self: &DWalletCoordinatorInner,
): Option<u64> {
    self.last_processed_checkpoint_sequence_number
}

#[test_only]
public(package) fun last_session_sequence_number(self: &DWalletCoordinatorInner): u64 {
    self.session_management.next_session_sequence_number - 1
}