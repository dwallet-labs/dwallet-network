use class_groups::KnowledgeOfDiscreteLogUCProof;
use class_groups::{
    construct_knowledge_of_decryption_key_public_parameters_per_crt_prime,
    construct_setup_parameters_per_crt_prime, generate_keypairs_per_crt_prime,
    generate_knowledge_of_decryption_key_proofs_per_crt_prime, CompactIbqf,
    CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS, CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER, MAX_PRIMES,
};
use crypto_bigint::rand_core::{OsRng, RngCore};
use crypto_bigint::Uint;
use dwallet_mpc_types::dwallet_mpc::ClassGroupsPublicKeyAndProofBytes;
use fastcrypto::encoding::{Base64, Encoding};
use fastcrypto::traits::{FromUniformBytes, ToFromBytes};
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use serde::{Deserialize, Serialize};

pub type ClassGroupsProof = KnowledgeOfDiscreteLogUCProof;
pub type ClassGroupsDecryptionKey = [Uint<{ CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS }>; MAX_PRIMES];
pub type ClassGroupsEncryptionKeyAndProof = [(
    CompactIbqf<{ CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS }>,
    ClassGroupsProof,
); MAX_PRIMES];
type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;
pub type DKGDecentralizedOutput =
    <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput;
pub type SingleEncryptionKeyAndProof = (
    CompactIbqf<{ CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS }>,
    ClassGroupsProof,
);
pub const NUM_OF_CLASS_GROUPS_KEYS: usize = MAX_PRIMES;
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

pub const RNG_SEED_SIZE: usize = 32;

impl ClassGroupsKeyPairAndProof {
    pub fn new(
        decryption_key: ClassGroupsDecryptionKey,
        encryption_key_and_proof: ClassGroupsEncryptionKeyAndProof,
    ) -> Self {
        Self {
            decryption_key_per_crt_prime: decryption_key,
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

    pub fn decryption_key(&self) -> ClassGroupsDecryptionKey {
        self.decryption_key_per_crt_prime.clone()
    }
}

/// Generate a class groups keypair and proof from a seed.
pub fn generate_class_groups_keypair_and_proof_from_seed(
    seed: [u8; RANDOM_SEED_SIZE],
) -> ClassGroupsKeyPairAndProof {
    let setup_parameters_per_crt_prime =
        construct_setup_parameters_per_crt_prime(DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER).unwrap();
    let language_public_parameters_per_crt_prime =
        construct_knowledge_of_decryption_key_public_parameters_per_crt_prime(
            setup_parameters_per_crt_prime.each_ref(),
        )
        .unwrap();

    let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed);
    let decryption_key =
        generate_keypairs_per_crt_prime(setup_parameters_per_crt_prime.clone(), &mut rng).unwrap();

    let encryption_key_and_proof = generate_knowledge_of_decryption_key_proofs_per_crt_prime(
        language_public_parameters_per_crt_prime.clone(),
        decryption_key,
    )
    .unwrap();

    ClassGroupsKeyPairAndProof::new(decryption_key, encryption_key_and_proof)
}

/// Generates a cryptographically secure random seed for class groups key generation.
pub fn sample_seed() -> [u8; RANDOM_SEED_SIZE] {
    let mut bytes = [0u8; RANDOM_SEED_SIZE];
    OsRng.fill_bytes(&mut bytes);
    bytes
}

/// Writes a class group key pair and proof, encoded in Base64,
/// to a file and returns the public key.
pub fn write_class_groups_keypair_and_proof_to_file<P: AsRef<std::path::Path> + Clone>(
    keypair: &ClassGroupsKeyPairAndProof,
    path: P,
) -> DwalletMPCResult<String> {
    let serialized = bcs::to_bytes(keypair)?;
    let contents = Base64::encode(serialized);
    std::fs::write(path.clone(), contents)
        .map_err(|e| DwalletMPCError::FailedToWriteCGKey(e.to_string()))?;
    Ok(Base64::encode(keypair.public_bytes()))
}

/// Reads a class group key pair and proof (encoded in Base64) from a file.
pub fn read_class_groups_from_file<P: AsRef<std::path::Path>>(
    path: P,
) -> DwalletMPCResult<Box<ClassGroupsKeyPairAndProof>> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    let decoded = Base64::decode(contents.as_str())
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    let keypair: Box<ClassGroupsKeyPairAndProof> = Box::new(bcs::from_bytes(&decoded)?);
    Ok(keypair)
}

/// Writes a class group key seed, encoded in Base64,
/// to a file and returns the encoded seed string.
pub fn write_class_groups_seed_to_file<P: AsRef<std::path::Path> + Clone>(
    seed: [u8; RANDOM_SEED_SIZE],
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
) -> DwalletMPCResult<[u8; RANDOM_SEED_SIZE]> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    let decoded = Base64::decode(contents.as_str())
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    Ok(decoded.try_into().map_err(|e| {
        DwalletMPCError::FailedToReadCGKey(format!("failed to read class group seed: {:?}", e))
    })?)
}

/// Extracts [`DWalletPublicKeys`] from the given [`DKGDecentralizedOutput`].
// Can't use the TryFrom trait as it leads to conflicting implementations.
// Must use `anyhow::Result`, because this function is being used also in the centralized party crate.
pub fn public_keys_from_dkg_output(
    value: DKGDecentralizedOutput,
) -> anyhow::Result<DWalletPublicKeys> {
    Ok(DWalletPublicKeys {
        centralized_public_share: bcs::to_bytes(&value.centralized_party_public_key_share)?,
        decentralized_public_share: bcs::to_bytes(&value.public_key_share)?,
        public_key: bcs::to_bytes(&value.public_key)?,
    })
}
