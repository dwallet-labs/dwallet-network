//! This module provides a wrapper around the Sign protocol from the 2PC-MPC library.
//!
//! It integrates the Sign party (representing a round in the protocol).

use crate::dwallet_mpc::mpc_session::AsyncProtocol;
use dwallet_mpc_types::dwallet_mpc::{MPCPublicInput, MPCPublicOutput};
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use mpc::Party;
use twopc_mpc::dkg::Protocol;
use twopc_mpc::secp256k1;
use twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters;

/// The index of the last sign cryptographic round.
/// Needed to be known in advance as this cryptographic step should ideally get computed only once
/// by the `sign aggregation` protocol.
pub(crate) const LAST_SIGN_ROUND_INDEX: usize = 1;
/// The time a validator waits for each other validator to produce the result of the last sign
/// computation round.
/// Used to determine how long a validator should wait before running the final step of the sign
/// MPC flow.
pub(crate) const SIGN_LAST_ROUND_COMPUTATION_CONSTANT_SECONDS: usize = 15;

pub(super) type SignFirstParty =
    <AsyncProtocol as twopc_mpc::sign::Protocol>::SignDecentralizedParty;
pub(super) type SignPublicInput =
    <AsyncProtocol as twopc_mpc::sign::Protocol>::SignDecentralizedPartyPublicInput;

/// A trait for generating the public input for decentralized `Sign` round in the MPC protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing [`Party::PublicInput`].
pub(super) trait SignPartyPublicInputGenerator: Party {
    fn generate_public_input(
        protocol_public_parameters: Vec<u8>,
        dkg_output: MPCPublicOutput,
        message: Vec<u8>,
        presign: MPCPublicOutput,
        centralized_signed_message: Vec<u8>,
        decryption_key_share_public_parameters: <AsyncProtocol as twopc_mpc::sign::Protocol>::DecryptionKeySharePublicParameters,
    ) -> DwalletMPCResult<MPCPublicInput>;
}

impl SignPartyPublicInputGenerator for SignFirstParty {
    fn generate_public_input(
        protocol_public_parameters: Vec<u8>,
        dkg_output: MPCPublicOutput,
        message: Vec<u8>,
        presign: MPCPublicOutput,
        centralized_signed_message: Vec<u8>,
        decryption_key_share_public_parameters: <AsyncProtocol as twopc_mpc::sign::Protocol>::DecryptionKeySharePublicParameters,
    ) -> DwalletMPCResult<MPCPublicInput> {
        let public_input = SignPublicInput::from((
            bcs::from_bytes(&protocol_public_parameters)?,
            bcs::from_bytes::<<AsyncProtocol as twopc_mpc::sign::Protocol>::HashedMessage>(
                &message,
            )?,
            bcs::from_bytes::<<AsyncProtocol as Protocol>::DecentralizedPartyDKGOutput>(
                &dkg_output,
            )?,
            bcs::from_bytes::<<AsyncProtocol as twopc_mpc::presign::Protocol>::Presign>(&presign)?,
            bcs::from_bytes::<<AsyncProtocol as twopc_mpc::sign::Protocol>::SignMessage>(
                &centralized_signed_message,
            )?,
            decryption_key_share_public_parameters,
        ));

        Ok(bcs::to_bytes(&public_input)?)
    }
}

/// Verifies that a single partial signature — i.e., a message that has only been signed by the
/// client side in the 2PC-MPC protocol — is valid regarding the given dWallet DKG output.
/// Returns Ok if the message is valid, Err otherwise.
pub(crate) fn verify_partial_signature(
    hashed_message: &[u8],
    dwallet_decentralized_output: &[u8],
    presign: &[u8],
    partially_signed_message: &[u8],
    protocol_public_parameters: &ProtocolPublicParameters,
) -> DwalletMPCResult<()> {
    let message: secp256k1::Scalar = bcs::from_bytes(hashed_message)?;
    let dkg_output = bcs::from_bytes::<<AsyncProtocol as Protocol>::DecentralizedPartyDKGOutput>(
        &dwallet_decentralized_output,
    )?;
    let presign: <AsyncProtocol as twopc_mpc::presign::Protocol>::Presign =
        bcs::from_bytes(presign)?;
    let partial: <AsyncProtocol as twopc_mpc::sign::Protocol>::SignMessage =
        bcs::from_bytes(partially_signed_message)?;
    twopc_mpc::sign::decentralized_party::signature_partial_decryption_round::Party::verify_encryption_of_signature_parts_prehash_class_groups(
        protocol_public_parameters,
        dkg_output,
        presign,
        partial,
        message,
    ).map_err(|err| {
        DwalletMPCError::TwoPCMPCError(format!("{:?}", err))
    })
}
