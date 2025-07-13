// Copyright (c) dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// This module defines the data structures for the Class Groups public key and proof.
/// 
/// The full Class Groups public key consists of 13 public keys, each with a corresponding proof.
/// Due to Sui's limitations on object size (≤ 250KB) and transaction size (≤ 128KB), 
/// the full key must be split into parts and stored dynamically using `table_vec`.
module ika_common::class_groups_public_key_and_proof;

// === Imports ===

use sui::table_vec;

// === Structs ===

/// `ClassGroupsPublicKeyAndProofBuilder` is used to construct a `ClassGroupsPublicKeyAndProof` object.
public struct ClassGroupsPublicKeyAndProofBuilder has key, store {
    id: UID,
    /// A `TableVec` that dynamically stores public keys and their corresponding proofs.
    public_keys_and_proofs: table_vec::TableVec<vector<u8>>,
}

/// `ClassGroupsPublicKeyAndProof` stores the full Class Groups public key and proof.
/// This object can only be created using `ClassGroupsPublicKeyAndProofBuilder`.
public struct ClassGroupsPublicKeyAndProof has key, store {
    id: UID,
    /// A `TableVec` that dynamically stores public keys and their corresponding proofs.
    public_keys_and_proofs: table_vec::TableVec<vector<u8>>,
}

// === Public Functions ===

/// Creates a new `ClassGroupsPublicKeyAndProofBuilder` instance.
public fun empty(
    ctx: &mut TxContext,
): ClassGroupsPublicKeyAndProofBuilder {
    ClassGroupsPublicKeyAndProofBuilder { 
        id: object::new(ctx),
        public_keys_and_proofs: table_vec::empty(ctx),
    }
}

/// Adds a public key and its corresponding proof to the `ClassGroupsPublicKeyAndProofBuilder`.
///
/// Due to Sui's transaction argument size limit (≤ 16KB), each public key-proof pair
/// must be split into two parts before being stored.
public fun add_public_key_and_proof(
    self: &mut ClassGroupsPublicKeyAndProofBuilder,
    public_key_and_proof_first_part: vector<u8>,
    public_key_and_proof_second_part: vector<u8>,
) {
    let mut full_public_key_and_proof = vector::empty();
    full_public_key_and_proof.append(public_key_and_proof_first_part);
    full_public_key_and_proof.append(public_key_and_proof_second_part);
    self.public_keys_and_proofs.push_back(full_public_key_and_proof);
}

/// Finalizes the construction of a `ClassGroupsPublicKeyAndProof` object.
public fun finish(
    self: ClassGroupsPublicKeyAndProofBuilder,
    ctx: &mut TxContext,
): ClassGroupsPublicKeyAndProof {
    let ClassGroupsPublicKeyAndProofBuilder { id, public_keys_and_proofs } = self;
    id.delete();
    ClassGroupsPublicKeyAndProof { 
        id: object::new(ctx),
        public_keys_and_proofs 
    }
}

/// Drops the `ClassGroupsPublicKeyAndProof` object, removing all public keys and proofs before deletion.
public fun drop(self: ClassGroupsPublicKeyAndProof) {
    let ClassGroupsPublicKeyAndProof { id, mut public_keys_and_proofs } = self;
    while (!public_keys_and_proofs.is_empty()) {
        public_keys_and_proofs.pop_back();
    };
    public_keys_and_proofs.destroy_empty();
    id.delete();
}

/// Destroys the `ClassGroupsPublicKeyAndProof` object, returning the stored public keys and proofs.
///
/// This function removes the object from storage and returns the `TableVec` containing 
/// the public keys and their corresponding proofs.
public fun destroy(
    self: ClassGroupsPublicKeyAndProof,
): table_vec::TableVec<vector<u8>> {
    let ClassGroupsPublicKeyAndProof { id, public_keys_and_proofs } = self;
    id.delete();
    public_keys_and_proofs
}  
