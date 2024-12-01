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

    /// Represents the presign result of a the second and final presign round.
    public struct Presign has key, store {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        first_round_session_id: ID,
        first_round_output: vector<u8>,
        second_round_output: vector<u8>,
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
        presign_first_round_output: vector<u8>,
        presign_second_round_output: vector<u8>,
        centralized_signed_message: vector<u8>,
    }


    /// Object representing the output of the signing process.
    public struct SignOutput has key {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        output: vector<u8>,
    }

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    /// Error raised when the sender is not the system address.
    const ENotSystemAddress: u64 = 0;
    const EDwalletCapMismatch: u64 = 1;
    const EDwalletMismatch: u64 = 2;
    // >>>>>>>>>>>>>>>>>>>>>>>> Error codes >>>>>>>>>>>>>>>>>>>>>>>>

    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<
    /// System address for asserting system-level actions.
    const SYSTEM_ADDRESS: address = @0x0;
    // >>>>>>>>>>>>>>>>>>>>>>>> Constants >>>>>>>>>>>>>>>>>>>>>>>>

    /// Starts the first Distributed Key Generation (DKG) session.
    ///
    /// This function creates a new `DWalletCap` object,
    /// transfers it to the session initiator,
    /// and emits a `StartDKGFirstRoundEvent` to signal
    /// the beginning of the DKG process.
    ///
    /// ### Effects
    /// - Generates a new `DWalletCap` object.
    /// - Transfers the `DWalletCap` to the session initiator (`ctx.sender`).
    /// - Emits a `StartDKGFirstRoundEvent`.
    ///
    /// ### Emits
    /// - `StartDKGFirstRoundEvent`:
    ///   - `session_id`: The generated session ID.
    ///   - `initiator`: The address of the transaction sender.
    ///   - `dwallet_cap_id`: The ID of the created `DWalletCap`.
    public fun launch_dkg_first_round(ctx: &mut TxContext) {
        let dwallet_cap = create_dwallet_cap(ctx);
        let dwallet_cap_id = object::id(&dwallet_cap);
        transfer::public_transfer(dwallet_cap, tx_context::sender(ctx));
        let initiator = tx_context::sender(ctx);
        let session_id = tx_context::fresh_object_address(ctx);
        event::emit(StartDKGFirstRoundEvent {
            session_id,
            initiator,
            dwallet_cap_id,
        });
    }

    /// Creates the output of the first DKG round.
    ///
    /// This function transfers the output of the first DKG round
    /// to the session initiator and ensures it is securely linked
    /// to the `DWalletCap` of the session.
    /// This function is called by blockchain itself.
    /// Validtors call it, it's part of the blockchain logic.
    ///
    /// ### Effects
    /// - Transfers the output of the first round to the initiator.
    /// - Emits necessary metadata and links it to the associated session.
    ///
    /// ### Parameters
    /// - `initiator`: The address of the user who initiated the DKG session.
    /// - `session_id`: The ID of the DKG session.
    /// - `output`: The output data from the first round.
    /// - `dwallet_cap_id`: The ID of the associated `DWalletCap`.
    /// - `ctx`: The transaction context.
    ///
    /// ### Panics
    /// - Panics with `ENotSystemAddress` if the sender is not the system address.
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
    /// - `output`: The decentrelaized output of the second DKG round.
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
    /// This function emits a `StartPresignFirstRoundEvent`, which signals validators
    /// to begin processing the first round of the presign process.
    ///
    /// ### Effects
    /// - Links the presign session to the specified dWallet.
    /// - Emits a `StartPresignFirstRoundEvent` with relevant details.
    ///
    /// ### Emits
    /// - `StartPresignFirstRoundEvent`:
    ///   - `session_id`: The unique ID of the presign session.
    ///   - `initiator`: The address of the session initiator.
    ///   - `dwallet_id`: The ID of the linked dWallet.
    ///   - `dwallet_cap_id`: The capability ID of the linked dWallet.
    ///   - `dkg_output`: The DKG process output linked to this dWallet.
    ///
    /// ### Parameters
    /// - `dwallet`: A reference to the target dWallet.
    /// - `ctx`: The mutable transaction context.
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
    #[allow(unused_function)]
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
        first_round_session_id: ID,
        first_round_output: vector<u8>,
        second_round_output: vector<u8>,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);

        let output = Presign {
            id: object::new(ctx),
            session_id,
            first_round_session_id,
            dwallet_id,
            dwallet_cap_id,
            first_round_output,
            second_round_output,
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
        first_round_session_id: ID,
        first_round_output: vector<u8>,
        second_round_output: vector<u8>,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        ctx: &mut TxContext
    ) {
        create_second_presign_round_output(
            initiator,
            session_id,
            first_round_session_id,
            first_round_output,
            second_round_output,
            dwallet_cap_id,
            dwallet_id,
            ctx
        );
    }

    /// Initiates the signing process for a given dWallet.
    ///
    /// This function emits a `StartSignEvent`, providing all necessary
    /// metadata and ensuring the integrity of the signing process.
    /// It validates the linkage between the `DWallet`, `DWalletCap`, and `Presign`.
    ///
    /// ### Effects
    /// - Validates the linkage between dWallet components.
    /// - Emits a `StartSignEvent` with the hashed message, presign outputs,
    ///   and additional metadata.
    ///
    /// ### Emits
    /// - `StartSignEvent`:
    ///   - Includes session details, hashed message, presign outputs,
    ///     and DKG output.
    ///
    /// ### Parameters
    /// - `dwallet_cap`: The capability associated with the dWallet.
    /// - `hashed_message`: The message to be signed (already hashed).
    /// - `dwallet`: The dWallet object.
    /// - `presign`: The presign object containing intermediate outputs.
    /// - `centralized_signed_message`: Optionally includes a centralized signature.
    /// - `presign_session_id`: The session ID of the presign process.
    /// - `ctx`: The mutable transaction context.
    public fun sign(
        dwallet_cap: &DWalletCap,
        hashed_message: vector<u8>,
        dwallet: &DWallet<Secp256K1>,
        presign: &Presign,
        centralized_signed_message: vector<u8>,
        presign_session_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(object::id(dwallet_cap) == get_dwallet_cap_id<Secp256K1>(dwallet), EDwalletCapMismatch);
        assert!(object::id(dwallet) == presign.dwallet_id, EDwalletMismatch);

        let id = object::id_from_address(tx_context::fresh_object_address(ctx));
        event::emit(StartSignEvent {
            session_id: id,
            presign_session_id,
            initiator: tx_context::sender(ctx),
            dwallet_id: object::id(dwallet),
            dwallet_cap_id: object::id(dwallet_cap),
            presign_first_round_output: presign.first_round_output,
            presign_second_round_output: presign.second_round_output,
            centralized_signed_message,
            dkg_output: get_dwallet_output<Secp256K1>(dwallet),
            hashed_message
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
        dwallet_id: ID,
        initiator: address,
        session_id: ID,
        output: vector<u8>,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
        let output = SignOutput {
            id: object::new(ctx),
            session_id,
            dwallet_id,
            output,
        };
        transfer::transfer(output, initiator);
    }

    #[test_only]
    /// Call the underlying `create_sign_output`.
    /// This function is intended for testing purposes only and should not be used in production.
    /// See Move pattern: https://move-book.com/move-basics/testing.html#utilities-with-test_only
    public fun create_sign_output_for_testing(
        dwallet_id: ID,
        initiator: address,
        session_id: ID,
        output: vector<u8>,
        ctx: &mut TxContext
    ) {
        create_sign_output(
            dwallet_id,
            initiator,
            session_id,
            output,
            ctx
        );
    }

    /// Generates a mock `DWallet<Secp256K1>` object for testing purposes.
    ///
    /// This function creates a dWallet object with random data,
    /// useful for testing or initialization in non-production environments.
    ///
    /// ### Parameters
    /// - `ctx`: The transaction context for generating IDs and objects.
    /// - `dwallet_cap_id`: The ID of the capability associated with the mock dWallet.
    /// - `dkg_output`: The decentralized DKG output.
    ///
    /// ### Effects
    /// - Creates and initializes a `DWallet<Secp256K1>` object.
    /// - Links the dWallet to the provided capability.
    ///
    /// ### Returns
    /// - `DWallet<Secp256K1>`: A mock dWallet object.
    public fun create_mock_dwallet(
        dkg_output: vector<u8>,
        ctx: &mut TxContext
    ): DWallet<Secp256K1> {
        let dwallet_cap = create_dwallet_cap(ctx);
        let dwallet_cap_id = object::id(&dwallet_cap);
        transfer::public_transfer(dwallet_cap, tx_context::sender(ctx));
        let session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        dwallet::create_dwallet<Secp256K1>(session_id, dwallet_cap_id, dkg_output, ctx)
    }

    /// Generates a new mock `Presign` object with random IDs and data.
    /// This function is useful for testing or initializing Presign objects.
    public fun create_mock_presign(
        dwallet_id: ID,
        dwallet_cap_id: ID,
        first_round_output: vector<u8>,
        second_round_output: vector<u8>,
        first_round_session_id: ID,
        ctx: &mut TxContext,
    ): Presign {
        let id = object::new(ctx);
        let session_id = object::id_from_address(tx_context::fresh_object_address(ctx));

        // Create and return the Presign object.
        Presign {
            id,
            session_id,
            dwallet_id,
            dwallet_cap_id,
            first_round_session_id,
            first_round_output,
            second_round_output,
        }
    }
}
