//! The `bytes_party` module defines the API for managing MPC parties within the MPC manager.
//! This module wraps the various MPC parties, providing an interface
//! to progress each party through the rounds of the MPC protocol as needed.
//!
//! The [`BytesParty`] trait enables the MPC manager to seamlessly advance the [`MPCParty`]
//! instances to the next round.
use crate::dwallet_mpc::dkg::{AsyncProtocol, FirstDKGBytesParty, SecondDKGBytesParty};
use crate::dwallet_mpc::mpc_events::{StartDKGFirstRoundEvent, StartDKGSecondRoundEvent};
use anyhow::Error;
use group::PartyID;
use pera_types::base_types::{ObjectID, PeraAddress};
use pera_types::event::Event;
use pera_types::messages_dwallet_mpc::MPCRound;
use std::collections::HashMap;
use pera_types::error::PeraError;


pub trait BytesParty: Sync + Send {
    /// Trait defining the functionality to advance an MPC party to the next round.
    ///
    /// # Arguments
    ///
    /// * `messages` — A hashmap of messages received from other parties, keyed by [`PartyID`].
    /// * `auxiliary_input` — A serialized vector of auxiliary input data.
    ///
    /// # Returns
    ///
    /// * `Ok(AdvanceResult)` on success, which represents either advancement to the next round
    ///   or the finalization of the protocol.
    /// * `Err(twopc_mpc::Error)` if an error occurs.
    fn advance(
        self,
        messages: HashMap<PartyID, Vec<u8>>,
        auxiliary_input: Vec<u8>,
    ) -> Result<AdvanceResult, twopc_mpc::Error>;
}

pub type MPCMessage = Vec<u8>;
pub type MPCOutput = Vec<u8>;

/// Represents the outcome of advancing an MPC party to the next round.
///
/// This enum indicates whether the party should advance to the next round or
/// finalize its protocol execution.
pub enum AdvanceResult {
    /// Contains the message to send to other parties, and the next [`MPCParty`] to use.
    Advance((MPCMessage, MPCParty)),
    /// Indicates that the protocol has completed, containing the final output.
    Finalize(MPCOutput),
}

/// Holds information about the current MPC session.
pub struct MPCSessionInfo {
    /// Unique identifier for the MPC session.
    pub session_id: ObjectID,
    /// The address of the user that initiated this MPC session.
    pub initiating_user_address: PeraAddress,
    /// The `DWalletCap` object's ID associated with the `DWallet` object.
    pub dwallet_cap_id: ObjectID,
    /// The current MPC round in the protocol.
    pub mpc_round: MPCRound,
}

/// Enum representing the different parties used in the MPC manager.
pub enum MPCParty {
    /// A placeholder party used as a default.
    /// Does not implement the `BytesParty` trait and should never be used.
    DefaultParty,
    /// The party used in the first round of the DKG protocol.
    FirstDKGBytesParty(FirstDKGBytesParty),
    /// The party used in the second round of the DKG protocol.
    SecondDKGBytesParty(SecondDKGBytesParty),
}

/// Default party implementation for `MPCParty`.
///
/// This variant allows using `mem::take`, which requires the `Default` trait.
/// The `DefaultParty` variant is used when a specific party has not been set.
impl Default for MPCParty {
    fn default() -> Self {
        MPCParty::DefaultParty
    }
}

impl MPCParty {
    /// Advances the party to the next round by processing incoming messages and auxiliary input.
    /// Returns the next [`MPCParty`] to use, or the final output if the protocol has completed.
    pub fn advance(
        self,
        messages: HashMap<PartyID, MPCMessage>,
        auxiliary_input: &Vec<u8>,
    ) -> Result<AdvanceResult, twopc_mpc::Error> {
        match self {
            MPCParty::FirstDKGBytesParty(party) => party.advance(messages, auxiliary_input),
            MPCParty::SecondDKGBytesParty(party) => party.advance(messages, auxiliary_input),
            MPCParty::DefaultParty => Err(twopc_mpc::Error::InvalidParameters),
        }
    }

    /// Parses an [`Event`] to extract the corresponding [`MPCParty`],
    /// auxiliary input, and session information.
    ///
    /// Returns an error if the event type does not correspond to any known MPC rounds
    /// or if deserialization fails.
    pub fn from_event(
        event: &Event,
        number_of_parties: u16,
        party_id: PartyID,
    ) -> anyhow::Result<(MPCParty, Vec<u8>, MPCSessionInfo)> {
        match &event.type_ {
            t if t == &StartDKGFirstRoundEvent::type_() => {
                let deserialized_event: StartDKGFirstRoundEvent = bcs::from_bytes(&event.contents)?;
                Self::dkg_first_party(number_of_parties, party_id, deserialized_event)
            }
            t if t == &StartDKGSecondRoundEvent::type_() => {
                let deserialized_event: StartDKGSecondRoundEvent =
                    bcs::from_bytes(&event.contents)?;
                Self::dkg_second_party(number_of_parties, party_id, deserialized_event)
            }
            _ => Err(PeraError::NonMPCEvent.into()),
        }
    }

    fn dkg_second_party(
        number_of_parties: u16,
        party_id: PartyID,
        deserialized_event: StartDKGSecondRoundEvent,
    ) -> Result<(MPCParty, Vec<u8>, MPCSessionInfo), Error> {
        Ok((
            MPCParty::SecondDKGBytesParty(SecondDKGBytesParty {
                party: <AsyncProtocol as twopc_mpc::dkg::Protocol>::ProofVerificationRoundParty::default(),
            }),
            SecondDKGBytesParty::generate_auxiliary_input(
                number_of_parties,
                party_id,
                deserialized_event.first_round_output,
                deserialized_event.public_key_share_and_proof,
                deserialized_event.first_round_session_id.bytes.to_vec(),
            )?,
            MPCSessionInfo {
                session_id: ObjectID::from(deserialized_event.session_id),
                initiating_user_address: deserialized_event.sender,
                dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
                mpc_round: MPCRound::DKGSecond,
            },
        ))
    }

    fn dkg_first_party(
        number_of_parties: u16,
        party_id: PartyID,
        deserialized_event: StartDKGFirstRoundEvent,
    ) -> Result<(MPCParty, Vec<u8>, MPCSessionInfo), Error> {
        Ok((
            MPCParty::FirstDKGBytesParty(FirstDKGBytesParty {
                party: <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareRoundParty::default(),
            }),
            FirstDKGBytesParty::generate_auxiliary_input(
                number_of_parties,
                party_id,
                deserialized_event.session_id.bytes.to_vec(),
            ),
            MPCSessionInfo {
                session_id: deserialized_event.session_id.bytes,
                initiating_user_address: deserialized_event.sender,
                dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
                mpc_round: MPCRound::DKGFirst,
            },
        ))
    }
}
