use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::dkg::{
    DKGFirstParty, DKGFirstPartyPublicInputGenerator, DKGSecondParty,
    DKGSecondPartyPublicInputGenerator,
};
use crate::dwallet_mpc::mpc_events::{
    StartBatchedPresignEvent, StartBatchedSignEvent, StartNetworkDKGEvent, StartSignEvent,
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
use ika_types::messages_dwallet_mpc::{DBSuiEvent, StartDKGFirstRoundEvent};
use ika_types::messages_dwallet_mpc::{
    DWalletMPCEventTrait, DWalletMPCSuiEvent, IkaPackagesConfig, MPCProtocolInitData, SessionInfo,
    SingleSignSessionData, StartDKGSecondRoundEvent, StartEncryptedShareVerificationEvent,
    StartEncryptionKeyVerificationEvent, StartPresignFirstRoundEvent,
};
use ika_types::messages_dwallet_mpc::{SignData, StartPartialSignaturesVerificationEvent};
use mpc::{AsynchronouslyAdvanceable, Weight, WeightedThresholdAccessStructure};
use rand_core::OsRng;
use serde::de::DeserializeOwned;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use sui_json_rpc_types::SuiEvent;
use sui_types::base_types::{EpochId, ObjectID, SuiAddress};
use tracing::warn;

pub mod batches_manager;
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
    malicious_parties: &[PartyID],
    epoch_store: &AuthorityPerEpochStore,
) -> DwalletMPCResult<Vec<AuthorityName>> {
    malicious_parties
        .iter()
        .map(|party_id| party_id_to_authority_name(*party_id, &epoch_store))
        .collect::<DwalletMPCResult<Vec<AuthorityName>>>()
}

/// Parses the session info from the event and returns it.
/// Return `None` if the event is not a DWallet MPC event.
pub(crate) fn session_info_from_event(
    event: DBSuiEvent,
    dwallet_network_key_version: Option<u8>,
    packages_config: &IkaPackagesConfig,
) -> anyhow::Result<Option<SessionInfo>> {
    let expected_event = DWalletMPCSuiEvent::<StartDKGSecondRoundEvent>::type_(packages_config).to_canonical_string(false);
    let actual = event.type_.to_canonical_string(false);
    warn!(
        "Expected event: {}", expected_event
    );
    warn!(
        "Actual event: {}", actual
    );
    match &event.type_ {
        t if t == &DWalletMPCSuiEvent::<StartDKGFirstRoundEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartDKGFirstRoundEvent> =
                bcs::from_bytes(&event.contents)?;
            Ok(Some(dkg_first_party_session_info(deserialized_event)?))
        }
        t if t == &DWalletMPCSuiEvent::<StartDKGSecondRoundEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartDKGSecondRoundEvent> =
                bcs::from_bytes(&event.contents)?;
            Ok(Some(dkg_second_party_session_info(
                &deserialized_event.event_data,
                if cfg!(feature = "with-network-dkg") {
                    dwallet_network_key_version.ok_or(DwalletMPCError::MissingKeyVersion)?
                } else {
                    0
                },
            )))
        }
        t if t == &DWalletMPCSuiEvent::<StartPresignFirstRoundEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartPresignFirstRoundEvent> =
                bcs::from_bytes(&event.contents)?;
            Ok(Some(presign_party_session_info(
                deserialized_event.event_data,
            )))
        }
        t if t == &DWalletMPCSuiEvent::<StartSignEvent<SignData>>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartSignEvent<SignData>> =
                bcs::from_bytes(&event.contents)?;
            Ok(Some(sign_party_session_info(
                &deserialized_event.event_data,
            )))
        }
        t if t
            == &DWalletMPCSuiEvent::<StartPartialSignaturesVerificationEvent<SignData>>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletMPCSuiEvent<
                StartPartialSignaturesVerificationEvent<SignData>,
            > = bcs::from_bytes(&event.contents)?;
            Ok(Some(get_verify_partial_signatures_session_info(
                &deserialized_event.event_data,
            )))
        }
        t if t == &DWalletMPCSuiEvent::<StartBatchedSignEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartBatchedSignEvent> =
                bcs::from_bytes(&event.contents)?;
            Ok(Some(batched_sign_session_info(
                &deserialized_event.event_data,
            )))
        }
        t if t == &DWalletMPCSuiEvent::<StartBatchedPresignEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartBatchedPresignEvent> =
                bcs::from_bytes(&event.contents)?;
            Ok(Some(batched_presign_session_info(
                &deserialized_event.event_data,
            )))
        }
        t if t == &DWalletMPCSuiEvent::<StartNetworkDKGEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartNetworkDKGEvent> =
                bcs::from_bytes(&event.contents)?;
            Ok(Some(network_dkg::network_dkg_session_info(
                deserialized_event.event_data,
            )?))
        }
        t if t
            == &DWalletMPCSuiEvent::<StartEncryptedShareVerificationEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletMPCSuiEvent<StartEncryptedShareVerificationEvent> =
                bcs::from_bytes(&event.contents)?;
            Ok(Some(start_encrypted_share_verification_session_info(
                deserialized_event.event_data,
            )))
        }
        t if t
            == &DWalletMPCSuiEvent::<StartEncryptionKeyVerificationEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletMPCSuiEvent<StartEncryptionKeyVerificationEvent> =
                bcs::from_bytes(&event.contents)?;
            Ok(Some(start_encryption_key_verification_session_info(
                deserialized_event.event_data,
            )))
        }
        _ => Ok(None),
    }
}

fn start_encrypted_share_verification_session_info(
    deserialized_event: StartEncryptedShareVerificationEvent,
) -> SessionInfo {
    SessionInfo {
        session_id: deserialized_event.session_id,
        initiating_user_address: Default::default(),
        mpc_round: MPCProtocolInitData::EncryptedShareVerification(deserialized_event),
    }
}

