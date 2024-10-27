// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_const)]
module pera_system::dwallet_2pc_mpc_ecdsa_k1 {
    use pera_system::dwallet::{create_dwallet_cap, DWalletCap};
    use pera::event;

    public struct DKGSession has key {
        id: UID,
        dwallet_cap_id: ID,
        sender: address,
    }

    public struct DKGFirstRoundOutput has key {
        id: UID,
        session_id: ID,
        output: vector<u8>,
    }

    // <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<
    /// Event to start a `DKG` session, caught by the Validators.
    public struct CreatedDKGSessionEvent has copy, drop {
        session_id: ID,
        sender: address,
    }
    // >>>>>>>>>>>>>>>>>>>>>>>> Events >>>>>>>>>>>>>>>>>>>>>>>>

    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<
    const SYSTEM_ADDRESS: address = @0x0;
    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<

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
        });
        transfer::freeze_object(session);
        cap
    }

    #[allow(unused_function)]
    /// Create the first DKG MPC first round output, transfer it to the initiating user.
    /// This function is called by blockchain itself.
    /// Validtors call it, it's part of the blockchain logic.
    fun create_dkg_first_round_output(
        session: &DKGSession,
        output: vector<u8>,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
        let output = DKGFirstRoundOutput {
            id: object::new(ctx),
            session_id: object::id(session),
            output,
        };
        transfer::transfer(output, session.sender);
    }
}