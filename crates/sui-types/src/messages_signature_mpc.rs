// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::committee::EpochId;
use crate::crypto::{default_hash, AuthoritySignInfo, AuthorityStrongQuorumSignInfo};
use crate::digests::{SignatureMPCMessageDigest, SignatureMPCOutputDigest};
use crate::error::SuiResult;
use crate::message_envelope::{Envelope, Message, UnauthenticatedMessage};
use crate::{committee::Committee, error::SuiError};
use anyhow::anyhow;
use std::collections::{HashMap, HashSet};

use commitment::Pedersen;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

pub use crate::digests::CheckpointContentsDigest;
pub use crate::digests::CheckpointDigest;

pub use group::PartyID;

use crypto_bigint::{Uint, U256, U64};
use ecdsa::{
    elliptic_curve::{ops::Reduce, Scalar},
    hazmat::{bits2field, DigestPrimitive},
    signature::{digest::Digest, Verifier},
    Signature, VerifyingKey,
};
pub use enhanced_maurer::language::EnhancedLanguageStatementAccessors;
pub use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use enhanced_maurer::{
    committed_linear_evaluation, encryption_of_discrete_log, encryption_of_tuple,
};
use group::{ristretto, secp256k1, self_product, CyclicGroupElement, GroupElement as _, Samplable, StatisticalSecuritySizedNumber, AffineXCoordinate};
use homomorphic_encryption::{
    AdditivelyHomomorphicDecryptionKey,
    AdditivelyHomomorphicEncryptionKey, GroupsPublicParametersAccessors,
};
use k256::{elliptic_curve::scalar::IsHigh, sha2::digest::FixedOutput};
use maurer::{
    committment_of_discrete_log, discrete_log_ratio_of_committed_values, knowledge_of_discrete_log,
};
pub use proof::aggregation::{
    CommitmentRoundParty, DecommitmentRoundParty, ProofAggregationRoundParty, ProofShareRoundParty,
};
use proof::{range, range::bulletproofs};
pub use tiresias::{
    decryption_key_share::PublicParameters as DecryptionPublicParameters,
    encryption_key::PublicParameters as EncryptionPublicParameters,
    test_exports::deal_trusted_shares as tiresias_deal_trusted_shares, DecryptionKeyShare,
    LargeBiPrimeSizedNumber, PaillierModulusSizedNumber, SecretKeyShareSizedNumber,
    AdjustedLagrangeCoefficientSizedNumber
};
use twopc_mpc::sign::DIMENSION;

use crate::base_types::ObjectRef;

pub type InitSignatureMPCProtocolSequenceNumber = u64;
pub type SignatureMPCRound = u64;
pub type SignatureMPCMessageKind = u64;
pub type SignatureMPCTimestamp = u64;

// TODO: Copied from test
pub const RANGE_CLAIMS_PER_SCALAR: usize =
    Uint::<{ secp256k1::SCALAR_LIMBS }>::BITS / bulletproofs::RANGE_CLAIM_BITS;

pub(crate) const MASK_LIMBS: usize =
    secp256k1::SCALAR_LIMBS + StatisticalSecuritySizedNumber::LIMBS + U64::LIMBS;

pub const RANGE_CLAIMS_PER_MASK: usize =
    (Uint::<MASK_LIMBS>::BITS / bulletproofs::RANGE_CLAIM_BITS).next_power_of_two();

pub const NUM_RANGE_CLAIMS: usize = DIMENSION * RANGE_CLAIMS_PER_SCALAR + RANGE_CLAIMS_PER_MASK;
pub type TwopcMPCResult<T> = twopc_mpc::Result<T>;
pub type TwopcMPCError = twopc_mpc::Error;

pub type EncDLCommitmentRoundParty = enhanced_maurer::aggregation::commitment_round::Party<
    { maurer::SOUND_PROOFS_REPETITIONS },
    { RANGE_CLAIMS_PER_SCALAR },
    { ristretto::SCALAR_LIMBS },
    bulletproofs::RangeProof,
    tiresias::RandomnessSpaceGroupElement,
    encryption_of_discrete_log::Language<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
    >,
    //ProtocolContext,
    PhantomData<()>,
>;

pub type EncDLCommitment =
    <EncDLCommitmentRoundParty as proof::aggregation::CommitmentRoundParty<
        EncDLProofAggregationOutput,
    >>::Commitment;

pub type EncDLDecommitmentRoundParty = enhanced_maurer::aggregation::decommitment_round::Party<
    { maurer::SOUND_PROOFS_REPETITIONS },
    { RANGE_CLAIMS_PER_SCALAR },
    { ristretto::SCALAR_LIMBS },
    bulletproofs::RangeProof,
    tiresias::RandomnessSpaceGroupElement,
    encryption_of_discrete_log::Language<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
    >,
    //ProtocolContext,
    PhantomData<()>,
>;

pub type EncDLDecommitment =
    <EncDLDecommitmentRoundParty as proof::aggregation::DecommitmentRoundParty<
        EncDLProofAggregationOutput,
    >>::Decommitment;

pub type EncDLProofShareRoundParty = enhanced_maurer::aggregation::proof_share_round::Party<
    { maurer::SOUND_PROOFS_REPETITIONS },
    { RANGE_CLAIMS_PER_SCALAR },
    { ristretto::SCALAR_LIMBS },
    bulletproofs::RangeProof,
    tiresias::RandomnessSpaceGroupElement,
    encryption_of_discrete_log::Language<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
    >,
    //ProtocolContext,
    PhantomData<()>,
