use crate::twopc_mpc_protocols::N;
use commitment::GroupsPublicParametersAccessors;
use crypto_bigint::{Uint, U256};
use enhanced_maurer::encryption_of_discrete_log::StatementAccessors;
use enhanced_maurer::language::EnhancedLanguageStatementAccessors;
use enhanced_maurer::{
    encryption_of_discrete_log, EnhanceableLanguage, EnhancedLanguage, Error, Proof,
    PublicParameters as MaurerPublicParameters, WitnessSpaceGroupElement,
};
use group::{secp256k1, GroupElement, Samplable};
use homomorphic_encryption::{
    AdditivelyHomomorphicDecryptionKey, AdditivelyHomomorphicEncryptionKey,
    GroupsPublicParametersAccessors as PublicParametersAccessors,
};
use maurer::{language, SOUND_PROOFS_REPETITIONS};
use proof::range;
use proof::range::bulletproofs;
use proof::range::bulletproofs::{COMMITMENT_SCHEME_MESSAGE_SPACE_SCALAR_LIMBS, RANGE_CLAIM_BITS};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use tiresias::{
    CiphertextSpaceGroupElement, CiphertextSpaceValue, DecryptionKey, EncryptionKey,
    LargeBiPrimeSizedNumber, PlaintextSpaceGroupElement, RandomnessSpaceGroupElement,
};
use twopc_mpc::paillier::PLAINTEXT_SPACE_SCALAR_LIMBS;
use twopc_mpc::secp256k1::paillier::bulletproofs::ProtocolPublicParameters;
pub use twopc_mpc::secp256k1::{Scalar, SCALAR_LIMBS};

pub type LangPublicParams = language::PublicParameters<SOUND_PROOFS_REPETITIONS, Lang>;

pub const RANGE_CLAIMS_PER_SCALAR: usize =
    Uint::<{ secp256k1::SCALAR_LIMBS }>::BITS / RANGE_CLAIM_BITS;

pub type Lang = encryption_of_discrete_log::Language<
    { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
    { U256::LIMBS },
    secp256k1::GroupElement,
    EncryptionKey,
>;

pub type SecretShareProof = Proof<
    { maurer::SOUND_PROOFS_REPETITIONS },
    RANGE_CLAIMS_PER_SCALAR,
    COMMITMENT_SCHEME_MESSAGE_SPACE_SCALAR_LIMBS,
    bulletproofs::RangeProof,
    RandomnessSpaceGroupElement,
    Lang,
    PhantomData<()>,
>;
pub(crate) type EnhancedLang<
    const REPETITIONS: usize,
    const NUM_RANGE_CLAIMS: usize,
    UnboundedWitnessSpaceGroupElement,
    Lang,
> = EnhancedLanguage<
    REPETITIONS,
    NUM_RANGE_CLAIMS,
    { COMMITMENT_SCHEME_MESSAGE_SPACE_SCALAR_LIMBS },
    range::bulletproofs::RangeProof,
    UnboundedWitnessSpaceGroupElement,
    Lang,
>;

#[derive(Serialize, Deserialize, Clone)]
pub struct EncryptedUserShareAndProof {
    pub proof: SecretShareProof,
    pub encrypted_user_share: CiphertextSpaceValue,
    pub range_proof_commitment: range::CommitmentSchemeCommitmentSpaceValue<
        { COMMITMENT_SCHEME_MESSAGE_SPACE_SCALAR_LIMBS },
        { RANGE_CLAIMS_PER_SCALAR },
        bulletproofs::RangeProof,
    >,
}

fn pad_vector(vec: Vec<u8>) -> Vec<u8> {
    let target_length = 256;
    if vec.len() >= target_length {
        return vec;
    }
    let mut padded_vec = vec![0; target_length - vec.len()];
    padded_vec.extend(vec);
    padded_vec
}

pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
    let (encryption_key, decryption_key) = DecryptionKey::generate(&mut OsRng).unwrap();
    let decryption_key = bcs::to_bytes(&decryption_key.secret_key).unwrap();
    let encryption_key = bcs::to_bytes(&encryption_key).unwrap();
    (encryption_key, decryption_key)
}

