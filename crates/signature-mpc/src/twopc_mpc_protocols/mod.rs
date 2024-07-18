// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::{HashMap, HashSet};

pub use commitment::{Commitment};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use k256::{AffinePoint, CompressedPoint, elliptic_curve, sha2};
pub use group::PartyID;
use k256::sha2::Digest;
use crypto_bigint::{ U256};
use ecdsa::{elliptic_curve::{ops::Reduce}, hazmat::{bits2field, DigestPrimitive}, RecoveryId, Signature, VerifyingKey};
use ecdsa::signature::DigestVerifier;
pub use enhanced_maurer::language::EnhancedLanguageStatementAccessors;
pub use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use k256::elliptic_curve::group::GroupEncoding;

use group::{secp256k1, GroupElement as _, AffineXCoordinate};
pub use group::Value;
use homomorphic_encryption::{
    AdditivelyHomomorphicDecryptionKey,
    AdditivelyHomomorphicEncryptionKey, GroupsPublicParametersAccessors,
};
use k256::{ sha2::digest::FixedOutput};
pub use proof::aggregation::{
    CommitmentRoundParty, DecommitmentRoundParty, ProofAggregationRoundParty, ProofShareRoundParty,
};
pub use tiresias::{
    decryption_key_share::PublicParameters as DecryptionPublicParameters,
    encryption_key::PublicParameters as EncryptionPublicParameters,
    test_exports::deal_trusted_shares as tiresias_deal_trusted_shares, DecryptionKeyShare,
    LargeBiPrimeSizedNumber, PaillierModulusSizedNumber, SecretKeyShareSizedNumber,
    AdjustedLagrangeCoefficientSizedNumber
};
use tiresias::{EncryptionKey, ProtocolError};
use twopc_mpc::paillier::PLAINTEXT_SPACE_SCALAR_LIMBS;
pub use twopc_mpc::secp256k1::{SCALAR_LIMBS, GroupElement, Scalar};
pub use twopc_mpc::secp256k1::paillier::bulletproofs::{
    CentralizedPartyPresign, DecommitmentProofVerificationRoundParty, DKGCommitmentRoundParty, EncryptionOfSecretKeyShareRoundParty, PresignCommitmentRoundParty, ProtocolPublicParameters, SignatureHomomorphicEvaluationParty, SignatureNonceSharesCommitmentsAndBatchedProof, SignaturePartialDecryptionParty, SignatureThresholdDecryptionParty,  DKGCentralizedPartyOutput,
    PublicKeyShareDecommitmentAndProof, PublicNonceEncryptedPartialSignatureAndProof, SecretKeyShareEncryptionAndProof, DKGDecentralizedPartyOutput, DecentralizedPartyPresign, EncDHCommitment, EncDHCommitmentRoundParty,EncDHProofAggregationRoundParty, EncDHProofShareRoundParty, EncDLCommitment, EncDHDecommitment, EncDLDecommitment, EncDHProofShare, EncDLProofShare,
    EncDLCommitmentRoundParty,  EncDLDecommitmentRoundParty, EncryptedMaskedNoncesRoundParty, EncryptedMaskedKeyShareRoundParty, PresignDecentralizedPartyOutput,
    EncDLProofAggregationOutput, EncDLProofAggregationRoundParty,EncDLProofShareRoundParty, EncryptedMaskAndMaskedNonceShare, EncryptedNonceShareAndPublicShare, EncDHDecommitmentRoundParty, EncDHProofAggregationOutput, DKGDecommitmentRoundParty, DKGDecommitmentRoundState};

pub use twopc_mpc::{Error, Result};
pub use twopc_mpc::secp256k1::paillier::bulletproofs::{PresignProofVerificationRoundParty, SignaturePartialDecryptionProofParty, SignaturePartialDecryptionProofVerificationParty, SignatureVerificationParty};
use twopc_mpc::secp256k1::paillier::bulletproofs::{DecryptionShare, PartialDecryptionProof};

