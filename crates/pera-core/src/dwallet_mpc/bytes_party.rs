//! The `bytes_party` module defines the API for managing MPC parties within the MPC manager.
//! This module wraps the various MPC parties, providing an interface
//! to progress each party through the rounds of the MPC protocol as needed.
//!
//! The [`BytesParty`] trait enables the MPC manager to seamlessly advance the [`MPCParty`]
//! instances to the next round.
use crate::dwallet_mpc::dkg::{
    advance, DKGFirstParty, DKGFirstPartyAuxiliaryInputGenerator, DKGSecondParty,
    DKGSecondPartyAuxiliaryInputGenerator,
};
use crate::dwallet_mpc::mpc_events::{StartDKGFirstRoundEvent, StartDKGSecondRoundEvent, StartPresignFirstRoundEvent, StartPresignSecondRoundEvent, StartSignRoundEvent};
use anyhow::Error;
use group::PartyID;
use pera_types::base_types::ObjectID;
use pera_types::error::{PeraError, PeraResult};
use pera_types::event::Event;
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use std::collections::HashMap;
use mpc::{AsynchronousOutput, WeightedThresholdAccessStructure};
use twopc_mpc::class_groups::{EncryptionOfSecretKeyShareRoundParty, ProofVerificationRoundParty};
use crate::dwallet_mpc::mpc_manager::DWalletMPCManager;
use crate::dwallet_mpc::presign::{PresignFirstParty, PresignFirstRound, PresignSecondParty, PresignSecondRound};
use crate::dwallet_mpc::sign::SignFirstParty;

/// Represents the outcome of advancing an MPC party to the next round.
///
/// This enum indicates whether the party should advance to the next round or
/// finalize its protocol execution.
pub enum AdvanceResult {
    /// Contains the message to send to other parties, and the next [`MPCParty`] to use.
    Advance((Vec<u8>, MPCParty)),
    /// Indicates that the protocol has completed, containing the final output.
    Finalize(Vec<u8>),
    FinalizeAsync(AsynchronousOutput<Vec<u8>>),
}

/// Enum representing the different parties used in the MPC manager.
pub enum MPCParty {
    /// A placeholder party used as a default.
    /// Does not implement the `BytesParty` trait and should never be used.
    DefaultParty,
    /// The party used in the first round of the DKG protocol.
    FirstDKGBytesParty(DKGFirstParty),
    /// The party used in the second round of the DKG protocol.
    SecondDKGBytesParty(DKGSecondParty),
    /// The party used in the first round of the presign protocol.
    FirstPresignBytesParty(PresignFirstParty),
    /// The party used in the second round of the presign protocol.
    SecondPresignBytesParty(PresignSecondParty),
    /// The party used in the sign protocol.
    SignBytesParty(SignFirstParty),
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

// todo(zeev): replace all errors with DWalletMPCError, anyhow and from...
impl MPCParty {
    /// Advances the party to the next round by processing incoming messages and auxiliary input.
    /// Returns the next [`MPCParty`] to use, or the final output if the protocol has completed.
    pub fn advance(
        self,
        messages: HashMap<PartyID, Vec<u8>>,
        auxiliary_input: &[u8],
    ) -> PeraResult<AdvanceResult> {
        match self {
            MPCParty::FirstDKGBytesParty(party) => {
                let aux = bcs::from_bytes(&auxiliary_input)?;
                let a = advance::<DKGFirstParty>(party, messages, aux)?;
                match a {
                    mpc::AdvanceResult::Advance((message, new_party)) => Ok(
                        AdvanceResult::Advance((message, MPCParty::FirstDKGBytesParty(new_party))),
                    ),
                    mpc::AdvanceResult::Finalize(output) => Ok(AdvanceResult::Finalize(output)),
                    mpc::AdvanceResult::FinalizeAsync(output) => Ok(AdvanceResult::FinalizeAsync(AsynchronousOutput {
                        malicious_parties: output.malicious_parties,
                        output: output.output,
                    })),
                }
            }
            MPCParty::SecondDKGBytesParty(party) => {
                let aux = bcs::from_bytes(&auxiliary_input)?;
                let a = advance::<DKGSecondParty>(party, messages, aux)?;
                match a {
                    mpc::AdvanceResult::Advance((message, new_party)) => Ok(
                        AdvanceResult::Advance((message, MPCParty::SecondDKGBytesParty(new_party))),
                    ),
                    mpc::AdvanceResult::Finalize(output) => Ok(AdvanceResult::Finalize(output)),
                    mpc::AdvanceResult::FinalizeAsync(output) => Ok(AdvanceResult::FinalizeAsync(AsynchronousOutput {
                        malicious_parties: output.malicious_parties,
                        output: output.output,
                    })),
                }
            }
            MPCParty::FirstPresignBytesParty(party) => {
                let aux = bcs::from_bytes(&auxiliary_input)?;
                let a = advance::<PresignFirstParty>(party, messages, aux)?;
                match a {
                    mpc::AdvanceResult::Advance((message, new_party)) => Ok(
                        AdvanceResult::Advance((message, MPCParty::FirstPresignBytesParty(new_party))),
                    ),
                    mpc::AdvanceResult::Finalize(output) => Ok(AdvanceResult::Finalize(output)),
                    mpc::AdvanceResult::FinalizeAsync(output) => Ok(AdvanceResult::FinalizeAsync(AsynchronousOutput {
                        malicious_parties: output.malicious_parties,
                        output: output.output,
                    })),
                }
            }
            MPCParty::SecondPresignBytesParty(party) => {
                let aux = bcs::from_bytes(&auxiliary_input)?;
                let a = advance::<PresignSecondParty>(party, messages, aux)?;
                match a {
                    mpc::AdvanceResult::Advance((message, new_party)) => Ok(
                        AdvanceResult::Advance((message, MPCParty::SecondPresignBytesParty(new_party))),
                    ),
                    mpc::AdvanceResult::Finalize(output) => Ok(AdvanceResult::Finalize(output)),
                    mpc::AdvanceResult::FinalizeAsync(output) => Ok(AdvanceResult::FinalizeAsync(AsynchronousOutput {
                        malicious_parties: output.malicious_parties,
                        output: output.output,
                    })),
                }
            }
            MPCParty::SignBytesParty(party) => {
                let aux = bcs::from_bytes(&auxiliary_input)?;
                let a = advance::<SignFirstParty>(party, messages, aux)?;
                match a {
                    mpc::AdvanceResult::Advance((message, new_party)) => Ok(
                        AdvanceResult::Advance((message, MPCParty::SignBytesParty(new_party))),
                    ),
                    mpc::AdvanceResult::Finalize(output) => Ok(AdvanceResult::Finalize(output)),
                    mpc::AdvanceResult::FinalizeAsync(output) => Ok(AdvanceResult::FinalizeAsync(AsynchronousOutput {
                        malicious_parties: output.malicious_parties,
                        output: output.output,
                    })),
                }
            }
            MPCParty::DefaultParty => Err(PeraError::InternalDWalletMPCError),
        }
    }

