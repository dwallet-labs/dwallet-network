use anyhow::anyhow;
use class_groups::Secp256k1EncryptionKey;
use dwallet_mpc_types::dwallet_mpc::{
    MPCPublicOutput, MPCPublicOutputClassGroups, SerializedWrappedMPCPublicOutput,
};
use group::{CyclicGroupElement, GroupElement};
use homomorphic_encryption::{AdditivelyHomomorphicEncryptionKey, PlaintextSpaceGroupElement};
use twopc_mpc::secp256k1::class_groups::AsyncProtocol;
use twopc_mpc::secp256k1::SCALAR_LIMBS;

/// Verifies the given secret share matches the given dWallets`
/// DKG output centralized_party_public_key_share.
pub fn verify_secret_share(
    protocol_public_parameters: &Vec<u8>,
    secret_share: Vec<u8>,
    dkg_output: SerializedWrappedMPCPublicOutput,
) -> anyhow::Result<()> {
    let secret_share: MPCPublicOutputClassGroups = bcs::from_bytes(&secret_share)?;
    let secret_share = match secret_share {
        MPCPublicOutputClassGroups::V1(output) => output,
        _ => {
            return Err(anyhow!(
                "invalid centralized public output version: expected ClassGroups::V1, got {:?}",
                secret_share
            ));
        }
    };
    let dkg_output = bcs::from_bytes(&dkg_output)?;
    match dkg_output {
        MPCPublicOutput::ClassGroups(MPCPublicOutputClassGroups::V1(dkg_output)) => {
            <AsyncProtocol as twopc_mpc::dkg::Protocol>::verify_centralized_party_secret_key_share(
                &bcs::from_bytes(protocol_public_parameters)?,
                bcs::from_bytes(&dkg_output)?,
                bcs::from_bytes(&secret_share)?,
            )
            .map_err(Into::into)
        }
    }
}
