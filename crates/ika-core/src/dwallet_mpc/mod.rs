use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::dkg::{
    DKGFirstParty, DKGFirstPartyPublicInputGenerator, DKGSecondParty,
    DKGSecondPartyPublicInputGenerator,
};
use crate::dwallet_mpc::mpc_manager::DWalletMPCManager;
use crate::dwallet_mpc::mpc_session::PublicInput;
use crate::dwallet_mpc::presign::{PresignParty, PresignPartyPublicInputGenerator};
use crate::dwallet_mpc::reshare::{ResharePartyPublicInputGenerator, ReshareSecp256k1Party};
use crate::dwallet_mpc::sign::{SignFirstParty, SignPartyPublicInputGenerator};
use class_groups::SecretKeyShareSizedInteger;
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCMessage, MPCPrivateInput, MPCPrivateOutput,
    SerializedWrappedMPCPublicOutput, VersionedImportedDWalletPublicOutput,
};
use group::PartyID;
use ika_types::committee::Committee;
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{
    DBSuiEvent, DWalletDKGFirstRoundRequestEvent, DWalletImportedKeyVerificationRequestEvent,
    SessionIdentifier, SignRequestEvent,
};
use ika_types::messages_dwallet_mpc::{
    DWalletDKGSecondRoundRequestEvent, DWalletSessionEvent, DWalletSessionEventTrait,
    EncryptedShareVerificationRequestEvent, IkaPackagesConfig, MPCProtocolInitData,
    PresignRequestEvent, SessionInfo,
};
use ika_types::messages_dwallet_mpc::{
    DWalletEncryptionKeyReconfigurationRequestEvent, DWalletNetworkDKGEncryptionKeyRequestEvent,
};
use ika_types::messages_dwallet_mpc::{
    FutureSignRequestEvent, MakeDWalletUserSecretKeySharesPublicRequestEvent,
};
use mpc::{AsynchronouslyAdvanceable, Weight, WeightedThresholdAccessStructure};
use serde::de::DeserializeOwned;
use serde_json::json;
use shared_wasm_class_groups::message_digest::{message_digest, Hash};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use std::vec::Vec;
use sui_types::base_types::{EpochId, ObjectID};
use sui_types::dynamic_field::Field;
use sui_types::id::ID;
use tracing::warn;

mod cryptographic_computations_orchestrator;
mod dkg;
pub mod dwallet_mpc_service;
mod encrypt_user_share;
mod malicious_handler;
pub mod mpc_manager;
pub mod mpc_outputs_verifier;
pub mod mpc_session;
pub mod network_dkg;
mod presign;

pub mod dwallet_mpc_metrics;
mod make_dwallet_user_secret_key_shares_public;
mod reshare;
pub(crate) mod sign;

pub const FIRST_EPOCH_ID: EpochId = 0;
static LOG_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Convert an `authority_name` to the tangible party ID (`PartyID`) in the `committee`.
pub(crate) fn authority_name_to_party_id_from_committee(
    committee: &Committee,
    authority_name: &AuthorityName,
) -> DwalletMPCResult<PartyID> {
    // The index of the authority `authority_name` in the `committee`.
    // This value is in the range `0..number_of_tangible_parties`,
    // and represents a unique index to the set of authority names.
    let authority_index = committee
        .authority_index(authority_name)
        .ok_or(DwalletMPCError::AuthorityNameNotFound(*authority_name))?;

    // A tangible party ID is of type `PartyID` and in the range `1..=number_of_tangible_parties`.
    // Increment the index to transform it from 0-based to 1-based.
    let tangible_party_id: u32 = authority_index.checked_add(1).expect("should never have more than 2^32 parties");
    let tangible_party_id: u16 = tangible_party_id.try_into().expect("should never have more than 2^16 parties");

    Ok(tangible_party_id)
}

