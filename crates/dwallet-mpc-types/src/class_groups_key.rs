use serde::{Deserialize, Serialize};
use crypto_bigint::Uint;
use crypto_bigint::rand_core::{OsRng, SeedableRng};
use class_groups::{construct_knowledge_of_discrete_log_public_parameters_per_crt_prime, construct_setup_parameters_per_crt_prime, generate_keypairs_per_crt_prime, generate_knowledge_of_decryption_key_proofs_per_crt_prime, CompactIbqf, KnowledgeOfDiscreteLogUCProof, CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS, CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS, DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER, MAX_PRIMES};
use fastcrypto::encoding::{Base64, Encoding};

pub type ClassGroupsPublicKeyAndProofBytes = Vec<u8>;
pub type ClassGroupsDecryptionKey = [Uint<CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS>; MAX_PRIMES];
pub type ClassGroupsEncryptionKeyAndProof = [(
    CompactIbqf<CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS>,
    ClassGroupsProof,
); MAX_PRIMES];
#[cfg(feature = "mock-class-groups")]
pub type ClassGroupsProof = [u8;5];

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
        bcs::to_bytes(&self.public()).unwrap()
    }

    pub fn public(&self) -> ClassGroupsEncryptionKeyAndProof {
        self.encryption_key_and_proof.clone()
    }
}

pub fn generate_class_groups_keypair_and_proof_from_seed(
    seed: [u8; 32],
) -> ClassGroupsKeyPairAndProof {
    #[cfg(feature = "mock-class-groups")]
    {
        let decryption_key: [Uint<CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS>; 13] =
            std::array::from_fn(|_| Uint::<CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS>::default());

        let encryption_key_and_proof: [(CompactIbqf<CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS>, [u8;5]); 13] =
            std::array::from_fn(|_| {
                (
                    CompactIbqf::<CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS>::default(),
                    [1,2,3,4,5],
                )
            });

        return ClassGroupsKeyPairAndProof::new(decryption_key, encryption_key_and_proof);
    }

    #[cfg(not(feature = "mock-class-groups"))] {
        let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed);

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

        ClassGroupsKeyPairAndProof::new(decryption_key, encryption_key_and_proof)
    }
}

pub fn write_class_groups_keypair_and_proof_to_file<P: AsRef<std::path::Path> + Clone>(
    keypair: &ClassGroupsKeyPairAndProof,
    path: P,
) -> anyhow::Result<String> {
    let serialized = bcs::to_bytes(keypair)?;
    let contents = Base64::encode(serialized);
    std::fs::write(path.clone(), contents)?;
    read_class_groups_from_file(path)?;
    Ok(Base64::encode(keypair.public_bytes()))
}

pub fn read_class_groups_from_file<P: AsRef<std::path::Path>>(
    path: P,
) -> anyhow::Result<ClassGroupsKeyPairAndProof> {
    let contents = std::fs::read_to_string(path)?;
    let decoded = Base64::decode(contents.as_str()).map_err(|e| anyhow::anyhow!(e))?;
    let keypair: ClassGroupsKeyPairAndProof = bcs::from_bytes(&decoded)?;
    Ok(keypair)
}