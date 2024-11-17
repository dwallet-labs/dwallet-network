// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module pera_system::dwallet_2pc_mpc_ecdsa_k1 {
    use pera_system::dwallet;
    use pera_system::dwallet::{create_dwallet_cap, DWalletCap};
    use pera::event;

    /// Represents the Secp256K1 DWallet type.
    public struct Secp256K1 has drop {}

    /// A struct to hold the output of the first round of the DKG.
    /// An instance of this struct is being transferred to the user that initiated the DKG after
    /// the first round is completed.
    /// The user can then use this output to start the second round of the DKG.
    public struct DKGFirstRoundOutput has key {
        id: UID,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
    }

    // <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<
    /// Event to start a `DKG` session, caught by the Validators.
    public struct StartDKGFirstRoundEvent has copy, drop {
        session_id: address,
        sender: address,
        dwallet_cap_id: ID,
    }

    public struct CompletedFirstDKGRoundEvent has copy, drop {
        session_id: ID,
        sender: address,
    }

    /// Being emitted to start the validator's second DKG round
    /// Each validators catch this event and start the second round of the DKG.
    public struct StartDKGSecondRoundEvent has copy, drop {
        session_id: address,
        sender: address,
        first_round_output: vector<u8>,
        public_key_share_and_proof: vector<u8>,
        dwallet_cap_id: ID,
        first_round_session_id: ID,
    }

    /// Being emitted when the second round of the DKG is completed.
    /// Contains all the relevant data from that round.
    public struct CompletedDKGSecondRoundEvent has copy, drop {
        session_id: ID,
        sender: address,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        value: vector<u8>,
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
    public fun launch_dkg_first_round(
        ctx: &mut TxContext
    ): address {
        let dwallet_cap_id = create_dwallet_cap(ctx);
        let sender = tx_context::sender(ctx);
        let session_id = tx_context::fresh_object_address(ctx);
        event::emit(StartDKGFirstRoundEvent {
            session_id,
            sender,
            dwallet_cap_id,
        });
        session_id
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
            session_id,
            output,
            dwallet_cap_id,
        };
        transfer::transfer(output, sender);

        event::emit(CompletedFirstDKGRoundEvent {
            session_id,
            sender,
        });
    }

    /// Function to launch the second DKG round.
    /// Emits an event with all the needed data.
    /// Each validator then catches this event and starts the second round of the DKG.
    public fun launch_dkg_second_round(
        dwallet_cap: &DWalletCap,
        public_key_share_and_proof: vector<u8>,
        first_round_output: vector<u8>,
        first_round_session_id: ID,
        ctx: &mut TxContext
    ): address {
        let session_id = tx_context::fresh_object_address(ctx);
        let created_proof_mpc_session_event = StartDKGSecondRoundEvent {
            session_id,
            sender: tx_context::sender(ctx),
            first_round_output,
            public_key_share_and_proof,
            dwallet_cap_id: object::id(dwallet_cap),
            first_round_session_id
        };
        event::emit(created_proof_mpc_session_event);
        session_id
    }

    /// Create the second DKG MPC first output, which is the actual [`dwallet::DWallet`].
    /// This function is called by blockchain itself.
    /// Validators call it, it's part of the blockchain logic.
    public fun create_dkg_second_round_output(
        session_initiator: address,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        let dwallet = dwallet::create_dwallet<Secp256K1>(session_id, dwallet_cap_id, output, ctx);
        let completed_proof_mpc_session_event = CompletedDKGSecondRoundEvent {
            session_id,
            sender: session_initiator,
            dwallet_cap_id,
            dwallet_id: object::id(&dwallet),
            value: output,
        };
        transfer::public_freeze_object(dwallet);
        event::emit(completed_proof_mpc_session_event);
    }
}