// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module pera_system::dwallet {
    use pera::event;

    public struct InitiateDKGSessionEvent has copy, drop {
        session_id: ID,
    }

    public struct InitiateDKGSessionData has key {
        id: UID,
        sender: address,
    }

    /// Function to launch proof MPC flow.
    public fun launch_initiate_dkg_session(ctx: &mut TxContext) {
        let session_data = InitiateDKGSessionData {
            id: object::new(ctx),
            sender: tx_context::sender(ctx)
        };
        let created_proof_mpc_session_event = InitiateDKGSessionEvent {
            session_id: object::id(&session_data),
        };
        event::emit(created_proof_mpc_session_event);
        transfer::freeze_object(session_data);
    }
}
