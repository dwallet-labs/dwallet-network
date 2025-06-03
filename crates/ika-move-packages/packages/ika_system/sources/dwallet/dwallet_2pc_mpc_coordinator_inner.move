// Copyright (c) dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// This module handles the logic for creating and managing dWallets using the Secp256K1 signature scheme
/// and the DKG process. It leverages validators to execute MPC (Multi-Party Computation)
/// protocols to ensure trustless and decentralized wallet creation and key management.

module ika_system::dwallet_2pc_mpc_coordinator_inner;

use sui::table_vec::{Self, TableVec};
use ika::ika::IKA;
use sui::sui::SUI;
use sui::object_table::{Self, ObjectTable};
use sui::table::{Self, Table};
use sui::balance::{Self, Balance};
use sui::bcs;
use sui::coin::{Coin};
use sui::bag::{Self, Bag};
use sui::event;
use sui::ed25519::ed25519_verify;
use ika_system::address;
use ika_system::bls_committee::{Self, BlsCommittee};
use sui::vec_map::{VecMap};
use sui::event::emit;
use ika_system::dwallet_pricing::{Self, DWalletPricing, DWalletPricingValue, DWalletPricingCalculationVotes};

const CHECKPOINT_MESSAGE_INTENT: vector<u8> = vector[1, 0, 0];

const DKG_FIRST_ROUND_PROTOCOL_FLAG: u32 = 0;
const DKG_SECOND_ROUND_PROTOCOL_FLAG: u32 = 1;
const RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG: u32 = 2;
const MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG: u32 = 3;
const IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG: u32 = 4;
const PRESIGN_PROTOCOL_FLAG: u32 = 5;
const SIGN_PROTOCOL_FLAG: u32 = 6;
const FUTURE_SIGN_PROTOCOL_FLAG: u32 = 7;
const SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG: u32 = 8;

// Message data type constants corresponding to MessageKind enum variants (in ika-types/src/message.rs)
const DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE: u64 = 0;
const DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE: u64 = 1;
const DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE: u64 = 2;
const DWALLET_SIGN_MESSAGE_TYPE: u64 = 3;
const DWALLET_PRESIGN_MESSAGE_TYPE: u64 = 4;
const DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE: u64 = 5;
const DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE: u64 = 6;
const DWALLET_MPC_NETWORK_RESHARE_OUTPUT_MESSAGE_TYPE: u64 = 7;
const MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE: u64 = 8;
const DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE: u64 = 9;
const SET_MAX_ACTIVE_SESSIONS_BUFFER_MESSAGE_TYPE: u64 = 10;
const SET_GAS_FEE_REIMBURSEMENT_SUI_SYSTEM_CALL_VALUE_MESSAGE_TYPE: u64 = 11;

public(package) fun lock_last_active_session_sequence_number(self: &mut DWalletCoordinatorInner) {
    self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch = true;
}

/// Session management data for the dWallet coordinator.
public struct SessionManagement has store {
    sessions: ObjectTable<u64, DWalletSession>,
    // Holds events keyed by the ID of the corresponding `DWalletSession` session.
    user_requested_sessions_events: Bag,
    number_of_completed_user_initiated_sessions: u64,
    started_system_sessions_count: u64,
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
    /// A list of paused curves in case of emergency.
    /// e.g. [secp256k1, ristretto]
    paused_curves: vector<u32>,
    /// A list of paused signature algorithms in case of emergency.
    /// e.g. [ecdsa, schnorr]
    paused_signature_algorithms: vector<u32>,
    /// A list of paused hash schemes in case of emergency.
    /// e.g. [sha256, keccak256]
    paused_hash_schemes: vector<u32>,
    /// A list of signature algorithms that are allowed for global presign.
    signature_algorithms_allowed_global_presign: vector<u32>,
}

/// Pricing and fee management data for the dWallet coordinator.
public struct PricingAndFeeManagement has store {
    /// The pricing for the current epoch.
    current: DWalletPricing,
    /// The default pricing.
    default: DWalletPricing,
    /// The votes for the pricing set by validators.
    /// The key is the validator ID to their votes.
    validator_votes: Table<ID, DWalletPricing>,
    /// The votes for the pricing calculation, if set, we have to complete the pricing
    /// calculation before we advance to the next epoch.
    calculation_votes: Option<DWalletPricingCalculationVotes>,
    /// The value of the gas fee reimbursement for system calls.
    gas_fee_reimbursement_sui_system_call_value: u64,
    /// Sui gas fee reimbursement to fund the network writing tx responses to sui.
    gas_fee_reimbursement_sui: Balance<SUI>,
    /// The fees paid for consensus validation in IKA.
    consensus_validation_fee_charged_ika: Balance<IKA>,
}

/// A shared object that holds all the Ika system object used to manage dWallets:
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
/// information regarding `pricing`, all the `sessions` and the `next_session_sequence_number` that will be used for the next session,
/// and various other fields, like the supported and paused curves, signing algorithms and hashes.
public struct DWalletCoordinatorInner has store {
    current_epoch: u64,
    session_management: SessionManagement,
    // TODO: change it to versioned
    /// The key is the ID of `DWallet`.
    dwallets: ObjectTable<ID, DWallet>,
    // TODO: change it to versioned
    /// The key is the ID of `DWalletNetworkEncryptionKey`.
    dwallet_network_encryption_keys: ObjectTable<ID, DWalletNetworkEncryptionKey>,
    // TODO: change it to versioned
    /// A table mapping user addresses to encryption key object IDs.
    encryption_keys: ObjectTable<address, EncryptionKey>,
    /// A table mapping id to their presign sessions.
    presign_sessions: ObjectTable<ID, PresignSession>,
    /// A table mapping id to their partial centralized signed messages.
    partial_centralized_signed_messages: ObjectTable<ID, PartialUserSignature>,
    /// Pricing and fee management data.
    pricing_and_fee_management: PricingAndFeeManagement,
    /// The active committees.
    active_committee: BlsCommittee,
    /// The previous committee.
    previous_committee: BlsCommittee,
    /// The total messages processed.
    total_messages_processed: u64,
    /// The last checkpoint sequence number processed.
    last_processed_checkpoint_sequence_number: Option<u64>,
    /// The last checkpoint sequence number processed in the previous epoch.
    previous_epoch_last_checkpoint_sequence_number: u64,
    /// Support data for curves, algorithms, and their configurations.
    support_config: SupportConfig,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

public struct DWalletSessionEventKey has copy, drop, store {}

/// An Ika MPC session.
public struct DWalletSession has key, store {
    id: UID,

