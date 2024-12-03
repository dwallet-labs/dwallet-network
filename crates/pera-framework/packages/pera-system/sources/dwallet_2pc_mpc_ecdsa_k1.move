// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_const, unused_field, unused_variable, unused_use)]
module pera_system::dwallet_2pc_mpc_ecdsa_k1 {
    use pera_system::dwallet;
    use pera_system::dwallet::{DWallet, create_dwallet_cap, DWalletCap, get_dwallet_cap_id, get_dwallet_output};
    use pera::event;

    /// `MessageApproval` represents a message that was approved.
    /// Bound to a `DWalletCap`.
    public struct MessageApproval has store, drop {
        dwallet_cap_id: ID,
        message: vector<u8>,
    }

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

    /// Event that indicates the completion of a Sign flow and includes signatures on all the messages.
    public struct CompletedSignEvent has copy, drop {
        session_id: ID,
        signed_messages: vector<vector<u8>>,
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
    const EMesssageApprovalDWalletMismatch: u64 = 1;
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

    /// Create a set of message approvals.
    /// The messages must be approved in the same order as they were created.
    /// The messages must be approved by the same `dwallet_cap_id`.
    public fun approve_messages(
        dwallet_cap: &DWalletCap,
        mut messages: vector<vector<u8>>
    ): vector<MessageApproval> {
        let dwallet_cap_id = object::id(dwallet_cap);
        let mut message_approvals = vector::empty<MessageApproval>();
        while (vector::length(&messages) > 0) {
            let message = vector::pop_back(&mut messages);
            vector::push_back(&mut message_approvals, MessageApproval {
                dwallet_cap_id,
                message,
            });
        };
        message_approvals
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
        first_round_session_id: ID,
        first_round_output: vector<u8>,
        second_round_output: vector<u8>,
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
        first_round_session_id: ID,
        first_round_output: vector<u8>,
        second_round_output: vector<u8>,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);

        let output = Presign {
            id: object::new(ctx),
            session_id,
            first_round_session_id,
            dwallet_id,
            dwallet_cap_id,
            first_round_output,
            second_round_output,
        };

        let event = CompletedPresignEvent {
            sender: session_initiator,
            dwallet_id,
            presign_id: object::id(&output),
        };

        event::emit(event);
        transfer::transfer(output, session_initiator);
    }

    public struct StartBatchedSignEvent has copy, drop {
        session_id: ID,
        hashed_messages: vector<vector<u8>>,
        initiating_user: address
    }

    /// Event emitted by the user to start the signing process.
    public struct StartSignEvent has copy, drop {
        session_id: ID,
        presign_session_id: ID,
        sender: address,
        batched_session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        dkg_output: vector<u8>,
        hashed_message: vector<u8>,
        presign_first_round_output: vector<u8>,
        presign_second_round_output: vector<u8>,
        centralized_signed_message: vector<u8>
    }

    /// Remove a `MessageApproval` and return the `dwallet_cap_id` and the `message`.
    public fun remove_message_approval(message_approval: MessageApproval): (ID, vector<u8>) {
        let MessageApproval {
            dwallet_cap_id,
            message
        } = message_approval;
        (dwallet_cap_id, message)
    }

    /// Starts the signing process.
    #[allow(unused_const, unused_field, unused_variable)]
    public fun sign(
        dwallet_cap_id: ID,
        mut message_approvals: vector<MessageApproval>,
        hashed_messages: vector<vector<u8>>,
        presign: &Presign,
        dwallet: &DWallet<Secp256K1>,
        centralized_signed_messages: vector<vector<u8>>,
        presign_session_id: ID,
        ctx: &mut TxContext
    ) {
        let messages_len: u64 = vector::length(&hashed_messages);
        let approval_len: u64 = vector::length(&message_approvals);
        assert!(messages_len == approval_len, EMesssageApprovalDWalletMismatch);

        let mut i: u64 = 0;
        while (i < messages_len) {
            let message_approval = vector::pop_back(&mut message_approvals);
            let (message_approval_dwallet_cap_id, approved_message) = remove_message_approval(message_approval);
            assert!(get_dwallet_cap_id(dwallet) == message_approval_dwallet_cap_id, EMesssageApprovalDWalletMismatch);
            let message = vector::borrow(&hashed_messages, i);
            assert!(message == &approved_message, EMesssageApprovalDWalletMismatch);
            i = i + 1;
        };

        let batch_session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        event::emit(StartBatchedSignEvent {
            session_id: batch_session_id,
            hashed_messages,
            initiating_user: tx_context::sender(ctx)
        });
        let mut i = 0;
        let messages_length = vector::length(&hashed_messages);
        while (i < messages_length) {
            let id = object::id_from_address(tx_context::fresh_object_address(ctx));
            let event = StartSignEvent {
                session_id: id,
                presign_session_id,
                sender: tx_context::sender(ctx),
                batched_session_id: batch_session_id,
                dwallet_id: object::id(dwallet),
                dwallet_cap_id,
                presign_first_round_output: presign.first_round_output,
                presign_second_round_output: presign.second_round_output,
                centralized_signed_message: centralized_signed_messages[i],
                dkg_output: get_dwallet_output<Secp256K1>(dwallet),
                hashed_message: hashed_messages[i],
            };
            event::emit(event);
            i = i + 1;
        };
    }

    /// Object representing the output of the signing process.
    public struct SignOutput has key {
        id: UID,
        session_id: ID,
        output: vector<u8>,
    }

    /// Todo: Remove this function
    public fun mock_sign(
        hashed_messages: vector<vector<u8>>,
        presign_first_round_output: vector<u8>,
        presign_second_round_output: vector<u8>,
        dkg_output: vector<u8>,
        centralized_signed_messages: vector<vector<u8>>,
        presign_session_id: ID,
        ctx: &mut TxContext
    ) {
        let batch_session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        event::emit(StartBatchedSignEvent {
            session_id: batch_session_id,
            hashed_messages,
            initiating_user: tx_context::sender(ctx)
        });
        let mut i = 0;
        let messages_length = vector::length(&hashed_messages);
        while (i < messages_length) {
            let id = object::id_from_address(tx_context::fresh_object_address(ctx));
            let event = StartSignEvent {
                session_id: id,
                presign_session_id,
                sender: tx_context::sender(ctx),
                batched_session_id: batch_session_id,
                dwallet_id: id,
                dwallet_cap_id: id,
                presign_first_round_output: presign_first_round_output,
                presign_second_round_output: presign_second_round_output,
                centralized_signed_message: centralized_signed_messages[i],
                dkg_output,
                hashed_message: hashed_messages[i],
            };
            event::emit(event);
            i = i + 1;
        };
    }

    /// Emit event with the MPC Sign protocol output. 
    /// The initiating user should consume the emitted event.
    public fun create_sign_output(signed_messages: vector<vector<u8>>, batch_session_id: ID, ctx: &mut TxContext) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        event::emit(CompletedSignEvent {
            session_id: batch_session_id,
            signed_messages,
        });
    }
}