pub type InitSignatureMPCProtocolSequenceNumber = u64;
pub type SignatureMPCRound = u64;
pub type SignatureMPCMessageKind = u64;
pub type SignatureMPCTimestamp = u64;
pub type PublicKeyValue = group::Value<GroupElement>;
pub type SignatureK256Secp256k1 = Signature<k256::Secp256k1>;

struct PublicParameters {
    tiresias_public_parameters: tiresias::encryption_key::PublicParameters
}

#[derive(Clone, Debug)]
pub enum Hash {
    KECCAK256 = 0,
    SHA256 = 1
}

impl From<u8> for Hash {
    fn from(value: u8) -> Self {
        match value  {
            0 => Self::KECCAK256,
            1 => Self::SHA256,
            _ => panic!()
        }
    }
}

impl From<Hash> for u8 {
    fn from(value: Hash) -> Self {
        match value  {
            Hash::KECCAK256 => 0,
            Hash::SHA256 => 1
        }
    }
}

pub fn initiate_centralized_party_dkg(//tiresias_public_parameters: &str, epoch: EpochId, party_id: PartyID, threshold: PartyID, number_of_parties: PartyID, session_id: SignatureMpcSessionID
) -> twopc_mpc::Result<DKGCommitmentRoundParty<ProtocolContext>> {
    pub const N: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");

    let protocol_public_parameters = ProtocolPublicParameters::new(N);

    Ok(DKGCommitmentRoundParty::new(protocol_public_parameters, PhantomData))
}

pub fn decommitment_round_centralized_party_dkg(
    state: DKGDecommitmentRoundState<ProtocolContext>
) -> twopc_mpc::Result<DKGDecommitmentRoundParty<ProtocolContext>> {
    pub const N: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");

    let protocol_public_parameters = ProtocolPublicParameters::new(N);

    DKGDecommitmentRoundParty::new(protocol_public_parameters, state, PhantomData)
}

// -------------------------------------------------------------------------------------------------
// DKG Decentralized Party Parties
// -------------------------------------------------------------------------------------------------

pub fn initiate_decentralized_party_dkg(
    decryption_key_share_public_parameters: <DecryptionKeyShare as AdditivelyHomomorphicDecryptionKeyShare<PLAINTEXT_SPACE_SCALAR_LIMBS, EncryptionKey>>::PublicParameters,
    //epoch: EpochId,
    party_id: PartyID,
    parties: HashSet<PartyID>,
    //session_id: SignatureMPCSessionID,
) -> twopc_mpc::Result<EncryptionOfSecretKeyShareRoundParty<ProtocolContext>> {
    let protocol_public_parameters = ProtocolPublicParameters::new(*decryption_key_share_public_parameters.encryption_scheme_public_parameters.plaintext_space_public_parameters().modulus);

    Ok(EncryptionOfSecretKeyShareRoundParty::new(
        protocol_public_parameters, party_id,
        decryption_key_share_public_parameters.threshold,
        parties,
        PhantomData,
    ))
}

pub fn decentralized_party_dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(
    tiresias_public_parameters: &str,
    commitment_to_centralized_party_secret_key_share: Commitment,
    centralized_party_public_key_share_decommitment_and_proof: PublicKeyShareDecommitmentAndProof<ProtocolContext>,
    secret_key_share_encryption_and_proof: SecretKeyShareEncryptionAndProof<ProtocolContext>,
) -> twopc_mpc::Result<(DKGDecentralizedPartyOutput, Vec<u8>)> {
    let protocol_public_parameters = ProtocolPublicParameters::new(LargeBiPrimeSizedNumber::from_be_hex(tiresias_public_parameters));

    let decommitment_proof_verification_round_party =
        DecommitmentProofVerificationRoundParty::new(protocol_public_parameters, commitment_to_centralized_party_secret_key_share, PhantomData);
    let output = decommitment_proof_verification_round_party
        .verify_decommitment_and_proof_of_centralized_party_public_key_share(
            centralized_party_public_key_share_decommitment_and_proof,
            secret_key_share_encryption_and_proof,
        )?;
    let public_key: AffinePoint = output.public_key.into();

    Ok((output, public_key.to_bytes().to_vec()))
}

