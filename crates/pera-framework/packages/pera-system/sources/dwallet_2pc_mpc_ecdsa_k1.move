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
#[allow(unused_function, unused_use, unused_field, unused_const, unused_mut_parameter, unused_variable)]
module pera_system::dwallet_2pc_mpc_ecdsa_k1 {
    use pera_system::dwallet;
    use pera_system::dwallet::{DWallet, create_dwallet_cap, DWalletCap, get_dwallet_cap_id, get_dwallet_output};
    use pera::event;
    #[test_only]
    use pera::object;

    /// Represents the `Secp256K1` dWallet type.
    ///
    /// This struct serves as a marker to identify and signify
    /// the dWallet cryptographic scheme used for ECDSA
    ///(Elliptic Curve Digital Signature Algorithm)
    /// based on the `Secp256K1` curve.
    public struct Secp256K1 has drop {}

    /// Represents a message that was approved as part of a dWallet process.
    ///
    /// This struct binds the message to a specific `DWalletCap` for
    /// traceability and accountability within the system.
    ///
    /// ### Fields
    /// - **`dwallet_cap_id`**: The identifier of the DWallet capability
    ///   associated with this approval.
    /// - **`message`**: The message that has been approved.
    public struct MessageApproval has store, drop {
        dwallet_cap_id: ID,
        message: vector<u8>,
    }

    /// A struct to hold the output of the first round of the DKG.
    /// An instance of this struct is being transferred to the user that initiated the DKG after
    /// the first round is completed.
    /// The user can then use this output to start the second round of the DKG.
    public struct DKGFirstRoundOutputEvent has copy, drop {
        session_id: ID,
        output: vector<u8>,
    }

    /// Represents the result of the second and final
    /// presign round.
    ///
    /// This struct holds information on both rounds of the presign process,
    /// linking them to the corresponding DWallet session.
    ///
    /// ### Fields
    /// - **`id`**: Unique identifier for this presign object.
    /// - **`session_id`**: The session ID of this presign process.
    /// - **`dwallet_id`**: The DWallet identifier associated with the presign.
    /// - **`dwallet_cap_id`**: The DWallet capability identifier for this presign.
    /// - **`first_round_session_id`**: The session ID for the first round of the presign.
    /// - **`first_round_output`**: The output from the first round of the presign.
    /// - **`second_round_output`**: The output from the second round of the presign.
    public struct Presign has key, store {
        id: UID,
        dwallet_id: ID,
        first_round_session_id: ID,
        presign: vector<u8>,
    }

    /// Messages that has been signed by a user, a.k.a the centralized party, but not yet by the blockchain.
    /// Used for scenarios where the user need to first agree to sign some transaction, and the blockchain signs this transaction only later,
    /// when some other conditions are met.
    ///
    /// Can be used to implement an order-book based exchange, for example.
    /// User A first agrees to buy BTC with ETH at price X, and signs a transaction with this information.
    /// When a matching user B, that agrees to sell BTC for ETH at price X, signs a transaction with this information,
    /// the blockchain can sign both transactions, and the exchange is completed.
    public struct PartiallySignedMessages has key {
        id: UID,
        /// The presigns bytes for each message.
        /// The matching presign objects are being "burned" before this object is created.
        presigns: vector<vector<u8>>,
        /// The presigns session IDs.
        presign_session_ids: vector<ID>,
        /// The hashed messages that are being signed.
        messages: vector<vector<u8>>,
        /// The user centralized signatures for each message.
        signatures: vector<vector<u8>>,
        dwallet_id: ID,
        /// The DKG output of the DWallet.
        dwallet_output: vector<u8>,
        dwallet_cap_id: ID,
    }

    /// Event emitted when a [`PartiallySignedMessages`] object is created.
    public struct CreatedPartiallySignedMessagesEvent has copy, drop {
        /// The object's ID.
        partial_signatures_object_id: ID,
    }

    /// Event emitted to start the first DKG round.
    ///
    /// This event is caught by Validators, who use it to initiate the first round of the DKG.
    ///
    /// ### Fields
    /// - **`session_id`**: The unique session identifier for the DKG process.
    /// - **`initiator`**: The address of the user who initiated the DKG process.
    /// - **`dwallet_cap_id`**: The identifier for the DWallet capability.
    public struct StartDKGFirstRoundEvent has copy, drop {
        session_id: address,
        initiator: address,
        dwallet_cap_id: ID,
    }

