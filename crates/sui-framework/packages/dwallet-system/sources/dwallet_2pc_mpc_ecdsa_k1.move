// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear


#[allow(unused_const)]
module dwallet_system::dwallet_2pc_mpc_ecdsa_k1 {
    /// Note: currently in order to start DKG, Presign and Sign, the Valdators are waiting for an Event.
    /// The events are here below, this is a "hack" to pass the information, we might find a better way in the future.

    use std::vector;

    use dwallet::event;
    use dwallet::object::{Self, ID, UID};
    use dwallet::transfer;
    use dwallet::tx_context::{Self, TxContext};

    use dwallet_system::dwallet;
    use dwallet_system::dwallet::{
        create_dwallet_cap,
        create_malicious_aggregator_sign_output,
        create_sign_output,
        DWallet,
        get_dwallet_cap_id,
        DWalletCap,
        get_dwallet_public_key,
        get_messages,
        get_sign_data,
        get_output,
        PartialUserSignedMessages,
        SignSession, get_public_key, EncryptionKey, create_encrypted_user_share, get_encryption_key,
    };

    #[test_only]
    friend dwallet_system::dwallet_tests;

    #[test_only]
    friend dwallet_system::dwallet_ecdsa_k1_tests;


    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    const ENotSystemAddress: u64 = 0;
    const EMesssagesLengthMustBeGreaterThanZero: u64 = 1;
    const EPresignOutputAndPresignMismatch: u64 = 2;
    const ESignInvalidSignatureParts: u64 = 3;
    const ENotSupported: u64 = 4;
    const EEmptyCommitment: u64 = 5;
    const EEncryptUserShare: u64 = 6;

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<


    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<
    const KECCAK256: u8 = 0;
    const SHA256: u8 = 1;

    const SYSTEM_ADDRESS: address = @0x0;
    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<

    struct Secp256K1 has drop {}

    // <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<
    /// Event to start a `DKG` session, caught by the Validators.
    struct NewDKGSessionEvent has copy, drop {
        session_id: ID,
        dwallet_cap_id: ID,
        commitment_to_centralized_party_secret_key_share: vector<u8>,
        sender: address,
    }

    /// Event to start a `PreSign` session, caught by the Validators.
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

    /// `NewSignDataEvent` is embedded inside a `NewSignSessionEvent`.
    /// This is particular for the Dwallet Type.
    struct NewSignDataEvent has store, copy, drop {
        presign_session_id: ID,
        hash: u8,
        dkg_output: vector<u8>,
        public_nonce_encrypted_partial_signature_and_proofs: vector<u8>,
        presigns: vector<u8>,
    }

    // <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<

    /// `DKGSessionOutput` stores the DKG session output.
    struct DKGSession has key {
        id: UID,
        dwallet_cap_id: ID,
        commitment_to_centralized_party_secret_key_share: vector<u8>,
        sender: address,
    }

    /// `DKGSessionOutput` stores the DKG session output.
    struct DKGSessionOutput has key {
        id: UID,
        session_id: ID,
        dwallet_cap_id: ID,
        commitment_to_centralized_party_secret_key_share: vector<u8>,
        secret_key_share_encryption_and_proof: vector<u8>,
    }

    #[allow(unused_field)]
    /// `PresignSession` stores the Presign session data that is held by the Blockchain.
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
    /// `PresignSessionOutput` is the Presign session output that was generated by the Blockhain.
    struct PresignSessionOutput has key {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        output: vector<u8>,
    }

    #[allow(unused_field)]
    /// `Presign` stores the Presign data that is sent to the user.
    struct Presign has key {
        id: UID,
        session_id: ID,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        presigns: vector<u8>,
    }

    #[allow(unused_field)]
    /// `SignData` holds the data that is used to `Sign` the message.
    struct SignData has store {
        presign_session_id: ID,
        hash: u8,
        public_nonce_encrypted_partial_signature_and_proofs: vector<u8>,
        presigns: vector<u8>,
    }