fn start_encryption_key_verification_session_info(
    deserialized_event: StartEncryptionKeyVerificationEvent,
) -> SessionInfo {
    SessionInfo {
        session_id: deserialized_event.session_id,
        initiating_user_address: Default::default(),
        mpc_round: MPCProtocolInitData::EncryptionKeyVerification(deserialized_event),
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
        session_id: deserialized_event.session_id,
        // TODO (#642): Remove the redundant initiating user address field
        initiating_user_address: SuiAddress::from_bytes(
            deserialized_event.session_id.into_bytes(),
        )?,
        mpc_round: MPCProtocolInitData::DKGFirst(deserialized_event.event_data),
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
    deserialized_event: &StartDKGSecondRoundEvent,
    dwallet_network_key_version: u8,
) -> SessionInfo {
    SessionInfo {
        session_id: ObjectID::from(deserialized_event.session_id),
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCProtocolInitData::DKGSecond(
            deserialized_event.clone(),
            dwallet_network_key_version,
        ),
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

fn presign_party_session_info(deserialized_event: StartPresignFirstRoundEvent) -> SessionInfo {
    SessionInfo {
        session_id: deserialized_event.session_id,
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCProtocolInitData::Presign(deserialized_event),
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

fn sign_party_session_info(deserialized_event: &StartSignEvent<SignData>) -> SessionInfo {
    SessionInfo {
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
            presign_session_id: deserialized_event.signature_algorithm_data.presign_id,
        }),
    }
}

fn get_verify_partial_signatures_session_info(
    deserialized_event: &StartPartialSignaturesVerificationEvent<SignData>,
) -> SessionInfo {
    SessionInfo {
        session_id: deserialized_event.session_id,
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCProtocolInitData::PartialSignatureVerification(deserialized_event.clone()),
    }
}

fn batched_sign_session_info(deserialized_event: &StartBatchedSignEvent) -> SessionInfo {
    SessionInfo {
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: deserialized_event.initiator,
        mpc_round: MPCProtocolInitData::BatchedSign(deserialized_event.hashed_messages.clone()),
    }
}

fn batched_presign_session_info(deserialized_event: &StartBatchedPresignEvent) -> SessionInfo {
    SessionInfo {
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
    event: DBSuiEvent,
    dwallet_mpc_manager: &DWalletMPCManager,
) -> DwalletMPCResult<(MPCPublicInput, MPCPrivateInput)> {
    let packages_config = &dwallet_mpc_manager.epoch_store()?.packages_config;
    match &event.type_ {
        t if t == &DWalletMPCSuiEvent::<StartNetworkDKGEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartNetworkDKGEvent> =
                bcs::from_bytes(&event.contents)?;
            Ok((
                network_dkg::network_dkg_public_input(
                    deserialized_event.event_data,
                    &dwallet_mpc_manager.validators_data_for_network_dkg,
                )?,
                Some(bcs::to_bytes(
                    &dwallet_mpc_manager.node_config.class_groups_private_key,
                )?),
            ))
        }
        t if t == &DWalletMPCSuiEvent::<StartDKGFirstRoundEvent>::type_(packages_config) => {
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                DWalletMPCNetworkKeyScheme::Secp256k1,
                dwallet_mpc_manager.network_key_version(DWalletMPCNetworkKeyScheme::Secp256k1)?,
            )?;
            Ok((dkg_first_public_input(protocol_public_parameters)?, None))
        }
        t if t == &DWalletMPCSuiEvent::<StartDKGSecondRoundEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartDKGSecondRoundEvent> =
                bcs::from_bytes(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                DWalletMPCNetworkKeyScheme::Secp256k1,
                dwallet_mpc_manager.network_key_version(DWalletMPCNetworkKeyScheme::Secp256k1)?,
            )?;
            Ok((
                dkg_second_public_input(deserialized_event.event_data, protocol_public_parameters)?,
                None,
            ))
        }
        t if t == &DWalletMPCSuiEvent::<StartPresignFirstRoundEvent>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartPresignFirstRoundEvent> =
                bcs::from_bytes(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                DWalletMPCNetworkKeyScheme::Secp256k1,
                deserialized_event
                    .event_data
                    .dwallet_mpc_network_key_version,
            )?;
            Ok((
                presign_public_input(deserialized_event.event_data, protocol_public_parameters)?,
                None,
            ))
        }
        t if t == &DWalletMPCSuiEvent::<StartSignEvent<SignData>>::type_(packages_config) => {
            let deserialized_event: DWalletMPCSuiEvent<StartSignEvent<SignData>> =
                bcs::from_bytes(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                DWalletMPCNetworkKeyScheme::Secp256k1,
                deserialized_event
                    .event_data
                    .dwallet_mpc_network_key_version,
            )?;
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
            Ok((vec![], None))
        }
        t if t
            == &DWalletMPCSuiEvent::<StartEncryptionKeyVerificationEvent>::type_(
                packages_config,
            ) =>
        {
            Ok((vec![], None))
        }
        t if t
            == &DWalletMPCSuiEvent::<StartPartialSignaturesVerificationEvent<SignData>>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletMPCSuiEvent<
                StartPartialSignaturesVerificationEvent<SignData>,
            > = bcs::from_bytes(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                DWalletMPCNetworkKeyScheme::Secp256k1,
                deserialized_event
                    .event_data
                    .dwallet_mpc_network_decryption_key_version,
            )?;
            Ok((protocol_public_parameters, None))
        }
        _ => Err(DwalletMPCError::NonMPCEvent(event.type_.name.to_string()).into()),
    }
}
