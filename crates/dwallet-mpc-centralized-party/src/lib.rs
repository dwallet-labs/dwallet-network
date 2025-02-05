//! This crate contains the cryptographic logic for the centralized 2PC-MPC party.

// Allowed to improve code readability.
#![allow(unused_qualifications)]

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
use k256::{elliptic_curve, U256};
use mpc::two_party::Round;
use rand_core::{OsRng, SeedableRng};
use std::fmt;
use std::marker::PhantomData;
use twopc_mpc::secp256k1::SCALAR_LIMBS;

use class_groups_constants::{
    protocol_public_parameters, public_keys_from_dkg_output, DWalletPublicKeys,
};
use twopc_mpc::languages::class_groups::{
    construct_encryption_of_discrete_log_public_parameters, EncryptionOfDiscreteLogProofWithoutCtx,
};
use twopc_mpc::{secp256k1, ProtocolPublicParameters};

type AsyncProtocol = secp256k1::class_groups::AsyncProtocol;
type DKGCentralizedParty = <AsyncProtocol as twopc_mpc::dkg::Protocol>::DKGCentralizedParty;
pub type SignCentralizedParty = <AsyncProtocol as twopc_mpc::sign::Protocol>::SignCentralizedParty;
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

/// Executes the second phase of the DKG protocol, part of a three-phase DKG flow.
///
/// This function is invoked by the centralized party to produce:
/// - A public key share and its proof.
/// - Centralized DKG output required for further protocol steps.
/// # Warning
/// The secret (private) key returned from this function should never be sent,
/// and should always be kept private.
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
/// This is okay since a malicious blockchain can always block a client.
pub fn create_dkg_output(
    protocol_public_parameters: Vec<u8>,
    key_scheme: u8,
    decentralized_first_round_public_output: Vec<u8>,
    session_id: String,
) -> anyhow::Result<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> {
    let decentralized_first_round_public_output: EncryptionOfSecretKeyShareAndPublicKeyShare =
        bcs::from_bytes(&decentralized_first_round_public_output)
            .context("Failed to deserialize decentralized first round output")?;
    let public_parameters = bcs::from_bytes(&protocol_public_parameters_by_key_scheme(
        protocol_public_parameters,
        key_scheme,
    )?)?;

    let session_id = commitment::CommitmentSizedNumber::from_le_hex(&session_id);

    let round_result = DKGCentralizedParty::advance(
        decentralized_first_round_public_output.clone(),
        &(),
        &(public_parameters, session_id).into(),
        &mut OsRng,
    )
    .context("advance() failed on the DKGCentralizedParty")?;

    // Centralized Public Key Share and Proof.
    let public_key_share_and_proof = bcs::to_bytes(&round_result.outgoing_message)?;
    // Public Output:
    // centralized_public_key_share + public_key + decentralized_party_public_key_share
    let public_output = bcs::to_bytes(&round_result.public_output)?;
    // Centralized Secret Key Share.
    // Warning:
    // The secret (private) key share returned from this function should never be sent,
    // and should always be kept private.
    let centralized_secret_output = bcs::to_bytes(&round_result.private_output)?;
    let public_keys = bcs::to_bytes(&DWalletPublicKeys {
        centralized_public_share: bcs::to_bytes(&round_result.public_output.public_key_share)?,
        decentralized_public_share: bcs::to_bytes(
            &round_result
                .public_output
                .decentralized_party_public_key_share,
        )?,
        public_key: bcs::to_bytes(&round_result.public_output.public_key)?,
    })?;
    Ok((
        public_key_share_and_proof,
        public_output,
        centralized_secret_output,
        public_keys,
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

/// Executes the centralized phase of the Sign protocol,
/// first part of the protocol.
///
/// The [`advance_centralized_sign_party`] function is
/// called by the client (the centralized party).
pub fn advance_centralized_sign_party(
    protocol_public_parameters: Vec<u8>,
    key_scheme: u8,
    decentralized_party_dkg_public_output: Vec<u8>,
    centralized_party_secret_key_share: Vec<u8>,
    presigns: Vec<Vec<u8>>,
    messages: Vec<Vec<u8>>,
    hash_type: u8,
    presign_session_ids: Vec<String>,
) -> anyhow::Result<Vec<SignedMessages>> {
    let decentralized_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput = bcs::from_bytes(&decentralized_party_dkg_public_output)?;
    let centralized_public_output = twopc_mpc::class_groups::DKGCentralizedPartyOutput::<
        { secp256k1::SCALAR_LIMBS },
        secp256k1::GroupElement,
    > {
        public_key_share: decentralized_output.centralized_party_public_key_share,
        public_key: decentralized_output.public_key,
        decentralized_party_public_key_share: decentralized_output.public_key_share,
    };
    let signed_messages: Vec<_> = messages
        .iter()
        .enumerate()
        .map(|(index, message)| {
            let session_id =
                commitment::CommitmentSizedNumber::from_le_hex(&presign_session_ids[index]);
            let presign: <AsyncProtocol as twopc_mpc::presign::Protocol>::Presign =
                bcs::from_bytes(&presigns[index])?;
            let hashed_message = message_digest(&message, &hash_type.try_into()?)
                .context("Message digest failed")?;
            let centralized_party_public_input =
                <AsyncProtocol as twopc_mpc::sign::Protocol>::SignCentralizedPartyPublicInput::from(
                    (
                        hashed_message,
                        centralized_public_output.clone(),
                        presign,
                        bcs::from_bytes(&protocol_public_parameters_by_key_scheme(
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

            Ok(bcs::to_bytes(&round_result.outgoing_message)?)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(signed_messages)
}

fn protocol_public_parameters_by_key_scheme(
    protocol_public_parameters: Vec<u8>,
    key_scheme: u8,
) -> anyhow::Result<Vec<u8>> {
    let key_scheme = DWalletMPCNetworkKeyScheme::try_from(key_scheme)?;
    let encryption_scheme_public_parameters = bcs::from_bytes(&protocol_public_parameters)?;
    match key_scheme {
        DWalletMPCNetworkKeyScheme::Secp256k1 => {
            Ok(bcs::to_bytes(&ProtocolPublicParameters::new::<
                { secp256k1::SCALAR_LIMBS },
                { SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS },
                { SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS },
                secp256k1::GroupElement,
            >(
                encryption_scheme_public_parameters
            ))?)
        }
        DWalletMPCNetworkKeyScheme::Ristretto => {
            todo!()
        }
    }
}

/// Derives a Secp256k1 class groups keypair from a given seed.
///
/// The class groups public encryption key being used to encrypt a Secp256k1 keypair will be
/// different from the encryption key used to encrypt a Ristretto keypair.
/// The plaintext space/fundamental group will correspond to the order
/// of the respective elliptic curve.
/// The secret decryption key may be the same in terms of correctness,
/// but to simplify security analysis and implementation current version maintain distinct key-pairs.
/// # Warning
/// The secret (private) key returned from this function should never be sent,
/// and should always be kept private.
pub fn generate_secp256k1_cg_keypair_from_seed_internal(
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

/// Encrypts the given secret key share with the given encryption key.
/// Returns a serialized tuple containing the `proof of encryption`,
/// and an encrypted `secret key share`.
pub fn encrypt_secret_key_share_and_prove(
    secret_key_share: Vec<u8>,
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
    let parsed_secret_key_share = bcs::from_bytes(&secret_key_share)?;
    let witness = (parsed_secret_key_share, randomness).into();
    let (proof, statements) = EncryptionOfSecretShareProof::prove(
        &PhantomData,
        &language_public_parameters,
        vec![witness],
        &mut OsRng,
    )?;
    // todo(scaly): why is it derived from statements?
    let (encryption_of_discrete_log, _) = statements.first().unwrap().clone().into();
    Ok(bcs::to_bytes(&(proof, encryption_of_discrete_log.value()))?)
}

/// Verifies the given secret share matches the given dWallets`
/// DKG output centralized_party_public_key_share.
pub fn verify_secret_share(secret_share: Vec<u8>, dkg_output: Vec<u8>) -> anyhow::Result<bool> {
    let expected_public_key = cg_secp256k1_public_key_share_from_secret_share(secret_share)?;
    let dkg_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput =
        bcs::from_bytes(&dkg_output)?;
    Ok(dkg_output.centralized_party_public_key_share == expected_public_key.value())
}

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

/// Derives a dWallets` public key share from a private key share.
fn cg_secp256k1_public_key_share_from_secret_share(
    secret_key_share: Vec<u8>,
) -> anyhow::Result<group::secp256k1::GroupElement> {
    let public_parameters = group::secp256k1::group_element::PublicParameters::default();
    let generator_group_element =
        group::secp256k1::group_element::GroupElement::generator_from_public_parameters(
            &public_parameters,
        )?;
    Ok(generator_group_element.scale(&Uint::<{ SCALAR_LIMBS }>::from_be_slice(&secret_key_share)))
}

/// Derives [`DWalletPublicKeys`] from the given dwallet DKG output.
pub fn public_keys_from_dwallet_output(output: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    bcs::to_bytes(&public_keys_from_dkg_output(bcs::from_bytes(&output)?)?).map_err(Into::into)
}
