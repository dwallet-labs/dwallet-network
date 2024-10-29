// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_const)]
module pera_system::dwallet_2pc_mpc_ecdsa_k1 {
    use pera_system::dwallet;
    use pera_system::dwallet::{create_dwallet_cap, DWalletCap};
    use pera::event;

    public struct Secp256K1 has drop {}

    public struct DKGSession has key {
        id: UID,
        dwallet_cap_id: ID,
        sender: address,
    }

    public struct DKGFirstRoundOutput has key {
        id: UID,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
    }

    // <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<
    /// Event to start a `DKG` session, caught by the Validators.
    public struct CreatedDKGSessionEvent has copy, drop {
        session_id: ID,
        sender: address,
        dwallet_cap_id: ID,
    }

    public struct CompletedDKGRoundEvent has copy, drop {
        session_id: ID,
        sender: address,
    }
    // >>>>>>>>>>>>>>>>>>>>>>>> Events >>>>>>>>>>>>>>>>>>>>>>>>

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    const ENotSystemAddress: u64 = 0;
    // >>>>>>>>>>>>>>>>>>>>>>>> Error codes >>>>>>>>>>>>>>>>>>>>>>>>

    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<
    const SYSTEM_ADDRESS: address = @0x0;
    // >>>>>>>>>>>>>>>>>>>>>>>> Constants >>>>>>>>>>>>>>>>>>>>>>>>

    /// Starts the first Distributed Key Generation (DKG) session. Two MPC sessions are required to
    /// create a Dwallet.
    /// Capabilities are used to control access to the Dwallet.
    /// This function start the DKG proccess in the Validators.
    public fun start_first_dkg_session(
        ctx: &mut TxContext
    ): DWalletCap {
        let cap = create_dwallet_cap(ctx);
        let sender = tx_context::sender(ctx);
        let session = DKGSession {
            id: object::new(ctx),
            dwallet_cap_id: object::id(&cap),
            sender,
        };
        event::emit(CreatedDKGSessionEvent {
            session_id: object::id(&session),
            sender,
            dwallet_cap_id: object::id(&cap),
        });
        transfer::freeze_object(session);
        cap
    }

    #[allow(unused_function)]
    /// Create the first DKG MPC first round output, transfer it to the initiating user.
    /// This function is called by blockchain itself.
    /// Validtors call it, it's part of the blockchain logic.
    fun create_dkg_first_round_output(
        sender: address,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
        let output = DKGFirstRoundOutput {
            id: object::new(ctx),
            session_id: session_id,
            output,
            dwallet_cap_id,
        };
        transfer::transfer(output, sender);

       let completed_proof_mpc_session_event = CompletedDKGRoundEvent {
           session_id: session_id,
           sender: sender,
       };

       event::emit(completed_proof_mpc_session_event);
    }

        public struct StartDKGSecondRoundEvent has copy, drop {
            session_id: ID,
            sender: address,
            first_round_output: vector<u8>,
            public_key_share_and_proof: vector<u8>,
            dwallet_cap_id: ID,
        }


      public struct DKGSecondRoundData has key {
          id: UID,
          sender: address,
          input: vector<u8>
      }

      public struct CompletedSecondDKGRoundEvent has copy, drop {
              session_id: ID,
              sender: address,
              dwallet_cap_id: ID,
              dwallet_id: ID,
              value: vector<u8>,
          }

        /// Function to launch proof MPC flow.
        public fun launch_dkg_second_round(dwallet_cap: &DWalletCap, public_key_share_and_proof: vector<u8>, first_round_output: vector<u8>, ctx: &mut TxContext) {
            let session_data = DKGSecondRoundData {
                id: object::new(ctx),
                sender: tx_context::sender(ctx),
                input: first_round_output
            };
            let created_proof_mpc_session_event = StartDKGSecondRoundEvent {
                session_id: object::id(&session_data),
                sender: tx_context::sender(ctx),
                first_round_output,
                public_key_share_and_proof,
                dwallet_cap_id: object::id(dwallet_cap),
            };
            event::emit(created_proof_mpc_session_event);
            transfer::freeze_object(session_data);
        }


           public fun create_second_dkg_round_output(session_initiator: address, session_id: ID, output: vector<u8>, dwallet_cap_id: ID, ctx: &mut TxContext) {
              assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
              let dwallet = dwallet::create_dwallet<Secp256K1>(session_id, dwallet_cap_id, output, ctx);

              let completed_proof_mpc_session_event = CompletedSecondDKGRoundEvent {
                  session_id: session_id,
                  sender: session_initiator,
                  dwallet_cap_id: dwallet_cap_id,
                  dwallet_id: object::id(&dwallet),
                  value: output,
              };

              transfer::public_freeze_object(dwallet);
              event::emit(completed_proof_mpc_session_event);
          }


}