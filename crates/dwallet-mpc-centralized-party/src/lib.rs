//! This crate contains the cryptographic logic for the centralized 2PC-MPC party.

use anyhow::{anyhow, Context};
use class_groups::setup::get_setup_parameters_secp256k1;
use class_groups::{
    CiphertextSpaceGroupElement, CiphertextSpaceValue, DecryptionKey, EncryptionKey,
    Secp256k1DecryptionKey, SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};
use dwallet_mpc_types::dwallet_mpc::DWalletMPCNetworkKeyScheme;
use group::{CyclicGroupElement, GroupElement, Samplable};
use homomorphic_encryption::{
    AdditivelyHomomorphicDecryptionKey, AdditivelyHomomorphicEncryptionKey,
    GroupsPublicParametersAccessors,
};
use k256::ecdsa::hazmat::bits2field;
use k256::ecdsa::signature::digest::{Digest, FixedOutput};
use k256::elliptic_curve::bigint::{Encoding, Uint};
use k256::elliptic_curve::ops::Reduce;
use k256::elliptic_curve::{group::prime::PrimeCurveAffine, Group};
use k256::{elliptic_curve, U256};
use mpc::two_party::Round;
use rand_core::{OsRng, SeedableRng};
use std::fmt;
use std::marker::PhantomData;
use twopc_mpc::secp256k1::SCALAR_LIMBS;

use class_groups_constants::protocol_public_parameters;
use group::KnownOrderGroupElement;
use k256::elliptic_curve::subtle::CtOption;
use twopc_mpc::languages::class_groups::{
    construct_encryption_of_discrete_log_public_parameters, EncryptionOfDiscreteLogProofWithoutCtx,
};
use twopc_mpc::secp256k1::class_groups::{
    FUNDAMENTAL_DISCRIMINANT_LIMBS, NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};
use twopc_mpc::{secp256k1, ProtocolPublicParameters};

type AsyncProtocol = secp256k1::class_groups::AsyncProtocol;
type DKGCentralizedParty = <AsyncProtocol as twopc_mpc::dkg::Protocol>::DKGCentralizedParty;
type SignCentralizedParty = <AsyncProtocol as twopc_mpc::sign::Protocol>::SignCentralizedParty;
type EncryptionOfSecretKeyShareAndPublicKeyShare =
    <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareAndPublicKeyShare;
pub type NoncePublicShareAndEncryptionOfMaskedNonceSharePart =
<AsyncProtocol as twopc_mpc::presign::Protocol>::NoncePublicShareAndEncryptionOfMaskedNonceSharePart;

/// Supported hash functions for message digest.
#[derive(Clone, Debug)]
enum Hash {
    KECCAK256 = 0,
    SHA256 = 1,
}

type HashedMessages = Vec<u8>;
type SignedMessages = Vec<u8>;
type EncryptionOfSecretShareProof = EncryptionOfDiscreteLogProofWithoutCtx<
    SCALAR_LIMBS,
    SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    secp256k1::GroupElement,
>;

type Secp256k1EncryptionKey = EncryptionKey<
    SCALAR_LIMBS,
    SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    secp256k1::GroupElement,
>;

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hash_name = match self {
            Hash::KECCAK256 => "KECCAK256",
            Hash::SHA256 => "SHA256",
        };
        write!(f, "{}", hash_name)
    }
}

impl TryFrom<u8> for Hash {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Hash::KECCAK256),
            1 => Ok(Hash::SHA256),
            _ => Err(anyhow::Error::msg(format!(
                "invalid value for Hash enum: {}",
                value
            ))),
        }
    }
}

/// Executes the second phase of the DKG protocol, part of a three-phase DKG flow.
///
/// This function is invoked by the centralized party to produce:
/// - A public key share and its proof.
/// - Centralized DKG output required for further protocol steps.
///
/// # Parameters
/// — `decentralized_first_round_output`:
///    Serialized output of the decentralized party from the first DKG round.
/// — `session_id`: Unique hexadecimal string identifying the session.
///
/// # Returns
/// A tuple containing:
/// - Serialized public key share and proof.
/// - Serialized centralized DKG output.
///
/// # Errors
/// Returns an error if decoding or advancing the protocol fails.
pub fn create_dkg_output(
    protocol_public_parameters: Vec<u8>,
    key_scheme: u8,
    decentralized_first_round_output: Vec<u8>,
    session_id: String,
) -> anyhow::Result<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> {
    let decentralized_first_round_output: EncryptionOfSecretKeyShareAndPublicKeyShare =
        bcs::from_bytes(&decentralized_first_round_output)
            .context("Failed to deserialize decentralized first round output")?;
    let public_parameters = bcs::from_bytes(&get_protocol_public_parameters(
        protocol_public_parameters,
        key_scheme,
    )?)?;

    let session_id = commitment::CommitmentSizedNumber::from_le_hex(&session_id);

    let round_result = DKGCentralizedParty::advance(
        decentralized_first_round_output.clone(),
        &(),
        &(public_parameters, session_id).into(),
        &mut OsRng,
    )
    .context("advance() failed on the DKGCentralizedParty")?;

    let public_key_share_and_proof = bcs::to_bytes(&round_result.outgoing_message)?;
    let centralized_public_output = bcs::to_bytes(&round_result.public_output)?;
    let centralized_secret_output = bcs::to_bytes(&round_result.private_output)?;
    let centralized_public_share = bcs::to_bytes(&round_result.public_output.public_key_share)?;

    Ok((
        public_key_share_and_proof,
        centralized_public_output,
        centralized_secret_output,
        centralized_public_share,
    ))
}

