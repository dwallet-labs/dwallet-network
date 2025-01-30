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

    const KEY_SCHEME_CLASS_GROUPS: u8 = 0;
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
    const EExtraDataAndMessagesLenMismatch: u64 = 7;

    // <<<<<<<<<<<<<<< Error Codes <<<<<<<<<<<<<<

    /// `DWallet` represents a decentralized wallet (dWallet) that is
    /// created after the Distributed key generation (DKG) process.
    public struct DWallet<phantom T> has key, store {
        /// Unique identifier for the dWallet.
        id: UID,

        /// The session ID that generated this dWallet.
        session_id: ID,

        /// The ID of the capability associated with this dWallet.
        dwallet_cap_id: ID,

        /// The decentralized party public output in the DKG process.
        decentralized_public_output: vector<u8>,

        /// The centralized party public output in the DKG process.
        centralized_public_output: vector<u8>,

        /// The MPC network decryption key version that is used to decrypt this dWallet.
        dwallet_mpc_network_decryption_key_version: u8,
    }

    /// Creates a new [`DWallet`] object of type `T`.
    ///
    /// This function initializes a decentralized wallet (`DWallet`) after the second DKG round,
    /// linking it to the appropriate capability ID and storing the outputs from the DKG process.
    ///
    /// ### Parameters
    /// - `session_id`: A unique identifier for the session that generated this dWallet.
    /// - `dwallet_cap_id`: The unique identifier for the capability associated with this dWallet.
    /// - `decentralized_public_output`: The decentralized public output produced during the second DKG round.
    /// - `dwallet_mpc_network_decryption_key_version`: The version of the MPC network decryption key
    ///    used for this dWallet.
    /// - `centralized_public_output`: The centralized public output produced during the DKG process.
    /// - `ctx`: A mutable transaction context used to create the dWallet object.
    ///
    /// ### Returns
    /// A new [`DWallet`] object of type `T`.
    public(package) fun create_dwallet<T: drop>(
        session_id: ID,
        dwallet_cap_id: ID,
        decentralized_public_output: vector<u8>,
        dwallet_mpc_network_decryption_key_version: u8,
        centralized_public_output: vector<u8>,
        ctx: &mut TxContext
    ): DWallet<T> {
        DWallet<T> {
            id: object::new(ctx),
            session_id,
            dwallet_cap_id,
            decentralized_public_output,
            dwallet_mpc_network_decryption_key_version,
            centralized_public_output,
        }
    }

    /// Retrieve the ID of the `DWalletCap` associated with a given dWallet.
    public(package) fun get_dwallet_cap_id<T: drop>(dwallet: &DWallet<T>): ID {
        dwallet.dwallet_cap_id
    }

    /// Retrieves the decentralized public output of the second DKG round for a given dWallet..
    public(package) fun get_dwallet_decentralized_public_output<T: drop>(dwallet: &DWallet<T>): vector<u8> {
        dwallet.decentralized_public_output
    }

    /// Retrieves the centralized public output for a given dWallet.
    public(package) fun get_dwallet_centralized_public_output<T: drop>(dwallet: &DWallet<T>): vector<u8> {
        dwallet.centralized_public_output
    }

    /// Retrieves the MPC network decryption key version for a given dWallet.
    public(package) fun get_dwallet_mpc_network_decryption_key_version<T: drop>(dwallet: &DWallet<T>): u8 {
        dwallet.dwallet_mpc_network_decryption_key_version
    }

    /// Represents a capability granting control over a specific dWallet.
    public struct DWalletCap has key, store {
        /// Unique identifier for the dWallet capability.
        id: UID,
    }

    /// Create a new [`DWalletCap`] object.
    ///
    /// The holder of the `DWalletCap` has control and ownership over
    /// the associated `DWallet`.
    ///
    /// ### Returns
    /// The newly created `DWalletCap` object.
    public(package) fun create_dwallet_cap(ctx: &mut TxContext): DWalletCap {
        DWalletCap {
            id: object::new(ctx),
        }
    }

    /// Represents an encryption key used to encrypt a dWallet centralized (user) secret key share.
    ///
    /// Encryption keys facilitate secure data transfer between accounts on the
    /// dWallet Network by ensuring that sensitive information remains confidential during transmission.
    /// Each address on the dWallet Network is associated with a unique encryption key.
    /// When an external party intends to send encrypted data to a particular account, they use the recipient’s
    /// encryption key to encrypt the data. The recipient is then the sole entity capable of decrypting
    /// and accessing this information, ensuring secure, end-to-end encryption.
    public struct EncryptionKey has key {
        /// Unique identifier for the `EncryptionKey`.
        id: UID,

        /// Scheme identifier for the encryption key (e.g., Class Groups).
        encryption_key_scheme: u8,

        /// Serialized encryption key.
        encryption_key: vector<u8>,

        /// Address of the encryption key owner.
        key_owner_address: address,

        /// Signature for the encryption key, signed by the `key_signer_public_key`.
        encryption_key_signature: vector<u8>,

        /// The public key that was used to sign the `encryption_key`.
        key_signer_public_key: vector<u8>,
    }

    /// Retrieves the encryption key from an `EncryptionKey` object.
    public(package) fun get_encryption_key(key: &EncryptionKey): vector<u8> {
        key.encryption_key
    }

    /// Event emitted when an encryption key is created.
    ///
    /// This event is emitted after the blockchain verifies the encryption key's validity
    /// and creates the corresponding `EncryptionKey` object.
    public struct CreatedEncryptionKeyEvent has copy, drop {
        /// A unique identifier for the session related to the encryption key creation.
        session_id: ID,

        /// The unique identifier of the created `EncryptionKey` object.
        encryption_key_id: ID,
    }


    /// Event emitted to start an encryption key verification process.
    ///
    /// Since Ika does not support native functions, this event is emitted and
    /// caught by the blockchain to initiate the verification process.
    /// This process ensures the encryption key's validity and compliance with the system's requirements.
    public struct StartEncryptionKeyVerificationEvent has copy, drop {
        /// Scheme identifier for the encryption key (e.g., Class Groups).
        encryption_key_scheme: u8,

        /// Serialized encryption key to be verified.
        encryption_key: vector<u8>,

        /// Signature for the encryption key.
        encryption_key_signature: vector<u8>,

        /// The public key of the signer, used to verify
        /// the signature on the encryption key.
        key_signer_public_key: vector<u8>,

        /// The address of the user initiating the verification process.
        initiator: address,

        /// A unique identifier for the session related to this verification.
        session_id: ID,
    }

    /// Shared object that holds the active encryption keys per user.
    ///
    /// This object maintains a mapping between user addresses and their active encryption keys,
    /// enabling efficient retrieval and management of encryption keys within the Ika blockchain.
    public struct ActiveEncryptionKeys has key {
        /// Unique identifier for the `ActiveEncryptionKeys` object.
        id: UID,

        /// A table mapping user addresses to encryption key object IDs.
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

    /// Registers an encryption key to be used later for encrypting a
    /// centralized secret key share.
    ///
    /// The encryption key is saved as an immutable object after verification.
    /// This function emits an event (`StartEncryptionKeyVerificationEvent`) that is caught
    /// by the blockchain.
    /// The blockchain then performs necessary verifications and invokes `create_encryption_key()`
    /// to finalize and store the key. This flow is required because verification can only
    /// be implemented in Rust, as native functions are not supported in Ika.
    ///
    /// ### Parameters
    /// - `encryption_key`: The serialized encryption key to be registered.
    /// - `encryption_key_signature`: The signature of the encryption key, signed by the signer.
    /// - `key_signer_public_key`: The public key of the signer used to verify the encryption key signature.
    /// - `encryption_key_scheme`: The scheme of the encryption key (e.g., Class Groups).
    /// - `_pera_system_state`: The Pera system state object. Its ID is always 0x5.
    /// Needed so the TX will get ordered in consensus before getting executed.
    public fun register_encryption_key(
        encryption_key: vector<u8>,
        encryption_key_signature: vector<u8>,
        key_signer_public_key: vector<u8>,
        encryption_key_scheme: u8,
        // TODO (#529): Create a dedicated, small shared object instead of using the system state.
        _pera_system_state: &PeraSystemState,
        ctx: &mut TxContext
    ) {
        assert!(is_valid_encryption_key_scheme(encryption_key_scheme), EInvalidEncryptionKeyScheme);
        assert!(
            ed25519_verify(&encryption_key_signature, &key_signer_public_key, &encryption_key),
            EInvalidEncryptionKeySignature
        );
        event::emit(
            StartEncryptionKeyVerificationEvent {
                encryption_key_scheme,
                encryption_key,
                encryption_key_signature,
                key_signer_public_key,
                initiator: tx_context::sender(ctx),
                session_id: object::id_from_address(tx_context::fresh_object_address(ctx)),
            }
        );
    }

    /// Creates an encryption key object.
    ///
    /// This function is called by the blockchain after it verifies that the
    /// `key_signer_public_key` matches the `initiator` address. This flow ensures
    /// that verification is handled securely in Rust since native functions are
    /// not supported in Ika.
    ///
    /// The created encryption key object is immutable.
    /// An event (`CreatedEncryptionKeyEvent`) is emitted to signal the successful
    /// creation of the encryption key.
    ///
    /// ### Parameters
    /// - `encryption_key`: The serialized encryption key to be created.
    /// - `encryption_key_signature`: The signature of the encryption key, signed by the signer.
    /// - `signer_public_key`: The public key of the signer used to verify the encryption key.
    /// - `encryption_key_scheme`: The scheme of the encryption key (e.g., Class Groups).
    /// - `initiator`: The address of the user initiating the encryption key creation.
    /// - `session_id`: A unique identifier for the session associated with this encryption key.
    /// - `ctx`: A mutable transaction context used to create and freeze the encryption key object.
    #[allow(unused_function)]
    fun create_encryption_key(
        encryption_key: vector<u8>,
        encryption_key_signature: vector<u8>,
        key_signer_public_key: vector<u8>,
        encryption_key_scheme: u8,
        initiator: address,
        session_id: ID,
        ctx: &mut TxContext
    ) {
        // Ensure the caller is the system address
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);

        // Create the encryption key object
        let encryption_key_obj = EncryptionKey {
            id: object::new(ctx),
            encryption_key_scheme,
            encryption_key,
            key_owner_address: initiator,
            encryption_key_signature,
            key_signer_public_key,
        };

        // Emit an event to signal the creation of the encryption key
        event::emit(CreatedEncryptionKeyEvent {
            encryption_key_id: object::id(&encryption_key_obj),
            session_id,
        });

        // Freeze the encryption key object to make it immutable
        transfer::freeze_object(encryption_key_obj);
    }

    /// Validates encryption key schemes.
    fun is_valid_encryption_key_scheme(scheme: u8): bool {
        scheme == KEY_SCHEME_CLASS_GROUPS
    }

    #[test_only]
    public(package) fun create_encryption_key_for_testing(
        key: vector<u8>,
        signature: vector<u8>,
        sender_pubkey: vector<u8>,
        encryption_key_scheme: u8,
        initiator: address,
        ctx: &mut TxContext
    ): EncryptionKey {
        return EncryptionKey {
            id: object::new(ctx),
            encryption_key_scheme,
            encryption_key: key,
            key_owner_address: initiator,
            encryption_key_signature: signature,
            key_signer_public_key: sender_pubkey,
        }
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
            vector::push_back(&mut message_approvals, create_message_approval(
                dwallet_cap_id,
                hash_scheme,
                message,
            ));
            i = i + 1;
        };
        vector::reverse(&mut message_approvals);
        message_approvals
    }

    // Todo (#565): Move the hash calculation into the rust code.
    public(package) fun hash_message_approvals(message_approvals: &vector<MessageApproval>): vector<vector<u8>> {
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

    // Todo (#565): Move the hash calculation into the rust code.
    public(package) fun hash_messages(messages: &vector<vector<u8>>, hash_scheme: u8): vector<vector<u8>> {
        let mut hashed_messages = vector::empty();
        let messages_length = vector::length(messages);
        let mut i: u64 = 0;
        while (i < messages_length) {
            let message = &messages[i];
            let hashed_message = hash_message(*message, hash_scheme);
            vector::push_back(&mut hashed_messages, hashed_message);
            i = i + 1;
        };
        hashed_messages
    }

    /// Hashes the given message using the specified hash scheme.
    // Todo (#565): Move the hash calculation into the rust code.
    public(package) fun hash_message(message: vector<u8>, hash_scheme: u8): vector<u8> {
        assert!(is_supported_hash_scheme(hash_scheme), EInvalidHashScheme);
        return match (hash_scheme) {
                KECCAK256 => hash::keccak256(&message),
                SHA256 => std::hash::sha2_256(message),
                _ => vector::empty(),
        }
    }

    /// Checks if the given hash scheme is supported for message signing.
    fun is_supported_hash_scheme(val: u8): bool {
        return match (val) {
                KECCAK256 | SHA256 => true,
        _ => false,
        }
    }

    /// Messages that have been signed by a user, a.k.a the centralized party,
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
    ///
    /// D: The type of data that can be stored with the object,
    /// specific to each Digital Signature Algorithm.
    public struct PartialCentralizedSignedMessages<D: store> has key {
        /// A unique identifier for this object.
        id: UID,

        /// The messages that are being signed.
        messages: vector<vector<u8>>,

        /// The unique identifier of the associated dWallet.
        dwallet_id: ID,

        /// The DKG decentralized party public output for the dWallet.
        dwallet_decentralized_public_output: vector<u8>,

        /// The unique identifier of the dWallet capability.
        dwallet_cap_id: ID,

        /// The MPC network decryption key version that is used to decrypt the associated dWallet.
        dwallet_mpc_network_decryption_key_version: u8,

        /// Data specific to every DSA implementation.
        /// Every message (from `messages`) corresponds with a `D`, so the order of
        /// the messages and the data fields must be maintained.
        signature_algorithm_data: vector<D>,
    }

    /// Event emitted to start a batched sign process.
    public struct StartBatchedSignEvent has copy, drop {
        /// The session identifier for the batched sign process.
        session_id: ID,

        /// A list of hashed messages to be signed.
        /// Todo (#565): Move the hash calculation into the Rust code.
        hashed_messages: vector<vector<u8>>,

        /// The address of the user who initiated the protocol.
        initiator: address
    }

    /// Event emitted to signal the completion of a Sign process.
    ///
    /// This event contains signatures for all signed messages in the batch.
    public struct CompletedSignEvent has copy, drop {
        /// The session identifier for the signing process.
        session_id: ID,

        /// The ID of the batched sign output object (`BatchedSignOutput`).
        output_object_id: ID,

         /// List of signatures in the same order as the sign function message approvals input.
        signatures: vector<vector<u8>>,

        /// Indicates whether the future sign feature was used to start the session.
        is_future_sign: bool,
    }

    /// The output of a batched Sign session.
    public struct BatchedSignOutput has key, store {
        /// A unique identifier for the batched sign output.
        id: UID,

        /// The session identifier for the batched sign process.
        session_id: ID,

        /// A collection of signatures (of the sign process messages)
        /// generated in the batched sign session.
        signatures: vector<vector<u8>>,

        /// The unique identifier of the associated dWallet.
        dwallet_id: ID,
    }

    /// Event emitted to initiate the signing process.
    ///
    /// This event is captured by Validators to start the signing protocol.
    /// It includes all the necessary information to link the signing process
    /// to a specific dWallet, and batched process.
    /// D: The type of data that can be stored with the object,
    /// specific to each Digital Signature Algorithm.
    public struct StartSignEvent<D: copy + drop> has copy, drop {
        /// A unique identifier for this signing session.
        session_id: ID,

        /// The address of the user who initiated the signing event.
        initiator: address,

        /// A unique identifier for the batched signing process this session belongs to.
        batched_session_id: ID,

        /// The unique identifier for the dWallet used in the session.
        dwallet_id: ID,

        /// The output from the dWallet DKG process used in this session.
        dwallet_decentralized_public_output: vector<u8>,

        /// The hashed message to be signed in this session.
        hashed_message: vector<u8>,

        /// The MPC network decryption key version that is used to decrypt the associated dWallet.
        dwallet_mpc_network_decryption_key_version: u8,

        /// Data that can be stored with the object,
        /// specific to each Digital Signature Algorithm.
        signature_algorithm_data: D,

        /// Indicates whether the future sign feature was used to start the session.
        is_future_sign: bool,
    }

    // This object MUST be Hot Potato.
    /// Stores data essential for the signing process and specific to every signature algorithm implementation.
    /// Must be passed to the signing functions and used in the programmable transaction block;
    /// otherwise, the transaction will fail due to the "Hot Potato" pattern.
    /// D: The type of data that can be stored with the object,
    /// specific to each Digital Signature Algorithm.
    public struct SignatureAlgorithmData<D: drop + copy + store> {
        data: D,
    }

    /// Creates a new [`PartialCentralizedSignedMessages`] object.
    /// This object is used to store messages that have been signed by a user,
    /// but not yet by the blockchain.
    /// T: The eliptic curve type used for the dWallet (for example `Secp256k1`).
    /// D: The type of data that can be stored with the object,
    /// specific to each Digital Signature Algorithm.
    public(package) fun create_partial_centralized_signed_messages<D: store + drop + copy>(
        messages: vector<vector<u8>>,
        dwallet_id: ID,
        dwallet_decentralized_public_output: vector<u8>,
        dwallet_cap_id: ID,
        dwallet_mpc_network_decryption_key_version: u8,
        signature_algorithm_data: vector<D>,
        session_id: ID,
        initiator: address,
        ctx: &mut TxContext
    ) {
        let partial_signatures = PartialCentralizedSignedMessages<D> {
            id: object::new(ctx),
            messages,
            dwallet_id,
            dwallet_decentralized_public_output,
            dwallet_cap_id,
            dwallet_mpc_network_decryption_key_version,
            signature_algorithm_data,
        };

        event::emit(CreatedPartialCentralizedSignedMessagesEvent {
            partial_signatures_object_id: object::id(&partial_signatures),
            session_id,
        });

        transfer::transfer(partial_signatures, initiator);
    }

     // todo(zeev): remove this.
    #[test_only]
    public fun partial_signatures_for_testing<D: store + drop + copy>(
        messages: vector<vector<u8>>,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        dwallet_mpc_network_decryption_key_version: u8,
        signature_algorithm_data: vector<D>,
        ctx: &mut TxContext
    ): PartialCentralizedSignedMessages<D> {
        PartialCentralizedSignedMessages<D> {
            id: object::new(ctx),
            messages,
            dwallet_id,
            dwallet_decentralized_public_output: vector::empty(),
            dwallet_cap_id,
            dwallet_mpc_network_decryption_key_version,
            signature_algorithm_data,
        }
    }

    /// Initiates the signing process for a given dWallet of type T.
    ///
    /// This function emits a `StartSignEvent` and a `StartBatchedSignEvent`,
    /// providing all necessary metadata to ensure the integrity of the signing process.
    /// It validates the linkage between the `DWallet`, `DWalletCap`, and `SignatureAlgorithmData` objects.
    ///
    /// # Effects
    /// - Ensures a valid linkage between `DWallet`, `DWalletCap`, and `SignatureAlgorithmData`.
    /// - Validates that `signature_algorithm_data` and `message_approvals` have the same length.
    /// - Emits the following events:
    ///   - `StartBatchedSignEvent`: Contains the session details and the list of hashed messages.
    ///   - `StartSignEvent`: Includes details for each message signing process.
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
    public fun sign<T: drop, D: copy + drop + store>(
        dwallet: &DWallet<T>,
        message_approvals: vector<MessageApproval>,
        signature_algorithm_data: vector<SignatureAlgorithmData<D>>,
        _pera_system_state: &PeraSystemState,
        ctx: &mut TxContext
    ) {
        let signature_algorithm_data_unpacked = vector::map!(signature_algorithm_data, | SignatureAlgorithmData { data } | data);
        emit_sign_events<D>(
            message_approvals,
            object::id(dwallet),
            dwallet.dwallet_cap_id,
            dwallet.decentralized_public_output,
            dwallet.dwallet_mpc_network_decryption_key_version,
            signature_algorithm_data_unpacked,
            false,
            ctx
        );
    }

    /// Emits events to initiate the signing process for each message.
    ///
    /// This function ensures that all messages have the correct approvals, calculates
    /// their hashes, and emits signing events.
    ///
    /// # Effects
    /// - Checks that the number of `signature_algorithm_data` items matches `message_approvals`.
    /// - Generates a new session ID for batch signing.
    /// - Emits `StartBatchedSignEvent` containing session details and hashed messages.
    /// - Iterates through `signature_algorithm_data`, verifying approvals and emitting `StartSignEvent` for each.
    ///
    /// # Aborts
    /// - **`EExtraDataAndMessagesLenMismatch`**: If `signature_algorithm_data` and `message_approvals` have different lengths.
    /// - **`EMissingApprovalOrWrongApprovalOrder`**: If message approvals are incorrect or missing.
    fun emit_sign_events<D: copy + drop + store>(
        message_approvals: vector<MessageApproval>,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        dwallet_decentralized_public_output: vector<u8>,
        dwallet_mpc_network_decryption_key_version: u8,
        signature_algorithm_data: vector<D>,
        is_future_sign: bool,
        ctx: &mut TxContext
    ){
        assert!(vector::length(&signature_algorithm_data) == vector::length(&message_approvals), EExtraDataAndMessagesLenMismatch);
        // Todo (#565): Move the hash calculation into the rust code.
        let hashed_messages = hash_message_approvals(&message_approvals);

        let batch_session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        event::emit(StartBatchedSignEvent {
            session_id: batch_session_id,
            hashed_messages,
            initiator: tx_context::sender(ctx)
        });

        vector::zip_map!(signature_algorithm_data, message_approvals, |data, message_approval| {
            let MessageApproval {
                dwallet_cap_id:  message_approval_dwallet_cap_id,
                hash_scheme,
                message
            } = message_approval;
            assert!(dwallet_cap_id == message_approval_dwallet_cap_id, EMessageApprovalDWalletMismatch);
            // Todo (#565): Move the hash calculation into the rust code.
            let hashed_message = hash_message(message, hash_scheme);

            let id = object::id_from_address(tx_context::fresh_object_address(ctx));
            event::emit(StartSignEvent<D> {
                session_id: id,
                initiator: tx_context::sender(ctx),
                batched_session_id: batch_session_id,
                dwallet_id,
                dwallet_decentralized_public_output,
                hashed_message,
                dwallet_mpc_network_decryption_key_version,
                signature_algorithm_data: data,
                is_future_sign,
            });
            true
        });
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
    #[allow(unused_function)]
    fun create_sign_output(
        signatures: vector<vector<u8>>,
        batch_session_id: ID,
        initiator: address,
        dwallet_id: ID,
        is_future_sign: bool,
        ctx: &mut TxContext
    ) {
        // Ensure that only the system address can call this function.
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);

        let sign_output = BatchedSignOutput {
            id: object::new(ctx),
            signatures: signatures,
            dwallet_id,
            session_id: batch_session_id,
        };

        // Emit the CompletedSignEvent with session ID and signed messages.
        // This is only used to notify the user that the signing process is complete.
        event::emit(CompletedSignEvent {
            session_id: batch_session_id,
            output_object_id: object::id(&sign_output),
            signatures,
            is_future_sign,
        });

        transfer::transfer(sign_output, initiator);
    }

    /// Event emitted when a [`PartialCentralizedSignedMessages`] object is created.
    public struct CreatedPartialCentralizedSignedMessagesEvent has copy, drop {
        /// The unique identifier of the created `PartialCentralizedSignedMessages` object.
        partial_signatures_object_id: ID,
        session_id: ID,
    }

    /// An event emitted to start the MPC verification, ensuring that the received partial signature is a valid partial
    /// signature on the given message by the given dWallet.
    public struct StartPartialSignaturesVerificationEvent<D: copy + store + drop> has copy, drop {
        session_id: ID,
        messages: vector<vector<u8>>,
        hashed_messages: vector<vector<u8>>,
        dwallet_id: ID,
        dwallet_decentralized_public_output: vector<u8>,
        dwallet_cap_id: ID,
        dwallet_mpc_network_decryption_key_version: u8,
        signature_data: vector<D>,
        initiator: address,
    }

    /// A function to publish messages signed by the user on chain with on-chain verification,
    /// without launching the chain's sign flow immediately.
    ///
    /// See the docs of [`PartialCentralizedSignedMessages`] for
    /// more details on when this may be used.
    public fun request_future_sign<T: drop, D: copy + drop + store>(
        dwallet: &DWallet<T>,
        messages: vector<vector<u8>>,
        signature_algorithm_data: vector<SignatureAlgorithmData<D>>,
        hash_scheme: u8,
        _pera_system_state: &PeraSystemState,
        ctx: &mut TxContext
    ) {
        let messages_len = vector::length(&messages);
        let signature_algorithm_data_len = vector::length(&signature_algorithm_data);
        assert!(messages_len == signature_algorithm_data_len, EExtraDataAndMessagesLenMismatch);

        let signature_algorithm_data_unpacked = vector::map!(signature_algorithm_data, |SignatureAlgorithmData { data }| data);
        let hashed_messages = hash_messages(&messages, hash_scheme);
        event::emit(StartPartialSignaturesVerificationEvent {
            session_id: object::id_from_address(tx_context::fresh_object_address(ctx)),
            dwallet_id: object::id(dwallet),
            dwallet_decentralized_public_output: dwallet.decentralized_public_output,
            dwallet_cap_id: dwallet.dwallet_cap_id,
            dwallet_mpc_network_decryption_key_version: dwallet.dwallet_mpc_network_decryption_key_version,
            messages,
            hashed_messages,
            signature_data: signature_algorithm_data_unpacked,
            initiator: tx_context::sender(ctx),
        });
    }



    /// A function to create a [`SignatureAlgorithmData`] object.
    /// Extra fields are used to store additional data with the object, specific to every protocol implementation.
    /// D: The type of the extra fields that can be stored with the object.
    public(package) fun create_signature_algorithm_data<D: store + copy + drop>(data: D): SignatureAlgorithmData<D> {
        SignatureAlgorithmData { data }
    }

    /// Initiates a signing flow using a previously published [`PartialCentralizedSignedMessages`].
    ///
    /// This function takes a partial signature object, validates approvals for each message,
    /// and emits the necessary signing events.
    ///
    /// ## Type Parameters
    /// - `D`: Represents additional data fields specific for each implementation.
    ///
    /// ## Parameters
    /// - `partial_signature`: A previously published `PartialCentralizedSignedMessages<D>` object
    ///   containing messages that require approval.
    /// - `message_approvals`: A list of approvals corresponding to the messages in `partial_signature`.
    /// - `ctx`: The transaction context.
    /// ## Notes
    /// - See [`PartialCentralizedSignedMessages`] documentation for more details on usage scenarios.
    /// - The function ensures that messages and approvals have a one-to-one correspondence before proceeding.
    public fun sign_with_partial_centralized_message_signatures<D: store + copy + drop>(
        partial_signature: PartialCentralizedSignedMessages<D>,
        message_approvals: vector<MessageApproval>,
        _pera_system_state: &PeraSystemState,
        ctx: &mut TxContext
    ) {
        let PartialCentralizedSignedMessages<D> {
            id,
            messages,
            dwallet_id,
            dwallet_decentralized_public_output,
            dwallet_cap_id,
            dwallet_mpc_network_decryption_key_version,
            signature_algorithm_data,
        } = partial_signature;
        object::delete(id);

        // Ensure that each message has a corresponding approval; otherwise, abort.
        compare_messages_with_approvals(&messages, &message_approvals);

        // Emit signing events to finalize the signing process.
        emit_sign_events<D>(
            message_approvals,
            dwallet_id,
            dwallet_cap_id,
            dwallet_decentralized_public_output,
            dwallet_mpc_network_decryption_key_version,
            signature_algorithm_data,
            true,
            ctx
        );
    }

    /// Compares the messages with the approvals to ensure they match.
    /// This function can be called by the user to verify that the messages and approvals match,
    /// before calling the `sign_with_partial_centralized_message_signatures` function.
    public fun compare_messages_with_approvals(
        messages: &vector<vector<u8>>,
        approvals: &vector<MessageApproval>
    ) {
        let compare_vector = vector::zip_map_ref!(messages, approvals, |message, approval| {
            if (message == approval.message) {true} else {false}
        });
        assert!(vector::all!(&compare_vector, |v| { v == true }), EMissingApprovalOrWrongApprovalOrder);
    }

    #[test_only]
    /// Call the underlying `create_sign_output`.
    /// This function is intended for testing purposes only and should not be used in production.
    /// See Move pattern: https://move-book.com/move-basics/testing.html#utilities-with-test_only
    public fun create_sign_output_for_testing(
        signed_messages: vector<vector<u8>>,
        batch_session_id: ID,
        initiator: address,
        dwallet_id: ID,
        ctx: &mut TxContext
    ) {
        // Call the main create_sign_output function with the provided parameters
        create_sign_output(
            signed_messages,
            batch_session_id,
            initiator,
            dwallet_id,
            false,
            ctx
        );
    }
}