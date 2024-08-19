use std::marker::PhantomData;

use crate::twopc_mpc_protocols::Secp256K1GroupElement;
use anyhow::Result;
use commitment::GroupsPublicParametersAccessors;
use crypto_bigint::{Uint, U256};
use enhanced_maurer::encryption_of_discrete_log::StatementAccessors;
use enhanced_maurer::language::EnhancedLanguageStatementAccessors;
use enhanced_maurer::{
    encryption_of_discrete_log, EnhancedLanguage, Proof, PublicParameters as MaurerPublicParameters,
};
use group::{secp256k1, GroupElement, Samplable};
use homomorphic_encryption::{
    AdditivelyHomomorphicDecryptionKey,
    GroupsPublicParametersAccessors as PublicParametersAccessors,
};
use maurer::{language, SOUND_PROOFS_REPETITIONS};
use proof::range;
use proof::range::bulletproofs;
use proof::range::bulletproofs::{COMMITMENT_SCHEME_MESSAGE_SPACE_SCALAR_LIMBS, RANGE_CLAIM_BITS};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use tiresias::{
    CiphertextSpaceGroupElement, CiphertextSpaceValue, DecryptionKey, EncryptionKey,
    PlaintextSpaceGroupElement, RandomnessSpaceGroupElement,
};
use twopc_mpc::paillier::PLAINTEXT_SPACE_SCALAR_LIMBS;
use twopc_mpc::secp256k1::paillier::bulletproofs::DKGDecentralizedPartyOutput;
pub use twopc_mpc::secp256k1::{Scalar as Secp256k1Scalar, SCALAR_LIMBS};

type LangPublicParams = language::PublicParameters<SOUND_PROOFS_REPETITIONS, EncDescLogLang>;

const RANGE_CLAIMS_PER_SCALAR: usize = Uint::<{ secp256k1::SCALAR_LIMBS }>::BITS / RANGE_CLAIM_BITS;

type EncDescLogLang = encryption_of_discrete_log::Language<
    { tiresias::PLAINTEXT_SPACE_SCALAR_LIMBS },
    { U256::LIMBS },
    secp256k1::GroupElement,
    EncryptionKey,
>;

type SecretShareProof = Proof<
    { SOUND_PROOFS_REPETITIONS },
    RANGE_CLAIMS_PER_SCALAR,
    COMMITMENT_SCHEME_MESSAGE_SPACE_SCALAR_LIMBS,
    bulletproofs::RangeProof,
    RandomnessSpaceGroupElement,
    EncDescLogLang,
    PhantomData<()>,
>;

/// Struct to hold the encrypted user share, the proof of the encryption,
/// and range proof commitment.
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

/// Generate a keypair for the Paillier encryption scheme.
pub fn generate_keypair() -> Result<(Vec<u8>, Vec<u8>)> {
    let (encryption_key, decryption_key) = DecryptionKey::generate(&mut OsRng)?;
    let decryption_key = bcs::to_bytes(&decryption_key.secret_key)?;
    let encryption_key = bcs::to_bytes(&encryption_key)?;
    Ok((encryption_key, decryption_key))
}

/// Create a public parameters object for encryption of discrete log language.
/// # Parameters
/// `encryption_key`: The public key (of the Paillier encryption scheme) to encrypt to,
/// serialized as bcs bytes.
pub fn encryption_of_discrete_log_public_parameters(
    encryption_key: Vec<u8>,
) -> Result<LangPublicParams> {
    let secp256k1_scalar_public_parameters = secp256k1::scalar::PublicParameters::default();
    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();

    let paillier_public_parameters: tiresias::encryption_key::PublicParameters =
        bcs::from_bytes(&encryption_key)?;

    let generator = secp256k1_group_public_parameters.generator;

    Ok(encryption_of_discrete_log::PublicParameters::<
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
    ))
}