/// Convert a `committee` to a `WeightedThresholdAccessStructure` that is used by the cryptographic library.
pub(crate) fn generate_access_structure_from_committee(
    committee: &Committee,
) -> DwalletMPCResult<WeightedThresholdAccessStructure> {
    let party_to_weight: HashMap<PartyID, Weight> = committee
        .voting_rights
        .iter()
        .map(|(name, stake)| {
            let tangible_party_id = authority_name_to_party_id_from_committee(committee, name)?;
            let weight: Weight = (*stake).try_into().expect("should never have more than 2^16 stake units");

            Ok((
                tangible_party_id,
                weight,
            ))
        })
        .collect::<DwalletMPCResult<HashMap<PartyID, Weight>>>()?;
    let threshold: PartyID = committee.quorum_threshold().try_into().expect("should never have more than 2^16 parties");

    // TODO: use error directly
    WeightedThresholdAccessStructure::new(threshold, party_to_weight)
        .map_err(|e| DwalletMPCError::TwoPCMPCError(e.to_string()))}

/// Convert a given `party_id` to it's corresponding authority name (address).
pub(crate) fn party_id_to_authority_name(
    party_id: PartyID,
    epoch_store: &AuthorityPerEpochStore,
) -> DwalletMPCResult<AuthorityName> {
    // A tangible party ID is of type `PartyID` and in the range `1..=number_of_tangible_parties`.
    // Convert it to an index to the committee authority names, which is in the range `0..number_of_tangible_parties`,
    // Decrement the index to transform it from 1-based to 0-based.
    // Safe to decrement as `PartyID` is `u16`, will never overflow.
    let index = u32::from(party_id) - 1;

    let authority_name = *epoch_store
        .committee()
        .authority_by_index(index)
        .ok_or(DwalletMPCError::AuthorityIndexNotFound(party_id - 1))?;

    Ok(authority_name)
}

/// Convert a given [`Vec<PartyID>`] to the corresponding [`Vec<AuthorityName>`].
pub(crate) fn party_ids_to_authority_names(
    party_ids: &[PartyID],
    epoch_store: &AuthorityPerEpochStore,
) -> DwalletMPCResult<Vec<AuthorityName>> {
    party_ids
        .iter()
        .map(|party_id| party_id_to_authority_name(*party_id, epoch_store))
        .collect::<DwalletMPCResult<Vec<AuthorityName>>>()
}

/// The type of the event is different when we receive an emitted event and when we
/// fetch the event's the dynamic field directly from Sui.
/// This function first tried to deserialize the event as a [`DWalletSessionEvent`], and if it fails,
/// it tries to deserialize it as a [`Field<ID, DWalletSessionEvent<T>>`].
fn deserialize_event_or_dynamic_field<T: DeserializeOwned + DWalletSessionEventTrait>(
    event_contents: &[u8],
) -> Result<DWalletSessionEvent<T>, bcs::Error> {
    bcs::from_bytes::<DWalletSessionEvent<T>>(event_contents).or_else(|_| {
        bcs::from_bytes::<Field<ID, DWalletSessionEvent<T>>>(event_contents)
            .map(|field| field.value)
    })
}

