//! This crate contains the cryptographic logic for the centralized 2PC-MPC party.

// Allowed to improve code readability.
#![allow(unused_qualifications)]

use anyhow::{anyhow, Context};
use class_groups::dkg::Secp256k1Party;
use class_groups::setup::get_setup_parameters_secp256k1;
use class_groups::{
    CiphertextSpaceGroupElement, DecryptionKey, EncryptionKey, Secp256k1DecryptionKey,
    SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS, SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, SerializedWrappedMPCPublicOutput,
    VersionedCentralizedDKGPublicOutput, VersionedDwalletDKGFirstRoundPublicOutput,
    VersionedDwalletDKGSecondRoundPublicOutput, VersionedDwalletUserSecretShare,
    VersionedEncryptedUserShare, VersionedImportedDWalletPublicOutput,
    VersionedImportedDwalletOutgoingMessage, VersionedNetworkDkgOutput, VersionedPresignOutput,
    VersionedPublicKeyShareAndProof, VersionedSignOutput, VersionedUserSignedMessage,
};
use group::{secp256k1, CyclicGroupElement, GroupElement, Samplable};
use homomorphic_encryption::{
    AdditivelyHomomorphicDecryptionKey, AdditivelyHomomorphicEncryptionKey,
    GroupsPublicParametersAccessors,
};
use mpc::two_party::Round;
use mpc::Party;
use rand_core::{OsRng, SeedableRng};
use twopc_mpc::secp256k1::SCALAR_LIMBS;

use serde::{Deserialize, Serialize};
use shared_wasm_class_groups::message_digest::message_digest;
use twopc_mpc::dkg::Protocol;
use twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters;
use twopc_mpc::sign::verify_signature;

type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;
type DKGCentralizedParty = <AsyncProtocol as twopc_mpc::dkg::Protocol>::DKGCentralizedPartyRound;
pub type SignCentralizedParty = <AsyncProtocol as twopc_mpc::sign::Protocol>::SignCentralizedParty;

/// Contains the public keys of the DWallet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, Hash)]
pub struct DWalletPublicKeys {
    pub centralized_public_share: Vec<u8>,
    pub decentralized_public_share: Vec<u8>,
    pub public_key: Vec<u8>,
}
pub type DKGDecentralizedOutput =
    <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput;

/// Extracts [`DWalletPublicKeys`] from the given [`DKGDecentralizedOutput`].
// Can't use the TryFrom trait as it leads to conflicting implementations.
// Must use `anyhow::Result`, because this function is being used also
// in the centralized party crate.
fn public_keys_from_dkg_output(value: DKGDecentralizedOutput) -> anyhow::Result<DWalletPublicKeys> {
    Ok(DWalletPublicKeys {
        centralized_public_share: bcs::to_bytes(&value.centralized_party_public_key_share)?,
        decentralized_public_share: bcs::to_bytes(&value.public_key_share)?,
        public_key: bcs::to_bytes(&value.public_key)?,
    })
}

type SignedMessage = Vec<u8>;

type Secp256k1EncryptionKey = EncryptionKey<
    SCALAR_LIMBS,
    SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    secp256k1::GroupElement,
>;

type ImportSecretKeyFirstStep =
    <AsyncProtocol as twopc_mpc::dkg::Protocol>::TrustedDealerDKGCentralizedPartyRound;

pub struct CentralizedDKGWasmResult {
    pub public_key_share_and_proof: Vec<u8>,
    pub public_output: Vec<u8>,
    pub centralized_secret_output: Vec<u8>,
}

