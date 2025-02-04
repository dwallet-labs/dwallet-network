use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::dkg::{
    DKGFirstParty, DKGFirstPartyPublicInputGenerator, DKGSecondParty,
    DKGSecondPartyPublicInputGenerator,
};
use crate::dwallet_mpc::ecdsa_k1::SignData;
use crate::dwallet_mpc::mpc_events::{
    StartBatchedPresignEvent, StartBatchedSignEvent, StartDKGFirstRoundEvent, StartNetworkDKGEvent,
    StartPresignFirstRoundEvent, StartPresignSecondRoundEvent, StartSignEvent,
};
use crate::dwallet_mpc::mpc_manager::DWalletMPCManager;
use crate::dwallet_mpc::presign::{
    PresignFirstParty, PresignFirstPartyPublicInputGenerator, PresignSecondParty,
    PresignSecondPartyPublicInputGenerator,
};
use crate::dwallet_mpc::sign::{SignFirstParty, SignPartyPublicInputGenerator};
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCMessage, MPCPrivateInput, MPCPrivateOutput, MPCPublicInput,
    MPCPublicOutput,
};
use group::PartyID;
use mpc::{AsynchronouslyAdvanceable, Weight, WeightedThresholdAccessStructure};
use ika_types::crypto::AuthorityName;
use sui_types::base_types::{EpochId, ObjectID};
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use sui_types::event::Event;
use ika_types::messages_dwallet_mpc::{
    MPCProtocolInitData, SessionInfo, SingleSignSessionData, StartDKGSecondRoundEvent,
    StartEncryptedShareVerificationEvent, StartEncryptionKeyVerificationEvent,
};
use serde::de::DeserializeOwned;
use std::collections::{HashMap, HashSet};

pub mod batches_manager;
mod cryptographic_computations_orchestrator;
mod dkg;
pub mod dwallet_mpc_service;
mod ecdsa_k1;
mod encrypt_user_share;
mod malicious_handler;
pub(crate) mod mpc_events;
pub mod mpc_manager;
pub mod mpc_outputs_verifier;
pub mod mpc_session;
pub mod network_dkg;
mod presign;
pub(crate) mod sign;

pub const FIRST_EPOCH_ID: EpochId = 0;

/// Convert a given authority name (address) to it's corresponding [`PartyID`].
/// The [`PartyID`] is the index of the authority in the committee.
pub(crate) fn authority_name_to_party_id(
    authority_name: &ika_types::crypto::AuthorityName,
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

/// Convert a given [`PartyID`] to it's corresponding authority name (address).
pub(crate) fn party_id_to_authority_name(
    party_id: PartyID,
    epoch_store: &AuthorityPerEpochStore,
) -> DwalletMPCResult<AuthorityName> {
    Ok(epoch_store
        .committee()
        .authority_by_index(party_id as u32 - 1)
        .ok_or(DwalletMPCError::AuthorityIndexNotFound(party_id - 1))?
        .clone())
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
        t if t == &StartSignEvent::<SignData>::type_(SignData::type_().into()) => {
            let deserialized_event: StartSignEvent<SignData> = bcs::from_bytes(&event.contents)?;
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
        t if t == &StartEncryptedShareVerificationEvent::type_() => {
            let deserialized_event: StartEncryptedShareVerificationEvent =
                bcs::from_bytes(&event.contents)?;
            Ok(Some(start_encrypted_share_verification_session_info(
                deserialized_event,
            )))
        }
        t if t == &StartEncryptionKeyVerificationEvent::type_() => {
            let deserialized_event: StartEncryptionKeyVerificationEvent =
                bcs::from_bytes(&event.contents)?;
            Ok(Some(start_encryption_key_verification_session_info(
                deserialized_event,
            )))
        }
        _ => Ok(None),
    }
}

fn start_encrypted_share_verification_session_info(
    deserialized_event: StartEncryptedShareVerificationEvent,
) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id,
        session_id: deserialized_event.session_id,
        initiating_user_address: Default::default(),
        mpc_round: MPCProtocolInitData::EncryptedShareVerification(deserialized_event),
    }
}

fn start_encryption_key_verification_session_info(
    deserialized_event: StartEncryptionKeyVerificationEvent,
) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id,
        session_id: deserialized_event.session_id,
        initiating_user_address: Default::default(),
        mpc_round: MPCProtocolInitData::EncryptionKeyVerification(deserialized_event),
    }
}

fn dkg_second_public_input(
    deserialized_event: StartDKGSecondRoundEvent,
    protocol_public_parameters: Vec<u8>,
) -> DwalletMPCResult<Vec<u8>> {
    Ok(DKGSecondParty::generate_public_input(
        protocol_public_parameters,
        deserialized_event.first_round_output.clone(),
        deserialized_event
            .centralized_public_key_share_and_proof
            .clone(),
    )?)
}

fn dkg_second_party_session_info(
    deserialized_event: &StartDKGSecondRoundEvent,
    dwallet_network_key_version: u8,
) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.first_round_session_id,
        session_id: ObjectID::from(deserialized_event.session_id),
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCProtocolInitData::DKGSecond(
            deserialized_event.clone(),
            dwallet_network_key_version,
        ),
    }
}

