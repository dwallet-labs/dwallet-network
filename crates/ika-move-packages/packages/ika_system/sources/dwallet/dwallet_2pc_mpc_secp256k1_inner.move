// Copyright (c) dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// TODO: remove #[allow(unused_field)]
/// This module handles the logic for creating and managing dWallets using the Secp256K1 signature scheme
/// and the DKG process. It leverages validators to execute MPC (Multi-Party Computation)
/// protocols to ensure trustless and decentralized wallet creation and key management.
#[allow(unused_field)]
module ika_system::dwallet_2pc_mpc_secp256k1_inner;

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
use ika_system::bls_committee::{BlsCommittee};

/// Supported hash schemes for message signing.
const KECCAK256: u8 = 0;
const SHA256: u8 = 1;

public(package) fun set_table_vec(_a: &mut TableVec<vector<u8>>, _b: &TableVec<vector<u8>>) {
    while (!_a.is_empty()) {
        _a.pop_back();
    };
    let mut i = 0;
    while (i < _b.length()) {
        let vec = _b.borrow(i);
        let vec_len = vec.length();
        let mut j = 0;
        let mut new_vec: vector<u8> = vector[];
        while (j < vec_len) {
            new_vec.push_back(*(vec.borrow(j)));
            j = j + 1;
        };
        _a.push_back(new_vec);
        i = i + 1;
    }
}

const CHECKPOINT_MESSAGE_INTENT: vector<u8> = vector[1, 0, 0];

public struct DWalletCoordinatorInner has store {
    current_epoch: u64,
    // TODO: change it to versioned
    /// The key is the ID of `DWallet`.
    dwallets: ObjectTable<ID, DWallet>,
    // TODO: change it to versioned
    /// The key is the ID of `DWalletNetworkDecryptionKey`.
    dwallet_network_decryption_keys: ObjectTable<ID, DWalletNetworkDecryptionKey>,
    // TODO: change it to versioned
    /// A table mapping user addresses to encryption key object IDs.
    encryption_keys: ObjectTable<address, EncryptionKey>,
    /// A table mapping id to their partial centralized signed messages.
    ecdsa_partial_centralized_signed_messages: ObjectTable<ID, ECDSAPartialUserSignature>,
    /// The computation IKA price per unit size for the current epoch.
    pricing: DWalletPricing2PcMpcSecp256K1,
    active_epochs: ObjectTable<u64, DWalletEpochCoordinator>,
    /// Sui gas fee reimbursement to fund the network writing tx responses to sui.
    gas_fee_reimbursement_sui: Balance<SUI>,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

public struct DWalletEpochCoordinator has key, store {
    id: UID,
    committee: BlsCommittee,
    session_count: u32,
    /// The total messages processed.
    total_messages_processed: u64,
    /// The last checkpoint sequence number processed.
    last_processed_checkpoint_sequence_number: Option<u32>,
    /// The fees paid for consuenes validation in IKA.
    consensus_validation_fee_charged_ika: Balance<IKA>,
}

/// Represents a capability granting control over a specific dWallet.
public struct DWalletCap has key, store {
    id: UID,
    dwallet_id: ID,
}

/// Represents a capability granting control over a specific dWallet network decryption key.
public struct DWalletNetworkDecryptionKeyCap has key, store {
    id: UID,
    dwallet_network_decryption_key_id: ID,
}



/// `DWalletNetworkDecryptionKey` represents a network decryption key of
/// the homomorphiclly encrypted netowrk share.
public struct DWalletNetworkDecryptionKey has key, store {
    id: UID,
    dwallet_network_decryption_key_cap_id: ID,
    current_epoch: u64,
    //TODO: make sure to include class gorup type and version inside the bytes with the rust code
    current_epoch_shares: table_vec::TableVec<vector<u8>>,
    //TODO: make sure to include class gorup type and version inside the bytes with the rust code
    next_epoch_shares: table_vec::TableVec<vector<u8>>,
    //TODO: make sure to include class gorup type and version inside the bytes with the rust code
    previous_epoch_shares: table_vec::TableVec<vector<u8>>,

    //TODO: make sure to include class gorup type and version inside the bytes with the rust code
    public_output: table_vec::TableVec<vector<u8>>,
    /// The fees paid for computation in IKA.
    computation_fee_charged_ika: Balance<IKA>,
    state: DWalletNetworkDecryptionKeyState,
}

public enum DWalletNetworkDecryptionKeyState has copy, drop, store {
    AwaitingNetworkDKG,
    NetworkDKGCompleted,
}


/// Represents an encryption key used to encrypt a dWallet centralized (user) secret key share.
///
/// Encryption keys facilitate secure data transfer between accounts on the
/// Ika by ensuring that sensitive information remains confidential during transmission.
/// Each address on the Ika is associated with a unique encryption key.
/// When an external party intends to send encrypted data to a particular account, they use the recipient’s
/// encryption key to encrypt the data. The recipient is then the sole entity capable of decrypting
/// and accessing this information, ensuring secure, end-to-end encryption.
public struct EncryptionKey has key, store {
    /// Unique identifier for the `EncryptionKey`.
    id: UID,

    created_at_epoch: u64,

    //TODO: make sure to include class gorup type and version inside the bytes with the rust code
    /// Serialized encryption key.
    encryption_key: vector<u8>,

    /// Signature for the encryption key, signed by the `signer_public_key`.
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

public struct UnverifiedECDSAPartialUserSignatureCap has key, store {
    /// A unique identifier for this object.
    id: UID,

    /// The unique identifier of the associated PartialCentralizedSignedMessage.
    partial_centralized_signed_message_id: ID,
}

public struct VerifiedECDSAPartialUserSignatureCap has key, store {
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
public struct ECDSAPartialUserSignature has key, store {
    /// A unique identifier for this object.
    id: UID,

    created_at_epoch: u64,

    /// The unique identifier of the associated dWallet.
    dwallet_id: ID,

    cap_id: ID,

    hash_scheme: u8,

    /// The messages that are being signed.
    message: vector<u8>,

    presign: ECDSAPresign,

    /// The centralized party signature of a message.
    message_centralized_signature: vector<u8>,

    state: ECDSAPartialUserSignatureState,
}

public enum ECDSAPartialUserSignatureState has copy, drop, store {
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

    /// The ID of the capability associated with this dWallet.
    dwallet_cap_id: ID,

    /// The MPC network decryption key id that is used to decrypt this dWallet.
    dwallet_network_decryption_key_id: ID,

    /// A table mapping id to their encryption key object.
    encrypted_user_secret_key_shares: ObjectTable<ID, EncryptedUserSecretKeyShare>,

    ecdsa_presigns: ObjectTable<ID, ECDSAPresign>,

    ecdsa_signs: ObjectTable<ID, ECDSASign>,

