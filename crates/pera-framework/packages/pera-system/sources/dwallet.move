// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// This module defines the core data structures and functions for
/// working with dWallets in the Ika system.
///
/// ## Overview
///
/// - A **dWallet** (`DWallet`) represents a wallet created after the Distributed Key Generation (DKG) process.
///   It encapsulates the session ID, capability ID, and the outputs from the DKG rounds.
/// - A **dWallet capability** (`DWalletCap`) grants ownership and control over a corresponding `DWallet`.
///
/// ## Key Concepts
///
/// - **DWallet**: A generic wallet structure with a phantom type `T`.
/// - **DWalletCap**: A capability object that grants control over a specific dWallet.
/// - **Session ID**: A unique identifier for the DKG session.
module pera_system::dwallet {
    use pera::event;
    use pera::table::{Self, Table};
    use pera::ed25519::ed25519_verify;
    use pera_system::pera_system::PeraSystemState;
    use pera::hash;

    const CLASS_GROUPS: u8 = 0;
    const SYSTEM_ADDRESS: address = @0x0;

    /// Supported hash schemes for message signing.
    const KECCAK256: u8 = 0;
    const SHA256: u8 = 1;

    // >>>>>>>>>>>>>>> Error Codes >>>>>>>>>>>>>>

    const EInvalidEncryptionKeyScheme: u64 = 0;
    const EInvalidEncryptionKeySignature: u64 = 1;
    const EInvalidEncryptionKeyOwner: u64 = 2;
    const ENotSystemAddress: u64 = 3;
    const EMessageApprovalDWalletMismatch: u64 = 4;
    const EMissingApprovalOrWrongApprovalOrder: u64 = 5;
    const EInvalidHashScheme: u64 = 6;

    // <<<<<<<<<<<<<<< Error Codes <<<<<<<<<<<<<<

    // todo(zeev): rename network key everywhere.
    /// `DWallet` represents a wallet that is created after the DKG process.
    ///
    /// ### Fields
    /// - `id`: Unique identifier for the dWallet.
    /// - `session_id`: The session ID that generated this dWallet.
    /// - `dwallet_cap_id`: The ID of the capability associated with this dWallet.
    /// - `decentralized_output`: Decentralized public output of the second DKG round.
    /// - `centralized_output`: Centralized public output.
    /// - `dwallet_mpc_network_key_version`: Version of the MPC network key.
    public struct DWallet<phantom T> has key, store {
        id: UID,
        session_id: ID,
        dwallet_cap_id: ID,
        decentralized_output: vector<u8>,
        centralized_output: vector<u8>,
        dwallet_mpc_network_key_version: u8,
    }

    /// todo(zeev): check why we transfer both public key and address.
    /// An Encryption key that is used to encrypt a dWallet centralized (user) secret key share.
    /// Encryption keys facilitate secure data transfer between accounts on the
    /// dWallet Network by ensuring that sensitive information remains confidential during transmission.
    /// Each address on the dWallet Network is associated with a unique encryption key.
    /// When an external party intends to send encrypted data to a particular account, they use the recipient’s
    /// encryption key to encrypt the data.
    /// The recipient is then the sole entity capable of decrypting and accessing this information, ensuring secure,
    /// end-to-end encryption.
    /// ### Fields
    /// - `id`: Unique identifier for the encryption key.
    /// - `scheme`: Scheme identifier for the encryption key (e.g., Class Groups).
    /// - `encryption_key`: Serialized encryption key.
    /// - `key_owner_address`: Address of the encryption key owner.
    /// - `encryption_key_signature`: Signature for the encryption key, signed by the owner.
    /// - `key_owner_pubkey`: Public key of the encryption key owner.
    public struct EncryptionKey has key {
        id: UID,
        scheme: u8,
        encryption_key: vector<u8>,
        key_owner_address: address,
        encryption_key_signature: vector<u8>,
        key_owner_pubkey: vector<u8>,
    }

    /// Event emitted when an encryption key is created.
    public struct CreatedEncryptionKeyEvent has copy, drop {
        scheme: u8,
        encryption_key: vector<u8>,
        key_owner_address: address,
        encryption_key_signature: vector<u8>,
        key_owner_pubkey: vector<u8>,
        session_id: ID,
        encryption_key_id: ID,
    }

