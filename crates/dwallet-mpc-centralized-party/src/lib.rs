/// This crate contains the cryptographic logic for the centralized 2PC-MPC party
use mpc::two_party::Round;
use rand_core::OsRng;

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
///   Received from the `pera_system::dwallet_2pc_mpc_ecdsa_k1::start_first_dkg_session` transaction.
pub fn create_dkg_output(
    decentralized_first_round_output: Vec<u8>,
    session_id: String,
) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let decentralized_first_round_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareAndPublicKeyShare = bcs::from_bytes(&decentralized_first_round_output)?;
    let public_parameters = class_groups_constants::protocol_public_parameters()?;
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