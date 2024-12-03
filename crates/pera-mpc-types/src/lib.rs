use crypto_bigint::Uint;
use serde::{Deserialize, Serialize};

// Todo (#): Change types to real types once the class groups keygen is ready and doesn't take forever
// currently using dummy type for the proof as it has no default implemented yet
pub type MockProof = Vec<u8>;
pub type ClassGroupsPublicKeyAndProofBytes = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassGroupsPublicKeyAndProof(
    class_groups::CompactIbqf<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>,
    MockProof,
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassGroupsKeyPairAndProof(
    Uint<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>,
    class_groups::CompactIbqf<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>,
    MockProof,
);

impl ClassGroupsKeyPairAndProof {
    pub fn public_bytes(&self) -> ClassGroupsPublicKeyAndProofBytes {
        bcs::to_bytes(&self.public()).unwrap()
    }

    pub fn public(&self) -> ClassGroupsPublicKeyAndProof {
        ClassGroupsPublicKeyAndProof(self.1.clone(), self.2.clone())
    }
}

pub fn generate_class_groups_keypair_and_proof_from_seed(
    seed: [u8; 32],
) -> ClassGroupsKeyPairAndProof {
    let secret_key_share = Uint::from_u8(1);
    let public_key_share = class_groups::CompactIbqf::default();
    let proof = vec![1u8; 32];
    // Todo (#): Uncomment this lines once the class groups keygen is ready and doesn't take forever
    // let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed);
    // let _ = class_groups::dkg::proof_helpers::generate_secret_share_sized_keypair_and_proof(rng);
    ClassGroupsKeyPairAndProof(secret_key_share, public_key_share, proof)
}