pub fn initiate_centralized_party_presign(
    dkg_output: DKGCentralizedPartyOutput,
) -> twopc_mpc::Result<PresignCommitmentRoundParty<ProtocolContext>> {
    pub const N: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");

    let protocol_public_parameters = ProtocolPublicParameters::new(N);

    PresignCommitmentRoundParty::new(
        PhantomData::<()>,
        protocol_public_parameters,
        dkg_output,
    )
}

pub fn finalize_centralized_party_presign(
    dkg_output: DKGCentralizedPartyOutput,
    signature_nonce_shares_and_commitment_randomnesses: Vec<(Scalar, Scalar)>,
) -> twopc_mpc::Result<PresignProofVerificationRoundParty<ProtocolContext>> {
    pub const N: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");

    let protocol_public_parameters = ProtocolPublicParameters::new(N);

    PresignProofVerificationRoundParty::new(
        signature_nonce_shares_and_commitment_randomnesses,
        PhantomData::<()>,
        protocol_public_parameters,
        dkg_output,
    )
}

pub fn finalize_centralized_party_sign(
    messages: Vec<Scalar>,
    dkg_output: DKGCentralizedPartyOutput,
    public_nonce_encrypted_partial_signature_and_proofs: Vec<PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>>,
    signatures_s: Vec<Scalar>
) -> twopc_mpc::Result<()> {
    pub const N: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");

    let protocol_public_parameters = ProtocolPublicParameters::new(N);

    let parties = messages.into_iter().map(|message|     SignatureVerificationParty::new(
        message,
        dkg_output.public_key,
        &protocol_public_parameters.group_public_parameters,
    )).collect::<Result<Vec<_>>>()?;

    public_nonce_encrypted_partial_signature_and_proofs.into_iter().zip(signatures_s.into_iter()).zip(parties.into_iter()).map(|((public_nonce_encrypted_partial_signature_and_proof, signature_s,), party)|
        GroupElement::new(public_nonce_encrypted_partial_signature_and_proof.public_nonce, &protocol_public_parameters.group_public_parameters).map_err(Error::from).and_then(|public_nonce| party.verify_signature(public_nonce.x(), signature_s))
    ).collect()
}

pub fn verify_signature(
    messages: Vec<Vec<u8>>,
    hash: &Hash,
    public_key: PublicKeyValue,
    signatures: Vec<Vec<u8>>
) -> bool {
    messages.into_iter().zip(signatures).map(|(message, signature)| {
        match hash {
            Hash::KECCAK256 => {
                let signature: Signature<k256::Secp256k1> =
                    Signature::from_slice(signature.as_slice()).unwrap();
                let verifying_key = VerifyingKey::<k256::Secp256k1>::from_affine(public_key.into()).unwrap();
                verifying_key.verify_digest(sha3::Keccak256::new_with_prefix(message), &signature)
            },
            Hash::SHA256 => {
                let signature: Signature<k256::Secp256k1> =
                    Signature::from_slice(signature.as_slice()).unwrap();
                let verifying_key = VerifyingKey::<k256::Secp256k1>::from_affine(public_key.into()).unwrap();
                verifying_key.verify_digest(sha2::Sha256::new_with_prefix(message), &signature)
            }
        }
    }).collect::<ecdsa::Result<Vec<_>>>().is_ok()
}

