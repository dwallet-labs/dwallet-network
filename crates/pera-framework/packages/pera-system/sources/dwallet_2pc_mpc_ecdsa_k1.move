// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// This module handles the logic for creating and managing dWallets using the Secp256K1 signature scheme
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
    use pera_system::dwallet::{DWallet, create_dwallet_cap, DWalletCap, get_dwallet_cap_id, get_dwallet_output};
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

    /// Object to store the output of the first round of the presign session.
    public struct PresignSessionOutput has key {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        output: vector<u8>,
    }

    /// Represents the presign result of a the second and final presign round.
    public struct Presign has key {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        presigns: vector<u8>,
    }

    /// Event emitted to start the first DKG round.
    /// Validators catch this event to initiate the first round of the DKG.
    public struct StartDKGFirstRoundEvent has copy, drop {
        session_id: address,
        initiator: address,
        dwallet_cap_id: ID,
    }

    /// Event emitted to start the second DKG round.
    /// Validators catch this event to start the second round of the DKG process.
    public struct StartDKGSecondRoundEvent has copy, drop {
        session_id: address,
        initiator: address,
        first_round_output: vector<u8>,
        public_key_share_and_proof: vector<u8>,
        dwallet_cap_id: ID,
        first_round_session_id: ID,
    }

    /// Event emitted when the second round of the DKG is completed.
    /// Contains all relevant data from the second DKG round.
    public struct CompletedDKGSecondRoundEvent has copy, drop {
        session_id: ID,
        initiator: address,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        value: vector<u8>,
    }

    /// Event emitted to initiate a `Presign` session, caught by the Validators.
    public struct StartPresignFirstRoundEvent has copy, drop {
        session_id: ID,
        initiator: address,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        dkg_output: vector<u8>,
    }

    /// Event emitted to initiate the second round of a `Presign` session, caught by the Validators.
    public struct StartPresignSecondRoundEvent has copy, drop {
        session_id: ID,
        initiator: address,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        dkg_output: vector<u8>,
        first_round_output: vector<u8>,
        first_round_session_id: ID,
    }

    /// Event emitted when the presign second round is completed.
    public struct CompletedPresignEvent has copy, drop {
        initiator: address,
        dwallet_id: ID,
        presign_id: ID,
    }

    /// Event emitted by the user to start the signing process.
    public struct StartSignEvent has copy, drop {
        session_id: ID,
        presign_session_id: ID,
        initiator: address,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        dkg_output: vector<u8>,
        hashed_message: vector<u8>,
        presign: vector<u8>,
        centralized_signed_message: vector<u8>,
    }


    /// Object representing the output of the signing process.
    public struct SignOutput has key {
        id: UID,
        session_id: ID,
        output: vector<u8>,
    }

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
        let initiator = tx_context::sender(ctx);
        let session_id = tx_context::fresh_object_address(ctx);
        event::emit(StartDKGFirstRoundEvent {
            session_id,
            initiator,
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
            initiator: tx_context::sender(ctx),
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
    /// - `initiator`: The address of the user who initiated the DKG session.
    /// - `session_id`: The ID of the current DKG session.
    /// - `output`: The output of the second DKG round.
    /// - `dwallet_cap_id`: The ID of the associated dWallet capability.
    /// - `ctx`: The transaction context.
    #[allow(unused_function)]
    fun create_dkg_second_round_output(
        initiator: address,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
        let dwallet = dwallet::create_dwallet<Secp256K1>(session_id, dwallet_cap_id, output, ctx);
        event::emit(CompletedDKGSecondRoundEvent {
            session_id,
            initiator,
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
        initiator: address,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
        ctx: &mut TxContext
    ) {
        create_dkg_first_round_output(
            initiator,
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
        initiator: address,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
        ctx: &mut TxContext
    ) {
        create_dkg_second_round_output(
            initiator,
            session_id,
            output,
            dwallet_cap_id,
            ctx
        );
    }

    /// Starts the first round of the presign session for a specified dWallet.
    ///
    /// This function emits a `StartPresignFirstRoundEvent`, which is caught by validators
    /// to initiate the presign process.
    ///
    /// ### Parameters
    /// - `dwallet`: A reference to the [`DWallet`] object of type `Secp256K1` for
    ///    which the presign session is being initiated.
    /// - `ctx`: The mutable transaction context used to create and emit the event.
    ///
    /// ### Emits
    /// - `StartPresignFirstRoundEvent`: Includes session ID, initiator address, dWallet ID,
    ///    dWallet capability ID, and DKG output.
    public fun launch_presign_first_round(
        dwallet: &DWallet<Secp256K1>,
        ctx: &mut TxContext
    ) {
        let session_id = tx_context::fresh_object_address(ctx);
        event::emit(StartPresignFirstRoundEvent {
            session_id: object::id_from_address(session_id),
            initiator: tx_context::sender(ctx),
            dwallet_id: object::id(dwallet),
            dwallet_cap_id: get_dwallet_cap_id<Secp256K1>(dwallet),
            dkg_output: get_dwallet_output<Secp256K1>(dwallet),
        });
    }

    /// Creates the first presign round output and initiates the
    /// second presign round.
    ///
    /// This function is called by validators as part of the blockchain logic.
    /// It creates the output for the first presign round, transfers it to the initiating user,
    /// and then emits an event to launch the second presign round.
    ///
    /// ### Parameters
    /// - `initiator`: The address of the user who initiated the session.
    /// - `session_id`: The session ID of the current presign session.
    /// - `first_round_output`: The output data generated in the first round of the presign session.
    /// - `dwallet_cap_id`: The ID of the associated `DWalletCap` for the dWallet.
    /// - `dwallet_id`: The ID of the associated dWallet.
    /// - `dkg_output`: The output data from the DKG process.
    /// - `ctx`: The transaction context used to handle state updates and object creation.
    ///
    /// ### Effects
    /// - Creates a `PresignSessionOutput` object and transfers it to the `initiator`.
    /// - Emits a `StartPresignSecondRoundEvent` to indicate the beginning of the second presign round.
    /// - Asserts that the caller is the system address for security purposes.
    ///
    /// ### Panics
    /// - Panics with `ENotSystemAddress` if the sender of the transaction is not the system address.
    #[allow(unused_function)]
    fun create_first_presign_round_output_and_launch_second_round(
        initiator: address,
        session_id: ID,
        first_round_output: vector<u8>,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        dkg_output: vector<u8>,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);

        let output = PresignSessionOutput {
            id: object::new(ctx),
            session_id,
            dwallet_id,
            dwallet_cap_id,
            output: first_round_output,
        };
        transfer::transfer(output, initiator);
        launch_presign_second_round(
            initiator,
            dwallet_id,
            dkg_output,
            dwallet_cap_id,
            first_round_output,
            session_id,
            ctx
        );
    }

    /// Launches the second round of the presign session.
    ///
    /// This function emits a `StartPresignSecondRoundEvent`, which is caught by validators
    /// to begin the second round of the presign process.
    ///
    /// ### Parameters
    /// - `initiator`: The address of the user initiating the presign session.
    /// - `dwallet_id`: The ID of the associated dWallet.
    /// - `dkg_output`: The output from the DKG process.
    /// - `dwallet_cap_id`: The ID of the associated `DWalletCap`.
    /// - `first_round_output`: The output from the first round of the presign process.
    /// - `first_round_session_id`: The session ID of the first presign round.
    /// - `ctx`: The transaction context used to emit the event.
    ///
    /// ### Panics
    /// - Panics with `ENotSystemAddress` if the sender of the transaction is not the system address.
    ///
    /// ### Emits
    /// - `StartPresignSecondRoundEvent`: Includes session ID, initiator address, dWallet ID, dWallet capability ID,
    ///   DKG output, first round output, and first round session ID.
    fun launch_presign_second_round(
        initiator: address,
        dwallet_id: ID,
        dkg_output: vector<u8>,
        dwallet_cap_id: ID,
        first_round_output: vector<u8>,
        first_round_session_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);

        let session_id = object::id_from_address(tx_context::fresh_object_address(ctx));

        event::emit(StartPresignSecondRoundEvent {
            session_id,
            initiator,
            dwallet_id,
            dwallet_cap_id,
            dkg_output,
            first_round_output,
            first_round_session_id,
        });
    }


    #[test_only]
    /// Call the underlying `create_first_presign_round_output_and_launch_second_round`.
    /// This function is intended for testing purposes only and should not be used in production.
    /// See Move pattern: https://move-book.com/move-basics/testing.html#utilities-with-test_only
    public fun create_first_presign_round_output_and_launch_second_round_for_testing(
        initiator: address,
        session_id: ID,
        first_round_output: vector<u8>,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        dkg_output: vector<u8>,
        ctx: &mut TxContext
    ) {
        create_first_presign_round_output_and_launch_second_round(
            initiator,
            session_id,
            first_round_output,
            dwallet_cap_id,
            dwallet_id,
            dkg_output,
            ctx
        );
    }

    #[test_only]
    /// Call the underlying `launch_presign_second_round`.
    /// This function is intended for testing purposes only and should not be used in production.
    /// See Move pattern: https://move-book.com/move-basics/testing.html#utilities-with-test_only
    public fun launch_presign_second_round_for_testing(
        initiator: address,
        dwallet_id: ID,
        dkg_output: vector<u8>,
        dwallet_cap_id: ID,
        first_round_output: vector<u8>,
        first_round_session_id: ID,
        ctx: &mut TxContext
    ) {
        launch_presign_second_round(
            initiator,
            dwallet_id,
            dkg_output,
            dwallet_cap_id,
            first_round_output,
            first_round_session_id,
            ctx
        );
    }

    /// Completes the presign session by creating the output of the
    /// second presign round and transferring it to the session initiator.
    ///
    /// This function is called by validators as part of the blockchain logic.
    /// It creates a `Presign` object representing the second presign round output,
    /// emits a `CompletedPresignEvent`, and transfers the result to the initiating user.
    ///
    /// ### Parameters
    /// - `initiator`: The address of the user who initiated the presign session.
    /// - `session_id`: The ID of the presign session.
    /// - `output`: The presign result data.
    /// - `dwallet_cap_id`: The ID of the associated `DWalletCap`.
    /// - `dwallet_id`: The ID of the associated `DWallet`.
    /// - `ctx`: The transaction context.
    ///
    /// ### Emits
    /// - `CompletedPresignEvent`: Includes the initiator, dWallet ID, and presign ID.
    ///
    /// ### Panics
    /// - Panics with `ENotSystemAddress` if the sender of the transaction is not the system address.
    ///
    /// ### Effects
    /// - Creates a `Presign` object and transfers it to the session initiator.
    /// - Emits a `CompletedPresignEvent`.
    #[allow(unused_function)]
    fun create_second_presign_round_output(
        initiator: address,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);

        let output = Presign {
            id: object::new(ctx),
            session_id,
            dwallet_id,
            dwallet_cap_id,
            presigns: output,
        };

        event::emit(CompletedPresignEvent {
            initiator,
            dwallet_id,
            presign_id: object::id(&output),
        });
        transfer::transfer(output, initiator);
    }

    #[test_only]
    /// Call the underlying `create_second_presign_round_output`.
    /// This function is intended for testing purposes only and should not be used in production.
    /// See Move pattern: https://move-book.com/move-basics/testing.html#utilities-with-test_only
    public fun create_second_presign_round_output_for_testing(
        initiator: address,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        ctx: &mut TxContext
    ) {
        create_second_presign_round_output(
            initiator,
            session_id,
            output,
            dwallet_cap_id,
            dwallet_id,
            ctx
        );
    }

    /// Starts the signing process by emitting a `StartSignEvent`.
    ///
    /// ### Parameters
    /// - `hashed_message`: The hashed message to be signed.
    /// - `presign`: The presign output from the presign round.
    /// - `dkg_output`: The output from the DKG process.
    /// - `centralized_signed_message`: A message signed in a centralized manner (optional in hybrid modes).
    /// - `presign_session_id`: The session ID of the presign session.
    /// - `ctx`: The transaction context.
    public fun sign(
        hashed_message: vector<u8>,
        presign: vector<u8>,
        dkg_output: vector<u8>,
        centralized_signed_message: vector<u8>,
        presign_session_id: ID,
        ctx: &mut TxContext
    ) {
        let id = object::id_from_address(tx_context::fresh_object_address(ctx));
        event::emit(StartSignEvent {
            session_id: id,
            presign_session_id,
            initiator: tx_context::sender(ctx),
            dwallet_id: id,
            dwallet_cap_id: id,
            presign,
            centralized_signed_message,
            dkg_output,
            hashed_message,
        });
    }

    /// Creates the output of the signing process and transfers it to the initiating user.
    /// This function is called by blockchain itself.
    /// Validtors call it, it's part of the blockchain logic.
    ///
    /// ### Parameters
    /// - `initiator`: The address of the user who initiated the signing process.
    /// - `session_id`: The session ID of the signing process.
    /// - `output`: The signing output data.
    /// - `ctx`: The transaction context.
    #[allow(unused_function)]
    fun create_sign_output(
        initiator: address,
        session_id: ID,
        output: vector<u8>,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
        let output = SignOutput {
            id: object::new(ctx),
            session_id,
            output,
        };
        transfer::transfer(output, initiator);
    }

    #[test_only]
    /// Call the underlying `create_sign_output`.
    /// This function is intended for testing purposes only and should not be used in production.
    /// See Move pattern: https://move-book.com/move-basics/testing.html#utilities-with-test_only
    public fun create_sign_output_for_testing(
        initiator: address,
        session_id: ID,
        output: vector<u8>,
        ctx: &mut TxContext
    ) {
        create_sign_output(initiator, session_id, output, ctx);
    }

}
