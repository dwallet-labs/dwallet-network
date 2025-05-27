//! This module provides a wrapper around the Sign protocol from the 2PC-MPC library.
//!
//! It integrates the Sign party (representing a round in the protocol).

use crate::dwallet_mpc::mpc_session::AsyncProtocol;
use dwallet_mpc_types::dwallet_mpc::{
    MPCPublicInput, SerializedWrappedMPCPublicOutput, VersionedDwalletDKGSecondRoundPublicOutput,
    VersionedPresignOutput, VersionedUserSignedMessage,
};
use group::PartyID;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use mpc::Party;
use std::collections::HashSet;
use twopc_mpc::dkg::Protocol;
use twopc_mpc::secp256k1;
use twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters;

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
        dkg_output: SerializedWrappedMPCPublicOutput,
        message: Vec<u8>,
        presign: SerializedWrappedMPCPublicOutput,
        centralized_signed_message: Vec<u8>,
        decryption_key_share_public_parameters: <AsyncProtocol as twopc_mpc::sign::Protocol>::DecryptionKeySharePublicParameters,
        expected_decrypters: HashSet<PartyID>,
    ) -> DwalletMPCResult<MPCPublicInput>;
}

impl SignPartyPublicInputGenerator for SignFirstParty {
    fn generate_public_input(
        protocol_public_parameters: Vec<u8>,
        dkg_output: SerializedWrappedMPCPublicOutput,
        message: Vec<u8>,
        presign: SerializedWrappedMPCPublicOutput,
        centralized_signed_message: SerializedWrappedMPCPublicOutput,
        decryption_key_share_public_parameters: <AsyncProtocol as twopc_mpc::sign::Protocol>::DecryptionKeySharePublicParameters,
        expected_decrypters: HashSet<PartyID>,
    ) -> DwalletMPCResult<MPCPublicInput> {
        let dkg_output = bcs::from_bytes(&dkg_output)?;
        let presign = bcs::from_bytes(&presign)?;
        let centralized_signed_message = bcs::from_bytes(&centralized_signed_message)?;
        match dkg_output {
            VersionedDwalletDKGSecondRoundPublicOutput::V1(output) => {
                let VersionedPresignOutput::V1(presign) = presign;
                let VersionedUserSignedMessage::V1(centralized_signed_message) =
                    centralized_signed_message;
                let public_input = SignPublicInput::from((
                    expected_decrypters,
                    bcs::from_bytes(&protocol_public_parameters)?,
                    bcs::from_bytes::<<AsyncProtocol as twopc_mpc::sign::Protocol>::HashedMessage>(
                        &message,
                    )?,
                    bcs::from_bytes::<<AsyncProtocol as Protocol>::DecentralizedPartyDKGOutput>(
                        &output,
                    )?,
                    bcs::from_bytes::<<AsyncProtocol as twopc_mpc::presign::Protocol>::Presign>(
                        &presign,
                    )?,
                    bcs::from_bytes::<<AsyncProtocol as twopc_mpc::sign::Protocol>::SignMessage>(
                        &centralized_signed_message,
                    )?,
                    decryption_key_share_public_parameters,
                ));

                Ok(bcs::to_bytes(&public_input)?)
            }
        }
    }
}

/// Verifies that a single partial signature — i.e., a message that has only been signed by the
/// client side in the 2PC-MPC protocol — is valid regarding the given dWallet DKG output.
/// Returns Ok if the message is valid, Err otherwise.
pub(crate) fn verify_partial_signature(
    hashed_message: &[u8],
    dwallet_decentralized_output: &SerializedWrappedMPCPublicOutput,
    presign: &SerializedWrappedMPCPublicOutput,
    partially_signed_message: &SerializedWrappedMPCPublicOutput,
    protocol_public_parameters: &ProtocolPublicParameters,
) -> DwalletMPCResult<()> {
    let dkg_output: VersionedDwalletDKGSecondRoundPublicOutput =
        bcs::from_bytes(dwallet_decentralized_output)?;
    let presign: VersionedPresignOutput = bcs::from_bytes(presign)?;
    let partially_signed_message: VersionedUserSignedMessage =
        bcs::from_bytes(partially_signed_message)?;
    match dkg_output {
        VersionedDwalletDKGSecondRoundPublicOutput::V1(dkg_output) => {
            let VersionedPresignOutput::V1(presign) = presign;
            let VersionedUserSignedMessage::V1(partially_signed_message) = partially_signed_message;
            let message: secp256k1::Scalar = bcs::from_bytes(hashed_message)?;
            let dkg_output = bcs::from_bytes::<
                <AsyncProtocol as Protocol>::DecentralizedPartyDKGOutput,
            >(&dkg_output)?;
            let presign: <AsyncProtocol as twopc_mpc::presign::Protocol>::Presign =
                bcs::from_bytes(&presign)?;
            let partial: <AsyncProtocol as twopc_mpc::sign::Protocol>::SignMessage =
                bcs::from_bytes(&partially_signed_message)?;
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
    }
}