    /// Event emitted to signal the completion of a Sign process.
    ///
    /// This event contains signatures for all signed messages in the batch.
    ///
    /// ### Fields
    /// - **`session_id`**: The session identifier for the signing process.
    /// - **`signed_messages`**: A collection of signed messages.
    public struct CompletedSignEvent has copy, drop {
        session_id: ID,
        signed_messages: vector<vector<u8>>,
    }

    /// Event emitted to start the second round of the DKG process.
    ///
    /// This event is caught by Validators to start the second round of DKG.
    ///
    /// ### Fields
    /// - **`session_id`**: The session identifier.
    /// - **`initiator`**: The address of the user who initiated the event.
    /// - **`first_round_output`**: The output from the first round of the DKG.
    /// - **`public_key_share_and_proof`**: The public key share and its proof.
    /// - **`dwallet_cap_id`**: The DWallet capability identifier.
    public struct StartDKGSecondRoundEvent has copy, drop {
        session_id: address,
        initiator: address,
        first_round_output: vector<u8>,
        public_key_share_and_proof: vector<u8>,
        dwallet_cap_id: ID,
        first_round_session_id: ID,
    }

    /// Event emitted when the second round of the
    /// Distributed Key Generation (DKG) is completed.
    ///
    /// This event contains all relevant data produced from the
    /// second round of the DKG process.
    /// Validators and users utilize this event to
    /// finalize and store the results of the DKG.
    ///
    /// ### Fields
    /// - **`session_id`**: The unique identifier for the DKG session.
    /// - **`initiator`**: The address of the user who initiated the DKG process.
    /// - **`dwallet_cap_id`**: The identifier of the DWallet capability used in the DKG process.
    /// - **`dwallet_id`**: The identifier of the DWallet created as a result of the DKG process.
    /// - **`value`**: The value produced by the second round of the DKG, typically representing
    ///   the combined and validated output from all participating parties.
    public struct CompletedDKGSecondRoundEvent has copy, drop {
        session_id: ID,
        initiator: address,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        value: vector<u8>,
    }

    /// Event emitted to initiate the first round of a Presign session.
    ///
    /// ### Fields
    /// - **`session_id`**: The session identifier.
    /// - **`initiator`**: The address of the user who initiated the event.
    /// - **`dwallet_id`**: The DWallet identifier.
    /// - **`dwallet_cap_id`**: The DWallet capability identifier.
    /// - **`dkg_output`**: The output from the DKG process.
    public struct StartPresignFirstRoundEvent has copy, drop {
        session_id: ID,
        initiator: address,
        dwallet_id: ID,
        dkg_output: vector<u8>,
        batch_session_id: ID,
    }

    /// Event emitted to initiate the second round of a `Presign` session.
    ///
    /// This event is caught by Validators to initiate the second round of the Presign process.
    /// The second round is a crucial step in the multi-party computation (MPC) protocol
    /// to generate pre-signatures for ECDSA signing.
    ///
    /// ### Fields
    /// - **`session_id`**: The unique identifier for the current presign session.
    /// - **`initiator`**: The address of the user who initiated the presign session.
    /// - **`dwallet_id`**: The identifier of the DWallet associated with this presign session.
    /// - **`dwallet_cap_id`**: The identifier of the DWallet capability used in this session.
    /// - **`dkg_output`**: The output produced from the Distributed Key Generation (DKG) process.
    /// - **`first_round_output`**: The output of the first round of the presign session.
    /// - **`first_round_session_id`**: The session identifier for the first round of the presign.
    public struct StartPresignSecondRoundEvent has copy, drop {
        session_id: ID,
        initiator: address,
        dwallet_id: ID,
        dkg_output: vector<u8>,
        first_round_output: vector<u8>,
        first_round_session_id: ID,
        batch_session_id: ID,
    }

