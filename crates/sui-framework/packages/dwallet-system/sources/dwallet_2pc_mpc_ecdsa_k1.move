// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[allow(unused_const)]
module dwallet_system::dwallet_2pc_mpc_ecdsa_k1 {
    use dwallet::object::{Self, UID, ID};
    use dwallet::transfer;
    use dwallet::tx_context::{Self, TxContext};
    use dwallet_system::dwallet::{create_dwallet_cap, MessageApproval, remove};
    use std::vector;

    const ENotSystemAddress: u64 = 0;
    const EMesssageApprovalDWalletMismatch: u64 = 1;
    const EPresignOutputAndPresignMismatch: u64 = 2;
    const ENotSupported: u64 = 2;

    const KECCAK256: u8 = 0;
    const SHA256: u8 = 1;


    #[allow(unused_field)]
    struct DWallet has key, store {
        id: UID,
        session_id: ID,
        dwallet_cap_id: ID,
        output: vector<u8>,
        public_key: vector<u8>,
        encrypted_secret_key_share: vector<u8>,
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
        dkg_output: vector<u8>,
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
    struct SignSession has key {
        id: UID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        hash: u8,
        messages: vector<vector<u8>>,
        dkg_output: vector<u8>,
        public_nonce_encrypted_partial_signature_and_proofs: vector<u8>,
        presigns: vector<u8>,
        sender: address,
    }

    #[allow(unused_field)]
    struct SignOutput has key {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        hash: u8,
        signatures: vector<vector<u8>>,
        sender: address,
    }

    public fun create_dkg_session(commitment_to_centralized_party_secret_key_share: vector<u8>, ctx: &mut TxContext) {
        let cap = create_dwallet_cap(ctx);
        let session = DKGSession {
            id: object::new(ctx),
            dwallet_cap_id: object::id(&cap),
            commitment_to_centralized_party_secret_key_share,
            sender: tx_context::sender(ctx),
        };
        transfer::freeze_object(session);
        transfer::public_transfer(cap, tx_context::sender(ctx));
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

        let (output, public_key, encrypted_secret_key_share) = dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(commitment_to_centralized_party_secret_key_share, secret_key_share_encryption_and_proof, centralized_party_public_key_share_decommitment_and_proof);

        let result = DWallet {
            id: object::new(ctx),
            session_id,
            dwallet_cap_id,
            output,
            public_key,
            encrypted_secret_key_share,
        };
        transfer::freeze_object(result);
    }

    native fun dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(commitment_to_centralized_party_secret_key_share: vector<u8>, secret_key_share_encryption_and_proof: vector<u8>, centralized_party_public_key_share_decommitment_and_proofs: vector<u8>): (vector<u8>, vector<u8>, vector<u8>);

    public fun create_presign_session(dwallet: &DWallet, message_approvals: vector<MessageApproval>, commitments_and_proof_to_centralized_party_nonce_shares: vector<u8>, hash: u8, ctx: &mut TxContext) {
        assert!(hash == SHA256, ENotSupported);
        let dwallet_id = object::id(dwallet);
        let dwallet_cap_id = dwallet.dwallet_cap_id;
        let messages = vector::empty<vector<u8>>();

        while (vector::length(&message_approvals) > 0) {
            let message_approval = vector::pop_back(&mut message_approvals);
            let (message_approval_dwallet_cap_id, message) = remove(message_approval);
            assert!(dwallet_cap_id == message_approval_dwallet_cap_id, EMesssageApprovalDWalletMismatch);
            vector::push_back(&mut messages, message);
        };

        vector::destroy_empty(message_approvals);

        let session = PresignSession {
            id: object::new(ctx),
            dwallet_id,
            dwallet_cap_id,
            hash,
            dkg_output: dwallet.output,
            commitments_and_proof_to_centralized_party_nonce_shares,
            messages,
            sender: tx_context::sender(ctx),
        };
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

    public fun create_sign_session(dwallet: &DWallet, session: &PresignSession, output: PresignSessionOutput, presign: Presign, public_nonce_encrypted_partial_signature_and_proofs: vector<u8>, ctx: &mut TxContext) {

        assert!(object::id(session) == output.session_id && object::id(dwallet) == output.dwallet_id && output.dwallet_id == presign.dwallet_id && output.dwallet_cap_id == presign.dwallet_cap_id && output.session_id == presign.session_id, EPresignOutputAndPresignMismatch);

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
            session_id: _,
            dwallet_id,
            dwallet_cap_id,
            presigns,
        } = presign;
        object::delete(id);

        let sign = SignSession {
            id: object::new(ctx),
            dwallet_id,
            dwallet_cap_id,
            hash: session.hash,
            messages: session.messages,
            dkg_output: dwallet.output,
            public_nonce_encrypted_partial_signature_and_proofs,
            presigns,
            sender: tx_context::sender(ctx),
        };
        transfer::freeze_object(sign);
    }

    #[allow(unused_function)]
    fun create_sign_output(session: &SignSession, signatures: vector<vector<u8>>, ctx: &mut TxContext) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        let sign_output = SignOutput {
            id: object::new(ctx),
            session_id: object::id(session),
            dwallet_id: session.dwallet_id,
            hash: session.hash,
            dwallet_cap_id: session.dwallet_cap_id,
            signatures,
            sender: session.sender
        };
        transfer::transfer(sign_output, session.sender);
    }
}