    /// An event emitted to start an encryption key verification process.
    /// Ika does not support native functions, so an event is emitted and
    /// caught by the blockchain, which then starts the verification process,
    /// similar to the MPC processes.
    public struct StartEncryptionKeyVerificationEvent has copy, drop {
        scheme: u8,
        encryption_key: vector<u8>,
        key_owner_address: address,
        encryption_key_signature: vector<u8>,
        sender_pubkey: vector<u8>,
        initiator: address,
        session_id: ID,
    }

    /// Represents a capability granting control over a specific dWallet.
    ///
    /// ### Fields
    /// - `id`: Unique identifier for the dWallet capability.
    public struct DWalletCap has key, store {
        id: UID,
    }

    /// Retrieves the encryption key from an `EncryptionKey` object.
    ///
    /// ### Parameters
    /// - `key`: A read reference to the `EncryptionKey` object.
    ///
    /// ### Returns
    /// A `vector<u8>` containing the encryption key.
    public(package) fun get_encryption_key(key: &EncryptionKey): vector<u8> {
        key.encryption_key
    }

    /// Creates a new [`DWallet`] object of type `T`.
    ///
    /// ### Parameters
    /// - `session_id`: Session ID that generated this dWallet.
    /// - `dwallet_cap_id`: Capability ID associated with this dWallet.
    /// - `decentralized_output`: Decentralized output of the second DKG round.
    /// - `dwallet_mpc_network_key_version`: Version of the MPC network key.
    /// - `dkg_centralized_public_output`: Centralized public output of the DKG round.
    /// - `ctx`: Mutable transaction context.
    ///
    /// ### Returns
    /// A new [`DWallet`] object of type `T`.
    public(package) fun create_dwallet<T: drop>(
        session_id: ID,
        dwallet_cap_id: ID,
        decentralized_output: vector<u8>,
        dwallet_mpc_network_key_version: u8,
        dkg_centralized_public_output: vector<u8>,
        ctx: &mut TxContext
    ): DWallet<T> {
        DWallet<T> {
            id: object::new(ctx),
            session_id,
            dwallet_cap_id,
            decentralized_output,
            dwallet_mpc_network_key_version,
            centralized_output: dkg_centralized_public_output,
        }
    }

    /// Shared object that holds the active encryption keys per user.
    ///
    /// ### Fields
    /// - `id`: Unique identifier for the object.
    /// - `encryption_keys`: Table mapping user addresses to encryption key object IDs.
    public struct ActiveEncryptionKeys has key {
        id: UID,
        encryption_keys: Table<address, ID>,
    }

    /// Create a shared object that holds the active encryption keys per user.
    public fun create_active_encryption_keys(ctx: &mut TxContext) {
        transfer::share_object(ActiveEncryptionKeys {
            id: object::new(ctx),
            encryption_keys: table::new(ctx),
        });
    }

    /// Get the active encryption key ID by user adders.
    public fun get_active_encryption_key(
        active_encryption_keys: &ActiveEncryptionKeys,
        key_owner: address,
    ): &ID {
        table::borrow(&active_encryption_keys.encryption_keys, key_owner)
    }

    /// Updates or inserts an encryption key as the active key for a user.
    public fun upsert_active_encryption_key(
        active_encryption_keys: &mut ActiveEncryptionKeys,
        encryption_key: &EncryptionKey,
        ctx: &mut TxContext
    ) {
        assert!(encryption_key.key_owner_address == tx_context::sender(ctx), EInvalidEncryptionKeyOwner);
        if (table::contains(&active_encryption_keys.encryption_keys, encryption_key.key_owner_address)) {
            table::remove(&mut active_encryption_keys.encryption_keys, encryption_key.key_owner_address);
        };
        table::add(
            &mut active_encryption_keys.encryption_keys,
            encryption_key.key_owner_address,
            object::id(encryption_key)
        );
    }