/// Parses the session info from the event and returns it.
/// Return `None` if the event is not a DWallet MPC event.
pub(crate) fn session_info_from_event(
    event: DBSuiEvent,
    packages_config: &IkaPackagesConfig,
) -> anyhow::Result<Option<SessionInfo>> {
    match &event.type_ {
        t if t
            == &DWalletSessionEvent::<DWalletImportedKeyVerificationRequestEvent>::type_(
                packages_config,
            ) =>
        {
            Ok(Some(
                dwallet_imported_key_verification_request_event_session_info(
                    deserialize_event_or_dynamic_field::<DWalletImportedKeyVerificationRequestEvent>(
                        &event.contents,
                    )?,
                ),
            ))
        }
        t if t
            == &DWalletSessionEvent::<MakeDWalletUserSecretKeySharesPublicRequestEvent>::type_(
                packages_config,
            ) =>
        {
            Ok(Some(
                make_dwallet_user_secret_key_shares_public_request_event_session_info(
                    deserialize_event_or_dynamic_field::<
                        MakeDWalletUserSecretKeySharesPublicRequestEvent,
                    >(&event.contents)?,
                ),
            ))
        }
        t if t
            == &DWalletSessionEvent::<DWalletDKGFirstRoundRequestEvent>::type_(packages_config) =>
        {
            Ok(Some(dkg_first_party_session_info(
                deserialize_event_or_dynamic_field::<DWalletDKGFirstRoundRequestEvent>(
                    &event.contents,
                )?,
            )?))
        }
        t if t
            == &DWalletSessionEvent::<DWalletDKGSecondRoundRequestEvent>::type_(
                packages_config,
            ) =>
        {
            Ok(Some(dkg_second_party_session_info(
                deserialize_event_or_dynamic_field::<DWalletDKGSecondRoundRequestEvent>(
                    &event.contents,
                )?,
            )))
        }
        t if t == &DWalletSessionEvent::<PresignRequestEvent>::type_(packages_config) => {
            let deserialized_event: DWalletSessionEvent<PresignRequestEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            Ok(Some(presign_party_session_info(deserialized_event)))
        }
        t if t == &DWalletSessionEvent::<SignRequestEvent>::type_(packages_config) => {
            let deserialized_event: DWalletSessionEvent<SignRequestEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            Ok(Some(sign_party_session_info(&deserialized_event)))
        }
        t if t == &DWalletSessionEvent::<FutureSignRequestEvent>::type_(packages_config) => {
            let deserialized_event: DWalletSessionEvent<FutureSignRequestEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            Ok(Some(get_verify_partial_signatures_session_info(
                &deserialized_event,
            )))
        }
        t if t
            == &DWalletSessionEvent::<DWalletNetworkDKGEncryptionKeyRequestEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletSessionEvent<
                DWalletNetworkDKGEncryptionKeyRequestEvent,
            > = deserialize_event_or_dynamic_field(&event.contents)?;
            Ok(Some(network_dkg::network_dkg_session_info(
                deserialized_event,
                DWalletMPCNetworkKeyScheme::Secp256k1,
            )?))
        }
        t if t
            == &DWalletSessionEvent::<DWalletEncryptionKeyReconfigurationRequestEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletSessionEvent<
                DWalletEncryptionKeyReconfigurationRequestEvent,
            > = deserialize_event_or_dynamic_field(&event.contents)?;
            Ok(Some(
                reshare::network_decryption_key_reshare_session_info_from_event(deserialized_event),
            ))
        }
        t if t
            == &DWalletSessionEvent::<EncryptedShareVerificationRequestEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletSessionEvent<EncryptedShareVerificationRequestEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            Ok(Some(start_encrypted_share_verification_session_info(
                deserialized_event,
            )))
        }
        _ => Ok(None),
    }
}

fn start_encrypted_share_verification_session_info(
    deserialized_event: DWalletSessionEvent<EncryptedShareVerificationRequestEvent>,
) -> SessionInfo {
    SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::EncryptedShareVerification(deserialized_event),
    }
}

fn dkg_first_public_input(
    protocol_public_parameters: &twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
) -> DwalletMPCResult<<DKGFirstParty as mpc::Party>::PublicInput> {
    <DKGFirstParty as DKGFirstPartyPublicInputGenerator>::generate_public_input(
        protocol_public_parameters.clone(),
    )
}

fn make_dwallet_user_secret_key_shares_public_request_event_session_info(
    deserialized_event: DWalletSessionEvent<MakeDWalletUserSecretKeySharesPublicRequestEvent>,
) -> SessionInfo {
    SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::MakeDWalletUserSecretKeySharesPublicRequest(
            deserialized_event,
        ),
    }
}

fn dwallet_imported_key_verification_request_event_session_info(
    deserialized_event: DWalletSessionEvent<DWalletImportedKeyVerificationRequestEvent>,
) -> SessionInfo {
    SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::DWalletImportedKeyVerificationRequest(deserialized_event),
    }
}

fn dkg_first_party_session_info(
    deserialized_event: DWalletSessionEvent<DWalletDKGFirstRoundRequestEvent>,
) -> anyhow::Result<SessionInfo> {
    Ok(SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::DKGFirst(deserialized_event),
    })
}