>;

pub type EncDLProofShare =
    <EncDLProofShareRoundParty as proof::aggregation::ProofShareRoundParty<
        EncDLProofAggregationOutput,
    >>::ProofShare;

pub type EncDLProofAggregationRoundParty =
    enhanced_maurer::aggregation::proof_aggregation_round::Party<
        { maurer::SOUND_PROOFS_REPETITIONS },
        { RANGE_CLAIMS_PER_SCALAR },
        { ristretto::SCALAR_LIMBS },
        bulletproofs::RangeProof,
        tiresias::RandomnessSpaceGroupElement,
        encryption_of_discrete_log::Language<
            { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
            { secp256k1::SCALAR_LIMBS },
            secp256k1::GroupElement,
            tiresias::EncryptionKey,
        >,
        //ProtocolContext,
        PhantomData<()>,
    >;
pub type EncDLProofAggregationOutput = enhanced_maurer::aggregation::Output<
    { maurer::SOUND_PROOFS_REPETITIONS },
    { RANGE_CLAIMS_PER_SCALAR },
    { ristretto::SCALAR_LIMBS },
    bulletproofs::RangeProof,
    tiresias::RandomnessSpaceGroupElement,
    encryption_of_discrete_log::Language<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
    >,
    //ProtocolContext,
    PhantomData<()>,
>;

pub type EncDLProof = enhanced_maurer::Proof<
    { maurer::SOUND_PROOFS_REPETITIONS },
    { RANGE_CLAIMS_PER_SCALAR },
    { ristretto::SCALAR_LIMBS },
    bulletproofs::RangeProof,
    tiresias::RandomnessSpaceGroupElement,
    encryption_of_discrete_log::Language<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
    >,
    //ProtocolContext,
    PhantomData<()>,
>;

pub type EncDHCommitmentRoundParty = enhanced_maurer::aggregation::commitment_round::Party<
    { maurer::SOUND_PROOFS_REPETITIONS },
    { RANGE_CLAIMS_PER_SCALAR },
    { ristretto::SCALAR_LIMBS },
    bulletproofs::RangeProof,
    self_product::GroupElement<2, tiresias::RandomnessSpaceGroupElement>,
    encryption_of_tuple::Language<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
    >,
    //ProtocolContext,
    PhantomData<()>,
>;

pub type EncDHCommitment =
    <EncDHCommitmentRoundParty as proof::aggregation::CommitmentRoundParty<
        EncDHProofAggregationOutput,
    >>::Commitment;

pub type EncDHDecommitmentRoundParty = enhanced_maurer::aggregation::decommitment_round::Party<
    { maurer::SOUND_PROOFS_REPETITIONS },
    { RANGE_CLAIMS_PER_SCALAR },
    { ristretto::SCALAR_LIMBS },
    bulletproofs::RangeProof,
    self_product::GroupElement<2, tiresias::RandomnessSpaceGroupElement>,
    encryption_of_tuple::Language<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
    >,
    //ProtocolContext,
    PhantomData<()>,
>;

pub type EncDHDecommitment =
    <EncDHDecommitmentRoundParty as proof::aggregation::DecommitmentRoundParty<
        EncDHProofAggregationOutput,
    >>::Decommitment;

pub type EncDHProofShareRoundParty = enhanced_maurer::aggregation::proof_share_round::Party<
    { maurer::SOUND_PROOFS_REPETITIONS },
    { RANGE_CLAIMS_PER_SCALAR },
    { ristretto::SCALAR_LIMBS },
    bulletproofs::RangeProof,
    self_product::GroupElement<2, tiresias::RandomnessSpaceGroupElement>,
    encryption_of_tuple::Language<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
    >,
    //ProtocolContext,
    PhantomData<()>,
>;

pub type EncDHProofShare =
    <EncDHProofShareRoundParty as proof::aggregation::ProofShareRoundParty<
        EncDHProofAggregationOutput,
    >>::ProofShare;

pub type EncDHProofAggregationRoundParty =
    enhanced_maurer::aggregation::proof_aggregation_round::Party<
        { maurer::SOUND_PROOFS_REPETITIONS },
        { RANGE_CLAIMS_PER_SCALAR },
        { ristretto::SCALAR_LIMBS },
        bulletproofs::RangeProof,
        self_product::GroupElement<2, tiresias::RandomnessSpaceGroupElement>,
        encryption_of_tuple::Language<
            { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
            { secp256k1::SCALAR_LIMBS },
            secp256k1::GroupElement,
            tiresias::EncryptionKey,
        >,
        //ProtocolContext,
        PhantomData<()>,
    >;

pub type EncDHProofAggregationOutput = enhanced_maurer::aggregation::Output<
    { maurer::SOUND_PROOFS_REPETITIONS },
    { RANGE_CLAIMS_PER_SCALAR },
    { ristretto::SCALAR_LIMBS },
    bulletproofs::RangeProof,
    self_product::GroupElement<2, tiresias::RandomnessSpaceGroupElement>,
    encryption_of_tuple::Language<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
    >,
    //ProtocolContext,
    PhantomData<()>,
>;

pub type EncDHProof = enhanced_maurer::Proof<
    { maurer::SOUND_PROOFS_REPETITIONS },
    { RANGE_CLAIMS_PER_SCALAR },
    { ristretto::SCALAR_LIMBS },
    bulletproofs::RangeProof,
    self_product::GroupElement<2, tiresias::RandomnessSpaceGroupElement>,
    encryption_of_tuple::Language<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
    >,
    //ProtocolContext,
    PhantomData<()>,
>;

pub type DComEvalProof = committed_linear_evaluation::Proof<
    { NUM_RANGE_CLAIMS },
    { RANGE_CLAIMS_PER_SCALAR },
    { RANGE_CLAIMS_PER_MASK },
    { ristretto::SCALAR_LIMBS },
    { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
    { secp256k1::SCALAR_LIMBS },
    { DIMENSION },
    secp256k1::GroupElement,
    tiresias::EncryptionKey,
    bulletproofs::RangeProof,
    group::direct_product::GroupElement<
        self_product::GroupElement<DIMENSION, secp256k1::Scalar>,
        tiresias::RandomnessSpaceGroupElement,
    >,
    //ProtocolContext,
    PhantomData<()>,
>;

pub type SchnorrProof =
    knowledge_of_discrete_log::Proof<secp256k1::Scalar, secp256k1::GroupElement, PhantomData<()>>;

pub type ComDLProof = maurer::Proof<
    { maurer::SOUND_PROOFS_REPETITIONS },
    committment_of_discrete_log::Language<
        { secp256k1::SCALAR_LIMBS },
        secp256k1::Scalar,
        secp256k1::GroupElement,
        Pedersen<1, { secp256k1::SCALAR_LIMBS }, secp256k1::Scalar, secp256k1::GroupElement>,
    >,
    PhantomData<()>,
>;
pub type ComRatioProof = maurer::Proof<
    { maurer::SOUND_PROOFS_REPETITIONS },
    discrete_log_ratio_of_committed_values::Language<
        { secp256k1::SCALAR_LIMBS },
        secp256k1::Scalar,
        secp256k1::GroupElement,
    >,
    PhantomData<()>,
>;

pub type Secp256k1GroupElementValue = secp256k1::group_element::Value;

// -------------------------------------------------------------------------------------------------
// DKG Centralized Party
// -------------------------------------------------------------------------------------------------

pub type DKGSignatureMPCCentralizedCommitment = commitment::Commitment;

pub type DKGSignatureMPCCentralizedCommitmentRoundParty =
    twopc_mpc::dkg::centralized_party::commitment_round::Party<
        { secp256k1::SCALAR_LIMBS },
        { ristretto::SCALAR_LIMBS },
        { RANGE_CLAIMS_PER_SCALAR },
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
        bulletproofs::RangeProof,
        tiresias::RandomnessSpaceGroupElement,
        //ProtocolContext,
        PhantomData<()>,
    >;

pub type DKGSignatureMPCCentralizedPublicKeyShareDecommitmentAndProof =
    twopc_mpc::dkg::centralized_party::decommitment_round::PublicKeyShareDecommitmentAndProof<
        secp256k1::group_element::Value,
        SchnorrProof,
    >;

pub type DKGSignatureMPCCentralizedOutput = twopc_mpc::dkg::centralized_party::Output<
    secp256k1::group_element::Value,
    group::Value<secp256k1::scalar::Scalar>,
    tiresias::CiphertextSpaceValue,
>;

pub fn initiate_centralized_party_dkg(//paillier_public_parameters: &str, epoch: EpochId, party_id: PartyID, threshold: PartyID, number_of_parties: PartyID, session_id: SignatureMpcSessionID
) -> DKGSignatureMPCCentralizedCommitmentRoundParty {
    pub const N: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");

    // TODO: replace unwrap
    let paillier_public_parameters = tiresias::encryption_key::PublicParameters::new(N).unwrap();

    let secp256k1_scalar_public_parameters = secp256k1::scalar::PublicParameters::default();

    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();

    let bulletproofs_public_parameters =
        bulletproofs::PublicParameters::<{ RANGE_CLAIMS_PER_SCALAR }>::default();

    DKGSignatureMPCCentralizedCommitmentRoundParty {
        protocol_context: PhantomData::<()>,
        // protocol_context: ProtocolContext {
        //     epoch,
        //     party_id,
        //     number_of_parties,
        //     session_id,
        // },
        scalar_group_public_parameters: secp256k1_scalar_public_parameters.clone(),
        group_public_parameters: secp256k1_group_public_parameters.clone(),
        encryption_scheme_public_parameters: paillier_public_parameters.clone(),
        range_proof_public_parameters: bulletproofs_public_parameters.clone(),
        unbounded_encdl_witness_public_parameters: paillier_public_parameters
            .randomness_space_public_parameters()
            .clone(),
    }
}

// -------------------------------------------------------------------------------------------------
// DKG Decentralized Party Messages
// -------------------------------------------------------------------------------------------------

pub type DKGSignatureMPCDecentralizedOutput =
    twopc_mpc::dkg::decentralized_party::decommitment_proof_verification_round::Output<
        secp256k1::group_element::Value,
        tiresias::CiphertextSpaceValue,
    >;

// -------------------------------------------------------------------------------------------------
// DKG Decentralized Party Parties
// -------------------------------------------------------------------------------------------------

pub fn initiate_decentralized_party_dkg(
    tiresias_public_parameters: EncryptionPublicParameters,
    epoch: EpochId,
    party_id: PartyID,
    parties: HashSet<PartyID>,
    session_id: SignatureMPCSessionID,
) -> TwopcMPCResult<DKGSignatureMPCEncryptionOfSecretKeyShareRoundParty> {
    let secp256k1_scalar_public_parameters = secp256k1::scalar::PublicParameters::default();

    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();

    let bulletproofs_public_parameters =
        bulletproofs::PublicParameters::<{ RANGE_CLAIMS_PER_SCALAR }>::default();

    Ok(DKGSignatureMPCEncryptionOfSecretKeyShareRoundParty {
        party_id,
        parties,
        protocol_context: PhantomData::<()>,
        // protocol_context: ProtocolContext {
        //     epoch,
        //     party_id,
        //     number_of_parties,
        //     session_id,
        // },
        scalar_group_public_parameters: secp256k1_scalar_public_parameters.clone(),
        group_public_parameters: secp256k1_group_public_parameters.clone(),
        encryption_scheme_public_parameters: tiresias_public_parameters.clone(),
        unbounded_encdl_witness_public_parameters: tiresias_public_parameters
            .randomness_space_public_parameters()
            .clone(),
        range_proof_public_parameters: bulletproofs_public_parameters.clone(),
    })
}

pub fn decentralized_party_dkg_verify_decommitment_and_proof_of_centralized_party_public_key_share(
    paillier_public_parameters: &str,
    commitment_to_centralized_party_secret_key_share: DKGSignatureMPCCentralizedCommitment,
    centralized_party_public_key_share_decommitment_and_proof: DKGSignatureMPCCentralizedPublicKeyShareDecommitmentAndProof,
    secret_key_share_encryption_and_proof: DKGSignatureMPCSecretKeyShareEncryptionAndProof,
) -> DKGSignatureMPCDecentralizedOutput {
    // TODO: replace unwrap
    let paillier_public_parameters = tiresias::encryption_key::PublicParameters::new(
        LargeBiPrimeSizedNumber::from_be_hex(paillier_public_parameters),
    )
    .unwrap();
    let secp256k1_scalar_public_parameters = secp256k1::scalar::PublicParameters::default();

    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();

    let decommitment_proof_verification_round =
        DKGSignatureMPCDecommitmentProofVerificationRoundParty {
            protocol_context: PhantomData::<()>,
            // protocol_context: ProtocolContext {
            //     epoch,
            //     party_id,
            //     number_of_parties,
            //     session_id,
            // },
            scalar_group_public_parameters: secp256k1_scalar_public_parameters.clone(),
            group_public_parameters: secp256k1_group_public_parameters.clone(),
            encryption_scheme_public_parameters: paillier_public_parameters,
            commitment_to_centralized_party_secret_key_share,

            _unbounded_witness_choice: PhantomData::<tiresias::RandomnessSpaceGroupElement>,
            _range_proof_choice: PhantomData::<bulletproofs::RangeProof>,
        };

    let output = decommitment_proof_verification_round
        .verify_decommitment_and_proof_of_centralized_party_public_key_share(
            centralized_party_public_key_share_decommitment_and_proof,
            secret_key_share_encryption_and_proof,
        )
        .unwrap();

    output
}

pub type DKGSignatureMPCEncryptionOfSecretKeyShareRoundParty =
    twopc_mpc::dkg::decentralized_party::encryption_of_secret_key_share_round::Party<
        { secp256k1::SCALAR_LIMBS },
        { ristretto::SCALAR_LIMBS },
        { RANGE_CLAIMS_PER_SCALAR },
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
        bulletproofs::RangeProof,
        tiresias::RandomnessSpaceGroupElement,
        //ProtocolContext,
        PhantomData<()>,
    >;

pub type DKGSignatureMPCDecommitmentProofVerificationRoundParty =
    twopc_mpc::dkg::decentralized_party::decommitment_proof_verification_round::Party<
        { secp256k1::SCALAR_LIMBS },
        { ristretto::SCALAR_LIMBS },
        { RANGE_CLAIMS_PER_SCALAR },
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
        bulletproofs::RangeProof,
        tiresias::RandomnessSpaceGroupElement,
        //ProtocolContext,
        PhantomData<()>,
    >;

//
// pub type DKGSignatureMPCSecretShare = schnorr::enhanced::StatementSpaceGroupElement<
//     { maurer::SOUND_PROOFS_REPETITIONS },
//     { RANGE_CLAIMS_PER_SCALAR },
//     { bulletproofs::COMMITMENT_SCHEME_MESSAGE_SPACE_SCALAR_LIMBS },
//     bulletproofs::RangeProof,
//     tiresias::RandomnessSpaceGroupElement,
//     encryption_of_discrete_log::Language<
//         { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
//         { secp256k1::SCALAR_LIMBS },
//         secp256k1::GroupElement,
//         tiresias::EncryptionKey,
//     >
// >;
// pub type DKGSignatureMPCSecretShareValue = <DKGSignatureMPCSecretShare as group::GroupElement>::Value;

pub type DKGSignatureMPCSecretKeyShareEncryptionAndProof =
    twopc_mpc::dkg::decentralized_party::SecretKeyShareEncryptionAndProof<
        secp256k1::group_element::Value,
        range::CommitmentSchemeCommitmentSpaceValue<
            { ristretto::SCALAR_LIMBS },
            { RANGE_CLAIMS_PER_SCALAR },
            bulletproofs::RangeProof,
        >,
        tiresias::CiphertextSpaceValue,
        EncDLProof,
    >;

// -------------------------------------------------------------------------------------------------
// Presign Centralized Party
// -------------------------------------------------------------------------------------------------

pub fn initiate_centralized_party_presign(
    // paillier_public_parameters: &str,
    // epoch: EpochId,
    // party_id: PartyID,
    // parties: HashSet<PartyID>,
    // session_id: SignatureMPCSessionID,
    dkg_output: DKGSignatureMPCCentralizedOutput,
) -> TwopcMPCResult<PresignSignatureMPCCentralizedCommitmentRoundParty> {
    pub const N: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");

    // TODO: replace unwrap
    let paillier_public_parameters = tiresias::encryption_key::PublicParameters::new(N).unwrap();

    let secp256k1_scalar_public_parameters = secp256k1::scalar::PublicParameters::default();

    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();

    let bulletproofs_public_parameters =
        bulletproofs::PublicParameters::<{ RANGE_CLAIMS_PER_SCALAR }>::default();

    let unbounded_encdl_witness_public_parameters = paillier_public_parameters
        .randomness_space_public_parameters()
        .clone();

    let unbounded_encdh_witness_public_parameters = self_product::PublicParameters::new(
        paillier_public_parameters
            .randomness_space_public_parameters()
            .clone(),
    );

    PresignSignatureMPCCentralizedCommitmentRoundParty::new(
        PhantomData::<()>,
        // protocol_context: ProtocolContext {
        //     epoch,
        //     party_id,
        //     number_of_parties,
        //     session_id,
        // },
        secp256k1_scalar_public_parameters.clone(),
        secp256k1_group_public_parameters.clone(),
        paillier_public_parameters.clone(),
        unbounded_encdl_witness_public_parameters.clone(),
        unbounded_encdh_witness_public_parameters.clone(),
        bulletproofs_public_parameters.clone(),
        dkg_output,
    )
}

pub type PresignSignatureMPCCentralizedCommitmentRoundParty =
    twopc_mpc::presign::centralized_party::commitment_round::Party<
        { secp256k1::SCALAR_LIMBS },
        { ristretto::SCALAR_LIMBS },
        { RANGE_CLAIMS_PER_SCALAR },
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
        bulletproofs::RangeProof,
        tiresias::RandomnessSpaceGroupElement,
        self_product::GroupElement<2, tiresias::RandomnessSpaceGroupElement>,
        //ProtocolContext,
        PhantomData<()>,
    >;

pub type PresignSignatureMPCCentralizedDecommitmentRoundParty =
    twopc_mpc::presign::centralized_party::proof_verification_round::Party<
        { secp256k1::SCALAR_LIMBS },
        { ristretto::SCALAR_LIMBS },
        { RANGE_CLAIMS_PER_SCALAR },
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
        tiresias::RandomnessSpaceGroupElement,
        self_product::GroupElement<2, tiresias::RandomnessSpaceGroupElement>,
        bulletproofs::RangeProof,
        //ProtocolContext,
        PhantomData<()>,
    >;

pub type PresignSignatureMPCCentralizedSignatureNonceSharesCommitmentsAndBatchedProof = twopc_mpc::presign::centralized_party::commitment_round::SignatureNonceSharesCommitmentsAndBatchedProof::<
    { secp256k1::SCALAR_LIMBS },
    secp256k1::group_element::Value,
    maurer::Proof<
        { maurer::SOUND_PROOFS_REPETITIONS },
        maurer::knowledge_of_decommitment::Language<
            { maurer::SOUND_PROOFS_REPETITIONS },
            { secp256k1::SCALAR_LIMBS },
            commitment::Pedersen<1, { secp256k1::SCALAR_LIMBS }, secp256k1::Scalar, secp256k1::GroupElement>,
        >,
        //ProtocolContext,
        PhantomData<()>,
    >,
>;

pub type PresignSignatureMPCCentralizedPartyPresign =
    twopc_mpc::presign::centralized_party::Presign<
        secp256k1::group_element::Value,
        secp256k1::Scalar,
        tiresias::CiphertextSpaceValue,
    >;

// -------------------------------------------------------------------------------------------------
// Presign Decentralized Party Messages
// -------------------------------------------------------------------------------------------------

pub type PresignSignatureMPCDecentralizedPartyOutput =
    twopc_mpc::presign::decentralized_party::Output<
        secp256k1::group_element::Value,
        range::CommitmentSchemeCommitmentSpaceValue<
            { ristretto::SCALAR_LIMBS },
            { RANGE_CLAIMS_PER_SCALAR },
            bulletproofs::RangeProof,
        >,
        tiresias::CiphertextSpaceValue,
        EncDHProof,
        EncDLProof,
    >;

pub type PresignSignatureMPCMasksAndEncryptedMaskedKeyShare =
    encryption_of_tuple::StatementSpaceGroupElement<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        tiresias::EncryptionKey,
    >;

pub type PresignSignatureMPCEncryptedNonceShareAndPublicShare =
    encryption_of_discrete_log::StatementSpaceGroupElement<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
    >;

// -------------------------------------------------------------------------------------------------
// Presign Decentralized Party Parties
// -------------------------------------------------------------------------------------------------

pub type PresignSignatureMPCDecentralizedEncryptedMaskedKeyShareRoundParty = twopc_mpc::presign::decentralized_party::encrypted_masked_key_share_and_public_nonce_shares_round::Party::<
    { secp256k1::SCALAR_LIMBS },
    { ristretto::SCALAR_LIMBS },
    { RANGE_CLAIMS_PER_SCALAR },
    { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
    secp256k1::GroupElement,
    tiresias::EncryptionKey,
    bulletproofs::RangeProof,
    tiresias::RandomnessSpaceGroupElement,
    self_product::GroupElement<2, tiresias::RandomnessSpaceGroupElement>,
    //ProtocolContext,
    PhantomData<()>,
>;

pub type PresignSignatureMPCDecentralizedEncryptedMaskedNoncesRoundParty =
    twopc_mpc::presign::decentralized_party::encrypted_masked_nonces_round::Party<
        { secp256k1::SCALAR_LIMBS },
        { ristretto::SCALAR_LIMBS },
        { RANGE_CLAIMS_PER_SCALAR },
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
        bulletproofs::RangeProof,
        self_product::GroupElement<2, tiresias::RandomnessSpaceGroupElement>,
        //ProtocolContext,
        PhantomData<()>,
    >;

pub type PresignSignatureMPCDecentralizedPartyPresign =
    twopc_mpc::presign::decentralized_party::Presign<
        secp256k1::group_element::Value,
        tiresias::CiphertextSpaceValue,
    >;

pub type IndividualEncryptedNonceSharesAndPublicShares = HashMap<PartyID,
    Vec<
        encryption_of_discrete_log::StatementSpaceGroupElement<
            { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
            { secp256k1::SCALAR_LIMBS },
            secp256k1::GroupElement,
            tiresias::EncryptionKey,
        >,>,
>;
pub type IndividualEncryptedMaskedNonceShares = HashMap<PartyID,
    Vec<encryption_of_tuple::StatementSpaceGroupElement<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        tiresias::EncryptionKey,
    >,>,
>;


pub fn new_decentralized_party_presign_batch(
    parties: HashSet<PartyID>,
    commitments_and_proof_to_centralized_party_nonce_shares: PresignSignatureMPCCentralizedSignatureNonceSharesCommitmentsAndBatchedProof,
    mask_and_encrypted_masked_key_shares: Vec<PresignSignatureMPCMasksAndEncryptedMaskedKeyShare>,
    individual_encrypted_nonce_shares_and_public_shares: IndividualEncryptedNonceSharesAndPublicShares,

    encrypted_nonce_share_and_public_shares: Vec<
        PresignSignatureMPCEncryptedNonceShareAndPublicShare,
    >,
    individual_encrypted_masked_nonce_shares: IndividualEncryptedMaskedNonceShares,
    encrypted_masked_nonce_shares: Vec<PresignSignatureMPCMasksAndEncryptedMaskedKeyShare>,
) -> TwopcMPCResult<Vec<PresignSignatureMPCDecentralizedPartyPresign>> {
    // TODO: fix individual_encrypted_nonce_shares_and_public_shares and individual_encrypted_nonce_shares_and_public_shares
    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();
    PresignSignatureMPCDecentralizedPartyPresign::new_batch::<
        { secp256k1::SCALAR_LIMBS },
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        secp256k1::GroupElement,
        tiresias::EncryptionKey,
        //ProtocolContext,
        PhantomData<()>,
    >(
        parties.clone(),
        commitments_and_proof_to_centralized_party_nonce_shares,
        mask_and_encrypted_masked_key_shares,
        parties.clone().into_iter().map(|p| (p, encrypted_nonce_share_and_public_shares.clone())).collect(),
        encrypted_nonce_share_and_public_shares,
        parties.clone().into_iter().map(|p| (p, encrypted_masked_nonce_shares.clone())).collect(),
        encrypted_masked_nonce_shares,
        &secp256k1_group_public_parameters,
    )
}

pub type EncryptedDecentralizedPartySecretKeyShare = tiresias::CiphertextSpaceGroupElement;
pub type EncryptedDecentralizedPartySecretKeyShareValue =
    <tiresias::CiphertextSpaceGroupElement as group::GroupElement>::Value;

pub fn initiate_decentralized_party_presign(
    tiresias_public_parameters: EncryptionPublicParameters,
    epoch: EpochId,
    party_id: PartyID,
    parties: HashSet<PartyID>,
    session_id: SignatureMPCSessionID,
    dkg_output: DKGSignatureMPCDecentralizedOutput,
) -> TwopcMPCResult<PresignSignatureMPCDecentralizedEncryptedMaskedKeyShareRoundParty> {
    let secp256k1_scalar_public_parameters = secp256k1::scalar::PublicParameters::default();

    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();

    let bulletproofs_public_parameters =
        bulletproofs::PublicParameters::<{ RANGE_CLAIMS_PER_SCALAR }>::default();

    let unbounded_encdl_witness_public_parameters = tiresias_public_parameters
        .randomness_space_public_parameters()
        .clone();

    let unbounded_encdh_witness_public_parameters = self_product::PublicParameters::new(
        tiresias_public_parameters
            .randomness_space_public_parameters()
            .clone(),
    );

    PresignSignatureMPCDecentralizedEncryptedMaskedKeyShareRoundParty::new(
        party_id,
        parties,
        PhantomData::<()>,
        // protocol_context: ProtocolContext {
        //     epoch,
        //     party_id,
        //     number_of_parties,
        //     session_id,
        // },
        secp256k1_scalar_public_parameters.clone(),
        secp256k1_group_public_parameters.clone(),
        tiresias_public_parameters.clone(),
        unbounded_encdl_witness_public_parameters,
        unbounded_encdh_witness_public_parameters,
        bulletproofs_public_parameters.clone(),
        dkg_output,
    )
}

// -------------------------------------------------------------------------------------------------
// Sign Centralized Party
// -------------------------------------------------------------------------------------------------

pub type SignSignatureMPCCentralizedParty = twopc_mpc::sign::centralized_party::Party<
    { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
    { secp256k1::SCALAR_LIMBS },
    { RANGE_CLAIMS_PER_SCALAR },
    { RANGE_CLAIMS_PER_MASK },
    { ristretto::SCALAR_LIMBS },
    { NUM_RANGE_CLAIMS },
    secp256k1::GroupElement,
    tiresias::EncryptionKey,
    bulletproofs::RangeProof,
    group::direct_product::GroupElement<
        self_product::GroupElement<DIMENSION, secp256k1::Scalar>,
        tiresias::RandomnessSpaceGroupElement,
    >,
    //ProtocolContext,
    PhantomData<()>,
>;

pub type SignSignatureMPCCentralizedPublicNonceEncryptedPartialSignatureAndProof =
    twopc_mpc::sign::centralized_party::PublicNonceEncryptedPartialSignatureAndProof<
        secp256k1::group_element::Value,
        range::CommitmentSchemeCommitmentSpaceValue<
            { ristretto::SCALAR_LIMBS },
            NUM_RANGE_CLAIMS,
            bulletproofs::RangeProof,
        >,
        tiresias::CiphertextSpaceValue,
        ComDLProof,
        ComRatioProof,
        DComEvalProof,
    >;

pub fn initiate_centralized_party_sign(
    //paillier_public_parameters: &str, epoch: EpochId, party_id: PartyID, threshold: PartyID, number_of_parties: PartyID, session_id: SignatureMpcSessionID
    dkg_output: DKGSignatureMPCCentralizedOutput,
    presigns: Vec<PresignSignatureMPCCentralizedPartyPresign>,
) -> TwopcMPCResult<Vec<SignSignatureMPCCentralizedParty>> {
    pub const N: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");

    // TODO: replace unwrap
    let secp256k1_scalar_public_parameters = secp256k1::scalar::PublicParameters::default();

    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();

    let bulletproofs_public_parameters =
        bulletproofs::PublicParameters::<{ NUM_RANGE_CLAIMS }>::default();

    let paillier_public_parameters = tiresias::encryption_key::PublicParameters::new(N).unwrap();

    let unbounded_dcom_eval_witness_public_parameters = group::direct_product::PublicParameters(
        self_product::PublicParameters::new(secp256k1_scalar_public_parameters.clone()),
        paillier_public_parameters
            .randomness_space_public_parameters()
            .clone(),
    );

    presigns
        .into_iter()
        .map(|p| {
            SignSignatureMPCCentralizedParty::new(
                PhantomData::<()>,
                secp256k1_scalar_public_parameters.clone(),
                secp256k1_group_public_parameters.clone(),
                paillier_public_parameters.clone(),
                unbounded_dcom_eval_witness_public_parameters.clone(),
                bulletproofs_public_parameters.clone(),
                dkg_output.clone(),
                p,
            )
        })
        .collect()
}
pub fn initiate_decentralized_party_sign(
    //paillier_public_parameters: &str, epoch: EpochId, party_id: PartyID, threshold: PartyID, number_of_parties: PartyID, session_id: SignatureMpcSessionID
    tiresias_key_share_decryption_key_share: SecretKeyShareSizedNumber,
    tiresias_decryption_key_share_public_parameters: DecryptionPublicParameters,
    epoch: EpochId,
    party_id: PartyID,
    parties: HashSet<PartyID>,
    session_id: SignatureMPCSessionID,
    dkg_output: DKGSignatureMPCDecentralizedOutput,
    presigns: Vec<PresignSignatureMPCDecentralizedPartyPresign>,
) -> TwopcMPCResult<Vec<SignSignatureMPCDecentralizedParty>> {
    let secp256k1_scalar_public_parameters = secp256k1::scalar::PublicParameters::default();

    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();

    let bulletproofs_public_parameters =
        bulletproofs::PublicParameters::<{ NUM_RANGE_CLAIMS }>::default();

    let paillier_public_parameters = tiresias_decryption_key_share_public_parameters.encryption_scheme_public_parameters.clone();

    let unbounded_dcom_eval_witness_public_parameters = group::direct_product::PublicParameters(
        self_product::PublicParameters::new(secp256k1_scalar_public_parameters.clone()),
        paillier_public_parameters
            .randomness_space_public_parameters()
            .clone(),
    );

    let decryption_key_share = DecryptionKeyShare::new(
        party_id,
        tiresias_key_share_decryption_key_share,
        &tiresias_decryption_key_share_public_parameters,
    )?;
    presigns
        .into_iter()
        .map(|p| {
            SignSignatureMPCDecentralizedParty::new(
                decryption_key_share.clone(),
                tiresias_decryption_key_share_public_parameters.clone(),
                PhantomData::<()>,
                secp256k1_scalar_public_parameters.clone(),
                secp256k1_group_public_parameters.clone(),
                paillier_public_parameters.clone(),
                unbounded_dcom_eval_witness_public_parameters.clone(),
                bulletproofs_public_parameters.clone(),
                dkg_output.clone(),
                p,
            )
        })
        .collect()
}
pub fn decrypt_signature_decentralized_party_sign(
    public_key: Secp256k1GroupElementValue,
    messages: Vec<Vec<u8>>,
    lagrange_coefficients: HashMap<PartyID, AdjustedLagrangeCoefficientSizedNumber>,
    tiresias_decryption_key_share_public_parameters: DecryptionPublicParameters,
    decryption_shares: HashMap<PartyID, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>>,
    public_nonce_encrypted_partial_signature_and_proofs: Vec<SignSignatureMPCCentralizedPublicNonceEncryptedPartialSignatureAndProof>
) -> TwopcMPCResult<Vec<Vec<u8>>> {
    let secp256k1_scalar_public_parameters = secp256k1::scalar::PublicParameters::default();

    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();


    let decryption_shares: Vec<(HashMap<_, _>, HashMap<_, _>)> = (0..public_nonce_encrypted_partial_signature_and_proofs.len())
        .map(|i| {
            decryption_shares
                .iter()
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
    let public_key = secp256k1::GroupElement::new(public_key, &secp256k1_group_public_parameters)?;

    messages.into_iter().zip(public_nonce_encrypted_partial_signature_and_proofs.into_iter()).zip(decryption_shares.into_iter()).map(|((message, public_nonce_encrypted_partial_signature_and_proof), (partial_signature_decryption_shares, masked_nonce_decryption_shares))| {

        let m = message_digest(&message);
        let nonce_x_coordinate = secp256k1::GroupElement::new(public_nonce_encrypted_partial_signature_and_proof.public_nonce, &secp256k1_group_public_parameters)?.x();

        let (nonce_x_coordinate, signature) = SignSignatureMPCDecentralizedParty::decrypt_signature(
                m,
                public_key,
                nonce_x_coordinate,
                lagrange_coefficients.clone(),
                &tiresias_decryption_key_share_public_parameters,
                secp256k1_scalar_public_parameters.clone(),
                partial_signature_decryption_shares,
                masked_nonce_decryption_shares,
            )?;

        let signature_s: k256::Scalar = signature.into();

        Ok(Signature::<k256::Secp256k1>::from_scalars(k256::Scalar::from(nonce_x_coordinate), signature_s).unwrap().to_vec())
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
// Sign Decentralized Party Messages
// -------------------------------------------------------------------------------------------------

//HERE

// -------------------------------------------------------------------------------------------------
// Sign Decentralized Party Parties
// -------------------------------------------------------------------------------------------------

pub type SignSignatureMPCDecentralizedParty = twopc_mpc::sign::decentralized_party::Party<
    { secp256k1::SCALAR_LIMBS },
    { ristretto::SCALAR_LIMBS },
    { RANGE_CLAIMS_PER_SCALAR },
    { RANGE_CLAIMS_PER_MASK },
    { NUM_RANGE_CLAIMS },
    { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
    secp256k1::GroupElement,
    tiresias::EncryptionKey,
    DecryptionKeyShare,
    bulletproofs::RangeProof,
    group::direct_product::GroupElement<
        self_product::GroupElement<DIMENSION, secp256k1::Scalar>,
        tiresias::RandomnessSpaceGroupElement,
    >,
    //ProtocolContext,
    PhantomData<()>,
>;

// -------------------------------------------------------------------------------------------------
// MPC common
// -------------------------------------------------------------------------------------------------

const SESSION_ID_LENGTH: usize = 32;
/// The session id of the mpc is working on.
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignatureMPCSessionID(pub [u8; SESSION_ID_LENGTH]);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolContext {
    epoch: EpochId,
    party_id: PartyID,
    number_of_parties: PartyID,
    session_id: SignatureMPCSessionID,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SignatureMPCBulletProofAggregatesMessage {
    Commitment((Vec<EncDHCommitment>, Vec<EncDLCommitment>)),
    Decommitment((Vec<EncDHDecommitment>, Vec<EncDLDecommitment>)),
    ProofShare((Vec<EncDHProofShare>, Vec<EncDLProofShare>)),
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
        commitment_to_centralized_party_secret_key_share: DKGSignatureMPCCentralizedCommitment,
        secret_key_share_encryption_and_proof: DKGSignatureMPCSecretKeyShareEncryptionAndProof,
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
        output: PresignSignatureMPCDecentralizedPartyOutput,
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
        presigns: Vec<PresignSignatureMPCDecentralizedPartyPresign>,
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
        commitment_to_centralized_party_secret_key_share: DKGSignatureMPCCentralizedCommitment,
    },
    Presign {
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        dkg_output: DKGSignatureMPCDecentralizedOutput,
        commitments_and_proof_to_centralized_party_nonce_shares:
            PresignSignatureMPCCentralizedSignatureNonceSharesCommitmentsAndBatchedProof,
    },
    Sign {
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        public_key: Secp256k1GroupElementValue,
        messages: Vec<Vec<u8>>,
        dkg_output: DKGSignatureMPCDecentralizedOutput,
        public_nonce_encrypted_partial_signature_and_proofs: Vec<SignSignatureMPCCentralizedPublicNonceEncryptedPartialSignatureAndProof>,
        presigns: Vec<PresignSignatureMPCDecentralizedPartyPresign>,
    },
}