fn dkg_first_public_input(protocol_public_parameters: Vec<u8>) -> DwalletMPCResult<Vec<u8>> {
    <DKGFirstParty as DKGFirstPartyPublicInputGenerator>::generate_public_input(
        protocol_public_parameters,
    )
}

fn dkg_first_party_session_info(deserialized_event: StartDKGFirstRoundEvent) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCProtocolInitData::DKGFirst,
    }
}

fn presign_first_public_input(
    deserialized_event: StartPresignFirstRoundEvent,
    protocol_public_parameters: Vec<u8>,
) -> DwalletMPCResult<Vec<u8>> {
    Ok(
        <PresignFirstParty as PresignFirstPartyPublicInputGenerator>::generate_public_input(
            protocol_public_parameters,
            deserialized_event.dkg_output.clone(),
        )?,
    )
}

fn presign_first_party_session_info(
    deserialized_event: StartPresignFirstRoundEvent,
) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCProtocolInitData::PresignFirst(
            deserialized_event.dwallet_id.bytes,
            deserialized_event.dkg_output,
            deserialized_event.batch_session_id.bytes,
            deserialized_event.dwallet_mpc_network_key_version,
        ),
    }
}

fn presign_second_public_input(
    deserialized_event: StartPresignSecondRoundEvent,
    protocol_public_parameters: Vec<u8>,
) -> DwalletMPCResult<Vec<u8>> {
    Ok(
        <PresignSecondParty as PresignSecondPartyPublicInputGenerator>::generate_public_input(
            protocol_public_parameters,
            deserialized_event.dkg_output.clone(),
            deserialized_event.first_round_output.clone(),
        )?,
    )
}

fn presign_second_party_session_info(
    deserialized_event: &StartPresignSecondRoundEvent,
) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.first_round_session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCProtocolInitData::PresignSecond(
            deserialized_event.dwallet_id.bytes,
            deserialized_event.first_round_output.clone(),
            deserialized_event.batch_session_id.bytes,
        ),
    }
}

fn sign_public_input(
    deserialized_event: &StartSignEvent<SignData>,
    dwallet_mpc_manager: &DWalletMPCManager,
    protocol_public_parameters: Vec<u8>,
) -> DwalletMPCResult<Vec<u8>> {
    let decryption_pp = dwallet_mpc_manager.get_decryption_key_share_public_parameters(
        // The `StartSignRoundEvent` is assign with a Secp256k1 dwallet.
        // Todo (#473): Support generic network key scheme
        DWalletMPCNetworkKeyScheme::Secp256k1,
        deserialized_event.dwallet_mpc_network_key_version,
    )?;
    Ok(
        <SignFirstParty as SignPartyPublicInputGenerator>::generate_public_input(
            protocol_public_parameters,
            deserialized_event
                .dwallet_decentralized_public_output
                .clone(),
            deserialized_event.hashed_message.clone(),
            deserialized_event
                .signature_algorithm_data
                .presign_output
                .clone(),
            deserialized_event
                .signature_algorithm_data
                .message_centralized_signature
                .clone(),
            bcs::from_bytes(&decryption_pp)?,
        )?,
    )
}

fn sign_party_session_info(
    deserialized_event: &StartSignEvent<SignData>,
    _party_id: PartyID,
) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.signature_algorithm_data.presign_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCProtocolInitData::Sign(SingleSignSessionData {
            batch_session_id: deserialized_event.batched_session_id.bytes,
            hashed_message: deserialized_event.hashed_message.clone(),
            dwallet_id: deserialized_event.dwallet_id.bytes,
            dwallet_decentralized_public_output: deserialized_event
                .dwallet_decentralized_public_output
                .clone(),
            network_key_version: deserialized_event.dwallet_mpc_network_key_version,
            is_future_sign: deserialized_event.is_future_sign,
        }),
    }
}

fn batched_sign_session_info(deserialized_event: &StartBatchedSignEvent) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCProtocolInitData::BatchedSign(deserialized_event.hashed_messages.clone()),
    }
}

fn batched_presign_session_info(deserialized_event: &StartBatchedPresignEvent) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCProtocolInitData::BatchedPresign(deserialized_event.batch_size),
    }
}

