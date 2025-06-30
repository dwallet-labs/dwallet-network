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
pub struct RootSeed(pub [u8; RootSeed::SEED_LENGTH]);

impl RootSeed {
    pub const SEED_LENGTH: usize = 32;

    pub fn new(seed: [u8; Self::SEED_LENGTH]) -> Self {
        RootSeed(seed)
    }

    pub fn seed(&self) -> [u8; Self::SEED_LENGTH] {
        self.0
    }

    /// Generates a cryptographically secure random seed.
    pub fn random_seed() -> Self {
        let mut bytes = [0u8; Self::SEED_LENGTH];
        OsCsRng.fill_bytes(&mut bytes);
        RootSeed(bytes)
    }
}

// impl ClassGroupsDecryptionKey {
//     pub fn from_seed(
//         seed: RootSeed,
//     ) -> Self {
//         let setup_parameters_per_crt_prime =
//             construct_setup_parameters_per_crt_prime(DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER).unwrap();
//
//         let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed.seed());
//         ClassGroupsDecryptionKey(generate_keypairs_per_crt_prime(setup_parameters_per_crt_prime.clone(), &mut rng).unwrap())
//     }
//
//     pub fn decryption_key(&self) -> [Uint<{ CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS }>; MAX_PRIMES] {
//         self.0
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassGroupsKeyPairAndProof {
    #[serde(with = "group::helpers::const_generic_array_serialization")]
    decryption_key_per_crt_prime: ClassGroupsDecryptionKey,
    #[serde(with = "group::helpers::const_generic_array_serialization")]
    encryption_key_and_proof: ClassGroupsEncryptionKeyAndProof,
}

/// Contains the public keys of the DWallet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, Hash)]
pub struct DWalletPublicKeys {
    pub centralized_public_share: Vec<u8>,
    pub decentralized_public_share: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl ClassGroupsKeyPairAndProof {
    pub fn from_seed(seed: &RootSeed) -> Self {
        let setup_parameters_per_crt_prime =
            construct_setup_parameters_per_crt_prime(DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER)
                .unwrap();
        let language_public_parameters_per_crt_prime =
            construct_knowledge_of_decryption_key_public_parameters_per_crt_prime(
                setup_parameters_per_crt_prime.each_ref(),
            )
            .unwrap();

        let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed.0);
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

/// A wrapper around `ClassGroupsKeyPairAndProof` that ensures the deserialized value
/// is constructed directly on the heap via `Box`, avoiding large stack allocations.
///
/// # Why This Exists
///
/// In debug builds, Rust has a significantly smaller stack size compared to release builds.
/// Deserializing a large or deeply nested struct like `ClassGroupsKeyPairAndProof` directly
/// can lead to a stack overflow if it's first constructed on the stack before being boxed.
///
/// By wrapping the struct inside a `Box` field (`inner`) within this wrapper, we allow the
/// deserializer (`bcs::from_bytes`) to allocate the entire structure directly on the heap,
/// bypassing the stack and preventing overflow.
#[derive(Deserialize)]
struct ClassGroupsKeyPairAndProofWrapper {
    inner: Box<ClassGroupsKeyPairAndProof>,
}

/// Reads a class group key pair and proof (encoded in Base64) from a file.
pub fn read_class_groups_from_file<P: AsRef<std::path::Path>>(
    path: P,
) -> DwalletMPCResult<Box<ClassGroupsKeyPairAndProof>> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    let decoded = Base64::decode(contents.as_str())
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    let keypair: ClassGroupsKeyPairAndProofWrapper = bcs::from_bytes(&decoded)?;
    Ok(keypair.inner)
}

/// Writes a class group key seed, encoded in Base64,
/// to a file and returns the encoded seed string.
pub fn write_class_groups_seed_to_file<P: AsRef<std::path::Path> + Clone>(
    seed: [u8; RootSeed::SEED_LENGTH],
    path: P,
) -> DwalletMPCResult<String> {
    let contents = Base64::encode(seed);
    std::fs::write(path.clone(), contents.clone())
        .map_err(|e| DwalletMPCError::FailedToWriteCGKey(e.to_string()))?;
    Ok(contents)
}

/// Reads a class group seed (encoded in Base64) from a file.
pub fn read_class_groups_seed_from_file<P: AsRef<std::path::Path>>(
    path: P,
) -> DwalletMPCResult<RootSeed> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    let decoded = Base64::decode(contents.as_str())
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    Ok(RootSeed::new(decoded.try_into().map_err(|e| {
        DwalletMPCError::FailedToReadCGKey(format!("failed to read class group seed: {:?}", e))
    })?))
}

pub mod class_groups_as_base64 {
    use super::*;
    use base64::engine::general_purpose;
    use base64::Engine;
    use serde::de::Error;
    use serde::{Deserializer, Serializer};
    use std::sync::Arc;

    pub fn serialize<S>(
        value: &Arc<impl serde::Serialize>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = bcs::to_bytes(&**value).map_err(serde::ser::Error::custom)?;
        let encoded = general_purpose::STANDARD.encode(bytes);
        serializer.serialize_str(&encoded)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Arc<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: serde::de::DeserializeOwned,
    {
        let encoded = String::deserialize(deserializer)?;
        let bytes = general_purpose::STANDARD
            .decode(&encoded)
            .map_err(Error::custom)?;
        let value: T = bcs::from_bytes(&bytes).map_err(Error::custom)?;
        Ok(Arc::new(value))
    }
}
