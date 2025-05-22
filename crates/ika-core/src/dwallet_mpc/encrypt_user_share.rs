use class_groups::{
    CiphertextSpaceGroupElement, CiphertextSpaceValue, SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};
use dwallet_mpc_types::dwallet_mpc::{
    MPCPublicOutput, MPCPublicOutputClassGroups, SerializedWrappedMPCPublicOutput,
};
use group::GroupElement;
use homomorphic_encryption::GroupsPublicParametersAccessors;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::StartEncryptedShareVerificationEvent;
use std::marker::PhantomData;
use twopc_mpc::languages::class_groups::construct_encryption_of_discrete_log_public_parameters;
use twopc_mpc::secp256k1;
use twopc_mpc::secp256k1::class_groups::{
    AsyncProtocol, EncryptionOfSecretShareProof, ProtocolPublicParameters,
};
use twopc_mpc::secp256k1::SCALAR_LIMBS;

/// Verifies that the given encrypted secret key share matches the encryption of the dWallet's
/// secret share, validates the signature on the dWallet's public share,
/// and ensures the signing public key matches the address that initiated this transaction.
pub(crate) fn verify_encrypted_share(
    verification_data: &StartEncryptedShareVerificationEvent,
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
        MPCPublicOutput::ClassGroups(MPCPublicOutputClassGroups::V1(dkg_public_output)) => {
            let decentralized_public_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput =
                bcs::from_bytes(&dkg_public_output)?;
            let protocol_public_params: ProtocolPublicParameters =
                bcs::from_bytes(protocol_public_parameters)?;
            let language_public_parameters = construct_encryption_of_discrete_log_public_parameters::<
                SCALAR_LIMBS,
                { SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS },
                { SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS },
                secp256k1::GroupElement,
            >(
                protocol_public_params.scalar_group_public_parameters,
                protocol_public_params.group_public_parameters.clone(),
                bcs::from_bytes(encryption_key)?,
            );
            let (proof, encrypted_centralized_secret_key_share): (
                EncryptionOfSecretShareProof,
                CiphertextSpaceValue<SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS>,
            ) = bcs::from_bytes(encrypted_centralized_secret_share_and_proof)?;
            let encrypted_centralized_secret_key_share_for_statement =
                CiphertextSpaceGroupElement::new(
                    encrypted_centralized_secret_key_share,
                    &language_public_parameters
                        .encryption_scheme_public_parameters
                        .ciphertext_space_public_parameters(),
                )?;
            let centralized_public_key_share = secp256k1::GroupElement::new(
                decentralized_public_output.centralized_party_public_key_share,
                &protocol_public_params.group_public_parameters,
            )?;
            let statement = (
                encrypted_centralized_secret_key_share_for_statement,
                centralized_public_key_share,
            )
                .into();

            proof
                .verify(&PhantomData, &language_public_parameters, vec![statement])
                .map_err(Into::into)
        }
    }
}
