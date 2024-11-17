// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// This module handles the logic for creating and dWallets using the Secp256K1 signature scheme
/// and the DKG process. It leverages validators to execute MPC (Multi-Party Computation)
/// protocols to ensure trustless and decentralized wallet creation and key management.
///
/// ## Overview
///
/// - **Secp256K1**: The cryptographic curve used for this implementation.
/// - dWallets are created through two phases of DKG:
///   1. The first phase outputs partial results for the user.
///   2. The second phase generates the dWallet.
/// - **Capabilities**: Access to dWallets is controlled via capabilities (`DWalletCap`).
///
/// ## Features
///
/// - Emit events for validators to coordinate DKG rounds.
/// - Transfer intermediate results and final outputs to the initiating user.
/// - Ensure secure and decentralized key generation and management.
module pera_system::dwallet_2pc_mpc_ecdsa_k1 {
    use pera_system::dwallet;
    use pera_system::dwallet::{create_dwallet_cap, DWalletCap};
    use pera::event;

    /// Represents the `Secp256K1` dWallet type.
    /// This struct is a phantom type that signifies the dWallet cryptographic scheme.
    public struct Secp256K1 has drop {}

    /// Holds the output of the first DKG round.
    /// The first-round output is transferred to the user after the initial phase is completed.
    /// It is then used to initiate the second round of the DKG.
    public struct DKGFirstRoundOutput has key {
        id: UID,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
    }

    // <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<
    /// Event emitted to start the first DKG round.
    /// Validators catch this event to initiate the first round of the DKG.
    public struct StartDKGFirstRoundEvent has copy, drop {
        session_id: address,
        sender: address,
        dwallet_cap_id: ID,
    }

    /// Event emitted when the first round of the DKG is completed.
    public struct CompletedFirstDKGRoundEvent has copy, drop {
        session_id: ID,
        initiator: address,
    }

    /// Event emitted to start the second DKG round.
    /// Validators catch this event to start the second round of the DKG process.
    public struct StartDKGSecondRoundEvent has copy, drop {
        session_id: address,
        sender: address,
        first_round_output: vector<u8>,
        public_key_share_and_proof: vector<u8>,
        dwallet_cap_id: ID,
        first_round_session_id: ID,
    }

    /// Event emitted when the second round of the DKG is completed.
    /// Contains all relevant data from the second DKG round.
    public struct CompletedDKGSecondRoundEvent has copy, drop {
        session_id: ID,
        sender: address,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        value: vector<u8>,
    }
    // >>>>>>>>>>>>>>>>>>>>>>>> Events >>>>>>>>>>>>>>>>>>>>>>>>

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    /// Error raised when the sender is not the system address.
    const ENotSystemAddress: u64 = 0;
    // >>>>>>>>>>>>>>>>>>>>>>>> Error codes >>>>>>>>>>>>>>>>>>>>>>>>

    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<
    /// System address for asserting system-level actions.
    const SYSTEM_ADDRESS: address = @0x0;
    // >>>>>>>>>>>>>>>>>>>>>>>> Constants >>>>>>>>>>>>>>>>>>>>>>>>

    /// Starts the first Distributed Key Generation (DKG) session.
    /// This function initializes the DKG process for creating a dWallet.
    /// It emits an event for validators to begin the first round of DKG.
    /// The dWallet capability (`DWalletCap`) is created and transferred to the user.
    public fun launch_dkg_first_round(ctx: &mut TxContext): address {
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

    /// Creates the output of the first DKG round.
    /// This function transfers the output to the initiating user and emits
    /// an event indicating the completion of the first round.
    /// This function is called by blockchain itself.
    /// Validtors call it, it's part of the blockchain logic.
    ///
    /// ### Parameters
    /// - `initiator`: The address of the user who initiated the DKG session.
    /// - `session_id`: The ID of the current DKG session.
    /// - `output`: The raw output of the first DKG round.
    /// - `dwallet_cap_id`: The ID of the associated dWallet capability.
    #[allow(unused_function)]
    fun create_dkg_first_round_output(
        initiator: address,
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
        transfer::transfer(output, initiator);

        event::emit(CompletedFirstDKGRoundEvent {
            session_id,
            initiator,
        });
    }

    /// Starts the second DKG round.
    /// Emits an event for validators to begin the second round of the DKG process.
    ///
    /// ### Parameters
    /// - `dwallet_cap`: The capability for the associated dWallet.
    /// - `public_key_share_and_proof`: Public key share and proof from the first round.
    /// - `first_round_output`: Output from the first DKG round.
    /// - `first_round_session_id`: Session ID of the first DKG round.
    public fun launch_dkg_second_round(
        dwallet_cap: &DWalletCap,
        public_key_share_and_proof: vector<u8>,
        first_round_output: vector<u8>,
        first_round_session_id: ID,
        ctx: &mut TxContext
    ): address {
        let session_id = tx_context::fresh_object_address(ctx);
        event::emit(StartDKGSecondRoundEvent {
            session_id,
            sender: tx_context::sender(ctx),
            first_round_output,
            public_key_share_and_proof,
            dwallet_cap_id: object::id(dwallet_cap),
            first_round_session_id,
        });
        session_id
    }

    /// Completes the second DKG round and creates the final [`DWallet`].
    /// This function finalizes the DKG process and emits an event with all relevant data.
    /// This function is called by blockchain itself.
    /// Validtors call it, it's part of the blockchain logic.
    ///
    /// ### Parameters
    /// - `session_initiator`: The address of the user who initiated the DKG session.
    /// - `session_id`: The ID of the current DKG session.
    /// - `output`: The output of the second DKG round.
    /// - `dwallet_cap_id`: The ID of the associated dWallet capability.
    /// - `ctx`: The transaction context.
    #[allow(unused_function)]
    fun create_dkg_second_round_output(
        session_initiator: address,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
        let dwallet = dwallet::create_dwallet<Secp256K1>(session_id, dwallet_cap_id, output, ctx);
        event::emit(CompletedDKGSecondRoundEvent {
            session_id,
            sender: session_initiator,
            dwallet_cap_id,
            dwallet_id: object::id(&dwallet),
            value: output,
        });
        transfer::public_freeze_object(dwallet);
    }

    #[allow(unused_function)]
    #[test_only]
    /// Call the underlying `create_dkg_first_round_output`.
    /// This function is intended for testing purposes only and should not be used in production.
    /// See Move pattern: https://move-book.com/move-basics/testing.html#utilities-with-test_only
    public fun create_dkg_first_round_output_for_testing(
        sender: address,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
        ctx: &mut TxContext
    ) {
        create_dkg_first_round_output(
            sender,
            session_id,
            output,
            dwallet_cap_id,
            ctx
        );
    }

    #[allow(unused_function)]
    #[test_only]
    /// Call the underline `create_dkg_second_round_output`.
    /// This function is intended for testing purposes only and should not be used in production.
    /// See Move pattern: https://move-book.com/move-basics/testing.html#utilities-with-test_only
    public fun create_dkg_second_round_output_for_testing(
        session_initiator: address,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
        ctx: &mut TxContext
    ) {
        create_dkg_second_round_output(
            session_initiator,
            session_id,
            output,
            dwallet_cap_id,
            ctx
        );
    }
}
