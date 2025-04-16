use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::dkg::{
    DKGFirstParty, DKGFirstPartyPublicInputGenerator, DKGSecondParty,
    DKGSecondPartyPublicInputGenerator,
};
use crate::dwallet_mpc::mpc_manager::DWalletMPCManager;
use crate::dwallet_mpc::presign::{PresignParty, PresignPartyPublicInputGenerator};
use crate::dwallet_mpc::sign::{SignFirstParty, SignPartyPublicInputGenerator};
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCMessage, MPCPrivateInput, MPCPrivateOutput, MPCPublicInput,
    MPCPublicOutput,
};
use group::PartyID;
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::StartNetworkDKGEvent;
use ika_types::messages_dwallet_mpc::StartPartialSignaturesVerificationEvent;
use ika_types::messages_dwallet_mpc::{DBSuiEvent, StartDKGFirstRoundEvent, StartSignEvent};
use ika_types::messages_dwallet_mpc::{
    DWalletMPCEventTrait, DWalletMPCSuiEvent, IkaPackagesConfig, MPCProtocolInitData, SessionInfo,
    StartDKGSecondRoundEvent, StartEncryptedShareVerificationEvent, StartPresignFirstRoundEvent,
};
use jsonrpsee::core::Serialize;
use k256::elliptic_curve::ops::Reduce;
use mpc::{AsynchronouslyAdvanceable, Weight, WeightedThresholdAccessStructure};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use sha3::digest::FixedOutput as Sha3FixedOutput;
use sha3::Digest as Sha3Digest;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use sui_types::base_types::{EpochId, ObjectID};
use sui_types::id::{ID, UID};

use shared_wasm_class_groups::message_digest::{message_digest, Hash};

mod cryptographic_computations_orchestrator;
mod dkg;
pub mod dwallet_mpc_service;
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

/// Convert a given [`Vec<PartyID>`] to the corresponding [`Vec<AuthorityName>`].
pub(crate) fn party_ids_to_authority_names(
    party_ids: &[PartyID],
    epoch_store: &AuthorityPerEpochStore,
) -> DwalletMPCResult<Vec<AuthorityName>> {
    party_ids
        .iter()
        .map(|party_id| party_id_to_authority_name(*party_id, &epoch_store))
        .collect::<DwalletMPCResult<Vec<AuthorityName>>>()
}

/// Rust version of the Move sui::dynamic_field::Field type.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Field<N, V> {
    pub id: UID,
    pub name: N,
    pub value: V,
}

/// The type of the event is different when we receive an emitted event and when we
/// fetch the event's the dynamic field directly from Sui.
/// This function first tried to deserialize the event as a [`DWalletMPCSuiEvent`], and if it fails,
/// it tries to deserialize it as a [`Field<ID, DWalletMPCSuiEvent<T>>`].
fn deserialize_event_or_dynamic_field<T: DeserializeOwned + DWalletMPCEventTrait>(
    event_contents: &[u8],
) -> Result<DWalletMPCSuiEvent<T>, bcs::Error> {
    bcs::from_bytes::<DWalletMPCSuiEvent<T>>(event_contents).or_else(|_| {
        bcs::from_bytes::<Field<ID, DWalletMPCSuiEvent<T>>>(event_contents).map(|field| field.value)
    })
}

/// Parses the session info from the event and returns it.
/// Return `None` if the event is not a DWallet MPC event.
pub(crate) fn session_info_from_event(
    event: DBSuiEvent,
    packages_config: &IkaPackagesConfig,
) -> anyhow::Result<Option<SessionInfo>> {
    match &event.type_ {
        t if t == &DWalletMPCSuiEvent::<StartDKGFirstRoundEvent>::type_(packages_config) => {
            Ok(Some(dkg_first_party_session_info(
                deserialize_event_or_dynamic_field::<StartDKGFirstRoundEvent>(&event.contents)?,
            )?))
        }
        t if t == &DWalletMPCSuiEvent::<StartDKGSecondRoundEvent>::type_(packages_config) => {
            Ok(Some(dkg_second_party_session_info(
                deserialize_event_or_dynamic_field::<StartDKGSecondRoundEvent>(&event.contents)?,
            )))
        }
        t if t == &DWalletMPCSuiEvent::<StartPresignFirstRoundEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartPresignFirstRoundEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            Ok(Some(presign_party_session_info(deserialized_event)))
        }
        t if t == &DWalletMPCSuiEvent::<StartSignEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartSignEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            Ok(Some(sign_party_session_info(&deserialized_event)))
        }
        t if t
            == &DWalletMPCSuiEvent::<StartPartialSignaturesVerificationEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletMPCSuiEvent<StartPartialSignaturesVerificationEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            Ok(Some(get_verify_partial_signatures_session_info(
                &deserialized_event,
            )))
        }
        t if t == &DWalletMPCSuiEvent::<StartNetworkDKGEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartNetworkDKGEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            Ok(Some(network_dkg::network_dkg_session_info(
                deserialized_event,
                DWalletMPCNetworkKeyScheme::Secp256k1,
            )?))
        }
        t if t
            == &DWalletMPCSuiEvent::<StartEncryptedShareVerificationEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletMPCSuiEvent<StartEncryptedShareVerificationEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            Ok(Some(start_encrypted_share_verification_session_info(
                deserialized_event,
            )))
        }
        _ => Ok(None),
    }
}