/// Executes the second phase of the DKG protocol, part of a three-phase DKG flow.
///
/// This function is invoked by the centralized party to produce:
/// - A public key share and its proof.
/// - Centralized DKG output required for further protocol steps.
/// # Warning
/// The secret (private) key returned from this function should never be sent
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
/// Return an error if decoding or advancing the protocol fails.
/// This is okay since a malicious blockchain can always block a client.
pub fn create_dkg_output(
    network_dkg_public_output: SerializedWrappedMPCPublicOutput,
    key_scheme: u32,
    decentralized_first_round_public_output: SerializedWrappedMPCPublicOutput,
    session_id: String,
) -> anyhow::Result<CentralizedDKGWasmResult> {
    let decentralized_first_round_public_output =
        bcs::from_bytes(&decentralized_first_round_public_output)?;
    match decentralized_first_round_public_output {
        VersionedDwalletDKGFirstRoundPublicOutput::V1(decentralized_first_round_public_output) => {
            let (decentralized_first_round_public_output, _): <<AsyncProtocol as Protocol>::EncryptionOfSecretKeyShareRoundParty as Party>::PublicOutput =
                bcs::from_bytes(&decentralized_first_round_public_output)
                    .context("failed to deserialize decentralized first round DKG output")?;
            let public_parameters = bcs::from_bytes(&protocol_public_parameters_by_key_scheme(
                network_dkg_public_output,
                key_scheme,
            )?)?;

            let session_id = commitment::CommitmentSizedNumber::from_le_hex(&session_id);

            let round_result = DKGCentralizedParty::advance(
                decentralized_first_round_public_output,
                &(),
                &(public_parameters, session_id).into(),
                &mut OsRng,
            )
            .context("advance() failed on the DKGCentralizedParty")?;

            // Centralized Public Key Share and Proof.
            let public_key_share_and_proof =
                VersionedPublicKeyShareAndProof::V1(bcs::to_bytes(&round_result.outgoing_message)?);

            let public_key_share_and_proof = bcs::to_bytes(&public_key_share_and_proof)?;

            // Public Output:
            // centralized_public_key_share + public_key + decentralized_party_public_key_share
            let public_output = bcs::to_bytes(&VersionedCentralizedDKGPublicOutput::V1(
                bcs::to_bytes(&round_result.public_output)?,
            ))?;
            // Centralized Secret Key Share.
            // Warning:
            // The secret (private)
            // key share returned from this function should never be sent
            // and should always be kept private.
            let centralized_secret_output =
                VersionedDwalletUserSecretShare::V1(bcs::to_bytes(&round_result.private_output)?);
            let centralized_secret_output = bcs::to_bytes(&centralized_secret_output)?;
            Ok(CentralizedDKGWasmResult {
                public_output,
                public_key_share_and_proof,
                centralized_secret_output,
            })
        }
    }
}

/// Executes the centralized phase of the Sign protocol,
///  the first part of the protocol.
///
/// The [`advance_centralized_sign_party`] function is
/// called by the client (the centralized party).
pub fn advance_centralized_sign_party(
    network_dkg_public_output: SerializedWrappedMPCPublicOutput,
    key_scheme: u32,
    decentralized_party_dkg_public_output: SerializedWrappedMPCPublicOutput,
    centralized_party_secret_key_share: SerializedWrappedMPCPublicOutput,
    presign: SerializedWrappedMPCPublicOutput,
    message: Vec<u8>,
    hash_type: u32,
) -> anyhow::Result<SignedMessage> {
    let decentralized_party_dkg_public_output =
        bcs::from_bytes(&decentralized_party_dkg_public_output)?;
    match decentralized_party_dkg_public_output {
        VersionedDwalletDKGSecondRoundPublicOutput::V1(decentralized_party_dkg_public_output) => {
            let presign = bcs::from_bytes(&presign)?;
            let VersionedPresignOutput::V1(presign) = presign;
            let centralized_party_secret_key_share: VersionedDwalletUserSecretShare =
                bcs::from_bytes(&centralized_party_secret_key_share)?;
            let VersionedDwalletUserSecretShare::V1(centralized_party_secret_key_share) =
                centralized_party_secret_key_share;
            let decentralized_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput = bcs::from_bytes(&decentralized_party_dkg_public_output)?;
            let centralized_public_output = twopc_mpc::class_groups::DKGCentralizedPartyOutput::<
                { secp256k1::SCALAR_LIMBS },
                secp256k1::GroupElement,
            > {
                public_key_share: decentralized_output.centralized_party_public_key_share,
                public_key: decentralized_output.public_key,
                decentralized_party_public_key_share: decentralized_output.public_key_share,
            };
            let presign: <AsyncProtocol as twopc_mpc::presign::Protocol>::Presign =
                bcs::from_bytes(&presign)?;
            let hashed_message = message_digest(&message, &hash_type.try_into()?)
                .context("Message digest failed")?;
            let centralized_party_public_input =
                <AsyncProtocol as twopc_mpc::sign::Protocol>::SignCentralizedPartyPublicInput::from(
                    (
                        hashed_message,
                        centralized_public_output.clone(),
                        presign,
                        bcs::from_bytes(&protocol_public_parameters_by_key_scheme(
                            network_dkg_public_output.clone(),
                            key_scheme,
                        )?)?,
                    ),
                );

            let round_result = SignCentralizedParty::advance(
                (),
                &bcs::from_bytes(&centralized_party_secret_key_share)?,
                &centralized_party_public_input,
                &mut OsRng,
            )
            .context("advance() failed on the SignCentralizedParty")?;

            let signed_message =
                VersionedUserSignedMessage::V1(bcs::to_bytes(&round_result.outgoing_message)?);
            let signed_message = bcs::to_bytes(&signed_message)?;
            Ok(signed_message)
        }
    }
}

