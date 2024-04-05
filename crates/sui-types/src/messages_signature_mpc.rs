// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::committee::EpochId;
use crate::crypto::{default_hash, AuthoritySignInfo, AuthorityStrongQuorumSignInfo};
use crate::digests::{SignatureMPCMessageDigest, SignatureMPCOutputDigest};
use crate::error::SuiResult;
use crate::message_envelope::{Envelope, Message, UnauthenticatedMessage};
use crate::{committee::Committee, error::SuiError};
use std::collections::{HashMap, HashSet};

pub use commitment::{Commitment};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

pub use crate::digests::CheckpointContentsDigest;
pub use crate::digests::CheckpointDigest;

pub use group::PartyID;

use crypto_bigint::{ U256};
use ecdsa::{
    elliptic_curve::{ops::Reduce, Scalar},
    hazmat::{bits2field, DigestPrimitive},
    signature::{digest::Digest},
    Signature,
};
pub use enhanced_maurer::language::EnhancedLanguageStatementAccessors;
pub use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;

use group::{secp256k1, GroupElement as _,};
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
use tiresias::{EncryptionKey};
use twopc_mpc::paillier::PLAINTEXT_SPACE_SCALAR_LIMBS;
pub use twopc_mpc::secp256k1::{SCALAR_LIMBS, GroupElement};
pub use twopc_mpc::secp256k1::paillier::bulletproofs::{
        CentralizedPartyPresign, DecommitmentProofVerificationRoundParty, DKGCommitmentRoundParty, EncryptionOfSecretKeyShareRoundParty, PresignCommitmentRoundParty, ProtocolPublicParameters, SignatureHomomorphicEvaluationParty, SignatureNonceSharesCommitmentsAndBatchedProof, SignaturePartialDecryptionParty, SignatureThresholdDecryptionParty,  DKGCentralizedPartyOutput,
        PublicKeyShareDecommitmentAndProof, PublicNonceEncryptedPartialSignatureAndProof, SecretKeyShareEncryptionAndProof, DKGDecentralizedPartyOutput, DecentralizedPartyPresign, EncDHCommitment, EncDHCommitmentRoundParty,EncDHProofAggregationRoundParty, EncDHProofShareRoundParty, EncDLCommitment, EncDHDecommitment, EncDLDecommitment, EncDHProofShare, EncDLProofShare,
                                                       EncDLCommitmentRoundParty,  EncDLDecommitmentRoundParty, EncryptedMaskedNoncesRoundParty, EncryptedMaskedKeyShareRoundParty, PresignDecentralizedPartyOutput,
                                                       EncDLProofAggregationOutput, EncDLProofAggregationRoundParty,EncDLProofShareRoundParty, EncryptedMaskAndMaskedNonceShare, EncryptedNonceShareAndPublicShare, EncDHDecommitmentRoundParty, EncDHProofAggregationOutput};
use crate::base_types::ObjectRef;

pub use twopc_mpc::{Result, Error};

pub type InitSignatureMPCProtocolSequenceNumber = u64;
pub type SignatureMPCRound = u64;
pub type SignatureMPCMessageKind = u64;
pub type SignatureMPCTimestamp = u64;

struct PublicParameters {
    tiresias_public_parameters: tiresias::encryption_key::PublicParameters
}

pub fn initiate_centralized_party_dkg(//tiresias_public_parameters: &str, epoch: EpochId, party_id: PartyID, threshold: PartyID, number_of_parties: PartyID, session_id: SignatureMpcSessionID
) -> twopc_mpc::Result<DKGCommitmentRoundParty<ProtocolContext>> {
    pub const N: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");

    let protocol_public_parameters = ProtocolPublicParameters::new(N);

    Ok(DKGCommitmentRoundParty::new(protocol_public_parameters, PhantomData))
}

// -------------------------------------------------------------------------------------------------
// DKG Decentralized Party Parties
// -------------------------------------------------------------------------------------------------

