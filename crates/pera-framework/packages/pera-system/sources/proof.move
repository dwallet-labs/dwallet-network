// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// TODO (#228): Remove this module.
/// The proof module
/// Responsible to start and manage the Proof generation MPC flow.
/// Used only for testing the way we launch & manage an MPC flow.
module pera_system::proof {
    use pera::event;

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    const ENotSystemAddress: u64 = 0;
    // >>>>>>>>>>>>>>>>>>>>>>>> Error codes >>>>>>>>>>>>>>>>>>>>>>>>

    /// Event to start a `ProofMPCSession`, caught by the Validators.
    public struct CreatedProofMPCSessionEvent has copy, drop {
        session_id: ID,
        sender: address,
    }

    /// Event that is being emitted when the proof MPC flow is completed.
    public struct CompletedProofMPCSessionEvent has copy, drop {
        session_id: ID,
        sender: address,
    }

    /// Stores the session data for the proof MPC flow.
    public struct ProofSessionData has key {
        id: UID,
    }

    /// Function to launch proof MPC flow.
    public fun launch_proof_mpc_flow(ctx: &mut TxContext) {
        let session_data = ProofSessionData {
            id: object::new(ctx),
        };
        // Emit event to start MPC flow.
        // Part of the implementation of section 3.2.1 in the Pera Async paper.
        // When iterating over the transactions in the batch,
        // MPC transactions will get executed locally
        // to catch the event with all the needed data to start the MPC flow.
        let created_proof_mpc_session_event = CreatedProofMPCSessionEvent {
            // The session ID is a random, unique ID of the ProofSessionData object.
            // It is needed so the user will be able to know, when fetching the generated proof data,
            // that the proof was generated for this specific session.
            session_id: object::id(&session_data),
            sender: tx_context::sender(ctx),
        };
        event::emit(created_proof_mpc_session_event);
        transfer::freeze_object(session_data);
    }

    /// Stores the result of the proof MPC flow so it will be accessible for the initiating user.
    public struct ProofSessionOutput has key {
        id: UID,
        session_id: ID,
        proof: vector<u8>,
    }

    /// Function to create the proof session output.
    /// Creates it and transfers it to the user that initiated the proof MPC flow.
    /// Should be called only as a system transaction after
    /// all the validators received and verified the Rust `SignatureMPCOutput`.
    public fun create_proof_session_output(
        session_initiator: address,
        session_id: ID,
        output: vector<u8>,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        let proof_session_result = ProofSessionOutput {
            id: object::new(ctx),
            session_id,
            proof: output,
        };
        transfer::transfer(proof_session_result, session_initiator);

        let completed_proof_mpc_session_event = CompletedProofMPCSessionEvent {
            session_id: session_id,
            sender: session_initiator,
        };

        event::emit(completed_proof_mpc_session_event);
    }
}