fn dkg_second_public_input(
    deserialized_event: &DWalletDKGSecondRoundRequestEvent,
    protocol_public_parameters: twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
) -> DwalletMPCResult<<DKGSecondParty as mpc::Party>::PublicInput> {
    <DKGSecondParty as DKGSecondPartyPublicInputGenerator>::generate_public_input(
        protocol_public_parameters,
        deserialized_event.first_round_output.clone(),
        deserialized_event
            .centralized_public_key_share_and_proof
            .clone(),
    )
}

fn dkg_second_party_session_info(
    deserialized_event: DWalletSessionEvent<DWalletDKGSecondRoundRequestEvent>,
) -> SessionInfo {
    SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        mpc_round: MPCProtocolInitData::DKGSecond(deserialized_event.clone()),

        epoch: deserialized_event.epoch,
    }
}

pub(crate) fn presign_public_input(
    session_identifier: SessionIdentifier,
    deserialized_event: PresignRequestEvent,
    protocol_public_parameters: twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
) -> DwalletMPCResult<<PresignParty as mpc::Party>::PublicInput> {
    <PresignParty as PresignPartyPublicInputGenerator>::generate_public_input(
        protocol_public_parameters,
        // TODO: IMPORTANT: for global presign for schnorr / eddsa signature where the presign is not per dWallet - change the code to support it (remove unwrap).
        deserialized_event.dwallet_public_output.clone().ok_or(
            DwalletMPCError::MPCSessionError {
                session_identifier,
                error: "presign public input cannot be None as we only support ECDSA".to_string(),
            },
        )?,
    )
}

fn presign_party_session_info(
    deserialized_event: DWalletSessionEvent<PresignRequestEvent>,
) -> SessionInfo {
    SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::Presign(deserialized_event),
    }
}

fn get_expected_decrypters(
    epoch_store: Arc<AuthorityPerEpochStore>,
    session_identifier: SessionIdentifier,
) -> DwalletMPCResult<HashSet<PartyID>> {
    let committee = epoch_store.committee();
    let total_votes = committee.total_votes();
    let mut shuffled_committee = committee.shuffle_by_stake_from_seed(session_identifier);
    let weighted_threshold_access_structure =
        epoch_store.get_weighted_threshold_access_structure()?;
    let expected_decrypters_votes = weighted_threshold_access_structure.threshold as u32
        + (total_votes as f64 * 0.10).floor() as u32;
    let mut votes_sum = 0;
    let mut expected_decrypters = vec![];
    while votes_sum < expected_decrypters_votes {
        let authority_name = shuffled_committee.pop().unwrap();
        let authority_index = epoch_store.authority_name_to_party_id(&authority_name)?;
        votes_sum += weighted_threshold_access_structure.party_to_weight[&authority_index] as u32;
        expected_decrypters.push(authority_index);
    }
    Ok(expected_decrypters
        .into_iter()
        .collect::<HashSet<PartyID>>())
}

fn sign_session_public_input(
    deserialized_event: &DWalletSessionEvent<SignRequestEvent>,
    dwallet_mpc_manager: &DWalletMPCManager,
    protocol_public_parameters: twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
) -> DwalletMPCResult<<SignFirstParty as mpc::Party>::PublicInput> {
    let decryption_pp = dwallet_mpc_manager.get_decryption_key_share_public_parameters(
        // The `StartSignRoundEvent` is assign with a Secp256k1 dwallet.
        // Todo (#473): Support generic network key scheme
        &deserialized_event
            .event_data
            .dwallet_network_decryption_key_id,
    )?;

    let expected_decrypters = get_expected_decrypters(
        dwallet_mpc_manager.epoch_store()?,
        deserialized_event.session_identifier_digest(),
    )?;

    <SignFirstParty as SignPartyPublicInputGenerator>::generate_public_input(
        protocol_public_parameters,
        deserialized_event
            .event_data
            .dwallet_decentralized_public_output
            .clone(),
        bcs::to_bytes(
            &message_digest(
                &deserialized_event.event_data.message.clone(),
                &Hash::try_from(deserialized_event.event_data.hash_scheme)
                    .map_err(|e| DwalletMPCError::SignatureVerificationFailed(e.to_string()))?,
            )
            .map_err(|e| DwalletMPCError::SignatureVerificationFailed(e.to_string()))?,
        )?,
        deserialized_event.event_data.presign.clone(),
        deserialized_event
            .event_data
            .message_centralized_signature
            .clone(),
        decryption_pp,
        expected_decrypters,
    )
}

