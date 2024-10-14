use crate::signature_mpc::mpc_events::{CompletedProofMPCSessionEvent, CreatedProofMPCEvent, InitDKGMPCEvent};
use crate::signature_mpc::mpc_manager::CreatableParty;
use group::{secp256k1, PartyID, Samplable};
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKey;
use maurer::knowledge_of_discrete_log::PublicParameters;
use maurer::Language;
use proof::GroupsPublicParametersAccessors;
use rand_core::CryptoRngCore;
use std::collections::HashSet;
use std::iter;
use std::marker::PhantomData;
use tiresias::test_helpers::{N, SECRET_KEY};
use twopc_mpc::secp256k1::paillier::bulletproofs::PaillierProtocolPublicParameters;
// TODO (#228): Remove this file & all proof MPC code.

/// Create dummy witnesses for the dummy proof flow.
fn sample_witnesses<const REPETITIONS: usize, Lang: Language<REPETITIONS>>(
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
pub type AsyncProtocol=
    twopc_mpc::secp256k1::paillier::bulletproofs::AsyncProtocol<PhantomData<()>>;
pub type DKGParty =
    <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareRoundParty;

pub fn setup_paillier_secp256k1() -> (PaillierProtocolPublicParameters, tiresias::DecryptionKey) {
    let paillier_protocol_public_parameters =
        PaillierProtocolPublicParameters::new::<secp256k1::GroupElement>(N);

    let decryption_key = tiresias::DecryptionKey::new(
        SECRET_KEY,
        &paillier_protocol_public_parameters
            .protocol_public_parameters
            .encryption_scheme_public_parameters,
    )
    .unwrap();

    (paillier_protocol_public_parameters, decryption_key)
}

impl CreatableParty for DKGParty {
    type InitEvent = InitDKGMPCEvent;
    type FinalizeEvent = CompletedProofMPCSessionEvent;

    fn new(parties: HashSet<PartyID>, party_id: PartyID) -> Self {
        Self::default()
    }

    fn first_auxiliary_input() -> Self::AuxiliaryInput {
        let (paillier_protocol_public_parameters, decryption_key) = setup_paillier_secp256k1();

        let (secp256k1_group_public_parameters, _) = setup_paillier_secp256k1();

        let parties = (0..3).collect::<HashSet<PartyID>>();
        Self::AuxiliaryInput {
            protocol_public_parameters: secp256k1_group_public_parameters,
            party_id: 1,
            threshold: 3,
            number_of_parties: 4,
            parties: parties.clone(),
            protocol_context: PhantomData,
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
