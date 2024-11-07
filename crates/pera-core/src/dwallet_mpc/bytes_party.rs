//! The `bytes_party` module defines the API for managing MPC parties within the MPC manager.
//! This module wraps the various MPC parties, providing an interface
//! to progress each party through the rounds of the MPC protocol as needed.
//!
//! The `BytesParty` trait enables the MPC manager to seamlessly advance the `MPCParty`
//! instances to the next round.

use crate::dwallet_mpc::dkg::{deserialize_mpc_messages, AsyncProtocol, DKGFirstParty, DKGSecondParty, FirstDKGBytesParty, SecondDKGBytesParty};
use crate::dwallet_mpc::mpc_events::{
    StartDKGFirstRoundEvent, StartDKGSecondRoundEvent, StartPresignFirstRoundEvent,
    StartPresignSecondRoundEvent,
};
use crate::dwallet_mpc::presign::{
    FirstPresignBytesParty, PresignFirstParty, PresignSecondParty, SecondPresignBytesParty,
};
use group::PartyID;
use pera_types::base_types::ObjectID;
use pera_types::error::{PeraError, PeraResult};
use pera_types::event::Event;
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use std::collections::HashMap;
use mpc::{Error, Party};
use twopc_mpc::class_groups::EncryptionOfSecretKeyShareRoundParty;
use crate::dwallet_mpc::mpc_manager::{mpc_error_to_pera_error, twopc_error_to_pera_error};

/// Trait defining the functionality to advance an MPC party to the next round.
///
/// # Arguments
///
/// * `messages` - A hashmap of messages received from other parties, keyed by `PartyID`.
/// * `auxiliary_input` - A serialized vector of auxiliary input data.
///
/// # Returns
///
/// * `Ok(AdvanceResult)` on success, which represents either advancement to the next round
///   or the finalization of the protocol.
/// * `Err(twopc_mpc::Error)` if an error occurs.
pub trait BytesParty: Sync + Send {
    fn advance(
        self,
        messages: HashMap<PartyID, Vec<u8>>,
        auxiliary_input: Vec<u8>,
    ) -> PeraResult<AdvanceResult>;
}

/// Represents the outcome of advancing an MPC party to the next round.
///
/// This enum indicates whether the party should advance to the next round or
/// finalize its protocol execution.
pub enum AdvanceResult {
    /// Contains the message to send to other parties and the next `MPCParty` to use.
    Advance((Vec<u8>, MPCParty)),
    /// Indicates that the protocol has completed, containing the final output.
    Finalize(Vec<u8>),
}

/// Enum representing the different parties used in the MPC manager.
pub enum MPCParty {
    /// A placeholder party used as a default. Does not implement the `BytesParty` trait and should never be used.
    DefaultParty,
    /// The party used in the first round of the DKG protocol.
    FirstDKGBytesParty(DKGFirstParty),
    /// The party used in the second round of the DKG protocol.
    SecondDKGBytesParty(DKGSecondParty),
    /// The party used in the first round of the presign protocol.
    FirstPresignBytesParty(FirstPresignBytesParty),
    /// The party used in the second round of the presign protocol.
    SecondPresignBytesParty(SecondPresignBytesParty),
}

/// Default party implementation for `MPCParty`.
///
/// This variant allows the use of `mem::take`, which requires the `Default` trait.
/// The `DefaultParty` variant is used when a specific party has not been set.
impl Default for MPCParty {
    fn default() -> Self {
        MPCParty::DefaultParty
    }
}