pub fn sample_dwallet_keypair_inner(
    network_dkg_public_output: SerializedWrappedMPCPublicOutput,
) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let protocol_public_parameters: ProtocolPublicParameters =
        bcs::from_bytes(&protocol_public_parameters_by_key_scheme(
            network_dkg_public_output,
            DWalletMPCNetworkKeyScheme::Secp256k1 as u32,
        )?)?;
    let secret_key = twopc_mpc::secp256k1::Scalar::sample(
        &protocol_public_parameters
            .as_ref()
            .scalar_group_public_parameters,
        &mut OsRng,
    )?;
    let public_parameters = group::secp256k1::group_element::PublicParameters::default();
    let generator_group_element =
        group::secp256k1::group_element::GroupElement::generator_from_public_parameters(
            &public_parameters,
        )?;

    let expected_public_key = secret_key * generator_group_element;
    let bytes_public_key = bcs::to_bytes(&expected_public_key.value())?;
    Ok((bcs::to_bytes(&secret_key)?, bytes_public_key))
}

pub fn verify_secp_signature_inner(
    public_key: Vec<u8>,
    signature: Vec<u8>,
    message: Vec<u8>,
    network_dkg_public_output: SerializedWrappedMPCPublicOutput,
    hash_type: u32,
) -> anyhow::Result<bool> {
    let VersionedSignOutput::V1(signature) = bcs::from_bytes(&signature)?;
    let protocol_public_parameters: ProtocolPublicParameters =
        bcs::from_bytes(&protocol_public_parameters_by_key_scheme(
            network_dkg_public_output,
            DWalletMPCNetworkKeyScheme::Secp256k1 as u32,
        )?)?;
    let public_key = twopc_mpc::secp256k1::GroupElement::new(
        bcs::from_bytes(&public_key)?,
        &protocol_public_parameters.group_public_parameters,
    )?;
    let hashed_message =
        message_digest(&message, &hash_type.try_into()?).context("Message digest failed")?;
    let (r, s): (secp256k1::Scalar, secp256k1::Scalar) = bcs::from_bytes(&signature)?;
    Ok(verify_signature(r, s, hashed_message, public_key).is_ok())
}