    /// Starts a Distributed Key Generation (DKG) session.
    /// Capabilities are used to control access to the Dwallet.
    /// This function start the DKG proccess in the Validators.
    public fun create_dkg_session(
        commitment_to_centralized_party_secret_key_share: vector<u8>,
        ctx: &mut TxContext
    ): DWalletCap {
        assert!(commitment_to_centralized_party_secret_key_share != vector::empty<u8>(), EEmptyCommitment);
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
    /// Create the final DKG output, transfer it to the user.
    /// This function is called by blockchain itself.
    /// Validtors call it, it's part of the blockchain logic.
    fun create_dkg_output(
        session: &DKGSession,
        commitment_to_centralized_party_secret_key_share: vector<u8>,
        secret_key_share_encryption_and_proof: vector<u8>,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
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

    #[allow(unused_function)]
    #[test_only]
    /// Call the underline `create_dkg_output`.
    /// See Move pattern: https://move-book.com/move-basics/testing.html#utilities-with-test_only
    public fun create_dkg_output_for_testing(
        session: &DKGSession,
        commitment_to_centralized_party_secret_key_share: vector<u8>,
        secret_key_share_encryption_and_proof: vector<u8>,
        ctx: &mut TxContext
    ) {
        create_dkg_output(
            session,
            commitment_to_centralized_party_secret_key_share,
            secret_key_share_encryption_and_proof,
            ctx
        );
    }

    /// Create a new Dwallet.
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
        // `public_key` is the ECDSA public key.
        // Note that these function returns only after 2/3 or the validators reached concesus.
        let (output, public_key) = dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(
            commitment_to_centralized_party_secret_key_share,
            secret_key_share_encryption_and_proof,
            centralized_party_public_key_share_decommitment_and_proof
        );

        let dwallet = dwallet::create_dwallet<Secp256K1>(session_id, dwallet_cap_id, output, public_key, ctx);
        // Create dwallet + make it immutable.
        transfer::public_freeze_object(dwallet);
    }

    /// This function implements steps 4 and 5 of the 2PCMPC - Protocol 4 (DKG):
    /// Verifies commitment and zk-proof for $X_A$, and computes $X:=X_A+X_B$.
    /// [Source](https://eprint.iacr.org/archive/2024/253/20240217:153208)
    native fun dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(
        commitment_to_centralized_party_secret_key_share: vector<u8>,
        secret_key_share_encryption_and_proof: vector<u8>,
        centralized_party_public_key_share_decommitment_and_proofs: vector<u8>
    ): (vector<u8>, vector<u8>);


    /// Create a new Presgin session.
    /// Note that the `Dwallet` is immutable, and can be called by everyone.
    /// But, the `commitments_and_proof_to_centralized_party_nonce_shares` is owned by a specific user.
    /// This is trhe first part of the `PreSign` process.
    public fun create_presign_session(
        dwallet: &DWallet<Secp256K1>,
        // Note that in terms on the MPC, the `messages` is not mandatory on pre-signing,
        // currently it will be provided to prevent some attack vectors.
        messages: vector<vector<u8>>,
        commitments_and_proof_to_centralized_party_nonce_shares: vector<u8>,
        // hash = sha256 or sha3.
        hash: u8,
        ctx: &mut TxContext
    ) {
        assert!(hash == SHA256 || hash == KECCAK256, ENotSupported);
        let messages_len: u64 = vector::length(&messages);
        assert!(messages_len > 0, EMesssagesLengthMustBeGreaterThanZero);
        let dwallet_id = object::id(dwallet);
        let dwallet_cap_id = get_dwallet_cap_id(dwallet);
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
            dkg_output: get_output(dwallet),
            commitments_and_proof_to_centralized_party_nonce_shares,
            messages,
            sender,
        });
        transfer::freeze_object(session);
    }

    #[allow(unused_function)]
    /// This function is called by blockchain itself.
    /// Validtors call it, it's part of the blockchain logic.
    /// This is the _FIRST PART_ of the `PreSign` proccess (the validators first output this).
    /// Later this is used to finalize the `PreSign` session.
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
    /// This is the _SECOND PART of the `PreSign` proccess.
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

    /// Verifies parts of the signature.
    native fun sign_verify_encrypted_signature_parts_prehash(
        messages: vector<vector<u8>>,
        dkg_output: vector<u8>,
        public_nonce_encrypted_partial_signature_and_proofs: vector<u8>,
        presigns: vector<u8>,
        hash: u8
    ): bool;

    /// This function starts the `Sign` proccess, note that it must get `PresignSessionOutput` and `Presign`.
    /// The user needs to call this function after receiving the `Presign` and `PresignSessionOutput`.
    /// The user needs to provide the `public_nonce_encrypted_partial_signature_and_proofs`.
    public fun create_partial_user_signed_messages(
        dwallet: &DWallet<Secp256K1>,
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

        let valid_signature_parts = sign_verify_encrypted_signature_parts_prehash(
            session.messages,
            get_output(dwallet),
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

        // This event is caught by the blockhain.
        // This is a "hack" to pass the information.
        // Note: that in this case event is not emmitted!
        // It is passed to `create_partial_user_signed_messages()` func.
        let sign_data_event = NewSignDataEvent {
            presign_session_id: session_id,
            hash: session.hash,
            dkg_output: get_output(dwallet),
            public_nonce_encrypted_partial_signature_and_proofs,
            presigns,
        };

        dwallet::create_partial_user_signed_messages(
            dwallet_id,
            dwallet_cap_id,
            session.messages,
            get_public_key(dwallet),
            sign_data,
            sign_data_event,
            ctx
        )
    }

    #[allow(unused_function)]
    #[test_only]
    /// Call the underline `verify_and_create_sign_output`.
    public fun verify_and_create_sign_output_for_testing(
        session: &SignSession<SignData>,
        signatures: vector<vector<u8>>,
        aggregator_public_key: vector<u8>,
        ctx: &mut TxContext
    ) {
        verify_and_create_sign_output(
            session,
            signatures,
            aggregator_public_key,
            ctx
        );
    }

    #[allow(unused_function)]
    /// This function is called by blockchain itself.
    /// Validators call it, it's part of the blockchain logic.
    /// NOT a native function.
    fun verify_and_create_sign_output(
        session: &SignSession<SignData>,
        signatures: vector<vector<u8>>,
        aggregator_public_key: vector<u8>,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        if (verify_signatures_native(
            get_messages(session),
            signatures,
            get_sign_data(session).hash,
            get_dwallet_public_key(session),
        )) {
            create_sign_output(session, convert_signatures_to_canonical_form(signatures), ctx);
        } else {
            create_malicious_aggregator_sign_output(aggregator_public_key, session, signatures, ctx);
        }
    }

    /// The "cannoical form" is the serialized signature using the standard `ecdsa` crate.
    /// The form of `sigs` is the one used in the `2pc-mpc` crate.
    fun convert_signatures_to_canonical_form(sigs: vector<vector<u8>>): vector<vector<u8>> {
        vector::reverse(&mut sigs);
        let i = 0;
        let sigs_length = vector::length(&sigs);
        let parsed_sigs: vector<vector<u8>> = vector[];
        // Using this hacky way to map because Move vector tooling is limited.
        while (i < sigs_length) {
            vector::push_back(&mut parsed_sigs, convert_signature_to_canonical_form(vector::pop_back(&mut sigs)));
            i = i + 1;
        };
        parsed_sigs
    }

    native fun convert_signature_to_canonical_form(signature: vector<u8>): vector<u8>;

    /// Verifies the ECDSA signatures.
    native fun verify_signatures_native(
        messages: vector<vector<u8>>,
        sigs: vector<vector<u8>>,
        hash: u8,
        dwallet_public_key: vector<u8>,
    ): bool;

    /// Encrypt a user share with an AHE encryption key.
    public fun encrypt_user_share(
        dwallet: &DWallet<Secp256K1>,
        encryption_key: &EncryptionKey,
        encrypted_secret_share_and_proof: vector<u8>,
        ctx: &mut TxContext,
    ) {
        let is_valid = verify_encrypted_user_secret_share_secp256k1(
            get_encryption_key(encryption_key),
            encrypted_secret_share_and_proof,
            get_output(dwallet),
        );

        assert!(is_valid, EEncryptUserShare);

        create_encrypted_user_share(
            object::id(dwallet),
            encrypted_secret_share_and_proof,
            object::id(encryption_key),
            ctx
        );
    }

    #[allow(unused_function)]
    native fun verify_encrypted_user_secret_share_secp256k1(
        secret_share_public_key: vector<u8>,
        encrypted_secret_share_and_proof: vector<u8>,
        dwallet_output: vector<u8>,
    ): bool;


    #[test_only
    public(friend) fun create_mock_sign_data(presign_session_id: ID): SignData {
        SignData {
            presign_session_id,
            hash: SHA256,
            public_nonce_encrypted_partial_signature_and_proofs: vector::empty<u8>(),
            presigns: vector::empty<u8>()
        }
    }

    #[test_only]
    public(friend) fun create_mock_sign_data_event(presign_session_id: ID): NewSignDataEvent {
        NewSignDataEvent {
            presign_session_id,
            hash: 0,
            dkg_output: vector::empty<u8>(),
            public_nonce_encrypted_partial_signature_and_proofs: vector::empty<u8>(),
            presigns: vector::empty<u8>()
        }
    }
}
