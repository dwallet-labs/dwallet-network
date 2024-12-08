use crypto_bigint::Uint;
use serde::{Deserialize, Serialize};

// Todo (#369): Change types to real types once the class groups keygen is ready and doesn't take forever
pub type MockProof = Vec<u8>;
pub type ClassGroupsPublicKeyAndProofBytes = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassGroupsPublicKeyAndProof {
    pub encryption_key:
        class_groups::CompactIbqf<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>,
    pub proof: MockProof,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassGroupsKeyPairAndProof {
    decryption_key: Uint<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>,
    encryption_key:
        class_groups::CompactIbqf<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>,
    proof: MockProof,
}

impl ClassGroupsKeyPairAndProof {
    pub fn public_bytes(&self) -> ClassGroupsPublicKeyAndProofBytes {
        bcs::to_bytes(&self.public()).unwrap()
    }

    pub fn public(&self) -> ClassGroupsPublicKeyAndProof {
        ClassGroupsPublicKeyAndProof {
            encryption_key: self.encryption_key.clone(),
            proof: self.proof.clone(),
        }
    }
}

pub fn generate_class_groups_keypair_and_proof_from_seed(
    seed: [u8; 32],
) -> ClassGroupsKeyPairAndProof {
    let decryption_key = Uint::from_u8(1);
    let encryption_key = class_groups::CompactIbqf::default();
    let proof = vec![1u8; 32];
    // Todo (#369): Uncomment this lines once the class groups keygen is ready and doesn't take forever
    // let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed);
    // let _ = class_groups::dkg::proof_helpers::generate_secret_share_sized_keypair_and_proof(rng);
    ClassGroupsKeyPairAndProof {
        encryption_key,
        decryption_key,
        proof,
    }
}