    session_sequence_number: u64,

    dwallet_network_encryption_key_id: ID,

    /// The fees paid for consensus validation in IKA.
    consensus_validation_fee_charged_ika: Balance<IKA>,

    /// The fees paid for computation in IKA.
    computation_fee_charged_ika: Balance<IKA>,

    /// Sui gas fee reimbursement to fund the network writing tx responses to sui.
    gas_fee_reimbursement_sui: Balance<SUI>,
}

/// Represents a capability granting control over a specific dWallet.
public struct DWalletCap has key, store {
    id: UID,
    dwallet_id: ID,
}

/// Represents a capability granting control over a specific imported key dWallet.
public struct ImportedKeyDWalletCap has key, store {
    id: UID,
    dwallet_id: ID,
}

/// Represents a capability granting control over a specific dWallet network encryption key.
public struct DWalletNetworkEncryptionKeyCap has key, store {
    id: UID,
    dwallet_network_encryption_key_id: ID,
}

/// `DWalletNetworkEncryptionKey` represents a (threshold) encryption key owned by the network.
/// It stores the `network_dkg_public_output`, which in turn stores the encryption key itself (divided to chunks, due to space limitations).
/// Before the first reconfiguration (which happens at every epoch switch,)
/// `network_dkg_public_output` also holds the encryption of the current encryption key shares
/// (encrypted to each validator's encryption key, and decrypted by them whenever they start)
/// and the public verification keys of all validators, from which the public parameters of the threshold encryption scheme
/// can be generated.
/// After the first reconfiguration, `reconfiguration_public_outputs` holds this information updated for the `current_epoch`.
public struct DWalletNetworkEncryptionKey has key, store {
    id: UID,
    dwallet_network_encryption_key_cap_id: ID,
    current_epoch: u64,
    reconfiguration_public_outputs: sui::table::Table<u64, TableVec<vector<u8>>>,
    network_dkg_public_output: TableVec<vector<u8>>,
    /// The fees paid for computation in IKA.
    computation_fee_charged_ika: Balance<IKA>,
    state: DWalletNetworkEncryptionKeyState,
}

public enum DWalletNetworkEncryptionKeyState has copy, drop, store {
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

/// Represents an encryption key used to encrypt a dWallet centralized (user) secret key share.
///
/// Encryption keys facilitate secure data transfer between accounts on the
/// Ika by ensuring that sensitive information remains confidential during transmission.
///
/// Each address on the Ika is associated with a unique encryption key.
/// When a user intends to send encrypted data (i.e. when sharing the secret key share to grant access and/or transfer a dWallet) to another user,
/// they use the recipient's encryption key to encrypt the data.
/// The recipient is then the sole entity capable of decrypting and accessing this information, ensuring secure, end-to-end encryption.
public struct EncryptionKey has key, store {
    /// Unique identifier for the `EncryptionKey`.
    id: UID,

    created_at_epoch: u64,

    curve: u32,

    //TODO: make sure to include class group type and version inside the bytes with the rust code
    /// Serialized encryption key.
    encryption_key: vector<u8>,

    /// Signature for the encryption key, signed by the `signer_public_key`.
    /// Used to verify the data originated from the `signer_address`.
    encryption_key_signature: vector<u8>,

    /// The public key that was used to sign the `encryption_key`.
    signer_public_key: vector<u8>,

    /// Address of the encryption key owner.
    signer_address: address,
}

/// A verified Encrypted dWallet centralized secret key share.
///
/// This struct represents an encrypted centralized secret key share tied to
/// a specific dWallet (`DWallet`).
/// It includes cryptographic proof that the encryption is valid and securely linked
/// to the associated `dWallet`.
public struct EncryptedUserSecretKeyShare has key, store {
    /// A unique identifier for this encrypted user share object.
    id: UID,

    created_at_epoch: u64,

    /// The ID of the dWallet associated with this encrypted secret share.
    dwallet_id: ID,

    // TODO(@Omer): once we verify the proof, I don't see a need to save it. In fact, I modified the code to not return the proof after verification, just the encryption.
    /// The encrypted centralized secret key share along with a cryptographic proof
    /// that the encryption corresponds to the dWallet's secret key share.
    encrypted_centralized_secret_share_and_proof: vector<u8>,

    /// The ID of the `EncryptionKey` object used to encrypt the secret share.
    encryption_key_id: ID,

    encryption_key_address: address,

    /// The ID of the `EncryptedUserSecretKeyShare` the secret was re-encrypted from (None if created during dkg).
    source_encrypted_user_secret_key_share_id: Option<ID>,

    state: EncryptedUserSecretKeyShareState,
}
public enum EncryptedUserSecretKeyShareState has copy, drop, store {
    AwaitingNetworkVerification,
    NetworkVerificationCompleted,
    NetworkVerificationRejected,
    KeyHolderSigned {
        /// The signed public share corresponding to the encrypted secret key share,
        /// used to verify its authenticity.
        user_output_signature: vector<u8>,
    }
}

public struct UnverifiedPartialUserSignatureCap has key, store {
    /// A unique identifier for this object.
    id: UID,

    /// The unique identifier of the associated PartialCentralizedSignedMessage.
    partial_centralized_signed_message_id: ID,
}

public struct VerifiedPartialUserSignatureCap has key, store {
    /// A unique identifier for this object.
    id: UID,

    /// The unique identifier of the associated PartialCentralizedSignedMessage.
    partial_centralized_signed_message_id: ID,
}

// TODO: add hash_scheme
/// Message that have been signed by a user, a.k.a the centralized party,
/// but not yet by the blockchain.
/// Used for scenarios where the user needs to first agree to sign some transaction,
/// and the blockchain signs this transaction later,
/// when some other conditions are met.
///
/// Can be used to implement an order-book-based exchange, for example.
/// User `A` first agrees to buy BTC with ETH at price X, and signs a transaction with this information.
/// When a matching user `B`, that agrees to sell BTC for ETH at price X,
/// signs a transaction with this information,
/// the blockchain can sign both transactions, and the exchange is completed.
public struct PartialUserSignature has key, store {
    /// A unique identifier for this object.
    id: UID,

