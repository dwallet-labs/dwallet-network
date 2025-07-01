use class_groups::publicly_verifiable_secret_sharing::chinese_remainder_theorem::{
    construct_knowledge_of_decryption_key_public_parameters_per_crt_prime,
    construct_setup_parameters_per_crt_prime, generate_keypairs_per_crt_prime,
    generate_knowledge_of_decryption_key_proofs_per_crt_prime, CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS, MAX_PRIMES,
};
use class_groups::{CompactIbqf, DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER};
use crypto_bigint::rand_core::RngCore;
use crypto_bigint::Uint;
use fastcrypto::encoding::{Base64, Encoding};
use group::OsCsRng;
use ika_types::committee::{ClassGroupsEncryptionKeyAndProof, ClassGroupsProof};
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use merlin::Transcript;
use rand_chacha::rand_core::SeedableRng;
use serde::{Deserialize, Serialize};

pub type ClassGroupsDecryptionKey = [Uint<{ CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS }>; MAX_PRIMES];
type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;
pub type DKGDecentralizedOutput =
    <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput;
pub type SingleEncryptionKeyAndProof = (
    CompactIbqf<{ CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS }>,
    ClassGroupsProof,
);
/// The number of primes used in the class groups key,
/// each prime corresponds to a dynamic object.
pub const NUM_OF_CLASS_GROUPS_KEY_OBJECTS: usize = MAX_PRIMES;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RootSeed([u8; RootSeed::SEED_LENGTH]);

impl RootSeed {
    pub const SEED_LENGTH: usize = 32;

    pub fn new(seed: [u8; Self::SEED_LENGTH]) -> Self {
        RootSeed(seed)
    }

    /// Generates a cryptographically secure random seed.
    pub fn random_seed() -> Self {
        let mut bytes = [0u8; Self::SEED_LENGTH];
        OsCsRng.fill_bytes(&mut bytes);
        RootSeed(bytes)
    }

    /// Reads a class group seed (encoded in Base64) from a file.
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> DwalletMPCResult<Self> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| DwalletMPCError::FailedToReadSeed(e.to_string()))?;
        let decoded = Base64::decode(contents.as_str())
            .map_err(|e| DwalletMPCError::FailedToReadSeed(e.to_string()))?;
        Ok(RootSeed::new(decoded.try_into().map_err(|e| {
            DwalletMPCError::FailedToReadSeed(format!("failed to read class group seed: {:?}", e))
        })?))
    }

    /// Writes the seed, encoded in Base64,
    /// to a file and returns the encoded seed string.
    pub fn save_to_file<P: AsRef<std::path::Path> + Clone>(
        &self,
        path: P,
    ) -> DwalletMPCResult<String> {
        let contents = Base64::encode(self.0);
        std::fs::write(path.clone(), contents.clone())
            .map_err(|e| DwalletMPCError::FailedToWriteSeed(e.to_string()))?;
        Ok(contents)
    }

    /// Derive a seed for deterministically generating
    /// this validator's class-groups decryption key and proof [`ClassGroupsKeyPairAndProof`].
    ///
    /// We don't use the root seed directly, as it would be used for other purposes
    /// (such as for generating randomness during an MPC session). Instead, we derive a seed
    /// from it using a distinct hard-coded label.
    fn class_groups_decryption_key_seed(&self) -> [u8; Self::SEED_LENGTH] {
        // Add a distinct descriptive label, and the root seed itself.
        let mut transcript = Transcript::new(b"Class Groups Decryption Key Seed");
        transcript.append_message(b"root seed", &self.0);

        // Generate a new seed from it (internally, it uses a hash function to pseudo-randomly generate it).
        let mut seed: [u8; 32] = [0; 32];
        transcript.challenge_bytes(b"seed", &mut seed);

        seed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassGroupsKeyPairAndProof {
    decryption_key_per_crt_prime: ClassGroupsDecryptionKey,
    encryption_key_and_proof: ClassGroupsEncryptionKeyAndProof,
}

impl ClassGroupsKeyPairAndProof {
    /// Generates a ClassGroupsKeyPairAndProof from a root seed.
    ///
    /// This method deterministically generates class group keys using ChaCha20Rng
    /// seeded with its dedicated seed, that is derived from the provided root seed. The same seed will always produce
    /// the same key pair.
    ///
    /// The seed should be cryptographically secure and kept confidential.
    pub fn from_seed(root_seed: &RootSeed) -> Self {
        let setup_parameters_per_crt_prime =
            construct_setup_parameters_per_crt_prime(DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER)
                .unwrap();
        let language_public_parameters_per_crt_prime =
            construct_knowledge_of_decryption_key_public_parameters_per_crt_prime(
                setup_parameters_per_crt_prime.each_ref(),
            )
            .unwrap();

        let seed = root_seed.class_groups_decryption_key_seed();
        let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed);
        let decryption_key =
            generate_keypairs_per_crt_prime(setup_parameters_per_crt_prime.clone(), &mut rng)
                .unwrap();

        let encryption_key_and_proof = generate_knowledge_of_decryption_key_proofs_per_crt_prime(
            language_public_parameters_per_crt_prime.clone(),
            decryption_key,
            &mut rng,
        )
        .unwrap();

        ClassGroupsKeyPairAndProof {
            decryption_key_per_crt_prime: decryption_key,
            encryption_key_and_proof,
        }
    }

    pub fn encryption_key_and_proof(&self) -> ClassGroupsEncryptionKeyAndProof {
        // Safe to unwrap because the serialization should never fail.
        self.encryption_key_and_proof.clone()
    }

    pub fn decryption_key(&self) -> ClassGroupsDecryptionKey {
        self.decryption_key_per_crt_prime
    }
}