pub fn new_decentralized_party_presign_batch(
    parties: HashSet<PartyID>,
    commitments_and_proof_to_centralized_party_nonce_shares: SignatureNonceSharesCommitmentsAndBatchedProof<ProtocolContext>,
    encrypted_mask_and_masked_key_shares: Vec<EncryptedMaskAndMaskedNonceShare>,
    individual_encrypted_nonce_shares_and_public_shares: HashMap<PartyID, Vec<Value<EncryptedNonceShareAndPublicShare>>>,
    encrypted_nonce_share_and_public_shares: Vec<
        EncryptedNonceShareAndPublicShare,
    >,
    individual_encrypted_masked_nonce_shares: HashMap<PartyID, Vec<Value<EncryptedMaskAndMaskedNonceShare>>>,
    encrypted_masked_nonce_shares: Vec<EncryptedMaskAndMaskedNonceShare>,
) -> twopc_mpc::Result<Vec<DecentralizedPartyPresign>> {
    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();

    DecentralizedPartyPresign::new_batch::<
        SCALAR_LIMBS,
        PLAINTEXT_SPACE_SCALAR_LIMBS,
        GroupElement,
        EncryptionKey,
        ProtocolContext,
    >(
        parties.clone(),
        commitments_and_proof_to_centralized_party_nonce_shares,
        encrypted_mask_and_masked_key_shares,
        individual_encrypted_nonce_shares_and_public_shares,
        encrypted_nonce_share_and_public_shares,
        individual_encrypted_masked_nonce_shares,
        encrypted_masked_nonce_shares,
        &secp256k1_group_public_parameters,
    )
}

pub type EncryptedDecentralizedPartySecretKeyShare = tiresias::CiphertextSpaceGroupElement;
pub type EncryptedDecentralizedPartySecretKeyShareValue =
<tiresias::CiphertextSpaceGroupElement as group::GroupElement>::Value;

pub fn initiate_decentralized_party_presign(
    decryption_key_share_public_parameters: <DecryptionKeyShare as AdditivelyHomomorphicDecryptionKeyShare<PLAINTEXT_SPACE_SCALAR_LIMBS, EncryptionKey>>::PublicParameters,
    //epoch: EpochId,
    party_id: PartyID,
    parties: HashSet<PartyID>,
    //session_id: SignatureMPCSessionID,
    dkg_output: DKGDecentralizedPartyOutput,
) -> twopc_mpc::Result<EncryptedMaskedKeyShareRoundParty<ProtocolContext>> {
    let protocol_public_parameters = ProtocolPublicParameters::new(*decryption_key_share_public_parameters.encryption_scheme_public_parameters.plaintext_space_public_parameters().modulus);

    EncryptedMaskedKeyShareRoundParty::new(
        party_id,
        decryption_key_share_public_parameters.threshold,
        parties,
        PhantomData::<()>,
        protocol_public_parameters,
        dkg_output,
    )
}

pub fn initiate_centralized_party_sign(
    dkg_output: DKGCentralizedPartyOutput,
    presigns: Vec<CentralizedPartyPresign>,
) -> twopc_mpc::Result<Vec<SignatureHomomorphicEvaluationParty<ProtocolContext>>> {
    pub const N: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");

    let protocol_public_parameters = ProtocolPublicParameters::new(N);

    presigns
        .into_iter()
        .map(|presign| {
            SignatureHomomorphicEvaluationParty::new(
                PhantomData::<()>,
                protocol_public_parameters.clone(),
                dkg_output.clone(),
                presign,
            )
        })
        .collect()
}

