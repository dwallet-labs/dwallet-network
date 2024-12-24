use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::dkg::{
    DKGFirstParty, DKGFirstPartyPublicInputGenerator, DKGSecondParty,
    DKGSecondPartyPublicInputGenerator,
};
use crate::dwallet_mpc::mpc_events::{
    StartBatchedPresignEvent, StartBatchedSignEvent, StartDKGFirstRoundEvent,
    StartDKGSecondRoundEvent, StartNetworkDKGEvent, StartPresignFirstRoundEvent,
    StartPresignSecondRoundEvent, StartSignRoundEvent,
};
use crate::dwallet_mpc::mpc_manager::DWalletMPCManager;
use crate::dwallet_mpc::mpc_party::{AsyncProtocol, MPCParty};
use crate::dwallet_mpc::presign::{
    PresignFirstParty, PresignFirstPartyPublicInputGenerator, PresignSecondParty,
    PresignSecondPartyPublicInputGenerator,
};
use crate::dwallet_mpc::sign::{SignFirstParty, SignPartyPublicInputGenerator};
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::MPCMessage;
use group::PartyID;
use mpc::{AsynchronouslyAdvanceable, WeightedThresholdAccessStructure};
use pera_types::base_types::AuthorityName;
use pera_types::base_types::{EpochId, ObjectID};
use pera_types::dwallet_mpc::DWalletMPCNetworkKeyScheme;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::event::Event;
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use twopc_mpc::sign::Protocol;

pub mod batches_manager;
mod dkg;
pub(crate) mod mpc_events;
pub mod mpc_instance;
pub mod mpc_manager;
pub mod mpc_outputs_verifier;
pub(crate) mod mpc_party;
pub mod network_dkg;
mod presign;
pub(crate) mod sign;
pub mod dwallet_network_mpc_keys;

pub const FIRST_EPOCH_ID: EpochId = 0;

/// The message a Validator can send to the other parties while
/// running a dWallet MPC session.
#[derive(Clone)]
pub struct DWalletMPCMessage {
    /// The serialized message.
    pub(crate) message: MPCMessage,
    /// The authority (Validator) that sent the message.
    pub(crate) authority: AuthorityName,
}

/// Convert a given authority name (address) to it's corresponding [`PartyID`].
/// The [`PartyID`] is the index of the authority in the committee.
pub(crate) fn authority_name_to_party_id(
    authority_name: &AuthorityName,
    epoch_store: &AuthorityPerEpochStore,
) -> DwalletMPCResult<PartyID> {
    epoch_store
        .committee()
        .authority_index(authority_name)
        // Need to add 1 because the authority index is 0-based,
        // and the twopc_mpc library uses 1-based party IDs.
        .map(|index| (index + 1) as PartyID)
        .ok_or_else(|| DwalletMPCError::AuthorityNameNotFound(*authority_name))
}

/// Parses the session info from the event and returns it.
/// Return `None` if the event is not a DWallet MPC event.
pub(crate) fn session_info_from_event(
    event: &Event,
    party_id: PartyID,
    dwallet_network_key_version: Option<u8>,
) -> anyhow::Result<Option<SessionInfo>> {
    match &event.type_ {
        t if t == &StartDKGFirstRoundEvent::type_() => {
            let deserialized_event: StartDKGFirstRoundEvent = bcs::from_bytes(&event.contents)?;
            Ok(Some(dkg_first_party_session_info(deserialized_event)))
        }
        t if t == &StartDKGSecondRoundEvent::type_() => {
            let deserialized_event: StartDKGSecondRoundEvent = bcs::from_bytes(&event.contents)?;
            Ok(Some(dkg_second_party_session_info(
                &deserialized_event,
                if cfg!(feature = "with-network-dkg") {
                    dwallet_network_key_version.ok_or(DwalletMPCError::MissingKeyVersion)?
                } else {
                    0
                },
            )))
        }
        t if t == &StartPresignFirstRoundEvent::type_() => {
            let deserialized_event: StartPresignFirstRoundEvent = bcs::from_bytes(&event.contents)?;
            Ok(Some(presign_first_party_session_info(deserialized_event)))
        }
        t if t == &StartPresignSecondRoundEvent::type_() => {
            let deserialized_event: StartPresignSecondRoundEvent =
                bcs::from_bytes(&event.contents)?;
            Ok(Some(presign_second_party_session_info(&deserialized_event)))
        }
        t if t == &StartSignRoundEvent::type_() => {
            let deserialized_event: StartSignRoundEvent = bcs::from_bytes(&event.contents)?;
            Ok(Some(sign_party_session_info(&deserialized_event, party_id)))
        }
        t if t == &StartBatchedSignEvent::type_() => {
            let deserialized_event: StartBatchedSignEvent = bcs::from_bytes(&event.contents)?;
            Ok(Some(batched_sign_session_info(&deserialized_event)))
        }
        t if t == &StartBatchedPresignEvent::type_() => {
            let deserialized_event: StartBatchedPresignEvent = bcs::from_bytes(&event.contents)?;
            Ok(Some(batched_presign_session_info(&deserialized_event)))
        }
        t if t == &StartNetworkDKGEvent::type_() => {
            let deserialized_event: StartNetworkDKGEvent = bcs::from_bytes(&event.contents)?;
            Ok(Some(network_dkg::network_dkg_session_info(
                deserialized_event,
            )?))
        }
        _ => Ok(None),
    }
}

fn dkg_second_party(
    deserialized_event: StartDKGSecondRoundEvent,
    dwallet_network_key_version: u8,
) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
    Ok((
        MPCParty::SecondDKGBytesParty,
        DKGSecondParty::generate_public_input(
            deserialized_event.first_round_output.clone(),
            deserialized_event.public_key_share_and_proof.clone(),
        )?,
        dkg_second_party_session_info(&deserialized_event, dwallet_network_key_version),
    ))
}