    /// Parses an [`Event`] to extract the corresponding [`MPCParty`],
    /// auxiliary input, and session information.
    ///
    /// Returns an error if the event type does not correspond to any known MPC rounds
    /// or if deserialization fails.
    pub fn from_event(
        event: &Event,
        dwallet_mpc_manager: &DWalletMPCManager,
        party_id: PartyID,
    ) -> anyhow::Result<(MPCParty, Vec<u8>, SessionInfo)> {
        let weighted_threshold_access_structure = dwallet_mpc_manager
            .weighted_threshold_access_structure
            .clone();
        match &event.type_ {
            t if t == &StartDKGFirstRoundEvent::type_() => {
                let deserialized_event: StartDKGFirstRoundEvent = bcs::from_bytes(&event.contents)?;
                Self::dkg_first_party(weighted_threshold_access_structure, party_id, deserialized_event)
            }
            t if t == &StartDKGSecondRoundEvent::type_() => {
                let deserialized_event: StartDKGSecondRoundEvent =
                    bcs::from_bytes(&event.contents)?;
                Self::dkg_second_party(weighted_threshold_access_structure, party_id, deserialized_event)
            }
            t if t == &StartPresignFirstRoundEvent::type_() => {
                let deserialized_event: StartPresignFirstRoundEvent = bcs::from_bytes(&event.contents)?;
                Self::presign_first_party(
                    weighted_threshold_access_structure,
                    party_id,
                    deserialized_event,
                )
            }
            t if t == &StartPresignSecondRoundEvent::type_() => {
                let deserialized_event: StartPresignSecondRoundEvent =
                    bcs::from_bytes(&event.contents)?;
                Self::presign_second_party(
                    weighted_threshold_access_structure,
                    party_id,
                    deserialized_event,
                )
            }
            t if t == &StartSignRoundEvent::type_() => {
                let deserialized_event: StartSignRoundEvent = bcs::from_bytes(&event.contents)
                    .map_err(|_| PeraError::DWalletMPCInvalidUserInput)?;
                Self::sign_party(
                    weighted_threshold_access_structure,
                    party_id,
                    deserialized_event,
                    dwallet_mpc_manager,
                )
            }
            _ => Err(PeraError::NonMPCEvent.into()),
        }
    }

