use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use group::PartyID;
use ika_types::committee::Committee;
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{DWalletSessionEvent, DWalletSessionEventTrait};
use message_digest::message_digest::message_digest;
use mpc::{Weight, WeightedThresholdAccessStructure};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::vec::Vec;
use sui_types::base_types::{EpochId, ObjectID};
use sui_types::dynamic_field::Field;
use sui_types::id::ID;

mod cryptographic_computations_orchestrator;
pub mod dwallet_mpc_service;
mod malicious_handler;
pub mod mpc_manager;
pub mod mpc_outputs_verifier;
pub mod mpc_session;

pub mod dwallet_mpc_metrics;
mod mpc_protocols;
mod native_computations;

pub(crate) use mpc_protocols::{dwallet_dkg, network_dkg, presign, reconfiguration, sign};
pub(crate) use native_computations::{
    encrypt_user_share, make_dwallet_user_secret_key_shares_public,
};

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
    let tangible_party_id: u32 = authority_index
        .checked_add(1)
        .expect("should never have more than 2^32 parties");
    let tangible_party_id: u16 = tangible_party_id
        .try_into()
        .expect("should never have more than 2^16 parties");

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
            let weight: Weight = (*stake)
                .try_into()
                .expect("should never have more than 2^16 stake units");

            Ok((tangible_party_id, weight))
        })
        .collect::<DwalletMPCResult<HashMap<PartyID, Weight>>>()?;
    let threshold: PartyID = committee
        .quorum_threshold()
        .try_into()
        .expect("should never have more than 2^16 parties");

    // TODO: use error directly
    WeightedThresholdAccessStructure::new(threshold, party_to_weight)
        .map_err(|e| DwalletMPCError::TwoPCMPCError(e.to_string()))
}

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

// TODO (#683): Parse the network key version from the network key object ID
#[allow(unused)]
pub(crate) fn network_key_version_from_key_id(_key_id: &ObjectID) -> u8 {
    0
}