fn sign_party_session_info(
    deserialized_event: &DWalletSessionEvent<SignRequestEvent>,
) -> SessionInfo {
    SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::Sign(deserialized_event.clone()),
    }
}

fn get_verify_partial_signatures_session_info(
    deserialized_event: &DWalletSessionEvent<FutureSignRequestEvent>,
) -> SessionInfo {
    SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::PartialSignatureVerification(deserialized_event.clone()),
    }
}

#[allow(unused)]
fn calculate_total_voting_weight(
    weighted_parties: &HashMap<PartyID, Weight>,
    parties: &HashSet<PartyID>,
) -> usize {
    let mut total_voting_weight = 0;
    for party in parties {
        if let Some(weight) = weighted_parties.get(party) {
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
    serialized_messages: HashMap<usize, HashMap<PartyID, MPCMessage>>,
    public_input: &P::PublicInput,
    private_input: P::PrivateInput,
    logger: &MPCSessionLogger,
) -> DwalletMPCResult<
    mpc::AsynchronousRoundResult<MPCMessage, MPCPrivateOutput, SerializedWrappedMPCPublicOutput>,
> {
    let DeserializeMPCMessagesResponse {
        messages,
        malicious_parties,
    } = deserialize_mpc_messages(&serialized_messages);

    // Update logger with malicious parties detected during deserialization.
    let logger = logger.clone().with_malicious_parties(malicious_parties);

    logger.write_logs_to_disk(session_id, party_id, access_threshold, &serialized_messages);

    // When a `ThresholdNotReached` error is received, the system now waits for additional messages
    // (including those from previous rounds) and retries.
    let res = match P::advance_with_guaranteed_output(
        session_id,
        party_id,
        access_threshold,
        messages.clone(),
        Some(private_input),
        public_input,
        &mut rand_core::OsRng,
    ) {
        Ok(res) => res,
        Err(e) => {
            let general_error = DwalletMPCError::TwoPCMPCError(format!(
                "MPC error in party {party_id} session {} at round #{} {:?}",
                session_id,
                messages.len() + 1,
                e
            ));
            return match e.into() {
                // No threshold was reached, so we can't proceed.
                mpc::Error::ThresholdNotReached => {
                    return Err(DwalletMPCError::TWOPCMPCThresholdNotReached)
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
            let private_output = bcs::to_bytes(&private_output)?;
            mpc::AsynchronousRoundResult::Finalize {
                malicious_parties,
                private_output,
                public_output: bcs::to_bytes(&public_output)?,
            }
        }
    })
}

struct DeserializeMPCMessagesResponse<M: DeserializeOwned + Clone> {
    /// round -> {party -> message}
    messages: HashMap<usize, HashMap<PartyID, M>>,
    #[allow(dead_code)]
    malicious_parties: Vec<PartyID>,
}

/// Deserializes the messages received from other parties for the next advancement.
/// Any value that fails to deserialize is considered to be sent by a malicious party.
/// Returns the deserialized messages or an error including the IDs of the malicious parties.
fn deserialize_mpc_messages<M: DeserializeOwned + Clone>(
    messages: &HashMap<usize, HashMap<PartyID, MPCMessage>>,
) -> DeserializeMPCMessagesResponse<M> {
    let mut deserialized_results = HashMap::new();
    let mut malicious_parties = Vec::new();

    for (index, message_batch) in messages.iter() {
        let mut valid_messages = HashMap::new();

        for (party_id, message) in message_batch {
            match bcs::from_bytes::<M>(message) {
                Ok(value) => {
                    valid_messages.insert(*party_id, value);
                }
                Err(e) => {
                    tracing::error!(
                        party_id=?party_id,
                        error=?e,
                        "malicious party detected — failed to deserialize a message from party"
                    );
                    malicious_parties.push(*party_id);
                }
            }
        }

        if !valid_messages.is_empty() {
            deserialized_results.insert(*index, valid_messages);
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
pub(super) async fn session_input_from_event(
    event: DBSuiEvent,
    dwallet_mpc_manager: &DWalletMPCManager,
) -> DwalletMPCResult<(PublicInput, MPCPrivateInput)> {
    let packages_config = &dwallet_mpc_manager.epoch_store()?.packages_config;
    match &event.type_ {
        t if t
            == &DWalletSessionEvent::<DWalletImportedKeyVerificationRequestEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletSessionEvent<
                DWalletImportedKeyVerificationRequestEvent,
            > = deserialize_event_or_dynamic_field(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                &deserialized_event
                    .event_data
                    .dwallet_network_encryption_key_id,
            )?;
            let dwallet_id = CommitmentSizedNumber::from_le_slice(
                deserialized_event.event_data.dwallet_id.to_vec().as_slice(),
            );
            let VersionedImportedDWalletPublicOutput::V1(centralized_party_message) =
                bcs::from_bytes(&deserialized_event.event_data.centralized_party_message)?;
            let public_input = (
                protocol_public_parameters,
                dwallet_id,
                bcs::from_bytes(&centralized_party_message)?,
            )
                .into();
            Ok((
                PublicInput::DWalletImportedKeyVerificationRequest(public_input),
                None,
            ))
        }
        t if t
            == &DWalletSessionEvent::<MakeDWalletUserSecretKeySharesPublicRequestEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletSessionEvent<
                MakeDWalletUserSecretKeySharesPublicRequestEvent,
            > = bcs::from_bytes(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                &deserialized_event
                    .event_data
                    .dwallet_network_decryption_key_id,
            )?;
            Ok((
                PublicInput::MakeDWalletUserSecretKeySharesPublicPublicInput(
                    protocol_public_parameters,
                ),
                None,
            ))
        }
        t if t
            == &DWalletSessionEvent::<DWalletNetworkDKGEncryptionKeyRequestEvent>::type_(
                packages_config,
            ) =>
        {
            let class_groups_key_pair_and_proof = dwallet_mpc_manager
                .node_config
                .class_groups_key_pair_and_proof
                .clone();
            let class_groups_key_pair_and_proof = class_groups_key_pair_and_proof
                .ok_or(DwalletMPCError::ClassGroupsKeyPairNotFound)?;
            Ok((
                PublicInput::NetworkEncryptionKeyDkg(network_dkg::network_dkg_public_input(
                    &dwallet_mpc_manager
                        .epoch_store()?
                        .get_weighted_threshold_access_structure()?,
                    dwallet_mpc_manager
                        .validators_class_groups_public_keys_and_proofs
                        .clone(),
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                )?),
                Some(bcs::to_bytes(
                    &class_groups_key_pair_and_proof
                        .class_groups_keypair()
                        .decryption_key(),
                )?),
            ))
        }
        t if t
            == &DWalletSessionEvent::<DWalletEncryptionKeyReconfigurationRequestEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletSessionEvent<
                DWalletEncryptionKeyReconfigurationRequestEvent,
            > = deserialize_event_or_dynamic_field(&event.contents)?;
            let class_groups_key_pair_and_proof = dwallet_mpc_manager
                .node_config
                .class_groups_key_pair_and_proof
                .clone();
            let class_groups_key_pair_and_proof = class_groups_key_pair_and_proof
                .ok_or(DwalletMPCError::ClassGroupsKeyPairNotFound)?;
            Ok((
                PublicInput::NetworkEncryptionKeyReconfiguration(<ReshareSecp256k1Party as ResharePartyPublicInputGenerator>::generate_public_input(
                    dwallet_mpc_manager.epoch_store()?.committee().as_ref(),
                    dwallet_mpc_manager.must_get_next_active_committee().await,
                    dwallet_mpc_manager.get_decryption_key_share_public_parameters(
                        &deserialized_event
                            .event_data
                            .dwallet_network_decryption_key_id,
                    )?,
                    dwallet_mpc_manager
                        .get_network_dkg_public_output(
                            &deserialized_event
                                .event_data
                                .dwallet_network_decryption_key_id,
                        )
                        .await?,
                )?),
                Some(bcs::to_bytes(
                    &class_groups_key_pair_and_proof
                        .class_groups_keypair()
                        .decryption_key(),
                )?),
            ))
        }
        t if t
            == &DWalletSessionEvent::<DWalletDKGFirstRoundRequestEvent>::type_(packages_config) =>
        {
            let deserialized_event: DWalletSessionEvent<DWalletDKGFirstRoundRequestEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                &deserialized_event
                    .event_data
                    .dwallet_network_decryption_key_id,
            )?;
            Ok((
                PublicInput::DKGFirst(dkg_first_public_input(&protocol_public_parameters)?),
                None,
            ))
        }
        t if t
            == &DWalletSessionEvent::<DWalletDKGSecondRoundRequestEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletSessionEvent<DWalletDKGSecondRoundRequestEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                &deserialized_event
                    .event_data
                    .dwallet_network_decryption_key_id,
            )?;
            Ok((
                PublicInput::DKGSecond(dkg_second_public_input(
                    &deserialized_event.event_data,
                    protocol_public_parameters,
                )?),
                None,
            ))
        }
        t if t == &DWalletSessionEvent::<PresignRequestEvent>::type_(packages_config) => {
            let deserialized_event: DWalletSessionEvent<PresignRequestEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                &deserialized_event
                    .event_data
                    .dwallet_network_decryption_key_id,
            )?;
            Ok((
                PublicInput::Presign(presign_public_input(
                    deserialized_event.session_identifier_digest(),
                    deserialized_event.event_data,
                    protocol_public_parameters,
                )?),
                None,
            ))
        }
        t if t == &DWalletSessionEvent::<SignRequestEvent>::type_(packages_config) => {
            let deserialized_event: DWalletSessionEvent<SignRequestEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                &deserialized_event
                    .event_data
                    .dwallet_network_decryption_key_id,
            )?;
            Ok((
                PublicInput::Sign(sign_session_public_input(
                    &deserialized_event,
                    dwallet_mpc_manager,
                    protocol_public_parameters,
                )?),
                None,
            ))
        }
        t if t
            == &DWalletSessionEvent::<EncryptedShareVerificationRequestEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletSessionEvent<EncryptedShareVerificationRequestEvent> =
                bcs::from_bytes(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                &deserialized_event
                    .event_data
                    .dwallet_network_decryption_key_id,
            )?;
            Ok((
                PublicInput::EncryptedShareVerification(protocol_public_parameters),
                None,
            ))
        }
        t if t == &DWalletSessionEvent::<FutureSignRequestEvent>::type_(packages_config) => {
            let deserialized_event: DWalletSessionEvent<FutureSignRequestEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;
            let protocol_public_parameters = dwallet_mpc_manager.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                &deserialized_event
                    .event_data
                    .dwallet_network_decryption_key_id,
            )?;
            Ok((
                PublicInput::PartialSignatureVerification(protocol_public_parameters),
                None,
            ))
        }
        _ => Err(DwalletMPCError::NonMPCEvent(event.type_.name.to_string())),
    }
}