pub fn create_imported_dwallet_centralized_step_inner(
    network_dkg_public_output: SerializedWrappedMPCPublicOutput,
    dwallet_id: String,
    secret_key: Vec<u8>,
) -> anyhow::Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let protocol_public_parameters: ProtocolPublicParameters =
        bcs::from_bytes(&protocol_public_parameters_by_key_scheme(
            network_dkg_public_output,
            DWalletMPCNetworkKeyScheme::Secp256k1 as u32,
        )?)?;
    let secret_key = bcs::from_bytes(&secret_key)?;
    let dwallet_id = commitment::CommitmentSizedNumber::from_le_hex(&dwallet_id);
    let centralized_party_public_input = (protocol_public_parameters.clone(), dwallet_id).into();

    match ImportSecretKeyFirstStep::advance(
        (),
        &secret_key,
        &centralized_party_public_input,
        &mut OsRng,
    ) {
        Ok(round_result) => {
            let public_output = round_result.public_output;
            let outgoing_message = round_result.outgoing_message;
            let secret_share = round_result.private_output;
            Ok((
                bcs::to_bytes(&VersionedDwalletUserSecretShare::V1(bcs::to_bytes(
                    &secret_share,
                )?))?,
                bcs::to_bytes(&VersionedImportedDWalletPublicOutput::V1(bcs::to_bytes(
                    &public_output,
                )?))?,
                bcs::to_bytes(&VersionedImportedDwalletOutgoingMessage::V1(bcs::to_bytes(
                    &outgoing_message,
                )?))?,
            ))
        }
        Err(e) => Err(e.into()),
    }
}

