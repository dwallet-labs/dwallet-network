use class_groups::reconfiguration::{PublicInput, Secp256k1Party};
use class_groups::DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER;
use dwallet_classgroups_types::ClassGroupsEncryptionKeyAndProof;
use dwallet_mpc_types::dwallet_mpc::MPCPublicInput;
use group::{secp256k1, GroupElement, PartyID};
use ika_types::committee::Committee;
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{
    DWalletDecryptionKeyReshareRequestEvent, DWalletMPCSuiEvent, MPCProtocolInitData, SessionInfo,
};
use mpc::{Party, Weight, WeightedThresholdAccessStructure};
use std::collections::HashMap;
use twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters;

pub(super) type ReshareSecp256k1Party = Secp256k1Party;

pub(super) trait ResharePartyPublicInputGenerator: Party {
    /// Generates the public input required for the reshare protocol.
    fn generate_public_input(
        committee: &Committee,
        new_committee: Committee,
        protocol_public_parameters: Vec<u8>,
        decryption_key_share_public_parameters: Vec<u8>,
    ) -> DwalletMPCResult<MPCPublicInput>;
}

fn authority_name_to_party_id_from_committee(
    committee: &Committee,
    authority_name: &AuthorityName,
) -> DwalletMPCResult<PartyID> {
    committee
        .authority_index(authority_name)
        // Need to add 1 because the authority index is 0-based,
        // and the twopc_mpc library uses 1-based party IDs.
        .map(|index| (index + 1) as PartyID)
        .ok_or_else(|| DwalletMPCError::AuthorityNameNotFound(*authority_name))
}

fn current_tangible_party_id_to_upcoming(
    current_committee: Committee,
    upcoming_committee: Committee,
) -> HashMap<PartyID, Option<PartyID>> {
    current_committee
        .voting_rights
        .iter()
        .map(|(name, _)| {
            // Safe to unwrap because we know the name is in the current committee.
            let current_party_id =
                authority_name_to_party_id_from_committee(&current_committee, name).unwrap();
            let upcoming_party_id =
                authority_name_to_party_id_from_committee(&upcoming_committee, name).ok();
            (current_party_id, upcoming_party_id)
        })
        .collect()
}

impl ResharePartyPublicInputGenerator for ReshareSecp256k1Party {
    fn generate_public_input(
        current_committee: &Committee,
        upcoming_committee: Committee,
        protocol_public_parameters: Vec<u8>,
        decryption_key_share_public_parameters: Vec<u8>,
    ) -> DwalletMPCResult<MPCPublicInput> {
        let current_committee = current_committee.clone();
        let quorum_threshold = current_committee.quorum_threshold();
        let weighted_parties: HashMap<PartyID, Weight> = current_committee
            .voting_rights
            .iter()
            .map(|(name, weight)| {
                Ok((
                    authority_name_to_party_id_with_committee(&current_committee, name)?,
                    *weight as Weight,
                ))
            })
            .collect::<DwalletMPCResult<HashMap<PartyID, Weight>>>()?;

        let current_access_structure =
            create_access_structure(&current_committee, quorum_threshold)?;
        let upcoming_access_structure =
            create_access_structure(&upcoming_committee, upcoming_committee.quorum_threshold())?;

        let plaintext_space_public_parameters = secp256k1::scalar::PublicParameters::default();

        let current_encryption_keys_per_crt_prime_and_proofs =
            extract_encryption_keys_from_committee(&current_committee)?;

        let upcoming_encryption_keys_per_crt_prime_and_proofs =
            extract_encryption_keys_from_committee(&upcoming_committee)?;

        let protocol_public_parameters =
            bcs::from_bytes::<ProtocolPublicParameters>(&protocol_public_parameters)?;
        let encryption_scheme_public_parameters = protocol_public_parameters
            .encryption_scheme_public_parameters
            .encryption_key
            .value();

        let public_input: <ReshareSecp256k1Party as Party>::PublicInput = PublicInput::new::<
            secp256k1::GroupElement,
        >(
            &current_access_structure,
            upcoming_access_structure,
            plaintext_space_public_parameters.clone(),
            current_encryption_keys_per_crt_prime_and_proofs.clone(),
            upcoming_encryption_keys_per_crt_prime_and_proofs.clone(),
            bcs::from_bytes(&decryption_key_share_public_parameters)?,
            DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
            current_tangible_party_id_to_upcoming(current_committee, upcoming_committee).clone(),
            encryption_scheme_public_parameters,
        )
        .map_err(|e| {
            DwalletMPCError::TwoPCMPCError("failed to generate public input".to_string())
        })?;

        Ok(bcs::to_bytes(&public_input)?)
    }
}

pub(super) fn network_decryption_key_reshare_session_info_from_event(
    deserialized_event: DWalletMPCSuiEvent<DWalletDecryptionKeyReshareRequestEvent>,
) -> SessionInfo {
    SessionInfo {
        sequence_number: deserialized_event.session_sequence_number,
        session_id: deserialized_event.session_id,
        mpc_round: MPCProtocolInitData::DecryptionKeyReshare(deserialized_event),
        is_immediate: true,
    }
}

fn create_access_structure(
    committee: &Committee,
    quorum_threshold: u64,
) -> DwalletMPCResult<WeightedThresholdAccessStructure> {
    let weighted_parties: HashMap<PartyID, Weight> = committee
        .voting_rights
        .iter()
        .map(|(name, weight)| {
            Ok((
                authority_name_to_party_id_from_committee(committee, name)?,
                *weight as Weight,
            ))
        })
        .collect::<DwalletMPCResult<HashMap<PartyID, Weight>>>()?;

    WeightedThresholdAccessStructure::new(quorum_threshold as PartyID, weighted_parties)
        .map_err(|e| DwalletMPCError::TwoPCMPCError(e.to_string()))
}

fn extract_encryption_keys_from_committee(
    committee: &Committee,
) -> DwalletMPCResult<HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>> {
    committee
        .class_groups_public_keys_and_proofs
        .iter()
        .map(|(name, key)| {
            let party_id = authority_name_to_party_id_from_committee(committee, name)?;
            let key = bcs::from_bytes(key)?;
            Ok((party_id, key))
        })
        .collect::<DwalletMPCResult<HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>>>()
}
