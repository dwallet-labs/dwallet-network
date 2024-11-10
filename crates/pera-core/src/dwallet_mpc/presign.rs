//! This module provides a wrapper around the Presign protocol from the 2PC-MPC library.
//!
//! It integrates both Presign parties (each representing a round in the Presign protocol) and
//! implements the [`BytesParty`] trait for seamless interaction with other MPC components.

use crate::dwallet_mpc::bytes_party::{AdvanceResult, BytesParty, MPCParty};
use crate::dwallet_mpc::dkg::deserialize_mpc_messages;
use crate::dwallet_mpc::mpc_manager::twopc_error_to_pera_error;
use group::PartyID;
use mpc::{Advance, Party};
use pera_types::error::{PeraError, PeraResult};
use std::collections::{HashMap, HashSet};

pub type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;
pub type PresignFirstParty =
    <AsyncProtocol as twopc_mpc::presign::Protocol>::EncryptionOfMaskAndMaskedNonceShareRoundParty;
pub type PresignSecondParty = <AsyncProtocol as twopc_mpc::presign::Protocol>::NoncePublicShareAndEncryptionOfMaskedNonceShareRoundParty;

/// A wrapper for the first round of the Presign protocol.
///
/// This struct represents the initial round of the Presign protocol.
pub struct FirstPresignBytesParty {
    pub party: PresignFirstParty,
}

impl FirstPresignBytesParty {
    /// Generates the auxiliary input required for the first Presign round.
    /// It is necessary for advancing the party to the next round of the Presign protocol.
    pub(crate) fn generate_auxiliary_input(
        session_id: Vec<u8>,
        number_of_parties: u16,
        party_id: PartyID,
        dkg_output: Vec<u8>,
    ) -> Vec<u8> {
        bcs::to_bytes(&PresignFirstParty::generate_auxiliary_input(
            session_id,
            number_of_parties,
            party_id,
            dkg_output,
        ))
        .unwrap()
    }
}

impl BytesParty for FirstPresignBytesParty {
    fn advance(
        self,
        messages: HashMap<PartyID, Vec<u8>>,
        auxiliary_input: Vec<u8>,
    ) -> PeraResult<AdvanceResult> {
        let auxiliary_input =
            // This is not a validator malicious behaviour, as the authority input is being sent by the initiating user.
            // In this case this MPC session should be cancelled.
            bcs::from_bytes(&auxiliary_input).map_err(|_| PeraError::DWalletMPCInvalidUserInput)?;
        let result = self
            .party
            .advance(
                deserialize_mpc_messages(messages)?,
                &auxiliary_input,
                &mut rand_core::OsRng,
            )
            .map_err(twopc_error_to_pera_error)?;
        match result {
            mpc::AdvanceResult::Advance((message, new_party)) => Ok(AdvanceResult::Advance((
                bcs::to_bytes(&message).unwrap(),
                MPCParty::FirstPresignBytesParty(Self { party: new_party }),
            ))),
            mpc::AdvanceResult::Finalize(output) => {
                Ok(AdvanceResult::Finalize(bcs::to_bytes(&output).unwrap()))
            }
        }
    }
}

/// A trait for generating auxiliary input for the initial round of the Presign protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::AuxiliaryInput`. It defines the parameters and logic
/// necessary to initiate the first round of the DKG protocol,
/// preparing the party with the essential session information and other contextual data.
pub trait PresignFirstRound: mpc::Party {
    fn generate_auxiliary_input(
        session_id: Vec<u8>,
        number_of_parties: u16,
        party_id: PartyID,
        dkg_output: Vec<u8>,
    ) -> Self::AuxiliaryInput;
}

