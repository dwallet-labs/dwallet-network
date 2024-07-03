// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear


/// Note: currently in order to start DKG, Presign and Sign, the Valdators are waiting for an Event.
/// The events are here below, this is a "hack" to pass the information, we might find a better way in the future.

#[allow(unused_const)]
module dwallet_system::dwallet_2pc_mpc_ecdsa_k1 {
    use std::vector;

    use dwallet::event;
    use dwallet::object::{Self, ID, UID};
    use dwallet::transfer;
    use dwallet::tx_context::{Self, TxContext};

    use dwallet_system::dwallet;
    use dwallet_system::dwallet::{create_dwallet_cap, DWalletCap, PartialUserSignedMessages};

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    const ENotSystemAddress: u64 = 0;
    const EMesssagesLengthMustBeGreaterThanZero: u64 = 1;
    const EPresignOutputAndPresignMismatch: u64 = 2;
    const ESignInvalidSignatureParts: u64 = 3;
    const ENotSupported: u64 = 4;

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<


    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<
    const KECCAK256: u8 = 0;
    const SHA256: u8 = 1;

    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<


    // <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<

    struct NewDKGSessionEvent has copy, drop {
        session_id: ID,
        dwallet_cap_id: ID,
        commitment_to_centralized_party_secret_key_share: vector<u8>,
        sender: address,
    }

    struct NewPresignSessionEvent has copy, drop {
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        hash: u8,
        dkg_output: vector<u8>,
        commitments_and_proof_to_centralized_party_nonce_shares: vector<u8>,
        messages: vector<vector<u8>>,
        sender: address,
    }

    struct NewSignDataEvent has store, copy, drop {
        presign_session_id: ID,
        hash: u8,
        dkg_output: vector<u8>,
        public_nonce_encrypted_partial_signature_and_proofs: vector<u8>,
        presigns: vector<u8>,
    }

    // <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<


    #[allow(unused_field)]
    struct DWallet has key, store {
        id: UID,
        session_id: ID,
        dwallet_cap_id: ID,
        output: vector<u8>,
        public_key: vector<u8>,
    }

    struct DKGSession has key {
        id: UID,
        dwallet_cap_id: ID,
        commitment_to_centralized_party_secret_key_share: vector<u8>,
        sender: address,
    }

    struct DKGSessionOutput has key {
        id: UID,
        session_id: ID,
        dwallet_cap_id: ID,
        commitment_to_centralized_party_secret_key_share: vector<u8>,
        secret_key_share_encryption_and_proof: vector<u8>,
    }

    #[allow(unused_field)]
    struct PresignSession has key {
        id: UID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        hash: u8,
        commitments_and_proof_to_centralized_party_nonce_shares: vector<u8>,
        messages: vector<vector<u8>>,
        sender: address,
    }

    #[allow(unused_field)]
    struct PresignSessionOutput has key {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        output: vector<u8>,
    }

    #[allow(unused_field)]
    struct Presign has key {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        presigns: vector<u8>,
    }

    #[allow(unused_field)]
    struct SignData has store {
        presign_session_id: ID,
        hash: u8,
        public_nonce_encrypted_partial_signature_and_proofs: vector<u8>,
        presigns: vector<u8>,
    }

    /// Starts a DKG session.
    /// Capabilities are used to control access to the wallet.
    /// todo(zeev): might be entry?
    public fun create_dkg_session(
        commitment_to_centralized_party_secret_key_share: vector<u8>,
        ctx: &mut TxContext
    ): DWalletCap {
        let cap = create_dwallet_cap(ctx);
        let sender = tx_context::sender(ctx);
        let session = DKGSession {
            id: object::new(ctx),
            dwallet_cap_id: object::id(&cap),
            commitment_to_centralized_party_secret_key_share,
            sender,
        };
        event::emit(NewDKGSessionEvent {
            session_id: object::id(&session),
            dwallet_cap_id: object::id(&cap),
            commitment_to_centralized_party_secret_key_share,
            sender,
        });
        transfer::freeze_object(session);
        /// todo(omer): why do we need to return the cap?
        cap
    }

