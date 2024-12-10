use crate::dwallet_mpc::dkg::{
    DKGFirstParty, DKGFirstPartyPublicInputGenerator, DKGSecondParty,
    DKGSecondPartyPublicInputGenerator,
};
use crate::dwallet_mpc::mpc_events::{
    StartBatchedSignEvent, StartDKGFirstRoundEvent, StartDKGSecondRoundEvent,
    StartPresignFirstRoundEvent, StartPresignSecondRoundEvent, StartSignRoundEvent,
};
use crate::dwallet_mpc::mpc_manager::DWalletMPCManager;
use crate::dwallet_mpc::network_dkg::{KeyTypes, NetworkDkg};
use crate::dwallet_mpc::presign::{
    PresignFirstParty, PresignFirstPartyPublicInputGenerator, PresignSecondParty,
    PresignSecondPartyPublicInputGenerator,
};
use crate::dwallet_mpc::sign::{SignFirstParty, SignPartyPublicInputGenerator};
use anyhow::Error;
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{MPCMessage, MPCPublicInput};
use group::PartyID;
use mpc::{AsynchronouslyAdvanceable, WeightedThresholdAccessStructure};
use pera_types::base_types::ObjectID;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::error::{PeraError, PeraResult};
use pera_types::event::Event;
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub(super) type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;

/// Enum representing the different parties used in the MPC manager.
#[derive(Clone)]
pub enum MPCParty {
    /// The party used in the first round of the DKG protocol.
    FirstDKGBytesParty,
    /// The party used in the second round of the DKG protocol.
    SecondDKGBytesParty,
    /// The party used in the first round of the presign protocol.
    FirstPresignBytesParty,
    /// The party used in the second round of the presign protocol.
    SecondPresignBytesParty,
    /// The party used in the sign protocol.
    SignBytesParty(HashMap<PartyID, twopc_mpc::secp256k1::class_groups::DecryptionKeyShare>),
    /// The party used in the network DKG protocol.
    NetworkDkg(KeyTypes),
}

impl MPCParty {
    /// Advances the party to the next round by processing incoming messages and public input.
    /// Returns the next [`MPCParty`] to use, or the final output if the protocol has completed.
    pub fn advance(
        &self,
        messages: Vec<HashMap<PartyID, MPCMessage>>,
        session_id: ObjectID,
        party_id: PartyID,
        access_threshold: &WeightedThresholdAccessStructure,
        public_input: MPCPublicInput,
    ) -> DwalletMPCResult<mpc::AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>> {
        let session_id = CommitmentSizedNumber::from_le_slice(session_id.to_vec().as_slice());
        match &self {
            MPCParty::FirstDKGBytesParty => {
                let public_input = bcs::from_bytes(&public_input)?;
                advance::<DKGFirstParty>(
                    session_id,
                    party_id,
                    access_threshold,
                    messages,
                    public_input,
                    (),
                )
            }
            MPCParty::SecondDKGBytesParty => {
                let public_input = bcs::from_bytes(&public_input)?;
                advance::<DKGSecondParty>(
                    session_id,
                    party_id,
                    access_threshold,
                    messages,
                    public_input,
                    (),
                )
            }
            MPCParty::FirstPresignBytesParty => {
                let public_input = bcs::from_bytes(&public_input)?;
                advance::<PresignFirstParty>(
                    session_id,
                    party_id,
                    access_threshold,
                    messages,
                    public_input,
                    (),
                )
            }
            MPCParty::SecondPresignBytesParty => {
                let public_input = bcs::from_bytes(&public_input)?;
                advance::<PresignSecondParty>(
                    session_id,
                    party_id,
                    access_threshold,
                    messages,
                    public_input,
                    (),
                )
            }
            MPCParty::SignBytesParty(decryption_key_share) => {
                let public_input = bcs::from_bytes(&public_input)?;
                advance::<SignFirstParty>(
                    session_id,
                    party_id,
                    access_threshold,
                    messages,
                    public_input,
                    decryption_key_share.clone(),
                )
            }
            MPCParty::NetworkDkg(key_type) => NetworkDkg::advance(
                access_threshold,
                party_id,
                &public_input,
                key_type,
                messages,
            ),
        }
    }