impl PresignFirstRound for PresignFirstParty {
    fn generate_auxiliary_input(
        session_id: Vec<u8>,
        number_of_parties: u16,
        party_id: PartyID,
        dkg_output: Vec<u8>,
    ) -> Self::AuxiliaryInput {
        let secp256k1_group_public_parameters =
            class_groups_constants::protocol_public_parameters();

        let parties = (0..number_of_parties).collect::<HashSet<PartyID>>();
        let session_id = commitment::CommitmentSizedNumber::from_le_slice(&session_id);
        Self::AuxiliaryInput {
            protocol_public_parameters: secp256k1_group_public_parameters.clone(),
            party_id,
            threshold: ((number_of_parties * 2) + 2) / 3,
            number_of_parties,
            dkg_output: bcs::from_bytes(&dkg_output).unwrap(), // todo: remove unwrap
            parties: parties.clone(),
            session_id,
        }
    }
}

/// A wrapper for the second round of the Presign protocol.
///
/// This struct represents the final round of the Presign protocol.
pub struct SecondPresignBytesParty {
    pub party: PresignSecondParty,
}
impl SecondPresignBytesParty {
    /// Generates the auxiliary input required for the second Presign round.
    /// It is necessary for advancing the party to the next round of the Presign protocol.
    /// The `session_id` is the unique identifier for the MPC session from the first round.
    pub(crate) fn generate_auxiliary_input(
        session_id: Vec<u8>,
        number_of_parties: u16,
        party_id: PartyID,
        dkg_output: Vec<u8>,
        first_round_output: Vec<u8>,
    ) -> Vec<u8> {
        let first_round_output = bcs::from_bytes(&first_round_output).unwrap();
        bcs::to_bytes(&PresignSecondParty::generate_auxiliary_input(
            session_id,
            number_of_parties,
            party_id,
            dkg_output,
            first_round_output,
        ))
        .unwrap()
    }
}

impl BytesParty for SecondPresignBytesParty {
    fn advance(
        self,
        messages: HashMap<PartyID, Vec<u8>>,
        auxiliary_input: Vec<u8>,
    ) -> PeraResult<AdvanceResult> {
        let auxiliary_input =
            // This is not a validator malicious behaviour, as the authority input is being sent by the initiating user.
            // In this case this MPC session should be cancelled.
            bcs::from_bytes(&auxiliary_input).map_err(|_| PeraError::DWalletMPCInvalidUserInput)?;
        let result = self
            .party
            .advance(
                deserialize_mpc_messages(messages)?,
                &auxiliary_input,
                &mut rand_core::OsRng,
            )
            .map_err(twopc_error_to_pera_error)?;
        match result {
            mpc::AdvanceResult::Advance((message, new_party)) => Ok(AdvanceResult::Advance((
                bcs::to_bytes(&message).unwrap(),
                MPCParty::SecondPresignBytesParty(Self { party: new_party }),
            ))),
            mpc::AdvanceResult::Finalize(output) => {
                Ok(AdvanceResult::Finalize(bcs::to_bytes(&output).unwrap()))
            }
        }
    }
}

/// A trait for generating auxiliary input for the last round of the Presign protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::AuxiliaryInput`. It defines the parameters and logic
/// necessary to initiate the second round of the Presign protocol,
/// preparing the party with the essential session information and other contextual data.
pub trait PresignSecondRound: mpc::Party {
    fn generate_auxiliary_input(
        session_id: Vec<u8>,
        number_of_parties: u16,
        party_id: PartyID,
        dkg_output: Vec<u8>,
        first_round_output: <PresignFirstParty as Party>::Output,
    ) -> Self::AuxiliaryInput;
}

impl PresignSecondRound for PresignSecondParty {
    fn generate_auxiliary_input(
        session_id: Vec<u8>,
        number_of_parties: u16,
        party_id: PartyID,
        dkg_output: Vec<u8>,
        first_round_output: <PresignFirstParty as Party>::Output,
    ) -> Self::AuxiliaryInput {
        let first_round_auxiliary_input = PresignFirstParty::generate_auxiliary_input(
            session_id,
            number_of_parties,
            party_id,
            dkg_output,
        );
        (first_round_auxiliary_input, first_round_output.clone()).into()
    }
}
