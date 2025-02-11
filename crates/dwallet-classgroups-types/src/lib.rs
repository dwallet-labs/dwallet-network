#[cfg(feature = "mock-class-groups")]
pub mod mock_class_groups;

#[cfg(not(feature = "mock-class-groups"))]
use class_groups::{
    construct_knowledge_of_decryption_key_public_parameters_per_crt_prime,
    construct_setup_parameters_per_crt_prime, generate_keypairs_per_crt_prime,
    generate_knowledge_of_decryption_key_proofs_per_crt_prime, CompactIbqf,
    KnowledgeOfDiscreteLogUCProof, CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS, DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER, MAX_PRIMES,
};
#[cfg(feature = "mock-class-groups")]
use class_groups::{
    CompactIbqf, CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS, CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    MAX_PRIMES,
};
use class_groups::KnowledgeOfDiscreteLogUCProof;
use crypto_bigint::Uint;
use fastcrypto::encoding::{Base64, Encoding};
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
// #[cfg(feature = "mock-class-groups")]
// use mock_class_groups::ClassGroupsProof;
use rand_chacha::rand_core::SeedableRng;
use serde::{Deserialize, Serialize};
use crate::mock_class_groups::CGKeyPairAndProofForMockFromFile;

// #[cfg(not(feature = "mock-class-groups"))]
pub type ClassGroupsProof = KnowledgeOfDiscreteLogUCProof;
pub type ClassGroupsPublicKeyAndProofBytes = Vec<u8>;
pub type ClassGroupsDecryptionKey = [Uint<{ CRT_FUNDAMENTAL_DISCRIMINANT_LIMBS }>; MAX_PRIMES];
pub type ClassGroupsEncryptionKeyAndProof = [(
    CompactIbqf<{ CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS }>,
    ClassGroupsProof,
); MAX_PRIMES];
type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;
pub type DKGDecentralizedOutput =
    <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassGroupsKeyPairAndProof {
    #[serde(with = "group::helpers::const_generic_array_serialization")]
    decryption_key_per_crt_prime: ClassGroupsDecryptionKey,
    #[serde(with = "group::helpers::const_generic_array_serialization")]
    encryption_key_and_proof: ClassGroupsEncryptionKeyAndProof,
}

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
/// When using the `mock-class-groups` feature, this function will return a mock keypair and proof.
/// Note that the mock feature is **only** for development and testing purposes.
pub fn generate_class_groups_keypair_and_proof_from_seed(
    seed: [u8; 32],
) -> ClassGroupsKeyPairAndProof {
    #[cfg(feature = "mock-class-groups")]
    {

        let contents = std::fs::read_to_string("class-groups-mock-key")
            .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string())).unwrap();
        let decoded = Base64::decode(contents.as_str())
            .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string())).unwrap();
        let keypair: CGKeyPairAndProofForMockFromFile = bcs::from_bytes(&decoded).unwrap();

        return ClassGroupsKeyPairAndProof::new(keypair.decryption_key, keypair.encryption_key_and_proof);
    }

    #[cfg(not(feature = "mock-class-groups"))]
    {
        let setup_parameters_per_crt_prime =
            construct_setup_parameters_per_crt_prime(DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER)
                .unwrap();
        let language_public_parameters_per_crt_prime =
            construct_knowledge_of_decryption_key_public_parameters_per_crt_prime(
                setup_parameters_per_crt_prime.each_ref(),
            )
            .unwrap();

        let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed);
        let decryption_key =
            generate_keypairs_per_crt_prime(setup_parameters_per_crt_prime.clone(), &mut rng)
                .unwrap();

        let encryption_key_and_proof = generate_knowledge_of_decryption_key_proofs_per_crt_prime(
            language_public_parameters_per_crt_prime.clone(),
            decryption_key,
        )
        .unwrap();

        ClassGroupsKeyPairAndProof::new(decryption_key, encryption_key_and_proof)
    }
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
) -> DwalletMPCResult<ClassGroupsKeyPairAndProof> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    let decoded = Base64::decode(contents.as_str())
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    Ok(bcs::from_bytes(&decoded)?)
}

/// Contains the public keys of the DWallet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, Hash)]
pub struct DWalletPublicKeys {
    pub centralized_public_share: Vec<u8>,
    pub decentralized_public_share: Vec<u8>,
    pub public_key: Vec<u8>,
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
