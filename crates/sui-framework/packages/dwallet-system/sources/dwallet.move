// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// Generic dWallet representation.
/// This is conceptually the `Dwallet` interface.
module dwallet_system::dwallet {
    use std::vector;

    use dwallet::event;
    use dwallet::object::{Self, ID, UID};
    use dwallet::transfer;
    use dwallet::tx_context;
    use dwallet::tx_context::TxContext;

    friend dwallet_system::dwallet_2pc_mpc_ecdsa_k1;

    #[test_only]
    friend dwallet_system::dwallet_tests;

    #[test_only]
    friend dwallet_system::dwallet_ecdsa_k1_tests;

    #[test_only]
    friend dwallet_system::native_dwallet_tests;

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    const ENotSystemAddress: u64 = 0;
    const EMesssageApprovalDWalletMismatch: u64 = 1;

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

    #[allow(unused_function)]
    /// This function is called by blockchain itself.
    /// Validtors call it, it's part of the blockchain logic.
    /// NOT a native function.
    fun create_sign_output<S: store>(session: &SignSession<S>, signatures: vector<vector<u8>>, ctx: &mut TxContext) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);

        let sign_output = SignOutput {
            id: object::new(ctx),
            session_id: object::id(session),
            dwallet_id: session.dwallet_id,
            dwallet_cap_id: session.dwallet_cap_id,
            signatures,
            sender: session.sender,
        };
        transfer::transfer(sign_output, session.sender);
    }
}