    state: DWalletState,
}

public enum DWalletState has copy, drop, store {
    Requested,
    AwaitingUser {
        first_round_output: vector<u8>,
    },
    AwaitingNetworkVerification,
    NetworkRejectedSecondRound,
    Active {
        /// The output of the DKG process.
        public_output: vector<u8>,
    }
}

/// Represents the result of the second and final presign round.
/// This struct links the results of both presign rounds to a specific dWallet ID.
public struct ECDSAPresign has key, store {
    /// Unique identifier for the presign object.
    id: UID,

    created_at_epoch: u64,

    /// ID of the associated dWallet.
    dwallet_id: ID,

    /// Serialized output of the presign process.
    presign: vector<u8>,
}

/// The output of a batched Sign session.
public struct ECDSASign has key, store {
    /// A unique identifier for the batched sign output.
    id: UID,

    created_at_epoch: u64,

    /// The unique identifier of the associated dWallet.
    dwallet_id: ID,

    /// The session identifier for the sign process.
    session_id: ID,

    state: ECDSASignState,
}

public enum ECDSASignState has copy, drop, store {
    Requested,
    NetworkRejected,
    Completed {
        signature: vector<u8>,
    }
}

public struct DWalletEvent<E: copy + drop> has copy, drop {
    epoch: u64,
    session_id: ID,
    event_data: E,
}

/// Event emitted when an encryption key is created.
///
/// This event is emitted after the blockchain verifies the encryption key's validity
/// and creates the corresponding `EncryptionKey` object.
public struct CreatedEncryptionKeyEvent has copy, drop {
    /// The unique identifier of the created `EncryptionKey` object.
    encryption_key_id: ID,

    signer_address: address,
}

public struct DWalletNetworkDKGDecryptionKeyRequestEvent has copy, drop {
    dwallet_network_decryption_key_id: ID,
}

/// An event emitted when the first round of the DKG process is completed.
///
/// This event is emitted by the blockchain to notify the user about
/// the completion of the first round.
/// The user should catch this event to generate inputs for
/// the second round and call the `request_dwallet_dkg_second_round()` function.
public struct CompletedDWalletNetworkDKGDecryptionKeyEvent has copy, drop {
       dwallet_network_decryption_key_id: ID,
       public_output: vector<u8>,
}

// DKG TYPES

/// Event emitted to start the first round of the DKG process.
///
/// This event is caught by the blockchain, which is then using it to
/// initiate the first round of the DKG.
public struct DWalletDKGFirstRoundRequestEvent has copy, drop {
    /// The unique session identifier for the DKG process.
    dwallet_id: ID,

    /// The identifier for the dWallet capability.
    dwallet_cap_id: ID,

    /// The MPC network decryption key id that is used to decrypt associated dWallet.
    dwallet_network_decryption_key_id: ID,
}

/// An event emitted when the first round of the DKG process is completed.
///
/// This event is emitted by the blockchain to notify the user about
/// the completion of the first round.
/// The user should catch this event to generate inputs for
/// the second round and call the `request_dwallet_dkg_second_round()` function.
public struct CompletedDKGFirstdRoundEvent has copy, drop {
    /// The unique session identifier for the DKG process.
    dwallet_id: ID,

    /// The decentralized public output data produced by the first round of the DKG process.
    first_round_output: vector<u8>,
}

/// Event emitted to initiate the second round of the DKG process.
///
/// This event is emitted to notify Validators to begin the second round of the DKG.
/// It contains all necessary data to ensure proper continuation of the process.
public struct DWalletDKGSecondRoundRequestEvent has copy, drop {
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
    singer_public_key: vector<u8>,
}

/// Event emitted upon the completion of the second (and final) round of the
/// Distributed Key Generation (DKG).
///
/// This event provides all necessary data generated from the second
/// round of the DKG process.
/// Emitted to notify the centralized party.
public struct CompletedDWalletDKGSecondRoundEvent has copy, drop {
    /// The identifier of the dWallet created as a result of the DKG process.
    dwallet_id: ID,

    /// The public output for the second round of the DKG process.
    public_output: vector<u8>,
    encrypted_user_secret_key_share_id: ID,
    session_id: ID
}

public struct RejectedDWalletDKGSecondRoundEvent has copy, drop {
    /// The identifier of the dWallet created as a result of the DKG process.
    dwallet_id: ID,

    /// The public output for the second round of the DKG process.
    public_output: vector<u8>,
}

// END OF DKG TYPES

// ENCRYPTED USER SHARE TYPES



/// Event emitted to start an encrypted dWallet centralized (user) key share
/// verification process.
/// Ika does not support native functions, so an event is emitted and
/// caught by the blockchain, which then starts the verification process,
/// similar to the MPC processes.
public struct EncryptedShareVerificationRequestEvent has copy, drop {
    /// Encrypted centralized secret key share and the associated cryptographic proof of encryption.
    encrypted_centralized_secret_share_and_proof: vector<u8>,

    /// The public output of the centralized party,
    /// belongs to the dWallet that its centralized
    /// secret share is being encrypted.
    /// This is not passed by the user,
    /// but taken from the blockhain during event creation.
    public_output: vector<u8>,

    /// The ID of the dWallet that this encrypted secret key share belongs to.
    dwallet_id: ID,

    /// The encryption key used to encrypt the secret key share with.
    encryption_key: vector<u8>,

    /// The `EncryptionKey` Move object ID.
    encryption_key_id: ID,

    encrypted_user_secret_key_share_id: ID,

    source_encrypted_user_secret_key_share_id: ID,
}

public struct CompletedEncryptedShareVerificationEvent has copy, drop {
    /// The ID of the `EncryptedUserSecretKeyShare` Move object.
    encrypted_user_secret_key_share_id: ID,

    /// The ID of the dWallet associated with this encrypted secret share.
    dwallet_id: ID,
}

public struct RejectedEncryptedShareVerificationEvent has copy, drop {
    /// The ID of the `EncryptedUserSecretKeyShare` Move object.
    encrypted_user_secret_key_share_id: ID,

    /// The ID of the dWallet associated with this encrypted secret share.
    dwallet_id: ID,
}

public struct AcceptReEncryptedUserShareEvent has copy, drop {
    /// The ID of the `EncryptedUserSecretKeyShare` Move object.
    encrypted_user_secret_key_share_id: ID,

    /// The ID of the dWallet associated with this encrypted secret share.
    dwallet_id: ID,

    user_output_signature: vector<u8>,

    encryption_key_id: ID,

    encryption_key_address: address,
}
// END OF ENCRYPTED USER SHARE TYPES

// PRESIGN TYPES

/// Event emitted to initiate the first round of a Presign session.
///
/// This event is used to signal Validators to start the
/// first round of the Presign process.
/// The event includes all necessary details to link
/// the session to the corresponding dWallet
/// and DKG process.
public struct ECDSAPresignRequestEvent has copy, drop {
    /// ID of the associated dWallet.
    dwallet_id: ID,

    /// The output produced by the DKG process,
    /// used as input for the Presign session.
    dwallet_public_output: vector<u8>,

    /// The MPC network decryption key id that is used to decrypt associated dWallet.
    dwallet_network_decryption_key_id: ID,
}

/// Event emitted when the presign batch is completed.
///
/// This event indicates the successful completion of a batched presign process.
/// It provides details about the presign objects created and their associated metadata.
public struct CompletedECDSAPresignEvent has copy, drop {
    /// The ID of the dWallet associated with this batch.
    dwallet_id: ID,