    created_at_epoch: u64,

    presign_cap: VerifiedPresignCap,

    dwallet_id: ID,

    cap_id: ID,

    curve: u32,

    signature_algorithm: u32,

    hash_scheme: u32,

    /// The messages that are being signed.
    message: vector<u8>,

    /// The centralized party signature of a message.
    message_centralized_signature: vector<u8>,

    state: PartialUserSignatureState,
}

public enum PartialUserSignatureState has copy, drop, store {
    AwaitingNetworkVerification,
    NetworkVerificationCompleted,
    NetworkVerificationRejected
}

/// `DWallet` represents a decentralized wallet (dWallet) that is
/// created after the Distributed key generation (DKG) process.
public struct DWallet has key, store {
    /// Unique identifier for the dWallet.
    id: UID,

    created_at_epoch: u64,

    /// The elliptic curve used for the dWallet.
    curve: u32,

    /// If not set, the user secret key shares is not public, and the user will need to
    /// keep it encrypted using encrypted user secret key shares. It is
    /// the case where we have zero trust for the dWallet because the
    /// user participation is required.
    /// If set, the user secret key shares is public, the network can sign
    /// without the user participation. In this case, it is trust minimalized
    /// security for the user.
    public_user_secret_key_share: Option<vector<u8>>,

    /// The ID of the capability associated with this dWallet.
    dwallet_cap_id: ID,

    /// The MPC network encryption key id that is used to encrypt this dWallet network secret key share.
    dwallet_network_encryption_key_id: ID,

    /// Key was imported.
    is_imported_key_dwallet: bool,

    /// A table mapping id to their encryption key object.
    encrypted_user_secret_key_shares: ObjectTable<ID, EncryptedUserSecretKeyShare>,

    sign_sessions: ObjectTable<ID, SignSession>,

    state: DWalletState,
}

public enum DWalletState has copy, drop, store {
    // DKG
    DKGRequested,
    NetworkRejectedDKGRequest,
    AwaitingUserDKGVerificationInitiation {
        first_round_output: vector<u8>,
    },
    AwaitingNetworkDKGVerification,
    NetworkRejectedDKGVerification,

    // Imported Key
    AwaitingUserImportedKeyInitiation,
    AwaitingNetworkImportedKeyVerification,
    NetworkRejectedImportedKeyVerification,

    AwaitingKeyHolderSignature {
        public_output: vector<u8>,
    },

    // Active for both DKG and Imported Key
    Active {
        /// The output of the DKG process.
        public_output: vector<u8>,
    }
}

public struct UnverifiedPresignCap has key, store {
    id: UID,

    /// The ID of the dWallet for which this Presign has been created and can be used by exclusively, if set.
    /// Optional, since some key signature algorithms (e.g., Schnorr and EdDSA) can support global presigns,
    /// which can be used for any dWallet (under the same network key). Others, like ECDSA, must have this set.
    dwallet_id: Option<ID>,

    /// The ID of the presign.
    presign_id: ID,
}

public struct VerifiedPresignCap has key, store {
    id: UID,

    /// The ID of the dWallet for which this Presign has been created and can be used by exclusively, if set.
    /// Optional, since some key signature algorithms (e.g., Schnorr and EdDSA) can support global presigns,
    /// which can be used for any dWallet (under the same network key). Others, like ECDSA, must have this set.
    dwallet_id: Option<ID>,

    /// The ID of the presign.
    presign_id: ID,
}

/// A session of the Presign protocol.
/// When `state` is `PresignState::Completed`, holds a presign:
/// a single-use precomputation that does not depend on the message,
/// used to speed up the (online) Sign protocol.
public struct PresignSession has key, store {
    /// Unique identifier for the presign object.
    id: UID,

    created_at_epoch: u64,

    /// The elliptic curve used for the dWallet.
    curve: u32,

    /// The signature algorithm for the presign.
    signature_algorithm: u32,

    /// The ID of the dWallet for which this Presign has been created and can be used by exclusively, if set.
    /// Optional, since some key signature algorithms (e.g., Schnorr and EdDSA) can support global presigns,
    /// which can be used for any dWallet (under the same network key).
    dwallet_id: Option<ID>,

    cap_id: ID,

    state: PresignState,
}

public enum PresignState has copy, drop, store {
    Requested,
    NetworkRejected,
    Completed {
        presign: vector<u8>,
    }
}

/// A Sign session. When `state` is `SignState::Completed`, holds the `signature`.
public struct SignSession has key, store {
    id: UID,

    created_at_epoch: u64,

    /// The unique identifier of the associated dWallet.
    dwallet_id: ID,

    /// The session identifier for the sign process.
    session_id: ID,

    state: SignState,
}

public enum SignState has copy, drop, store {
    Requested,
    NetworkRejected,
    Completed {
        signature: vector<u8>,
    }
}

/// The dWallet MPC session type
/// User initiated sessions have a sequence number, which is used to determine in which epoch
/// the session will get completed.
/// System sessions are guaranteed to always get completed in the epoch they were created in.
public enum SessionType has copy, drop, store {
    User {
        sequence_number: u64,
    },
    System
}

public struct DWalletEvent<E: copy + drop + store> has copy, drop, store {
    epoch: u64,
    session_type: SessionType,
    session_id: ID,
    event_data: E,
}

/// Event emitted when an encryption key is created.
///
/// This event is emitted after the blockchain verifies the encryption key's validity
/// and creates the corresponding `EncryptionKey` object.
public struct CreatedEncryptionKeyEvent has copy, drop, store {
    /// The unique identifier of the created `EncryptionKey` object.
    encryption_key_id: ID,

