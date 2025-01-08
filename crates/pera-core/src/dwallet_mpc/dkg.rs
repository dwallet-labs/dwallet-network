//! This module provides a wrapper around the DKG protocol from the 2PC-MPC library.
//!
//! It integrates both DKG parties (each representing a round in the DKG protocol).
use crate::dwallet_mpc::mpc_session::AsyncProtocol;
use dwallet_mpc_types::dwallet_mpc::MPCPublicInput;
use mpc::Party;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use twopc_mpc::dkg::Protocol;

/// This struct represents the initial round of the DKG protocol.
pub(super) type DKGFirstParty = <AsyncProtocol as Protocol>::EncryptionOfSecretKeyShareRoundParty;
/// This struct represents the final round of the DKG protocol.
pub(super) type DKGSecondParty = <AsyncProtocol as Protocol>::ProofVerificationRoundParty;

/// A trait for generating the public input for the initial round of the DKG protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing [`Party::PublicInput`].
/// It defines the parameters and logic
/// necessary to initiate the first round of the DKG protocol,
/// preparing the party with the essential session information and other contextual data.
pub(super) trait DKGFirstPartyPublicInputGenerator: Party {
    /// Generates the public input required for the first round of the DKG protocol.
    fn generate_public_input(
        protocol_public_parameters: Vec<u8>,
    ) -> DwalletMPCResult<MPCPublicInput>;
}

/// A trait for generating the public input for the last round of the DKG protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing [`Party::PublicInput`].
/// It defines the parameters and logic
/// necessary to initiate the second round of the DKG protocol,
/// preparing the party with the essential session information and other contextual data.
pub(super) trait DKGSecondPartyPublicInputGenerator: Party {
    /// Generates the public input required for the second round of the DKG protocol.
    fn generate_public_input(
        protocol_public_parameters: Vec<u8>,
        first_round_output: Vec<u8>,
        centralized_party_public_key_share: Vec<u8>,
    ) -> DwalletMPCResult<MPCPublicInput>;
}

impl DKGFirstPartyPublicInputGenerator for DKGFirstParty {
    fn generate_public_input(
        protocol_public_parameters: Vec<u8>,
    ) -> DwalletMPCResult<MPCPublicInput> {
        let input: Self::PublicInput = bcs::from_bytes(&protocol_public_parameters)?;
        bcs::to_bytes(&input).map_err(|e| DwalletMPCError::BcsError(e))
    }
}

impl DKGSecondPartyPublicInputGenerator for DKGSecondParty {
    fn generate_public_input(
        protocol_public_parameters: Vec<u8>,
        first_round_output_buf: Vec<u8>,
        centralized_party_public_key_share_buf: Vec<u8>,
    ) -> DwalletMPCResult<MPCPublicInput> {
        let first_round_output: <DKGFirstParty as Party>::PublicOutput =
            bcs::from_bytes(&first_round_output_buf).map_err(|e| DwalletMPCError::BcsError(e))?;
        let centralized_party_public_key_share: <AsyncProtocol as Protocol>::PublicKeyShareAndProof =
            bcs::from_bytes(&centralized_party_public_key_share_buf).map_err(|e| DwalletMPCError::BcsError(e))?;

        let input: Self::PublicInput = (
            bcs::from_bytes(&protocol_public_parameters)?,
            first_round_output,
            centralized_party_public_key_share,
        )
            .into();
        bcs::to_bytes(&input).map_err(|e| DwalletMPCError::BcsError(e))
    }
}
