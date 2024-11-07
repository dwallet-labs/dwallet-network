//! This module provides a wrapper around the Presign protocol from the 2PC-MPC library.
//!
//! It integrates both Presign parties (each representing a round in the Presign protocol) and
//! implements the [`BytesParty`] trait for seamless interaction with other MPC components.

use crate::dwallet_mpc::bytes_party::{AdvanceResult, BytesParty, MPCParty};
use group::PartyID;
use mpc::{Advance, Party};
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
    ///
    /// # Arguments
    ///
    /// * `number_of_parties` - The total number of participating parties.
    /// * `party_id` - The ID of the current party.
    /// * `session_id` - A unique identifier for the MPC session.
    /// * `dkg_output` - The decentralized final output of the DKG protocol.
    ///
    /// # Returns
    ///
    /// A serialized vector containing the auxiliary input data required to advance
    /// the party to the next round.
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
    ) -> Result<AdvanceResult, twopc_mpc::Error> {
        let auxiliary_input =
            bcs::from_bytes(&auxiliary_input).map_err(|_| twopc_mpc::Error::InvalidParameters)?;
        let messages = messages
            .into_iter()
            .map(|(k, v)| {
                let message = bcs::from_bytes(&v).map_err(|_| twopc_mpc::Error::InvalidParameters);
                return match message {
                    Ok(message) => Ok((k, message)),
                    Err(err) => Err(err),
                };
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        let result = self
            .party
            .advance(messages, &auxiliary_input, &mut rand_core::OsRng)?;

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
    ///
    /// # Arguments
    ///
    /// * `number_of_parties` - The total number of participating parties.
    /// * `party_id` - The ID of the current party.
    /// * `first_round_output` - The output from the first round of the Presign protocol.
    /// * `dkg_output` - The decentralized final output of the DKG protocol.
    /// * `session_id` - A unique identifier for the MPC session.(session ID of the first round)
    ///
    /// # Returns
    ///
    /// A serialized vector containing the auxiliary input data required to advance
    /// the party.
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
    ) -> Result<AdvanceResult, twopc_mpc::Error> {
        let auxiliary_input =
            bcs::from_bytes(&auxiliary_input).map_err(|_| twopc_mpc::Error::InvalidParameters)?;
        let messages = messages
            .into_iter()
            .map(|(k, v)| {
                let message = bcs::from_bytes(&v).map_err(|_| twopc_mpc::Error::InvalidParameters);
                return match message {
                    Ok(message) => Ok((k, message)),
                    Err(err) => Err(err),
                };
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        let result = self
            .party
            .advance(messages, &auxiliary_input, &mut rand_core::OsRng)?;

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
