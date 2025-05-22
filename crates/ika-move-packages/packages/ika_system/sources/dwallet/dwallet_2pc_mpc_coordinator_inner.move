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
use sui::balance::{Self, Balance};
use sui::bcs;
use sui::coin::{Coin};
use sui::bag::{Self, Bag};
use sui::event;
use sui::ed25519::ed25519_verify;
use ika_system::address;
use ika_system::dwallet_pricing::{DWalletPricing2PcMpcSecp256K1, PricingPerOperation};
use ika_system::bls_committee::{Self, BlsCommittee};
use sui::vec_map::{Self, VecMap};

const CHECKPOINT_MESSAGE_INTENT: vector<u8> = vector[1, 0, 0];

public(package) fun lock_last_active_session_sequence_number(self: &mut DWalletCoordinatorInner) {
    self.locked_last_user_initiated_session_to_complete_in_current_epoch = true;
}

/// A shared object that holds all the Ika system object used to manage dWallets:
///
/// Most importantly, the `dwallets` themselves, which holds the public key and public key shares,
/// and the encryption of the network's share under the network's threshold encryption key.
/// The encryption of the network's secret key share for every dWallet points to an encryption key in `dwallet_network_encryption_keys`,
/// which also stores the encrypted decryption key shares of each validator and their public verification keys.
///
/// For the user side, the secret key share is stored encrypted to the user encryption key (in `encryption_keys`) inside the dWallet,
/// together with a signature on the public key (shares).
/// Together, these constitute the necessairy information to create a signature with the user.
///
/// Next, `presign_sessions` holds the outputs of the Presign protocol which are later used for the signing protocol,
/// and `partial_centralized_signed_messages` holds the partial signatures of users awaiting for a future sign once a `MessageApproval` is presented.
///
/// Additionally, this structure holds management infromation, like the `previous_committee` and `active_committee` comittees,
/// information regarding `pricing`, all the `sessions` and the `next_session_sequence_number` that will be used for the next session,
/// and various other fields, like the supported and paused curves, signing algorithms and hashes.
public struct DWalletCoordinatorInner has store {
    current_epoch: u64,
    sessions: ObjectTable<u64, DWalletSession>,
    session_start_events: Bag,
    number_of_completed_user_initiated_sessions: u64,
    started_system_sessions_count: u64,
    completed_system_sessions_count: u64,
    /// The sequence number to assign to the next user-requested session.
    /// Initialized to `1` and incremented at every new session creation.
    next_session_sequence_number: u64,
    /// The last MPC session to process in the current epoch.
    /// The validators of the Ika network must always begin sessions,
    /// when they become available to them, so long their sequence numebr is lesser or equal to this value.
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
    /// The computation IKA price per unit size for the current epoch.
    pricing: DWalletPricing2PcMpcSecp256K1,
    /// Sui gas fee reimbursement to fund the network writing tx responses to sui.
    gas_fee_reimbursement_sui: Balance<SUI>,
    /// The fees paid for consensus validation in IKA.
    consensus_validation_fee_charged_ika: Balance<IKA>,
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
    /// A nested map of supported curves to signature algorithms to hash schemes.
    /// e.g. secp256k1 -> [(ecdsa -> [sha256, keccak256]), (schnorr -> [sha256])]
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap<u32, VecMap<u32, vector<u32>>>,
    // TODO(@Omer): paused_curves_to_signature_algorithms_to_hash_schemes: VecMap<u32, VecMap<u32, vector<u32>>>,
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
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

public struct DWalletSessionEventKey has copy, drop, store {}

/// An Ika MPC session.
public struct DWalletSession has key, store {
    id: UID,

    session_sequence_number: u64,

    // TODO(@Omer): this should be an `Option<>`, as non-dWallet MPC sessions might not be related to any network encryption key.
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

/// Represents a capability granting control over a specific dWallet network decryption key.
public struct DWalletNetworkEncryptionKeyCap has key, store {
    id: UID,
    dwallet_network_encryption_key_id: ID,
}

/// `DWalletNetworkEncryptionKey` represents a (threshold) encryption key owned by the network.
/// It stores the `network_dkg_public_output`, which in turn stores the encryption key itself (divided to chunks, due to space limitations).
/// Before the first reconfiguration (which happens at every epoch switch,)
/// `network_dkg_public_output` also holds the encryption of the current decryption key shares
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
    AwaitingNetworkReconfiguration,
    /// Reconfiguration request finished, but we didn't switch an epoch yet.
    AwaitingNextEpochReconfiguration,
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

