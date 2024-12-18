pub mod dwallet_mpc;

use class_groups::{KnowledgeOfDiscreteLogUCProof, construct_knowledge_of_discrete_log_public_parameters_per_crt_prime, construct_setup_parameters_per_crt_prime, generate_keypairs_per_crt_prime, generate_knowledge_of_decryption_key_proofs_per_crt_prime, CompactIbqf, DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
    MAX_PRIMES,
    CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};
use crypto_bigint::rand_core::OsRng;
use crypto_bigint::Uint;
use group::secp256k1;
use rand_chacha::rand_core::SeedableRng;
use serde::{Deserialize, Serialize};

// Todo (#369): Change types to real types once the class groups keygen is ready and doesn't take forever
pub type ClassGroupsPublicKeyAndProofBytes = Vec<u8>;
pub type ClassGroupsDecryptionKey = [Uint<CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS>; MAX_PRIMES];
pub type ClassGroupsEncryptionKeyAndProof = [(
    CompactIbqf<CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS>,
    KnowledgeOfDiscreteLogUCProof,
); MAX_PRIMES];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassGroupsKeyPairAndProof {
    decryption_key: ClassGroupsDecryptionKey,
    encryption_key_and_proof: ClassGroupsEncryptionKeyAndProof,
}

impl ClassGroupsKeyPairAndProof {
    pub fn public_bytes(&self) -> ClassGroupsPublicKeyAndProofBytes {
        bcs::to_bytes(&self.public()).unwrap()
    }

    pub fn public(&self) -> ClassGroupsEncryptionKeyAndProof {
        self.encryption_key_and_proof.clone()
    }
}

pub fn generate_class_groups_keypair_and_proof_from_seed(
    seed: [u8; 32],
) -> ClassGroupsKeyPairAndProof {
    // let decryption_key = Uint::from_u8(1);
    // let encryption_key = class_groups::CompactIbqf::default();
    // let proof = vec![1u8; 32];
    // Todo (#369): Uncomment this lines once the class groups keygen is ready and doesn't take forever
    let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed);
    // let _ = class_groups::dkg::proof_helpers::generate_secret_share_sized_keypair_and_proof(rng);

    let plaintext_space_public_parameters = secp256k1::scalar::PublicParameters::default();

    let setup_parameters_per_crt_prime =
        construct_setup_parameters_per_crt_prime(DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER).unwrap();
    let language_public_parameters_per_crt_prime =
        construct_knowledge_of_discrete_log_public_parameters_per_crt_prime(
            setup_parameters_per_crt_prime.each_ref(),
        )
        .unwrap();

    let decryption_key =
        generate_keypairs_per_crt_prime(setup_parameters_per_crt_prime.clone(), &mut rng)
            .unwrap();

    let encryption_key_and_proof = generate_knowledge_of_decryption_key_proofs_per_crt_prime(
        language_public_parameters_per_crt_prime.clone(),
        decryption_key,
        &mut OsRng,
    )
    .unwrap();

    ClassGroupsKeyPairAndProof {
        decryption_key,
        encryption_key_and_proof,
    }
}
