// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_const)]
module dwallet_system::dwallet_2pc_mpc_ecdsa_k1 {
    use std::vector;
    use dwallet::object::{Self, UID, ID};
    use dwallet::transfer;
    use dwallet::event;
    use dwallet::tx_context::{Self, TxContext};
    use dwallet_system::dwallet::{create_dwallet_cap, SignMessages, DWalletCap};
    use dwallet_system::dwallet;

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

    public fun create_dkg_session(commitment_to_centralized_party_secret_key_share: vector<u8>, ctx: &mut TxContext): DWalletCap {
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
        cap
    }

    #[allow(unused_function)]
    fun create_dkg_output(session: &DKGSession, commitment_to_centralized_party_secret_key_share: vector<u8>, secret_key_share_encryption_and_proof: vector<u8>, ctx: &mut TxContext) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        let output = DKGSessionOutput {
            id: object::new(ctx),
            session_id: object::id(session),
            dwallet_cap_id: session.dwallet_cap_id,
            commitment_to_centralized_party_secret_key_share,
            secret_key_share_encryption_and_proof
        };
        transfer::transfer(output, session.sender);
    }

    public fun create_dwallet(output: DKGSessionOutput, centralized_party_public_key_share_decommitment_and_proof: vector<u8>, ctx: &mut TxContext) {
        let DKGSessionOutput {
            id,
            session_id,
            dwallet_cap_id,
            commitment_to_centralized_party_secret_key_share,
            secret_key_share_encryption_and_proof,
        } = output;
        object::delete(id);

        let (output, public_key) = dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(commitment_to_centralized_party_secret_key_share, secret_key_share_encryption_and_proof, centralized_party_public_key_share_decommitment_and_proof);

        let result = DWallet {
            id: object::new(ctx),
            session_id,
            dwallet_cap_id,
            output,
            public_key,
        };
        transfer::freeze_object(result);
    }

    native fun dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(commitment_to_centralized_party_secret_key_share: vector<u8>, secret_key_share_encryption_and_proof: vector<u8>, centralized_party_public_key_share_decommitment_and_proofs: vector<u8>): (vector<u8>, vector<u8>);

    public fun create_presign_session(dwallet: &DWallet, messages: vector<vector<u8>>, commitments_and_proof_to_centralized_party_nonce_shares: vector<u8>, hash: u8, ctx: &mut TxContext) {
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
    fun create_presign(session: &PresignSession, presigns: vector<u8>, ctx: &mut TxContext) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        let presign = Presign {
            id: object::new(ctx),
            session_id: object::id(session),
            dwallet_id: session.dwallet_id,
            dwallet_cap_id: session.dwallet_cap_id,
            presigns,
        };
        transfer::transfer(presign, session.sender);
    }

    native fun sign_verify_encrypted_signature_parts_prehash(messages: vector<vector<u8>>, dkg_output: vector<u8>, public_nonce_encrypted_partial_signature_and_proofs: vector<u8>, presigns: vector<u8>, hash: u8): bool;

    public fun create_sign_messages(dwallet: &DWallet, session: &PresignSession, output: PresignSessionOutput, presign: Presign, public_nonce_encrypted_partial_signature_and_proofs: vector<u8>, ctx: &mut TxContext): SignMessages<SignData, NewSignDataEvent> {
        assert!(object::id(session) == output.session_id && object::id(dwallet) == output.dwallet_id && output.dwallet_id == presign.dwallet_id && output.dwallet_cap_id == presign.dwallet_cap_id && output.session_id == presign.session_id, EPresignOutputAndPresignMismatch);

        let valid_signature_parts = sign_verify_encrypted_signature_parts_prehash(session.messages, dwallet.output, public_nonce_encrypted_partial_signature_and_proofs, presign.presigns, session.hash);
        assert!(valid_signature_parts, ESignInvalidSignatureParts);

        let PresignSessionOutput {
            id,
            session_id: _,
            dwallet_id: _,
            dwallet_cap_id: _,
            output: _,
        } = output;
        object::delete(id);

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

        let sign_data_event = NewSignDataEvent {
            presign_session_id: session_id,
            hash: session.hash,
            dkg_output: dwallet.output,
            public_nonce_encrypted_partial_signature_and_proofs,
            presigns,
        };

        dwallet::create_sign_messages(dwallet_id, dwallet_cap_id, session.messages, sign_data, sign_data_event, ctx)
    }
}