fn dkg_second_party_session_info(
    deserialized_event: &StartDKGSecondRoundEvent,
    dwallet_network_key_version: u8,
) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.first_round_session_id.bytes,
        session_id: ObjectID::from(deserialized_event.session_id),
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCRound::DKGSecond(
            deserialized_event.dwallet_cap_id.bytes,
            dwallet_network_key_version,
        ),
    }
}

fn dkg_first_party(
    deserialized_event: StartDKGFirstRoundEvent,
) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
    Ok((
        MPCParty::FirstDKGBytesParty,
        <DKGFirstParty as DKGFirstPartyPublicInputGenerator>::generate_public_input()?,
        dkg_first_party_session_info(deserialized_event),
    ))
}

fn dkg_first_party_session_info(deserialized_event: StartDKGFirstRoundEvent) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: deserialized_event.initiator,
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
        presign_first_party_session_info(deserialized_event),
    ))
}

fn presign_first_party_session_info(
    deserialized_event: StartPresignFirstRoundEvent,
) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCRound::PresignFirst(
            deserialized_event.dwallet_id.bytes,
            deserialized_event.dkg_output,
            deserialized_event.batch_session_id.bytes,
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
        presign_second_party_session_info(&deserialized_event),
    ))
}

fn presign_second_party_session_info(
    deserialized_event: &StartPresignSecondRoundEvent,
) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.first_round_session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCRound::PresignSecond(
            deserialized_event.dwallet_id.bytes,
            deserialized_event.first_round_output.clone(),
            deserialized_event.batch_session_id.bytes,
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
        MPCParty::SignBytesParty(decryption_key_share),
        <SignFirstParty as SignPartyPublicInputGenerator>::generate_public_input(
            deserialized_event.dkg_output.clone(),
            deserialized_event.hashed_message.clone(),
            deserialized_event.presign.clone(),
            deserialized_event.centralized_signed_message.clone(),
            dwallet_mpc_manager
                .node_config
                .dwallet_mpc_decryption_shares_public_parameters
                .clone()
                .ok_or_else(|| {
                    DwalletMPCError::MissingDwalletMPCDecryptionSharesPublicParameters
                })?,
        )?,
        sign_party_session_info(&deserialized_event, party_id),
    ))
}

fn sign_party_session_info(
    deserialized_event: &StartSignRoundEvent,
    party_id: PartyID,
) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.presign_session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: deserialized_event.initiator,
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
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCRound::BatchedSign(deserialized_event.hashed_messages.clone()),
    }
}

fn batched_presign_session_info(deserialized_event: &StartBatchedPresignEvent) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCRound::BatchedPresign(deserialized_event.batch_size),
    }
}

pub(crate) fn advance<P: AsynchronouslyAdvanceable>(
    session_id: CommitmentSizedNumber,
    party_id: PartyID,
    access_threshold: &WeightedThresholdAccessStructure,
    messages: Vec<HashMap<PartyID, MPCMessage>>,
    public_input: P::PublicInput,
    private_input: P::PrivateInput,
) -> DwalletMPCResult<mpc::AsynchronousRoundResult<Vec<u8>, P::PrivateOutput, Vec<u8>>> {
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
            private_output,
            public_output,
        } => {
            let public_output: P::PublicOutputValue = public_output.into();
            let public_output = bcs::to_bytes(&public_output)?;
            mpc::AsynchronousRoundResult::Finalize {
                malicious_parties,
                private_output,
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

/// Parses an [`Event`] to extract the corresponding [`MPCParty`],
/// auxiliary input, and session information.
///
/// Returns an error if the event type does not correspond to any known MPC rounds
/// or if deserialization fails.
pub(crate) fn from_event(
    event: &Event,
    dwallet_mpc_manager: &DWalletMPCManager,
    party_id: PartyID,
) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
    match &event.type_ {
        t if t == &StartDKGFirstRoundEvent::type_() => {
            let deserialized_event: StartDKGFirstRoundEvent = bcs::from_bytes(&event.contents)?;
            dkg_first_party(deserialized_event)
        }
        t if t == &StartDKGSecondRoundEvent::type_() => {
            let deserialized_event: StartDKGSecondRoundEvent = bcs::from_bytes(&event.contents)?;
            dkg_second_party(
                deserialized_event,
                // Todo (#394): Remove the hardcoded network key type
                if cfg!(feature = "with-network-dkg") {
                    dwallet_mpc_manager.network_key_version(DWalletMPCNetworkKeyScheme::Secp256k1)?
                } else {
                    0
                },
            )
        }
        t if t == &StartPresignFirstRoundEvent::type_() => {
            let deserialized_event: StartPresignFirstRoundEvent = bcs::from_bytes(&event.contents)?;
            presign_first_party(deserialized_event)
        }
        t if t == &StartPresignSecondRoundEvent::type_() => {
            let deserialized_event: StartPresignSecondRoundEvent =
                bcs::from_bytes(&event.contents)?;
            presign_second_party(deserialized_event)
        }
        t if t == &StartSignRoundEvent::type_() => {
            let deserialized_event: StartSignRoundEvent = bcs::from_bytes(&event.contents)?;
            sign_party(party_id, deserialized_event, dwallet_mpc_manager)
        }
        t if t == &StartNetworkDKGEvent::type_() => {
            let deserialized_event: StartNetworkDKGEvent = bcs::from_bytes(&event.contents)?;
            network_dkg::network_dkg_party(deserialized_event)
        }
        _ => Err(DwalletMPCError::NonMPCEvent.into()),
    }
}
