//! This module provides a wrapper around the DKG protocol from the 2PC-MPC library.
//!
//! It integrates both DKG parties (each representing a round in the DKG protocol) and
//! implements the [`BytesParty`] trait for seamless interaction with other MPC components.

use crate::dwallet_mpc::bytes_party::{AdvanceResult, BytesParty, MPCParty};
use group::PartyID;
use mpc::{Advance, Party};
use std::collections::{HashMap, HashSet};
use twopc_mpc::dkg::Protocol;

pub type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;
pub type DKGFirstParty = <AsyncProtocol as Protocol>::EncryptionOfSecretKeyShareRoundParty;
pub type DKGSecondParty = <AsyncProtocol as Protocol>::ProofVerificationRoundParty;

/// A wrapper for the first round of the DKG protocol.
///
/// This struct represents the initial round of the DKG protocol.
pub struct FirstDKGBytesParty {
    pub(crate) party: DKGFirstParty,
}

impl FirstDKGBytesParty {
    /// Generates the auxiliary input required for the first DKG round.
    /// It is necessary for advancing the party to the next round of the DKG protocol.
    ///
    /// # Arguments
    ///
    /// * `number_of_parties` - The total number of participating parties.
    /// * `party_id` - The ID of the current party.
    /// * `session_id` - A unique identifier for the MPC session.
    ///
    /// # Returns
    ///
    /// A serialized vector containing the auxiliary input data required to advance
    /// the party to the next round.
    pub fn generate_auxiliary_input(
        number_of_parties: u16,
        party_id: PartyID,
        session_id: Vec<u8>,
    ) -> Vec<u8> {
        bcs::to_bytes(&DKGFirstParty::generate_auxiliary_input(
            session_id,
            number_of_parties,
            party_id,
        ))
        .unwrap()
    }
}

impl BytesParty for FirstDKGBytesParty {
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
                let message =
                    bcs::from_bytes(&v).map_err(|_| twopc_mpc::Error::InvalidParameters);
                return match message {
                    Ok(message) => Ok((k, message)),
                    Err(err) => Err(err),
                }
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        let result = self
            .party
            .advance(messages, &auxiliary_input, &mut rand_core::OsRng)?;

        match result {
            mpc::AdvanceResult::Advance((message, new_party)) => Ok(AdvanceResult::Advance((
                bcs::to_bytes(&message).unwrap(),
                MPCParty::FirstDKGBytesParty(Self { party: new_party }),
            ))),
            mpc::AdvanceResult::Finalize(output) => {
                Ok(AdvanceResult::Finalize(bcs::to_bytes(&output).unwrap()))
            }
        }
    }
}

/// A trait for generating auxiliary input for the initial round of the DKG protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::AuxiliaryInput`. It defines the parameters and logic
/// necessary to initiate the first round of the DKG protocol,
/// preparing the party with the essential session information and other contextual data.
trait DKGFirstRound: Party {
    /// Generates the auxiliary input required for the first round of the DKG protocol.
    ///
    /// # Arguments
    ///
    /// * `session_id` - A unique identifier for the current MPC session.
    /// * `number_of_parties` - The total number of participating parties in the DKG session.
    /// * `party_id` - The unique ID of the current party.
    ///
    /// # Returns
    ///
    /// A structured auxiliary input compatible with the DKG protocol's requirements,
    /// enabling the party to correctly initiate and participate in the first round.
    fn generate_auxiliary_input(
        session_id: Vec<u8>,
        number_of_parties: u16,
        party_id: PartyID,
    ) -> Self::AuxiliaryInput;
}

impl DKGFirstRound for DKGFirstParty {
    fn generate_auxiliary_input(
        session_id: Vec<u8>,
        number_of_parties: u16,
        party_id: PartyID,
    ) -> Self::AuxiliaryInput {
        let secp256k1_group_public_parameters =
            class_groups_constants::protocol_public_parameters();

        let parties = (0..number_of_parties).collect::<HashSet<PartyID>>();
        let session_id = commitment::CommitmentSizedNumber::from_le_slice(&session_id);
        Self::AuxiliaryInput {
            protocol_public_parameters: secp256k1_group_public_parameters,
            party_id,
            threshold: ((number_of_parties * 2) + 2) / 3,
            number_of_parties,
            parties: parties.clone(),
            session_id,
        }
    }
}

