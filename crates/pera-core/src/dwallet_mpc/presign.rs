//! This module provides a wrapper around the Presign protocol from the 2PC-MPC library.
//!
//! It integrates both Presign parties (each representing a round in the Presign protocol)
use pera_types::error::PeraResult;
use crate::dwallet_mpc::mpc_party::AsyncProtocol;

pub(super) type PresignFirstParty =
    <AsyncProtocol as twopc_mpc::presign::Protocol>::EncryptionOfMaskAndMaskedNonceShareRoundParty;
pub(super) type PresignSecondParty = <AsyncProtocol as twopc_mpc::presign::Protocol>::NoncePublicShareAndEncryptionOfMaskedNonceShareRoundParty;

/// A trait for generating the public input for the initial round of the Presign protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::PublicInput`.
pub(super) trait PresignFirstPartyPublicInputGenerator: mpc::Party {
    fn generate_public_input(dkg_output: Vec<u8>) -> PeraResult<Vec<u8>>;
}

/// A trait for generating the public input for the last round of the Presign protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::PublicInput`.
pub(super) trait PresignSecondPartyPublicInputGenerator: mpc::Party {
    fn generate_public_input(
        dkg_output: Vec<u8>,
        first_round_output: Vec<u8>,
    ) -> PeraResult<Vec<u8>>;
}

impl PresignFirstPartyPublicInputGenerator for PresignFirstParty {
    fn generate_public_input(dkg_output: Vec<u8>) -> PeraResult<Vec<u8>> {
        let pub_input = Self::PublicInput {
            protocol_public_parameters: class_groups_constants::protocol_public_parameters(),
            dkg_output: bcs::from_bytes(&dkg_output)?,
        };
        Ok(bcs::to_bytes(&pub_input)?)
    }
}

impl PresignSecondPartyPublicInputGenerator for PresignSecondParty {
    fn generate_public_input(
        dkg_output: Vec<u8>,
        first_round_output: Vec<u8>,
    ) -> PeraResult<Vec<u8>> {
        let first_round_public_input =
            <PresignFirstParty as PresignFirstPartyPublicInputGenerator>::generate_public_input(
                dkg_output,
            )?;
        let first_round_public_input: <PresignFirstParty as mpc::Party>::PublicInput =
            bcs::from_bytes(&first_round_public_input)?;
        let first_round_output: <PresignFirstParty as mpc::Party>::PublicOutput =
            bcs::from_bytes(&first_round_output)?;
        let input: Self::PublicInput =
            (first_round_public_input, first_round_output.clone()).into();
        Ok(bcs::to_bytes(&input)?)
    }
}