/// Generate a proof of the user share encryption.
/// The encryption scheme is Maurer with a range proof (Enhanced Maurer).
pub fn generate_proof(
    encryption_key: Vec<u8>,
    user_share: Vec<u8>,
    language_public_parameters: encryption_of_discrete_log::PublicParameters<
        PLAINTEXT_SPACE_SCALAR_LIMBS,
        SCALAR_LIMBS,
        twopc_mpc::secp256k1::GroupElement,
        EncryptionKey,
    >,
) -> Result<EncryptedUserShareAndProof> {
    let paillier_public_parameters: tiresias::encryption_key::PublicParameters =
        bcs::from_bytes(&encryption_key)?;

    let unbounded_witness_public_parameters = language_public_parameters
        .randomness_space_public_parameters()
        .clone();

    let plaintext = PlaintextSpaceGroupElement::new(
        (&U256::from_be_slice(&user_share)).into(),
        paillier_public_parameters.plaintext_space_public_parameters(),
    )?;
    let randomness = RandomnessSpaceGroupElement::sample(
        language_public_parameters
            .encryption_scheme_public_parameters
            .randomness_space_public_parameters(),
        &mut OsRng,
    )?;

    let enhanced_language_public_parameters = MaurerPublicParameters::new::<
        bulletproofs::RangeProof,
        RandomnessSpaceGroupElement,
        EncDescLogLang,
    >(
        unbounded_witness_public_parameters,
        bulletproofs::PublicParameters::default(),
        language_public_parameters,
    )?;

    let witness: language::WitnessSpaceGroupElement<1, EncDescLogLang> =
        (plaintext, randomness).into();
    let witness =
        EnhancedLanguage::<
            { SOUND_PROOFS_REPETITIONS },
            RANGE_CLAIMS_PER_SCALAR,
            { COMMITMENT_SCHEME_MESSAGE_SPACE_SCALAR_LIMBS },
            bulletproofs::RangeProof,
            RandomnessSpaceGroupElement,
            EncDescLogLang,
        >::generate_witness(witness, &enhanced_language_public_parameters, &mut OsRng)?;

    let (proofs, statements) = SecretShareProof::prove(
        &PhantomData,
        &enhanced_language_public_parameters,
        vec![witness],
        &mut OsRng,
    )?;

    Ok(EncryptedUserShareAndProof {
        proof: proofs,
        encrypted_user_share: statements[0]
            .language_statement()
            .encrypted_discrete_log()
            .value(),
        range_proof_commitment: statements[0].range_proof_commitment().value(),
    })
}

/// Verify the proof of the user share encryption.
pub fn verify_proof(
    language_public_parameters: LangPublicParams,
    proof_public_output: EncryptedUserShareAndProof,
    centralized_public_key_share: group::Value<secp256k1::GroupElement>,
) -> Result<()> {
    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();
    let range_proof_enc_dl_public_parameters =
        bulletproofs::PublicParameters::<RANGE_CLAIMS_PER_SCALAR>::default();

    let unbounded_witness_public_parameters = language_public_parameters
        .randomness_space_public_parameters()
        .clone();

    let language_public_parameters1 = language_public_parameters.clone();
    let enhanced_language_public_parameters = MaurerPublicParameters::new::<
        bulletproofs::RangeProof,
        RandomnessSpaceGroupElement,
        EncDescLogLang,
    >(
        unbounded_witness_public_parameters,
        bulletproofs::PublicParameters::default(),
        language_public_parameters1,
    )?;

    let range_proof_commitment = range::CommitmentSchemeCommitmentSpaceGroupElement::<
        { COMMITMENT_SCHEME_MESSAGE_SPACE_SCALAR_LIMBS },
        { RANGE_CLAIMS_PER_SCALAR },
        bulletproofs::RangeProof,
    >::new(
        proof_public_output.range_proof_commitment,
        range_proof_enc_dl_public_parameters
            .commitment_scheme_public_parameters
            .commitment_space_public_parameters(),
    )?;

    let public_key_share = group::secp256k1::group_element::GroupElement::new(
        centralized_public_key_share,
        &secp256k1_group_public_parameters,
    )?;

    let encrypted_secret_share: CiphertextSpaceGroupElement = CiphertextSpaceGroupElement::new(
        proof_public_output.encrypted_user_share,
        language_public_parameters
            .encryption_scheme_public_parameters
            .ciphertext_space_public_parameters(),
    )?;

    let statement = (
        range_proof_commitment,
        (encrypted_secret_share, public_key_share).into(),
    )
        .into();

    Ok(proof_public_output.proof.verify(
        &PhantomData,
        &enhanced_language_public_parameters,
        vec![statement],
        &mut OsRng,
    )?)
}

/// Decrypt the user share using the Paillier decryption key.
pub fn decrypt_user_share(
    encryption_key: Vec<u8>,
    decryption_key: Vec<u8>,
    encrypted_user_share_and_proof: EncryptedUserShareAndProof,
) -> Result<Vec<u8>> {
    let paillier_public_parameters: tiresias::encryption_key::PublicParameters =
        bcs::from_bytes(&encryption_key)?;
    let ciphertext = CiphertextSpaceGroupElement::new(
        encrypted_user_share_and_proof.encrypted_user_share,
        paillier_public_parameters.ciphertext_space_public_parameters(),
    )?;
    let decryption_key = bcs::from_bytes(&decryption_key)?;
    let decryption_key = DecryptionKey::new(decryption_key, &paillier_public_parameters)?;
    // Safe to `unwrap` as decryption in Paillier always succeeds
    let plaintext = decryption_key
        .decrypt(&ciphertext, &paillier_public_parameters)
        .unwrap();
    Ok(bcs::to_bytes(&plaintext.value())?)
}

fn secret_key_matches_public_key(
    secret_key: Secp256k1Scalar,
    public_key: Secp256K1GroupElement,
) -> Result<bool> {
    let public_parameters = secp256k1::group_element::PublicParameters::default();
    let generator_group_element =
        Secp256K1GroupElement::new(public_parameters.generator, &public_parameters)?;
    Ok(secret_key * generator_group_element == public_key)
}

