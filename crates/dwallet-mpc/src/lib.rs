use group::secp256k1;
use mpc::two_party::Round;
use rand_core::OsRng;
use std::marker::PhantomData;
use tiresias::test_helpers::N;
use twopc_mpc::secp256k1::paillier::bulletproofs::PaillierProtocolPublicParameters;

type AsyncProtocol = twopc_mpc::secp256k1::paillier::bulletproofs::AsyncProtocol<PhantomData<()>>;
type DKGCentralizedParty = <AsyncProtocol as twopc_mpc::dkg::Protocol>::DKGCentralizedParty;

pub fn create_centralized_output(first_round_output: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let first_round_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareAndPublicKeyShare = bcs::from_bytes(&first_round_output)?;

    let secp256k1_group_public_parameters =
        PaillierProtocolPublicParameters::new::<secp256k1::GroupElement>(N);
    let auxiliary_input = (secp256k1_group_public_parameters, PhantomData).into();

    let (outgoing_message, _) =
        DKGCentralizedParty::advance(first_round_output, &auxiliary_input, &mut OsRng)?;
    let outgoing_message = bcs::to_bytes(&outgoing_message)?;
    Ok(outgoing_message)
}