impl MPCParty {
    /// Advances the party to the next round by processing incoming messages and auxiliary input.
    ///
    /// # Arguments
    ///
    /// * `messages` - A hashmap of messages received from other parties, keyed by `PartyID`.
    /// * `auxiliary_input` - A serialized vector containing additional data for the protocol.
    ///
    /// # Returns
    ///
    /// * `Ok(AdvanceResult)` on success, which can either advance the party or finalize the protocol.
    /// * `Err(twopc_mpc::Error)` if an error occurs or if the `DefaultParty` variant is used.
    pub fn advance(
        self,
        messages: HashMap<PartyID, Vec<u8>>,
        auxiliary_input: Vec<u8>,
    ) -> PeraResult<AdvanceResult> {
        match self {
            MPCParty::FirstDKGBytesParty(party) => {
                let aux = bcs::from_bytes(&auxiliary_input)?;
                let a = advance::<DKGFirstParty>(party, messages, aux)?;
                match a {
                    mpc::AdvanceResult::Advance((message, new_party)) => Ok(AdvanceResult::Advance((
                        message,
                        MPCParty::FirstDKGBytesParty(new_party ),
                    ))),
                    mpc::AdvanceResult::Finalize(output) => {
                        Ok(AdvanceResult::Finalize(output))
                    }
                }
            }
            MPCParty::SecondDKGBytesParty(party) => {
                let aux = bcs::from_bytes(&auxiliary_input)?;
                let a = advance::<DKGSecondParty>(party, messages, aux)?;
                match a {
                    mpc::AdvanceResult::Advance((message, new_party)) => Ok(AdvanceResult::Advance((
                        message,
                        MPCParty::SecondDKGBytesParty( new_party ),
                    ))),
                    mpc::AdvanceResult::Finalize(output) => {
                        Ok(AdvanceResult::Finalize(output))
                    }
                }
            }
            MPCParty::FirstPresignBytesParty(party) => party.advance(messages, auxiliary_input),
            MPCParty::SecondPresignBytesParty(party) => party.advance(messages, auxiliary_input),
            MPCParty::DefaultParty => Err(PeraError::InternalDWalletMPCError),
        }
    }

    /// Parses an `Event` to extract the corresponding `MPCParty`, auxiliary input, and session information.
    ///
    /// # Arguments
    ///
    /// * `event` - The event data to parse.
    /// * `number_of_parties` - The total number of parties in the MPC session.
    /// * `party_id` - The unique identifier for the party.
    ///
    /// # Returns
    ///
    /// * `Ok(Some((MPCParty, Vec<u8>, SessionInfo)))` on success, containing the party, auxiliary input,
    ///   and session info required to begin an MPC round.
    /// * `Ok(None)` if the event type does not correspond to any known MPC rounds.
    /// * `Err(anyhow::Error)` if parsing fails or if an error occurs.
    pub fn from_event(
        event: &Event,
        number_of_parties: u16,
        party_id: PartyID,
    ) -> anyhow::Result<Option<(Self, Vec<u8>, SessionInfo)>> {
        if event.type_ == StartDKGFirstRoundEvent::type_() {
            let deserialized_event: StartDKGFirstRoundEvent = bcs::from_bytes(&event.contents)?;
            return Ok(Some((
                MPCParty::FirstDKGBytesParty(
                    <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareRoundParty::default()
                ),
                FirstDKGBytesParty::generate_auxiliary_input(number_of_parties, party_id, deserialized_event.session_id.bytes.to_vec()),
                SessionInfo {
                    session_id: deserialized_event.session_id.bytes,
                    initiating_user_address: deserialized_event.sender,
                    dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
                    mpc_round: MPCRound::DKGFirst,
                },
            )));
        } else if event.type_ == StartDKGSecondRoundEvent::type_() {
            let deserialized_event: StartDKGSecondRoundEvent = bcs::from_bytes(&event.contents)?;
            return Ok(Some((
                MPCParty::SecondDKGBytesParty(
                    <AsyncProtocol as twopc_mpc::dkg::Protocol>::ProofVerificationRoundParty::default()
                ),
                SecondDKGBytesParty::generate_auxiliary_input(
                    number_of_parties, party_id,
                    deserialized_event.first_round_output,
                    deserialized_event.public_key_share_and_proof,
                    deserialized_event.first_round_session_id.bytes.to_vec(),
                )?,
                SessionInfo {
                    session_id: ObjectID::from(deserialized_event.session_id),
                    initiating_user_address: deserialized_event.sender,
                    dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
                    mpc_round: MPCRound::DKGSecond,
                },
            )));
        } else if event.type_ == StartPresignFirstRoundEvent::type_() {
            let deserialized_event: StartPresignFirstRoundEvent = bcs::from_bytes(&event.contents)?;
            return Ok(Some((
                MPCParty::FirstPresignBytesParty(FirstPresignBytesParty {
                    party: PresignFirstParty::default(),
                }),
                FirstPresignBytesParty::generate_auxiliary_input(
                    deserialized_event.session_id.bytes.to_vec(),
                    number_of_parties,
                    party_id,
                    deserialized_event.dkg_output.clone(),
                ),
                SessionInfo {
                    session_id: deserialized_event.session_id.bytes,
                    initiating_user_address: deserialized_event.sender,
                    dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
                    mpc_round: MPCRound::PresignFirst(
                        deserialized_event.dwallet_id.bytes,
                        deserialized_event.dkg_output,
                    ),
                },
            )));
        } else if event.type_ == StartPresignSecondRoundEvent::type_() {
            let deserialized_event: StartPresignSecondRoundEvent =
                bcs::from_bytes(&event.contents)?;
            return Ok(Some((
                MPCParty::SecondPresignBytesParty(SecondPresignBytesParty {
                    party: PresignSecondParty::default(),
                }),
                SecondPresignBytesParty::generate_auxiliary_input(
                    deserialized_event.first_round_session_id.bytes.to_vec(),
                    number_of_parties,
                    party_id,
                    deserialized_event.dkg_output,
                    deserialized_event.first_round_output.clone(),
                ),
                SessionInfo {
                    session_id: deserialized_event.session_id.bytes,
                    initiating_user_address: deserialized_event.sender,
                    dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
                    mpc_round: MPCRound::PresignSecond(
                        deserialized_event.dwallet_id.bytes,
                        deserialized_event.first_round_output,
                    ),
                },
            )));
        }
        Ok(None)
    }
}