/// Computes the message digest of a given message using the specified hash function.
fn message_digest(message: &[u8], hash_type: &Hash) -> anyhow::Result<secp256k1::Scalar> {
    let hash = match hash_type {
        Hash::KECCAK256 => bits2field::<k256::Secp256k1>(
            &sha3::Keccak256::new_with_prefix(message).finalize_fixed(),
        )
        .map_err(|e| anyhow::Error::msg(format!("KECCAK256 bits2field error: {:?}", e)))?,

        Hash::SHA256 => {
            bits2field::<k256::Secp256k1>(&sha2::Sha256::new_with_prefix(message).finalize_fixed())
                .map_err(|e| anyhow::Error::msg(format!("SHA256 bits2field error: {:?}", e)))?
        }
    };
    #[allow(clippy::useless_conversion)]
    let m = <elliptic_curve::Scalar<k256::Secp256k1> as Reduce<U256>>::reduce_bytes(&hash.into());
    Ok(U256::from(m).into())
}

/// Executes the centralized phase of the Sign protocol, first part of the protocol.
///
/// The [`create_sign_output`] function is called by the client (aka the centralized party).
///
/// The `session_id` is a unique identifier for the session, represented as a hexadecimal string.
/// The `hash` must fit the [`Hash`] enum.
pub fn create_sign_output(
    protocol_public_parameters: Vec<u8>,
    key_scheme: u8,
    centralized_party_dkg_output: Vec<u8>,
    centralized_party_secret_key_share: Vec<u8>,
    presigns: Vec<Vec<u8>>,
    messages: Vec<Vec<u8>>,
    hash: u8,
    session_ids: Vec<String>,
) -> anyhow::Result<(Vec<HashedMessages>, Vec<SignedMessages>)> {
    let centralized_party_dkg_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::CentralizedPartyDKGPublicOutput =
        bcs::from_bytes(&centralized_party_dkg_output)?;
    let (signed_messages, hashed_messages): (Vec<_>, Vec<_>) = messages
        .into_iter()
        .enumerate()
        .map(|(index, message)| {
            let session_id = commitment::CommitmentSizedNumber::from_le_hex(&session_ids[index]);
            let presign: <AsyncProtocol as twopc_mpc::presign::Protocol>::Presign =
                bcs::from_bytes(&presigns[index])?;
            let hashed_message =
                message_digest(&message, &hash.try_into()?).context("Message digest failed")?;
            let centralized_party_public_input =
                <AsyncProtocol as twopc_mpc::sign::Protocol>::SignCentralizedPartyPublicInput::from(
                    (
                        hashed_message,
                        centralized_party_dkg_output.clone(),
                        presign,
                        bcs::from_bytes(&get_protocol_public_parameters(
                            protocol_public_parameters.clone(),
                            key_scheme,
                        )?)?,
                        session_id,
                    ),
                );

            let round_result = SignCentralizedParty::advance(
                (),
                &bcs::from_bytes(&centralized_party_secret_key_share)?,
                &centralized_party_public_input,
                &mut OsRng,
            )
            .context("advance() failed on the SignCentralizedParty")?;

            let signed_message = bcs::to_bytes(&round_result.outgoing_message)?;
            let hashed_message_bytes = bcs::to_bytes(&hashed_message)?;
            Ok((signed_message, hashed_message_bytes))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .unzip();

    Ok((signed_messages, hashed_messages))
}

fn get_protocol_public_parameters(
    protocol_public_parameters: Vec<u8>,
    key_scheme: u8,
) -> anyhow::Result<Vec<u8>> {
    let key_scheme = DWalletMPCNetworkKeyScheme::try_from(key_scheme)?;

    match key_scheme {
        DWalletMPCNetworkKeyScheme::Secp256k1 => {
            Ok(bcs::to_bytes(&ProtocolPublicParameters::new::<
                { secp256k1::SCALAR_LIMBS },
                { FUNDAMENTAL_DISCRIMINANT_LIMBS },
                { NON_FUNDAMENTAL_DISCRIMINANT_LIMBS },
                secp256k1::GroupElement,
            >(bcs::from_bytes(
                &protocol_public_parameters,
            )?))?)
        }
        DWalletMPCNetworkKeyScheme::Ristretto => {
            todo!()
        }
    }
}

/// Derives a Secp256k1 class groups keypair from a given seed.
///
/// The class groups key being used to encrypt a Secp256k1 keypair should be different from
/// the encryption key used to encrypt a Ristretto keypair, due to cryptographic reasons.
/// This function derives a class groups keypair to encrypt a Secp256k1 secret from the given seed.
pub fn generate_secp_cg_keypair_from_seed_internal(
    seed: [u8; 32],
) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed);
    let setup_parameters = get_setup_parameters_secp256k1();
    let (encryption_key, decryption_key) =
        Secp256k1DecryptionKey::generate(setup_parameters, &mut rng)?;
    let decryption_key = bcs::to_bytes(&decryption_key.decryption_key)?;
    let encryption_key = bcs::to_bytes(&encryption_key)?;
    Ok((encryption_key, decryption_key))
}

