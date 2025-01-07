// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// This module defines the core data structures and functions for
/// working with dWallets in the pera system.
///
/// ## Overview
///
/// - A **dWallet** (`DWallet`) represents a wallet that is created after the Distributed Key Generation (DKG) process.
///   It encapsulates the session ID, capability ID, and the output of the DKG's second round.
/// - A **dWallet capability** (`DWalletCap`) represents a capability that grants
///   ownership and control over a corresponding `DWallet`.
///
/// ## Key Concepts
///
/// - **DWallet**: A generic wallet structure with a phantom type `T`.
/// - **DWalletCap**: A capability object granting control over a specific dWallet.
/// - **Session ID**: A unique identifier for the DKG session.
module pera_system::dwallet {
    use pera::table::{Self, Table};
    use pera::ed25519::ed25519_verify;

    const CLASS_GROUPS: u8 = 0;

    // >>>>>>>>>>>>>>> Error Codes >>>>>>>>>>>>>>

    const EInvalidEncryptionKeyScheme: u64 = 0;
    const EInvalidEncryptionKeySignature: u64 = 1;
    const EInvalidEncryptionKeyOwner: u64 = 2;

    // <<<<<<<<<<<<<<< Error Codes <<<<<<<<<<<<<<

    #[allow(unused_field)]
    /// `DWallet` represents a wallet that is created after the DKG process.
    ///
    /// ### Fields
    /// - `id`: The unique identifier for the dWallet object.
    /// - `session_id`: The ID of the session that generated this dWallet.
    /// - `dwallet_cap_id`: The ID of the dWallet capability associated with this wallet.
    /// - `output`: The output of the second DKG round, represented as a `vector<u8>`.
    public struct DWallet<phantom T> has key, store {
        id: UID,
        session_id: ID,
        dwallet_cap_id: ID,
        // The output of the second DKG round.
        output: vector<u8>,
        dwallet_mpc_network_key_version: u8,
    }

    /// An Additively Homomorphic Encryption (AHE) public key
    /// that can be used to encrypt a user share in order to prove to the network that
    /// the recipient can sign with a dWallet when it is transferred or access is granted to it.
    public struct EncryptionKey has key {
        id: UID,
        scheme: u8,
        encryption_key: vector<u8>,
        key_owner_address: address,
        encryption_key_signature: vector<u8>,
    }

    /// `DWalletCap` holder controls a corresponding `DWallet`.
    ///
    /// ### Fields
    /// - `id`: The unique identifier for the dWallet capability object.
    public struct DWalletCap has key, store {
        id: UID,
    }

    /// A generic function to create a new [`DWallet`] object of type `T`.
    ///
    /// ### Parameters
    /// - `session_id`: The ID of the session that generates this dWallet.
    /// - `dwallet_cap_id`: The ID of the dWallet capability associated with this dWallet.
    /// - `output`: The output of the second DKG round, represented as a `vector<u8>`.
    /// - `ctx`: A mutable transaction context used to create the dWallet object.
    ///
    /// ### Returns
    /// A new [`DWallet`] object of the specified type `T`.
    public(package) fun create_dwallet<T: drop>(
        session_id: ID,
        dwallet_cap_id: ID,
        output: vector<u8>,
        dwallet_mpc_network_key_version: u8,
        ctx: &mut TxContext
    ): DWallet<T> {
        DWallet<T> {
            id: object::new(ctx),
            session_id,
            dwallet_cap_id,
            output,
            dwallet_mpc_network_key_version,
        }
    }

    /// Shared object that holds the active encryption keys per user.
    /// 'encryption_keys' is a key-value table where the key is the user address
    /// and the value is the encryption key object ID.
    public struct ActiveEncryptionKeys has key {
        id: UID,
        encryption_keys: Table<address, ID>,
    }

    /// Create a shared object that holds the active encryption keys per user.
    public fun create_active_encryption_keys(ctx: &mut TxContext) {
        transfer::share_object(ActiveEncryptionKeys {
            id: object::new(ctx),
            encryption_keys: table::new(ctx),
        });
    }

    /// Get the active encryption key ID by user adderss.
    public fun get_active_encryption_key(
        encryption_key_holder: &ActiveEncryptionKeys,
        key_owner: address,
    ): &ID {
        table::borrow(&encryption_key_holder.encryption_keys, key_owner)
    }

    /// Set the active encryption key for a user (the sender).
    public fun upsert_active_encryption_key(
        encryption_key_holder: &mut ActiveEncryptionKeys,
        encryption_key: &EncryptionKey,
        ctx: &mut TxContext
    ) {
        assert!(encryption_key.key_owner_address == tx_context::sender(ctx), EInvalidEncryptionKeyOwner);
        if (table::contains(&encryption_key_holder.encryption_keys, encryption_key.key_owner_address)) {
            table::remove(&mut encryption_key_holder.encryption_keys, encryption_key.key_owner_address);
        };
        table::add(
            &mut encryption_key_holder.encryption_keys,
            encryption_key.key_owner_address,
            object::id(encryption_key)
        );
    }

    /// Register an encryption key to encrypt a user share.
    /// The key is saved as an immutable object.
    public fun register_encryption_key(
        key: vector<u8>,
        signature: vector<u8>,
        sender_sui_pubkey: vector<u8>,
        scheme: u8,
        ctx: &mut TxContext
    ) {
        assert!(is_valid_encryption_key_scheme(scheme), EInvalidEncryptionKeyScheme);
        assert!(ed25519_verify(&signature, &sender_sui_pubkey, &key), EInvalidEncryptionKeySignature);
        // TODO (#453): Verify the ed2551 public key matches the sender's address.
        transfer::freeze_object(EncryptionKey {
            id: object::new(ctx),
            scheme,
            encryption_key: key,
            key_owner_address: tx_context::sender(ctx),
            encryption_key_signature: signature,
        });
    }

    /// Create a new [`DWalletCap`] object.
    ///
    /// The holder of the `DWalletCap` has control and ownership over
    /// the associated `DWallet`.
    ///
    /// ### Parameters
    /// - `ctx`: A mutable transaction context used to create the `DWalletCap` object.
    ///
    /// ### Returns
    /// The newly created `DWalletCap` object.
    public(package) fun create_dwallet_cap(ctx: &mut TxContext): DWalletCap {
        DWalletCap {
            id: object::new(ctx),
        }
    }

    /// Retrieve the ID of the `DWalletCap` associated with a given dWallet.
    ///
    /// ### Parameters
    /// - `dwallet`: A reference to the [`DWallet`] object whose capability ID is to be retrieved.
    ///
    /// ### Returns
    /// The ID of the `DWalletCap` associated with the provided dWallet.
    public(package) fun get_dwallet_cap_id<T: drop>(dwallet: &DWallet<T>): ID {
        dwallet.dwallet_cap_id
    }

    /// Retrieve the output of the second DKG round for a given dWallet.
    ///
    /// ### Parameters
    /// - `dwallet`: A reference to the [`DWallet`] object whose DKG output is to be retrieved.
    ///
    /// ### Returns
    /// A `vector<u8>` representing the output of the second DKG round for the specified dWallet.
    public(package) fun get_dwallet_output<T: drop>(dwallet: &DWallet<T>): vector<u8> {
        dwallet.output
    }

    fun is_valid_encryption_key_scheme(scheme: u8): bool {
        scheme == CLASS_GROUPS
    }
}
