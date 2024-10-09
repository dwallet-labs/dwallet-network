// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// TODO (#228): Remove this module.
/// The proof module
/// Responsible to start and manage the Proof generation MPC flow
/// Used only for testing the way we launch & manage an MPC flow.
module pera_system::proof {
    use pera::event;

    /// Event to start a `MockMPCSession`, caught by the Validators.
    public struct CreatedProofMPCSessionEvent has copy, drop {
        session_id: ID,
        sender: address,
    }

    /// Event that is being emitted when the proof MPC flow is completed.
    public struct CompletedProofMPCSessionEvent has copy, drop {
        session_id: ID,
        sender: address,
    }

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
        // When iterating over the transactions in the batch, MPC transactions will get exectuted locally
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

    public struct ProofSessionResult has key {
        id: UID,
        session_id: ID,
        proof: vector<vector<u8>>,
    }

   public fun create_proof_session_result(session_initiator: address, session_id: ID, output: vector<vector<u8>>, ctx: &mut TxContext) {
       let proof_session_result = ProofSessionResult {
           id: object::new(ctx),
           session_id: session_id,
           proof: output,
       };
       transfer::transfer(proof_session_result, @0xbca51aa9957d2f3ebf39b270119c644862c32111295cd9f29caa88a41aab8199);

       let completed_proof_mpc_session_event = CompletedProofMPCSessionEvent {
           session_id: session_id,
           sender: session_initiator,
       };

       event::emit(completed_proof_mpc_session_event);
   }
}