    signer_address: address,
}

public struct DWalletNetworkDKGEncryptionKeyRequestEvent has copy, drop, store {
    dwallet_network_encryption_key_id: ID,
}


/// An event emitted when the first round of the DKG process is completed.
///
/// This event is emitted by the blockchain to notify the user about
/// the completion of the first round.
/// The user should catch this event to generate inputs for
/// the second round and call the `request_dwallet_dkg_second_round()` function.
public struct CompletedDWalletNetworkDKGEncryptionKeyEvent has copy, drop, store {
    dwallet_network_encryption_key_id: ID,
}

public struct RejectedDWalletNetworkDKGEncryptionKeyEvent has copy, drop, store {
    dwallet_network_encryption_key_id: ID,
}

public struct DWalletEncryptionKeyReconfigurationRequestEvent has copy, drop, store {
    dwallet_network_encryption_key_id: ID,
}

public struct CompletedDWalletEncryptionKeyReconfigurationEvent has copy, drop, store {
    dwallet_network_encryption_key_id: ID,
}

public struct RejectedDWalletEncryptionKeyReconfigurationEvent has copy, drop, store {
    dwallet_network_encryption_key_id: ID,
}

// DKG TYPES

/// Event emitted to start the first round of the DKG process.
///
/// This event is caught by the blockchain, which is then using it to
/// initiate the first round of the DKG.
public struct DWalletDKGFirstRoundRequestEvent has copy, drop, store {
    /// The unique session identifier for the DKG process.
    dwallet_id: ID,

    /// The identifier for the dWallet capability.
    dwallet_cap_id: ID,

    /// The MPC network encryption key id that is used to encrypt associated dWallet network secret key share.
    dwallet_network_encryption_key_id: ID,

    /// The elliptic curve used for the dWallet.
    curve: u32,
}

/// An event emitted when the first round of the DKG process is completed.
///
/// This event is emitted by the blockchain to notify the user about
/// the completion of the first round.
/// The user should catch this event to generate inputs for
/// the second round and call the `request_dwallet_dkg_second_round()` function.
public struct CompletedDWalletDKGFirstRoundEvent has copy, drop, store {
    /// The unique session identifier for the DKG process.
    dwallet_id: ID,

    /// The decentralized public output data produced by the first round of the DKG process.
    first_round_output: vector<u8>,
}

public struct RejectedDWalletDKGFirstRoundEvent has copy, drop, store {
    dwallet_id: ID,
}

/// Event emitted to initiate the second round of the DKG process.
///
/// This event is emitted to notify Validators to begin the second round of the DKG.
/// It contains all necessary data to ensure proper continuation of the process.
public struct DWalletDKGSecondRoundRequestEvent has copy, drop, store {
    encrypted_user_secret_key_share_id: ID,
    /// The unique session identifier for the DWallet.
    dwallet_id: ID,

    /// The output from the first round of the DKG process.
    first_round_output: vector<u8>,

    /// A serialized vector containing the centralized public key share and its proof.
    centralized_public_key_share_and_proof: vector<u8>,

    /// The unique identifier of the dWallet capability associated with this session.
    dwallet_cap_id: ID,

    /// Encrypted centralized secret key share and the associated cryptographic proof of encryption.
    encrypted_centralized_secret_share_and_proof: vector<u8>,

    /// The `EncryptionKey` object used for encrypting the secret key share.
    encryption_key: vector<u8>,

    /// The unique identifier of the `EncryptionKey` object.
    encryption_key_id: ID,

    encryption_key_address: address,

    /// The public output of the centralized party in the DKG process.
    user_public_output: vector<u8>,

    /// The Ed25519 public key of the initiator,
    /// used to verify the signature on the centralized public output.
    signer_public_key: vector<u8>,

    /// The MPC network encryption key id that is used to encrypt associated dWallet network secret key share.
    dwallet_network_encryption_key_id: ID,

    /// The elliptic curve used for the dWallet.
    curve: u32,
}

/// Event emitted upon the completion of the second (and final) round of the
/// Distributed Key Generation (DKG).
///
/// This event provides all necessary data generated from the second
/// round of the DKG process.
/// Emitted to notify the centralized party.
public struct CompletedDWalletDKGSecondRoundEvent has copy, drop, store {
    /// The identifier of the dWallet created as a result of the DKG process.
    dwallet_id: ID,

    /// The public output for the second round of the DKG process.
    public_output: vector<u8>,
    encrypted_user_secret_key_share_id: ID,
    session_id: ID
}

public struct RejectedDWalletDKGSecondRoundEvent has copy, drop, store {
    /// The identifier of the dWallet created as a result of the DKG process.
    dwallet_id: ID,

    /// The public output for the second round of the DKG process.
    public_output: vector<u8>,
}

// END OF DKG TYPES


public struct DWalletImportedKeyVerificationRequestEvent has copy, drop, store {
    /// The unique session identifier for the DWallet.
    dwallet_id: ID,

    encrypted_user_secret_key_share_id: ID,

    centralized_party_message: vector<u8>,

    /// The unique identifier of the dWallet capability associated with this session.
    dwallet_cap_id: ID,

    /// Encrypted centralized secret key share and the associated cryptographic proof of encryption.
    encrypted_centralized_secret_share_and_proof: vector<u8>,

    /// The `EncryptionKey` object used for encrypting the secret key share.
    encryption_key: vector<u8>,

    /// The unique identifier of the `EncryptionKey` object.
    encryption_key_id: ID,

    encryption_key_address: address,

    /// The public output of the centralized party in the DKG process.
    user_public_output: vector<u8>,

    /// The Ed25519 public key of the initiator,
    /// used to verify the signature on the centralized public output.
    signer_public_key: vector<u8>,

    /// The MPC network encryption key id that is used to encrypt associated dWallet network secret key share.
    dwallet_network_encryption_key_id: ID,

    /// The elliptic curve used for the dWallet.
    curve: u32,
}

public struct CompletedDWalletImportedKeyVerificationEvent has copy, drop, store {
    dwallet_id: ID,

    public_output: vector<u8>,
    encrypted_user_secret_key_share_id: ID,
    session_id: ID
}

public struct RejectedDWalletImportedKeyVerificationEvent has copy, drop, store {
    dwallet_id: ID,
}


// ENCRYPTED USER SHARE TYPES

/// Event emitted to start an encrypted dWallet centralized (user) key share
/// verification process.
/// Ika does not support native functions, so an event is emitted and
/// caught by the blockchain, which then starts the verification process,
/// similar to the MPC processes.
public struct EncryptedShareVerificationRequestEvent has copy, drop, store {
    /// Encrypted centralized secret key share and the associated cryptographic proof of encryption.
    encrypted_centralized_secret_share_and_proof: vector<u8>,

