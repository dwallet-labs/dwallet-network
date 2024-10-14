// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module pera_system::dwallet {
    use pera::event;

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    const ENotSystemAddress: u64 = 0;
    // >>>>>>>>>>>>>>>>>>>>>>>> Error codes >>>>>>>>>>>>>>>>>>>>>>>>

    public struct InitiateDKGSessionEvent has copy, drop {
        session_id: ID,
        sender: address,
    }

    public struct InitiateDKGSessionData has key {
        id: UID,
        sender: address,
    }

    public struct CompletedFirstDKGRoundData has key {
        id: UID,
        session_id: ID,
        value: vector<u8>,
    }

    public struct CompletedDKGRoundEvent has copy, drop {
        session_id: ID,
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
            sender: tx_context::sender(ctx)
        };
        event::emit(created_proof_mpc_session_event);
        transfer::freeze_object(session_data);
    }

    public fun create_first_dkg_round_output(session_initiator: address, session_id: ID, output: vector<u8>, ctx: &mut TxContext) {
       assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
       let proof_session_result = CompletedFirstDKGRoundData {
           id: object::new(ctx),
           session_id: session_id,
           value: output,
       };
       transfer::transfer(proof_session_result, session_initiator);

       let completed_proof_mpc_session_event = CompletedDKGRoundEvent {
           session_id: session_id,
           sender: session_initiator,
       };

       event::emit(completed_proof_mpc_session_event);
   }
}
