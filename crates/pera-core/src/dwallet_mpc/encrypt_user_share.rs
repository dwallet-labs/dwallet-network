use class_groups::{
    CiphertextSpaceGroupElement, CiphertextSpaceValue, SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};
use class_groups_constants::protocol_public_parameters;
use fastcrypto::ed25519::{Ed25519PublicKey, Ed25519Signature};
use fastcrypto::traits::{ToFromBytes, VerifyingKey};
use group::GroupElement;
use homomorphic_encryption::GroupsPublicParametersAccessors;
use pera_types::base_types::PeraAddress;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::messages_dwallet_mpc::StartEncryptedShareVerificationEvent;
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

/// Verifies that the given encrypted secret share matches the encryption of the dWallet's
/// secret share, validates the signature on the dWallet's public share,
/// and ensures the signing public key matches the address that initiated this transaction.
pub(crate) fn verify_encrypted_share(
    verification_data: StartEncryptedShareVerificationEvent,
) -> DwalletMPCResult<()> {
    verify_signatures(&verification_data)?;
    match chain_verify_secret_share_proof(
        verification_data.encrypted_secret_share_and_proof.clone(),
        verification_data.dwallet_output.clone(),
        verification_data.encryption_key.clone(),
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err(DwalletMPCError::EncryptedUserShareVerificationFailed),
    }
}

/// Verify the signature on the public share of the DWallet,
/// and that the public key that signed the public user share is matching the address that initiated this TX.
fn verify_signatures(
    verification_data: &StartEncryptedShareVerificationEvent,
) -> DwalletMPCResult<()> {
    let dkg_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput =
        bcs::from_bytes(&verification_data.dwallet_output)?;
    let signature =
        <Ed25519Signature as ToFromBytes>::from_bytes(&verification_data.signed_public_share)
            .map_err(|e| DwalletMPCError::EncryptedUserShareVerificationFailed)?;
    let public_key =
        <Ed25519PublicKey as ToFromBytes>::from_bytes(&verification_data.encryptor_ed25519_pubkey)
            .map_err(|e| DwalletMPCError::EncryptedUserShareVerificationFailed)?;
    public_key
        .verify(&bcs::to_bytes(&dkg_output.public_key_share)?, &signature)
        .map_err(|e| DwalletMPCError::EncryptedUserShareVerificationFailed)?;
    let derived_sui_addr = PeraAddress::from(&public_key);
    if derived_sui_addr != verification_data.initiator {
        return Err(DwalletMPCError::EncryptedUserSharePublicKeyDoesNotMatchAddress);
    }
    Ok(())
}

/// Verifies that the given secret encryption is the encryption of the given dwallet's secret share.
fn chain_verify_secret_share_proof(
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
