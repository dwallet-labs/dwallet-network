// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// Generic dWallet representation.
/// This is conceptually the `Dwallet` interface.
module dwallet_system::dwallet {
    use std::vector;

    use dwallet::event;
    use dwallet::object::{Self, ID, UID};
    use dwallet::table;
    use dwallet::table::Table;
    use dwallet::transfer;
    use dwallet::tx_context;
    use dwallet::tx_context::TxContext;

    friend dwallet_system::dwallet_2pc_mpc_ecdsa_k1;

    #[test_only]
    friend dwallet_system::dwallet_tests;

    #[test_only]
    friend dwallet_system::dwallet_ecdsa_k1_tests;

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    const EMesssageApprovalDWalletMismatch: u64 = 1;
    const EInvalidEncryptionKeyScheme: u64 = 2;
    const EInvalidEncryptionKeyOwner: u64 = 3;

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<


    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<

    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<


    // <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<
    /// Event to start a `Sign` session, caught by the Validators.
    /// This is a glboal event that expects a particular `SignDataEvent` per each `Dwallet` type.
    struct NewSignSessionEvent<E: store + copy + drop> has copy, drop {
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        messages: vector<vector<u8>>,
        sender: address,
        sign_data_event: E,
    }

    // <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<

    #[allow(unused_field)]
    /// `DWallet` represents a wallet that is created after the DKG process.
    struct DWallet<phantom T> has key, store {
        id: UID,
        session_id: ID,
        dwallet_cap_id: ID,
        // `output` is output for `verify_decommitment_and_proof_of_centralized_party_public_key_share()`
        output: vector<u8>,
        public_key: vector<u8>,
    }

    public(friend) fun create_dwallet<T: drop>(
        session_id: ID,
        dwallet_cap_id: ID,
        output: vector<u8>,
        public_key: vector<u8>,
        ctx: &mut TxContext
    ): DWallet<T> {
        DWallet<T> {
            id: object::new(ctx),
            session_id,
            dwallet_cap_id,
            output,
            public_key,
        }
    }

    public fun get_output<T: drop>(dwallet: &DWallet<T>): vector<u8> { dwallet.output }

    public fun get_dwallet_cap_id<T: drop>(dwallet: &DWallet<T>): ID { dwallet.dwallet_cap_id }

    public fun get_public_key<T: drop>(dwallet: &DWallet<T>): vector<u8> { dwallet.public_key }

    /// `DWalletCap` holder controls a corresponding `Dwallet`.
    struct DWalletCap has key, store {
        id: UID,
    }

    /// `MessageApprovalsHolder` holds `MessageApproval's`.
    struct MessageApprovalsHolder has key {
        id: UID,
        message_approvals: vector<MessageApproval>,
    }

    /// `MessageApproval` represents a message that was approved.
    /// Bound to a `DWalletCap`.
    struct MessageApproval has store {
        dwallet_cap_id: ID,
        message: vector<u8>,
    }

    /// Partially signed messages by the user, these messasegs are ready to be signed by the blockchain.
    /// It's only half of the `Sign` process.
    /// To Sign a message both this Struct and `MessageApproval` must be present.
    /// The messeses field must be the same as the messages in the `MessageApproval`, and in the same order.
    struct PartialUserSignedMessages<S: store, E: store + copy + drop> has key, store {
        id: UID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        messages: vector<vector<u8>>,
        dwallet_public_key: vector<u8>,
        sign_data: S,
        sign_data_event: E,
    }

    /// `SharedPartialUserSignedMessages` is a shared version of `PartialUserSignedMessages`.
    /// Since this is shared it passes throuh concesus, use only when needed, prefer `PartialUserSignedMessages`.
    struct SharedPartialUserSignedMessages<S: store, E: store + copy + drop> has key {
        id: UID,
        partial_user_signed_messages: PartialUserSignedMessages<S, E>,
    }

    /// `SignSession` holds the `Sign` session data, created when a `Sign` request is sent to the netowrk.
    struct SignSession<S: store> has key {
        id: UID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        messages: vector<vector<u8>>,
        sender: address,
        sign_data: S,
        dwallet_public_key: vector<u8>,
    }

    #[allow(unused_function)]
    #[test_only]
    /// Creates a sign session.
    public fun create_mock_sign_session<S: store>(
        messages: vector<vector<u8>>, dwallet_public_key: vector<u8>, sign_data: S, ctx: &mut TxContext,
    ) {
        let session = SignSession<S> {
            dwallet_id: object::id_from_address(@0x0),
            messages,
            sign_data,
            dwallet_public_key,
            sender: tx_context::sender(ctx),
            dwallet_cap_id: object::id_from_address(@0x0),
            id: object::new(ctx),
        };
        transfer::freeze_object(session);
    }