    /// The public output of the centralized party,
    /// belongs to the dWallet that its centralized
    /// secret share is being encrypted.
    /// This is not passed by the user,
    /// but taken from the blockchain during event creation.
    public_output: vector<u8>,

    /// The ID of the dWallet that this encrypted secret key share belongs to.
    dwallet_id: ID,

    /// The encryption key used to encrypt the secret key share with.
    encryption_key: vector<u8>,

    /// The `EncryptionKey` Move object ID.
    encryption_key_id: ID,

    encrypted_user_secret_key_share_id: ID,

    source_encrypted_user_secret_key_share_id: ID,
    dwallet_network_encryption_key_id: ID,

    curve: u32,
}

public struct CompletedEncryptedShareVerificationEvent has copy, drop, store {
    /// The ID of the `EncryptedUserSecretKeyShare` Move object.
    encrypted_user_secret_key_share_id: ID,

    /// The ID of the dWallet associated with this encrypted secret share.
    dwallet_id: ID,
}

public struct RejectedEncryptedShareVerificationEvent has copy, drop, store {
    /// The ID of the `EncryptedUserSecretKeyShare` Move object.
    encrypted_user_secret_key_share_id: ID,

    /// The ID of the dWallet associated with this encrypted secret share.
    dwallet_id: ID,
}

public struct AcceptEncryptedUserShareEvent has copy, drop, store {
    /// The ID of the `EncryptedUserSecretKeyShare` Move object.
    encrypted_user_secret_key_share_id: ID,

    /// The ID of the dWallet associated with this encrypted secret share.
    dwallet_id: ID,

    user_output_signature: vector<u8>,

    encryption_key_id: ID,

    encryption_key_address: address,
}
// END OF ENCRYPTED USER SHARE TYPES


public struct MakeDWalletUserSecretKeySharePublicRequestEvent has copy, drop, store {
    public_user_secret_key_share: vector<u8>,

    public_output: vector<u8>,

    curve: u32,

    dwallet_id: ID,

    dwallet_network_encryption_key_id: ID,
}

public struct CompletedMakeDWalletUserSecretKeySharePublicEvent has copy, drop, store {
    dwallet_id: ID,
}

public struct RejectedMakeDWalletUserSecretKeySharePublicEvent has copy, drop, store {
    dwallet_id: ID,
}

// PRESIGN TYPES

/// Event emitted to initiate the first round of a Presign session.
///
/// This event is used to signal Validators to start the
/// first round of the Presign process.
/// The event includes all necessary details to link
/// the session to the corresponding dWallet
/// and DKG process.
public struct PresignRequestEvent has copy, drop, store {
    /// The ID of the dWallet for which this Presign has been created and can be used by exclusively, if set.
    /// Optional, since some key signature algorithms (e.g., Schnorr and EdDSA) can support global presigns,
    /// which can be used for any dWallet (under the same network key).
    dwallet_id: Option<ID>,

    /// The ID of the presign.
    presign_id: ID,

    /// The output produced by the DKG process,
    /// used as input for the Presign session.
    dwallet_public_output: Option<vector<u8>>,

    /// The MPC network encryption key id that is used to encrypt associated dWallet network secret key share.
    dwallet_network_encryption_key_id: ID,

    /// The curve used for the presign.
    curve: u32,

    /// The signature algorithm for the presign.
    signature_algorithm: u32,
}

/// Event emitted when the presign batch is completed.
///
/// This event indicates the successful completion of a batched presign process.
/// It provides details about the presign objects created and their associated metadata.
public struct CompletedPresignEvent has copy, drop, store {
    /// The ID of the dWallet for which this Presign has been created and can be used by exclusively, if set.
    /// Optional, since some key signature algorithms (e.g., Schnorr and EdDSA) can support global presigns,
    /// which can be used for any dWallet (under the same network key).
    dwallet_id: Option<ID>,

    /// The session ID.
    session_id: ID,
    presign_id: ID,
    presign: vector<u8>,
}

public struct RejectedPresignEvent has copy, drop, store {
    /// The ID of the dWallet for which this Presign has been created and can be used by exclusively, if set.
    /// Optional, since some key signature algorithms (e.g., Schnorr and EdDSA) can support global presigns,
    /// which can be used for any dWallet (under the same network key).
    dwallet_id: Option<ID>,

    /// The session ID.
    session_id: ID,
    presign_id: ID
}

// END OF PRESIGN TYPES


/// Event emitted to initiate the signing process.
///
/// This event is captured by Validators to start the signing protocol.
/// It includes all the necessary information to link the signing process
/// to a specific dWallet, and batched process.
/// D: The type of data that can be stored with the object,
/// specific to each Digital Signature Algorithm.
public struct SignRequestEvent has copy, drop, store {
    sign_id: ID,

    /// The unique identifier for the dWallet used in the session.
    dwallet_id: ID,

    /// The output from the dWallet DKG process used in this session.
    dwallet_public_output: vector<u8>,

    /// The elliptic curve used for the dWallet.
    curve: u32,

    /// The signature algorithm used for the signing process.
    signature_algorithm: u32,

    hash_scheme: u32,

    /// The message to be signed in this session.
    message: vector<u8>,

    /// The MPC network encryption key id that is used to encrypt associated dWallet network secret key share.
    dwallet_network_encryption_key_id: ID,

    /// The presign object ID, this ID will
    /// be used as the signature MPC protocol ID.
    presign_id: ID,

    /// The presign protocol output as bytes.
    presign: vector<u8>,

    /// The centralized party signature of a message.
    message_centralized_signature: vector<u8>,

    /// Indicates whether the future sign feature was used to start the session.
    is_future_sign: bool,
}

/// Event emitted when a [`PartialCentralizedSignedMessages`] object is created.
public struct FutureSignRequestEvent has copy, drop, store {
    dwallet_id: ID,
    partial_centralized_signed_message_id: ID,
    message: vector<u8>,
    presign: vector<u8>,
    dwallet_public_output: vector<u8>,
    curve: u32,
    signature_algorithm: u32,
    hash_scheme: u32,
    message_centralized_signature: vector<u8>,
    dwallet_network_encryption_key_id: ID,
}

public struct CompletedFutureSignEvent has copy, drop, store {
    session_id: ID,
    dwallet_id: ID,
    partial_centralized_signed_message_id: ID,
}

public struct RejectedFutureSignEvent has copy, drop, store {
    session_id: ID,
    dwallet_id: ID,
    partial_centralized_signed_message_id: ID,
}

/// Event emitted to signal the completion of a Sign process.
///
/// This event contains signatures for all signed messages in the batch.
public struct CompletedSignEvent has copy, drop, store {
    sign_id: ID,