fn start_encrypted_share_verification_session_info(
    deserialized_event: DWalletMPCSuiEvent<StartEncryptedShareVerificationEvent>,
) -> SessionInfo {
    SessionInfo {
        sequence_number: deserialized_event.session_sequence_number,
        session_id: deserialized_event.session_id,
        mpc_round: MPCProtocolInitData::EncryptedShareVerification(deserialized_event),
    }
}

fn dkg_first_public_input(protocol_public_parameters: Vec<u8>) -> DwalletMPCResult<Vec<u8>> {
    <DKGFirstParty as DKGFirstPartyPublicInputGenerator>::generate_public_input(
        protocol_public_parameters,
    )
}

fn dkg_first_party_session_info(
    deserialized_event: DWalletMPCSuiEvent<StartDKGFirstRoundEvent>,
) -> anyhow::Result<SessionInfo> {
    Ok(SessionInfo {
        sequence_number: deserialized_event.session_sequence_number,
        session_id: deserialized_event.session_id,
        mpc_round: MPCProtocolInitData::DKGFirst(deserialized_event),
    })
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
    deserialized_event: DWalletMPCSuiEvent<StartDKGSecondRoundEvent>,
) -> SessionInfo {
    SessionInfo {
        sequence_number: deserialized_event.session_sequence_number,
        session_id: ObjectID::from(deserialized_event.session_id),
        mpc_round: MPCProtocolInitData::DKGSecond(deserialized_event.clone()),
    }
}

pub(crate) fn presign_public_input(
    deserialized_event: StartPresignFirstRoundEvent,
    protocol_public_parameters: Vec<u8>,
) -> DwalletMPCResult<Vec<u8>> {
    Ok(
        <PresignParty as PresignPartyPublicInputGenerator>::generate_public_input(
            protocol_public_parameters,
            deserialized_event.dkg_output.clone(),
        )?,
    )
}

fn presign_party_session_info(
    deserialized_event: DWalletMPCSuiEvent<StartPresignFirstRoundEvent>,
) -> SessionInfo {
    SessionInfo {
        sequence_number: deserialized_event.session_sequence_number,
        session_id: deserialized_event.session_id,
        mpc_round: MPCProtocolInitData::Presign(deserialized_event),
    }
}

fn sign_public_input(
    deserialized_event: &StartSignEvent,
    dwallet_mpc_manager: &DWalletMPCManager,
    protocol_public_parameters: Vec<u8>,
) -> DwalletMPCResult<Vec<u8>> {
    let decryption_pp = dwallet_mpc_manager.get_decryption_key_share_public_parameters(
        // The `StartSignRoundEvent` is assign with a Secp256k1 dwallet.
        // Todo (#473): Support generic network key scheme
        &deserialized_event.dwallet_mpc_network_key_id,
    )?;
    Ok(
        <SignFirstParty as SignPartyPublicInputGenerator>::generate_public_input(
            protocol_public_parameters,
            deserialized_event
                .dwallet_decentralized_public_output
                .clone(),
            bcs::to_bytes(
                &message_digest(
                    &deserialized_event.message.clone(),
                    &Hash::try_from(deserialized_event.hash_scheme)
                        .map_err(|e| DwalletMPCError::SignatureVerificationFailed(e.to_string()))?,
                )
                .map_err(|e| DwalletMPCError::SignatureVerificationFailed(e.to_string()))?,
            )?,
            deserialized_event.presign.clone(),
            deserialized_event.message_centralized_signature.clone(),
            bcs::from_bytes(&decryption_pp)?,
        )?,
    )
}

fn sign_party_session_info(deserialized_event: &DWalletMPCSuiEvent<StartSignEvent>) -> SessionInfo {
    SessionInfo {
        sequence_number: deserialized_event.session_sequence_number,
        session_id: deserialized_event.session_id,
        mpc_round: MPCProtocolInitData::Sign(deserialized_event.clone()),
    }
}