    /// The session ID.
    session_id: ID,
    presign_id: ID,
    presign: vector<u8>,
}

// END OF PRESIGN TYPES


/// Event emitted to initiate the signing process.
///
/// This event is captured by Validators to start the signing protocol.
/// It includes all the necessary information to link the signing process
/// to a specific dWallet, and batched process.
/// D: The type of data that can be stored with the object,
/// specific to each Digital Signature Algorithm.
public struct ECDSASignRequestEvent has copy, drop {
    sign_id: ID,

    /// The unique identifier for the dWallet used in the session.
    dwallet_id: ID,

    /// The output from the dWallet DKG process used in this session.
    dwallet_public_output: vector<u8>,

    hash_scheme: u8,

    /// The message to be signed in this session.
    message: vector<u8>,

    /// The MPC network decryption key id that is used to decrypt associated dWallet.
    dwallet_network_decryption_key_id: ID,

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
public struct ECDSAFutureSignRequestEvent has copy, drop {
    dwallet_id: ID,
    partial_centralized_signed_message_id: ID,
    message: vector<u8>,
    presign: vector<u8>,
    hash_scheme: u8,
    message_centralized_signature: vector<u8>,
}

public struct CompletedECDSAFutureSignEvent has copy, drop {
    dwallet_id: ID,
    partial_centralized_signed_message_id: ID,
}

public struct RejectedECDSAFutureSignEvent has copy, drop {
    dwallet_id: ID,
    partial_centralized_signed_message_id: ID,
}

/// Event emitted to signal the completion of a Sign process.
///
/// This event contains signatures for all signed messages in the batch.
public struct CompletedECDSASignEvent has copy, drop {
    sign_id: ID,

    /// The session identifier for the signing process.
    session_id: ID,

    /// List of signatures in the same order as the sign function message approvals input.
    signature: vector<u8>,

    /// Indicates whether the future sign feature was used to start the session.
    is_future_sign: bool,
}

public struct RejectedECDSASignEvent has copy, drop {
    sign_id: ID,

    /// The session identifier for the signing process.
    session_id: ID,

