use dwallet_mpc_types::dwallet_mpc::{
    MPCPublicOutput, MPCPublicOutputClassGroups, SerializedWrappedMPCPublicOutput,
};
use group::{CyclicGroupElement, GroupElement};
use twopc_mpc::secp256k1::class_groups::AsyncProtocol;
use twopc_mpc::secp256k1::SCALAR_LIMBS;

/// Verifies the given secret share matches the given dWallets`
/// DKG output centralized_party_public_key_share.
pub fn verify_secret_share(
    secret_share: Vec<u8>,
    dkg_output: SerializedWrappedMPCPublicOutput,
) -> anyhow::Result<bool> {
    let dkg_output = bcs::from_bytes(&dkg_output)?;
    match dkg_output {
        MPCPublicOutput::ClassGroups(MPCPublicOutputClassGroups::V1(dkg_output)) => {
            let expected_public_key =
                cg_secp256k1_public_key_share_from_secret_share(secret_share)?;
            let dkg_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput =
                bcs::from_bytes(&dkg_output)?;
            Ok(dkg_output.centralized_party_public_key_share == expected_public_key.value())
        }
    }
}
/// Derives a dWallets` public key share from a private key share.
fn cg_secp256k1_public_key_share_from_secret_share(
    secret_key_share: Vec<u8>,
) -> anyhow::Result<group::secp256k1::GroupElement> {
    let public_parameters = group::secp256k1::group_element::PublicParameters::default();
    let generator_group_element =
        group::secp256k1::group_element::GroupElement::generator_from_public_parameters(
            &public_parameters,
        )?;
    Ok(
        generator_group_element.scale(&crypto_bigint::Uint::<{ SCALAR_LIMBS }>::from_be_slice(
            &secret_key_share,
        )),
    )
}
