//! This crate contains the cryptographic logic for the centralized 2PC-MPC party.

use anyhow::Context;
use k256::ecdsa::hazmat::bits2field;
use k256::ecdsa::signature::digest::{Digest, FixedOutput};
use k256::elliptic_curve::ops::Reduce;
use k256::{elliptic_curve, U256};
use mpc::two_party::Round;
use rand_core::OsRng;
use std::fmt;
use twopc_mpc::secp256k1;

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
    decentralized_first_round_output: Vec<u8>,
    session_id: String,
) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let decentralized_first_round_output: EncryptionOfSecretKeyShareAndPublicKeyShare =
        bcs::from_bytes(&decentralized_first_round_output)
            .context("Failed to deserialize decentralized first round output")?;
    let public_parameters = class_groups_constants::protocol_public_parameters();
    let session_id = commitment::CommitmentSizedNumber::from_le_hex(&session_id);

    let (public_key_share_and_proof, centralized_output) = DKGCentralizedParty::advance(
        decentralized_first_round_output,
        &(public_parameters, session_id).into(),
        &mut OsRng,
    )
    .context("advance() failed on the DKGCentralizedParty")?;

    let public_key_share_and_proof = bcs::to_bytes(&public_key_share_and_proof)?;
    let centralized_output = bcs::to_bytes(&centralized_output)?;

    Ok((public_key_share_and_proof, centralized_output))
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
    centralized_party_dkg_output: Vec<u8>,
    presigns: Vec<Vec<u8>>,
    messages: Vec<Vec<u8>>,
    hash: u8,
    session_ids: Vec<String>,
) -> anyhow::Result<(Vec<HashedMessages>, Vec<SignedMessages>)> {
    let protocol_public_parameters = class_groups_constants::protocol_public_parameters();
    let centralized_party_dkg_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::CentralizedPartyDKGOutput =
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
            // Prepare auxiliary input.
            let centralized_party_auxiliary_input = (
                hashed_message,
                centralized_party_dkg_output.clone(),
                presign,
                protocol_public_parameters.clone(),
                session_id,
            )
                .into();

            let (sign_message, _) =
                SignCentralizedParty::advance((), &centralized_party_auxiliary_input, &mut OsRng)
                    .context("advance() failed on the SignCentralizedParty")?;

            let signed_message = bcs::to_bytes(&sign_message)?;
            let hashed_message_bytes = bcs::to_bytes(&hashed_message)?;
            Ok((signed_message, hashed_message_bytes))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .unzip();

    Ok((signed_messages, hashed_messages))
}