pub fn initiate_decentralized_party_sign(
    //tiresias_public_parameters: &str, epoch: EpochId, party_id: PartyID, threshold: PartyID, number_of_parties: PartyID, session_id: SignatureMpcSessionID
    tiresias_key_share_decryption_key_share: SecretKeyShareSizedNumber,
    decryption_key_share_public_parameters: <DecryptionKeyShare as AdditivelyHomomorphicDecryptionKeyShare<PLAINTEXT_SPACE_SCALAR_LIMBS, EncryptionKey>>::PublicParameters,
    //epoch: EpochId,
    party_id: PartyID,
    parties: HashSet<PartyID>,
    //session_id: SignatureMPCSessionID,
    dkg_output: DKGDecentralizedPartyOutput,
    presigns: Vec<DecentralizedPartyPresign>,
) -> twopc_mpc::Result<Vec<SignaturePartialDecryptionParty<ProtocolContext>>> {
    let protocol_public_parameters = ProtocolPublicParameters::new(*decryption_key_share_public_parameters.encryption_scheme_public_parameters.plaintext_space_public_parameters().modulus);

    let decryption_key_share = DecryptionKeyShare::new(
        party_id,
        tiresias_key_share_decryption_key_share,
        &decryption_key_share_public_parameters,
    )?;

    presigns
        .into_iter()
        .map(|presign| {
            SignaturePartialDecryptionParty::new(
                decryption_key_share_public_parameters.threshold,
                decryption_key_share.clone(),
                decryption_key_share_public_parameters.clone(),
                PhantomData,
                protocol_public_parameters.clone(),
                dkg_output.clone(),
                presign,
            )
        })
        .collect()
}

pub fn decentralized_party_sign_verify_encrypted_signature_parts_prehash(
    tiresias_public_parameters: &str,
    messages: Vec<Vec<u8>>,
    public_nonce_encrypted_partial_signature_and_proofs: Vec<PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>>,
    dkg_output: DKGDecentralizedPartyOutput,
    presigns: Vec<DecentralizedPartyPresign>,
    hash: Hash,
) -> twopc_mpc::Result<()> {
    let protocol_public_parameters = ProtocolPublicParameters::new(LargeBiPrimeSizedNumber::from_be_hex(tiresias_public_parameters));

    messages.into_iter().zip(public_nonce_encrypted_partial_signature_and_proofs.into_iter()).zip(presigns
        .into_iter())

        .map(|((message, public_nonce_encrypted_partial_signature_and_proofs), presign)| {
            let m = message_digest(&message, &hash);
            SignaturePartialDecryptionParty::verify_encrypted_signature_parts_prehash(
                m,
                public_nonce_encrypted_partial_signature_and_proofs,
                &PhantomData::<()>,
                &protocol_public_parameters.scalar_group_public_parameters,
                &protocol_public_parameters.group_public_parameters,
                &protocol_public_parameters.encryption_scheme_public_parameters,
                &protocol_public_parameters.unbounded_dcom_eval_witness_public_parameters,
                &protocol_public_parameters.range_proof_dcom_eval_public_parameters,
                dkg_output.clone(),
                presign,
                &mut OsRng
            )
        })
        .collect()
}

/// Returned when the signature decryption fails & contains all the necessary information to
/// start an Identifiable abort round.
pub struct DecryptionError {
    // The indices of the messages that their decryption failed out of the current messages batch.
    // We sign on a batch of messages at each time.
    pub failed_messages_indices: Vec<usize>,

    // The IDs of the parties that we used to decrypt the signature. We need only threshold of them to
    // decrypt the signature, and we communicate them to the other parties, so they'll know they should
    // use their decryption shares to find the malicious parties.
    pub involved_parties: Vec<PartyID>,
}