    /// Parses the session info from the event and returns it.
    /// Return `None` if the event is not a DWallet MPC event.
    pub fn session_info_from_event(
        event: &Event,
        party_id: PartyID,
    ) -> anyhow::Result<Option<SessionInfo>> {
        match &event.type_ {
            t if t == &StartDKGFirstRoundEvent::type_() => {
                let deserialized_event: StartDKGFirstRoundEvent = bcs::from_bytes(&event.contents)?;
                Ok(Some(Self::dkg_first_party_session_info(deserialized_event)))
            }
            t if t == &StartDKGSecondRoundEvent::type_() => {
                let deserialized_event: StartDKGSecondRoundEvent =
                    bcs::from_bytes(&event.contents)?;
                Ok(Some(Self::dkg_second_party_session_info(
                    &deserialized_event,
                )))
            }
            t if t == &StartPresignFirstRoundEvent::type_() => {
                let deserialized_event: StartPresignFirstRoundEvent =
                    bcs::from_bytes(&event.contents)?;
                Ok(Some(Self::presign_first_party_session_info(
                    deserialized_event,
                )))
            }
            t if t == &StartPresignSecondRoundEvent::type_() => {
                let deserialized_event: StartPresignSecondRoundEvent =
                    bcs::from_bytes(&event.contents)?;
                Ok(Some(Self::presign_second_party_session_info(
                    &deserialized_event,
                )))
            }
            t if t == &StartSignRoundEvent::type_() => {
                let deserialized_event: StartSignRoundEvent = bcs::from_bytes(&event.contents)
                    .map_err(|_| PeraError::DWalletMPCInvalidUserInput)?;
                Ok(Some(Self::sign_party_session_info(
                    &deserialized_event,
                    party_id,
                )))
            }
            t if t == &StartBatchedSignEvent::type_() => {
                let deserialized_event: StartBatchedSignEvent = bcs::from_bytes(&event.contents)
                    .map_err(|_| PeraError::DWalletMPCInvalidUserInput)?;
                Ok(Some(Self::batched_sign_session_info(&deserialized_event)))
            }
            _ => Ok(None),
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
    ) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
        match &event.type_ {
            t if t == &StartDKGFirstRoundEvent::type_() => {
                let deserialized_event: StartDKGFirstRoundEvent = bcs::from_bytes(&event.contents)?;
                Self::dkg_first_party(deserialized_event)
            }
            t if t == &StartDKGSecondRoundEvent::type_() => {
                let deserialized_event: StartDKGSecondRoundEvent =
                    bcs::from_bytes(&event.contents)?;
                Self::dkg_second_party(deserialized_event)
            }
            t if t == &StartPresignFirstRoundEvent::type_() => {
                let deserialized_event: StartPresignFirstRoundEvent =
                    bcs::from_bytes(&event.contents)?;
                Self::presign_first_party(deserialized_event)
            }
            t if t == &StartPresignSecondRoundEvent::type_() => {
                let deserialized_event: StartPresignSecondRoundEvent =
                    bcs::from_bytes(&event.contents)?;
                Self::presign_second_party(deserialized_event)
            }
            t if t == &StartSignRoundEvent::type_() => {
                let deserialized_event: StartSignRoundEvent = bcs::from_bytes(&event.contents)?;
                Self::sign_party(party_id, deserialized_event, dwallet_mpc_manager)
            }
            _ => Err(DwalletMPCError::NonMPCEvent.into()),
        }
    }