    /// The session identifier for the signing process.
    session_id: ID,

    /// The signature that was generated in this session.
    signature: vector<u8>,

    /// Indicates whether the future sign feature was used to start the session.
    is_future_sign: bool,
}

public struct RejectedSignEvent has copy, drop, store {
    sign_id: ID,

    /// The session identifier for the signing process.
    session_id: ID,

    /// Indicates whether the future sign feature was used to start the session.
    is_future_sign: bool,
}

/// Event containing dwallet 2pc-mpc checkpoint information, emitted during
/// the checkpoint submission message.
public struct DWalletCheckpointInfoEvent has copy, drop, store {
    epoch: u64,
    sequence_number: u64,
    timestamp_ms: u64,
}

// <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
const EDWalletMismatch: u64 = 1;
const EDWalletInactive: u64 = 2;
const EDWalletNotExists: u64 = 3;
const EWrongState: u64 = 4;
const EDWalletNetworkEncryptionKeyNotExist: u64 = 5;
const EInvalidEncryptionKeySignature: u64 = 6;
const EMessageApprovalMismatch: u64 = 7;
const EInvalidHashScheme: u64 = 8;
const ESignWrongState: u64 = 9;
const EPresignNotExist: u64 = 10;
const EIncorrectCap: u64 = 11;
const EUnverifiedCap: u64 = 12;
const EInvalidSource: u64 =13;
const EDWalletNetworkEncryptionKeyNotActive: u64 = 14;
const EInvalidPresign: u64 = 15;
const ECannotAdvanceEpoch: u64 = 16;
const EInvalidCurve: u64 = 17;
const EInvalidSignatureAlgorithm: u64 = 18;
const ECurvePaused: u64 = 19;
const ESignatureAlgorithmPaused: u64 = 20;
const EDWalletUserSecretKeySharesAlreadyPublic: u64 = 21;
const EMismatchCurve: u64 = 22;
const EImportedKeyDWallet: u64 = 23;
const ENotImportedKeyDWallet: u64 = 24;
const EHashSchemePaused: u64 = 25;
const EEncryptionKeyNotExist: u64 = 26;
const EMissingProtocolPricing: u64 = 27;
const EPricingCalculationVotesHasNotBeenStarted: u64 = 28;
const EPricingCalculationVotesMustBeCompleted: u64 = 29;
const ECannotSetDuringVotesCalculation: u64 = 30;

#[error]
const EIncorrectEpochInCheckpoint: vector<u8> = b"The checkpoint epoch is incorrect.";

#[error]
const EWrongCheckpointSequenceNumber: vector<u8> = b"The checkpoint sequence number should be the expected next one.";

#[error]
const EActiveBlsCommitteeMustInitialize: vector<u8> = b"First active committee must initialize.";

// >>>>>>>>>>>>>>>>>>>>>>>> Error codes >>>>>>>>>>>>>>>>>>>>>>>>

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

/// Start a Distributed Key Generation (DKG) session for the network (threshold) encryption key.
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

    // Emit an event to initiate the session in the Ika network.
    event::emit(self.create_system_dwallet_event(
        DWalletNetworkDKGEncryptionKeyRequestEvent {
            dwallet_network_encryption_key_id
        },
        ctx,
    ));

    // Return the capability.
    cap
}

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
        emit(RejectedDWalletNetworkDKGEncryptionKeyEvent {
            dwallet_network_encryption_key_id,
        });
        event::emit(self.create_system_dwallet_event(
            DWalletNetworkDKGEncryptionKeyRequestEvent {
                dwallet_network_encryption_key_id,
            },
            ctx,
        ));
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
        emit(RejectedDWalletEncryptionKeyReconfigurationEvent {
            dwallet_network_encryption_key_id,
        });
        event::emit(self.create_system_dwallet_event(
            DWalletEncryptionKeyReconfigurationRequestEvent {
                dwallet_network_encryption_key_id,
            },
            ctx,
        ));
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
    self: &mut DWalletCoordinatorInner, cap: &DWalletNetworkEncryptionKeyCap, ctx: &mut TxContext
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

    // Emit the event to the Ika network, requesting they start the reconfiguration session.
    event::emit(self.create_system_dwallet_event(
        DWalletEncryptionKeyReconfigurationRequestEvent {
            dwallet_network_encryption_key_id: cap.dwallet_network_encryption_key_id
        },
        ctx,
    ));
}

fun get_active_dwallet_network_encryption_key(
    self: &mut DWalletCoordinatorInner,
    dwallet_network_encryption_key_id: ID,
): &mut DWalletNetworkEncryptionKey {
    let dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(dwallet_network_encryption_key_id);

    assert!(dwallet_network_encryption_key.state != DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG, EDWalletNetworkEncryptionKeyNotActive);

    dwallet_network_encryption_key
}