    /// Event emitted when the presign batch is completed.
    public struct CompletedBatchedPresignEvent has copy, drop {
        /// The address of the user who initiated the batch.
        initiator: address,
        dwallet_id: ID,
        /// Tha batch session ID.
        session_id: ID,
        /// The ID of all the presign objects created in this batch.
        /// Each presign can be used to sign only one message.
        presign_ids: vector<ID>,
        /// The first round session IDs for each presign.
        /// The order of the session IDs corresponds to the order of the presigns.
        /// The first round session ID is needed for the centralized sign process.
        first_round_session_ids: vector<ID>,
        /// The serialized presign objects created in this batch.
        /// The order of the presigns corresponds to the order of the presign IDs.
        presigns: vector<vector<u8>>,
    }

    /// Event emitted to start the signing process.
    /// The event is caught by the validators to initiate the signing protocol.
    ///
    /// ### Fields
    /// - **`session_id`**: The unique identifier for this sign session.
    /// - **`presign_session_id`**: The unique identifier for the associated presign session.
    /// - **`initiator`**: The address of the user who initiated the sign event.
    /// - **`batched_session_id`**: The session identifier for the batched sign process.
    /// - **`dwallet_id`**: The unique identifier for the DWallet used in the session.
    /// - **`dwallet_cap_id`**: The identifier for the DWallet's capability.
    /// - **`dkg_output`**: The output of the DKG process used for the session.
    /// - **`hashed_message`**: The hashed message that will be signed during this session.
    /// - **`presign_first_round_output`**: The output from the first round of the presign process.
    /// - **`presign_second_round_output`**: The output from the second round of the presign process.
    /// - **`centralized_signed_message`**: The final signed message produced by the centralized sign process.
    public struct StartSignEvent has copy, drop {
        session_id: ID,
        presign_session_id: ID,
        initiator: address,
        batched_session_id: ID,
        dwallet_id: ID,
        dkg_output: vector<u8>,
        hashed_message: vector<u8>,
        presign: vector<u8>,
        centralized_signed_message: vector<u8>,
    }

    /// Event emitted to start a batched sign process.
    ///
    /// ### Fields
    /// - **`session_id`**: The session identifier for the batched sign process.
    /// - **`hashed_messages`**: A list of hashed messages to be signed.
    /// - **`initiator`**: The address of the user who initiated the protocol.
    public struct StartBatchedSignEvent has copy, drop {
        session_id: ID,
        hashed_messages: vector<vector<u8>>,
        initiator: address
    }

