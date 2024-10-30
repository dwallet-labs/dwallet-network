use crate::signature_mpc::mpc_events::{
    CompletedProofMPCSessionEvent, CreatedDKGSessionEvent, CreatedProofMPCEvent, MPCEvent,
};
use crate::signature_mpc::mpc_manager::{CreatableParty, SignatureMPCInstance};
use group::{secp256k1, PartyID, Samplable};
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKey;
use maurer::knowledge_of_discrete_log::PublicParameters;
use maurer::Language;
use mpc::{Advance, Party};
use proof::GroupsPublicParametersAccessors;
use rand_core::CryptoRngCore;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::iter;
use std::marker::PhantomData;
use twopc_mpc::dkg::Protocol;
// TODO (#228): Remove this file & all proof MPC code.

/// Create dummy witnesses for the dummy proof flow.
pub fn sample_witnesses<const REPETITIONS: usize, Lang: Language<REPETITIONS>>(
    language_public_parameters: &Lang::PublicParameters,
    batch_size: usize,
    rng: &mut impl CryptoRngCore,
) -> Vec<Lang::WitnessSpaceGroupElement> {
    iter::repeat_with(|| {
        Lang::WitnessSpaceGroupElement::sample(
            language_public_parameters.witness_space_public_parameters(),
            rng,
        )
        .unwrap()
    })
    .take(batch_size)
    .collect()
}

/// A party in the proof MPC flow.
pub type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;
pub type DKGFirstParty =
    <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareRoundParty;

pub type DKGSecondParty = <AsyncProtocol as twopc_mpc::dkg::Protocol>::ProofVerificationRoundParty;

pub trait AuxiliarySecond: mpc::Party {
    fn first_auxiliary_input(
        first_round_output: <DKGFirstParty as mpc::Party>::Output,
        centralized_party_public_key_share: <AsyncProtocol as twopc_mpc::dkg::Protocol>::PublicKeyShareAndProof,
        session_is: Vec<u8>,
    ) -> Self::AuxiliaryInput;
}

impl AuxiliarySecond for DKGSecondParty {
    fn first_auxiliary_input(
        first_round_output: <DKGFirstParty as Party>::Output,
        centralized_party_public_key_share: <AsyncProtocol as Protocol>::PublicKeyShareAndProof,
        session_id: Vec<u8>,
    ) -> Self::AuxiliaryInput {
        let first_round_auxiliary_input = DKGFirstParty::first_auxiliary_input(session_id);
        (
            first_round_auxiliary_input,
            first_round_output,
            centralized_party_public_key_share,
        )
            .into()
    }
}

pub trait AuxiliaryFirst: mpc::Party {
    fn first_auxiliary_input(session_id: Vec<u8>) -> Self::AuxiliaryInput;
}

impl AuxiliaryFirst for DKGFirstParty {
    fn first_auxiliary_input(session_id: Vec<u8>) -> Self::AuxiliaryInput {
        let secp256k1_group_public_parameters =
            class_groups_constants::protocol_public_parameters().unwrap();

        let parties = (0..3).collect::<HashSet<PartyID>>();
        let session_id = commitment::CommitmentSizedNumber::from_be_slice(&vec![]);
        Self::AuxiliaryInput {
            protocol_public_parameters: secp256k1_group_public_parameters,
            party_id: 1,
            threshold: 3,
            number_of_parties: 4,
            parties: parties.clone(),
            session_id,
        }
    }
}

/// The language used in the proof MPC flow.
type Lang = maurer::knowledge_of_discrete_log::Language<secp256k1::Scalar, secp256k1::GroupElement>;

/// The public parameters for the proof MPC flow.
type ProofPublicParameters =
    maurer::language::PublicParameters<{ maurer::SOUND_PROOFS_REPETITIONS }, Lang>;

/// Generate the public parameters for the proof MPC flow.
fn generate_language_public_parameters<const REPETITIONS: usize>() -> ProofPublicParameters {
    let secp256k1_scalar_public_parameters = secp256k1::scalar::PublicParameters::default();

    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();

    PublicParameters::new::<secp256k1::Scalar, secp256k1::GroupElement>(
        secp256k1_scalar_public_parameters,
        secp256k1_group_public_parameters.clone(),
        secp256k1_group_public_parameters.generator,
    )
}
