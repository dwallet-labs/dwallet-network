module ika_system::class_groups_public_key_and_proof;

use sui::table_vec;

const NUMBER_OF_KEYS: u64 = 13;

public struct ClassGroupsPublicKeyAndProofBuilder has key, store {
    id: UID,
    public_keys_and_proofs: table_vec::TableVec<vector<u8>>,
}

public struct ClassGroupsPublicKeyAndProof has store {
    public_keys_and_proofs: table_vec::TableVec<vector<u8>>,
}

public fun empty(
    ctx: &mut TxContext,
): ClassGroupsPublicKeyAndProofBuilder {
    ClassGroupsPublicKeyAndProofBuilder { 
        id: object::new(ctx),
        public_keys_and_proofs:  table_vec::empty(ctx),
    }
}

public fun add_public_key_and_proof(
    self: &mut ClassGroupsPublicKeyAndProofBuilder,
    public_key_and_proof: vector<u8>,
) {
    self.public_keys_and_proofs.push_back(public_key_and_proof);
}

public fun finish(
    self: ClassGroupsPublicKeyAndProofBuilder,
): ClassGroupsPublicKeyAndProof {
    assert!(self.public_keys_and_proofs.length() == NUMBER_OF_KEYS, 0);
    let ClassGroupsPublicKeyAndProofBuilder {id, public_keys_and_proofs} = self;
    id.delete();
    ClassGroupsPublicKeyAndProof { 
        public_keys_and_proofs 
    }
}

public fun drop(self: ClassGroupsPublicKeyAndProof) {
    let ClassGroupsPublicKeyAndProof { mut public_keys_and_proofs } = self;
    while (!public_keys_and_proofs.is_empty()) {
        public_keys_and_proofs.pop_back();
    };
    public_keys_and_proofs.destroy_empty();
}

public fun destroy(
    self: ClassGroupsPublicKeyAndProof,
): table_vec::TableVec<vector<u8>> {
    let ClassGroupsPublicKeyAndProof { public_keys_and_proofs } = self;
    public_keys_and_proofs
}