use tiresias::decryption_key_share::PublicParameters as DecryptionPublicParameters;
use std::collections::HashMap;
use group::PartyID;
use tiresias::{AdjustedLagrangeCoefficientSizedNumber, DecryptionKeyShare, EncryptionKey, PaillierModulusSizedNumber};
use twopc_mpc::secp256k1::paillier::bulletproofs::{ProtocolPublicParameters, PublicNonceEncryptedPartialSignatureAndProof, SignatureThresholdDecryptionParty};
use ecdsa::Signature;
use homomorphic_encryption::{AdditivelyHomomorphicDecryptionKeyShare, GroupsPublicParametersAccessors};
use twopc_mpc::paillier::PLAINTEXT_SPACE_SCALAR_LIMBS;
use crate::twopc_mpc_protocols::ProtocolContext;

pub type PartialDecryptionProof = <DecryptionKeyShare as AdditivelyHomomorphicDecryptionKeyShare<PLAINTEXT_SPACE_SCALAR_LIMBS, EncryptionKey>>::PartialDecryptionProof;
pub type DecryptionShare = <DecryptionKeyShare as AdditivelyHomomorphicDecryptionKeyShare<PLAINTEXT_SPACE_SCALAR_LIMBS, EncryptionKey>>::DecryptionShare;

pub struct DecryptionError {
    pub failed_messages_indices: Vec<usize>,
    pub involved_parties: Vec<PartyID>,
    pub decryption_shares: Vec<(HashMap<PartyID, DecryptionShare>, HashMap<PartyID, DecryptionShare>)>,
}

pub fn decrypt_signature_decentralized_party_sign(
    messages: Vec<Vec<u8>>,
    decryption_key_share_public_parameters: DecryptionPublicParameters,
    decryption_shares: HashMap<PartyID, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>>,
    public_nonce_encrypted_partial_signature_and_proofs: Vec<PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>>,
    signature_threshold_decryption_round_parties: Vec<SignatureThresholdDecryptionParty>,
) -> Result<Vec<Vec<u8>>, DecryptionError> {
    let protocol_public_parameters = ProtocolPublicParameters::new(*decryption_key_share_public_parameters.encryption_scheme_public_parameters.plaintext_space_public_parameters().modulus);
    let decrypters: Vec<_> = decryption_shares.keys().take(decryption_key_share_public_parameters.threshold.into()).copied().collect();

    let decryption_shares: Vec<(HashMap<_, _>, HashMap<_, _>)> = (0..public_nonce_encrypted_partial_signature_and_proofs.len())
        .map(|i| {
            decryption_shares
                .iter()
                .filter(|(party_id, _)| decrypters.contains(party_id))
                .map(|(party_id, decryption_share)| {
                    let (partial_signature_decryption_shares, masked_nonce_decryption_shares) = decryption_share[i].clone();
                    (
                        (*party_id, partial_signature_decryption_shares),
                        (*party_id, masked_nonce_decryption_shares),
                    )
                })
                .unzip()
        })
        .collect();

    let lagrange_coefficients: HashMap<PartyID, AdjustedLagrangeCoefficientSizedNumber> = decrypters
        .clone()
        .into_iter()
        .map(|j| {
            (
                j,
                DecryptionKeyShare::compute_lagrange_coefficient(
                    j,
                    decryption_key_share_public_parameters.number_of_parties,
                    decrypters.clone(),
                    &decryption_key_share_public_parameters,
                ),
            )
        })
        .collect();

    let mut failed_messages_indices = Vec::new();
    let messages_signatures: Vec<Vec<u8>> = signature_threshold_decryption_round_parties.into_iter()
        .zip(
            messages.into_iter()
                .zip(public_nonce_encrypted_partial_signature_and_proofs.into_iter())
                .zip(decryption_shares.iter()))
        .enumerate()
        .map(|(index, (signature_threshold_decryption_round_party, ((message, public_nonce_encrypted_partial_signature_and_proof), (partial_signature_decryption_shares, masked_nonce_decryption_shares))))| {
            let result = signature_threshold_decryption_round_party.decrypt_signature(
                lagrange_coefficients.clone(),
                partial_signature_decryption_shares.clone(),
                masked_nonce_decryption_shares.clone(),
            );

            match result {
                Ok((nonce_x_coordinate, signature_s)) => {
                    let signature_s_inner: k256::Scalar = signature_s.into();
                    Signature::<k256::Secp256k1>::from_scalars(k256::Scalar::from(nonce_x_coordinate), signature_s_inner).unwrap().to_vec()
                }
                Err(_) => {
                    failed_messages_indices.push(index);
                    Vec::new()
                }
            }
        })
        .collect();

    if !failed_messages_indices.is_empty() {
        return Err(DecryptionError {
            failed_messages_indices,
            involved_parties: decrypters,
            decryption_shares,
        });
    }

    Ok(messages_signatures)
}