pub enum GeneralResult {
    /// Contains the message to send to other parties and the next `MPCParty` to use.
    Advance((Vec<u8>, )),
    /// Indicates that the protocol has completed, containing the final output.
    Finalize(Vec<u8>),
}

pub trait Test {
    type A;
}
pub struct TestStruct;
impl Test for TestStruct {
    type A = ();
}

fn advance<P: mpc::Advance>(
    party: P,
    messages: HashMap<PartyID, Vec<u8>>,
    auxiliary_input: P::AuxiliaryInput,
) ->  PeraResult<mpc::AdvanceResult<Vec<u8>, P, Vec<u8>>> {
    // let auxiliary_input =
    //     // This is not a validator malicious behaviour, as the authority input is being sent by the initiating user.
    //     // In this case this MPC session should be cancelled.
    //     bcs::from_bytes(&auxiliary_input).map_err(|_| PeraError::DWalletMPCInvalidUserInput)?;

     let res = party
        .advance(
            deserialize_mpc_messages(messages)?,
            &auxiliary_input,
            &mut rand_core::OsRng,
        ).map_err(|error| {
    let Ok(error): Result<mpc::Error, _> = error.try_into() else {
        return PeraError::InternalDWalletMPCError;
    };
    return match error {
        Error::UnresponsiveParties(parties)
        | Error::InvalidMessage(parties)
        | Error::MaliciousMessage(parties) => PeraError::DWalletMPCMaliciousParties(parties),
        _ => PeraError::InternalDWalletMPCError,
    };
    }
    )?;

    Ok(match res {
        mpc::AdvanceResult::Advance((msg, party)) => mpc::AdvanceResult::Advance((bcs::to_bytes(&msg)?, party)),
        mpc::AdvanceResult::Finalize(output) => {
            let output: P::OutputValue = output.into();
            mpc::AdvanceResult::Finalize(bcs::to_bytes(&output)?) },
    })

    // match result {
    //     mpc::AdvanceResult::Advance((message, new_party)) => Ok(AdvanceResult::Advance((
    //         bcs::to_bytes(&message).unwrap(),
    //         MPCParty::FirstDKGBytesParty(Self { party: new_party }),
    //     ))),
    //     mpc::AdvanceResult::Finalize(output) => {
    //         Ok(AdvanceResult::Finalize(bcs::to_bytes(&output).unwrap()))
    //     }
    // }
}