    /// Event emitted to start a batched presign flow, i.e. a flow that creates multiple presigns at once.
    ///
    /// ### Fields
    /// - **`session_id`**: The session identifier for the batched sign process.
    /// - **`batch_size`**: The number of presign sessions to be started.
    public struct StartBatchedPresignEvent has copy, drop {
        session_id: ID,
        batch_size: u64,
        initiator: address
    }

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    /// Error raised when the sender is not the system address.
    const ENotSystemAddress: u64 = 0;
    const EMesssageApprovalDWalletMismatch: u64 = 1;
    const EDwalletMismatch: u64 = 2;
    const EApprovalsAndMessagesLenMismatch: u64 = 3;
    const EMissingApprovalOrWorngApprovalOrder: u64 = 4;
    const ECentrailizedsignedMessagesAndMessagesLenMismatch: u64 = 5;
    const EPresignsAndMessagesLenMismatch: u64 = 6;
    const EInvalidSignatures: u64 = 7;
    const EApprovalsAndSignaturesLenMismatch: u64 = 8;
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
    /// Creates the output of the first round of the DKG MPC, transferring it to the initiating user.
    /// This function is called by the blockchain itself.
    /// Validators call it as part of the blockchain logic.
    fun create_dkg_first_round_output(
        session_id: ID,
        output: vector<u8>,
        ctx: &TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
        event::emit(DKGFirstRoundOutputEvent {
            session_id,
            output,
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
        dwallet_mpc_network_key_version: u8,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
        let dwallet = dwallet::create_dwallet<Secp256K1>(
            session_id,
            dwallet_cap_id,
            output,
            dwallet_mpc_network_key_version,
            ctx
        );
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
        session_id: ID,
        output: vector<u8>,
        ctx: &mut TxContext
    ) {
        create_dkg_first_round_output(
            session_id,
            output,
            ctx
        );
    }

    #[test_only]
    public fun partial_signatures_for_testing(
        presigns: vector<vector<u8>>,
        presign_session_ids: vector<ID>,
        messages: vector<vector<u8>>,
        signatures: vector<vector<u8>>,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        ctx: &mut TxContext
    ) : PartiallySignedMessages {
        PartiallySignedMessages {
            id: object::new(ctx),
            presigns,
            presign_session_ids,
            messages,
            signatures,
            dwallet_id,
            dwallet_output: vector::empty(),
            dwallet_cap_id,
        }
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
            0,
            ctx
        );
    }

    /// Starts a batched presign session.
    ///
    /// This function emits a `StartPresignFirstRoundEvent` for each presign in the batch,
    /// which signals validators to begin processing the first round of the presign process.
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
    ///   - `dkg_output`: The DKG process output linked to this dWallet.
    ///
    /// ### Parameters
    /// - `dwallet`: A reference to the target dWallet.
    /// - `batch_size`: The number of presign sessions to be created in this batch.
    /// - `ctx`: The mutable transaction context.
    public fun launch_batched_presign(
        dwallet: &DWallet<Secp256K1>,
        batch_size: u64,
        ctx: &mut TxContext
    ) {
        let batch_session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        event::emit(StartBatchedPresignEvent {
            session_id: batch_session_id,
            batch_size,
            initiator: tx_context::sender(ctx)
        });
        let mut i = 0;
        while (i < batch_size) {
            let session_id = tx_context::fresh_object_address(ctx);
            i = i + 1;
            event::emit(StartPresignFirstRoundEvent {
                session_id: object::id_from_address(session_id),
                initiator: tx_context::sender(ctx),
                dwallet_id: object::id(dwallet),
                dkg_output: get_dwallet_output<Secp256K1>(dwallet),
                batch_session_id,
            });
        };
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
        first_round_output: vector<u8>,
        first_round_session_id: ID,
        batch_session_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);

        let session_id = object::id_from_address(tx_context::fresh_object_address(ctx));

        event::emit(StartPresignSecondRoundEvent {
            session_id,
            initiator,
            dwallet_id,
            dkg_output,
            first_round_output,
            first_round_session_id,
            batch_session_id,
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
        first_round_output: vector<u8>,
        first_round_session_id: ID,
        batch_session_id: ID,
        ctx: &mut TxContext
    ) {
        launch_presign_second_round(
            initiator,
            dwallet_id,
            dkg_output,
            first_round_output,
            first_round_session_id,
            batch_session_id,
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
    fun create_batched_presign_output(
        initiator: address,
        batch_session_id: ID,
        first_round_session_ids: vector<ID>,
        presigns: vector<vector<u8>>,
        dwallet_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
        let mut i: u64 = 0;
        let mut batch_presigns_ids: vector<ID> = vector::empty();
        let first_round_session_ids_len = vector::length(&first_round_session_ids);
        while (i < first_round_session_ids_len) {
            let first_round_session_id = first_round_session_ids[i];
            let presign = presigns[i];
            let output = Presign {
                id: object::new(ctx),
                first_round_session_id,
                dwallet_id,
                presign,
            };
            batch_presigns_ids.push_back(object::id(&output));
            transfer::transfer(output, initiator);
            i = i + 1;
        };

        event::emit(CompletedBatchedPresignEvent {
            initiator,
            dwallet_id,
            session_id: batch_session_id,
            presign_ids: batch_presigns_ids,
            presigns,
            first_round_session_ids,
        });
    }

    /// Create a set of message approvals.
    /// The messages must be approved in the same order as they were created.
    /// The messages must be approved by the same `dwallet_cap_id`.
    public fun approve_messages(
        dwallet_cap: &DWalletCap,
        messages: &mut vector<vector<u8>>
    ): vector<MessageApproval> {
        let dwallet_cap_id = object::id(dwallet_cap);
        let mut message_approvals = vector::empty<MessageApproval>();

        // Approve all messages and maintain their order.
        let messages_length = vector::length(messages);
        let mut i: u64 = 0;
        while (i < messages_length) {
            let message = vector::pop_back(messages);
            vector::push_back(&mut message_approvals, MessageApproval {
                dwallet_cap_id,
                message,
            });
            i = i + 1;
        };
        vector::reverse(&mut message_approvals);
        message_approvals
    }

    /// Remove a `MessageApproval` and return the `dwallet_cap_id`
    /// and the `message`.
    public fun remove_message_approval(message_approval: MessageApproval): (ID, vector<u8>) {
        let MessageApproval {
            dwallet_cap_id,
            message
        } = message_approval;
        (dwallet_cap_id, message)
    }

    #[test_only]
    /// Call the underlying `create_batched_presign_output`.
    /// This function is intended for testing purposes only and should not be used in production.
    /// See Move pattern: https://move-book.com/move-basics/testing.html#utilities-with-test_only
    public fun create_batched_presign_output_for_testing(
        initiator: address,
        session_id: ID,
        first_round_session_id: ID,
        presign: vector<u8>,
        dwallet_id: ID,
        ctx: &mut TxContext
    ) {
        create_batched_presign_output(
            initiator,
            session_id,
            vector[first_round_session_id],
            vector[presign],
            dwallet_id,
            ctx
        );
    }

    /// Initiates the signing process for a given dWallet.
    ///
    /// This function emits a `StartSignEvent`, providing all necessary
    /// metadata and ensuring the integrity of the signing process.
    /// It validates the linkage between the `DWallet`, `DWalletCap`, and `Presign`.
    /// It also "burns" the [`Presign`] object, by sending it to the system address,
    /// as every presign can only be used to sign only one message.
    ///
    /// ### Effects
    /// - Validates the linkage between dWallet components.
    /// - Verifies that the number of `hashed_messages`, `message_approvals`, and
    ///   `centralized_signed_messages` are equal.
    /// - Emits a `StartSignEvent` with the hashed message, presign outputs,
    ///   and additional metadata.
    ///
    /// ### Emits
    /// - `StartBatchedSignEvent`:
    ///   - Contains the session details and the list of hashed messages.
    /// - `StartSignEvent`:
    ///   - Includes session details, hashed message, presign outputs,
    ///     and DKG output.
    ///
    /// ### Aborts
    /// - **`EDwalletMismatch`**: If the `dwallet` object does not match the ID
    ///   in the `Presign` object.
    /// - **`EApprovalsAndMessagesLenMismatch`**: If the length of the `hashed_messages`
    ///   does not match the length of the `message_approvals`.
    /// - **`ECentrailizedsignedMessagesAndMessagesLenMismatch`**: If the length of
    ///   `hashed_messages` does not match the length of `centralized_signed_messages`.
    /// - **`EMesssageApprovalDWalletMismatch`**: If the DWalletCap ID does not match
    ///   the expected DWalletCap ID for any of the message approvals.
    /// - **`EMissingApprovalOrWorngApprovalOrder`**: If the approved messages are not
    ///   in the same order as the `hashed_messages`.
    ///
    /// ### Parameters
    /// - `dwallet_cap`: The capability associated with the dWallet.
    /// - `hashed_messages`: The list of hashed messages to be signed.
    /// - `message_approvals`: The approvals for the messages.
    /// - `presign`: The presign object containing intermediate outputs.
    /// - `dwallet`: The dWallet object.
    /// - `centralized_signed_messages`: The list of centralized signatures.
    /// - `presign_session_id`: The session ID of the presign process.
    /// - `ctx`: The mutable transaction context.
    public fun sign(
        message_approvals: &mut vector<MessageApproval>,
        mut hashed_messages: vector<vector<u8>>,
        mut presigns: vector<Presign>,
        dwallet: &DWallet<Secp256K1>,
        mut centralized_signed_messages: vector<vector<u8>>,
        ctx: &mut TxContext
    ) {
        let messages_len: u64 = vector::length(&hashed_messages);
        let presigns_len: u64 = vector::length(&presigns);
        let approvals_len: u64 = vector::length(message_approvals);
        let centralized_signed_len: u64 = vector::length(&centralized_signed_messages);
        assert!(messages_len == approvals_len, EApprovalsAndMessagesLenMismatch);
        assert!(messages_len == centralized_signed_len, ECentrailizedsignedMessagesAndMessagesLenMismatch);
        assert!(messages_len == presigns_len, EPresignsAndMessagesLenMismatch);
        let expected_dwallet_cap_id = get_dwallet_cap_id(dwallet);
        let batch_session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        event::emit(StartBatchedSignEvent {
            session_id: batch_session_id,
            hashed_messages,
            initiator: tx_context::sender(ctx)
        });
        let mut i = 0;
        let message_approvals_len = vector::length(message_approvals);
        while (i < message_approvals_len) {
            let presign = vector::pop_back(&mut presigns);
            assert!(object::id(dwallet) == presign.dwallet_id, EDwalletMismatch);
            // let message = verify_and_pop_message_approval(expected_dwallet_cap_id, hashed_messages, message_approvals);
            let message = vector::pop_back(&mut hashed_messages);
            let id = object::id_from_address(tx_context::fresh_object_address(ctx));
            let centralized_signed_message = vector::pop_back(&mut centralized_signed_messages);
            event::emit(StartSignEvent {
                session_id: id,
                presign_session_id: presign.first_round_session_id,
                initiator: tx_context::sender(ctx),
                batched_session_id: batch_session_id,
                dwallet_id: object::id(dwallet),
                presign: presign.presign,
                centralized_signed_message: centralized_signed_message,
                dkg_output: get_dwallet_output<Secp256K1>(dwallet),
                hashed_message: message,
            });
            transfer::transfer(presign, SYSTEM_ADDRESS);
            i = i + 1;
        };
        presigns.destroy_empty();
    }

    /// Emits a `CompletedSignEvent` with the MPC Sign protocol output.
    ///
    /// This function is called by the blockchain itself and is part of the core
    /// blockchain logic executed by validators. The emitted event contains the
    /// completed sign output that should be consumed by the initiating user.
    ///
    /// ### Parameters
    /// - **`signed_messages`**: A vector containing the signed message outputs.
    /// - **`batch_session_id`**: The unique identifier for the batch signing session.
    /// - **`ctx`**: The transaction context used for event emission.
    ///
    /// ### Requirements
    /// - The caller **must be the system address** (`@0x0`). If this condition is not met,
    ///   the function will abort with `ENotSystemAddress`.
    ///
    /// ### Events
    /// - **`CompletedSignEvent`**: Emitted with the `session_id` and `signed_messages`,
    ///   signaling the completion of the sign process for the batch session.
    ///
    /// ### Errors
    /// - **`ENotSystemAddress`**: If the caller is not the system address (`@0x0`),
    ///   the function will abort with this error.
    #[allow(unused_function)]
    fun create_sign_output(
        signed_messages: vector<vector<u8>>,
        batch_session_id: ID,
        ctx: &TxContext
    ) {
        // Ensure that only the system address can call this function.
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);

        // Emit the CompletedSignEvent with session ID and signed messages.
        event::emit(CompletedSignEvent {
            session_id: batch_session_id,
            signed_messages,
        });
    }

    #[test_only]
    /// Call the underlying `create_sign_output`.
    /// This function is intended for testing purposes only and should not be used in production.
    /// See Move pattern: https://move-book.com/move-basics/testing.html#utilities-with-test_only
    public fun create_sign_output_for_testing(
        signed_messages: vector<vector<u8>>,
        batch_session_id: ID,
        ctx: &mut TxContext
    ) {
        // Call the main create_sign_output function with the provided parameters
        create_sign_output(
            signed_messages,
            batch_session_id,
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
        dwallet::create_dwallet<Secp256K1>(session_id, dwallet_cap_id, dkg_output, 1, ctx)
    }

    /// Generates a new mock `Presign` object with random IDs and data.
    /// This function is useful for testing or initializing Presign objects.
    public fun create_mock_presign(
        dwallet_id: ID,
        presign: vector<u8>,
        first_round_session_id: ID,
        ctx: &mut TxContext,
    ): Presign {
        let id = object::new(ctx);

        // Create and return the Presign object.
        Presign {
            id,
            dwallet_id,
            presign,
            first_round_session_id,
        }
    }

    /// A function to publish messages signed by the user on chain with on-chain verification,
    /// without launching the chain's sign flow immediately.
    ///
    /// See the docs of [`PartiallySignedMessages`] for more details on when this may be used.
    public fun publish_partially_signed_messages(
        signatures: vector<vector<u8>>,
        messages: vector<vector<u8>>,
        mut presigns: vector<Presign>,
        dwallet: &DWallet<Secp256K1>,
        ctx: &mut TxContext
    ) {
        let messages_len = vector::length(&messages);
        let signatures_len = vector::length(&signatures);
        let presigns_len = vector::length(&presigns);
        assert!(messages_len == signatures_len, EApprovalsAndSignaturesLenMismatch);
        assert!(messages_len == presigns_len, EPresignsAndMessagesLenMismatch);
        let mut presigns_bytes: vector<vector<u8>> = vector::empty();
        let mut presign_session_ids: vector<ID> = vector::empty();
        let mut i = 0;
        while (i < messages_len) {
            let presign = vector::pop_back(&mut presigns);
            assert!(presign.dwallet_id == object::id(dwallet), EDwalletMismatch);
            presigns_bytes.push_back(presign.presign);
            presign_session_ids.push_back(presign.first_round_session_id);
            transfer::transfer(presign, SYSTEM_ADDRESS);
            i = i + 1;
        };
        presigns_bytes.reverse();
        presign_session_ids.reverse();
        presigns.destroy_empty();
        assert!(
            verify_partially_signed_signatures_native(
                signatures,
                messages,
                presigns_bytes,
                get_dwallet_output(dwallet)
            ),
            EInvalidSignatures
        );
        let partial_signatures = PartiallySignedMessages {
            id: object::new(ctx),
            presigns: presigns_bytes,
            presign_session_ids,
            messages,
            signatures,
            dwallet_output: get_dwallet_output(dwallet),
            dwallet_id: object::id(dwallet),
            dwallet_cap_id: get_dwallet_cap_id(dwallet),
        };
        event::emit(CreatedPartiallySignedMessagesEvent {
            partial_signatures_object_id: object::id(&partial_signatures),
        });
        transfer::transfer(partial_signatures, tx_context::sender(ctx));
    }

    /// A function to launch a sign flow with a previously published [`PartiallySignedMessages`].
    ///
    /// See the docs of [`PartiallySignedMessages`] for more details on when this may be used.
    public fun future_sign(
        partial_signature: PartiallySignedMessages,
        message_approvals: &mut vector<MessageApproval>,
        ctx: &mut TxContext
    ) {
        let PartiallySignedMessages {
            id,
            mut presigns,
            mut presign_session_ids,
            mut messages,
            mut signatures,
            dwallet_id,
            dwallet_cap_id,
            dwallet_output,
        } = partial_signature;
        object::delete(id);
        let message_approvals_len = vector::length(message_approvals);
        let messages_len = vector::length(&messages);
        assert!(message_approvals_len == messages_len, EApprovalsAndMessagesLenMismatch);
        let batch_session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        event::emit(StartBatchedSignEvent {
            session_id: batch_session_id,
            hashed_messages: messages,
            initiator: tx_context::sender(ctx)
        });
        let mut i = 0;
        while (i < message_approvals_len) {
            let message = vector::pop_back(&mut messages);
            // let message = verify_and_pop_message_approval(dwallet_cap_id, messages, message_approvals);
            let id = object::id_from_address(tx_context::fresh_object_address(ctx));
            let centralized_signed_message = vector::pop_back(&mut signatures);
            let presign = vector::pop_back(&mut presigns);
            let presign_session_id = vector::pop_back(&mut presign_session_ids);
            event::emit(StartSignEvent {
                session_id: id,
                presign_session_id,
                initiator: tx_context::sender(ctx),
                batched_session_id: batch_session_id,
                dwallet_id,
                presign,
                centralized_signed_message,
                dkg_output: dwallet_output,
                hashed_message: message,
            });
            i = i + 1;
        };
    }

    #[allow(unused_function)]
    fun verify_and_pop_message_approval(dwallet_cap_id: ID, mut messages: vector<vector<u8>>, message_approvals: &mut vector<MessageApproval>): vector<u8> {
        let message_approval = vector::pop_back(message_approvals);
        let (message_approval_dwallet_cap_id, approved_message) = remove_message_approval(message_approval);
        assert!(dwallet_cap_id == message_approval_dwallet_cap_id, EMesssageApprovalDWalletMismatch);
        let message = vector::pop_back(&mut messages);
        assert!(&message == &approved_message, EMissingApprovalOrWorngApprovalOrder);
        message
    }

    /// Verifies that the user's centralized party signatures are valid.
    native fun verify_partially_signed_signatures_native(
        partial_signatures: vector<vector<u8>>,
        messages: vector<vector<u8>>,
        presigns: vector<vector<u8>>,
        dkg_output: vector<u8>
    ): bool;
}