fn generate_signatures(
    lagrange_coefficients: HashMap<PartyID, AdjustedLagrangeCoefficientSizedNumber>,
    decryption_shares: Vec<(
        HashMap<PartyID, PaillierModulusSizedNumber>,
        HashMap<PartyID, PaillierModulusSizedNumber>,
    )>,
    public_nonce_encrypted_partial_signature_and_proofs: Vec<
        PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>,
    >,
    signature_threshold_decryption_round_parties: Vec<SignatureThresholdDecryptionParty>,
    messages: Vec<Vec<u8>>,
) -> std::result::Result<Vec<Vec<u8>>, Vec<usize>> {
    let mut failed_messages_indices = Vec::new();
    let messages_signatures: Vec<Vec<u8>> = signature_threshold_decryption_round_parties
        .into_iter()
        .zip(
            messages
                .into_iter()
                .zip(public_nonce_encrypted_partial_signature_and_proofs.into_iter())
                .zip(decryption_shares.iter()),
        )
        .enumerate()
        .map(
            |(
                 index,
                 (
                     signature_threshold_decryption_round_party,
                     (
                         (message, public_nonce_encrypted_partial_signature_and_proof),
                         (partial_signature_decryption_shares, masked_nonce_decryption_shares),
                     ),
                 ),
             )| {
                let result = signature_threshold_decryption_round_party.decrypt_signature(
                    lagrange_coefficients.clone(),
                    partial_signature_decryption_shares.clone(),
                    masked_nonce_decryption_shares.clone(),
                );

                match result {
                    Ok((nonce_x_coordinate, signature_s)) => {
                        let signature_s_inner: k256::Scalar = signature_s.into();
                        Signature::<k256::Secp256k1>::from_scalars(
                            k256::Scalar::from(nonce_x_coordinate),
                            signature_s_inner,
                        )
                            .unwrap()
                            .to_vec()
                    }
                    Err(_) => {
                        failed_messages_indices.push(index);
                        Vec::new()
                    }
                }
            },
        )
        .collect();

    if !failed_messages_indices.is_empty() {
        return Err(failed_messages_indices);
    }
    Ok(messages_signatures)
}

pub fn decrypt_signature_decentralized_party_sign(
    messages: Vec<Vec<u8>>,
    decryption_key_share_public_parameters: DecryptionPublicParameters,
    decryption_shares: HashMap<PartyID, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>>,
    public_nonce_encrypted_partial_signature_and_proofs: Vec<PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>>,
    signature_threshold_decryption_round_parties: Vec<SignatureThresholdDecryptionParty>,
) -> std::result::Result<Vec<Vec<u8>>, DecryptionError> {
    let protocol_public_parameters = ProtocolPublicParameters::new(*decryption_key_share_public_parameters.encryption_scheme_public_parameters.plaintext_space_public_parameters().modulus);

    // TODO: choose multiple?
    let decrypters: Vec<PartyID> = decryption_shares.keys().take(decryption_key_share_public_parameters.threshold.into()).copied().collect();

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

    let lagrange_coefficients: HashMap<
        PartyID,
        AdjustedLagrangeCoefficientSizedNumber,
    > = decrypters
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

    let messages_signatures_result = generate_signatures(
        lagrange_coefficients,
        decryption_shares.clone(),
        public_nonce_encrypted_partial_signature_and_proofs,
        signature_threshold_decryption_round_parties,
        messages,
    );

    match messages_signatures_result {
        Ok(messages_signatures) => Ok(messages_signatures),
        Err(failed_messages_indices) => Err(DecryptionError {
            failed_messages_indices,
            involved_parties: decrypters,
        }),
    }
}

 /// Identify the parties that acted maliciously while signing this specific message, after all the parties
 /// that were involved in decrypting it have sent a proof that they behaved honestly.
 ///
 /// In case one of the involved parties failed to generate a proof, only this party will be returned.
 /// If all the involved parties sent a proof, and some of the proofs are invalid, all the parties that
 /// their proof was invalid will be returned.
pub fn identify_message_malicious_parties(
    verification_round_party: SignaturePartialDecryptionProofVerificationParty,
    partial_signature_decryption_shares: HashMap<PartyID, DecryptionShare>,
    masked_nonce_decryption_shares: HashMap<PartyID, DecryptionShare>,
    decryption_key_share_public_parameters: DecryptionPublicParameters,
    signature_partial_decryption_proofs: HashMap<PartyID, PartialDecryptionProof>,
    decrypters: Vec<PartyID>,
) -> Vec<PartyID>
{
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

    let error = verification_round_party.identify_malicious_decrypters(
        lagrange_coefficients,
        partial_signature_decryption_shares,
        masked_nonce_decryption_shares,
        signature_partial_decryption_proofs,
        &mut OsRng,
    );

    match error {
        Error::Tiresias(tiresias::Error::ProtocolError(
                            ProtocolError::ProofVerificationError {
                                malicious_parties,
                                ..
                            })) => malicious_parties,
        _ => {
            panic!(error);
        }
    }
}

