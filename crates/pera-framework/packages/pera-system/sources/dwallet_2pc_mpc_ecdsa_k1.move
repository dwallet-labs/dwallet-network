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
    use pera_system::dwallet::{
        DWallet,
        create_dwallet_cap,
        DWalletCap,
        get_dwallet_cap_id,
        get_dwallet_decentralized_output,
        get_dwallet_centralized_output,
        get_dwallet_mpc_network_key_version,
        EncryptionKey,
        get_encryption_key,
        pop_and_verify_message_approval,
        MessageApproval,
        hash_messages,
    };
    use pera::event;

    /// Represents the `Secp256K1` dWallet type.
    ///
    /// This struct serves as a marker to identify and signify
    /// the dWallet cryptographic scheme used for ECDSA
    /// (Elliptic Curve Digital Signature Algorithm)
    /// based on the `Secp256K1` curve.
    public struct Secp256K1 has drop {}

    /// An event emitted when the first round of the DKG process is completed.
    ///
    /// This event is emitted by the blockchain to notify the user about the completion of the first round.
    /// The user should catch this event to generate inputs for the second round and call the `launch_dkg_second_round` function.
    public struct DKGFirstRoundOutputEvent has copy, drop {
        session_id: ID,
        output_object_id: ID,
        output: vector<u8>,
    }

    /// The output of the first round of the dWallet creation DKG process.
    public struct DKGFirstRoundOutput has key, store {
        id: UID,
        session_id: ID,
        output: vector<u8>,
    }

    /// The output of a batched Sign session.
    public struct BatchedSignOutput has key, store {
        id: UID,
        session_id: ID,
        signatures: vector<vector<u8>>,
        dwallet_id: ID,
    }

    /// Represents the result of the second and final presign round.
    ///
    /// This struct links the results of both presign rounds to a specific dWallet ID.
    ///
    /// ### Fields
    /// - `id`: Unique identifier for the presign object.
    /// - `dwallet_id`: ID of the associated dWallet.
    /// - `first_round_session_id`: Session ID for the first presign round.
    /// - `presign`: Serialized output of the presign process.
    public struct Presign has key, store {
        id: UID,
        dwallet_id: ID,
        first_round_session_id: ID,
        presign: vector<u8>,
    }

    /// Event emitted to start an encrypted dWallet centralized (user) key share
    /// verification process.
    /// Ika does not support native functions, so an event is emitted and
    /// caught by the blockchain, which then starts the verification process,
    /// similar to the MPC processes.
    public struct StartEncryptedShareVerificationEvent has copy, drop {
        encrypted_secret_share_and_proof: vector<u8>,
        /// The DKG centralized output of the dwallet that its secret is being encrypted.
        dwallet_centralized_public_output: vector<u8>,
        /// The dWallet that this encrypted secret key share belongs to.
        dwallet_id: ID,
        /// The encryption key used to encrypt the secret key share with.
        encryption_key: vector<u8>,
        /// The `EncryptionKey` Move object ID.
        encryption_key_id: ID,
        session_id: ID,
        /// The signed dWallet public_centralized_output,
        /// signed by the secret key that corresponds to `encryptor_ed25519_pubkey`.
        // todo(zeev): rename.
        signed_public_share: vector<u8>,
        /// The public key of the encryptor.
        /// Used to verify the signature on the public share (centralized public output).
        encryptor_ed25519_pubkey: vector<u8>,
        // todo(itay): do we even need this? it should be dest?
        initiator: address,
    }

    /// Event emitted when an encrypted share is created by the system transaction.
    public struct CreatedEncryptedSecretShareEvent has copy, drop {
        session_id: ID,
        /// The ID of the encrypted secret key share.
        encrypted_share_obj_id: ID,
        dwallet_id: ID,
        /// The encrypted secret key share and proof of encryption.
        encrypted_secret_share_and_proof: vector<u8>,
        /// The Encryption (Public) Key that was used to encrypt the secret key share.
        encryption_key_id: ID,
        /// Source address of the entity that encrypted this secret key share.
        encryptor_address: address,
        /// Source Public Key of the entity that encrypted this secret key share (used for verifications).
        encryptor_ed25519_pubkey: vector<u8>,
        /// Signed dWallet public centralized (user) output (singed by the encryptor entity).
        /// todo(zeev): rename.
        signed_public_share: vector<u8>,
    }


    /// Messages that have been signed by a user, a.k.a the centralized party,
    /// but not yet by the blockchain.
    /// Used for scenarios where the user needs to first agree to sign some transaction,
    /// and the blockchain signs this transaction only later,
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
        /// The messages that are being signed.
        messages: vector<vector<u8>>,
        /// The user centralized signatures for each message.
        signatures: vector<vector<u8>>,
        dwallet_id: ID,
        /// The DKG output of the DWallet.
        dwallet_output: vector<u8>,
        dwallet_cap_id: ID,
        dwallet_mpc_network_key_version: u8,
    }

    /// Event emitted when a [`PartiallySignedMessages`] object is created.
    public struct CreatedPartiallySignedMessagesEvent has copy, drop {
        partial_signatures_object_id: ID,
    }

    /// Event emitted to start the first round of the DKG process.
    ///
    /// This event is caught by the blockchain, which is then using it to
    /// initiate the first round of the DKG.
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

    /// A verified encrypted dWallet secret user key share.
    ///
    /// This struct represents an encrypted user secret key share tied to a specific dWallet.
    /// It includes cryptographic proof that the encryption is valid and securely linked
    /// to the associated `dWallet`.
    public struct EncryptedUserSecretKeyShare has key {
        /// A unique identifier for this encrypted user share object.
        id: UID,

        /// The ID of the dWallet associated with this encrypted secret share.
        dwallet_id: ID,

        /// The encrypted secret key share along with a cryptographic proof
        /// that the encryption corresponds to the dWallet's secret key share.
        encrypted_secret_share_and_proof: vector<u8>,

        /// The ID of the `EncryptionKey` object used to encrypt the secret share.
        encryption_key_id: ID,

        /// The signed public share corresponding to the encrypted secret key share,
        /// used to verify its authenticity.
        signed_public_share: vector<u8>,

        /// The Ed25519 public key of the encryptor, used to verify the signature
        /// on the encrypted secret share.
        /// todo(zeev): I think it's used to verify the signed_public_share?
        encryptor_ed25519_pubkey: vector<u8>,

        /// The address of the encryptor, identifying who performed the encryption.
        /// If the key is transferred to someone else, this is the source entity.
        /// If the key is re-encrypted by an entity, then this is the Ika address of this entity.
        encryptor_address: address,
    }

    /// Event emitted to initiate the second round of the DKG process.
    ///
    /// This event is emitted to notify Validators to begin the second round of the DKG.
    /// It contains all necessary data to ensure proper continuation of the process.
    ///
    /// ### Fields
    /// - **`session_id`**: The unique identifier for the DKG session.
    /// - **`initiator`**: The address of the user who initiated the second round.
    /// - **`first_round_output`**: The output from the first round of the DKG process.
    /// - **`public_key_share_and_proof`**: A serialized vector containing the public key share and
    ///     its proof from the first round.
    /// - **`dwallet_cap_id`**: The unique identifier of the dWallet capability associated with this session.
    /// - **`first_round_session_id`**: The session ID of the first round of the DKG process.
    /// - **`encrypted_secret_share_and_proof`**: Encrypted secret share and the associated cryptographic proof.
    /// - **`encryption_key`**: The encryption key used in the process.
    /// - **`encryption_key_id`**: The unique identifier of the EncryptionKey object.
    /// - **`signed_public_share`**: The signed public share corresponding to the encrypted secret key share.
    /// - **`encryptor_ed25519_pubkey`**: The Ed25519 public key of the entity that performed the encryption.
    /// - **`dkg_centralized_public_output`**: The centralized public output of the DKG process.
    public struct StartDKGSecondRoundEvent has copy, drop {
        session_id: address,
        initiator: address,
        first_round_output: vector<u8>,
        public_key_share_and_proof: vector<u8>,
        dwallet_cap_id: ID,
        first_round_session_id: ID,
        encrypted_secret_share_and_proof: vector<u8>,
        encryption_key: vector<u8>,
        encryption_key_id: ID,
        signed_public_share: vector<u8>,
        encryptor_ed25519_pubkey: vector<u8>,
        dkg_centralized_public_output: vector<u8>,
    }

    /// Event emitted upon the completion of the second round of the
    /// Distributed Key Generation (DKG).
    ///
    /// This event provides all necessary data generated from the second
    /// round of the DKG process.
    /// This event is emitted to notify Validators to finalize the DKG
    /// process and store its results.
    ///
    /// ### Fields
    /// - **`session_id`**: A unique identifier for the DKG session, linking all related events and actions.
    /// - **`initiator`**: The address of the user who initiated the DKG process.
    /// - **`dwallet_cap_id`**: The unique identifier of the dWallet capability associated with the session.
    /// - **`dwallet_id`**: The identifier of the dWallet created as a result of the DKG process.
    /// - **`value`**: The output value from the second round of the DKG process,
    ///   representing the validated and combined result from all participants.
    public struct CompletedDKGSecondRoundEvent has copy, drop {
        session_id: ID,
        initiator: address,
        dwallet_cap_id: ID,
        dwallet_id: ID,
        value: vector<u8>,
    }

    /// Event emitted to initiate the first round of a Presign session.
    ///
    /// This event is used to signal Validators to start the first round of the Presign process.
    /// The event includes all necessary details to link the session to the corresponding dWallet
    /// and DKG process.
    ///
    /// ### Fields
    /// - **`session_id`**: A unique identifier for the Presign session.
    /// - **`initiator`**: The address of the user who initiated the Presign session.
    /// - **`dwallet_id`**: The unique identifier of the associated dWallet.
    /// - **`dkg_output`**: The output produced by the DKG process, used as input for the Presign session.
    /// - **`batch_session_id`**: A unique identifier for the Presign batch session.
    /// - **`dwallet_mpc_network_key_version`**: The version of the dWallet's MPC network key.
    public struct StartPresignFirstRoundEvent has copy, drop {
        session_id: ID,
        initiator: address,
        dwallet_id: ID,
        dkg_output: vector<u8>,
        batch_session_id: ID,
        dwallet_mpc_network_key_version: u8,
    }

    /// Event emitted to initiate the second round of a `Presign` session.
    ///
    /// This event signals Validators to begin the second round of the Presign process.
    /// The second round is a critical step in the multi-party computation (MPC) protocol,
    /// enabling the generation of pre-signatures required for ECDSA signing.
    ///
    /// ### Fields
    /// - **`session_id`**: A unique identifier for the current Presign session.
    /// - **`initiator`**: The address of the user who initiated the Presign session.
    /// - **`dwallet_id`**: The unique identifier of the DWallet associated with this Presign session.
    /// - **`dkg_output`**: The output from the Distributed Key Generation (DKG) process, used as input for the Presign session.
    /// - **`first_round_output`**: The output generated from the first round of the Presign session.
    /// - **`first_round_session_id`**: The session identifier for the first round of the Presign process.
    /// - **`batch_session_id`**: A unique identifier linking this session to a batched Presign process.
    /// - **`dwallet_mpc_network_key_version`**: The version of the dWallet's MPC network key.
    public struct StartPresignSecondRoundEvent has copy, drop {
        session_id: ID,
        initiator: address,
        dwallet_id: ID,
        dkg_output: vector<u8>,
        first_round_output: vector<u8>,
        first_round_session_id: ID,
        batch_session_id: ID,
        dwallet_mpc_network_key_version: u8,
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

    /// Event emitted to initiate the signing process.
    ///
    /// This event is captured by Validators to start the signing protocol.
    /// It includes all the necessary information to link the signing process to a specific dWallet,
    /// Presign session, and batched process.
    ///
    /// ### Fields
    /// - **`session_id`**: A unique identifier for this signing session.
    /// - **`presign_session_id`**: A unique identifier for the associated Presign session.
    /// - **`initiator`**: The address of the user who initiated the signing event.
    /// - **`batched_session_id`**: A unique identifier for the batched signing process this session belongs to.
    /// - **`dwallet_id`**: The unique identifier for the dWallet used in the session.
    /// - **`dkg_output`**: The output from the Distributed Key Generation (DKG) process used in this session.
    /// - **`hashed_message`**: The hashed message to be signed in this session.
    /// - **`presign`**: The Presign output used to assist in the signing process.
    /// - **`centralized_signed_message`**: The final signed message generated by the centralized signing process.
    /// - **`dwallet_mpc_network_key_version`**: The version of the dWallet's MPC network key.
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
        dwallet_mpc_network_key_version: u8,
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
    /// - **`initiator`**: The address of the user who initiated the protocol.
    public struct StartBatchedPresignEvent has copy, drop {
        session_id: ID,
        batch_size: u64,
        initiator: address
    }

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    /// Error raised when the sender is not the system address.
    const ENotSystemAddress: u64 = 1;
    const EDwalletMismatch: u64 = 2;
    const EApprovalsAndMessagesLenMismatch: u64 = 3;
    const ECentralizedSignedMessagesAndMessagesLenMismatch: u64 = 5;
    const EPresignsAndMessagesLenMismatch: u64 = 6;
    const EApprovalsAndSignaturesLenMismatch: u64 = 7;
    const EInvalidSignatures: u64 = 8;
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
    /// Validators call it, it's part of the blockchain logic.
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
        session_id: ID,
        output: vector<u8>,
        initiator: address,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
        let dkg_output = DKGFirstRoundOutput {
            session_id,
            id: object::new(ctx),
            output,
        };
        event::emit(DKGFirstRoundOutputEvent {
            session_id,
            output_object_id: object::id(&dkg_output),
            output,
        });
        transfer::transfer(dkg_output, initiator);
    }

    /// Initiates the second round of the Distributed Key Generation (DKG) process
    /// and emits an event for validators to begin their participation in this round.
    ///
    /// This function handles the creation of a new DKG session ID and emits an event containing
    /// all the necessary parameters to continue the DKG process.
    /// ### Parameters
    /// - `dwallet_cap`: A reference to the `DWalletCap`, representing the capability associated with the dWallet.
    /// - `centralized_public_key_share_and_proof`: The user (centralized) public key share and proof from the first round.
    /// - `first_round_output`: A reference to the `DKGFirstRoundOutput` structure containing the output of the first DKG round.
    /// - `first_round_session_id`: The session ID associated with the first DKG round.
    /// - `encrypted_secret_share_and_proof`: Encrypted user secret key share and its proof.
    /// - `encryption_key`: A reference to the `EncryptionKey` object used for encrypting the secret key share.
    /// - `signed_centralized_public_output`: The centralized public output signed by the caller.
    /// - `encryptor_ed25519_pubkey`: The Ed25519 public key of the encryptor.
    /// - `centralized_public_output`: The centralized public output of the DKG process.
    public fun launch_dkg_second_round(
        dwallet_cap: &DWalletCap,
        centralized_public_key_share_and_proof: vector<u8>,
        first_round_output: &DKGFirstRoundOutput,
        first_round_session_id: ID,
        encrypted_secret_share_and_proof: vector<u8>,
        encryption_key: &EncryptionKey,
        // todo(scaly): is it the public key?
        signed_centralized_public_output: vector<u8>,
        encryptor_ed25519_pubkey: vector<u8>,
        // todo(scaly): is it the public eky?
        centralized_public_output: vector<u8>,
        ctx: &mut TxContext
    ): address {
        // todo(zeev): rename the event fields.
        let session_id = tx_context::fresh_object_address(ctx);
        event::emit(StartDKGSecondRoundEvent {
            session_id,
            initiator: tx_context::sender(ctx),
            first_round_output: first_round_output.output,
            public_key_share_and_proof: centralized_public_key_share_and_proof,
            dwallet_cap_id: object::id(dwallet_cap),
            first_round_session_id,
            encrypted_secret_share_and_proof,
            encryption_key: get_encryption_key(encryption_key),
            encryption_key_id: object::id(encryption_key),
            signed_public_share: signed_centralized_public_output,
            encryptor_ed25519_pubkey,
            dkg_centralized_public_output: centralized_public_output
        });
        session_id
    }


    /// Transfers an encrypted dWallet user secret key share from a source entity to destination entity.
    ///
    /// This function emits an event with the encrypted user secret key share, along with its cryptographic proof,
    /// to the blockchain. The chain verifies that the encrypted data matches the expected secret key share
    /// associated with the dWallet before creating an [`EncryptedUserSecretKeyShare`] object.
    ///
    /// ### Parameters
    /// - **`dwallet`**: A reference to the `DWallet<Secp256K1>` object to which the secret share is linked.
    /// - **`destination_encryption_key`**: A reference to the encryption key used for encrypting the secret key share.
    /// - **`encrypted_secret_share_and_proof`**: The encrypted secret key share, accompanied by a cryptographic proof.
    /// - **`source_signed_centralized_public_output`**: The signed centralized public output corresponding to the secret share.
    /// - **`source_ed25519_pubkey`**: The Ed25519 public key of the source (encryptor) used for verifying the signature.
    ///
    /// ### Effects
    /// - Emits a `StartEncryptedShareVerificationEvent`,
    /// which is captured by the blockchain to initiate the verification process.
    ///
    /// ### Emits
    /// - **`StartEncryptedShareVerificationEvent`**:
    ///   - Includes the encrypted secret share, cryptographic proof,
    ///     encryption key, and additional metadata required for verification.
    public fun transfer_encrypted_user_share(
        dwallet: &DWallet<Secp256K1>,
        destination_encryption_key: &EncryptionKey,
        encrypted_secret_share_and_proof: vector<u8>,
        source_signed_centralized_public_output: vector<u8>,
        source_ed25519_pubkey: vector<u8>,
        ctx: &mut TxContext,
    ) {
        let session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        event::emit(StartEncryptedShareVerificationEvent {
            encrypted_secret_share_and_proof,
            dwallet_centralized_public_output: get_dwallet_centralized_output<Secp256K1>(dwallet),
            dwallet_id: object::id(dwallet),
            encryption_key: get_encryption_key(destination_encryption_key),
            encryption_key_id: object::id(destination_encryption_key),
            session_id,
            signed_public_share: source_signed_centralized_public_output,
            encryptor_ed25519_pubkey: source_ed25519_pubkey,
            initiator: tx_context::sender(ctx),
        });
    }

    /// Creates an encrypted user secret key share after it has been verified by the blockchain.
    ///
    /// This function is invoked by the blockchain to generate an [`EncryptedUserSecretKeyShare`] object
    /// once the associated encryption and cryptographic proofs have been verified.
    /// It finalizes the process by storing the encrypted user share on-chain and emitting the relevant event.
    ///
    /// ### Parameters
    /// - **`dwallet_id`**: The unique identifier of the dWallet associated with the encrypted user share.
    /// - **`encrypted_secret_share_and_proof`**: The encrypted secret share along with its cryptographic proof.
    /// - **`encryption_key_id`**: The unique identifier of the encryption key used for the share.
    /// - **`session_id`**: A unique identifier for the session related to this operation.
    /// - **`signed_public_share`**: The signed public share corresponding to the encrypted secret share.
    /// - **`encryptor_ed25519_pubkey`**: The Ed25519 public key of the encryptor used for signing.
    /// - **`initiator`**: The address of the entity that performed the encryption.
    /// ### Effects
    /// - Creates an `EncryptedUserSecretKeyShare` object and stores it on-chain.
    /// - Transfers the newly created share to the initiator // todo(itay): mistake?
    /// - Emits a `CreatedEncryptedSecretShareEvent` which includes details about the encrypted share,
    ///   cryptographic proofs, and related metadata.
    #[allow(unused_function)]
    public(package) fun create_encrypted_user_share(
        dwallet_id: ID,
        encrypted_secret_share_and_proof: vector<u8>,
        encryption_key_id: ID,
        session_id: ID,
        // todo(zeev): rename
        signed_public_share: vector<u8>,
        encryptor_ed25519_pubkey: vector<u8>,
        // todo(itay): this name is not correct? should be dest?
        initiator: address,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
        let encrypted_user_share = EncryptedUserSecretKeyShare {
            id: object::new(ctx),
            dwallet_id,
            encrypted_secret_share_and_proof,
            encryption_key_id,
            signed_public_share,
            encryptor_ed25519_pubkey,
            encryptor_address: initiator,
        };
        event::emit(CreatedEncryptedSecretShareEvent {
            session_id,
            encrypted_share_obj_id: object::id(&encrypted_user_share),
            dwallet_id,
            encrypted_secret_share_and_proof,
            encryption_key_id,
            signed_public_share,
            encryptor_ed25519_pubkey,
            encryptor_address: initiator,
        });
        transfer::transfer(encrypted_user_share, initiator);
    }

    /// Completes the second round of the Distributed Key Generation (DKG) process and
    /// creates the [`DWallet`].
    ///
    /// This function finalizes the DKG process by creating a `DWallet` object and associating it with the
    /// cryptographic outputs of the second round. It also generates an encrypted user share and emits
    /// events to record the results of the process.
    /// This function is called by the blockchain.
    ///
    /// ### Parameters
    /// - **`initiator`**: The address of the user who initiated the DKG session.
    /// - **`session_id`**: A unique identifier for the current DKG session.
    /// - **`decentralized_output`**: The output of the second round of the DKG process,
    ///      representing the decentralized computation result.
    /// - **`dwallet_cap_id`**: The unique identifier of the `DWalletCap` associated with this session.
    /// - **`dwallet_mpc_network_key_version`**: The version of the MPC network key for the `DWallet`.
    /// - **`encrypted_secret_share_and_proof`**: The encrypted user secret key share and associated cryptographic proof.
    /// - **`encryption_key_id`**: The ID of the `EncryptionKey` used for encrypting the secret key share.
    /// - **`signed_public_share`**: The signed public share corresponding to the secret key share.
    /// - **`encryptor_ed25519_pubkey`**: The Ed25519 public key of the entity that encrypted the secret key share.
    /// - **`centralized_public_output`**: The centralized public output from the DKG process.
    ///
    /// ### Effects
    /// - Creates a new `DWallet` object using the provided session ID, DKG outputs, and other metadata.
    /// - Creates an encrypted user share and associates it with the `DWallet`.
    /// - Emits a `CompletedDKGSecondRoundEvent` to record the completion of the second DKG round.
    /// - Freezes the created `DWallet` object to make it immutable.
    ///
    /// ### Panics
    /// - **`ENotSystemAddress`**: If the function is not called by the system address.
    #[allow(unused_function)]
    fun create_dkg_second_round_output(
        initiator: address,
        session_id: ID,
        decentralized_output: vector<u8>,
        dwallet_cap_id: ID,
        dwallet_mpc_network_key_version: u8,
        encrypted_secret_share_and_proof: vector<u8>,
        encryption_key_id: ID,
        signed_public_share: vector<u8>,
        encryptor_ed25519_pubkey: vector<u8>,
        centralized_public_output: vector<u8>,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);

        let dwallet = dwallet::create_dwallet<Secp256K1>(
            session_id,
            dwallet_cap_id,
            decentralized_output,
            dwallet_mpc_network_key_version,
            centralized_public_output,
            ctx
        );

        create_encrypted_user_share(object::id(&dwallet),
            encrypted_secret_share_and_proof,
            encryption_key_id,
            session_id,
            signed_public_share,
            encryptor_ed25519_pubkey,
            initiator,
            ctx
        );

        event::emit(CompletedDKGSecondRoundEvent {
            session_id,
            initiator,
            dwallet_cap_id,
            dwallet_id: object::id(&dwallet),
            value: decentralized_output,
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
    ): DKGFirstRoundOutput {
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);
        DKGFirstRoundOutput {
            session_id,
            id: object::new(ctx),
            output,
        }
    }

    #[test_only]
    public fun partial_signatures_for_testing(
        presigns: vector<vector<u8>>,
        presign_session_ids: vector<ID>,
        messages: vector<vector<u8>>,
        signatures: vector<vector<u8>>,
        dwallet_id: ID,
        dwallet_cap_id: ID,
        dwallet_mpc_network_key_version: u8,
        ctx: &mut TxContext
    ): PartiallySignedMessages {
        PartiallySignedMessages {
            id: object::new(ctx),
            presigns,
            presign_session_ids,
            messages,
            signatures,
            dwallet_id,
            dwallet_output: vector::empty(),
            dwallet_cap_id,
            dwallet_mpc_network_key_version,
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
            vector::empty(),
            session_id,
            vector::empty(),
            vector::empty(),
            vector::empty(),
            ctx
        );
    }

    /// Starts a batched presign session.
    ///
    /// This function emits a `StartBatchedPresignEvent` for the entire batch and a
    /// `StartPresignFirstRoundEvent` for each presign in the batch. These events signal
    /// validators to begin processing the first round of the presign process for each session.
    /// - A unique `batch_session_id` is generated for the batch.
    /// - A loop creates and emits a `StartPresignFirstRoundEvent` for each session in the batch.
    /// - Each session is linked to the parent batch via `batch_session_id`.
    ///
    /// ### Effects
    /// - Associates the batched presign session with the specified dWallet.
    /// - Emits a `StartBatchedPresignEvent` containing the batch session details.
    /// - Emits a `StartPresignFirstRoundEvent` for each presign in the batch, with relevant details.
    ///
    /// ### Parameters
    /// - `dwallet`: A reference to the target dWallet. This is used to retrieve the dWallet's ID and output.
    /// - `batch_size`: The number of presign sessions to be created in this batch.
    /// - `ctx`: The mutable transaction context, used to generate unique object IDs and retrieve the initiator.
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
                dkg_output: get_dwallet_decentralized_output<Secp256K1>(dwallet),
                batch_session_id,
                dwallet_mpc_network_key_version: get_dwallet_mpc_network_key_version(dwallet),
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
        dwallet_mpc_network_key_version: u8,
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
            dwallet_mpc_network_key_version,
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
        let mpc_network_key_version = 0;
        launch_presign_second_round(
            initiator,
            dwallet_id,
            dkg_output,
            first_round_output,
            first_round_session_id,
            batch_session_id,
            mpc_network_key_version,
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
    /// - **`ECentralizedSignedMessagesAndMessagesLenMismatch`**: If the length of
    ///   `hashed_messages` does not match the length of `centralized_signed_messages`.
    /// - **`EMessageApprovalDWalletMismatch`**: If the DWalletCap ID does not match
    ///   the expected DWalletCap ID for any of the message approvals.
    /// - **`EMissingApprovalOrWrongApprovalOrder`**: If the approved messages are not
    ///   in the same order as the `messages`.
    ///
    /// ### Parameters
    /// - `dwallet_cap`: The capability associated with the dWallet.
    /// - `messages`: The list of messages to be signed.
    /// - `message_approvals`: The approvals for the messages.
    /// - `presign`: The presign object containing intermediate outputs.
    /// - `dwallet`: The dWallet object.
    /// - `centralized_signed_messages`: The list of centralized signatures.
    /// - `presign_session_id`: The session ID of the presign process.
    /// - `ctx`: The mutable transaction context.
    public fun sign(
        message_approvals: &mut vector<MessageApproval>,
        mut messages: vector<vector<u8>>,
        mut presigns: vector<Presign>,
        dwallet: &DWallet<Secp256K1>,
        mut centralized_signed_messages: vector<vector<u8>>,
        ctx: &mut TxContext
    ) {
        let messages_len: u64 = vector::length(&messages);
        let presigns_len: u64 = vector::length(&presigns);
        let approvals_len: u64 = vector::length(message_approvals);
        let centralized_signed_len: u64 = vector::length(&centralized_signed_messages);
        assert!(messages_len == approvals_len, EApprovalsAndMessagesLenMismatch);
        assert!(messages_len == centralized_signed_len, ECentralizedSignedMessagesAndMessagesLenMismatch);
        assert!(messages_len == presigns_len, EPresignsAndMessagesLenMismatch);
        let expected_dwallet_cap_id = get_dwallet_cap_id(dwallet);
        let batch_session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        let mut hashed_messages = hash_messages(message_approvals);
        event::emit(StartBatchedSignEvent {
            session_id: batch_session_id,
            hashed_messages,
            initiator: tx_context::sender(ctx)
        });
        let mut i = 0;
        while (i < approvals_len) {
            let presign = vector::pop_back(&mut presigns);
            assert!(object::id(dwallet) == presign.dwallet_id, EDwalletMismatch);
            let message = vector::pop_back(&mut messages);
            pop_and_verify_message_approval(expected_dwallet_cap_id, message, message_approvals);
            let id = object::id_from_address(tx_context::fresh_object_address(ctx));
            let centralized_signed_message = vector::pop_back(&mut centralized_signed_messages);
            let hashed_message = vector::pop_back(&mut hashed_messages);
            event::emit(StartSignEvent {
                session_id: id,
                presign_session_id: presign.first_round_session_id,
                initiator: tx_context::sender(ctx),
                batched_session_id: batch_session_id,
                dwallet_id: object::id(dwallet),
                presign: presign.presign,
                centralized_signed_message,
                dkg_output: get_dwallet_decentralized_output<Secp256K1>(dwallet),
                hashed_message,
                dwallet_mpc_network_key_version: get_dwallet_mpc_network_key_version<Secp256K1>(dwallet),
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
        initiator: address,
        dwallet_id: ID,
        ctx: &mut TxContext
    ) {
        // Ensure that only the system address can call this function.
        assert!(tx_context::sender(ctx) == SYSTEM_ADDRESS, ENotSystemAddress);

        let sign_output = BatchedSignOutput {
            id: object::new(ctx),
            signatures: signed_messages,
            dwallet_id,
            session_id: batch_session_id
        };
        transfer::transfer(sign_output, initiator);

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
        initiator: address,
        dwallet_id: ID,
        ctx: &mut TxContext
    ) {
        // Call the main create_sign_output function with the provided parameters
        create_sign_output(
            signed_messages,
            batch_session_id,
            initiator,
            dwallet_id,
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
    public(package) fun create_mock_dwallet_for_testing(
        dkg_output: vector<u8>,
        ctx: &mut TxContext
    ): DWallet<Secp256K1> {
        let dwallet_cap = create_dwallet_cap(ctx);
        let dwallet_cap_id = object::id(&dwallet_cap);
        transfer::public_transfer(dwallet_cap, tx_context::sender(ctx));
        let session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        let dwallet_mpc_network_key_version: u8 = 0;
        dwallet::create_dwallet<Secp256K1>(
            session_id,
            dwallet_cap_id,
            dkg_output,
            dwallet_mpc_network_key_version,
            vector[],
            ctx
        )
    }

    // TODO (#493): Remove mock functions before mainnet
    /// Created an immutable [`DWallet`] object with the given DKG output.
    public fun create_mock_dwallet(
        dkg_output: vector<u8>,
        dkg_centralized_output: vector<u8>,
        ctx: &mut TxContext
    ) {
        let dwallet_cap = create_dwallet_cap(ctx);
        let dwallet_cap_id = object::id(&dwallet_cap);
        transfer::public_transfer(dwallet_cap, tx_context::sender(ctx));
        let session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        let dwallet_mpc_network_key_version: u8 = 0;
        let dwallet = dwallet::create_dwallet<Secp256K1>(
            session_id,
            dwallet_cap_id,
            dkg_output,
            dwallet_mpc_network_key_version,
            dkg_centralized_output,
            ctx
        );
        transfer::public_freeze_object(dwallet);
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
            verify_partially_signed_signatures(
                signatures,
                messages,
                presigns_bytes,
                get_dwallet_decentralized_output(dwallet)
            ),
            EInvalidSignatures
        );
        let partial_signatures = PartiallySignedMessages {
            id: object::new(ctx),
            presigns: presigns_bytes,
            presign_session_ids,
            messages,
            signatures,
            dwallet_output: get_dwallet_decentralized_output(dwallet),
            dwallet_id: object::id(dwallet),
            dwallet_cap_id: get_dwallet_cap_id(dwallet),
            dwallet_mpc_network_key_version: get_dwallet_mpc_network_key_version(dwallet),
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
            dwallet_mpc_network_key_version,
        } = partial_signature;
        object::delete(id);
        let message_approvals_len = vector::length(message_approvals);
        let messages_len = vector::length(&messages);
        assert!(message_approvals_len == messages_len, EApprovalsAndMessagesLenMismatch);
        let batch_session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        let mut hashed_messages = hash_messages(message_approvals);
        event::emit(StartBatchedSignEvent {
            session_id: batch_session_id,
            hashed_messages,
            initiator: tx_context::sender(ctx)
        });
        let mut i = 0;
        while (i < message_approvals_len) {
            let message = vector::pop_back(&mut messages);
            pop_and_verify_message_approval(dwallet_cap_id, message, message_approvals);
            let id = object::id_from_address(tx_context::fresh_object_address(ctx));
            let centralized_signed_message = vector::pop_back(&mut signatures);
            let presign = vector::pop_back(&mut presigns);
            let presign_session_id = vector::pop_back(&mut presign_session_ids);
            let hashed_message = vector::pop_back(&mut hashed_messages);
            event::emit(StartSignEvent {
                session_id: id,
                presign_session_id,
                initiator: tx_context::sender(ctx),
                batched_session_id: batch_session_id,
                dwallet_id,
                presign,
                centralized_signed_message,
                dkg_output: dwallet_output,
                hashed_message,
                dwallet_mpc_network_key_version,
            });
            i = i + 1;
        };
    }

    /// Verifies that the user's centralized party signatures are valid.
    native fun verify_partially_signed_signatures(
        partial_signatures: vector<vector<u8>>,
        messages: vector<vector<u8>>,
        presigns: vector<vector<u8>>,
        dkg_output: vector<u8>
    ): bool;
}