/// Advance the epoch.
///
/// Checks that all the current epoch sessions are completed,
/// and updates the required metadata for the next epoch's sessions management.
///
/// Sets the current and previous committees.
///
/// Unlocks and updates `last_user_initiated_session_to_complete_in_current_epoch`.
///
/// And finally increments the `current_epoch`.
public(package) fun advance_epoch(
    self: &mut DWalletCoordinatorInner,
    next_committee: BlsCommittee,
    dwallet_network_encryption_key_caps: &vector<DWalletNetworkEncryptionKeyCap>,
): Balance<IKA> {
    assert!(self.pricing_and_fee_management.calculation_votes.is_none(), EPricingCalculationVotesMustBeCompleted);
    assert!(self.all_current_epoch_user_initiated_sessions_completed(), ECannotAdvanceEpoch);

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

fun get_dwallet(
    self: &DWalletCoordinatorInner,
    dwallet_id: ID,
): &DWallet {
    assert!(self.dwallets.contains(dwallet_id), EDWalletNotExists);

    self.dwallets.borrow(dwallet_id)
}

fun get_dwallet_mut(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
): &mut DWallet {
    assert!(self.dwallets.contains(dwallet_id), EDWalletNotExists);

    self.dwallets.borrow_mut(dwallet_id)
}

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

/// Creates a new MPC session and charges the user for it.
///
/// Payment is done in both Ika (for the MPC computation by the Ika network)
/// and Sui (for storing the public output in Sui).
/// The payment is saved in the session object, for it is to be distributed only upon the completion of the session.
///
/// The newly created session has its sequence number set to `next_session_sequence_number`, which is then incremented.
/// Finally, the last session to complete in current epoch is updated, if needed.
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

/// Creates a new MPC session that serves the system (i.e. the Ika network).
/// The current protocols that are supported for such is network DKG and Reconfiguration,
/// both of which are related to a particular `dwallet_network_encryption_key_id`.
/// No funds are charged, since there is no user to charge.
fun create_system_dwallet_event<E: copy + drop + store>(
    self: &mut DWalletCoordinatorInner,
    event_data: E,
    ctx: &mut TxContext,
): DWalletEvent<E> {
    self.session_management.started_system_sessions_count = self.session_management.started_system_sessions_count + 1;

    let event = DWalletEvent {
        epoch: self.current_epoch,
        session_type: SessionType::System,
        session_id: object::id_from_address(tx_context::fresh_object_address(ctx)),
        event_data,
    };

    event
}

fun get_active_dwallet_and_public_output(
    self: &DWalletCoordinatorInner,
    dwallet_id: ID,
): (&DWallet, vector<u8>) {
    assert!(self.dwallets.contains(dwallet_id), EDWalletNotExists);
    let dwallet = self.dwallets.borrow(dwallet_id);
    let public_output = dwallet.validate_active_and_get_public_output();
    (dwallet, *public_output)
}

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

/// Validates the `curve` selection is both supported, and not paused.
fun validate_curve(
    self: &DWalletCoordinatorInner,
    curve: u32,
) {
    assert!(self.support_config.supported_curves_to_signature_algorithms_to_hash_schemes.contains(&curve), EInvalidCurve);

    assert!(!self.support_config.paused_curves.contains(&curve), ECurvePaused);
}

/// Validates the `curve` and `signature_algorithm` selection is supported, and not paused.
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

/// Validates the `curve`, `signature_algorithm` and `hash_scheme` selection is supported, and not paused.
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

/// Registers an encryption key to be used later for encrypting a
/// centralized secret key share.
///
/// ### Parameters
/// - `encryption_key`: The serialized encryption key to be registered.
/// - `encryption_key_signature`: The signature of the encryption key, signed by the signer.
/// - `signer_public_key`: The public key of the signer used to verify the encryption key signature.
/// - `encryption_key_scheme`: The scheme of the encryption key (e.g., Class Groups).
/// Needed so the TX will get ordered in consensus before getting executed.
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

/// Represents a message that was approved to be signed by the dWallet corresponding to `dwallet_id`.
///
/// ### Fields
/// - **`dwallet_id`**: The identifier of the dWallet
///   associated with this approval.
/// - **`hash_scheme`**: The message hash scheme to use for signing.
/// - **`signature_algorithm`**: The signature algorithm with which the message can be signed.
/// - **`message`**: The message that has been approved.
public struct MessageApproval has store, drop {
    dwallet_id: ID,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector<u8>,
}

/// Represents a message that was approved to be signed by the imported key dWallet corresponding to `dwallet_id`.
///
/// ### Fields
/// - **`dwallet_id`**: The identifier of the dWallet
///   associated with this approval.
/// - **`hash_scheme`**: The message hash scheme to use for signing.
/// - **`signature_algorithm`**: The signature algorithm with which the message can be signed.
/// - **`message`**: The message that has been approved.
public struct ImportedKeyMessageApproval has store, drop {
    dwallet_id: ID,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector<u8>,
}

/// Approves `message` to be signed by the dWallet corresponding to `dwallet_cap`.
/// Binds the approval for a specific `signature_algorithm` and `hash_scheme` choice.
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

/// Approves `message` to be signed by the imported key dWallet corresponding to `imported_key_dwallet_cap`.
/// Binds the approval for a specific `signature_algorithm` and `hash_scheme` choice.
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

/// Starts the first Distributed Key Generation (DKG) session.
///
/// This function creates a new `DWalletCap` object,
/// transfers it to the session initiator (the user),
/// and emits a `DWalletDKGFirstRoundRequestEvent` to signal
/// the beginning of the DKG process.
///
/// ### Parameters
///
/// ### Effects
/// - Generates a new `DWalletCap` object.
/// - Transfers the `DWalletCap` to the session initiator (`ctx.sender`).
/// - Creates a new `DWallet` object and inserts it into the `dwallets` map.
/// - Emits a `DWalletDKGFirstRoundRequestEvent`.
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
/// Updates the `last_user_initiated_session_to_complete_in_current_epoch` field:
///  - If we already locked this field, we do nothing.
///  - Otherwise, we take the latest session whilst assuring
///    a maximum of `max_active_sessions_buffer` sessions to be completed in the current epoch.
fun update_last_user_initiated_session_to_complete_in_current_epoch(self: &mut DWalletCoordinatorInner) {
    if (self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch) {
        return
    };

    let new_last_user_initiated_session_to_complete_in_current_epoch = (
        self.session_management.number_of_completed_user_initiated_sessions + self.session_management.max_active_sessions_buffer
    ).min(
        self.session_management.next_session_sequence_number - 1
    );

    // Sanity check: only update this field if we need to.
    if (self.session_management.last_user_initiated_session_to_complete_in_current_epoch >= new_last_user_initiated_session_to_complete_in_current_epoch) {
        return
    };
    self.session_management.last_user_initiated_session_to_complete_in_current_epoch = new_last_user_initiated_session_to_complete_in_current_epoch;
}

/// Check whether all the user-initiated session that should complete in the current epoch are in fact completed.
/// This check is only relevant after `last_user_initiated_session_to_complete_in_current_epoch` is locked, and is called
/// as a requirement to advance the epoch.
/// Session sequence numbers are sequential, so ch
public(package) fun all_current_epoch_user_initiated_sessions_completed(self: &DWalletCoordinatorInner): bool {
    return (self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch &&
        (self.session_management.number_of_completed_user_initiated_sessions == self.session_management.last_user_initiated_session_to_complete_in_current_epoch) &&
        (self.session_management.completed_system_sessions_count == self.session_management.started_system_sessions_count))
}

/// Removes a user-initiated session and its corresponding event, charging the pre-paid gas amounts in both Sui and Ika
/// to be later distributed as part of the consensus validation and gas reimbursement fees.
///
/// Increments `number_of_completed_user_initiated_sessions`.
///
/// Notice: never called for a system session.
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

    object::delete(id);

    dwallet_network_encryption_key.computation_fee_charged_ika.join(computation_fee_charged_ika);
    self.pricing_and_fee_management.consensus_validation_fee_charged_ika.join(consensus_validation_fee_charged_ika);
    //self.gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
    gas_fee_reimbursement_sui
}

