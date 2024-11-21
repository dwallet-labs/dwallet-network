//! This module provides a wrapper around the Presign protocol from the 2PC-MPC library.
//!
//! It integrates both Presign parties (each representing a round in the Presign protocol) and
//! implements the [`BytesParty`] trait for seamless interaction with other MPC components.

use group::PartyID;
use mpc::{Party, WeightedThresholdAccessStructure};

use pera_types::error::PeraResult;

use crate::dwallet_mpc::dkg::AsyncProtocol;

pub type PresignFirstParty =
    <AsyncProtocol as twopc_mpc::presign::Protocol>::EncryptionOfMaskAndMaskedNonceShareRoundParty;
pub type PresignSecondParty = <AsyncProtocol as twopc_mpc::presign::Protocol>::NoncePublicShareAndEncryptionOfMaskedNonceShareRoundParty;

/// A trait for generating auxiliary input for the initial round of the Presign protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::AuxiliaryInput`. It defines the parameters and logic
/// necessary to initiate the first round of the DKG protocol,
/// preparing the party with the essential session information and other contextual data.
pub trait PresignFirstRound: mpc::Party {
    fn generate_auxiliary_input(
        session_id: Vec<u8>,
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        party_id: PartyID,
        dkg_output: Vec<u8>,
    ) -> PeraResult<Vec<u8>>;
}

impl PresignFirstRound for PresignFirstParty {
    fn generate_auxiliary_input(
        session_id: Vec<u8>,
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        party_id: PartyID,
        dkg_output: Vec<u8>,
    ) -> PeraResult<Vec<u8>> {
        let secp256k1_group_public_parameters =
            class_groups_constants::protocol_public_parameters();
        let session_id = commitment::CommitmentSizedNumber::from_le_slice(&session_id);

        Ok(bcs::to_bytes(&Self::AuxiliaryInput {
            weighted_threshold_access_structure,
            protocol_public_parameters: secp256k1_group_public_parameters.clone(),
            party_id,
            dkg_output: bcs::from_bytes(&dkg_output)?,
            session_id,
        })?)
    }
}

/// A trait for generating auxiliary input for the last round of the Presign protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::AuxiliaryInput`. It defines the parameters and logic
/// necessary to initiate the second round of the Presign protocol,
/// preparing the party with the essential session information and other contextual data.
pub trait PresignSecondRound: Party {
    fn generate_auxiliary_input(
        session_id: Vec<u8>,
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        party_id: PartyID,
        dkg_output: Vec<u8>,
        first_round_output: Vec<u8>,
    ) -> PeraResult<Vec<u8>>;
}

impl PresignSecondRound for PresignSecondParty {
    fn generate_auxiliary_input(
        session_id: Vec<u8>,
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        party_id: PartyID,
        dkg_output: Vec<u8>,
        first_round_output: Vec<u8>,
    ) -> PeraResult<Vec<u8>> {
        let first_round_auxiliary_input = PresignFirstParty::generate_auxiliary_input(
            session_id,
            weighted_threshold_access_structure,
            party_id,
            dkg_output,
        )?;
        let first_round_auxiliary_input: <PresignFirstParty as Party>::AuxiliaryInput = bcs::from_bytes(&first_round_auxiliary_input)?;
        let first_round_output: <PresignFirstParty as Party>::Output = bcs::from_bytes(&first_round_output)?;
        let aux_input: Self::AuxiliaryInput = (first_round_auxiliary_input, first_round_output.clone()).into();
        Ok(bcs::to_bytes(&aux_input)?)
    }
}