pub fn get_proof_public_parameters(pub_key: Vec<u8>) -> LangPublicParams {
    let secp256k1_scalar_public_parameters = secp256k1::scalar::PublicParameters::default();

    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();

    let paillier_public_parameters: tiresias::encryption_key::PublicParameters =
        bcs::from_bytes(&pub_key).unwrap();

    let generator = secp256k1_group_public_parameters.generator;

    encryption_of_discrete_log::PublicParameters::<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
        EncryptionKey,
    >::new::<
        { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
        EncryptionKey,
    >(
        secp256k1_scalar_public_parameters,
        secp256k1_group_public_parameters,
        paillier_public_parameters,
        generator,
    )
}

fn parse_plaintext(
    plaintext: Vec<u8>,
    public_parameters: &tiresias::encryption_key::PublicParameters,
) -> PlaintextSpaceGroupElement {
    let plaintext = pad_vector(plaintext);
    let plaintext: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_slice(&plaintext);
    let plaintext = PlaintextSpaceGroupElement::new(
        plaintext,
        public_parameters.plaintext_space_public_parameters(),
    )
    .unwrap();
    plaintext
}

pub fn generate_proof(
    encryption_key: Vec<u8>,
    user_share: Vec<u8>,
    language_public_parameters: encryption_of_discrete_log::PublicParameters<
        PLAINTEXT_SPACE_SCALAR_LIMBS,
        SCALAR_LIMBS,
        twopc_mpc::secp256k1::GroupElement,
        EncryptionKey,
    >,
) -> Result<EncryptedUserShareAndProof, Error> {
    let paillier_public_parameters: tiresias::encryption_key::PublicParameters =
        bcs::from_bytes(&encryption_key).unwrap();

    let unbounded_witness_public_parameters = language_public_parameters
        .randomness_space_public_parameters()
        .clone();

    let plaintext = parse_plaintext(user_share, &paillier_public_parameters);

    let randomness = RandomnessSpaceGroupElement::sample(
        language_public_parameters
            .encryption_scheme_public_parameters
            .randomness_space_public_parameters(),
        &mut OsRng,
    )
    .unwrap();

    let enhanced_language_public_parameters = enhanced_language_public_parameters::<
        { maurer::SOUND_PROOFS_REPETITIONS },
        RANGE_CLAIMS_PER_SCALAR,
        RandomnessSpaceGroupElement,
        Lang,
    >(
        unbounded_witness_public_parameters,
        language_public_parameters,
    );

    let witnesses = generate_witnesses(randomness, &enhanced_language_public_parameters, plaintext);

    let (proofs, statements) = match SecretShareProof::prove(
        &PhantomData,
        &enhanced_language_public_parameters,
        witnesses,
        &mut OsRng,
    ) {
        Ok((proofs, statements)) => (proofs, statements),
        Err(e) => {
            return Err(e);
        }
    };

    Ok(EncryptedUserShareAndProof {
        proof: proofs,
        encrypted_user_share: statements[0]
            .language_statement()
            .encrypted_discrete_log()
            .value(),
        range_proof_commitment: statements[0].range_proof_commitment().value(),
    })
}

fn generate_witnesses(
    randomness: RandomnessSpaceGroupElement,
    enhanced_language_public_parameters: &language::PublicParameters<
        { maurer::SOUND_PROOFS_REPETITIONS },
        EnhancedLang<
            { maurer::SOUND_PROOFS_REPETITIONS },
            RANGE_CLAIMS_PER_SCALAR,
            RandomnessSpaceGroupElement,
            Lang,
        >,
    >,
    plaintext: PlaintextSpaceGroupElement,
) -> Vec<
    WitnessSpaceGroupElement<
        { maurer::SOUND_PROOFS_REPETITIONS },
        RANGE_CLAIMS_PER_SCALAR,
        COMMITMENT_SCHEME_MESSAGE_SPACE_SCALAR_LIMBS,
        bulletproofs::RangeProof,
        RandomnessSpaceGroupElement,
        Lang,
    >,
> {
    let witnesses: Vec<language::WitnessSpaceGroupElement<1, Lang>> =
        vec![(plaintext, randomness).into()];

    let witnesses =
        EnhancedLanguage::<
            { maurer::SOUND_PROOFS_REPETITIONS },
            RANGE_CLAIMS_PER_SCALAR,
            { COMMITMENT_SCHEME_MESSAGE_SPACE_SCALAR_LIMBS },
            bulletproofs::RangeProof,
            RandomnessSpaceGroupElement,
            Lang,
        >::generate_witnesses(witnesses, &enhanced_language_public_parameters, &mut OsRng)
        .unwrap();
    witnesses
}

pub fn enhanced_language_public_parameters<
    const REPETITIONS: usize,
    const NUM_RANGE_CLAIMS: usize,
    UnboundedWitnessSpaceGroupElement: group::GroupElement + Samplable,
    Lang: EnhanceableLanguage<
        REPETITIONS,
        NUM_RANGE_CLAIMS,
        COMMITMENT_SCHEME_MESSAGE_SPACE_SCALAR_LIMBS,
        UnboundedWitnessSpaceGroupElement,
    >,
>(
    unbounded_witness_public_parameters: UnboundedWitnessSpaceGroupElement::PublicParameters,
    language_public_parameters: Lang::PublicParameters,
) -> language::PublicParameters<
    REPETITIONS,
    EnhancedLang<REPETITIONS, NUM_RANGE_CLAIMS, UnboundedWitnessSpaceGroupElement, Lang>,
> {
    MaurerPublicParameters::new::<
        bulletproofs::RangeProof,
        UnboundedWitnessSpaceGroupElement,
        Lang,
    >(
        unbounded_witness_public_parameters,
        bulletproofs::PublicParameters::default(),
        language_public_parameters,
    )
    .unwrap()
}
pub fn is_valid_proof(
    language_public_parameters: LangPublicParams,
    proof_public_output: EncryptedUserShareAndProof,
    centralized_public_keyshare: group::Value<secp256k1::GroupElement>,
) -> bool {
    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();
    let protocol_public_parameters = ProtocolPublicParameters::new(N);

    let unbounded_witness_public_parameters = language_public_parameters
        .randomness_space_public_parameters()
        .clone();

    let enhanced_language_public_parameters = enhanced_language_public_parameters::<
        { SOUND_PROOFS_REPETITIONS },
        RANGE_CLAIMS_PER_SCALAR,
        RandomnessSpaceGroupElement,
        Lang,
    >(
        unbounded_witness_public_parameters,
        language_public_parameters.clone(),
    );

    let range_proof_commitment = range::CommitmentSchemeCommitmentSpaceGroupElement::<
        { COMMITMENT_SCHEME_MESSAGE_SPACE_SCALAR_LIMBS },
        { RANGE_CLAIMS_PER_SCALAR },
        bulletproofs::RangeProof,
    >::new(
        proof_public_output.range_proof_commitment,
        protocol_public_parameters
            .range_proof_enc_dl_public_parameters
            .commitment_scheme_public_parameters
            .commitment_space_public_parameters(),
    )
    .unwrap();

    let public_key_share = group::secp256k1::group_element::GroupElement::new(
        centralized_public_keyshare,
        &secp256k1_group_public_parameters,
    )
    .unwrap();

    let encrypted_secret_share: CiphertextSpaceGroupElement = CiphertextSpaceGroupElement::new(
        proof_public_output.encrypted_user_share,
        language_public_parameters
            .encryption_scheme_public_parameters
            .ciphertext_space_public_parameters(),
    )
    .unwrap();

    let statement = (
        range_proof_commitment,
        (encrypted_secret_share, public_key_share.clone()).into(),
    )
        .into();

    proof_public_output
        .proof
        .verify(
            &PhantomData,
            &enhanced_language_public_parameters,
            vec![statement],
            &mut OsRng,
        )
        .is_ok()
}

pub fn decrypt_user_share(
    encryption_key: Vec<u8>,
    decryption_key: Vec<u8>,
    encrypted_user_share_and_proof: EncryptedUserShareAndProof,
) -> Vec<u8> {
    let paillier_public_parameters: tiresias::encryption_key::PublicParameters =
        bcs::from_bytes(&encryption_key).unwrap();
    let ciphertext = CiphertextSpaceGroupElement::new(
        encrypted_user_share_and_proof.encrypted_user_share,
        paillier_public_parameters
            .ciphertext_space_public_parameters(),
    )
    .unwrap();

    let decryption_key = bcs::from_bytes(&decryption_key).unwrap();
    let decryption_key = DecryptionKey::new(decryption_key, &paillier_public_parameters).unwrap();

    let plaintext = decryption_key
        .decrypt(&ciphertext, &paillier_public_parameters)
        .unwrap();
    let plaintext = bcs::to_bytes(&plaintext.value()).unwrap();
    plaintext
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::twopc_mpc_protocols::encrypt_user_share::{
        generate_keypair, generate_proof, get_proof_public_parameters,
    };
    use twopc_mpc::secp256k1::paillier::bulletproofs::DKGDecentralizedPartyOutput;

    /// This is a valid DKG output that contains the public key share that matches the secret key share
    const PUBLIC_DKG_OUTPUT: &str = "210264B04A7E32CA125C99C242A75ABFCF26EC6F815B3144A5E43A19AB1BBF1265852103B30BAFA4DB6353F42FBCEC9587373E3508DF986926A05890FDC4AF0ECF8D25B57ECE177918775F5E9CA540D8E0581748DD5FE17C4A8D1E23521A168E65699B7A590A892D34672985E4C35045313BF7F4C622063F9C4699A2C958E085A3FEBB07CEF331D44EE250FEB2D267492FE9B54C979296DFD487CEB8CA461A35DFD65C8B0541CCB7576904BD91A05C1A24E3B3C2506E05292C77045A83A8D4B769AAC3F8324130B609F51FC5F9FB810A25CBF1B4676BF0567F1C8D531ADC3BFDEA29070BC3A6F209F9D5CEEAC061E2DAD75217919252C7841B4D8D19097EC51F427247D09B96034394621FF6AECD40408B15B0820BEFB74928A2CB749E9524016730F0BE78DDCFAEC0F1AD505504F8C7EE29C4D7CE8BBEFF1F2C8C62B4105CD316C42D3D9410A8FD776A8D1885D96BEAE5A37B909147F3762CA18C2C0353AF26817AD36BF09F80E01C53A664904FCEB3AE434465BDAFCAE41384D8762609D659F3ECFC825DD3845B6908E2BD502B828A0D0A36DB326E0BD01ADA06AE0A46AEC3FF736FF8ADF129A0C75EF84106289931CC4A1C8E2812D8ACCC054FA54549B5197B2A5323E29868C6094944C95F60023A7D11464921A5C0C3126604FFC3732E6F84FB619EB6D9ACF01453186DF9B6AE2B4D8EE6666B6907FFAD2A36FA81480A16FF797CD2EE6D51FE3C84C593BB30666B5D7BCD5AC5E6DC8BC88773668049580A216F470E387FA507AB8AEB6719BA99029B77829C52EAADEA14D4DB5231B752FB2EB1A80621025D26C33D01846D86CF204CCA70FB457E66E3B66E11CF67ECA6E93ADF71DC9230";
    const SECRET_KEYSHARE: &str =
        "CA1D77DDAA83254CAE618319F2A916E5081A969D06E9448D97524626D59C2A06";

    #[test]
    fn verify_valid_proof_successfully() {
        let dgk_output = hex::decode(PUBLIC_DKG_OUTPUT).unwrap();
        let dgk_output = bcs::from_bytes::<DKGDecentralizedPartyOutput>(&dgk_output);
        let centralized_party_public_key_share =
            dgk_output.unwrap().centralized_party_public_key_share;
        let user_share = hex::decode(SECRET_KEYSHARE).expect("Decoding failed");

        let (encryption_key, decryption_key) = generate_keypair();
        let deserialized_pub_params: tiresias::encryption_key::PublicParameters =
            bcs::from_bytes(&encryption_key).unwrap();
        let language_public_parameters = get_proof_public_parameters(encryption_key.clone());

        let encrypted_user_share_and_proof = match generate_proof(
            encryption_key.clone(),
            user_share.clone(),
            language_public_parameters.clone(),
        ) {
            Ok(proof) => proof,
            Err(e) => panic!("Error generating proof: {:?}", e),
        };

        assert!(is_valid_proof(
            language_public_parameters,
            encrypted_user_share_and_proof.clone(),
            centralized_party_public_key_share,
        ));

        let decrypted = decrypt_user_share(
            encryption_key.clone(),
            decryption_key.clone(),
            encrypted_user_share_and_proof,
        );

        /// slice the first 32 bytes from user_share
        let mut user_share_mut = user_share;
        user_share_mut.reverse();
        assert_eq!(decrypted[0..32].to_vec(), user_share_mut);
    }
}