    #[allow(unused_function)]
    /// This function is called by blockchain itself.
    /// Validtors call it, it's part of the blockchain logic.
    fun create_dkg_output(
        session: &DKGSession,
        commitment_to_centralized_party_secret_key_share: vector<u8>,
        secret_key_share_encryption_and_proof: vector<u8>,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        let output = DKGSessionOutput {
            id: object::new(ctx),
            session_id: object::id(session),
            dwallet_cap_id: session.dwallet_cap_id,
            commitment_to_centralized_party_secret_key_share,
            secret_key_share_encryption_and_proof
        };
        // Send the blockchain DKG output to the user.
        transfer::transfer(output, session.sender);
    }

    /// Create a new dwallet.
    /// The user needs to call this function after receiving the DKG output.
    /// The user needs to provide the decommitment and proof of the centralized party public key share.
    public fun create_dwallet(
        output: DKGSessionOutput,
        centralized_party_public_key_share_decommitment_and_proof: vector<u8>,
        ctx: &mut TxContext
    ) {
        let DKGSessionOutput {
            id,
            session_id,
            dwallet_cap_id,
            commitment_to_centralized_party_secret_key_share,
            secret_key_share_encryption_and_proof,
        } = output;
        object::delete(id);

        // Native func.
        // todo(zeev): what is the output? and public key? which public key is this? of the ECDSA? rename output.
        // todo(zeev): Output is used by the blockchain to continue the DKG, need to rename and doc.
        // Answer on public_key == the ECDSA public key.
        // Note that these functuion returns only after 2/3 or the validators reached concesus.
        let (output, public_key) = dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(
            commitment_to_centralized_party_secret_key_share,
            secret_key_share_encryption_and_proof,
            centralized_party_public_key_share_decommitment_and_proof
        );

        let result = DWallet {
            id: object::new(ctx),
            session_id,
            dwallet_cap_id,
            output,
            public_key,
        };
        // Create dwallet + make it immutable.
        // todo(omer): why?
        transfer::freeze_object(result);
    }

    native fun dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(
        commitment_to_centralized_party_secret_key_share: vector<u8>,
        secret_key_share_encryption_and_proof: vector<u8>,
        centralized_party_public_key_share_decommitment_and_proofs: vector<u8>
    ): (vector<u8>, vector<u8>);


    /// Create a new Presgin session.
    /// Note that the dwallet is immutable, and can be called by everyone.
    /// But, the commitments_and_proof_to_centralized_party_nonce_shares is owned by a specific user.
    public fun create_presign_session(
        dwallet: &DWallet,
        // Note that in terms on the MPC, the `messages` is not mandatory on pre-signing,
        // currently it will be provided to prevent some attack vectors.
        messages: vector<vector<u8>>,
        commitments_and_proof_to_centralized_party_nonce_shares: vector<u8>,
        // hash = sha256 or sha3
        hash: u8,
        ctx: &mut TxContext
    ) {
        assert!(hash == SHA256 || hash == KECCAK256, ENotSupported);
        let messages_len = vector::length(&messages);
        assert!(messages_len > 0, EMesssagesLengthMustBeGreaterThanZero);
        let dwallet_id = object::id(dwallet);
        let dwallet_cap_id = dwallet.dwallet_cap_id;
        let sender = tx_context::sender(ctx);

        let session = PresignSession {
            id: object::new(ctx),
            dwallet_id,
            dwallet_cap_id,
            hash,
            commitments_and_proof_to_centralized_party_nonce_shares,
            messages,
            sender,
        };
        event::emit(NewPresignSessionEvent {
            session_id: object::id(&session),
            dwallet_id,
            dwallet_cap_id,
            hash,
            dkg_output: dwallet.output,
            commitments_and_proof_to_centralized_party_nonce_shares,
            messages,
            sender,
        });
        transfer::freeze_object(session);
    }