fn get_verify_partial_signatures_session_info(
    deserialized_event: &DWalletMPCSuiEvent<StartPartialSignaturesVerificationEvent>,
) -> SessionInfo {
    SessionInfo {
        sequence_number: deserialized_event.session_sequence_number,
        session_id: deserialized_event.session_id,
        mpc_round: MPCProtocolInitData::PartialSignatureVerification(deserialized_event.clone()),
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
    let DeserializeMPCMessagesResponse {
        messages,
        malicious_parties: _,
    } = deserialize_mpc_messages(messages);

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

struct DeserializeMPCMessagesResponse<M: DeserializeOwned + Clone> {
    messages: Vec<HashMap<PartyID, M>>,
    malicious_parties: Vec<PartyID>,
}

/// Deserializes the messages received from other parties for the next advancement.
/// Any value that fails to deserialize is considered to be sent by a malicious party.
/// Returns the deserialized messages or an error including the IDs of the malicious parties.
fn deserialize_mpc_messages<M: DeserializeOwned + Clone>(
    messages: Vec<HashMap<PartyID, MPCMessage>>,
) -> DeserializeMPCMessagesResponse<M> {
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
    DeserializeMPCMessagesResponse {
        messages: deserialized_results,
        malicious_parties,
    }
}

// TODO (#542): move this logic to run before writing the event to the DB, maybe include within the session info
/// Parses an [`Event`] to extract the corresponding [`MPCParty`],
/// public input, private input and session information.
///
/// Returns an error if the event type does not correspond to any known MPC rounds
/// or if deserialization fails.
pub(crate) async fn session_input_from_event(
    event: DBSuiEvent,
    dwallet_mpc_manager: &DWalletMPCManager,
) -> DwalletMPCResult<(MPCPublicInput, MPCPrivateInput)> {
    let packages_config = &dwallet_mpc_manager.epoch_store()?.packages_config;
    match &event.type_ {
        t if t == &DWalletMPCSuiEvent::<StartNetworkDKGEvent>::type_(packages_config) => Ok((
            network_dkg::network_dkg_public_input(
                dwallet_mpc_manager
                    .validators_class_groups_public_keys_and_proofs
                    .clone(),
                DWalletMPCNetworkKeyScheme::Secp256k1,
            )?,
            Some(bcs::to_bytes(
                &dwallet_mpc_manager
                    .node_config
                    .class_groups_key_pair_and_proof
                    .class_groups_keypair()
                    .decryption_key(),
            )?),
        )),
        t if t == &DWalletMPCSuiEvent::<StartDKGFirstRoundEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartDKGFirstRoundEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager
                .get_protocol_public_parameters(
                    // The event is assign with a Secp256k1 dwallet.
                    // Todo (#473): Support generic network key scheme
                    &deserialized_event
                        .event_data
                        .dwallet_network_decryption_key_id,
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                )
                .await;
            Ok((dkg_first_public_input(protocol_public_parameters)?, None))
        }
        t if t == &DWalletMPCSuiEvent::<StartDKGSecondRoundEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartDKGSecondRoundEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager
                .get_protocol_public_parameters(
                    // The event is assign with a Secp256k1 dwallet.
                    // Todo (#473): Support generic network key scheme
                    &deserialized_event.event_data.dwallet_mpc_network_key_id,
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                )
                .await;
            Ok((
                dkg_second_public_input(deserialized_event.event_data, protocol_public_parameters)?,
                None,
            ))
        }
        t if t == &DWalletMPCSuiEvent::<StartPresignFirstRoundEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartPresignFirstRoundEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager
                .get_protocol_public_parameters(
                    // The event is assign with a Secp256k1 dwallet.
                    // Todo (#473): Support generic network key scheme
                    &deserialized_event
                        .event_data
                        .dwallet_network_decryption_key_id,
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                )
                .await;
            Ok((
                presign_public_input(deserialized_event.event_data, protocol_public_parameters)?,
                None,
            ))
        }
        t if t == &DWalletMPCSuiEvent::<StartSignEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartSignEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager
                .get_protocol_public_parameters(
                    // The event is assign with a Secp256k1 dwallet.
                    // Todo (#473): Support generic network key scheme
                    &deserialized_event.event_data.dwallet_mpc_network_key_id,
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                )
                .await;
            Ok((
                sign_public_input(
                    &deserialized_event.event_data,
                    dwallet_mpc_manager,
                    protocol_public_parameters,
                )?,
                None,
            ))
        }
        t if t
            == &DWalletMPCSuiEvent::<StartEncryptedShareVerificationEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletMPCSuiEvent<StartEncryptedShareVerificationEvent> =
                bcs::from_bytes(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager
                .get_protocol_public_parameters(
                    // The event is assign with a Secp256k1 dwallet.
                    // Todo (#473): Support generic network key scheme
                    &deserialized_event.event_data.dwallet_mpc_network_key_id,
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                )
                .await;
            Ok((protocol_public_parameters, None))
        }
        t if t
            == &DWalletMPCSuiEvent::<StartPartialSignaturesVerificationEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletMPCSuiEvent<StartPartialSignaturesVerificationEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager
                .get_protocol_public_parameters(
                    // The event is assign with a Secp256k1 dwallet.
                    // Todo (#473): Support generic network key scheme
                    &deserialized_event.event_data.dwallet_mpc_network_key_id,
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                )
                .await;
            Ok((protocol_public_parameters, None))
        }
        _ => Err(DwalletMPCError::NonMPCEvent(event.type_.name.to_string()).into()),
    }
}

// TODO (#683): Parse the network key version from the network key object ID
pub(crate) fn network_key_version_from_key_id(_key_id: &ObjectID) -> u8 {
    0
}