// TODO (#683): Parse the network key version from the network key object ID
#[allow(unused)]
pub(crate) fn network_key_version_from_key_id(_key_id: &ObjectID) -> u8 {
    0
}

/// A struct to encapsulate MPC session logging parameters and functionality.
/// This separates logging-specific concerns from the core MPC advancement logic.
#[derive(Default, Clone)]
pub(crate) struct MPCSessionLogger {
    /// The MPC protocol name for logging purposes
    pub mpc_protocol_name: Option<String>,
    /// Mapping from party IDs to authority names for logging
    pub party_to_authority_map: Option<HashMap<PartyID, AuthorityName>>,
    /// Encoded class groups key pair and proof for logging
    pub encoded_class_groups_key_pair_and_proof: Option<MPCPrivateInput>,
    /// Decryption key shares for logging
    pub decryption_key_shares: Option<HashMap<PartyID, SecretKeyShareSizedInteger>>,
    /// Malicious parties detected during message processing
    pub malicious_parties: Option<Vec<PartyID>>,
}

impl MPCSessionLogger {
    /// Creates a new MPCSessionLogger with the provided parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the MPC protocol name
    pub fn with_protocol_name(mut self, name: String) -> Self {
        self.mpc_protocol_name = Some(name);
        self
    }

    /// Sets the party to authority mapping
    pub fn with_party_to_authority_map(mut self, map: HashMap<PartyID, AuthorityName>) -> Self {
        self.party_to_authority_map = Some(map);
        self
    }

