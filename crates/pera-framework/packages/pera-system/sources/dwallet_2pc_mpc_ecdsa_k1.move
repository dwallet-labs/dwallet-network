// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_const)]
module pera_system::dwallet_2pc_mpc_ecdsa_k1 {
    use pera_system::dwallet;
    use pera_system::dwallet::{DWallet, create_dwallet_cap, DWalletCap, get_dwallet_cap_id, get_dwallet_output};
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

    /// Event to start a `DKG` session, caught by the Validators.
    public struct StartDKGFirstRoundEvent has copy, drop {
        session_id: address,
        sender: address,
        dwallet_cap_id: ID,
    }

    /// Event emitted to start the validator's second DKG round.
    /// Each validator catches this event and starts the second round of the DKG.
    public struct StartDKGSecondRoundEvent has copy, drop {
        session_id: address,
        sender: address,
        first_round_output: vector<u8>,
        public_key_share_and_proof: vector<u8>,
        dwallet_cap_id: ID,
        first_round_session_id: ID,
    }

    /// Event emitted when the second round of the DKG is completed,
    /// containing all relevant data from that round.
    public struct CompletedDKGSecondRoundEvent has copy, drop {
        session_id: ID,
        sender: address,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        value: vector<u8>,
    }

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    const ENotSystemAddress: u64 = 0;
    const EInvalidParams: u64 = 1;
    // >>>>>>>>>>>>>>>>>>>>>>>> Error codes >>>>>>>>>>>>>>>>>>>>>>>>

    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<
    const SYSTEM_ADDRESS: address = @0x0;
    // >>>>>>>>>>>>>>>>>>>>>>>> Constants >>>>>>>>>>>>>>>>>>>>>>>>

    /// Starts the first Distributed Key Generation (DKG) session. Two MPC sessions are required to
    /// create a Dwallet.
    /// Capabilities are used to control access to the Dwallet.
    /// This function starts the DKG process within Validators.
    public fun launch_dkg_first_round(
        ctx: &mut TxContext
    ): address {
        let cap_id = create_dwallet_cap(ctx);
        let sender = tx_context::sender(ctx);
        let session_id = tx_context::fresh_object_address(ctx);
        event::emit(StartDKGFirstRoundEvent {
            session_id,
            sender,
            dwallet_cap_id: cap_id,
        });
        session_id
    }

    #[allow(unused_function)]
    /// Creates the output of the first round of the DKG MPC, transferring it to the initiating user.
    /// This function is called by the blockchain itself.
    /// Validators call it as part of the blockchain logic.
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
    }

    /// Launches the second DKG round, emitting an event with all required data.
    /// Each validator catches this event and starts the second round of the DKG.
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

    /// Completes the second DKG MPC round by creating the actual [`dwallet::DWallet`].
    /// This function is called by the blockchain itself.
    /// Validators call it as part of the blockchain logic.
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

    /// Event emitted to initiate a `Presign` session, caught by the Validators.
    public struct StartPresignFirstRoundEvent has copy, drop {
        session_id: ID,
        sender: address,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        dkg_output: vector<u8>,
    }

    /// Starts the first round of the presign session.
    public fun launch_presign_first_round(
        dwallet: &DWallet<Secp256K1>,
        ctx: &mut TxContext
    ) {
        let session_id = tx_context::fresh_object_address(ctx);
        let event = StartPresignFirstRoundEvent {
            session_id: object::id_from_address(session_id),
            sender: tx_context::sender(ctx),
            dwallet_id: object::id(dwallet),
            dwallet_cap_id: get_dwallet_cap_id<Secp256K1>(dwallet),
            dkg_output: get_dwallet_output<Secp256K1>(dwallet),
        };
        event::emit(event);
    }

    /// Object to store the output of the first round of the presign session.
    public struct PresignSessionOutput has key {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        output: vector<u8>,
    }

    /// Creates the first presign round output and transfers it to the initiating user.
    /// Validators call it as part of the blockchain logic.
    /// This also triggers the second round of the presign session.
    public fun create_first_presign_round_output_and_launch_second_round(
        session_initiator: address,
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
        transfer::transfer(output, session_initiator);
        launch_presign_second_round(
            session_initiator,
            dwallet_id,
            dkg_output,
            dwallet_cap_id,
            first_round_output,
            session_id,
            ctx
        );
    }

    /// Event emitted to initiate the second round of a `Presign` session, caught by the Validators.
    public struct StartPresignSecondRoundEvent has copy, drop {
        session_id: ID,
        sender: address,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        dkg_output: vector<u8>,
        first_round_output: vector<u8>,
        first_round_session_id: ID,
    }

    /// Launches the second round of the presign session.
    /// Validators catch this event and begin processing for the second round.
    public fun launch_presign_second_round(
        session_initiator: address,
        dwallet_id: ID,
        dkg_output: vector<u8>,
        dwallet_cap_id: ID,
        first_round_output: vector<u8>,
        first_round_session_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);

        let session_id = tx_context::fresh_object_address(ctx);
        let session_id = object::id_from_address(session_id);

        let event = StartPresignSecondRoundEvent {
            session_id,
            sender: session_initiator,
            dwallet_id,
            dwallet_cap_id,
            dkg_output,
            first_round_output: first_round_output,
            first_round_session_id: first_round_session_id,
        };
        event::emit(event);
    }

    /// Represents the presign result of a the second and final presign round.
    public struct Presign has key {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        presigns: vector<u8>,
    }

    /// Event emitted when the presign second round is completed.
    public struct CompletedPresignEvent has copy, drop {
        sender: address,
        dwallet_id: ID,
        presign_id: ID,
    }

    /// Completes the presign session by creating the output of the second presign round
    /// and transferring it to the session initiator.
    public fun create_second_presign_round_output(
        session_initiator: address,
        session_id: ID,
        output: vector<u8>,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);

        let output = Presign {
            id: object::new(ctx),
            session_id,
            dwallet_id,
            dwallet_cap_id,
            presigns: output,
        };

        let event = CompletedPresignEvent {
            sender: session_initiator,
            dwallet_id,
            presign_id: object::id(&output),
        };

        event::emit(event);
        transfer::transfer(output, session_initiator);
    }

    /// Event emitted by the user to start the signing process.
    public struct StartSignEvent has copy, drop {
        session_id: ID,
        presign_session_id: ID,
        sender: address,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        dkg_output: vector<u8>,
        hashed_message: vector<u8>,
        presign_first_round_output: vector<u8>,
        presign_second_round_output: vector<u8>,
        centralized_signed_message: vector<u8>,
    }