pub fn generate_proof(
    decryption_key_share_public_parameters: DecryptionPublicParameters,
    decryption_key_share: DecryptionKeyShare,
    designated_decrypting_party_id: PartyID,
    presign: DecentralizedPartyPresign,
    encryption_scheme_public_parameters: <EncryptionKey as AdditivelyHomomorphicEncryptionKey<
        PLAINTEXT_SPACE_SCALAR_LIMBS,
    >>::PublicParameters,
    public_nonce_encrypted_partial_signature_and_proof: PublicNonceEncryptedPartialSignatureAndProof<
        ProtocolContext,
    >,
) -> (PartialDecryptionProof, SignaturePartialDecryptionProofVerificationParty) {
    let proof_party = SignaturePartialDecryptionProofParty::new(
        decryption_key_share_public_parameters.threshold,
        designated_decrypting_party_id,
        decryption_key_share,
        decryption_key_share_public_parameters,
        presign,
        encryption_scheme_public_parameters,
        public_nonce_encrypted_partial_signature_and_proof,
    )
        .unwrap();

    proof_party
        .prove_correct_signature_partial_decryption(&mut OsRng)
        .unwrap()
}


pub fn message_digest(message: &[u8], hash: &Hash) -> secp256k1::Scalar {

    //todo: remove unwrap!
    let m = match hash {
        Hash::KECCAK256 =>  bits2field::<k256::Secp256k1>(
            &sha3::Keccak256::new_with_prefix(message).finalize_fixed(),
        ),
        Hash::SHA256 => bits2field::<k256::Secp256k1>(
            &sha2::Sha256::new_with_prefix(message).finalize_fixed(),
        ),
    }.unwrap();

    let m = <elliptic_curve::Scalar<k256::Secp256k1> as Reduce<U256>>::reduce_bytes(&m);
    U256::from(m).into()
}

// -------------------------------------------------------------------------------------------------
// MPC common
// -------------------------------------------------------------------------------------------------

// TODO: uncomment this and use a struct and not PhantomData
// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ProtocolContext {
//     epoch: EpochId,
//     party_id: PartyID,
//     number_of_parties: PartyID,
//     session_id: SignatureMPCSessionID,
// }

// TODO: remove this temp hack
pub type ProtocolContext = PhantomData<()>;

