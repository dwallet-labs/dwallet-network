use mpc::two_party::Round;
use rand_core::OsRng;
use log::log;
use mpc::two_party;

type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;
type DKGCentralizedParty = <AsyncProtocol as twopc_mpc::dkg::Protocol>::DKGCentralizedParty;

pub fn create_centralized_output(first_round_output: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let first_round_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareAndPublicKeyShare = bcs::from_bytes(&first_round_output)?;
    let pp = class_groups_constants::protocol_public_parameters()?;
    let session_id = commitment::CommitmentSizedNumber::from_u8(8);
    let auxiliary_input = (pp, session_id).into();
    let (outgoing_message, a) =
        DKGCentralizedParty::advance(first_round_output, &auxiliary_input, &mut OsRng)?;
    let outgoing_message = bcs::to_bytes(&outgoing_message)?;
    let a = hex::encode(&outgoing_message);
    Ok(outgoing_message)
}
