// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dwallet_system::dwallet {
    use std::vector;
    use dwallet::object::{Self, UID, ID};
    use dwallet::transfer;
    use dwallet::tx_context;
    use dwallet::tx_context::{TxContext};

    friend dwallet_system::dwallet_2pc_mpc_ecdsa_k1;

    const ENotSystemAddress: u64 = 0;
    const EMesssageApprovalDWalletMismatch: u64 = 1;

    struct DWalletCap has key, store {
        id: UID,
    }

    struct ApprovalsHolder has key {
        id: UID,
        message_approvals: vector<MessageApproval>,
    }

    struct MessageApproval has store, drop {
        dwallet_cap_id: ID,
        message: vector<u8>,
    }

    struct SignMessages<S: store> has key, store {
        id: UID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        messages: vector<vector<u8>>,
        sign_data: S,
    }

    struct SignSession<S> has key {
        id: UID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        messages: vector<vector<u8>>,
        sign_data: S,
        sender: address,
    }

    #[allow(unused_field)]
    struct SignOutput has key {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        signatures: vector<vector<u8>>,
        sender: address,
    }

    public(friend) fun create_dwallet_cap(ctx: &mut TxContext): DWalletCap {
        DWalletCap {
            id: object::new(ctx),
        }
    }

    public fun create_approvals_holder(message_approvals: vector<MessageApproval>, ctx: &mut TxContext) {
        let holder = ApprovalsHolder {
            id: object::new(ctx),
            message_approvals,
        };
        transfer::transfer(holder, tx_context::sender(ctx));
    }

    public fun remove_approvals_holder(holder: ApprovalsHolder): vector<MessageApproval> {
        let ApprovalsHolder {
            id,
            message_approvals,
        } = holder;
        object::delete(id);
        message_approvals
    }

    public fun approve_messages(dwallet_cap: &DWalletCap, messages: vector<vector<u8>>): vector<MessageApproval> {
        let dwallet_cap_id = object::id(dwallet_cap);
        let message_approvals = vector::empty<MessageApproval>();
        while(vector::length(&messages) > 0) {
            let message = vector::pop_back(&mut messages);
            vector::push_back(&mut message_approvals, MessageApproval {
                dwallet_cap_id,
                message,
            });
        };
        message_approvals
    }

    public fun dwallet_cap_id(message_approval: &MessageApproval): ID {
        message_approval.dwallet_cap_id
    }

    public fun message(message_approval: &MessageApproval): vector<u8> {
        message_approval.message
    }

    public fun remove(message_approval: MessageApproval): (ID, vector<u8>) {
        let MessageApproval {
            dwallet_cap_id,
            message
        } = message_approval;
        (dwallet_cap_id, message)
    }

    public(friend) fun create_sign_messages<T: store>(dwallet_id: ID, dwallet_cap_id: ID, messages: vector<vector<u8>>, sign_data: T, ctx: &mut TxContext): SignMessages<T> {
        SignMessages {
            id: object::new(ctx),
            dwallet_id,
            dwallet_cap_id,
            messages,
            sign_data,
        }
    }

    public fun sign_messages_dwallet_id<T: store>(sign_messages: &SignMessages<T>): ID {
        sign_messages.dwallet_id
    }

    public fun sign_messages_dwallet_cap_id<T: store>(sign_messages: &SignMessages<T>): ID {
        sign_messages.dwallet_cap_id
    }

    public fun sign_messages_messages<T: store>(sign_messages: &SignMessages<T>): vector<vector<u8>> {
        sign_messages.messages
    }

    public fun sign_messages<S: store>(sign_messages: SignMessages<S>, message_approvals: vector<MessageApproval>, ctx: &mut TxContext) {
        let SignMessages {
            id,
            dwallet_id,
            dwallet_cap_id,
            messages,
            sign_data,
        } = sign_messages;

        object::delete(id);
        let i = 0;
        let messages_len = vector::length(&messages);
        let approval_len = vector::length(&message_approvals);
        assert!(messages_len == approval_len, EMesssageApprovalDWalletMismatch);

        while (i < messages_len) {
            let message_approval = vector::pop_back(&mut message_approvals);
            let (message_approval_dwallet_cap_id, approved_message) = remove(message_approval);
            assert!(dwallet_cap_id == message_approval_dwallet_cap_id, EMesssageApprovalDWalletMismatch);
            let message = vector::borrow(&messages, i);
            assert!(message == &approved_message, EMesssageApprovalDWalletMismatch);
            i = i +1;
        };

        vector::destroy_empty(message_approvals);

        let sign_session = SignSession {
            id: object::new(ctx),
            dwallet_id,
            dwallet_cap_id,
            messages,
            sign_data,
            sender: tx_context::sender(ctx),
        };
        transfer::freeze_object(sign_session);
    }

    #[allow(unused_function)]
    fun create_sign_output<T: store>(session: &SignSession<T>, signatures: vector<vector<u8>>, ctx: &mut TxContext) {
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