pub fn config_signature_mpc_secret_for_network_for_testing(number_of_parties: PartyID) -> (DecryptionPublicParameters, HashMap<PartyID, SecretKeyShareSizedNumber>) {
    let t = (((number_of_parties * 2) / 3) + 1) as PartyID;

    pub const N: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");
    pub const SECRET_KEY: PaillierModulusSizedNumber = PaillierModulusSizedNumber::from_be_hex("19d698592b9ccb2890fb84be46cd2b18c360153b740aeccb606cf4168ee2de399f05273182bf468978508a5f4869cb867b340e144838dfaf4ca9bfd38cd55dc2837688aed2dbd76d95091640c47b2037d3d0ca854ffb4c84970b86f905cef24e876ddc8ab9e04f2a5f171b9c7146776c469f0d90908aa436b710cf4489afc73cd3ee38bb81e80a22d5d9228b843f435c48c5eb40088623a14a12b44e2721b56625da5d56d257bb27662c6975630d51e8f5b930d05fc5ba461a0e158cbda0f3266408c9bf60ff617e39ae49e707cbb40958adc512f3b4b69a5c3dc8b6d34cf45bc9597840057438598623fb65254869a165a6030ec6bec12fd59e192b3c1eefd33ef5d9336e0666aa8f36c6bd2749f86ea82290488ee31bf7498c2c77a8900bae00efcff418b62d41eb93502a245236b89c241ad6272724858122a2ebe1ae7ec4684b29048ba25b3a516c281a93043d58844cf3fa0c6f1f73db5db7ecba179652349dea8df5454e0205e910e0206736051ac4b7c707c3013e190423532e907af2e85e5bb6f6f0b9b58257ca1ec8b0318dd197f30352a96472a5307333f0e6b83f4f775fb302c1e10f21e1fcbfff17e3a4aa8bb6f553d9c6ebc2c884ae9b140dd66f21afc8610418e9f0ba2d14ecfa51ff08744a3470ebe4bb21bd6d65b58ac154630b8331ea620673ffbabb179a971a6577c407a076654a629c7733836c250000");
    pub const BASE: PaillierModulusSizedNumber = PaillierModulusSizedNumber::from_be_hex("03B4EFB895D3A85104F1F93744F9DB8924911747DE87ACEC55F1BF37C4531FD7F0A5B498A943473FFA65B89A04FAC2BBDF76FF14D81EB0A0DAD7414CF697E554A93C8495658A329A1907339F9438C1048A6E14476F9569A14BD092BCB2730DCE627566808FD686008F46A47964732DC7DCD2E6ECCE83F7BCCAB2AFDF37144ED153A118B683FF6A3C6971B08DE53DA5D2FEEF83294C21998FC0D1E219A100B6F57F2A2458EA9ABCFA8C5D4DF14B286B71BF5D7AD4FFEEEF069B64E0FC4F1AB684D6B2F20EAA235892F360AA2ECBF361357405D77E5023DF7BEDC12F10F6C35F3BE1163BC37B6C97D62616260A2862F659EB1811B1DDA727847E810D0C2FA120B18E99C9008AA4625CF1862460F8AB3A41E3FDB552187E0408E60885391A52EE2A89DD2471ECBA0AD922DEA0B08474F0BED312993ECB90C90C0F44EF267124A6217BC372D36F8231EB76B0D31DDEB183283A46FAAB74052A01F246D1C638BC00A47D25978D7DF9513A99744D8B65F2B32E4D945B0BA3B7E7A797604173F218D116A1457D20A855A52BBD8AC15679692C5F6AC4A8AF425370EF1D4184322F317203BE9678F92BFD25C7E6820D70EE08809424720249B4C58B81918DA02CFD2CAB3C42A02B43546E64430F529663FCEFA51E87E63F0813DA52F3473506E9E98DCD3142D830F1C1CDF6970726C190EAE1B5D5A26BC30857B4DF639797895E5D61A5EE");


    tiresias_deal_trusted_shares(t, number_of_parties, N, SECRET_KEY, BASE)
}


// a workaround to decrialize to PublicKeyValue - TODO: add from_bytes to PublicKeyValue
pub fn affine_point_to_public_key(public_key: &Vec<u8>) -> Option<PublicKeyValue> {

    let public_key: Option<AffinePoint> = AffinePoint::from_bytes(CompressedPoint::from_slice(public_key)).into();
    let public_key = public_key.map(|pk| bcs::to_bytes(&pk).ok()).flatten();
    let public_key = public_key.map(|pk| bcs::from_bytes::<PublicKeyValue>(&pk).ok()).flatten();

    public_key
}

pub fn recovery_id(message: Vec<u8>, public_key: PublicKeyValue, signature: SignatureK256Secp256k1, hash: &Hash) -> ecdsa::Result<RecoveryId> {
    let verifying_key = VerifyingKey::<k256::Secp256k1>::from_affine(public_key.into()).unwrap();
    match hash {
        Hash::KECCAK256 => RecoveryId::trial_recovery_from_digest(&verifying_key, sha3::Keccak256::new_with_prefix(message), &signature),
        Hash::SHA256 => RecoveryId::trial_recovery_from_digest(&verifying_key, sha2::Sha256::new_with_prefix(message), &signature)
    }
}