    /// Register an encryption key, to later use for encrypting a user share.
    /// The key is saved as an immutable object.
    /// The event emitted by this function is caught by the chain.
    /// The chain then calls "create_encryption_key" after verifications, in order to save it.
    /// We need to run the flow this way as this verification can only be done in Rust,
    /// and we can't use Native functions.
    /// ### Parameters
    /// - `encryption_key`: Serialized encryption key.
    /// - `signed_encryption_key`: Signed encryption key.
    /// - `sender_pubkey`: Public key of the sender.
    /// - `encryption_key_scheme`: Scheme of the encryption key.
    /// - `_pera_system_state`: The Pera system state object. Its ID is always 0x5.
    /// Needed so the TX will get ordered in consensus before getting executed.
    /// - `ctx`: Mutable transaction context.
    public fun register_encryption_key(
        encryption_key: vector<u8>,
        signed_encryption_key: vector<u8>,
        sender_pubkey: vector<u8>,
        encryption_key_scheme: u8,
        // TODO (#529): Create a dedicated, small shared object instead of using the system state.
        _pera_system_state: &PeraSystemState,
        ctx: &mut TxContext
    ) {
        assert!(is_valid_encryption_key_scheme(encryption_key_scheme), EInvalidEncryptionKeyScheme);
        assert!(
            ed25519_verify(&signed_encryption_key, &sender_pubkey, &encryption_key),
            EInvalidEncryptionKeySignature
        );
        event::emit(
            StartEncryptionKeyVerificationEvent {
                scheme: encryption_key_scheme,
                encryption_key,
                key_owner_address: tx_context::sender(ctx),
                encryption_key_signature: signed_encryption_key,
                // todo(zeev): rename.
                sender_pubkey,
                initiator: tx_context::sender(ctx),
                session_id: object::id_from_address(tx_context::fresh_object_address(ctx)),
            }
        );
    }

    /// Creates an encryption key object.
    /// Being called by the blockchain after it verifies
    /// the `sender_pubkey` matches the initiator address.
    /// // todo(zeev): validate this claim.
    /// We need to run the flow this way as this verification can only be done in rust.
    ///
    /// ### Parameters
    /// - `key`: Serialized encryption key.
    /// - `signature`: Encryption key signature.
    /// - `sender_pubkey`: Sender's Ika public key.
    /// - `scheme`: Encryption key scheme.
    /// - `initiator`: Initiator's address.
    /// - `session_id`: ID of the session.
    /// - `ctx`: Mutable transaction context.
    #[allow(unused_function)]
    fun create_encryption_key(
        key: vector<u8>,
        signature: vector<u8>,
        sender_pubkey: vector<u8>,
        scheme: u8,
        initiator: address,
        session_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
        let encryption_key = EncryptionKey {
            id: object::new(ctx),
            scheme,
            encryption_key: key,
            key_owner_address: initiator,
            encryption_key_signature: signature,
            key_owner_pubkey: sender_pubkey,
        };
        event::emit(CreatedEncryptionKeyEvent {
            scheme,
            encryption_key: key,
            key_owner_address: initiator,
            encryption_key_signature: signature,
            key_owner_pubkey: sender_pubkey,
            encryption_key_id: object::id(&encryption_key),
            session_id,
        });
        transfer::freeze_object(encryption_key);
    }

    #[test_only]
    public(package) fun create_encryption_key_for_testing(
        key: vector<u8>,
        signature: vector<u8>,
        sender_pubkey: vector<u8>,
        scheme: u8,
        initiator: address,
        ctx: &mut TxContext
    ): EncryptionKey {
        return EncryptionKey {
            id: object::new(ctx),
            scheme,
            encryption_key: key,
            key_owner_address: initiator,
            encryption_key_signature: signature,
            key_owner_pubkey: sender_pubkey,
        }
    }


    /// Create a new [`DWalletCap`] object.
    ///
    /// The holder of the `DWalletCap` has control and ownership over
    /// the associated `DWallet`.
    ///
    /// ### Parameters
    /// - `ctx`: A mutable transaction context used to create the `DWalletCap` object.
    ///
    /// ### Returns
    /// The newly created `DWalletCap` object.
    public(package) fun create_dwallet_cap(ctx: &mut TxContext): DWalletCap {
        DWalletCap {
            id: object::new(ctx),
        }
    }

    /// Retrieve the ID of the `DWalletCap` associated with a given dWallet.
    ///
    /// ### Parameters
    /// - `dwallet`: A reference to the [`DWallet`] object whose capability ID is to be retrieved.
    ///
    /// ### Returns
    /// The ID of the `DWalletCap` associated with the provided dWallet.
    public(package) fun get_dwallet_cap_id<T: drop>(dwallet: &DWallet<T>): ID {
        dwallet.dwallet_cap_id
    }

