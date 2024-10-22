use mpc::two_party::Round;
use rand_core::OsRng;
use log::log;

type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol<>;
type DKGCentralizedParty = <AsyncProtocol as twopc_mpc::dkg::Protocol>::DKGCentralizedParty;

pub fn create_centralized_output(first_round_output: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    log::debug!("first_round_output: {:?}", first_round_output);
    let first_round_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareAndPublicKeyShare = bcs::from_bytes(&first_round_output)?;
    log::debug!("first_round_output: {:?}", first_round_output);
    let pp = class_groups_constants::protocol_public_parameters()?;
    log::debug!("pp: {:?}", pp);
    let protocol_context = commitment::CommitmentSizedNumber::from_u8(8);
    log::debug!("protocol_context: {:?}", protocol_context);
    let auxiliary_input = (pp, protocol_context).into();
    log::debug!("auxiliary_input: {:?}", auxiliary_input);
    let (outgoing_message, _) =
        DKGCentralizedParty::advance(first_round_output, &auxiliary_input, &mut OsRng)?;
    log::debug!("outgoing_message: {:?}", outgoing_message);
    let outgoing_message = bcs::to_bytes(&outgoing_message)?;
    log::debug!("outgoing_message: {:?}", outgoing_message);
    Ok(outgoing_message)
}