    /// Indicates whether the future sign feature was used to start the session.
    is_future_sign: bool,
}

/// Event containing system-level checkpoint information, emitted during
/// the checkpoint submmision message.
public struct SystemCheckpointInfoEvent has copy, drop {
    epoch: u64,
    sequence_number: u32,
    timestamp_ms: u64,
}

// <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
const EEpochNotExist: u64 = 0;
const EDWalletMismatch: u64 = 1;
const EDWalletInactive: u64 = 2;
const EDWalletNotExists: u64 = 3;
const EWrongState: u64 = 4;
const EDWalletNetworkDecryptionKeyNotExist: u64 = 5;
const EInvalidEncryptionKeySignature: u64 = 6;
const EMessageApprovalMismatch: u64 = 7;
const EInvalidHashScheme: u64 = 8;
const ESignWrongState: u64 = 9;
const EPresignNotExist: u64 = 10;
const EIncorrectCap: u64 = 11;
const EUnverifiedCap: u64 = 12;
const EInvalidSource: u64 =13;
const EDWalletNetworkDecryptionKeyNotActive: u64 = 14;

#[error]
const EIncorrectEpochInCheckpoint: vector<u8> = b"The checkpoint epoch is incorrect.";

#[error]
const EWrongCheckpointSequenceNumber: vector<u8> = b"The checkpoint sequence number should be the expected next one.";

#[error]
const EActiveBlsCommitteeMustInitialize: vector<u8> = b"First active committee must initialize.";
// >>>>>>>>>>>>>>>>>>>>>>>> Error codes >>>>>>>>>>>>>>>>>>>>>>>>

public(package) fun create_dwallet_coordinator_inner(
    current_epoch: u64,
    committee: BlsCommittee,
    pricing: DWalletPricing2PcMpcSecp256K1,
    ctx: &mut TxContext
): DWalletCoordinatorInner {
    let mut active_epochs = object_table::new(ctx);
    active_epochs.add(current_epoch, DWalletEpochCoordinator {
        id: object::new(ctx),
        committee,
        session_count: 0,
        total_messages_processed: 0,
        last_processed_checkpoint_sequence_number: option::none(),
        consensus_validation_fee_charged_ika: balance::zero(),

    });
    DWalletCoordinatorInner {
        current_epoch,
        dwallets: object_table::new(ctx),
        dwallet_network_decryption_keys: object_table::new(ctx),
        encryption_keys: object_table::new(ctx),
        ecdsa_partial_centralized_signed_messages: object_table::new(ctx),
        pricing,
        active_epochs,
        gas_fee_reimbursement_sui: balance::zero(),
        extra_fields: bag::new(ctx),
    }
}

public(package) fun request_dwallet_network_decryption_key_dkg(
    self: &mut DWalletCoordinatorInner,
    ctx: &mut TxContext
): DWalletNetworkDecryptionKeyCap {
    let id = object::new(ctx);
    let dwallet_network_decryption_key_id = id.to_inner();
    let cap = DWalletNetworkDecryptionKeyCap {
        id: object::new(ctx),
        dwallet_network_decryption_key_id,
    };
    self.dwallet_network_decryption_keys.add(dwallet_network_decryption_key_id, DWalletNetworkDecryptionKey {
        id,
        dwallet_network_decryption_key_cap_id: object::id(&cap),
        current_epoch: self.current_epoch,
        //TODO: make sure to include class gorup type and version inside the bytes with the rust code
        current_epoch_shares: table_vec::empty(ctx),
        //TODO: make sure to include class gorup type and version inside the bytes with the rust code
        next_epoch_shares: table_vec::empty(ctx),
        //TODO: make sure to include class gorup type and version inside the bytes with the rust code
        previous_epoch_shares: table_vec::empty(ctx),
        public_output: table_vec::empty(ctx),
        computation_fee_charged_ika: balance::zero(),
        state: DWalletNetworkDecryptionKeyState::AwaitingNetworkDKG,
    });
    event::emit(self.create_current_epoch_dwallet_event(
        DWalletNetworkDKGDecryptionKeyRequestEvent {
            dwallet_network_decryption_key_id
        },
        ctx,
    ));
    cap
}

public(package) fun respond_dwallet_network_decryption_key_dkg(
    self: &mut DWalletCoordinatorInner,
    dwallet_network_decryption_key_id: ID,
    public_output: vector<u8>,
    key_shares: vector<u8>,
    is_last: bool,
) {
    let dwallet_network_decryption_key = self.dwallet_network_decryption_keys.borrow_mut(dwallet_network_decryption_key_id);
    dwallet_network_decryption_key.public_output.push_back(public_output);
    dwallet_network_decryption_key.current_epoch_shares.push_back(key_shares);
    dwallet_network_decryption_key.state = match (&dwallet_network_decryption_key.state) {
        DWalletNetworkDecryptionKeyState::AwaitingNetworkDKG => {
            if (is_last) {
                event::emit(CompletedDWalletNetworkDKGDecryptionKeyEvent {
                    dwallet_network_decryption_key_id,
                    public_output
                });
                DWalletNetworkDecryptionKeyState::NetworkDKGCompleted
            } else {
                DWalletNetworkDecryptionKeyState::AwaitingNetworkDKG
            }
        },
        _ => abort EWrongState
    };
}

public(package) fun respond_dwallet_network_decryption_key_reconfiguration(
    self: &mut DWalletCoordinatorInner,
    dwallet_network_decryption_key_id: ID,
    key_shares: vector<u8>,
) {
    let dwallet_network_decryption_key = self.dwallet_network_decryption_keys.borrow_mut(dwallet_network_decryption_key_id);
    dwallet_network_decryption_key.next_epoch_shares.push_back(key_shares);
}

public(package) fun advance_epoch_dwallet_network_decryption_key(
    self: &mut DWalletCoordinatorInner,
    cap: &DWalletNetworkDecryptionKeyCap,
) {
    let dwallet_network_decryption_key = self.get_active_dwallet_network_decryption_key(cap.dwallet_network_decryption_key_id);
    assert!(dwallet_network_decryption_key.dwallet_network_decryption_key_cap_id == cap.id.to_inner(), EIncorrectCap);
    dwallet_network_decryption_key.current_epoch = dwallet_network_decryption_key.current_epoch + 1;
    set_table_vec(&mut dwallet_network_decryption_key.previous_epoch_shares, &dwallet_network_decryption_key.current_epoch_shares);
    set_table_vec(&mut dwallet_network_decryption_key.current_epoch_shares, &dwallet_network_decryption_key.next_epoch_shares);
}

fun get_active_dwallet_network_decryption_key(
    self: &mut DWalletCoordinatorInner,
    dwallet_network_decryption_key_id: ID,
): &mut DWalletNetworkDecryptionKey {
    let dwallet_network_decryption_key = self.dwallet_network_decryption_keys.borrow_mut(dwallet_network_decryption_key_id);
    assert!(dwallet_network_decryption_key.state == DWalletNetworkDecryptionKeyState::NetworkDKGCompleted, EDWalletNetworkDecryptionKeyNotActive);
    dwallet_network_decryption_key
}

public(package) fun advance_epoch(
    self: &mut DWalletCoordinatorInner,
    next_committee: BlsCommittee,
    ctx: &mut TxContext
) {
    self.current_epoch = self.current_epoch + 1;
    self.active_epochs.add(self.current_epoch, DWalletEpochCoordinator {
        id: object::new(ctx),
        committee: next_committee,
        session_count: 0,
        total_messages_processed: 0,
        last_processed_checkpoint_sequence_number: option::none(),
        consensus_validation_fee_charged_ika: balance::zero(),
    });
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
        DWalletState::Requested | DWalletState::AwaitingUser { .. } | DWalletState::AwaitingNetworkVerification | DWalletState::NetworkRejectedSecondRound => abort EDWalletInactive,
    }
}

fun create_current_epoch_dwallet_event<E: copy + drop>(
    self: &DWalletCoordinatorInner,
    event_data: E,
    ctx: &mut TxContext,
): DWalletEvent<E> {
    DWalletEvent {
        epoch: self.current_epoch,
        session_id: object::id_from_address(tx_context::fresh_object_address(ctx)),
        event_data,
    }
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
    self.encryption_keys.borrow(address).id.to_inner()
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
    encryption_key: vector<u8>,
    encryption_key_signature: vector<u8>,
    signer_public_key: vector<u8>,
    ctx: &mut TxContext
) {
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

/// Represents a message that was approved as part of a dWallet process.
///
/// This struct binds the message to a specific `DWalletCap` for
/// traceability and accountability within the system.
///
/// ### Fields
/// - **`dwallet_cap_id`**: The identifier of the dWallet capability
///   associated with this approval.
/// - **`hash_scheme`**: The message hash scheme.
/// - **`message`**: The message that has been approved.
public struct MessageApproval has store, drop {
    dwallet_id: ID,
    hash_scheme: u8,
    message: vector<u8>,
}

/// Creates a `MessageApproval` object.
public(package) fun create_message_approval(
    dwallet_id: ID,
    hash_scheme: u8,
    message: vector<u8>,
): MessageApproval {
    assert!(is_supported_hash_scheme(hash_scheme), EInvalidHashScheme);
    let approval = MessageApproval {
        dwallet_id,
        hash_scheme,
        message,
    };
    approval
}

/// Approves a set of messages for a specific dWallet capability.
///
/// This function creates a list of `MessageApproval` objects for a given set of messages.
/// Each message is associated with the same `dWalletCap` and `hash_scheme`. The messages
/// must be approved in the same order as they were created to maintain their sequence.
///
/// ### Parameters
/// - `dwallet_cap`: A reference to the `DWalletCap` object representing the capability for which
///   the messages are being approved.
/// - `hash_scheme`: The hash scheme to be used for hashing the messages. For example:
///   - `KECCAK256`
///   - `SHA256`
/// - `messages`: A mutable vector containing the messages to be approved. The messages are removed
///   from this vector as they are processed and added to the approvals list.
///
/// ### Returns
/// A vector of `MessageApproval` objects corresponding to the approved messages.
///
/// ### Behavior
/// - The function iterates over the provided `messages` vector, processes each message by creating
///   a `MessageApproval` object, and pushes it into the `message_approvals` vector.
/// - The messages are approved in reverse order and then reversed again to preserve their original order.
///
/// ### Aborts
/// - Aborts if the provided `hash_scheme` is not supported by the system (checked during `create_message_approval`).
public fun approve_message(
    dwallet_cap: &DWalletCap,
    hash_scheme: u8,
    message: vector<u8>
): MessageApproval {
    create_message_approval(
        dwallet_cap.dwallet_id,
        hash_scheme,
        message,
    )
}

/// Checks if the given hash scheme is supported for message signing.
fun is_supported_hash_scheme(val: u8): bool {
    return match (val) {
            KECCAK256 | SHA256 => true,
    _ => false,
    }
}

fun charge(
    self: &mut DWalletCoordinatorInner,
    pricing: PricingPerOperation,
    dwallet_network_decryption_key_id: ID,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    assert!(self.dwallet_network_decryption_keys.contains(dwallet_network_decryption_key_id), EDWalletNetworkDecryptionKeyNotExist);
    assert!(self.active_epochs.contains(self.current_epoch), EEpochNotExist);

    let active_epoch = self.active_epochs.borrow_mut(self.current_epoch);
    active_epoch.consensus_validation_fee_charged_ika.join(payment_ika.split(pricing.consensus_validation_ika(), ctx).into_balance());
    active_epoch.session_count = active_epoch.session_count + 1;

    let dwallet_network_decryption_key = self.get_active_dwallet_network_decryption_key(dwallet_network_decryption_key_id);
    dwallet_network_decryption_key.computation_fee_charged_ika.join(payment_ika.split(pricing.computation_ika(), ctx).into_balance());

    self.gas_fee_reimbursement_sui.join(payment_sui.split(pricing.gas_fee_reimbursement_sui(), ctx).into_balance());
}

/// Starts the first Distributed Key Generation (DKG) session.
///
/// This function creates a new `DWalletCap` object,
/// transfers it to the session initiator,
/// and emits a `DWalletDKGFirstRoundRequestEvent` to signal
/// the beginning of the DKG process.
///
/// ### Parameters
///
/// ### Effects
/// - Generates a new `DWalletCap` object.
/// - Transfers the `DWalletCap` to the session initiator (`ctx.sender`).
/// - Emits a `DWalletDKGFirstRoundRequestEvent`.
public(package) fun request_dwallet_dkg_first_round(
    self: &mut DWalletCoordinatorInner,
    dwallet_network_decryption_key_id: ID,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): DWalletCap {
    let pricing = self.pricing.dkg_first_round();

    self.charge(pricing, dwallet_network_decryption_key_id, payment_ika, payment_sui, ctx);

    assert!(self.dwallet_network_decryption_keys.contains(dwallet_network_decryption_key_id), EDWalletNetworkDecryptionKeyNotExist);
    let id = object::new(ctx);
    let dwallet_id = id.to_inner();
    let dwallet_cap = DWalletCap {
        id: object::new(ctx),
        dwallet_id,
    };
    let dwallet_cap_id = object::id(&dwallet_cap);
    self.dwallets.add(dwallet_id, DWallet {
        id,
        created_at_epoch: self.current_epoch,
        dwallet_cap_id,
        dwallet_network_decryption_key_id,
        encrypted_user_secret_key_shares: object_table::new(ctx),
        ecdsa_presigns: object_table::new(ctx),
        ecdsa_signs: object_table::new(ctx),
        state: DWalletState::Requested,
    });
    event::emit(self.create_current_epoch_dwallet_event(
        DWalletDKGFirstRoundRequestEvent {
            dwallet_id,
            dwallet_cap_id,
            dwallet_network_decryption_key_id,
        },
        ctx,
    ));
    dwallet_cap
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
) {
    let dwallet = self.get_dwallet_mut(dwallet_id);
    dwallet.state = match (dwallet.state) {
        DWalletState::Requested => {
            event::emit(CompletedDKGFirstdRoundEvent {
                dwallet_id,
                first_round_output,
            });
            DWalletState::AwaitingUser {
                first_round_output
            }
        },
        _ => abort EWrongState
    };
}

// TODO (#493): Remove mock functions
public(package) fun create_first_round_dwallet_mock(
    self: &mut DWalletCoordinatorInner, first_round_output: vector<u8>, dwallet_network_decryption_key_id: ID, ctx: &mut TxContext
): DWalletCap {
    let id = object::new(ctx);
    let dwallet_id = id.to_inner();
    let dwallet_cap = DWalletCap {
        id: object::new(ctx),
        dwallet_id,
    };
    let dwallet_cap_id = object::id(&dwallet_cap);
    self.dwallets.add(dwallet_id, DWallet {
        id,
        created_at_epoch: self.current_epoch,
        dwallet_cap_id,
        dwallet_network_decryption_key_id,
        encrypted_user_secret_key_shares: object_table::new(ctx),
        ecdsa_presigns: object_table::new(ctx),
        ecdsa_signs: object_table::new(ctx),
        state: DWalletState::AwaitingUser {
            first_round_output
        },
    });
    dwallet_cap
}

// TODO (#493): Remove mock functions
public(package) fun mock_create_dwallet(
    self: &mut DWalletCoordinatorInner, output: vector<u8>, dwallet_network_decryption_key_id: ID, ctx: &mut TxContext
): DWalletCap {
    let id = object::new(ctx);
    let dwallet_id = id.to_inner();
    let dwallet_cap = DWalletCap {
        id: object::new(ctx),
        dwallet_id,
    };
    let dwallet_cap_id = object::id(&dwallet_cap);
    self.dwallets.add(dwallet_id, DWallet {
        id,
        created_at_epoch: self.current_epoch,
        dwallet_cap_id,
        dwallet_network_decryption_key_id,
        encrypted_user_secret_key_shares: object_table::new(ctx),
        ecdsa_presigns: object_table::new(ctx),
        ecdsa_signs: object_table::new(ctx),
        state: DWalletState::Active {
            public_output: output
        },
    });
    dwallet_cap
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
/// - `singer_public_key`: The Ed25519 public key of the initiator,
///    used to verify the signature on the public output.
public(package) fun request_dwallet_dkg_second_round(
    self: &mut DWalletCoordinatorInner,
    dwallet_cap: &DWalletCap,
    centralized_public_key_share_and_proof: vector<u8>,
    encrypted_centralized_secret_share_and_proof: vector<u8>,
    encryption_key_address: address,
    user_public_output: vector<u8>,
    singer_public_key: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    let encryption_key = self.encryption_keys.borrow(encryption_key_address);
    let encryption_key_id = encryption_key.id.to_inner();
    let encryption_key = encryption_key.encryption_key;

    let dwallet = self.get_dwallet(dwallet_cap.dwallet_id);

    let first_round_output = match (&dwallet.state) {
        DWalletState::AwaitingUser {
            first_round_output,
        } => {
            *first_round_output
        },
        _ => abort EWrongState
    };

    let pricing = self.pricing.dkg_second_round();

    self.charge(pricing, dwallet.dwallet_network_decryption_key_id, payment_ika, payment_sui, ctx);


    let emit_event = self.create_current_epoch_dwallet_event(
        DWalletDKGSecondRoundRequestEvent {
            dwallet_id: dwallet_cap.dwallet_id,
            first_round_output,
            centralized_public_key_share_and_proof,
            dwallet_cap_id: object::id(dwallet_cap),
            encrypted_centralized_secret_share_and_proof,
            encryption_key,
            encryption_key_id,
            encryption_key_address,
            user_public_output,
            singer_public_key,
        },
        ctx,
    );

    event::emit(emit_event);

    let dwallet = self.get_dwallet_mut(dwallet_cap.dwallet_id);
    dwallet.state = DWalletState::AwaitingNetworkVerification;
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
    encrypted_centralized_secret_share_and_proof: vector<u8>,
    encryption_key_address: address,
    session_id: ID,
    rejected: bool,
    ctx: &mut TxContext
) {
    let encryption_key = self.encryption_keys.borrow(encryption_key_address);
    let encryption_key_id = encryption_key.id.to_inner();
    let created_at_epoch = self.current_epoch;
    let dwallet = self.get_dwallet_mut(dwallet_id);

    dwallet.state = match (&dwallet.state) {
        DWalletState::AwaitingNetworkVerification => {
            if (rejected) {
                event::emit(RejectedDWalletDKGSecondRoundEvent {
                    dwallet_id,
                    public_output,
                });
                DWalletState::NetworkRejectedSecondRound
            } else {
                let encrypted_user_share = EncryptedUserSecretKeyShare {
                    id: object::new(ctx),
                    created_at_epoch,
                    dwallet_id,
                    encrypted_centralized_secret_share_and_proof,
                    encryption_key_id,
                    encryption_key_address,
                    source_encrypted_user_secret_key_share_id: option::none(),
                    state: EncryptedUserSecretKeyShareState::NetworkVerificationCompleted
                };
                let encrypted_user_secret_key_share_id = object::id(&encrypted_user_share);
                dwallet.encrypted_user_secret_key_shares.add(encrypted_user_secret_key_share_id, encrypted_user_share);

                event::emit(CompletedDWalletDKGSecondRoundEvent {
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

    let dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    let pricing = self.pricing.re_encrypt_user_share();

    self.charge(pricing, dwallet_network_decryption_key_id, payment_ika, payment_sui, ctx);


    event::emit(
        self.create_current_epoch_dwallet_event(
            EncryptedShareVerificationRequestEvent {
                encrypted_centralized_secret_share_and_proof,
                public_output,
                dwallet_id,
                encryption_key: destination_encryption_key,
                encryption_key_id: destination_encryption_key_id,
                encrypted_user_secret_key_share_id,
                source_encrypted_user_secret_key_share_id,
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
/// - `singer_public_key`: The Ed25519 public key of the encryptor, used for signing.
/// - `initiator`: The address of the entity that performed the encryption operation of this secret key share.
public(package) fun respond_re_encrypt_user_share_for(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    encrypted_user_secret_key_share_id: ID,
    rejected: bool,
) {
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
    let (dwallet, public_output) = self.get_active_dwallet_and_public_output(dwallet_id);
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
        AcceptReEncryptedUserShareEvent {
            encrypted_user_secret_key_share_id,
            dwallet_id,
            user_output_signature,
            encryption_key_id,
            encryption_key_address,
        }
    );
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
public(package) fun request_ecdsa_presign(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    let (dwallet, public_output) = self.get_active_dwallet_and_public_output(dwallet_id);

    let dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;

    let pricing = self.pricing.ecdsa_presign();

    self.charge(pricing, dwallet_network_decryption_key_id, payment_ika, payment_sui, ctx);

    event::emit(
        self.create_current_epoch_dwallet_event(
            ECDSAPresignRequestEvent {
                dwallet_id,
                dwallet_public_output: public_output,
                dwallet_network_decryption_key_id: dwallet_network_decryption_key_id,
            },
            ctx,
        )
    );
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
public(package) fun respond_ecdsa_presign(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    session_id: ID,
    presign: vector<u8>,
    ctx: &mut TxContext
) {
    let created_at_epoch = self.current_epoch;
    let (dwallet, _) = self.get_active_dwallet_and_public_output_mut(dwallet_id);

    let id = object::new(ctx);
    let presign_id = id.to_inner();
    dwallet.ecdsa_presigns.add(presign_id, ECDSAPresign {
        id,
        created_at_epoch,
        dwallet_id,
        presign,
    });
    event::emit(CompletedECDSAPresignEvent {
        dwallet_id,
        session_id,
        presign_id,
        presign
    });
}

/// Emits events to initiate the signing process for each message.
///
/// This function ensures that all messages have the correct approvals, calculates
/// their hashes, and emits signing events.
///
/// # Effects
/// - Checks that the number of `signature_algorithm_data` items matches `message_approvals`.
/// - Generates a new session ID for batch signing.
/// - Emits `RequestedBatchedSignEvent` containing session details and hashed messages.
/// - Iterates through `signature_algorithm_data`, verifying approvals and emitting `RequestedSignEvent` for each.
///
/// # Aborts
/// - **`EExtraDataAndMessagesLenMismatch`**: If `signature_algorithm_data` and `message_approvals` have different lengths.
/// - **`EMissingApprovalOrWrongApprovalOrder`**: If message approvals are incorrect or missing.
fun emit_ecdsa_sign_event(
    self: &mut DWalletCoordinatorInner,
    message_approval: MessageApproval,
    dwallet_id: ID,
    presign: ECDSAPresign,
    message_centralized_signature: vector<u8>,
    is_future_sign: bool,
    ctx: &mut TxContext
) {
    let created_at_epoch = self.current_epoch;
    let (dwallet, public_output) = self.get_active_dwallet_and_public_output_mut(dwallet_id);

    let MessageApproval {
        dwallet_id: message_approval_dwallet_id,
        hash_scheme,
        message
    } = message_approval;
    let ECDSAPresign {
        id,
        created_at_epoch: _,
        dwallet_id: presign_dwallet_id,
        presign,
    } = presign;
    let presign_id = id.to_inner();
    id.delete();
    assert!(presign_dwallet_id == message_approval_dwallet_id, EMessageApprovalMismatch);

    let id = object::new(ctx);
    let sign_id = id.to_inner();
    let dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    let emit_event = self.create_current_epoch_dwallet_event(
        ECDSASignRequestEvent {
            sign_id,
            dwallet_id,
            dwallet_public_output: public_output,
            hash_scheme,
            message,
            dwallet_network_decryption_key_id,
            presign_id,
            presign,
            message_centralized_signature,
            is_future_sign,
        },
        ctx,
    );
    let session_id = emit_event.session_id;
    let dwallet = self.get_dwallet_mut(dwallet_id);
    dwallet.ecdsa_signs.add(sign_id, ECDSASign {
        id,
        created_at_epoch,
        dwallet_id,
        session_id,
        state: ECDSASignState::Requested,
    });

    event::emit(emit_event);
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
public(package) fun request_ecdsa_sign(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    message_approval: MessageApproval,
    presign_id: ID,
    message_centralized_signature: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    let (dwallet, _) = self.get_active_dwallet_and_public_output_mut(dwallet_id);

    assert!(dwallet.ecdsa_presigns.contains(presign_id), EPresignNotExist);
    let presign = dwallet.ecdsa_presigns.remove(presign_id);
    let dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    let pricing = self.pricing.ecdsa_sign();

    self.charge(pricing, dwallet_network_decryption_key_id, payment_ika, payment_sui, ctx);

    self.emit_ecdsa_sign_event(
        message_approval,
        dwallet_id,
        presign,
        message_centralized_signature,
        false,
        ctx
    );
}

// TODO: add hash_scheme per message so we can validate that.
/// A function to publish messages signed by the user on chain with on-chain verification,
/// without launching the chain's sign flow immediately.
///
/// See the docs of [`PartialCentralizedSignedMessages`] for
/// more details on when this may be used.
public(package) fun request_ecdsa_future_sign(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    message: vector<u8>,
    presign_id: ID,
    hash_scheme: u8,
    message_centralized_signature: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): UnverifiedECDSAPartialUserSignatureCap {
    let (dwallet, _) = self.get_active_dwallet_and_public_output_mut(dwallet_id);
    let dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;

    // TODO: Change error
    assert!(dwallet.ecdsa_presigns.contains(presign_id), EPresignNotExist);

    let presign = dwallet.ecdsa_presigns.remove(presign_id);
    let id = object::new(ctx);
    let partial_centralized_signed_message_id = id.to_inner();
    let cap = UnverifiedECDSAPartialUserSignatureCap {
        id: object::new(ctx),
        partial_centralized_signed_message_id,
    };
    let emit_event = self.create_current_epoch_dwallet_event(
        ECDSAFutureSignRequestEvent {
                dwallet_id,
                partial_centralized_signed_message_id,
                message,
                presign: presign.presign,
                hash_scheme,
                message_centralized_signature
        },
        ctx,
    );
    self.ecdsa_partial_centralized_signed_messages.add(partial_centralized_signed_message_id, ECDSAPartialUserSignature {
        id: id,
        created_at_epoch: self.current_epoch,
        dwallet_id,
        cap_id: object::id(&cap),
        hash_scheme,
        message,
        presign,
        message_centralized_signature,
        state: ECDSAPartialUserSignatureState::AwaitingNetworkVerification,
    });

    let pricing = self.pricing.ecdsa_future_sign();

    self.charge(pricing, dwallet_network_decryption_key_id, payment_ika, payment_sui, ctx);


    event::emit(emit_event);

    cap
}

public(package) fun respond_ecdsa_future_sign(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    partial_centralized_signed_message_id: ID,
    rejected: bool,
) {
    let partial_centralized_signed_message = self.ecdsa_partial_centralized_signed_messages.borrow_mut(partial_centralized_signed_message_id);
    assert!(partial_centralized_signed_message.dwallet_id == dwallet_id, EDWalletMismatch);
    partial_centralized_signed_message.state = match(partial_centralized_signed_message.state) {
        ECDSAPartialUserSignatureState::AwaitingNetworkVerification => {
            if(rejected) {
                event::emit(RejectedECDSAFutureSignEvent {
                    dwallet_id,
                    partial_centralized_signed_message_id
                });
                ECDSAPartialUserSignatureState::NetworkVerificationRejected
            } else {
                event::emit(CompletedECDSAFutureSignEvent {
                    dwallet_id,
                    partial_centralized_signed_message_id
                });
                ECDSAPartialUserSignatureState::NetworkVerificationCompleted
            }
        },
        _ => abort EWrongState
    }
}

public(package) fun verifiy_ecdsa_partial_user_signature_cap(
    self: &mut DWalletCoordinatorInner,
    cap: UnverifiedECDSAPartialUserSignatureCap,
    ctx: &mut TxContext
): VerifiedECDSAPartialUserSignatureCap {
    let UnverifiedECDSAPartialUserSignatureCap {
        id,
        partial_centralized_signed_message_id
    } = cap;
    let cap_id = id.to_inner();
    id.delete();
    let partial_centralized_signed_message = self.ecdsa_partial_centralized_signed_messages.borrow_mut(partial_centralized_signed_message_id);
    assert!(partial_centralized_signed_message.cap_id == cap_id, EIncorrectCap);
    assert!(partial_centralized_signed_message.state == ECDSAPartialUserSignatureState::NetworkVerificationCompleted, EUnverifiedCap);
    let cap = VerifiedECDSAPartialUserSignatureCap {
        id: object::new(ctx),
        partial_centralized_signed_message_id,
    };
    partial_centralized_signed_message.cap_id = cap.id.to_inner();
    cap
}

/// Initiates a signing flow using a previously published [`ECDSAPartialUserSignature`].
///
/// This function takes a partial signature object, validates approvals for each message,
/// and emits the necessary signing events.
///
/// ## Type Parameters
/// - `D`: Represents additional data fields specific for each implementation.
///
/// ## Parameters
/// - `partial_signature`: A previously published `ECDSAPartialUserSignature<D>` object
///   containing messages that require approval.
/// - `message_approvals`: A list of approvals corresponding to the messages in `partial_signature`.
/// - `ctx`: The transaction context.
/// ## Notes
/// - See [`ECDSAPartialUserSignature`] documentation for more details on usage scenarios.
/// - The function ensures that messages and approvals have a one-to-one correspondence before proceeding.
public(package) fun request_ecdsa_sign_with_partial_user_signatures(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    partial_user_signature_cap: VerifiedECDSAPartialUserSignatureCap,
    message_approval: MessageApproval,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    let (dwallet, _) = self.get_active_dwallet_and_public_output(dwallet_id);

    let pricing = self.pricing.ecdsa_sign_with_partial_user_signature();

    self.charge(pricing, dwallet.dwallet_network_decryption_key_id, payment_ika, payment_sui, ctx);

    // Ensure that each message has a corresponding approval; otherwise, abort.
    self.compare_ecdsa_partial_user_signatures_with_approvals(&partial_user_signature_cap, &message_approval);

    let VerifiedECDSAPartialUserSignatureCap {
        id,
        partial_centralized_signed_message_id,
    } = partial_user_signature_cap;
    let verified_cap_id = id.to_inner();
    id.delete();
    let ECDSAPartialUserSignature {
        id,
        created_at_epoch: _,
        dwallet_id: partial_centralized_signed_messages_dwallet_id,
        cap_id,
        hash_scheme: _,
        message: _,
        presign,
        message_centralized_signature,
        state
    } = self.ecdsa_partial_centralized_signed_messages.remove(partial_centralized_signed_message_id);
    id.delete();
    assert!(cap_id == verified_cap_id && state == ECDSAPartialUserSignatureState::NetworkVerificationCompleted, EIncorrectCap);
    assert!(partial_centralized_signed_messages_dwallet_id == dwallet_id, EDWalletMismatch);
    assert!(presign.dwallet_id == dwallet_id, EDWalletMismatch);

    // Emit signing events to finalize the signing process.
    self.emit_ecdsa_sign_event(
        message_approval,
        dwallet_id,
        presign,
        message_centralized_signature,
        true,
        ctx
    );
}

/// Compares partial user signatures with message approvals to ensure they match.
/// This function can be called by the user to verify that the messages and approvals match,
/// before calling the `sign_with_partial_centralized_message_signatures` function.
public(package) fun compare_ecdsa_partial_user_signatures_with_approvals(
    self: &DWalletCoordinatorInner,
    partial_user_signature_cap: &VerifiedECDSAPartialUserSignatureCap,
    message_approval: &MessageApproval,
) {
    let partial_signature = self.ecdsa_partial_centralized_signed_messages.borrow(partial_user_signature_cap.partial_centralized_signed_message_id);

    assert!(partial_signature.dwallet_id == message_approval.dwallet_id && message_approval.message == message_approval.message && partial_signature.hash_scheme == message_approval.hash_scheme, EMessageApprovalMismatch);
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
public(package) fun respond_ecdsa_sign(
    self: &mut DWalletCoordinatorInner,
    dwallet_id: ID,
    sign_id: ID,
    session_id: ID,
    signature: vector<u8>,
    is_future_sign: bool,
    rejected: bool,
) {

    let (dwallet, _) = self.get_active_dwallet_and_public_output_mut(dwallet_id);

    let sign = dwallet.ecdsa_signs.borrow_mut(sign_id);

    sign.state = match(sign.state) {
        ECDSASignState::Requested => {
            if(rejected) {
                event::emit(RejectedECDSASignEvent {
                    sign_id,
                    session_id,
                    is_future_sign,
                });
                ECDSASignState::NetworkRejected
            } else {
                event::emit(CompletedECDSASignEvent {
                    sign_id,
                    session_id,
                    signature,
                    is_future_sign,
                });
                ECDSASignState::Completed { signature }
            }
        },
        _ => abort ESignWrongState
    };
}

public(package) fun process_checkpoint_message_by_quorum(
    self: &mut DWalletCoordinatorInner,
    epoch: u64,
    signature: vector<u8>,
    signers_bitmap: vector<u8>,
    message: vector<u8>,
    ctx: &mut TxContext,
) {
    assert!(self.active_epochs.contains(epoch), EEpochNotExist);
    let epoch_coordinator = self.active_epochs.borrow(epoch);
    let mut intent_bytes = CHECKPOINT_MESSAGE_INTENT;
    intent_bytes.append(message);
    intent_bytes.append(bcs::to_bytes(&epoch));

    epoch_coordinator.committee.verify_certificate(epoch, &signature, &signers_bitmap, &intent_bytes);

    self.process_checkpoint_message(epoch, message, ctx);
}

fun process_checkpoint_message(
    self: &mut DWalletCoordinatorInner,
    epoch: u64,
    message: vector<u8>,
    ctx: &mut TxContext,
) {
    assert!(self.active_epochs.contains(epoch), EEpochNotExist);
    let epoch_coordinator = self.active_epochs.borrow_mut(epoch);
    assert!(!epoch_coordinator.committee.members().is_empty(), EActiveBlsCommitteeMustInitialize);

    let mut bcs_body = bcs::new(copy message);

    let checkpoint_epoch = bcs_body.peel_u64();
    assert!(epoch == checkpoint_epoch, EIncorrectEpochInCheckpoint);

    let sequence_number = bcs_body.peel_u32();

    if(epoch_coordinator.last_processed_checkpoint_sequence_number.is_none()) {
        assert!(sequence_number == 0, EWrongCheckpointSequenceNumber);
        epoch_coordinator.last_processed_checkpoint_sequence_number.fill(sequence_number);
    } else {
        assert!(sequence_number > 0 && *epoch_coordinator.last_processed_checkpoint_sequence_number.borrow() + 1 == sequence_number, EWrongCheckpointSequenceNumber);
        epoch_coordinator.last_processed_checkpoint_sequence_number.swap(sequence_number);
    };

    let timestamp_ms = bcs_body.peel_u64();

    event::emit(SystemCheckpointInfoEvent {
        epoch,
        sequence_number,
        timestamp_ms,
    });

    let messages_len = bcs_body.peel_vec_length();
    let mut i = 0;
    let mut response_session_count = 0;
    while (i < messages_len) {
        let message_data_type = bcs_body.peel_vec_length();
            // Parses checkpoint BCS bytes directly.
            // Messages with `message_data_type` 1 & 2 are handled by the system module,
            // but their bytes must be extracted here to allow correct parsing of types 3 and above.
            // This step only extracts the bytes without further processing.
            if (message_data_type == 1) {
                // EndOfEpochMessage
                let len = bcs_body.peel_vec_length();
                let mut i = 0;
                while (i < len) {
                    let end_of_epch_message_type = bcs_body.peel_vec_length();
                    // AdvanceEpoch
                    if(end_of_epch_message_type == 0) {
                        let _new_epoch = bcs_body.peel_u64();
                        let _next_protocol_version = bcs_body.peel_u64();
                        let _epoch_start_timestamp_ms = bcs_body.peel_u64();
                    };
                    i = i + 1;
                };
            } else if (message_data_type == 2) {
                    //TestMessage
                    let _authority = bcs_body.peel_u32();
                    let _num = bcs_body.peel_u64();
            } else if (message_data_type == 3) {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let first_round_output = bcs_body.peel_vec_u8();
                self.respond_dwallet_dkg_first_round(dwallet_id, first_round_output);
                response_session_count = response_session_count + 1;
            } else if (message_data_type == 4) {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_output = bcs_body.peel_vec_u8();
                let encrypted_centralized_secret_share_and_proof = bcs_body.peel_vec_u8();
                let encryption_key_address = sui::address::from_bytes(bcs_body.peel_vec_u8());
                let rejected = bcs_body.peel_bool();
                self.respond_dwallet_dkg_second_round(
                    dwallet_id,
                    public_output,
                    encrypted_centralized_secret_share_and_proof,
                    encryption_key_address,
                    session_id,
                    rejected,
                    ctx,
                );
                response_session_count = response_session_count + 1;
            } else if (message_data_type == 5) {
                let dwallet_id = object::id_from_address(bcs_body.peel_address());
                let encrypted_user_secret_key_share_id = object::id_from_address(bcs_body.peel_address());
                let rejected = bcs_body.peel_bool();
                self.respond_re_encrypt_user_share_for(
                    dwallet_id,
                    encrypted_user_secret_key_share_id,
                    rejected,
                );
                response_session_count = response_session_count + 1;
            } else if (message_data_type == 6) {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let sign_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let signature = bcs_body.peel_vec_u8();
                let is_future_sign = bcs_body.peel_bool();
                let rejected = bcs_body.peel_bool();
                self.respond_ecdsa_sign(
                    dwallet_id,
                    sign_id,
                    session_id,
                    signature,
                    is_future_sign,
                    rejected,
                );
                response_session_count = response_session_count + 1;
            } else if (message_data_type == 8) {
                let dwallet_id = object::id_from_address(bcs_body.peel_address());
                let partial_centralized_signed_message_id = object::id_from_address(bcs_body.peel_address());
                let rejected = bcs_body.peel_bool();
                self.respond_ecdsa_future_sign(
                    dwallet_id,
                    partial_centralized_signed_message_id,
                    rejected,
                );
                response_session_count = response_session_count + 1;
            } else if (message_data_type == 7) {
                let dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let presign = bcs_body.peel_vec_u8();
                self.respond_ecdsa_presign(dwallet_id, session_id, presign, ctx);
                response_session_count = response_session_count + 1;
            } else if (message_data_type == 9) {
                let dwallet_network_decryption_key_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                let public_output = bcs_body.peel_vec_u8();
                let key_shares = bcs_body.peel_vec_u8();
                let is_last = bcs_body.peel_bool();
                self.respond_dwallet_network_decryption_key_dkg(dwallet_network_decryption_key_id, public_output, key_shares, is_last);
                response_session_count = response_session_count + 1;
            };
        i = i + 1;
    };
    let epoch_coordinator = self.active_epochs.borrow_mut(epoch);
    epoch_coordinator.total_messages_processed = epoch_coordinator.total_messages_processed + messages_len;
    epoch_coordinator.session_count = epoch_coordinator.session_count + response_session_count;
}