    public(friend) fun get_dwallet_public_key<S: store>(
        session: &SignSession<S>
    ): vector<u8> { session.dwallet_public_key }

    public(friend) fun get_sign_data<S: store>(session: &SignSession<S>): &S { &session.sign_data }

    public(friend) fun get_messages<S: store>(session: &SignSession<S>): vector<vector<u8>> { session.messages }

    public(friend) fun get_sender<S: store>(session: &SignSession<S>): address { session.sender }

    #[allow(unused_field)]
    struct SignOutputEvent has copy, drop {
        sign_output_id: ID,
        signatures: vector<vector<u8>>,
        dwallet_id: ID
    }

    #[allow(unused_field)]
    /// `SignOutput` is the final output from the Bloackchian(Valditors) of the `Sign` process.
    struct SignOutput has key {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        signatures: vector<vector<u8>>,
        sender: address,
    }

    /// Create a new `DWalletCap`
    /// The holder of this capability owns the `DWallet`.
    public(friend) fun create_dwallet_cap(ctx: &mut TxContext): DWalletCap {
        DWalletCap {
            id: object::new(ctx),
        }
    }

    /// Create a new `MessageApprovalsHolder`.
    public fun create_message_approvals_holder(message_approvals: vector<MessageApproval>, ctx: &mut TxContext) {
        let holder = MessageApprovalsHolder {
            id: object::new(ctx),
            message_approvals,
        };
        transfer::transfer(holder, tx_context::sender(ctx));
    }

    /// Removes the `MessageApprovalsHolder` and return the `MessageApproval`'s.
    public fun remove_message_approvals_holder(holder: MessageApprovalsHolder): vector<MessageApproval> {
        let MessageApprovalsHolder {
            id,
            message_approvals,
        } = holder;
        object::delete(id);
        message_approvals
    }

    /// Create a set of message approvals.
    /// The messages must be approved in the same order as they were created.
    /// The messages must be approved by the same `dwallet_cap_id`.
    public fun approve_messages(
        dwallet_cap: &DWalletCap,
        messages: vector<vector<u8>>
    ): vector<MessageApproval> {
        let dwallet_cap_id = object::id(dwallet_cap);
        let message_approvals = vector::empty<MessageApproval>();
        while (vector::length(&messages) > 0) {
            let message = vector::pop_back(&mut messages);
            vector::push_back(&mut message_approvals, MessageApproval {
                dwallet_cap_id,
                message,
            });
        };
        message_approvals
    }

    /// Get the corresponding `DWalletCap` ID from a `MessageApproval`.
    public fun message_approval_dwallet_cap_id(msg_approval: &MessageApproval): ID {
        msg_approval.dwallet_cap_id
    }

    /// Get the `message` from a `MessageApproval`.
    public fun message_approval_message(message_approval: &MessageApproval): vector<u8> {
        message_approval.message
    }

    /// Remove a `MessageApproval` and return the `dwallet_cap_id` and the `message`.
    public fun remove_message_approval(message_approval: MessageApproval): (ID, vector<u8>) {
        let MessageApproval {
            dwallet_cap_id,
            message
        } = message_approval;
        (dwallet_cap_id, message)
    }

    /// Create a `SharedPartialUserSignedMessages`.
    public fun create_shared_partial_user_signed_messages<S: store, E: store + copy + drop>(
        partial_user_signed_messages: PartialUserSignedMessages<S, E>,
        ctx: &mut TxContext
    ) {
        let holder = SharedPartialUserSignedMessages {
            id: object::new(ctx),
            partial_user_signed_messages,
        };
        transfer::share_object(holder);
    }

    /// The shared version of `Sign` function.
    public fun sign_shared<S: store, E: store + copy + drop>(
        shared: SharedPartialUserSignedMessages<S, E>,
        message_approvals: vector<MessageApproval>,
        ctx: &mut TxContext
    ) {
        let SharedPartialUserSignedMessages {
            id,
            partial_user_signed_messages,
        } = shared;
        object::delete(id);
        sign(partial_user_signed_messages, message_approvals, ctx)
    }

    /// Create a `PartialUserSignedMessages`.
    /// This part only creates the object, it will be later used in the `sign()` function.
    public(friend) fun create_partial_user_signed_messages<S: store, E: store + copy + drop>(
        dwallet_id: ID,
        dwallet_cap_id: ID,
        messages: vector<vector<u8>>,
        dwallet_public_key: vector<u8>,
        sign_data: S,
        sign_data_event: E,
        ctx: &mut TxContext
    ): PartialUserSignedMessages<S, E> {
        PartialUserSignedMessages {
            id: object::new(ctx),
            dwallet_id,
            dwallet_cap_id,
            messages,
            sign_data,
            sign_data_event,
            dwallet_public_key,
        }
    }

