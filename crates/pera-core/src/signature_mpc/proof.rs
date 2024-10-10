use crate::signature_mpc::mpc_events::{CompletedProofMPCSessionEvent, CreatedProofMPCEvent};
use crate::signature_mpc::mpc_manager::CreatableParty;
use group::{secp256k1, PartyID, Samplable};
use maurer::knowledge_of_discrete_log::PublicParameters;
use maurer::{Language, Proof};
use proof::aggregation::Instantiatable;
use proof::GroupsPublicParametersAccessors;
use rand_core::{CryptoRngCore, OsRng};
use std::collections::HashSet;
use std::iter;
use std::marker::PhantomData;

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
pub type ProofParty = proof::aggregation::asynchronous::Party<
    Proof<{ maurer::SOUND_PROOFS_REPETITIONS }, Lang, PhantomData<()>>,
>;

/// The language used in the proof MPC flow.
type Lang = maurer::knowledge_of_discrete_log::Language<secp256k1::Scalar, secp256k1::GroupElement>;

impl CreatableParty for ProofParty {
    type InitEvent = CreatedProofMPCEvent;
    type FinalizeEvent = CompletedProofMPCSessionEvent;

    fn new(parties: HashSet<PartyID>, party_id: PartyID) -> Self {
        let public_parameters =
            generate_language_public_parameters::<{ maurer::SOUND_PROOFS_REPETITIONS }>();
        let batch_size = 1;
        let witnesses = sample_witnesses::<{ maurer::SOUND_PROOFS_REPETITIONS }, Lang>(
            &public_parameters,
            batch_size,
            &mut OsRng,
        );
        let threshold = (((parties.len() * 2) + 2) / 3) as PartyID;
        ProofParty::new_session(
            party_id,
            threshold,
            parties,
            PhantomData,
            public_parameters,
            witnesses,
            &mut OsRng,
        )
        .unwrap()
    }
}

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