pub fn centralized_public_share_from_decentralized_output_inner(
    dkg_output: Vec<u8>,
) -> anyhow::Result<Vec<u8>> {
    let dkg_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput =
        bcs::from_bytes(&dkg_output)?;
    bcs::to_bytes(&dkg_output.centralized_party_public_key_share).map_err(Into::into)
}

/// Encrypts the given secret share to the given encryption key.
/// Returns a tuple of the encryption key and proof of encryption.
pub fn encrypt_secret_share_and_prove(
    secret_share: Vec<u8>,
    encryption_key: Vec<u8>,
) -> anyhow::Result<Vec<u8>> {
    let protocol_public_params = protocol_public_parameters();
    let language_public_parameters = construct_encryption_of_discrete_log_public_parameters::<
        SCALAR_LIMBS,
        { SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS },
        { SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS },
        secp256k1::GroupElement,
    >(
        protocol_public_params
            .scalar_group_public_parameters
            .clone(),
        protocol_public_params.group_public_parameters.clone(),
        bcs::from_bytes(&encryption_key)?,
    );
    let randomness = class_groups::RandomnessSpaceGroupElement::<
        { SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS },
    >::sample(
        language_public_parameters
            .encryption_scheme_public_parameters
            .randomness_space_public_parameters(),
        &mut OsRng,
    )?;
    let parsed_secret_key =
        secp256k1::Scalar::from(Uint::<{ SCALAR_LIMBS }>::from_be_slice(&secret_share));
    let witness = (parsed_secret_key, randomness).into();
    let (proof, statements) = EncryptionOfSecretShareProof::prove(
        &PhantomData,
        &language_public_parameters,
        vec![witness],
        &mut OsRng,
    )?;
    let (encryption_of_discrete_log, _) = statements.first().unwrap().clone().into();
    Ok(bcs::to_bytes(&(proof, encryption_of_discrete_log.value()))?)
}

/// Verifies the given secret share matches the given DWallet's DKG output centralized_party_public_key_share.
pub fn verify_secret_share(secret_share: Vec<u8>, dkg_output: Vec<u8>) -> anyhow::Result<bool> {
    let expected_public_key = cg_public_share_from_secret_share(secret_share)?;
    let dkg_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput =
        bcs::from_bytes(&dkg_output)?;
    Ok(dkg_output.centralized_party_public_key_share == expected_public_key.value())
}

/// Decrypts the given encrypted user share using the given decryption key.
pub fn decrypt_user_share_inner(
    encryption_key: Vec<u8>,
    decryption_key: Vec<u8>,
    encrypted_user_share_and_proof: Vec<u8>,
) -> anyhow::Result<Vec<u8>> {
    let (_, encryption_of_discrete_log): (
        EncryptionOfSecretShareProof,
        CiphertextSpaceValue<SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS>,
    ) = bcs::from_bytes(&encrypted_user_share_and_proof)?;
    let public_parameters: homomorphic_encryption::PublicParameters<
        SCALAR_LIMBS,
        Secp256k1EncryptionKey,
    > = bcs::from_bytes(&encryption_key)?;
    let ciphertext = CiphertextSpaceGroupElement::new(
        encryption_of_discrete_log,
        &public_parameters.ciphertext_space_public_parameters(),
    )?;

    let decryption_key = bcs::from_bytes(&decryption_key)?;
    let decryption_key: DecryptionKey<
        SCALAR_LIMBS,
        SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
        SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
        secp256k1::GroupElement,
    > = DecryptionKey::new(decryption_key, &public_parameters)?;
    let Some(plaintext): Option<<Secp256k1EncryptionKey as AdditivelyHomomorphicEncryptionKey<SCALAR_LIMBS>>::PlaintextSpaceGroupElement> = decryption_key
        .decrypt(&ciphertext, &public_parameters).into() else {
        return Err(anyhow!("Decryption failed"));
    };
    let secret_share_bytes = U256::from(&plaintext.value()).to_be_bytes().to_vec();
    Ok(secret_share_bytes)
}

/// Derives a DWallet's public share from a private share.
fn cg_public_share_from_secret_share(
    secret_share: Vec<u8>,
) -> anyhow::Result<group::secp256k1::GroupElement> {
    let public_parameters = group::secp256k1::group_element::PublicParameters::default();
    let generator_group_element =
        group::secp256k1::group_element::GroupElement::generator_from_public_parameters(
            &public_parameters,
        )?;
    Ok(generator_group_element.scale(&Uint::<{ SCALAR_LIMBS }>::from_be_slice(&secret_share)))
}