/// Parses the secret share & DKG output, and verifies that the secret share
/// matches the public key share.
pub fn parse_and_verify_secret_share(secret_share: &[u8], dkg_output: &[u8]) -> Result<bool> {
    let parsed_secret_key =
        Secp256k1Scalar::from(Uint::<{ SCALAR_LIMBS }>::from_be_slice(secret_share));
    let dkg_output = bcs::from_bytes::<DKGDecentralizedPartyOutput>(dkg_output)?;
    let public_share = Secp256K1GroupElement::new(
        dkg_output.centralized_party_public_key_share,
        &secp256k1::group_element::PublicParameters::default(),
    )?;
    secret_key_matches_public_key(parsed_secret_key, public_share)
}

#[cfg(test)]
mod tests {
    use twopc_mpc::secp256k1::paillier::bulletproofs::DKGDecentralizedPartyOutput;

    use crate::twopc_mpc_protocols::encrypt_user_share::{
        encryption_of_discrete_log_public_parameters, generate_keypair, generate_proof,
    };

    use super::*;

    /// This is a valid DKG output that contains the public
    /// key share that matches the secret key share.
    const PUBLIC_DKG_OUTPUT: &str = "210264B04A7E32CA125C99C242A75ABFCF26EC6F815B3144A5E43A19AB1BBF1265852103B30BAFA4DB6353F42FBCEC9587373E3508DF986926A05890FDC4AF0ECF8D25B57ECE177918775F5E9CA540D8E0581748DD5FE17C4A8D1E23521A168E65699B7A590A892D34672985E4C35045313BF7F4C622063F9C4699A2C958E085A3FEBB07CEF331D44EE250FEB2D267492FE9B54C979296DFD487CEB8CA461A35DFD65C8B0541CCB7576904BD91A05C1A24E3B3C2506E05292C77045A83A8D4B769AAC3F8324130B609F51FC5F9FB810A25CBF1B4676BF0567F1C8D531ADC3BFDEA29070BC3A6F209F9D5CEEAC061E2DAD75217919252C7841B4D8D19097EC51F427247D09B96034394621FF6AECD40408B15B0820BEFB74928A2CB749E9524016730F0BE78DDCFAEC0F1AD505504F8C7EE29C4D7CE8BBEFF1F2C8C62B4105CD316C42D3D9410A8FD776A8D1885D96BEAE5A37B909147F3762CA18C2C0353AF26817AD36BF09F80E01C53A664904FCEB3AE434465BDAFCAE41384D8762609D659F3ECFC825DD3845B6908E2BD502B828A0D0A36DB326E0BD01ADA06AE0A46AEC3FF736FF8ADF129A0C75EF84106289931CC4A1C8E2812D8ACCC054FA54549B5197B2A5323E29868C6094944C95F60023A7D11464921A5C0C3126604FFC3732E6F84FB619EB6D9ACF01453186DF9B6AE2B4D8EE6666B6907FFAD2A36FA81480A16FF797CD2EE6D51FE3C84C593BB30666B5D7BCD5AC5E6DC8BC88773668049580A216F470E387FA507AB8AEB6719BA99029B77829C52EAADEA14D4DB5231B752FB2EB1A80621025D26C33D01846D86CF204CCA70FB457E66E3B66E11CF67ECA6E93ADF71DC9230";
    const SECRET_KEY_SHARE: &str =
        "CA1D77DDAA83254CAE618319F2A916E5081A969D06E9448D97524626D59C2A06";

    #[test]
    fn verify_valid_proof_successfully() {
        let dkg_output = hex::decode(PUBLIC_DKG_OUTPUT).unwrap();
        let dkg_output = bcs::from_bytes::<DKGDecentralizedPartyOutput>(&dkg_output);
        let centralized_party_public_key_share =
            dkg_output.unwrap().centralized_party_public_key_share;
        let user_share = hex::decode(SECRET_KEY_SHARE).expect("Decoding failed");

        let (encryption_key, decryption_key) = generate_keypair().unwrap();

        let language_public_parameters =
            encryption_of_discrete_log_public_parameters(encryption_key.clone()).unwrap();

        let encrypted_user_share_and_proof = match generate_proof(
            encryption_key.clone(),
            user_share.clone(),
            language_public_parameters.clone(),
        ) {
            Ok(proof) => proof,
            Err(e) => panic!("Error generating proof: {:?}", e),
        };

        assert!(verify_proof(
            language_public_parameters,
            encrypted_user_share_and_proof.clone(),
            centralized_party_public_key_share,
        )
        .is_ok());

        let decrypted = decrypt_user_share(
            encryption_key.clone(),
            decryption_key.clone(),
            encrypted_user_share_and_proof,
        )
        .unwrap();

        // Slice the first 32 bytes from user share.
        let mut user_share_mut = user_share;
        user_share_mut.reverse();
        assert_eq!(decrypted[0..32].to_vec(), user_share_mut);
    }
}
