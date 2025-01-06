use class_groups::{
    CiphertextSpaceGroupElement, CiphertextSpaceValue, SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};
use class_groups_constants::protocol_public_parameters;
use group::GroupElement;
use homomorphic_encryption::GroupsPublicParametersAccessors;
use std::marker::PhantomData;
use twopc_mpc::languages::class_groups::{
    construct_encryption_of_discrete_log_public_parameters, EncryptionOfDiscreteLogProofWithoutCtx,
};
use twopc_mpc::secp256k1;
use twopc_mpc::secp256k1::class_groups::AsyncProtocol;
use twopc_mpc::secp256k1::SCALAR_LIMBS;

type SecretShareEncryptionProof = EncryptionOfDiscreteLogProofWithoutCtx<
    SCALAR_LIMBS,
    SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    secp256k1::GroupElement,
>;

/// Verifies that the given secret encryption is the encryption of the given dwallet's secret share.
pub fn chain_verify_secret_share_proof(
    encrypted_share_and_proof: Vec<u8>,
    dkg_output: Vec<u8>,
    encryption_key: Vec<u8>,
) -> anyhow::Result<()> {
    let protocol_public_params = protocol_public_parameters();
    let language_public_parameters = construct_encryption_of_discrete_log_public_parameters::<
        SCALAR_LIMBS,
        { SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS },
        { SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS },
        secp256k1::GroupElement,
    >(
        protocol_public_params
            .scalar_group_public_parameters
            .clone(),
        protocol_public_params.group_public_parameters.clone(),
        bcs::from_bytes(&encryption_key)?,
    );
    let dkg_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput =
        bcs::from_bytes(&dkg_output)?;
    let (proof, encrypted_secret_share): (
        SecretShareEncryptionProof,
        CiphertextSpaceValue<SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS>,
    ) = bcs::from_bytes(&encrypted_share_and_proof)?;
    let encrypted_secret_share = CiphertextSpaceGroupElement::new(
        encrypted_secret_share,
        &language_public_parameters
            .encryption_scheme_public_parameters
            .ciphertext_space_public_parameters(),
    )?;
    let public_key_share = secp256k1::GroupElement::new(
        dkg_output.centralized_party_public_key_share,
        &protocol_public_params.group_public_parameters,
    )?;
    let statement = (encrypted_secret_share, public_key_share).into();

    proof
        .verify(&PhantomData, &language_public_parameters, vec![statement])
        .map_err(Into::into)
}
