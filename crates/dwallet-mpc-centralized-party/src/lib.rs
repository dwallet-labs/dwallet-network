use k256::ecdsa::hazmat::bits2field;
use k256::ecdsa::signature::digest::{Digest, FixedOutput};
use k256::{elliptic_curve, U256};
use k256::elliptic_curve::ops::Reduce;
use log::debug;
/// This crate contains the cryptographic logic for the centralized 2PC-MPC party
use mpc::two_party::Round;
use rand_core::OsRng;
use twopc_mpc::class_groups::DKGCentralizedPartyOutput;
use twopc_mpc::secp256k1;
use twopc_mpc::secp256k1::GroupElement;

type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;
type DKGCentralizedParty = <AsyncProtocol as twopc_mpc::dkg::Protocol>::DKGCentralizedParty;

/// Executes the second phase of the DKG protocol, part of a three-phase DKG flow.
///
/// The [`create_dkg_output`] function is called by the client (aka the centralized party)
/// and is responsible for generating and returning the public key share and its proof, as well as the
/// centralized DKG output. These values are necessary for the decentralized party to complete the final
/// phase of the DKG protocol.
///
/// # Arguments
/// * `decentralized_first_round_output` - A serialized byte vector representing the output of the
///   decentralized party from the first round.
/// * `session_id` - A unique identifier for the session, represented as a hexadecimal string.
///   Received from the `pera_system::dwallet_2pc_mpc_ecdsa_k1::launch_dkg_first_round` transaction.
pub fn create_dkg_output(
    decentralized_first_round_output: Vec<u8>,
    session_id: String,
) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let decentralized_first_round_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareAndPublicKeyShare = bcs::from_bytes(&decentralized_first_round_output)?;
    let public_parameters = class_groups_constants::protocol_public_parameters();
    let session_id = commitment::CommitmentSizedNumber::from_le_hex(&session_id);
    let (public_key_share_and_proof, centralized_output) = DKGCentralizedParty::advance(
        decentralized_first_round_output,
        &(public_parameters, session_id).into(),
        &mut OsRng,
    )?;
    let public_key_share_and_proof = bcs::to_bytes(&public_key_share_and_proof)?;
    let centralized_output = bcs::to_bytes(&centralized_output)?;
    Ok((public_key_share_and_proof, centralized_output))
}

#[derive(Clone, Debug)]
pub enum Hash {
    KECCAK256 = 0,
    SHA256 = 1,
}

impl TryFrom<u8> for Hash {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Hash::KECCAK256),
            1 => Ok(Hash::SHA256),
            _ => Err(anyhow::Error::msg("Invalid value for Hash enum")),
        }
    }
}

pub fn message_digest(message: &[u8], hash_type: &Hash) -> secp256k1::Scalar {
    let hash = match hash_type {
        Hash::KECCAK256 => bits2field::<k256::Secp256k1>(
            &sha3::Keccak256::new_with_prefix(message).finalize_fixed(),
        ),
        Hash::SHA256 => {
            bits2field::<k256::Secp256k1>(&sha2::Sha256::new_with_prefix(message).finalize_fixed())
        }
    }
        .unwrap();
    #[allow(clippy::useless_conversion)]
    let m = <elliptic_curve::Scalar<k256::Secp256k1> as Reduce<U256>>::reduce_bytes(&hash.into());
    U256::from(m).into()
}

type SignCentralizedParty = <AsyncProtocol as twopc_mpc::sign::Protocol>::SignCentralizedParty;
pub fn create_sign_output(
    centralized_party_dkg_output: Vec<u8>,
    presign: Vec<u8>,
    message: Vec<u8>,
    hash: u8,
    session_id: String,
) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let centralized_party_dkg_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::CentralizedPartyDKGOutput = bcs::from_bytes(&centralized_party_dkg_output)?;
    let presign: <AsyncProtocol as twopc_mpc::presign::Protocol>::Presign = bcs::from_bytes(&presign)?;
    let session_id = commitment::CommitmentSizedNumber::from_le_hex(&session_id);
    let m = message_digest(&message, &hash.try_into()?);
    let protocol_public_parameters = class_groups_constants::protocol_public_parameters();

    let centralized_party_auxiliary_input =
        (
            m,
            centralized_party_dkg_output.clone(),
            presign.clone(),
            protocol_public_parameters.clone(),
            session_id
        ).into();
    let (sign_message, centralized_output) =
        SignCentralizedParty::advance((), &centralized_party_auxiliary_input, &mut OsRng)?;
    let sign_message = bcs::to_bytes(&sign_message)?;
    let centralized_output = bcs::to_bytes(&centralized_output)?;
    Ok((sign_message, centralized_output))
}