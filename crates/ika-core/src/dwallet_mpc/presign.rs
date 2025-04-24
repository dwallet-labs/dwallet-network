//! This module provides a wrapper around the Presign protocol from the 2PC-MPC library.
//!
//! It integrates both Presign parties (each representing a round in the Presign protocol).
use crate::dwallet_mpc::mpc_session::AsyncProtocol;
use dwallet_mpc_types::dwallet_mpc::{
    MPCPublicInput, MPCPublicOutput, MPCPublicOutputClassGroups, SerializedWrappedPublicOutput,
};
use ika_types::dwallet_mpc_error::DwalletMPCResult;

pub(super) type PresignParty = <AsyncProtocol as twopc_mpc::presign::Protocol>::PresignParty;

/// A trait for generating the public input for the Presign protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::PublicInput`.
pub(super) trait PresignPartyPublicInputGenerator: mpc::Party {
    fn generate_public_input(
        protocol_public_parameters: Vec<u8>,
        dkg_output: SerializedWrappedPublicOutput,
    ) -> DwalletMPCResult<MPCPublicInput>;
}

impl PresignPartyPublicInputGenerator for PresignParty {
    fn generate_public_input(
        protocol_public_parameters: Vec<u8>,
        dkg_output: SerializedWrappedPublicOutput,
    ) -> DwalletMPCResult<MPCPublicInput> {
        let dkg_output = bcs::from_bytes(&dkg_output)?;
        match dkg_output {
            MPCPublicOutput::ClassGroups(MPCPublicOutputClassGroups::V1(output)) => {
                let pub_input = Self::PublicInput {
                    protocol_public_parameters: bcs::from_bytes(&protocol_public_parameters)?,
                    dkg_output: bcs::from_bytes(&output)?,
                };
                Ok(bcs::to_bytes(&pub_input)?)
            }
        }
    }
}
