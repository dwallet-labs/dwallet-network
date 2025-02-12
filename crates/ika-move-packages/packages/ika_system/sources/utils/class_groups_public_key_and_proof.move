module ika_system::class_groups_public_key_and_proof;

use sui::table_vec;
use sui::transfer;

const NUMBER_OF_KEYS: u64 = 13;

public struct ClassGroupsPublicKeyAndProofBuilder has key, store {
    id: UID,
    public_keys_and_proofs: table_vec::TableVec<vector<u8>>,
}

public struct ClassGroupsPublicKeyAndProof has key, store {
    id: UID,
    public_keys_and_proofs: table_vec::TableVec<vector<u8>>,
}

public fun empty(
    ctx: &mut TxContext,
): ClassGroupsPublicKeyAndProofBuilder {
    let builder = ClassGroupsPublicKeyAndProofBuilder { 
        id: object::new(ctx),
        public_keys_and_proofs:  table_vec::empty(ctx),
    };
    // transfer::transfer(builder, ctx.sender());
    builder 
}

public fun add_public_key_and_proof(
    self: &mut ClassGroupsPublicKeyAndProofBuilder,
    public_key_and_proof: vector<u8>,
) {
    self.public_keys_and_proofs.push_back(public_key_and_proof);
}

public fun finish(
    self: ClassGroupsPublicKeyAndProofBuilder,
    ctx: &mut TxContext,
): ClassGroupsPublicKeyAndProof {
    assert!(self.public_keys_and_proofs.length() == NUMBER_OF_KEYS, 0);
    let ClassGroupsPublicKeyAndProofBuilder {id, public_keys_and_proofs} = self;
    id.delete();
    ClassGroupsPublicKeyAndProof { 
        id: object::new(ctx),
        public_keys_and_proofs 
    }
}

public fun drop(self: ClassGroupsPublicKeyAndProof) {
    let ClassGroupsPublicKeyAndProof { id, mut public_keys_and_proofs } = self;
    while (!public_keys_and_proofs.is_empty()) {
        public_keys_and_proofs.pop_back();
    };
    public_keys_and_proofs.destroy_empty();
    id.delete();
}

public fun destroy(
    self: ClassGroupsPublicKeyAndProof,
): table_vec::TableVec<vector<u8>> {
    let ClassGroupsPublicKeyAndProof { id, public_keys_and_proofs } = self;
    id.delete();
    public_keys_and_proofs
}