    fn dkg_second_party(
        deserialized_event: StartDKGSecondRoundEvent,
    ) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
        Ok((
            MPCParty::SecondDKGBytesParty,
            DKGSecondParty::generate_public_input(
                deserialized_event.first_round_output.clone(),
                deserialized_event.public_key_share_and_proof.clone(),
            )?,
            Self::dkg_second_party_session_info(&deserialized_event),
        ))
    }

    fn dkg_second_party_session_info(deserialized_event: &StartDKGSecondRoundEvent) -> SessionInfo {
        SessionInfo {
            flow_session_id: deserialized_event.first_round_session_id.bytes,
            session_id: ObjectID::from(deserialized_event.session_id),
            initiating_user_address: deserialized_event.initiator,
            dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
            mpc_round: MPCRound::DKGSecond,
        }
    }

    fn dkg_first_party(
        deserialized_event: StartDKGFirstRoundEvent,
    ) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
        Ok((
            MPCParty::FirstDKGBytesParty,
            <DKGFirstParty as DKGFirstPartyPublicInputGenerator>::generate_public_input()?,
            Self::dkg_first_party_session_info(deserialized_event),
        ))
    }

    fn dkg_first_party_session_info(deserialized_event: StartDKGFirstRoundEvent) -> SessionInfo {
        SessionInfo {
            flow_session_id: deserialized_event.session_id.bytes,
            session_id: deserialized_event.session_id.bytes,
            initiating_user_address: deserialized_event.initiator,
            dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
            mpc_round: MPCRound::DKGFirst,
        }
    }

    fn presign_first_party(
        deserialized_event: StartPresignFirstRoundEvent,
    ) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
        Ok((
            MPCParty::FirstPresignBytesParty,
            <PresignFirstParty as PresignFirstPartyPublicInputGenerator>::generate_public_input(
                deserialized_event.dkg_output.clone(),
            )?,
            Self::presign_first_party_session_info(deserialized_event),
        ))
    }

    fn presign_first_party_session_info(
        deserialized_event: StartPresignFirstRoundEvent,
    ) -> SessionInfo {
        SessionInfo {
            flow_session_id: deserialized_event.session_id.bytes,
            session_id: deserialized_event.session_id.bytes,
            initiating_user_address: deserialized_event.sender,
            dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
            mpc_round: MPCRound::PresignFirst(
                deserialized_event.dwallet_id.bytes,
                deserialized_event.dkg_output,
            ),
        }
    }

    fn presign_second_party(
        deserialized_event: StartPresignSecondRoundEvent,
    ) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
        Ok((
            MPCParty::SecondPresignBytesParty,
            <PresignSecondParty as PresignSecondPartyPublicInputGenerator>::generate_public_input(
                deserialized_event.dkg_output.clone(),
                deserialized_event.first_round_output.clone(),
            )?,
            Self::presign_second_party_session_info(&deserialized_event),
        ))
    }

    fn presign_second_party_session_info(
        deserialized_event: &StartPresignSecondRoundEvent,
    ) -> SessionInfo {
        SessionInfo {
            flow_session_id: deserialized_event.first_round_session_id.bytes,
            session_id: deserialized_event.session_id.bytes,
            initiating_user_address: deserialized_event.sender,
            dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
            mpc_round: MPCRound::PresignSecond(
                deserialized_event.dwallet_id.bytes,
                deserialized_event.first_round_output.clone(),
            ),
        }
    }

    fn sign_party(
        party_id: PartyID,
        deserialized_event: StartSignRoundEvent,
        dwallet_mpc_manager: &DWalletMPCManager,
    ) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
        let decryption_key_share = dwallet_mpc_manager.get_decryption_share()?;
        Ok((
            MPCParty::SignBytesParty(HashMap::from([(party_id, decryption_key_share)])),
            <SignFirstParty as SignPartyPublicInputGenerator>::generate_public_input(
                deserialized_event.dkg_output.clone(),
                deserialized_event.hashed_message.clone(),
                deserialized_event.presign_first_round_output.clone(),
                deserialized_event.presign_second_round_output.clone(),
                deserialized_event.centralized_signed_message.clone(),
                dwallet_mpc_manager
                    .node_config
                    .dwallet_mpc_decryption_shares_public_parameters
                    .clone()
                    .ok_or_else(|| {
                        DwalletMPCError::MissingDwalletMPCDecryptionSharesPublicParameters
                    })?,
            )?,
            Self::sign_party_session_info(&deserialized_event, party_id),
        ))
    }

    fn sign_party_session_info(
        deserialized_event: &StartSignRoundEvent,
        party_id: PartyID,
    ) -> SessionInfo {
        SessionInfo {
            flow_session_id: deserialized_event.presign_session_id.bytes,
            session_id: deserialized_event.session_id.bytes,
            initiating_user_address: deserialized_event.sender,
            dwallet_cap_id: deserialized_event.dwallet_cap_id.bytes,
            mpc_round: MPCRound::Sign(
                deserialized_event.batched_session_id.bytes,
                deserialized_event.hashed_message.clone(),
            ),
        }
    }

    fn batched_sign_session_info(deserialized_event: &StartBatchedSignEvent) -> SessionInfo {
        SessionInfo {
            flow_session_id: deserialized_event.session_id.bytes,
            session_id: deserialized_event.session_id.bytes,
            initiating_user_address: deserialized_event.initiating_user,
            // Dummy ID is the dwallet cap is not relevant in the batched sign flow.
            // TODO (#365): Remove the DWallet cap from the session info
            dwallet_cap_id: deserialized_event.session_id.bytes,
            mpc_round: MPCRound::BatchedSign(deserialized_event.hashed_messages.clone()),
        }
    }
}

