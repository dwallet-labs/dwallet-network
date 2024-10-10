use std::collections::HashSet;
use crate::signature_mpc::mpc_manager::CreatableParty;
use group::{secp256k1, PartyID};
use maurer::knowledge_of_discrete_log::PublicParameters;
use maurer::Proof;
use rand_core::OsRng;
use std::marker::PhantomData;
use maurer::test_helpers::sample_witnesses;
use proof::aggregation::Instantiatable;

pub type ProofParty = proof::aggregation::asynchronous::Party<
    Proof<{ maurer::SOUND_PROOFS_REPETITIONS }, Lang, PhantomData<()>>,
>;

type Lang = maurer::knowledge_of_discrete_log::Language<secp256k1::Scalar, secp256k1::GroupElement>;

impl ProofParty {
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

type ProofPublicParameters =
    maurer::language::PublicParameters<{ maurer::SOUND_PROOFS_REPETITIONS }, Lang>;

fn generate_language_public_parameters<const REPETITIONS: usize>() -> ProofPublicParameters {
    let secp256k1_scalar_public_parameters = secp256k1::scalar::PublicParameters::default();

    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();

    PublicParameters::new::<secp256k1::Scalar, secp256k1::GroupElement>(
        secp256k1_scalar_public_parameters,
        secp256k1_group_public_parameters.clone(),
        secp256k1_group_public_parameters.generator,
    )
}