    //TODO: make sure to include class gorup type and version inside the bytes with the rust code
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
    KeyHolderSiged {
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
    /// the case where we have zero trust for the dWallet becuase the
    /// user particiation is required.
    /// If set, the user secret key shares is public, the network can sign
    /// without the user participation. In this case, it is trust minimalized
    /// security for the user.
    public_user_secret_key_share: Option<vector<u8>>,

    /// The ID of the capability associated with this dWallet.
    dwallet_cap_id: ID,

    /// The MPC network decryption key id that is used to decrypt this dWallet.
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

public struct DWalletNetworkDKGDecryptionKeyRequestEvent has copy, drop, store {
    dwallet_network_encryption_key_id: ID,
}

public struct DWalletDecryptionKeyReshareRequestEvent has copy, drop, store {
    dwallet_network_encryption_key_id: ID,
}

public struct CompletedDWalletDecryptionKeyReshareEvent has copy, drop, store {
       dwallet_network_encryption_key_id: ID,
}

/// An event emitted when the first round of the DKG process is completed.
///
/// This event is emitted by the blockchain to notify the user about
/// the completion of the first round.
/// The user should catch this event to generate inputs for
/// the second round and call the `request_dwallet_dkg_second_round()` function.
public struct CompletedDWalletNetworkDKGDecryptionKeyEvent has copy, drop, store {
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

    /// The MPC network decryption key id that is used to decrypt associated dWallet.
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
public struct CompletedDWalletDKGFirstdRoundEvent has copy, drop, store {
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

    /// The MPC network decryption key id that is used to decrypt associated dWallet.
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

    /// The MPC network decryption key id that is used to decrypt associated dWallet.
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

    /// The MPC network decryption key id that is used to decrypt associated dWallet.
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

    /// The MPC network decryption key id that is used to decrypt associated dWallet.
    dwallet_network_encryption_key_id: ID,

    /// The presign object ID, this ID will
    /// be used as the singature MPC protocol ID.
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
/// the checkpoint submmision message.
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
    pricing: DWalletPricing2PcMpcSecp256K1,
    ctx: &mut TxContext
): DWalletCoordinatorInner {
    DWalletCoordinatorInner {
        current_epoch,
        sessions: object_table::new(ctx),
        session_start_events: bag::new(ctx),
        number_of_completed_user_initiated_sessions: 0,
        next_session_sequence_number: 1,
        last_user_initiated_session_to_complete_in_current_epoch: 0,
        // TODO (#856): Allow configuring the max_active_session_buffer field
        max_active_sessions_buffer: 100,
        locked_last_user_initiated_session_to_complete_in_current_epoch: false,
        dwallets: object_table::new(ctx),
        dwallet_network_encryption_keys: object_table::new(ctx),
        encryption_keys: object_table::new(ctx),
        presign_sessions: object_table::new(ctx),
        partial_centralized_signed_messages: object_table::new(ctx),
        pricing,
        gas_fee_reimbursement_sui: balance::zero(),
        consensus_validation_fee_charged_ika: balance::zero(),
        active_committee,
        previous_committee: bls_committee::empty(),
        total_messages_processed: 0,
        last_processed_checkpoint_sequence_number: option::none(),
        completed_system_sessions_count: 0,
        started_system_sessions_count: 0,
        previous_epoch_last_checkpoint_sequence_number: 0,
        supported_curves_to_signature_algorithms_to_hash_schemes: vec_map::empty(),
        paused_curves: vector[],
        paused_signature_algorithms: vector[],
        paused_hash_schemes: vector[],
        signature_algorithms_allowed_global_presign: vector[],
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
        dwallet_network_encryption_key_id,
        DWalletNetworkDKGDecryptionKeyRequestEvent {
            dwallet_network_encryption_key_id
        },
        ctx,
    ));

    // Return the capability.
    cap
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
) {
    // The DKG output can be large, so it is seperated into chunks.
    // We should only update the count once, so we check it is the last chunk before we do.
    if (is_last_chunk) {
        self.completed_system_sessions_count = self.completed_system_sessions_count + 1;
    };

    // Store this chunk as the last chunk in the network encryption public output chunks vector.
    let dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(dwallet_network_encryption_key_id);
    dwallet_network_encryption_key.network_dkg_public_output.push_back(network_public_output_chunk);

    // Change state to complete and emit an event to signify that only if it is the last chunk.
    dwallet_network_encryption_key.state = match (&dwallet_network_encryption_key.state) {
        DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG => {
            if (is_last_chunk) {
                event::emit(CompletedDWalletNetworkDKGDecryptionKeyEvent {
                    dwallet_network_encryption_key_id,
                });
                DWalletNetworkEncryptionKeyState::NetworkDKGCompleted
            } else {
                DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG
            }
        },
        _ => abort EWrongState
    };
}

/// Complete the Recondiguration session
/// and store the public output corresponding to the reconfigured network (threshold) encryption key.
///
/// Note: assumes the public output is divided into chunks and each `network_public_output_chunk` is delivered in order,
/// with `is_last_chunk` set for the last call.
public(package) fun respond_dwallet_network_encryption_key_reconfiguration(
    self: &mut DWalletCoordinatorInner,
    dwallet_network_encryption_key_id: ID,
    public_output: vector<u8>,
    is_last_chunk: bool,
) {
    // The Reconfiguration output can be large, so it is seperated into chunks.
    // We should only update the count once, so we check it is the last chunk before we do.
    if (is_last_chunk) {
        self.completed_system_sessions_count = self.completed_system_sessions_count + 1;
    };

    // Store this chunk as the last chunk in the chunks vector corresponding to the upcoming's epoch in the public outputs map.
    let dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(dwallet_network_encryption_key_id);
    let next_reconfiguration_public_output = dwallet_network_encryption_key.reconfiguration_public_outputs.borrow_mut(dwallet_network_encryption_key.current_epoch + 1);

    // Change state to complete and emit an event to signify that only if it is the last chunk.
    next_reconfiguration_public_output.push_back(public_output);
    dwallet_network_encryption_key.state = match (&dwallet_network_encryption_key.state) {
        DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration => {
            if (is_last_chunk) {
                event::emit(CompletedDWalletDecryptionKeyReshareEvent {
                    dwallet_network_encryption_key_id,
                });
                DWalletNetworkEncryptionKeyState::AwaitingNextEpochReconfiguration
            } else {
                DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration
            }
        },
        _ => abort EWrongState
    };
}

/// Advance the `current_epoch` and `state` of the network encryption key corresponding to `cap`,
/// finalizing the reonconfiguration of that key, and readying it for use in the next epoch.
public(package) fun advance_epoch_dwallet_network_encryption_key(
    self: &mut DWalletCoordinatorInner,
    cap: &DWalletNetworkEncryptionKeyCap,
): Balance<IKA> {
    // Get the corresponding network encryption key.
    let dwallet_network_encryption_key = self.get_active_dwallet_network_encryption_key(
        cap.dwallet_network_encryption_key_id
    );

    // Sanity checks: check the capability is the right one, and that the key is in the right state.
    assert!(dwallet_network_encryption_key.dwallet_network_encryption_key_cap_id == cap.id.to_inner(), EIncorrectCap);
    assert!(dwallet_network_encryption_key.state == DWalletNetworkEncryptionKeyState::AwaitingNextEpochReconfiguration, EWrongState);

    // Advance the current epoch and state.
    dwallet_network_encryption_key.current_epoch = dwallet_network_encryption_key.current_epoch + 1;
    dwallet_network_encryption_key.state = DWalletNetworkEncryptionKeyState::NetworkReconfigurationCompleted;

    // Return the fees.
    let mut epoch_computation_fee_charged_ika = sui::balance::zero<IKA>();
    epoch_computation_fee_charged_ika.join(dwallet_network_encryption_key.computation_fee_charged_ika.withdraw_all());
    return epoch_computation_fee_charged_ika
}

/// Emit an event to the Ika network to request a reconfiguration session for the network encryption key corresponding to `cap`.
public(package) fun emit_start_reconfiguration_event(
    self: &mut DWalletCoordinatorInner, cap: &DWalletNetworkEncryptionKeyCap, ctx: &mut TxContext
) {
    let dwallet_network_encryption_key = self.get_active_dwallet_network_encryption_key(cap.dwallet_network_encryption_key_id);

    // Set the state as awaiting reconfiguration.
    dwallet_network_encryption_key.state = DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration;

    // Initialize the chunks vector corresponding to the upcoming's epoch in the public outputs map.
    dwallet_network_encryption_key.reconfiguration_public_outputs.add(dwallet_network_encryption_key.current_epoch + 1, table_vec::empty(ctx));

    // Emit the event to the Ika network, requesting they start the reconfiguration session.
    event::emit(self.create_system_dwallet_event(
        cap.dwallet_network_encryption_key_id,
        DWalletDecryptionKeyReshareRequestEvent {
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
/// and updates the required metadata for the next epoch's sessions manageement.
///
/// Sets the current and previous comittees.
///
/// Unlocks and updates `last_user_initiated_session_to_complete_in_current_epoch`.
///
/// And finally increments the `current_epoch`.
public(package) fun advance_epoch(
    self: &mut DWalletCoordinatorInner,
    next_committee: BlsCommittee
): Balance<IKA> {
    assert!(self.all_current_epoch_user_initiated_sessions_completed(), ECannotAdvanceEpoch);

    if (self.last_processed_checkpoint_sequence_number.is_some()) {
        let last_processed_checkpoint_sequence_number = *self.last_processed_checkpoint_sequence_number.borrow();
        self.previous_epoch_last_checkpoint_sequence_number = last_processed_checkpoint_sequence_number;
    };

    self.locked_last_user_initiated_session_to_complete_in_current_epoch = false;
    self.update_last_user_initiated_session_to_complete_in_current_epoch();

    self.current_epoch = self.current_epoch + 1;

    self.previous_committee = self.active_committee;
    self.active_committee = next_committee;

    let mut epoch_consensus_validation_fee_charged_ika = sui::balance::zero<IKA>();
    epoch_consensus_validation_fee_charged_ika.join(self.consensus_validation_fee_charged_ika.withdraw_all());
    return epoch_consensus_validation_fee_charged_ika
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
    pricing: PricingPerOperation,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    event_data: E,
    ctx: &mut TxContext,
): DWalletEvent<E> {
    assert!(self.dwallet_network_encryption_keys.contains(dwallet_network_encryption_key_id), EDWalletNetworkEncryptionKeyNotExist);

    let computation_fee_charged_ika = payment_ika.split(pricing.computation_ika(), ctx).into_balance();

    let consensus_validation_fee_charged_ika = payment_ika.split(pricing.consensus_validation_ika(), ctx).into_balance();
    let gas_fee_reimbursement_sui = payment_sui.split(pricing.gas_fee_reimbursement_sui(), ctx).into_balance();

    let session_sequence_number = self.next_session_sequence_number;
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

    self.session_start_events.add(session.id.to_inner(), event);
    self.sessions.add(session_sequence_number, session);
    self.next_session_sequence_number = session_sequence_number + 1;
    self.update_last_user_initiated_session_to_complete_in_current_epoch();

    event
}

/// Creates a new MPC session that serves the system (i.e. the Ika network).
/// The current protocols that are supported for such is network DKG and Reconfiguration,
/// both of which are related to a particular `dwallet_network_encryption_key_id`.
/// No funds are charged, since there is no user to charge.
fun create_system_dwallet_event<E: copy + drop + store>(
    self: &mut DWalletCoordinatorInner,
    // TODO(@Omer): perhaps make it an option? what if we would want to add system sessions that aren't related to a particular network key?
    dwallet_network_encryption_key_id: ID,
    event_data: E,
    ctx: &mut TxContext,
): DWalletEvent<E> {
    assert!(self.dwallet_network_encryption_keys.contains(dwallet_network_encryption_key_id), EDWalletNetworkEncryptionKeyNotExist);
    self.started_system_sessions_count = self.started_system_sessions_count + 1;

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
    // TODO(@Omer): what happens if address is not found? Shouldn't we check `contains` first?
    self.encryption_keys.borrow(address).id.to_inner()
}

/// Validates the `curve` selection is both supported, and not paused.
fun validate_curve(
    self: &DWalletCoordinatorInner,
    curve: u32,
) {
    assert!(self.supported_curves_to_signature_algorithms_to_hash_schemes.contains(&curve), EInvalidCurve);
    // TODO(@omer): same structure of `supported_curves_to_signature_algorithms_to_hash_schemes` for paused.
    assert!(!self.paused_curves.contains(&curve), ECurvePaused);
}

/// Validates the `curve` and `signature_algorithm` selection is supported, and not paused.
fun validate_curve_and_signature_algorithm(
    self: &DWalletCoordinatorInner,
    curve: u32,
    signature_algorithm: u32,
) {
    self.validate_curve(curve);
    let supported_curve_to_signature_algorithms = self.supported_curves_to_signature_algorithms_to_hash_schemes[&curve];

    // TODO(@omer): same structure of `supported_curves_to_signature_algorithms_to_hash_schemes` for paused.
    assert!(supported_curve_to_signature_algorithms.contains(&signature_algorithm), EInvalidSignatureAlgorithm);
    assert!(!self.paused_signature_algorithms.contains(&signature_algorithm), ESignatureAlgorithmPaused);
}

/// Validates the `curve`, `signature_algorithm` and `hash_scheme` selection is supported, and not paused.
fun validate_curve_and_signature_algorithm_and_hash_scheme(
    self: &DWalletCoordinatorInner,
    curve: u32,
    signature_algorithm: u32,
    hash_scheme: u32,
) {
    self.validate_curve_and_signature_algorithm(curve, signature_algorithm);
    let supported_hash_schemes = self.supported_curves_to_signature_algorithms_to_hash_schemes[&curve][&signature_algorithm];

    // TODO(@omer): same structure of `supported_curves_to_signature_algorithms_to_hash_schemes` for paused.
    assert!(supported_hash_schemes.contains(&hash_scheme), EInvalidHashScheme);
    assert!(!self.paused_hash_schemes.contains(&hash_scheme), EHashSchemePaused);
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
/// - **`signature_algorithm`**: The signature algoirthm with which the message can be signed.
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
/// - **`signature_algorithm`**: The signature algoirthm with which the message can be signed.
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

    let pricing = self.pricing.dkg_first_round();

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
        pricing,
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
    if (self.locked_last_user_initiated_session_to_complete_in_current_epoch) {
        return
    };

    let new_last_user_initiated_session_to_complete_in_current_epoch = (
        self.number_of_completed_user_initiated_sessions + self.max_active_sessions_buffer
    ).min(
        self.next_session_sequence_number - 1
    );

    // Sanity check: only update this field if we need to.
    if (self.last_user_initiated_session_to_complete_in_current_epoch >= new_last_user_initiated_session_to_complete_in_current_epoch) {
        return
    };
    self.last_user_initiated_session_to_complete_in_current_epoch = new_last_user_initiated_session_to_complete_in_current_epoch;
}

/// Check whether all the user-initiated session that should complete in the current epoch are in fact completed.
/// This check is only relevant after `last_user_initiated_session_to_complete_in_current_epoch` is locked, and is called
/// as a requirement to advance the epoch.
/// Session sequence numbers are sequential, so ch
public(package) fun all_current_epoch_user_initiated_sessions_completed(self: &DWalletCoordinatorInner): bool {
    return (self.locked_last_user_initiated_session_to_complete_in_current_epoch &&
        (self.number_of_completed_user_initiated_sessions == self.last_user_initiated_session_to_complete_in_current_epoch) &&
        (self.completed_system_sessions_count == self.started_system_sessions_count))
}

/// Removes a user-initiated session, charging the pre-paid gas amounts in both Sui and Ika
/// to be later distributed as part of the consensus validation and gas reimburesement fees.
///
/// Increments `number_of_completed_user_initiated_sessions`.
///
/// Notice: never called for a system session.
fun remove_user_initiated_session_and_charge<E: copy + drop + store>(self: &mut DWalletCoordinatorInner, session_sequence_number: u64) {
    self.number_of_completed_user_initiated_sessions = self.number_of_completed_user_initiated_sessions + 1;
    self.update_last_user_initiated_session_to_complete_in_current_epoch();
    let session = self.sessions.remove(session_sequence_number);
    let DWalletSession {
        computation_fee_charged_ika,
        gas_fee_reimbursement_sui,
        consensus_validation_fee_charged_ika,
        dwallet_network_encryption_key_id,
        id,
        ..
    } = session;
    let dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(dwallet_network_encryption_key_id);
    let _: DWalletEvent<E> = self.session_start_events.remove(id.to_inner());
    object::delete(id);
    dwallet_network_encryption_key.computation_fee_charged_ika.join(computation_fee_charged_ika);
    self.consensus_validation_fee_charged_ika.join(consensus_validation_fee_charged_ika);
    self.gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
}

/// Creates the output of the first DKG round.
///
/// This function transfers the output of the first DKG round
/// to the session initiator and ensures it is securely linked
/// to the `DWalletCap` of the session.
/// This function is called by blockchain itself.
/// Validators call it, it's part of the blockchain logic.
///
/// ### Effects
/// - Transfers the output of the first round to the initiator.
/// - Emits necessary metadata and links it to the associated session.
///
/// ### Parameters
/// - `initiator`: The address of the user who initiated the DKG session.
/// - `session_id`: The ID of the DKG session.
/// - `decentralized_public_output`: The public output data from the first round.
/// - `dwallet_cap_id`: The ID of the associated `DWalletCap`.
/// - `ctx`: The transaction context.
///
/// ### Panics
/// - Panics with `ENotSystemAddress` if the sender is not the system address.
public(package) fun respond_dwallet_dkg_first_round(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    first_round_output: vector<u8>,
    rejected: bool,
    session_sequence_number: u64,
) {
    self.remove_user_initiated_session_and_charge<DWalletDKGFirstRoundRequestEvent>(session_sequence_number);

    let dwallet = self.get_dwallet_mut(dwallet_id);
    dwallet.state = match (dwallet.state) {
        DWalletState::DKGRequested => {
            if (rejected) {
                event::emit(RejectedDWalletDKGFirstRoundEvent {
                    dwallet_id,
                });
                DWalletState::NetworkRejectedDKGRequest
            } else {
                event::emit(CompletedDWalletDKGFirstdRoundEvent {
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

}

/// Initiates the second round of the Distributed Key Generation (DKG) process
/// and emits an event for validators to begin their participation in this round.
///
/// This function handles the creation of a new DKG session ID and emits an event containing
/// all the necessary parameters to continue the DKG process.
/// ### Parameters
/// - `dwallet_cap`: A reference to the `DWalletCap`, representing the capability associated with the dWallet.
/// - `centralized_public_key_share_and_proof`: The user (centralized) public key share and proof.
/// - `first_round_output`: A reference to the `DWalletDKGFirstRoundOutput` structure containing the output of the first DKG round.
/// - `encrypted_centralized_secret_share_and_proof`: Encrypted centralized secret key share and its proof.
/// - `encryption_key`: The `EncryptionKey` object used for encrypting the secret key share.
/// - `centralized_public_output`: The public output of the centralized party in the DKG process.
/// - `decentralized_user_output_signature`: The signature for the public output of the centralized party in the DKG process.
/// - `signer_public_key`: The Ed25519 public key of the initiator,
///    used to verify the signature on the public output.
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

    let pricing = self.pricing.dkg_second_round();

    let emit_event = self.charge_and_create_current_epoch_dwallet_event(
        dwallet_network_encryption_key_id,
        pricing,
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

/// Completes the second round of the Distributed Key Generation (DKG) process and
/// creates the [`DWallet`].
///
/// This function finalizes the DKG process by creating a `DWallet` object and associating it with the
/// cryptographic outputs of the second round. It also generates an encrypted user share and emits
/// events to record the results of the process.
/// This function is called by the blockchain.
///
/// ### Parameters
/// - **`session_id`**: A unique identifier for the current DKG session.
/// - **`decentralized_public_output`**: The public output of the second round of the DKG process,
///      representing the decentralized computation result.
/// - **`dwallet_cap_id`**: The unique identifier of the `DWalletCap` associated with this session.
/// - **`dwallet_mpc_network_decryption_key_version`**: The version of the MPC network key for the `DWallet`.
/// - **`encrypted_secret_share_and_proof`**: The encrypted user secret key share and associated cryptographic proof.
/// - **`encryption_key_id`**: The ID of the `EncryptionKey` used for encrypting the secret key share.
/// - **`signed_public_share`**: The signed public share corresponding to the secret key share.
/// - **`encryptor_ed25519_pubkey`**: The Ed25519 public key of the entity that encrypted the secret key share.
/// - **`centralized_public_output`**: The centralized public output from the DKG process.
///
/// ### Effects
/// - Creates a new `DWallet` object using the provided session ID, DKG outputs, and other metadata.
/// - Creates an encrypted user share and associates it with the `DWallet`.
/// - Emits a `CompletedDWalletDKGSecondRoundEvent` to record the completion of the second DKG round.
/// - Freezes the created `DWallet` object to make it immutable.
public(package) fun respond_dwallet_dkg_second_round(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    public_output: vector<u8>,
    encrypted_user_secret_key_share_id: ID,
    session_id: ID,
    rejected: bool,
    session_sequence_number: u64,
) {
    self.remove_user_initiated_session_and_charge<DWalletDKGSecondRoundRequestEvent>(session_sequence_number);
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

}

/// Transfers an encrypted dWallet user secret key share from a source entity to destination entity.
///
/// This function emits an event with the encrypted user secret key share, along with its cryptographic proof,
/// to the blockchain. The chain verifies that the encrypted data matches the expected secret key share
/// associated with the dWallet before creating an [`EncryptedUserSecretKeyShare`] object.
///
/// ### Parameters
/// - **`dwallet`**: A reference to the `DWallet<Secp256K1>` object to which the secret share is linked.
/// - **`destination_encryption_key`**: A reference to the encryption key used for encrypting the secret key share.
/// - **`encrypted_centralized_secret_share_and_proof`**: The encrypted secret key share, accompanied by a cryptographic proof.
/// - **`source_signed_centralized_public_output`**: The signed centralized public output corresponding to the secret share.
/// - **`source_ed25519_pubkey`**: The Ed25519 public key of the source (encryptor) used for verifying the signature.
///
/// ### Effects
/// - Emits a `EncryptedShareVerificationRequestEvent`,
/// which is captured by the blockchain to initiate the verification process.
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

    let pricing = self.pricing.re_encrypt_user_share();

    event::emit(
        self.charge_and_create_current_epoch_dwallet_event(
            dwallet_network_encryption_key_id,
            pricing,
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

/// Creates an encrypted user secret key share after it has been verified by the blockchain.
///
/// This function is invoked by the blockchain to generate an [`EncryptedUserSecretKeyShare`] object
/// once the associated encryption and cryptographic proofs have been verified.
/// It finalizes the process by storing the encrypted user share on-chain and emitting the relevant event.
///
/// ### Parameters
/// - `dwallet_id`: The unique identifier of the dWallet associated with the encrypted user share.
/// - `encrypted_centralized_secret_share_and_proof`: The encrypted centralized secret key share along with its cryptographic proof.
/// - `encryption_key_id`: The `EncryptionKey` Move object ID used to encrypt the secret key share.
/// - `centralized_user_output_signature`: The signed public share corresponding to the encrypted secret share.
/// - `signer_public_key`: The Ed25519 public key of the encryptor, used for signing.
/// - `initiator`: The address of the entity that performed the encryption operation of this secret key share.
public(package) fun respond_re_encrypt_user_share_for(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    encrypted_user_secret_key_share_id: ID,
    rejected: bool,
    session_sequence_number: u64
) {
    self.remove_user_initiated_session_and_charge<EncryptedShareVerificationRequestEvent>(session_sequence_number);
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
}

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
        EncryptedUserSecretKeyShareState::NetworkVerificationCompleted => EncryptedUserSecretKeyShareState::KeyHolderSiged {
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

    let pricing = self.pricing.imported_key_dwallet_verification();

    let emit_event = self.charge_and_create_current_epoch_dwallet_event(
        dwallet_network_encryption_key_id,
        pricing,
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

public(package) fun respond_imported_key_dwallet_verification(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    public_output: vector<u8>,
    encrypted_user_secret_key_share_id: ID,
    session_id: ID,
    rejected: bool,
    session_sequence_number: u64,
) {
    self.remove_user_initiated_session_and_charge<DWalletImportedKeyVerificationRequestEvent>(session_sequence_number);
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
                DWalletState::Active {
                    public_output
                }
            }
        },
        _ => abort EWrongState
    };

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

    let pricing = self.pricing.make_dwallet_user_secret_key_share_public();

    event::emit(
        self.charge_and_create_current_epoch_dwallet_event(
            dwallet_network_encryption_key_id,
            pricing,
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

public(package) fun respond_make_dwallet_user_secret_key_share_public(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    public_user_secret_key_share: vector<u8>,
    rejected: bool,
    session_sequence_number: u64,
) {
    self.remove_user_initiated_session_and_charge<MakeDWalletUserSecretKeySharePublicRequestEvent>(session_sequence_number);
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
    }
}

/// Starts a batched presign session.
///
/// This function emits a `RequestedBatchedPresignEvent` for the entire batch and a
/// `RequestedPresignFirstRoundEvent` for each presign in the batch. These events signal
/// validators to begin processing the first round of the presign process for each session.
/// - A unique `batch_session_id` is generated for the batch.
/// - A loop creates and emits a `RequestedPresignFirstRoundEvent` for each session in the batch.
/// - Each session is linked to the parent batch via `batch_session_id`.
///
/// ### Effects
/// - Associates the batched presign session with the specified dWallet.
/// - Emits a `RequestedBatchedPresignEvent` containing the batch session details.
/// - Emits a `RequestedPresignFirstRoundEvent` for each presign in the batch, with relevant details.
///
/// ### Parameters
/// - `dwallet_id`: The dWallet's ID to resquest presign.
/// - `ctx`: The mutable transaction context, used to generate unique object IDs and retrieve the initiator.
public(package) fun request_presign(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    signature_algorithm: u32,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): UnverifiedPresignCap {
    let created_at_epoch = self.current_epoch;

    assert!(!self.signature_algorithms_allowed_global_presign.contains(&signature_algorithm), EInvalidSignatureAlgorithm);

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

    let pricing = self.pricing.presign();

    event::emit(
        self.charge_and_create_current_epoch_dwallet_event(
            dwallet_network_encryption_key_id,
            pricing,
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

    assert!(self.signature_algorithms_allowed_global_presign.contains(&signature_algorithm), EInvalidSignatureAlgorithm);

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

    let pricing = self.pricing.presign();

    event::emit(
        self.charge_and_create_current_epoch_dwallet_event(
            dwallet_network_encryption_key_id,
            pricing,
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

/// Completes the presign session by creating the output of the
/// second presign round and transferring it to the session initiator.
///
/// This function is called by validators as part of the blockchain logic.
/// It creates a `Presign` object representing the second presign round output,
/// emits a `CompletedPresignEvent`, and transfers the result to the initiating user.
///
/// ### Parameters
/// - `initiator`: The address of the user who initiated the presign session.
/// - `session_id`: The ID of the presign session.
/// - `output`: The presign result data.
/// - `dwallet_cap_id`: The ID of the associated `DWalletCap`.
/// - `dwallet_id`: The ID of the associated `DWallet`.
/// - `ctx`: The transaction context.
///
/// ### Emits
/// - `CompletedPresignEvent`: Includes the initiator, dWallet ID, and presign ID.
///
/// ### Panics
/// - Panics with `ENotSystemAddress` if the sender of the transaction is not the system address.
///
/// ### Effects
/// - Creates a `Presign` object and transfers it to the session initiator.
/// - Emits a `CompletedPresignEvent`.
public(package) fun respond_presign(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: Option<ID>,
    presign_id: ID,
    session_id: ID,
    presign: vector<u8>,
    rejected: bool,
    session_sequence_number: u64
) {
    self.remove_user_initiated_session_and_charge<PresignRequestEvent>(session_sequence_number);

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
}

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

/// This function is a shared logic for both the normal and future sign flows.
/// It checks the presign is valid and removes it, thus assuring it is never used twice.
/// Finally it emits the sign event.
fun validate_and_initiate_sign(
    self: &mut DWalletCoordinatorInner,
    pricing: PricingPerOperation,
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
    assert!(presign_dwallet_id.is_none() || presign_dwallet_id.is_some_and!(|id| id == dwallet_id), EMessageApprovalMismatch);
    assert!(presign_signature_algorithm == signature_algorithm, EMessageApprovalMismatch);
    assert!(presign_cap_id == cap_id, EPresignNotExist);
    assert!(presign_id == presign_cap_presign_id, EPresignNotExist);
    assert!(presign_cap_dwallet_id == presign_dwallet_id, EPresignNotExist);
    assert!(dwallet.curve == curve, EDWalletMismatch);

    let id = object::new(ctx);
    let sign_id = id.to_inner();
    let dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;
    let emit_event = self.charge_and_create_current_epoch_dwallet_event(
        dwallet_network_encryption_key_id,
        pricing,
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


/// Initiates the signing process for a given dWallet of type T.
///
/// This function emits a `RequestedSignEvent` and a `RequestedBatchedSignEvent`,
/// providing all necessary metadata to ensure the integrity of the signing process.
/// It validates the linkage between the `DWallet`, `DWalletCap`, and `SignatureAlgorithmData` objects.
///
/// # Effects
/// - Ensures a valid linkage between `DWallet`, `DWalletCap`, and `SignatureAlgorithmData`.
/// - Validates that `signature_algorithm_data` and `message_approvals` have the same length.
/// - Emits the following events:
///   - `RequestedBatchedSignEvent`: Contains the session details and the list of hashed messages.
///   - `RequestedSignEvent`: Includes details for each message signing process.
///
/// # Aborts
/// - **`EExtraDataAndMessagesLenMismatch`**: If the number of `hashed_messages` does not
///   match the number of `signature_algorithm_data`.
/// - **`EMissingApprovalOrWrongApprovalOrder`**: If the approvals are missing or provided in the incorrect order.
///
/// # Parameters
/// - `message_approvals`: A vector of `MessageApproval` objects representing
///    approvals for the messages, which are destroyed at the end of the transaction.
/// - `dwallet`: A reference to the `DWallet` object being used for signing.
/// - `signature_algorithm_data`: A vector of `SignatureAlgorithmData` objects containing intermediate signing outputs,
///   which are unpacked and then destroyed at the end of the transaction.
///
/// # Type Parameters
/// - `T`: The elliptic curve type used for the dWallet.
/// D: The type of data that can be stored with the object,
/// specific to each Digital Signature Algorithm.
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

    let pricing = self.pricing.sign();

    let is_imported_key_dwallet = self.validate_and_initiate_sign(
        pricing,
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

    let pricing = self.pricing.sign();

    let is_imported_key_dwallet = self.validate_and_initiate_sign(
        pricing,
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

// TODO: add hash_scheme per message so we can validate that.
/// A function to publish messages signed by the user on chain with on-chain verification,
/// without launching the chain's sign flow immediately.
///
/// See the docs of [`PartialCentralizedSignedMessages`] for
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
    let pricing = self.pricing.future_sign();

    assert!(!self.paused_hash_schemes.contains(&hash_scheme), EHashSchemePaused);

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
    let emit_event = self.charge_and_create_current_epoch_dwallet_event(
        dwallet_network_encryption_key_id,
        pricing,
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

public(package) fun respond_future_sign(
    self: &mut DWalletCoordinatorInner,
    session_id: ID,
    dwallet_id: ID,
    partial_centralized_signed_message_id: ID,
    rejected: bool,
    session_sequence_number: u64
) {
    self.remove_user_initiated_session_and_charge<FutureSignRequestEvent>(session_sequence_number);
    let partial_centralized_signed_message = self.partial_centralized_signed_messages.borrow_mut(partial_centralized_signed_message_id);
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
    }
}

public(package) fun is_partial_user_signature_valid(
    self: &DWalletCoordinatorInner,
    cap: &UnverifiedPartialUserSignatureCap,
): bool {
    let partial_centralized_signed_message = self.partial_centralized_signed_messages.borrow(cap.partial_centralized_signed_message_id);
    partial_centralized_signed_message.cap_id == cap.id.to_inner() && partial_centralized_signed_message.state == PartialUserSignatureState::NetworkVerificationCompleted
}

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

/// Initiates a signing flow using a previously published [`PartialUserSignature`].
///
/// This function takes a partial signature object, validates approvals for each message,
/// and emits the necessary signing events.
///
/// ## Type Parameters
/// - `D`: Represents additional data fields specific for each implementation.
///
/// ## Parameters
/// - `partial_signature`: A previously published `PartialUserSignature<D>` object
///   containing messages that require approval.
/// - `message_approvals`: A list of approvals corresponding to the messages in `partial_signature`.
/// - `ctx`: The transaction context.
/// ## Notes
/// - See [`PartialUserSignature`] documentation for more details on usage scenarios.
/// - The function ensures that messages and approvals have a one-to-one correspondence before proceeding.
public(package) fun request_sign_with_partial_user_signature(
    self: &mut DWalletCoordinatorInner,
    partial_user_signature_cap: VerifiedPartialUserSignatureCap,
    message_approval: MessageApproval,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {

    let pricing = self.pricing.sign_with_partial_user_signature();

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
        curve: _,
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

    // Emit signing events to finalize the signing process.
    let is_imported_key_dwallet = self.validate_and_initiate_sign(
        pricing,
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

public(package) fun request_imported_key_sign_with_partial_user_signature(
    self: &mut DWalletCoordinatorInner,
    partial_user_signature_cap: VerifiedPartialUserSignatureCap,
    message_approval: ImportedKeyMessageApproval,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    let pricing = self.pricing.sign_with_partial_user_signature();

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
        curve: _,
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

    // Emit signing events to finalize the signing process.
    let is_imported_key_dwallet = self.validate_and_initiate_sign(
        pricing,
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

/// Emits a `CompletedSignEvent` with the MPC Sign protocol output.
///
/// This function is called by the blockchain itself and is part of the core
/// blockchain logic executed by validators. The emitted event contains the
/// completed sign output that should be consumed by the initiating user.
///
/// ### Parameters
/// - **`signed_messages`**: A vector containing the signed message outputs.
/// - **`batch_session_id`**: The unique identifier for the batch signing session.
/// - **`ctx`**: The transaction context used for event emission.
///
/// ### Requirements
/// - The caller **must be the system address** (`@0x0`). If this condition is not met,
///   the function will abort with `ENotSystemAddress`.
///
/// ### Events
/// - **`CompletedSignEvent`**: Emitted with the `session_id` and `signed_messages`,
///   signaling the completion of the sign process for the batch session.
///
/// ### Errors
/// - **`ENotSystemAddress`**: If the caller is not the system address (`@0x0`),
///   the function will abort with this error.
public(package) fun respond_sign(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    sign_id: ID,
    session_id: ID,
    signature: vector<u8>,
    is_future_sign: bool,
    rejected: bool,
    session_sequence_number: u64
) {
    self.remove_user_initiated_session_and_charge<SignRequestEvent>(session_sequence_number);
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
}

public(package) fun process_checkpoint_message_by_quorum(
    self: &mut DWalletCoordinatorInner,
    signature: vector<u8>,
    signers_bitmap: vector<u8>,
    message: vector<u8>,
) {
    let mut intent_bytes = CHECKPOINT_MESSAGE_INTENT;
    intent_bytes.append(message);
    intent_bytes.append(bcs::to_bytes(&self.current_epoch));

    self.active_committee.verify_certificate(self.current_epoch, &signature, &signers_bitmap, &intent_bytes);

    self.process_checkpoint_message(message);
}

fun process_checkpoint_message(
    self: &mut DWalletCoordinatorInner,
    message: vector<u8>,
) {
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
    while (i < len) {
        let message_data_type = bcs_body.peel_vec_length();
            // Parses checkpoint BCS bytes directly.
            // Messages with `message_data_type` 1 & 2 are handled by the system module,
            // but their bytes must be extracted here to allow correct parsing of types 3 and above.
            // This step only extracts the bytes without further processing.
            if (message_data_type == 0) {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let first_round_output = bcs_body.peel_vec_u8();
                let rejected = false;
                // TODO: Use this once we have a proper way to reject the first round
                //let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                self.respond_dwallet_dkg_first_round(dwallet_id, first_round_output, rejected, session_sequence_number);
            } else if (message_data_type == 1) {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_output = bcs_body.peel_vec_u8();
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                self.respond_dwallet_dkg_second_round(
                    dwallet_id,
                    public_output,
                    encrypted_user_secret_key_share_id,
                    session_id,
                    rejected,
                    session_sequence_number,
                );
            } else if (message_data_type == 2) {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                self.respond_re_encrypt_user_share_for(
                    dwallet_id,
                    encrypted_user_secret_key_share_id,
                    rejected,
                    session_sequence_number,
                );
            } else if (message_data_type == 3) {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let sign_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let signature = bcs_body.peel_vec_u8();
                let is_future_sign = bcs_body.peel_bool();
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                self.respond_sign(
                    dwallet_id,
                    sign_id,
                    session_id,
                    signature,
                    is_future_sign,
                    rejected,
                    session_sequence_number
                );
            } else if (message_data_type == 5) {
                let session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let partial_centralized_signed_message_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                self.respond_future_sign(
                    session_id,
                    dwallet_id,
                    partial_centralized_signed_message_id,
                    rejected,
                    session_sequence_number
                );
            } else if (message_data_type == 4) {
                let dwallet_id = bcs_body.peel_option!(|bcs_option| object::id_from_bytes(bcs_option.peel_vec_u8()));
                let presign_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let presign = bcs_body.peel_vec_u8();
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                self.respond_presign(dwallet_id, presign_id, session_id, presign, rejected, session_sequence_number);
            } else if (message_data_type == 6) {
                let dwallet_network_encryption_key_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_output = bcs_body.peel_vec_u8();
                let is_last = bcs_body.peel_bool();
                self.respond_dwallet_network_encryption_key_dkg(dwallet_network_encryption_key_id, public_output, is_last);
            } else if (message_data_type == 7) {
                let dwallet_network_encryption_key_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_output = bcs_body.peel_vec_u8();
                let is_last = bcs_body.peel_bool();
                self.respond_dwallet_network_encryption_key_reconfiguration(dwallet_network_encryption_key_id, public_output, is_last);
            } else if (message_data_type == 8) {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_user_secret_key_shares = bcs_body.peel_vec_u8();
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                self.respond_make_dwallet_user_secret_key_share_public(dwallet_id, public_user_secret_key_shares, rejected, session_sequence_number);
            } else if (message_data_type == 9) {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_output = bcs_body.peel_vec_u8();
                let encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let rejected = bcs_body.peel_bool();
                let session_sequence_number = bcs_body.peel_u64();
                self.respond_imported_key_dwallet_verification(
                    dwallet_id,
                    public_output,
                    encrypted_user_secret_key_share_id,
                    session_id,
                    rejected,
                    session_sequence_number
                );
            };
        i = i + 1;
    };
    self.total_messages_processed = self.total_messages_processed + i;
}

public(package) fun set_supported_curves_to_signature_algorithms_to_hash_schemes(
    self: &mut DWalletCoordinatorInner,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap<u32, VecMap<u32, vector<u32>>>,
) {
    self.supported_curves_to_signature_algorithms_to_hash_schemes = supported_curves_to_signature_algorithms_to_hash_schemes;
}

public(package) fun set_paused_curves_and_signature_algorithms(
    self: &mut DWalletCoordinatorInner,
    paused_curves: vector<u32>,
    paused_signature_algorithms: vector<u32>,
    paused_hash_schemes: vector<u32>,
) {
    self.paused_curves = paused_curves;
    self.paused_signature_algorithms = paused_signature_algorithms;
    self.paused_hash_schemes = paused_hash_schemes;
}
