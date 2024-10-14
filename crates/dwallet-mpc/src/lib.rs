use mpc::two_party::Round;
use pera_core::signature_mpc::dkg::{setup_paillier_secp256k1, AsyncProtocol};
use rand_core::{CryptoRngCore, OsRng};
use std::marker::PhantomData;
use twopc_mpc::dkg::decentralized_party::encryption_of_secret_key_share_round::AuxiliaryInput;

type DKGCentralizedParty = <AsyncProtocol as twopc_mpc::dkg::Protocol>::DKGCentralizedParty;
pub fn create_centralized_output(first_round_output: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let first_round_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareAndPublicKeyShare = bcs::from_bytes(&first_round_output)?;

    let (secp256k1_group_public_parameters, _) = setup_paillier_secp256k1();

    let auxiliary_input = (secp256k1_group_public_parameters, PhantomData).into();

    let (outgoing_message, _) =
        DKGCentralizedParty::advance(first_round_output, &auxiliary_input, &mut OsRng)?;
    // let output = bcs::to_bytes(&output)?;
    let outgoing_message = bcs::to_bytes(&outgoing_message)?;
    Ok(outgoing_message)
}