    /// Sets the encoded class groups key pair and proof
    pub fn with_class_groups_key_pair_and_proof(mut self, proof: MPCPrivateInput) -> Self {
        self.encoded_class_groups_key_pair_and_proof = Some(proof);
        self
    }

    /// Sets the decryption key shares
    pub fn with_decryption_key_shares(
        mut self,
        shares: HashMap<PartyID, SecretKeyShareSizedInteger>,
    ) -> Self {
        self.decryption_key_shares = Some(shares);
        self
    }

    /// Sets the malicious parties
    pub fn with_malicious_parties(mut self, parties: Vec<PartyID>) -> Self {
        self.malicious_parties = Some(parties);
        self
    }

    /// Writes MPC session logs to disk if logging is enabled
    pub fn write_logs_to_disk(
        &self,
        session_id: CommitmentSizedNumber,
        party_id: PartyID,
        access_threshold: &WeightedThresholdAccessStructure,
        messages: &HashMap<usize, HashMap<PartyID, MPCMessage>>,
    ) {
        if std::env::var("IKA_WRITE_MPC_SESSION_LOGS_TO_DISK").unwrap_or_default() != "1" {
            return;
        }

        warn!("Writing MPC session logs to disk");

        // Determine round number
        let round = messages.len();

        // Get (and initialize once) the log directory
        let log_dir = match self.get_log_dir() {
            Ok(dir) => dir,
            Err(err) => {
                warn!(?err, "Failed to get the logs directory");
                return;
            }
        };
        let filename = format!("session_{}_round_{}.json", session_id, round);
        let path = log_dir.join(&filename);

        // Serialize to JSON.
        let log = json!({
            "session_id": session_id,
            "round": round,
            "party_id": party_id,
            "access_threshold": access_threshold,
            "messages": messages,
            "mpc_protocol": self.mpc_protocol_name,
            "party_to_authority_map": self.party_to_authority_map,
            "class_groups_key_pair_and_proof": self.encoded_class_groups_key_pair_and_proof,
            "decryption_key_shares": self.decryption_key_shares,
            "malicious_parties": self.malicious_parties,
        });

        let mut file = match File::create(&path) {
            Ok(f) => f,
            Err(e) => {
                warn!("Failed to create log file {}: {}", path.display(), e);
                return;
            }
        };
        if let Err(e) = file.write_all(log.to_string().as_bytes()) {
            warn!("Failed to write to the log file {}: {}", path.display(), e);
        }
    }

    fn get_log_dir(&self) -> Result<&'static PathBuf, DwalletMPCError> {
        if let Some(dir) = LOG_DIR.get() {
            return Ok(dir);
        }

        // Otherwise, attempt creation
        const PRIMARY: &str = "/opt/ika/db/mpclogs/logs";
        const FALLBACK: &str = "/tmp/mpclogs/logs";

        let chosen = if fs::create_dir_all(PRIMARY).is_ok() {
            PRIMARY
        } else {
            // Primary failed → try fallback (propagate error if that fails).
            fs::create_dir_all(FALLBACK).map_err(|e| {
                DwalletMPCError::TwoPCMPCError(format!(
                    "Failed to create a fallback log directory {}: {}",
                    FALLBACK, e
                ))
            })?;
            FALLBACK
        };

        // Insert into our OnceLock (this only ever succeeds once).
        let pathbuf = PathBuf::from(chosen);
        LOG_DIR.set(pathbuf).map_err(|_| {
            DwalletMPCError::TwoPCMPCError("failed to set a global log directory".into())
        })?;

        // Safe to unwrap — we just set it
        Ok(LOG_DIR.get().unwrap())
    }
}