    /// Retrieve the output of the second DKG round for a given dWallet.
    ///
    /// ### Parameters
    /// - `dwallet`: A reference to the [`DWallet`] object whose DKG output is to be retrieved.
    ///
    /// ### Returns
    /// A `vector<u8>` representing the output of the second DKG round for the specified dWallet.
    public(package) fun get_dwallet_decentralized_output<T: drop>(dwallet: &DWallet<T>): vector<u8> {
        dwallet.decentralized_output
    }

    /// Retrieve the centralized public DKG output for a given dWallet.
    public(package) fun get_dwallet_centralized_output<T: drop>(dwallet: &DWallet<T>): vector<u8> {
        dwallet.centralized_output
    }

    public(package) fun get_dwallet_mpc_network_key_version<T: drop>(dwallet: &DWallet<T>): u8 {
        dwallet.dwallet_mpc_network_key_version
    }

    /// Validates encryption key schemes.
    fun is_valid_encryption_key_scheme(scheme: u8): bool {
        scheme == CLASS_GROUPS
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
        dwallet_cap_id: ID,
        hash_scheme: u8,
        message: vector<u8>,
    }

    /// Creates a `MessageApproval` object.
    public(package) fun create_message_approval(
        dwallet_cap_id: ID,
        hash_scheme: u8,
        message: vector<u8>,
    ): MessageApproval {
        assert!(is_supported_hash_scheme(hash_scheme), EInvalidHashScheme);
        let approval = MessageApproval {
            dwallet_cap_id,
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
    public fun approve_messages(
        dwallet_cap: &DWalletCap,
        hash_scheme: u8,
        messages: &mut vector<vector<u8>>
    ): vector<MessageApproval> {
        let dwallet_cap_id = object::id(dwallet_cap);
        let mut message_approvals = vector::empty<MessageApproval>();

        // Approve all messages and maintain their order.
        let messages_length = vector::length(messages);
        let mut i: u64 = 0;
        while (i < messages_length) {
            let message = vector::pop_back(messages);
            vector::push_back(&mut message_approvals, create_message_approval (
                dwallet_cap_id,
                hash_scheme,
                message,
            ));
            i = i + 1;
        };
        vector::reverse(&mut message_approvals);
        message_approvals
    }

    /// Remove a `MessageApproval` and return the `dwallet_cap_id`,
    /// `hash_scheme` and the `message`.
    fun remove_message_approval(message_approval: MessageApproval): (ID, u8, vector<u8>) {
        let MessageApproval {
            dwallet_cap_id,
            hash_scheme,
            message
        } = message_approval;
        (dwallet_cap_id, hash_scheme, message)
    }

    /// Pops the last message approval from the vector and verifies it against the given message & dwallet_cap_id.
    public(package) fun pop_and_verify_message_approval(
        dwallet_cap_id: ID,
        message_hash: vector<u8>,
        message_approvals: &mut vector<MessageApproval>
    ) {
        let message_approval = vector::pop_back(message_approvals);
        let (message_approval_dwallet_cap_id, _hash_scheme, approved_message) = remove_message_approval(message_approval);
        assert!(dwallet_cap_id == message_approval_dwallet_cap_id, EMessageApprovalDWalletMismatch);
        assert!(&message_hash == &approved_message, EMissingApprovalOrWrongApprovalOrder);
    }

    public(package) fun hash_messages(message_approvals: &vector<MessageApproval>): vector<vector<u8>>{
        let mut hashed_messages = vector::empty();
        let messages_length = vector::length(message_approvals);
        let mut i: u64 = 0;
        while (i < messages_length) {
            let message = &message_approvals[i].message;
            let hash_scheme = message_approvals[i].hash_scheme;
            let hashed_message = hash_message(*message, hash_scheme);
            vector::push_back(&mut hashed_messages, hashed_message);
            i = i + 1;
        };
        hashed_messages
    }

    /// Hashes the given message using the specified hash scheme.
    public(package) fun hash_message(message: vector<u8>, hash_scheme: u8): vector<u8> {
        assert!(is_supported_hash_scheme(hash_scheme), EInvalidHashScheme);
        return match (hash_scheme) {
                KECCAK256 => hash::keccak256(&message),
                SHA256 => std::hash::sha2_256(message),
                _ => vector::empty(),
        }
    }

    #[allow(unused_function)]
    /// Checks if the given hash scheme is supported for message signing.
    fun is_supported_hash_scheme(val: u8): bool {
        return match (val) {
                KECCAK256 | SHA256 => true,
        _ => false,
    }
}
}
