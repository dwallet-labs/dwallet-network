//! This module provides a wrapper around the Presign protocol from the 2PC-MPC library.
//!
//! It integrates both Presign parties (each representing a round in the Presign protocol).
use dwallet_mpc_types::dwallet_mpc::{
    SerializedWrappedMPCPublicOutput, VersionedDwalletDKGSecondRoundPublicOutput,
};
use ika_types::dwallet_mpc_error::DwalletMPCResult;
use ika_types::messages_dwallet_mpc::AsyncProtocol;

pub(crate) type PresignParty = <AsyncProtocol as twopc_mpc::presign::Protocol>::PresignParty;

/// A trait for generating the public input for the Presign protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::PublicInput`.
pub(crate) trait PresignPartyPublicInputGenerator: mpc::Party {
    fn generate_public_input(
        protocol_public_parameters: twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
        dkg_output: SerializedWrappedMPCPublicOutput,
    ) -> DwalletMPCResult<<PresignParty as mpc::Party>::PublicInput>;
}

impl PresignPartyPublicInputGenerator for PresignParty {
    fn generate_public_input(
        protocol_public_parameters: twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
        dkg_output: SerializedWrappedMPCPublicOutput,
    ) -> DwalletMPCResult<<PresignParty as mpc::Party>::PublicInput> {
        let dkg_output = bcs::from_bytes(&dkg_output)?;
        match dkg_output {
            VersionedDwalletDKGSecondRoundPublicOutput::V1(output) => {
                let pub_input = Self::PublicInput {
                    protocol_public_parameters,
                    dkg_output: bcs::from_bytes(&output)?,
                };
                Ok(pub_input)
            }
        }
    }
}
