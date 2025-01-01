use class_groups::{
    construct_knowledge_of_decryption_key_public_parameters_per_crt_prime,
    construct_setup_parameters_per_crt_prime, generate_keypairs_per_crt_prime,
    generate_knowledge_of_decryption_key_proofs_per_crt_prime, CompactIbqf,
    KnowledgeOfDiscreteLogUCProof, CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS, DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER, MAX_PRIMES,
};
use crypto_bigint::Uint;
use fastcrypto::encoding::{Base64, Encoding};
use rand_chacha::rand_core::SeedableRng;
use serde::{Deserialize, Serialize};

// Todo (#369): Change types to real types once the class groups
// Todo (#369): keygen is ready and doesn't take forever
pub type MockProof = Vec<u8>;
pub type ClassGroupsPublicKeyAndProofBytes = Vec<u8>;
pub type ClassGroupsDecryptionKey = [Uint<{ CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS }>; MAX_PRIMES];
pub type ClassGroupsEncryptionKeyAndProof = [(
    CompactIbqf<{ CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS }>,
    ClassGroupsProof,
); MAX_PRIMES];
#[cfg(feature = "mock-class-groups")]
pub type ClassGroupsProof = [u8; 5];
#[cfg(not(feature = "mock-class-groups"))]
pub type ClassGroupsProof = KnowledgeOfDiscreteLogUCProof;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassGroupsKeyPairAndProof {
    decryption_key: ClassGroupsDecryptionKey,
    encryption_key_and_proof: ClassGroupsEncryptionKeyAndProof,
}

impl ClassGroupsKeyPairAndProof {
    pub fn new(
        decryption_key: ClassGroupsDecryptionKey,
        encryption_key_and_proof: ClassGroupsEncryptionKeyAndProof,
    ) -> Self {
        Self {
            decryption_key,
            encryption_key_and_proof,
        }
    }

    pub fn public_bytes(&self) -> ClassGroupsPublicKeyAndProofBytes {
        // Safe to unwrap because the serialization should never fail.
        bcs::to_bytes(&self.public()).unwrap()
    }

    pub fn public(&self) -> ClassGroupsEncryptionKeyAndProof {
        self.encryption_key_and_proof.clone()
    }
}

/// Generates a class groups key pair, and proof that
/// the generated public key is a class groups key.
/// The keypair is generated from a given seed
/// by initiating a seed-based random number generator with it.
pub fn generate_class_groups_keypair_and_proof_from_seed(
    _seed: [u8; 32],
) -> ClassGroupsKeyPairAndProof {
    // Todo (#369): Uncomment this lines once the class groups
    // Todo (#369): keygen is ready and doesn't take forever
    // let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed);
    // let _ = class_groups::dkg::proof_helpers::generate_secret_share_sized_keypair_and_proof(rng);
    let decryption_key: [Uint<CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS>; 13] =
        std::array::from_fn(|_| Uint::<CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS>::default());

    let encryption_key_and_proof: [(CompactIbqf<CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS>, [u8; 5]);
        13] = std::array::from_fn(|_| {
        (
            CompactIbqf::<CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS>::default(),
            [1, 2, 3, 4, 5],
        )
    });

    return ClassGroupsKeyPairAndProof::new(decryption_key, encryption_key_and_proof);
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassGroupsKeyPairAndProofReal {
    decryption_key: ClassGroupsDecryptionKey,
    pub encryption_key_and_proof: [(
        CompactIbqf<CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS>,
        KnowledgeOfDiscreteLogUCProof,
    ); MAX_PRIMES],
}
pub fn read_class_groups_private_key_from_file_real<P: AsRef<std::path::Path>>(
    path: P,
) -> anyhow::Result<ClassGroupsDecryptionKey> {
    let contents = std::fs::read_to_string(path)?;
    let decoded = Base64::decode(contents.as_str()).map_err(|e| anyhow::anyhow!(e))?;
    let keypair: ClassGroupsKeyPairAndProofReal = bcs::from_bytes(&decoded)?;
    Ok(keypair.decryption_key)
}