    #[allow(unused_function)]
    /// This function is called by blockchain itself.
    /// Validtors call it, it's part of the blockchain logic.
    /// This is the first part of the out presign session (the validators first output this)
    fun create_presign_output(session: &PresignSession, output: vector<u8>, ctx: &mut TxContext) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        let output = PresignSessionOutput {
            id: object::new(ctx),
            session_id: object::id(session),
            dwallet_id: session.dwallet_id,
            dwallet_cap_id: session.dwallet_cap_id,
            output,
        };
        transfer::transfer(output, session.sender);
    }

    #[allow(unused_function)]
    /// This function is called by blockchain itself.
    /// Validtors call it, it's part of the blockchain logic.
    /// This is the second part of the presign session.
    /// todo: rename to finalize or something.
    fun create_presign(session: &PresignSession, presigns: vector<u8>, ctx: &mut TxContext) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        // The user needs this object and PresignSessionOutput in order to Sign the message.
        let presign = Presign {
            id: object::new(ctx),
            session_id: object::id(session),
            dwallet_id: session.dwallet_id,
            dwallet_cap_id: session.dwallet_cap_id,
            presigns,
        };
        transfer::transfer(presign, session.sender);
    }

    /// Verifies parts of the signature, todo(zeev): which and hwy?
    native fun sign_verify_encrypted_signature_parts_prehash(
        messages: vector<vector<u8>>,
        dkg_output: vector<u8>,
        public_nonce_encrypted_partial_signature_and_proofs: vector<u8>,
        presigns: vector<u8>,
        hash: u8
    ): bool;

    /// This function start the sign proccess, note that it must get PresignSessionOutput and Presign.
    /// The user needs to call this function after receiving the Presign and PresignSessionOutput.
    /// The user needs to provide the public_nonce_encrypted_partial_signature_and_proofs.
    public fun create_partial_user_signed_messages(
        dwallet: &DWallet,
        session: &PresignSession,
        output: PresignSessionOutput,
        presign: Presign,
        // The user part of the signature.
        public_nonce_encrypted_partial_signature_and_proofs: vector<u8>,
        ctx: &mut TxContext
    ): PartialUserSignedMessages<SignData, NewSignDataEvent> {
        assert!(
            object::id(session) == output.session_id && object::id(
                dwallet
            ) == output.dwallet_id && output.dwallet_id == presign.dwallet_id && output.dwallet_cap_id == presign.dwallet_cap_id && output.session_id == presign.session_id,
            EPresignOutputAndPresignMismatch
        );

        // todo(zeev): doc this.
        let valid_signature_parts = sign_verify_encrypted_signature_parts_prehash(
            session.messages,
            dwallet.output,
            public_nonce_encrypted_partial_signature_and_proofs,
            presign.presigns,
            session.hash
        );
        assert!(valid_signature_parts, ESignInvalidSignatureParts);

        // Drop the object so it won't be used again.
        let PresignSessionOutput {
            id,
            session_id: _,
            dwallet_id: _,
            dwallet_cap_id: _,
            output: _,
        } = output;
        object::delete(id);

        // Drop the object so it won't be used again.
        let Presign {
            id,
            session_id,
            dwallet_id,
            dwallet_cap_id,
            presigns,
        } = presign;
        object::delete(id);

        let sign_data = SignData {
            presign_session_id: session_id,
            hash: session.hash,
            public_nonce_encrypted_partial_signature_and_proofs,
            presigns,
        };

        // These events are listenbd by the blockhain.
        // This is a "hack" to pass the information.
        // Note: that in this case event is not emmitted!
        // It is passed to `create_partial_user_signed_messages()` func.
        let sign_data_event = NewSignDataEvent {
            presign_session_id: session_id,
            hash: session.hash,
            dkg_output: dwallet.output,
            public_nonce_encrypted_partial_signature_and_proofs,
            presigns,
        };

        dwallet::create_partial_user_signed_messages(
            dwallet_id,
            dwallet_cap_id,
            session.messages,
            sign_data,
            sign_data_event,
            ctx
        )
    }
}
