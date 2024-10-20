use mpc::two_party::Round;
use rand_core::OsRng;
use std::marker::PhantomData;

type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol<PhantomData<()>>;
type DKGCentralizedParty = <AsyncProtocol as twopc_mpc::dkg::Protocol>::DKGCentralizedParty;

pub fn create_centralized_output(first_round_output: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let first_round_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareAndPublicKeyShare = bcs::from_bytes(&first_round_output)?;

    let (pp, _) = twopc_mpc::tests::setup_class_groups_secp256k1();
    let auxiliary_input = (pp, PhantomData).into();

    let (outgoing_message, _) =
        DKGCentralizedParty::advance(first_round_output, &auxiliary_input, &mut OsRng)?;
    let outgoing_message = bcs::to_bytes(&outgoing_message)?;
    Ok(outgoing_message)
}