pub(in crate::dwallet_mpc) fn advance<P: AsynchronouslyAdvanceable>(
    session_id: CommitmentSizedNumber,
    party_id: PartyID,
    access_threshold: &WeightedThresholdAccessStructure,
    messages: Vec<HashMap<PartyID, MPCMessage>>,
    public_input: P::PublicInput,
    private_input: P::PrivateInput,
) -> DwalletMPCResult<mpc::AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>> {
    let messages = deserialize_mpc_messages(messages)?;

    let res = P::advance(
        session_id,
        party_id,
        access_threshold,
        messages,
        Some(private_input),
        &public_input,
        &mut rand_core::OsRng,
    )
    .map_err(|e| DwalletMPCError::TwoPCMPCError(format!("{:?}", e)))?;

    Ok(match res {
        mpc::AsynchronousRoundResult::Advance {
            malicious_parties,
            message,
        } => mpc::AsynchronousRoundResult::Advance {
            malicious_parties,
            message: bcs::to_bytes(&message)?,
        },
        mpc::AsynchronousRoundResult::Finalize {
            malicious_parties,
            private_output: _,
            public_output,
        } => {
            let public_output: P::PublicOutputValue = public_output.into();
            let public_output = bcs::to_bytes(&public_output)?;
            mpc::AsynchronousRoundResult::Finalize {
                malicious_parties,
                private_output: Vec::new(),
                public_output,
            }
        }
    })
}

/// Deserializes the messages received from other parties for the next advancement.
/// Any value that fails to deserialize is considered to be sent by a malicious party.
/// Returns the deserialized messages or an error including the IDs of the malicious parties.
fn deserialize_mpc_messages<M: DeserializeOwned + Clone>(
    messages: Vec<HashMap<PartyID, MPCMessage>>,
) -> DwalletMPCResult<Vec<HashMap<PartyID, M>>> {
    let mut deserialized_results = Vec::new();
    let mut malicious_parties = Vec::new();

    for message_batch in &messages {
        let mut valid_messages = HashMap::new();

        for (party_id, message) in message_batch {
            match bcs::from_bytes::<M>(&message) {
                Ok(value) => {
                    valid_messages.insert(*party_id, value);
                }
                Err(_) => {
                    malicious_parties.push(*party_id);
                }
            }
        }

        if !valid_messages.is_empty() {
            deserialized_results.push(valid_messages);
        }
    }

    if !malicious_parties.is_empty() {
        Err(DwalletMPCError::MaliciousParties(malicious_parties))
    } else {
        Ok(deserialized_results)
    }
}
