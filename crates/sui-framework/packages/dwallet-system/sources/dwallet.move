// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dwallet_system::dwallet {
    use std::vector;
    use dwallet::object::{Self, UID, ID};
    use dwallet::tx_context::{TxContext};

    friend dwallet_system::dwallet_2pc_mpc_ecdsa_k1;

    struct DWalletCap has key, store {
        id: UID,
    }

    struct MessageApproval has store {
        dwallet_cap_id: ID,
        message: vector<u8>,
    }

    public(friend) fun create_dwallet_cap(ctx: &mut TxContext): DWalletCap {
        DWalletCap {
            id: object::new(ctx),
        }
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


}
