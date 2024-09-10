// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// The proof module
/// Responsible to start and manage the Proof generation MPC flow
module dwallet_system::proof {
    use dwallet::event;
    use dwallet::object::{Self, ID, UID};
    use dwallet::transfer;
    use dwallet::tx_context;
    use dwallet::tx_context::TxContext;

    /// Event to start a `MockMPCSession`, caught by the Validators.
    struct CreatedProofMPCSessionEvent has copy, drop {
        session_id: ID,
        sender: address,
    }

    struct ProofSessionData has key {
        id: UID,
    }

    /// Function to launch proof MPC flow.
    public fun launch_proof_mpc_flow(ctx: &mut TxContext) {
        let session_data = ProofSessionData {
            id: object::new(ctx),
        };
        // Emit event to start MPC flow.
        // Part of the implementation of section 3.2.1 in the DWallet Async paper.
        // When iterating over the transactions in the batch, MPC transactions will get exectuted locally
        // to catch the event with all the needed data to start the MPC flow.
        let created_proof_mpc_session_event = CreatedProofMPCSessionEvent {
            session_id: object::id(&session_data),
            sender: tx_context::sender(ctx),
        };
        event::emit(created_proof_mpc_session_event);
        transfer::freeze_object(session_data);
    }
}