pub fn initiate_decentralized_party_dkg(
    decryption_key_share_public_parameters: <DecryptionKeyShare as AdditivelyHomomorphicDecryptionKeyShare<PLAINTEXT_SPACE_SCALAR_LIMBS, EncryptionKey>>::PublicParameters,
    epoch: EpochId,
    party_id: PartyID,
    parties: HashSet<PartyID>,
    session_id: SignatureMPCSessionID,
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
) -> twopc_mpc::Result<DKGDecentralizedPartyOutput> {
    let protocol_public_parameters = ProtocolPublicParameters::new(LargeBiPrimeSizedNumber::from_be_hex(tiresias_public_parameters));

    let decommitment_proof_verification_round_party =
        DecommitmentProofVerificationRoundParty::new(protocol_public_parameters, commitment_to_centralized_party_secret_key_share, PhantomData);

    decommitment_proof_verification_round_party
        .verify_decommitment_and_proof_of_centralized_party_public_key_share(
            centralized_party_public_key_share_decommitment_and_proof,
            secret_key_share_encryption_and_proof,
        )
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
    epoch: EpochId,
    party_id: PartyID,
    parties: HashSet<PartyID>,
    session_id: SignatureMPCSessionID,
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
    epoch: EpochId,
    party_id: PartyID,
    parties: HashSet<PartyID>,
    session_id: SignatureMPCSessionID,
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
) -> twopc_mpc::Result<()> {
    let protocol_public_parameters = ProtocolPublicParameters::new(LargeBiPrimeSizedNumber::from_be_hex(tiresias_public_parameters));

    messages.into_iter().zip(public_nonce_encrypted_partial_signature_and_proofs.into_iter()).zip(presigns
        .into_iter())

        .map(|((message, public_nonce_encrypted_partial_signature_and_proofs), presign)| {
            let m = message_digest(&message);
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
pub fn decrypt_signature_decentralized_party_sign(
    public_key: group::Value<GroupElement>,
    messages: Vec<Vec<u8>>,
    decryption_key_share_public_parameters: DecryptionPublicParameters,
    decryption_shares: HashMap<PartyID, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>>,
    public_nonce_encrypted_partial_signature_and_proofs: Vec<PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>>,
    signature_threshold_decryption_round_parties: Vec<SignatureThresholdDecryptionParty>,
) -> twopc_mpc::Result<Vec<Vec<u8>>> {
    let protocol_public_parameters = ProtocolPublicParameters::new(*decryption_key_share_public_parameters.encryption_scheme_public_parameters.plaintext_space_public_parameters().modulus);

    // TODO: choose multiple?
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

    signature_threshold_decryption_round_parties.into_iter().zip(messages.into_iter().zip(public_nonce_encrypted_partial_signature_and_proofs.into_iter()).zip(decryption_shares.into_iter())).map(|(signature_threshold_decryption_round_party, ((message, public_nonce_encrypted_partial_signature_and_proof), (partial_signature_decryption_shares, masked_nonce_decryption_shares)))| {

        let (nonce_x_coordinate, signature_s) = signature_threshold_decryption_round_party.decrypt_signature(
            lagrange_coefficients.clone(),
            partial_signature_decryption_shares,
            masked_nonce_decryption_shares,
        )?;

        let signature_s_inner: k256::Scalar = signature_s.into();

        Ok(Signature::<k256::Secp256k1>::from_scalars(k256::Scalar::from(nonce_x_coordinate), signature_s_inner).unwrap().to_vec())
    })
        .collect()
}

pub fn message_digest(message: &[u8]) -> secp256k1::Scalar {
    let m = bits2field::<k256::Secp256k1>(
        &<k256::Secp256k1 as DigestPrimitive>::Digest::new_with_prefix(message).finalize_fixed(),
    )
    .unwrap();

    let m = <Scalar<k256::Secp256k1> as Reduce<U256>>::reduce_bytes(&m);
    U256::from(m).into()
}

// -------------------------------------------------------------------------------------------------
// MPC common
// -------------------------------------------------------------------------------------------------

const SESSION_ID_LENGTH: usize = 32;
/// The session id of the mpc is working on.
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignatureMPCSessionID(pub [u8; SESSION_ID_LENGTH]);

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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SignatureMPCBulletProofAggregatesMessage {
    Commitment((Vec<EncDHCommitment<ProtocolContext>>, Vec<EncDLCommitment<ProtocolContext>>)),
    Decommitment((Vec<EncDHDecommitment<ProtocolContext>>, Vec<EncDLDecommitment<ProtocolContext>>)),
    ProofShare((Vec<EncDHProofShare<ProtocolContext>>, Vec<EncDLProofShare<ProtocolContext>>)),
}

impl SignatureMPCBulletProofAggregatesMessage {
    pub fn round(&self) -> SignatureMPCRound {
        match self {
            SignatureMPCBulletProofAggregatesMessage::Commitment(_) => 1,
            SignatureMPCBulletProofAggregatesMessage::Decommitment(_) => 2,
            SignatureMPCBulletProofAggregatesMessage::ProofShare(_) => 3,
        }
    }
}

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
// pub enum DKGSignatureMPCMessageProtocol {
//     BulletproofsCommitment(DKGSignatureMPCBulletproofsCommitment),
//     BulletproofsDecommitment(DKGSignatureMPCBulletproofsDecommitment),
//     BulletproofsProofShare(DKGSignatureMPCBulletproofsProofShare),
// }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SignatureMPCMessageProtocols {
    DKG(SignatureMPCBulletProofAggregatesMessage),
    PresignFirstRound(SignatureMPCBulletProofAggregatesMessage),
    PresignSecondRound(SignatureMPCBulletProofAggregatesMessage),
    Sign(Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignatureMPCMessageSummary {
    pub epoch: EpochId,
    pub message: SignatureMPCMessageProtocols,
    pub session_id: SignatureMPCSessionID,
}

impl Message for SignatureMPCMessageSummary {
    type DigestType = SignatureMPCMessageDigest;
    const SCOPE: IntentScope = IntentScope::SignatureMPCMessage;

    fn digest(&self) -> Self::DigestType {
        SignatureMPCMessageDigest::new(default_hash(self))
    }

    fn verify_user_input(&self) -> SuiResult {
        Ok(())
    }

    fn verify_epoch(&self, epoch: EpochId) -> SuiResult {
        fp_ensure!(
            self.epoch == epoch,
            SuiError::WrongEpoch {
                expected_epoch: epoch,
                actual_epoch: self.epoch,
            }
        );
        Ok(())
    }
}

impl UnauthenticatedMessage for SignatureMPCMessageSummary {}

impl SignatureMPCMessageSummary {
    pub fn new(
        epoch: EpochId,
        message: SignatureMPCMessageProtocols,
        session_id: SignatureMPCSessionID,
    ) -> SignatureMPCMessageSummary {
        Self {
            epoch,
            message,
            session_id,
        }
    }
}

impl Display for SignatureMPCMessageSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SignatureMPCMessage {{ epoch: {:?}, message: {:?}, session_id: {:?}}}",
            self.epoch, self.message, self.session_id,
        )
    }
}

pub type SignatureMPCMessageSummaryEnvelope<S> = Envelope<SignatureMPCMessageSummary, S>;
pub type SignedSignatureMPCMessageSummary = SignatureMPCMessageSummaryEnvelope<AuthoritySignInfo>;

/// This is a message validators publish to consensus in order to sign checkpoint
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignatureMPCMessage {
    pub summary: SignedSignatureMPCMessageSummary,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SignatureMPCOutputValue {
    DKG {
        commitment_to_centralized_party_secret_key_share: Vec<u8>,
        secret_key_share_encryption_and_proof: Vec<u8>,
    },
    PresignOutput(Vec<u8>),
    Presign(Vec<u8>),
    Sign(Vec<Vec<u8>>),
}

impl Display for SignatureMPCOutputValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SignatureMPCOutputValue::DKG {
                commitment_to_centralized_party_secret_key_share,
                secret_key_share_encryption_and_proof,
            } => {
                write!(
                    f,
                    "DKGSignatureMPCOutputValue::DKG {{ commitment_to_centralized_party_secret_key_share: {:?}, secret_key_share_encryption_and_proof: {:?}}}",
                    commitment_to_centralized_party_secret_key_share,
                    secret_key_share_encryption_and_proof,
                )
            }
            SignatureMPCOutputValue::PresignOutput(output) => {
                write!(
                    f,
                    "DKGSignatureMPCOutputValue::PresignOutput {{ output: {:?}}}",
                    output,
                )
            }
            SignatureMPCOutputValue::Presign(presigns) => {
                write!(
                    f,
                    "DKGSignatureMPCOutputValue::Presign {{ presigns: {:?}}}",
                    presigns,
                )
            }
            SignatureMPCOutputValue::Sign(sigs) => {
                write!(
                    f,
                    "DKGSignatureMPCOutputValue::Sign {{ sigs: {:?}}}",
                    sigs,
                )
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SignatureMPCOutput {
    pub epoch: EpochId,
    pub session_id: SignatureMPCSessionID,
    pub session_ref: ObjectRef,
    pub value: SignatureMPCOutputValue,
}

impl Message for SignatureMPCOutput {
    type DigestType = SignatureMPCOutputDigest;
    const SCOPE: IntentScope = IntentScope::SignatureMPCOutput;

    fn digest(&self) -> Self::DigestType {
        SignatureMPCOutputDigest::new(default_hash(self))
    }

    fn verify_user_input(&self) -> SuiResult {
        Ok(())
    }

    fn verify_epoch(&self, epoch: EpochId) -> SuiResult {
        fp_ensure!(
            self.epoch == epoch,
            SuiError::WrongEpoch {
                expected_epoch: epoch,
                actual_epoch: self.epoch,
            }
        );
        Ok(())
    }
}

impl UnauthenticatedMessage for SignatureMPCOutput {}

impl SignatureMPCOutput {
    pub fn new_dkg(
        epoch: EpochId,
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        commitment_to_centralized_party_secret_key_share: Commitment,
        secret_key_share_encryption_and_proof: SecretKeyShareEncryptionAndProof<ProtocolContext>,
    ) -> SuiResult<SignatureMPCOutput> {
        let commitment_to_centralized_party_secret_key_share =
            bcs::to_bytes(&commitment_to_centralized_party_secret_key_share).map_err(|e| {
                SuiError::ObjectSerializationError {
                    error: format!("{e}"),
                }
            })?;
        let secret_key_share_encryption_and_proof =
            bcs::to_bytes(&secret_key_share_encryption_and_proof).map_err(|e| {
                SuiError::ObjectSerializationError {
                    error: format!("{e}"),
                }
            })?;
        Ok(Self {
            epoch,
            session_id,
            session_ref,
            value: SignatureMPCOutputValue::DKG {
                commitment_to_centralized_party_secret_key_share,
                secret_key_share_encryption_and_proof,
            },
        })
    }
    pub fn new_presign_output(
        epoch: EpochId,
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        output: PresignDecentralizedPartyOutput<ProtocolContext>,
    ) -> SuiResult<SignatureMPCOutput> {
        let output = bcs::to_bytes(&output).map_err(|e| SuiError::ObjectSerializationError {
            error: format!("{e}"),
        })?;
        Ok(Self {
            epoch,
            session_id,
            session_ref,
            value: SignatureMPCOutputValue::PresignOutput(output),
        })
    }
    pub fn new_presign(
        epoch: EpochId,
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        presigns: Vec<DecentralizedPartyPresign>,
    ) -> SuiResult<SignatureMPCOutput> {
        let presigns =
            bcs::to_bytes(&presigns).map_err(|e| SuiError::ObjectSerializationError {
                error: format!("{e}"),
            })?;
        Ok(Self {
            epoch,
            session_id,
            session_ref,
            value: SignatureMPCOutputValue::Presign(presigns),
        })
    }
    pub fn new_sign(
        epoch: EpochId,
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        sigs: Vec<Vec<u8>>,
    ) -> SuiResult<SignatureMPCOutput> {
        Ok(Self {
            epoch,
            session_id,
            session_ref,
            value: SignatureMPCOutputValue::Sign(sigs),
        })
    }

    pub fn message_kind(&self) -> SignatureMPCMessageKind {
        match &self.value {
            SignatureMPCOutputValue::DKG { .. } => 1,
            SignatureMPCOutputValue::PresignOutput(_) => 2,
            SignatureMPCOutputValue::Presign(_) => 3,
            SignatureMPCOutputValue::Sign(_) => 4,
        }
    }
}

impl Display for SignatureMPCOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DKGSignatureMPCAggregatedProofSharesMessage {{ epoch: {:?}, session_id: {:?}, value: {}}}",
            self.epoch,
            self.session_id,
            self.value,
        )
    }
}