fn calculate_total_voting_weight(
    weighted_parties: &HashMap<PartyID, Weight>,
    parties: &HashSet<PartyID>,
) -> usize {
    let mut total_voting_weight = 0;
    for party in parties {
        if let Some(weight) = weighted_parties.get(&party) {
            total_voting_weight += *weight as usize;
        }
    }
    total_voting_weight
}
/// Advances the state of an MPC party and serializes the result into bytes.
///
/// This helper function wraps around a party `P`'s `advance()` method,
/// converting its output into a serialized byte format.
/// This abstraction allows the system's generic components to operate uniformly on byte arrays,
/// rather than requiring generics to handle the different message and output types
/// for each MPC protocol.
///
/// By maintaining a structured transition between instantiated types, and their
/// serialized forms, this function ensures compatibility across various components.
pub(crate) fn advance_and_serialize<P: AsynchronouslyAdvanceable>(
    session_id: CommitmentSizedNumber,
    party_id: PartyID,
    access_threshold: &WeightedThresholdAccessStructure,
    messages: Vec<HashMap<PartyID, MPCMessage>>,
    public_input: P::PublicInput,
    private_input: P::PrivateInput,
) -> DwalletMPCResult<mpc::AsynchronousRoundResult<MPCMessage, MPCPrivateOutput, MPCPublicOutput>> {
    let messages = deserialize_mpc_messages(messages)?;

    let res = match P::advance(
        session_id,
        party_id,
        access_threshold,
        messages.clone(),
        Some(private_input),
        &public_input,
        &mut rand_core::OsRng,
    ) {
        Ok(res) => res,
        Err(e) => {
            let general_error = DwalletMPCError::TwoPCMPCError(format!("{:?}", e));
            return match e.into() {
                // No threshold was reached, so we can't proceed.
                mpc::Error::ThresholdNotReached { honest_subset } => {
                    let malicious_actors = messages
                        .last()
                        .ok_or(general_error)?
                        .keys()
                        .filter(|party_id| !honest_subset.contains(*party_id))
                        .cloned()
                        .collect();
                    Err(DwalletMPCError::SessionFailedWithMaliciousParties(
                        malicious_actors,
                    ))
                }
                _ => Err(general_error),
            };
        }
    };

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
            let private_output = bcs::to_bytes(&private_output)?;
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
        Err(DwalletMPCError::SessionFailedWithMaliciousParties(
            malicious_parties,
        ))
    } else {
        Ok(deserialized_results)
    }
}

// TODO (#542): move this logic to run before writing the event to the DB, maybe include within the session info
/// Parses an [`Event`] to extract the corresponding [`MPCParty`],
/// public input, private input and session information.
///
/// Returns an error if the event type does not correspond to any known MPC rounds
/// or if deserialization fails.
pub(crate) fn session_input_from_event(
    event: &Event,
    dwallet_mpc_manager: &DWalletMPCManager,
) -> DwalletMPCResult<(MPCPublicInput, MPCPrivateInput)> {
    match &event.type_ {
        t if t == &StartNetworkDKGEvent::type_() => {
            let deserialized_event: StartNetworkDKGEvent = bcs::from_bytes(&event.contents)?;
            Ok((
                network_dkg::network_dkg_public_input(
                    deserialized_event,
                    &dwallet_mpc_manager.validators_data_for_network_dkg,
                )?,
                Some(bcs::to_bytes(&[1,2]
                    // &dwallet_mpc_manager.node_config.class_groups_private_key,
                )?),
            ))
        }
        t if t == &StartDKGFirstRoundEvent::type_() => {
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                DWalletMPCNetworkKeyScheme::Secp256k1,
                dwallet_mpc_manager.network_key_version(DWalletMPCNetworkKeyScheme::Secp256k1)?,
            )?;
            Ok((dkg_first_public_input(protocol_public_parameters)?, None))
        }
        t if t == &StartDKGSecondRoundEvent::type_() => {
            let deserialized_event: StartDKGSecondRoundEvent = bcs::from_bytes(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                DWalletMPCNetworkKeyScheme::Secp256k1,
                dwallet_mpc_manager.network_key_version(DWalletMPCNetworkKeyScheme::Secp256k1)?,
            )?;
            Ok((
                dkg_second_public_input(deserialized_event, protocol_public_parameters)?,
                None,
            ))
        }
        t if t == &StartPresignFirstRoundEvent::type_() => {
            let deserialized_event: StartPresignFirstRoundEvent = bcs::from_bytes(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                DWalletMPCNetworkKeyScheme::Secp256k1,
                deserialized_event.dwallet_mpc_network_key_version,
            )?;
            Ok((
                presign_first_public_input(deserialized_event, protocol_public_parameters)?,
                None,
            ))
        }
        t if t == &StartPresignSecondRoundEvent::type_() => {
            let deserialized_event: StartPresignSecondRoundEvent =
                bcs::from_bytes(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                DWalletMPCNetworkKeyScheme::Secp256k1,
                deserialized_event.dwallet_mpc_network_key_version,
            )?;
            Ok((
                presign_second_public_input(deserialized_event, protocol_public_parameters)?,
                None,
            ))
        }
        t if t == &StartSignEvent::<SignData>::type_(SignData::type_().into()) => {
            let deserialized_event: StartSignEvent<SignData> = bcs::from_bytes(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                DWalletMPCNetworkKeyScheme::Secp256k1,
                deserialized_event.dwallet_mpc_network_key_version,
            )?;
            Ok((
                sign_public_input(
                    &deserialized_event,
                    dwallet_mpc_manager,
                    protocol_public_parameters,
                )?,
                None,
            ))
        }
        t if t == &StartEncryptedShareVerificationEvent::type_() => Ok((vec![], None)),
        t if t == &StartEncryptionKeyVerificationEvent::type_() => Ok((vec![], None)),
        _ => Err(DwalletMPCError::NonMPCEvent(event.type_.name.to_string()).into()),
    }
}