/// A wrapper for the second round of the DKG protocol.
///
/// This struct represents the final round of the DKG protocol.
pub struct SecondDKGBytesParty {
    pub(crate) party: DKGSecondParty,
}

impl SecondDKGBytesParty {
    /// Generates the auxiliary input required for the second DKG round.
    /// It is necessary for advancing the party to the next round of the DKG protocol.
    ///
    /// # Arguments
    ///
    /// * `number_of_parties` - The total number of participating parties.
    /// * `party_id` - The ID of the current party.
    /// * `first_round_output` - The output from the first round of the DKG protocol.
    /// * `centralized_party_public_key_share` - The public key share of the centralized party.
    /// * `session_id` - A unique identifier for the MPC session.
    ///
    /// # Returns
    ///
    /// A serialized vector containing the auxiliary input data required to advance
    /// the party.
    pub fn generate_auxiliary_input(
        number_of_parties: u16,
        party_id: PartyID,
        first_round_output: Vec<u8>,
        centralized_party_public_key_share: Vec<u8>,
        session_id: Vec<u8>,
    ) -> anyhow::Result<Vec<u8>> {
        bcs::to_bytes(&DKGSecondParty::generate_auxiliary_input(
            number_of_parties,
            party_id,
            bcs::from_bytes(&first_round_output)?,
            bcs::from_bytes(&centralized_party_public_key_share)?,
            session_id,
        ))
        .map_err(|err| err.into())
    }
}

impl BytesParty for SecondDKGBytesParty {
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
                let message =
                    bcs::from_bytes(&v).map_err(|_| twopc_mpc::Error::InvalidParameters);
                return match message {
                    Ok(message) => Ok((k, message)),
                    Err(err) => Err(err),
                }
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        let result = self
            .party
            .advance(messages, &auxiliary_input, &mut rand_core::OsRng)?;

        match result {
            mpc::AdvanceResult::Advance((message, new_party)) => Ok(AdvanceResult::Advance((
                bcs::to_bytes(&message).unwrap(),
                MPCParty::SecondDKGBytesParty(Self { party: new_party }),
            ))),
            mpc::AdvanceResult::Finalize(output) => {
                Ok(AdvanceResult::Finalize(bcs::to_bytes(&output).unwrap()))
            }
        }
    }
}

/// A trait for generating auxiliary input for the last round of the DKG protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::AuxiliaryInput`. It defines the parameters and logic
/// necessary to initiate the second round of the DKG protocol,
/// preparing the party with the essential session information and other contextual data.
trait DKGSecondRound: Party {
    /// Generates the auxiliary input required for the second round of the DKG protocol.
    ///
    /// # Arguments
    ///
    /// * `number_of_parties` - The total number of participating parties in the DKG session.
    /// * `party_id` - The unique ID of the current party.
    /// * `first_round_output` - The output from the first round of the DKG protocol.
    /// * `centralized_party_public_key_share` - The public key share of the centralized party.
    /// * `session_id` - A unique identifier for the current MPC session.
    ///
    /// # Returns
    ///
    /// A structured auxiliary input compatible with the DKG protocol's requirements,
    /// enabling the party to correctly initiate and participate in the second round.
    fn generate_auxiliary_input(
        number_of_parties: u16,
        party_id: PartyID,
        first_round_output: <DKGFirstParty as Party>::Output,
        centralized_party_public_key_share: <AsyncProtocol as Protocol>::PublicKeyShareAndProof,
        session_is: Vec<u8>,
    ) -> Self::AuxiliaryInput;
}

impl DKGSecondRound for DKGSecondParty {
    fn generate_auxiliary_input(
        number_of_parties: u16,
        party_id: PartyID,
        first_round_output: <DKGFirstParty as Party>::Output,
        centralized_party_public_key_share: <AsyncProtocol as Protocol>::PublicKeyShareAndProof,
        session_id: Vec<u8>,
    ) -> Self::AuxiliaryInput {
        let first_round_auxiliary_input = DKGFirstParty::generate_auxiliary_input(
            session_id.clone(),
            number_of_parties,
            party_id,
        );
        (
            first_round_auxiliary_input,
            first_round_output,
            centralized_party_public_key_share,
        )
            .into()
    }
}