/// This function is called by the Ika network to respond to the dWallet DKG first round request made by the user.
/// Advances the dWallet's state and registers the output in it.
/// Also emits an event with the output.
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

/// Initiates the second round of the Distributed Key Generation (DKG) protocol
/// by emitting an event for the Ika validators to request the execution of this round.
///
/// Creates a new `EncryptedUserSecretKeyShare` object, with the state awaiting the network verification
/// that the user encrypted its user share correctly (the network will verify it as part of the second round).
///
/// Sets the state of the dWallet to `AwaitingNetworkDKGVerification`.
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

public struct NewImportedKeyDWalletEvent has copy, drop {
    dwallet_id: ID,
    dwallet_cap_id: ID,
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

/// Initiates the Presign protocol by creating a new `PresignSession` in `self.presign_sessions`
/// and emitting an event for the Ika validators to request its execution.
///
/// Creates an `UnverifiedPresignCap` for the new `presign_id` that can be exclusively used with this `dwallet_id`.
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

/// Initiates the Presign protocol by creating a new `PresignSession` in `self.presign_sessions`
/// and emitting an event for the Ika validators to request its execution.
///
/// Creates an `UnverifiedPresignCap` for the new `presign_id` that can be used with any dWallet.
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

/// This function is called by the Ika network to respond to the Presign request made by the user.
/// Advances the `PresignSession` state to `Completed` and registers the output (the presign) in it.
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

/// Checks that the presign corresponding to `cap` is valid by ensuring it is in the `Completed` state and that the IDs match.
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

/// Checks that the partial user signature corresponding to `cap` is valid, by assuring it is in the `NetworkVerificationCompleted`.
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
            if (message_data_type == DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE) {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let first_round_output = bcs_body.peel_vec_u8();
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                let gas_fee_reimbursement_sui = self.respond_dwallet_dkg_first_round(dwallet_id, first_round_output, rejected, session_sequence_number);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            } else if (message_data_type == DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE) {
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
            } else if (message_data_type == DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE) {
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
            } else if (message_data_type == DWALLET_SIGN_MESSAGE_TYPE) {
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
            } else if (message_data_type == DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE) {
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
            } else if (message_data_type == DWALLET_PRESIGN_MESSAGE_TYPE) {
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
            } else if (message_data_type == DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE) {
                let dwallet_network_encryption_key_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_output = bcs_body.peel_vec_u8();
                let is_last = bcs_body.peel_bool();
                let rejected = bcs_body.peel_bool();
                let gas_fee_reimbursement_sui = self.respond_dwallet_network_encryption_key_dkg(dwallet_network_encryption_key_id, public_output, is_last, rejected, ctx);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            } else if (message_data_type == DWALLET_MPC_NETWORK_RESHARE_OUTPUT_MESSAGE_TYPE) {
                let dwallet_network_encryption_key_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_output = bcs_body.peel_vec_u8();
                let is_last = bcs_body.peel_bool();
                let rejected = bcs_body.peel_bool();
                let gas_fee_reimbursement_sui = self.respond_dwallet_network_encryption_key_reconfiguration(dwallet_network_encryption_key_id, public_output, is_last, rejected, ctx);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            } else if (message_data_type == MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE) {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_user_secret_key_shares = bcs_body.peel_vec_u8();
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                let gas_fee_reimbursement_sui = self.respond_make_dwallet_user_secret_key_share_public(dwallet_id, public_user_secret_key_shares, rejected, session_sequence_number);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            } else if (message_data_type == DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE) {
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
            } else if (message_data_type == SET_MAX_ACTIVE_SESSIONS_BUFFER_MESSAGE_TYPE) {
                self.session_management.max_active_sessions_buffer = bcs_body.peel_u64();
            } else if (message_data_type == SET_GAS_FEE_REIMBURSEMENT_SUI_SYSTEM_CALL_VALUE_MESSAGE_TYPE) {
                self.pricing_and_fee_management.gas_fee_reimbursement_sui_system_call_value = bcs_body.peel_u64();
            };
        i = i + 1;
    };
    self.total_messages_processed = self.total_messages_processed + i;
    total_gas_fee_reimbursement_sui.into_coin(ctx)
}

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
    let mut i = 0;
    let curves = supported_curves_to_signature_algorithms_to_hash_schemes.keys();
    while (i < curves.length()) {
        let mut is_missing_pricing = false;
        let curve = curves[i];
        let signature_algorithms = &supported_curves_to_signature_algorithms_to_hash_schemes[&curve];
        let signature_algorithms = signature_algorithms.keys();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::none(), DKG_FIRST_ROUND_PROTOCOL_FLAG).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::none(), DKG_SECOND_ROUND_PROTOCOL_FLAG).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::none(), RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::none(), MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::none(), IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG).is_none();
        // Add here pricing validation for new protocols per curve.
        signature_algorithms.do_ref!(|signature_algorithm| {
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::some(*signature_algorithm), PRESIGN_PROTOCOL_FLAG).is_none();
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::some(*signature_algorithm), SIGN_PROTOCOL_FLAG).is_none();
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::some(*signature_algorithm), FUTURE_SIGN_PROTOCOL_FLAG).is_none();
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::some(*signature_algorithm), SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG).is_none();
            // Add here pricing validation for new protocols per curve per signature algorithm.
        });
        assert!(!is_missing_pricing, EMissingProtocolPricing);
        i = i + 1;
    };
}

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