use dwallet_mpc_types::dwallet_mpc::{
    MPCPublicOutput, MPCPublicOutputClassGroups, SerializedWrappedMPCPublicOutput,
};
use fastcrypto::traits::ToFromBytes;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::EncryptedShareVerificationRequestEvent;
use rand_core::OsRng;
use twopc_mpc::dkg::Protocol;
use twopc_mpc::secp256k1::class_groups::AsyncProtocol;

/// Verifies that the given encrypted secret key share matches the encryption of the dWallet's
/// secret share, validates the signature on the dWallet's public share,
/// and ensures the signing public key matches the address that initiated this transaction.
pub(crate) fn verify_encrypted_share(
    verification_data: &EncryptedShareVerificationRequestEvent,
    protocol_public_parameters: &Vec<u8>,
) -> DwalletMPCResult<()> {
    verify_centralized_secret_key_share_proof(
        &verification_data.encrypted_centralized_secret_share_and_proof,
        &verification_data.decentralized_public_output,
        &verification_data.encryption_key,
        protocol_public_parameters,
    )
    .map_err(|_| DwalletMPCError::EncryptedUserShareVerificationFailed)
}

/// Verifies that the given centralized secret key share
/// encryption is the encryption of the given dWallet's secret share.
fn verify_centralized_secret_key_share_proof(
    encrypted_centralized_secret_share_and_proof: &Vec<u8>,
    serialized_dkg_public_output: &SerializedWrappedMPCPublicOutput,
    encryption_key: &Vec<u8>,
    protocol_public_parameters: &Vec<u8>,
) -> anyhow::Result<()> {
    let dkg_public_output = bcs::from_bytes(serialized_dkg_public_output)?;
    match dkg_public_output {
        MPCPublicOutputClassGroups::V1(dkg_public_output) => {
            <AsyncProtocol as twopc_mpc::dkg::Protocol>::verify_encryption_of_centralized_party_share_proof(
                &bcs::from_bytes(&protocol_public_parameters)?, bcs::from_bytes(&dkg_public_output)?, bcs::from_bytes(&encryption_key)?, bcs::from_bytes(&encrypted_centralized_secret_share_and_proof)?, &mut OsRng).map_err(Into::<anyhow::Error>::into)?;
            Ok(())
        }
    }
}