pub type SignatureMPCOutputEnvelope<S> = Envelope<SignatureMPCOutput, S>;
pub type SignedSignatureMPCOutput = SignatureMPCOutputEnvelope<AuthoritySignInfo>;
pub type CertifiedSignatureMPCOutput =
    SignatureMPCOutputEnvelope<AuthorityStrongQuorumSignInfo>;

impl SignatureMPCMessage {
    pub fn verify(&self, committee: &Committee) -> SuiResult {
        self.summary.verify_authority_signatures(committee)
    }

    pub fn message_kind(&self) -> SignatureMPCMessageKind {
        match &self.summary.message {
            SignatureMPCMessageProtocols::DKG(_) => 1,
            SignatureMPCMessageProtocols::PresignFirstRound(_) => 2,
            SignatureMPCMessageProtocols::PresignSecondRound(_) => 3,
            SignatureMPCMessageProtocols::Sign(_) => 3,
        }
    }

    pub fn round(&self) -> SignatureMPCRound {
        match &self.summary.message {
            SignatureMPCMessageProtocols::DKG(m) => m.round(),
            SignatureMPCMessageProtocols::PresignFirstRound(m) => m.round(),
            SignatureMPCMessageProtocols::PresignSecondRound(m) => m.round(),
            SignatureMPCMessageProtocols::Sign(_) => 1,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum InitiateSignatureMPCProtocol {
    DKG {
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        commitment_to_centralized_party_secret_key_share: Commitment,
    },
    Presign {
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        dkg_output: DKGDecentralizedPartyOutput,
        commitments_and_proof_to_centralized_party_nonce_shares:
            SignatureNonceSharesCommitmentsAndBatchedProof<ProtocolContext>,
    },
    Sign {
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        public_key: group::Value<GroupElement>,
        messages: Vec<Vec<u8>>,
        dkg_output: DKGDecentralizedPartyOutput,
        public_nonce_encrypted_partial_signature_and_proofs: Vec<PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>>,
        presigns: Vec<DecentralizedPartyPresign>,
    },
}

pub fn config_signature_mpc_secret_for_network_for_testing(number_of_parties: PartyID) -> (DecryptionPublicParameters, HashMap<PartyID, SecretKeyShareSizedNumber>) {
    let t = (((number_of_parties * 2) / 3) + 1) as PartyID;
    
    pub const N: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");
    pub const SECRET_KEY: PaillierModulusSizedNumber = PaillierModulusSizedNumber::from_be_hex("19d698592b9ccb2890fb84be46cd2b18c360153b740aeccb606cf4168ee2de399f05273182bf468978508a5f4869cb867b340e144838dfaf4ca9bfd38cd55dc2837688aed2dbd76d95091640c47b2037d3d0ca854ffb4c84970b86f905cef24e876ddc8ab9e04f2a5f171b9c7146776c469f0d90908aa436b710cf4489afc73cd3ee38bb81e80a22d5d9228b843f435c48c5eb40088623a14a12b44e2721b56625da5d56d257bb27662c6975630d51e8f5b930d05fc5ba461a0e158cbda0f3266408c9bf60ff617e39ae49e707cbb40958adc512f3b4b69a5c3dc8b6d34cf45bc9597840057438598623fb65254869a165a6030ec6bec12fd59e192b3c1eefd33ef5d9336e0666aa8f36c6bd2749f86ea82290488ee31bf7498c2c77a8900bae00efcff418b62d41eb93502a245236b89c241ad6272724858122a2ebe1ae7ec4684b29048ba25b3a516c281a93043d58844cf3fa0c6f1f73db5db7ecba179652349dea8df5454e0205e910e0206736051ac4b7c707c3013e190423532e907af2e85e5bb6f6f0b9b58257ca1ec8b0318dd197f30352a96472a5307333f0e6b83f4f775fb302c1e10f21e1fcbfff17e3a4aa8bb6f553d9c6ebc2c884ae9b140dd66f21afc8610418e9f0ba2d14ecfa51ff08744a3470ebe4bb21bd6d65b58ac154630b8331ea620673ffbabb179a971a6577c407a076654a629c7733836c250000");
    pub const BASE: PaillierModulusSizedNumber = PaillierModulusSizedNumber::from_be_hex("03B4EFB895D3A85104F1F93744F9DB8924911747DE87ACEC55F1BF37C4531FD7F0A5B498A943473FFA65B89A04FAC2BBDF76FF14D81EB0A0DAD7414CF697E554A93C8495658A329A1907339F9438C1048A6E14476F9569A14BD092BCB2730DCE627566808FD686008F46A47964732DC7DCD2E6ECCE83F7BCCAB2AFDF37144ED153A118B683FF6A3C6971B08DE53DA5D2FEEF83294C21998FC0D1E219A100B6F57F2A2458EA9ABCFA8C5D4DF14B286B71BF5D7AD4FFEEEF069B64E0FC4F1AB684D6B2F20EAA235892F360AA2ECBF361357405D77E5023DF7BEDC12F10F6C35F3BE1163BC37B6C97D62616260A2862F659EB1811B1DDA727847E810D0C2FA120B18E99C9008AA4625CF1862460F8AB3A41E3FDB552187E0408E60885391A52EE2A89DD2471ECBA0AD922DEA0B08474F0BED312993ECB90C90C0F44EF267124A6217BC372D36F8231EB76B0D31DDEB183283A46FAAB74052A01F246D1C638BC00A47D25978D7DF9513A99744D8B65F2B32E4D945B0BA3B7E7A797604173F218D116A1457D20A855A52BBD8AC15679692C5F6AC4A8AF425370EF1D4184322F317203BE9678F92BFD25C7E6820D70EE08809424720249B4C58B81918DA02CFD2CAB3C42A02B43546E64430F529663FCEFA51E87E63F0813DA52F3473506E9E98DCD3142D830F1C1CDF6970726C190EAE1B5D5A26BC30857B4DF639797895E5D61A5EE");


    tiresias_deal_trusted_shares(t, number_of_parties, N, SECRET_KEY, BASE)
}