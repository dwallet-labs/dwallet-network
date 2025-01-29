//! This module provides a wrapper around the Sign protocol from the 2PC-MPC library.
//!
//! It integrates the Sign party (representing a round in the protocol).
use crate::dwallet_mpc::mpc_session::AsyncProtocol;
use dwallet_mpc_types::dwallet_mpc::{MPCPublicInput, MPCPublicOutput};
use pera_types::dwallet_mpc_error::DwalletMPCResult;
use twopc_mpc::dkg::Protocol;

/// The index of the last sign cryptographic round.
/// Needed to be known in advance as this cryptographic step should ideally get computed only once
/// by the `sign aggregation` protocol.
pub(crate) const LAST_SIGN_ROUND_INDEX: usize = 1;
/// The time a validator waits for each other validator to produce the result of the last sign
/// computation round.
/// Used to determine how long a validator should wait before running the final step of the sign
/// MPC flow.
pub(crate) const SIGN_LAST_ROUND_COMPUTATION_CONSTANT_SECONDS: usize = 30;

pub(super) type SignFirstParty =
    <AsyncProtocol as twopc_mpc::sign::Protocol>::SignDecentralizedParty;
pub(super) type SignPublicInput =
    <AsyncProtocol as twopc_mpc::sign::Protocol>::SignDecentralizedPartyPublicInput;

/// A trait for generating the public input for decentralized `Sign` round in the MPC protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing [`mpc::Party::PublicInput`].
pub(super) trait SignPartyPublicInputGenerator: mpc::Party {
    fn generate_public_input(
        protocol_public_parameters: Vec<u8>,
        dkg_output: MPCPublicOutput,
        hashed_message: Vec<u8>,
        presign: MPCPublicOutput,
        centralized_signed_message: Vec<u8>,
        decryption_key_share_public_parameters: <AsyncProtocol as twopc_mpc::sign::Protocol>::DecryptionKeySharePublicParameters,
    ) -> DwalletMPCResult<MPCPublicInput>;
}

impl SignPartyPublicInputGenerator for SignFirstParty {
    fn generate_public_input(
        protocol_public_parameters: Vec<u8>,
        dkg_output: MPCPublicOutput,
        hashed_message: Vec<u8>,
        presign: MPCPublicOutput,
        centralized_signed_message: Vec<u8>,
        decryption_key_share_public_parameters: <AsyncProtocol as twopc_mpc::sign::Protocol>::DecryptionKeySharePublicParameters,
    ) -> DwalletMPCResult<MPCPublicInput> {
        let auxiliary = SignPublicInput::from((
            bcs::from_bytes(&protocol_public_parameters)?,
            bcs::from_bytes::<<AsyncProtocol as twopc_mpc::sign::Protocol>::Message>(
                &hashed_message,
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

        Ok(bcs::to_bytes(&auxiliary)?)
    }
}
