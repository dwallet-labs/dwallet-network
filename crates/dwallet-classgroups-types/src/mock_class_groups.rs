use crate::ClassGroupsDecryptionKey;
use class_groups::{
    CompactIbqf, KnowledgeOfDiscreteLogUCProof, CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS, MAX_PRIMES,
};
use fastcrypto::encoding::{Base64, Encoding};
use group::PartyID;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ClassGroupsProof = [u8; 5];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CGKeyPairAndProofForMockFromFile {
    pub(crate) decryption_key: ClassGroupsDecryptionKey,
    pub encryption_key_and_proof: [(
        CompactIbqf<CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS>,
        KnowledgeOfDiscreteLogUCProof,
    ); MAX_PRIMES],
}

pub type CGEncryptionKeyAndProofForMockFromFile = [(
    CompactIbqf<{ CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS }>,
    KnowledgeOfDiscreteLogUCProof,
); MAX_PRIMES];

/// Mocks the Class Group encryption keys and proofs of all validators.
/// Reads the generated key from a file and return a map from `PartyID` to
/// the encryption key and proof.
pub fn mock_cg_encryption_keys_and_proofs() -> DwalletMPCResult<
    HashMap<
        PartyID,
        [(
            CompactIbqf<{ CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS }>,
            KnowledgeOfDiscreteLogUCProof,
        ); MAX_PRIMES],
    >,
> {
    let contents = std::fs::read_to_string("class-groups-mock-key")
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    let decoded = Base64::decode(contents.as_str())
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    let keypair: CGKeyPairAndProofForMockFromFile = bcs::from_bytes(&decoded)?;

    let mut encryption_keys_and_proofs = HashMap::new();
    (1..=4).for_each(|i| {
        encryption_keys_and_proofs.insert(i as PartyID, keypair.encryption_key_and_proof.clone());
    });
    Ok(encryption_keys_and_proofs)
}

/// Mocks the class groups decryption key, by reading generated key from a file.
pub fn mock_cg_private_key() -> DwalletMPCResult<ClassGroupsDecryptionKey> {
    let contents = std::fs::read_to_string("class-groups-mock-key")
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    let decoded = Base64::decode(contents.as_str())
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))?;
    let keypair: CGKeyPairAndProofForMockFromFile = bcs::from_bytes(&decoded)?;

    Ok(keypair.decryption_key)
}
