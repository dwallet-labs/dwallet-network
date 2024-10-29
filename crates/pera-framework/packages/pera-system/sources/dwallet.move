// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module pera_system::dwallet {
    use pera::event;

        #[allow(unused_field)]
        /// `DWallet` represents a wallet that is created after the DKG process.
        public struct DWallet<phantom T> has key, store {
            id: UID,
            session_id: ID,
            dwallet_cap_id: ID,
            // `output` of the DKG decentralized process.
            output: vector<u8>,
        }

        public(package) fun create_dwallet<T: drop>(
            session_id: ID,
            dwallet_cap_id: ID,
            output: vector<u8>,
            ctx: &mut TxContext
        ): DWallet<T> {
            DWallet<T> {
                id: object::new(ctx),
                session_id,
                dwallet_cap_id,
                output,
            }
        }

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

    public struct StartDKGSecondRoundEvent has copy, drop {
        session_id: ID,
        sender: address,
        first_round_output: vector<u8>,
        public_key_share_and_proof: vector<u8>
    }


    public struct DKGSecondRoundData has key {
        id: UID,
        sender: address,
        input: vector<u8>
    }

    public struct CompletedFirstDKGRoundData has key {
        id: UID,
        session_id: ID,
        value: vector<u8>,
    }

    public struct CompletedSecondDKGRoundData has key {
            id: UID,
            session_id: ID,
            value: vector<u8>,
        }

    public struct CompletedDKGRoundEvent has copy, drop {
        session_id: ID,
        sender: address,
    }

    public struct CompletedSecondDKGRoundEvent has copy, drop {
            session_id: ID,
            sender: address,
        }

    /// `DWalletCap` holder controls a corresponding `Dwallet`.
    public struct DWalletCap has key, store {
        id: UID,
    }

    /// Create a new `DWalletCap`
    /// The holder of this capability owns the `DWallet`.
    public(package) fun create_dwallet_cap(ctx: &mut TxContext): DWalletCap {
        DWalletCap {
            id: object::new(ctx),
        }
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

    /// Function to launch proof MPC flow.
    public fun launch_dkg_second_round(public_key_share_and_proof: vector<u8>, first_round_output: vector<u8>, ctx: &mut TxContext) {
        let session_data = DKGSecondRoundData {
            id: object::new(ctx),
            sender: tx_context::sender(ctx),
            input: first_round_output
        };
        let created_proof_mpc_session_event = StartDKGSecondRoundEvent {
            session_id: object::id(&session_data),
            sender: tx_context::sender(ctx),
            first_round_output,
            public_key_share_and_proof
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

   public fun create_second_dkg_round_output(session_initiator: address, session_id: ID, output: vector<u8>, ctx: &mut TxContext) {
      assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
      let proof_session_result = CompletedSecondDKGRoundData {
          id: object::new(ctx),
          session_id: session_id,
          value: output,
      };
      transfer::transfer(proof_session_result, session_initiator);

      let completed_proof_mpc_session_event = CompletedSecondDKGRoundEvent {
          session_id: session_id,
          sender: session_initiator,
      };

      event::emit(completed_proof_mpc_session_event);
  }
}