    /// Get the `Dwallet` ID from a `PartialUserSignedMessages`.
    public fun partial_user_signed_messages_dwallet_id<S: store, E: store + copy + drop>(
        partial_user_signed_messages: &PartialUserSignedMessages<S, E>
    ): ID {
        partial_user_signed_messages.dwallet_id
    }

    /// Get the `DwalletCap` ID from a `PartialUserSignedMessages`.
    public fun partial_user_signed_messages_dwallet_cap_id<S: store, E: store + copy + drop>(
        partial_user_signed_messages: &PartialUserSignedMessages<S, E>
    ): ID {
        partial_user_signed_messages.dwallet_cap_id
    }

    /// Get the `messages` from a `PartialUserSignedMessages`.
    public fun partial_user_signed_messages_messages<S: store, E: store + copy + drop>(
        partial_user_signed_messages: &PartialUserSignedMessages<S, E>
    ): vector<vector<u8>> {
        partial_user_signed_messages.messages
    }

    /// Main sign function.
    /// Note that we must have MessageApproval's for the messages, and a PartialUserSignedMessages object.
    /// Both must hold the same messages in the same order.
    public fun sign<S: store, E: store + copy + drop>(
        partial_user_signed_messages: PartialUserSignedMessages<S, E>,
        message_approvals: vector<MessageApproval>,
        ctx: &mut TxContext
    ) {
        let PartialUserSignedMessages {
            id,
            dwallet_id,
            dwallet_cap_id,
            messages,
            sign_data,
            sign_data_event,
            dwallet_public_key,
        } = partial_user_signed_messages;

        object::delete(id);
        let messages_len: u64 = vector::length(&messages);
        let approval_len: u64 = vector::length(&message_approvals);
        assert!(messages_len == approval_len, EMesssageApprovalDWalletMismatch);

        let i: u64 = 0;
        while (i < messages_len) {
            let message_approval = vector::pop_back(&mut message_approvals);
            let (message_approval_dwallet_cap_id, approved_message) = remove_message_approval(message_approval);
            assert!(dwallet_cap_id == message_approval_dwallet_cap_id, EMesssageApprovalDWalletMismatch);
            let message = vector::borrow(&messages, i);
            assert!(message == &approved_message, EMesssageApprovalDWalletMismatch);
            i = i + 1;
        };

        vector::destroy_empty(message_approvals);
        let sender = tx_context::sender(ctx);

        let sign_session = SignSession {
            id: object::new(ctx),
            dwallet_id,
            dwallet_cap_id,
            messages,
            sender,
            sign_data,
            dwallet_public_key
        };

        // This part actaully starts the `Sign` proccess in the blockchain.
        event::emit(NewSignSessionEvent {
            session_id: object::id(&sign_session),
            dwallet_id,
            dwallet_cap_id,
            messages,
            sender,
            sign_data_event,
        });
        transfer::freeze_object(sign_session);
    }

    /// The output that being written when an aggregator tries to publish an invalid signature.
    struct MaliciousAggregatorSignOutput has key {
        id: UID,
        aggregator_public_key: vector<u8>,
        epoch: u64,
        signatures: vector<vector<u8>>,
        messages: vector<vector<u8>>,
        dwallet_id: ID,
        session_id: ID,
    }

    /// An event that being emitted when an aggregator tries to publish an invalid signature.
    /// Being used to punish the aggregator.
    struct MaliciousAggregatorEvent has copy, drop {
        aggregator_public_key: vector<u8>,
        epoch: u64,
        signatures: vector<vector<u8>>,
        messages: vector<vector<u8>>,
        dwallet_id: ID,
        malicious_sign_output_id: ID,
    }

    /// Generic function to create a `SignOutput`.
    /// Creates the output for various signature algorithms.
    public(friend) fun create_sign_output<S: store>(
        session: &SignSession<S>,
        signatures: vector<vector<u8>>,
        ctx: &mut TxContext) {
        let sign_output = SignOutput {
            id: object::new(ctx),
            session_id: object::id(session),
            dwallet_id: session.dwallet_id,
            dwallet_cap_id: session.dwallet_cap_id,
            signatures,
            sender: session.sender,
        };
        event::emit(SignOutputEvent {
            sign_output_id: object::id(&sign_output),
            signatures,
            dwallet_id: session.dwallet_id,
        });
        transfer::freeze_object(sign_output);
    }

