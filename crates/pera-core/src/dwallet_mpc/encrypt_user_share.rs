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
use pera_types::messages_dwallet_mpc::{
    StartEncryptedShareVerificationEvent, StartEncryptionKeyVerificationEvent,
};
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
    verification_data: &StartEncryptedShareVerificationEvent,
) -> DwalletMPCResult<()> {
    verify_dwallet_public_output_signature(&verification_data)?;
    verify_centralized_secret_key_share_proof(
        &verification_data.encrypted_centralized_secret_share_and_proof,
        &verification_data.dkg_public_output,
        &verification_data.encryption_key,
    )
    .map_err(|_| DwalletMPCError::EncryptedUserShareVerificationFailed)
}

/// Verifies that the `verification_data`'s public key is matching the initiator Sui address.
pub(crate) fn verify_encryption_key(
    verification_data: &StartEncryptionKeyVerificationEvent,
) -> DwalletMPCResult<()> {
    let public_key =
        <Ed25519PublicKey as ToFromBytes>::from_bytes(&verification_data.sender_sui_pubkey)
            .map_err(|e| DwalletMPCError::EncryptedUserShareVerificationFailed)?;
    let derived_sui_addr = PeraAddress::from(&public_key);
    if derived_sui_addr != verification_data.initiator {
        return Err(DwalletMPCError::EncryptedUserSharePublicKeyDoesNotMatchAddress);
    }
    Ok(())
}

/// Verify the signature for the public output of the dWallet,
/// and that the public key that signed the dWallet public output
/// is matching the address that created and signed this encryption key share.
fn verify_dwallet_public_output_signature(
    verification_data: &StartEncryptedShareVerificationEvent,
) -> DwalletMPCResult<()> {
    let public_key =
        <Ed25519PublicKey as ToFromBytes>::from_bytes(&verification_data.initiator_public_key)
            .map_err(|e| DwalletMPCError::EncryptedUserShareVerificationFailed)?;
    let derived_ika_addr = PeraAddress::from(&public_key);
    if derived_ika_addr != verification_data.initiator {
        return Err(DwalletMPCError::EncryptedUserSharePublicKeyDoesNotMatchAddress);
    }
    let signature = <Ed25519Signature as ToFromBytes>::from_bytes(
        &verification_data.dkg_public_output_signature,
    )
    .map_err(|e| DwalletMPCError::EncryptedUserShareVerificationFailed)?;
    public_key
        .verify(&verification_data.dkg_public_output, &signature)
        .map_err(|e| DwalletMPCError::EncryptedUserShareVerificationFailed)?;
    Ok(())
}

/// Verifies that the given centralized secret key share
/// encryption is the encryption of the given dWallet's secret share.
fn verify_centralized_secret_key_share_proof(
    encrypted_centralized_secret_share_and_proof: &Vec<u8>,
    serialized_dkg_public_output: &Vec<u8>,
    encryption_key: &Vec<u8>,
) -> anyhow::Result<()> {
    let protocol_public_params = protocol_public_parameters();
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
    let dkg_public_output: <AsyncProtocol as twopc_mpc::dkg::Protocol>::CentralizedPartyDKGPublicOutput =
        bcs::from_bytes(serialized_dkg_public_output)?;
    let (proof, encrypted_centralized_secret_key_share): (
        SecretShareEncryptionProof,
        CiphertextSpaceValue<SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS>,
    ) = bcs::from_bytes(encrypted_centralized_secret_share_and_proof)?;
    let encrypted_centralized_secret_key_share_for_statement = CiphertextSpaceGroupElement::new(
        encrypted_centralized_secret_key_share,
        &language_public_parameters
            .encryption_scheme_public_parameters
            .ciphertext_space_public_parameters(),
    )?;
    let centralized_public_key_share = secp256k1::GroupElement::new(
        dkg_public_output.public_key_share,
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