/// Starts the signing process.
    public fun sign(
        dwallet_cap: &DWalletCap,
        hashed_message: vector<u8>,
        dwallet: &DWallet<Secp256K1>,
        presign_first_round: &PresignSessionOutput,
        presign_second_round: &Presign,
        centralized_signed_message: vector<u8>,
        presign_session_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(object::id(dwallet_cap) == get_dwallet_cap_id<Secp256K1>(dwallet), EInvalidParams);
        assert!(object::id(dwallet) == presign_second_round.dwallet_id, EInvalidParams);
        assert!(presign_second_round.dwallet_id == presign_first_round.dwallet_id, EInvalidParams);

        let id = object::id_from_address(tx_context::fresh_object_address(ctx));
        let event = StartSignEvent {
            session_id: id,
            presign_session_id,
            sender: tx_context::sender(ctx),
            dwallet_id: object::id(dwallet),
            dwallet_cap_id: object::id(dwallet_cap),
            presign_first_round_output: presign_first_round.output,
            presign_second_round_output: presign_second_round.presigns,
            centralized_signed_message,
            dkg_output: get_dwallet_output<Secp256K1>(dwallet),
            hashed_message
        };
        event::emit(event);
    }

    /// Todo: Remove this function
    public fun mock_sign(
        hashed_message: vector<u8>,
        presign_first_round_output: vector<u8>,
        presign: vector<u8>,
        dkg_output: vector<u8>,
        centralized_signed_message: vector<u8>,
        presign_session_id: ID,
        ctx: &mut TxContext
    ) {
        let id = object::id_from_address(tx_context::fresh_object_address(ctx));
        let event = StartSignEvent {
            session_id: id,
            presign_session_id,
            sender: tx_context::sender(ctx),
            dwallet_id: id,
            dwallet_cap_id: id,
            presign_first_round_output,
            presign_second_round_output: presign,
            centralized_signed_message,
            dkg_output,
            hashed_message
        };
        event::emit(event);
    }

    /// Object representing the output of the signing process.
    public struct SignOutput has key {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        output: vector<u8>,
    }

    /// Creates the output of the signing process and transfers it to the initiating user.
    public fun create_sign_output(dwallet_id: ID, initiating_user: address, session_id: ID, output: vector<u8>, ctx: &mut TxContext) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        let output = SignOutput {
            id: object::new(ctx),
            session_id,
            dwallet_id,
            output,
        };
        transfer::transfer(output, initiating_user);
    }
}