    /// Generic function to create a `MaliciousAggregatorSignOutput`.
    /// Creates the output for various signature algorithms.
    public(friend) fun create_malicious_aggregator_sign_output<S: store>(
        aggregator_public_key: vector<u8>,
        session: &SignSession<S>,
        signatures: vector<vector<u8>>,
        ctx: &mut TxContext
    ) {
        let failed_sign_output = MaliciousAggregatorSignOutput {
            id: object::new(ctx),
            aggregator_public_key,
            epoch: tx_context::epoch(ctx),
            signatures,
            messages: session.messages,
            dwallet_id: session.dwallet_id,
            session_id: object::id(session),
        };
        event::emit(MaliciousAggregatorEvent {
            aggregator_public_key,
            epoch: tx_context::epoch(ctx),
            signatures,
            messages: session.messages,
            dwallet_id: session.dwallet_id,
            malicious_sign_output_id: object::id(&failed_sign_output),
        });
        transfer::freeze_object(failed_sign_output);
    }

    /// Encrypt DWallet secret share with an AHE public key.
    struct EncryptedUserShare has key {
        id: UID,
        dwallet_id: ID,
        encrypted_secret_share_and_proof: vector<u8>,
        encryption_key_id: ID,
    }

    /// An Additively Homomorphic Encryption (AHE) public key
    /// that can be used to encrypt a user share in order to prove to the network that
    /// the recipient can sign with a dWallet when it is transferred or access is granted to it.
    struct EncryptionKey has key {
        id: UID,
        scheme: u8,
        encryption_key: vector<u8>,
        key_owner_address: address,
    }

    public fun get_encryption_key(encryption_key: &EncryptionKey): vector<u8> {
        encryption_key.encryption_key
    }

    const Paillier: u8 = 0;

    fun is_valid_encryption_key_scheme(scheme: u8): bool {
        scheme == Paillier // || scheme == ...
    }

    /// Register an encryption key to encrypt a user share.
    /// The key is saved as an immutable object.
    public fun register_encryption_key(key: vector<u8>, scheme: u8, ctx: &mut TxContext) {
        assert!(is_valid_encryption_key_scheme(scheme), EInvalidEncryptionKeyScheme);
        let encryption_key = EncryptionKey {
            id: object::new(ctx),
            scheme,
            encryption_key: key,
            key_owner_address: tx_context::sender(ctx),
        };
        transfer::freeze_object(encryption_key);
    }

    /// Shared object that holds the active encryption keys per user.
    /// 'encryption_keys' is a key-value table where the key is the user address
    /// and the value is the encryption key object ID.
    struct ActiveEncryptionKeys has key {
        id: UID,
        encryption_keys: Table<address, ID>,
    }

    /// Create a shared object that holds the active encryption keys per user.
    public fun create_active_encryption_keys(ctx: &mut TxContext) {
        let holder = ActiveEncryptionKeys {
            id: object::new(ctx),
            encryption_keys: table::new(ctx),
        };
        transfer::share_object(holder);
    }

    /// Set the active encryption key for a user (the sender).
    public fun set_active_encryption_key(
        encryption_key_holder: &mut ActiveEncryptionKeys,
        encryption_key: &EncryptionKey,
        ctx: &mut TxContext
    ) {
        assert!(encryption_key.key_owner_address == tx_context::sender(ctx), EInvalidEncryptionKeyOwner);
        if (table::contains(&encryption_key_holder.encryption_keys, encryption_key.key_owner_address)) {
            table::remove(&mut encryption_key_holder.encryption_keys, encryption_key.key_owner_address);
        };
        table::add(
            &mut encryption_key_holder.encryption_keys,
            encryption_key.key_owner_address,
            object::id(encryption_key)
        );
    }

    /// Get the active encryption key ID by user adderss.
    public fun get_active_encryption_key(
        encryption_key_holder: &ActiveEncryptionKeys,
        key_owner: address,
    ): &ID {
        table::borrow(&encryption_key_holder.encryption_keys, key_owner)
    }

    public(friend) fun create_encrypted_user_share(
        dwallet_id: ID,
        encrypted_secret_share_and_proof: vector<u8>,
        encryption_key_id: ID,
        ctx: &mut TxContext
    ) {
        let encrypted_user_share = EncryptedUserShare {
            id: object::new(ctx),
            dwallet_id,
            encrypted_secret_share_and_proof,
            encryption_key_id,
        };
        transfer::freeze_object(encrypted_user_share);
    }
}