    fn dkg_second_party(
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        party_id: PartyID,
        deserialized_event: StartDKGSecondRoundEvent,
    ) -> Result<(MPCParty, Vec<u8>, SessionInfo), Error> {
        Ok((
            MPCParty::SecondDKGBytesParty(DKGSecondParty::default()),
            <DKGSecondParty as DKGSecondPartyAuxiliaryInputGenerator>::generate_auxiliary_input(
                weighted_threshold_access_structure,
                party_id,
                deserialized_event.first_round_output,
                deserialized_event.public_key_share_and_proof,
                deserialized_event.first_round_session_id.bytes.to_vec(),
            ),
            SessionInfo {
                session_id: ObjectID::from(deserialized_event.session_id),
                initiating_user_address: deserialized_event.sender,
                dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
                mpc_round: MPCRound::DKGSecond,
            },
        ))
    }

    fn dkg_first_party(
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        party_id: PartyID,
        deserialized_event: StartDKGFirstRoundEvent,
    ) -> Result<(MPCParty, Vec<u8>, SessionInfo), Error> {
        Ok((
            MPCParty::FirstDKGBytesParty(DKGFirstParty::default()),
            DKGFirstParty::generate_auxiliary_input(
                deserialized_event.session_id.bytes.to_vec(),
                weighted_threshold_access_structure,
                party_id,
            ),
            SessionInfo {
                session_id: deserialized_event.session_id.bytes,
                initiating_user_address: deserialized_event.sender,
                dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
                mpc_round: MPCRound::DKGFirst,
            },
        ))
    }

    fn presign_first_party(
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        party_id: PartyID,
        deserialized_event: StartPresignFirstRoundEvent,
    ) -> Result<(MPCParty, Vec<u8>, SessionInfo), Error> {
        Ok((MPCParty::FirstPresignBytesParty(PresignFirstParty::default()),
            PresignFirstParty::generate_auxiliary_input(
                deserialized_event.session_id.bytes.to_vec(),
                weighted_threshold_access_structure,
                party_id,
                deserialized_event.dkg_output.clone(),
            )?,
            SessionInfo {
                session_id: deserialized_event.session_id.bytes,
                initiating_user_address: deserialized_event.sender,
                dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
                mpc_round: MPCRound::PresignFirst(
                    deserialized_event.dwallet_id.bytes,
                    deserialized_event.dkg_output,
                ),
            },))
    }

    fn presign_second_party(
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        party_id: PartyID,
        deserialized_event: StartPresignSecondRoundEvent,
    ) -> Result<(MPCParty, Vec<u8>, SessionInfo), Error> {
        Ok((MPCParty::SecondPresignBytesParty(PresignSecondParty::default()),
            PresignSecondParty::generate_auxiliary_input(
                deserialized_event.first_round_session_id.bytes.to_vec(),
                weighted_threshold_access_structure,
                party_id,
                deserialized_event.dkg_output,
                deserialized_event.first_round_output.clone(),
            )?,
            SessionInfo {
                session_id: deserialized_event.session_id.bytes,
                initiating_user_address: deserialized_event.sender,
                dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
                mpc_round: MPCRound::PresignSecond(
                    deserialized_event.dwallet_id.bytes,
                    deserialized_event.first_round_output,
                ),
            },
        ))
    }

    fn sign_party(
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        party_id: PartyID,
        deserialized_event: StartSignRoundEvent,
        dwallet_mpc_manager: &DWalletMPCManager,
    ) -> Result<(MPCParty, Vec<u8>, SessionInfo), Error> {
        let decryption_key_share = dwallet_mpc_manager.get_decryption_share() ?;
        let party = SignFirstParty::from(HashMap::from([(party_id, decryption_key_share)]));
        Ok((
            MPCParty::SignBytesParty(party),
            crate::dwallet_mpc::sign::generate_auxiliary_input(
                deserialized_event.presign_session_id.bytes.to_vec(),
                weighted_threshold_access_structure,
                party_id,
                deserialized_event.dkg_output,
                deserialized_event.hashed_message.clone(),
                deserialized_event.presign.clone(),
                deserialized_event.centralized_signed_message.clone(),
                dwallet_mpc_manager
                    .node_config
                    .dwallet_mpc_decryption_shares_public_parameters
                    .clone()
                    .ok_or_else(| | PeraError::InternalDWalletMPCError) ?,
            ) ?,
            SessionInfo {
                session_id: deserialized_event.session_id.bytes,
                initiating_user_address: deserialized_event.sender,
                dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
                mpc_round: MPCRound::Sign(party_id),
            },
        ))
    }
}