fn protocol_public_parameters_by_key_scheme(
    network_dkg_public_output: SerializedWrappedMPCPublicOutput,
    key_scheme: u32,
) -> anyhow::Result<Vec<u8>> {
    let mpc_public_output: VersionedNetworkDkgOutput = bcs::from_bytes(&network_dkg_public_output)?;

    match &mpc_public_output {
        VersionedNetworkDkgOutput::V1(network_dkg_public_output) => {
            let key_scheme = DWalletMPCNetworkKeyScheme::try_from(key_scheme)?;
            match key_scheme {
                DWalletMPCNetworkKeyScheme::Secp256k1 => {
                    let network_dkg_public_output: <Secp256k1Party as mpc::Party>::PublicOutput =
                        bcs::from_bytes(network_dkg_public_output)?;
                    let encryption_scheme_public_parameters = network_dkg_public_output
                        .default_encryption_scheme_public_parameters::<secp256k1::GroupElement>(
                    )?;
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
    }
}

/// Derives a Secp256k1 class groups keypair from a given seed.
///
/// The class groups public encryption key being used to encrypt a Secp256k1 keypair will be
/// different from the encryption key used to encrypt a Ristretto keypair.
/// The plaintext space/fundamental group will correspond to the order
/// of the respective elliptic curve.
/// The secret decryption key may be the same in terms of correctness,
/// but to simplify security analysis,
/// and the implementation current version maintains distinct key-pairs.
/// # Warning
/// The secret (private) key returned from this function should never be sent
/// and should always be kept private.
pub fn generate_secp256k1_cg_keypair_from_seed_internal(
    seed: [u8; 32],
) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed);
    let setup_parameters = get_setup_parameters_secp256k1();
    let (encryption_scheme_public_parameters, decryption_key) =
        Secp256k1DecryptionKey::generate(setup_parameters, &mut rng)?;
    let decryption_key = bcs::to_bytes(&decryption_key.decryption_key)?;
    let encryption_key = bcs::to_bytes(&encryption_scheme_public_parameters.encryption_key)?;
    Ok((encryption_key, decryption_key))
}

/// Encrypts the given secret key share with the given encryption key.
/// Returns a serialized tuple containing the `proof of encryption`,
/// and an encrypted `secret key share`.
pub fn encrypt_secret_key_share_and_prove(
    secret_key_share: SerializedWrappedMPCPublicOutput,
    encryption_key: Vec<u8>,
    network_dkg_public_output: SerializedWrappedMPCPublicOutput,
) -> anyhow::Result<Vec<u8>> {
    let protocol_public_params: ProtocolPublicParameters =
        bcs::from_bytes(&protocol_public_parameters_by_key_scheme(
            network_dkg_public_output,
            DWalletMPCNetworkKeyScheme::Secp256k1 as u32,
        )?)?;
    let secret_key_share: VersionedDwalletUserSecretShare = bcs::from_bytes(&secret_key_share)?;
    match secret_key_share {
        VersionedDwalletUserSecretShare::V1(secret_key_share) => {
            let encryption_key = bcs::from_bytes(&encryption_key)?;
            let secret_key_share = bcs::from_bytes(&secret_key_share)?;
            let result = <AsyncProtocol as twopc_mpc::dkg::Protocol>::encrypt_and_prove_centralized_party_share(&protocol_public_params, encryption_key, secret_key_share, &mut OsRng)?;
            Ok(bcs::to_bytes(&VersionedEncryptedUserShare::V1(
                bcs::to_bytes(&result)?,
            ))?)
        }
    }
}

/// Verifies the given secret share matches the given dWallets`
/// DKG output centralized_party_public_key_share.
pub fn verify_secret_share(
    secret_share: Vec<u8>,
    dkg_output: SerializedWrappedMPCPublicOutput,
    network_dkg_public_output: Vec<u8>,
) -> anyhow::Result<bool> {
    let protocol_public_params: ProtocolPublicParameters =
        bcs::from_bytes(&protocol_public_parameters_by_key_scheme(
            network_dkg_public_output,
            DWalletMPCNetworkKeyScheme::Secp256k1 as u32,
        )?)?;
    let dkg_output = bcs::from_bytes(&dkg_output)?;
    match dkg_output {
        VersionedDwalletDKGSecondRoundPublicOutput::V1(dkg_output) => {
            let dkg_output = bcs::from_bytes(&dkg_output)?;
            let secret_share = bcs::from_bytes(&secret_share)?;
            Ok(<twopc_mpc::secp256k1::class_groups::AsyncProtocol as twopc_mpc::dkg::Protocol>::verify_centralized_party_secret_key_share(
                &protocol_public_params,
                dkg_output,
                secret_share,
            )
                .is_ok())
        }
    }
}

/// Decrypts the given encrypted user share using the given decryption key.
pub fn decrypt_user_share_inner(
    decryption_key: Vec<u8>,
    encryption_key: Vec<u8>,
    dwallet_dkg_output: Vec<u8>,
    encrypted_user_share_and_proof: Vec<u8>,
    network_dkg_public_output: Vec<u8>,
) -> anyhow::Result<Vec<u8>> {
    let protocol_public_params: ProtocolPublicParameters =
        bcs::from_bytes(&protocol_public_parameters_by_key_scheme(
            network_dkg_public_output,
            DWalletMPCNetworkKeyScheme::Secp256k1 as u32,
        )?)?;
    let VersionedEncryptedUserShare::V1(encrypted_user_share_and_proof) =
        bcs::from_bytes(&encrypted_user_share_and_proof)?;
    let VersionedDwalletDKGSecondRoundPublicOutput::V1(dwallet_dkg_output) =
        bcs::from_bytes(&dwallet_dkg_output)?;
    let (_, encryption_of_discrete_log): <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptedSecretKeyShareMessage = bcs::from_bytes(&encrypted_user_share_and_proof)?;
    <twopc_mpc::secp256k1::class_groups::AsyncProtocol as Protocol>::verify_encryption_of_centralized_party_share_proof(
        &protocol_public_params,
        bcs::from_bytes(&dwallet_dkg_output)?,
        bcs::from_bytes(&encryption_key)?,
        bcs::from_bytes(&encrypted_user_share_and_proof)?,
        &mut OsRng,
    )
        .map_err(Into::<anyhow::Error>::into)?;
    let decryption_key = bcs::from_bytes(&decryption_key)?;
    let public_parameters = homomorphic_encryption::PublicParameters::<
        SCALAR_LIMBS,
        crate::Secp256k1EncryptionKey,
    >::new_from_secret_key(
        protocol_public_params
            .encryption_scheme_public_parameters
            .setup_parameters
            .clone(),
        decryption_key,
    )?;
    let ciphertext = CiphertextSpaceGroupElement::new(
        encryption_of_discrete_log,
        public_parameters.ciphertext_space_public_parameters(),
    )?;

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
    let secret_share_bytes = bcs::to_bytes(&plaintext.value())?;
    Ok(secret_share_bytes)
}
