// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::dwallet_mpc::{
    authority_name_to_party_id_from_committee, generate_access_structure_from_committee,
};
use class_groups::reconfiguration::{PublicInput, Secp256k1Party};
use class_groups::{
    DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER, Secp256k1DecryptionKeySharePublicParameters,
};
use dwallet_mpc_types::dwallet_mpc::VersionedNetworkDkgOutput;
use group::{PartyID, secp256k1};
use ika_types::committee::ClassGroupsEncryptionKeyAndProof;
use ika_types::committee::Committee;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{
    DWalletEncryptionKeyReconfigurationRequestEvent, DWalletSessionEvent, MPCRequestInput,
    MPCSessionRequest,
};
use mpc::Party;
use std::collections::HashMap;

pub(crate) type ReconfigurationSecp256k1Party = Secp256k1Party;

pub(crate) trait ReconfigurationPartyPublicInputGenerator: Party {
    /// Generates the public input required for the reconfiguration protocol.
    fn generate_public_input(
        committee: &Committee,
        new_committee: Committee,
        decryption_key_share_public_parameters: Secp256k1DecryptionKeySharePublicParameters,
        network_dkg_public_output: VersionedNetworkDkgOutput,
    ) -> DwalletMPCResult<<ReconfigurationSecp256k1Party as mpc::Party>::PublicInput>;
}

fn current_tangible_party_id_to_upcoming(
    current_committee: Committee,
    upcoming_committee: Committee,
) -> HashMap<PartyID, Option<PartyID>> {
    current_committee
        .voting_rights
        .iter()
        .map(|(name, _)| {
            // Todo (#972): Authority name can change, we need to use real const value for the committee - validator ID
            // Safe to unwrap because we know the name is in the current committee.
            let current_party_id =
                authority_name_to_party_id_from_committee(&current_committee, name).unwrap();
            let upcoming_party_id =
                authority_name_to_party_id_from_committee(&upcoming_committee, name).ok();
            (current_party_id, upcoming_party_id)
        })
        .collect()
}

impl ReconfigurationPartyPublicInputGenerator for ReconfigurationSecp256k1Party {
    fn generate_public_input(
        current_committee: &Committee,
        upcoming_committee: Committee,
        decryption_key_share_public_parameters: Secp256k1DecryptionKeySharePublicParameters,
        network_dkg_public_output: VersionedNetworkDkgOutput,
    ) -> DwalletMPCResult<<ReconfigurationSecp256k1Party as mpc::Party>::PublicInput> {
        let VersionedNetworkDkgOutput::V1(network_dkg_public_output) = network_dkg_public_output;
        let current_committee = current_committee.clone();

        let current_access_structure =
            generate_access_structure_from_committee(&current_committee)?;
        let upcoming_access_structure =
            generate_access_structure_from_committee(&upcoming_committee)?;

        let plaintext_space_public_parameters = secp256k1::scalar::PublicParameters::default();

        let current_encryption_keys_per_crt_prime_and_proofs =
            extract_encryption_keys_from_committee(&current_committee)?;

        let upcoming_encryption_keys_per_crt_prime_and_proofs =
            extract_encryption_keys_from_committee(&upcoming_committee)?;

        let public_input: <ReconfigurationSecp256k1Party as Party>::PublicInput =
            PublicInput::new::<secp256k1::GroupElement>(
                &current_access_structure,
                upcoming_access_structure,
                plaintext_space_public_parameters.clone(),
                current_encryption_keys_per_crt_prime_and_proofs.clone(),
                upcoming_encryption_keys_per_crt_prime_and_proofs.clone(),
                decryption_key_share_public_parameters,
                DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
                current_tangible_party_id_to_upcoming(current_committee, upcoming_committee)
                    .clone(),
                bcs::from_bytes(&network_dkg_public_output)?,
            )
            .map_err(|e| {
                DwalletMPCError::TwoPCMPCError(format!("failed to generate public input: {e:?}"))
            })?;

        Ok(public_input)
    }
}

pub(crate) fn network_decryption_key_reconfiguration_session_request_from_event(
    deserialized_event: DWalletSessionEvent<DWalletEncryptionKeyReconfigurationRequestEvent>,
) -> MPCSessionRequest {
    MPCSessionRequest {
        session_type: deserialized_event.session_type,
        session_identifier: deserialized_event.session_identifier_digest(),
        session_sequence_number: deserialized_event.session_sequence_number,
        epoch: deserialized_event.epoch,
        request_input: MPCRequestInput::NetworkEncryptionKeyReconfiguration(deserialized_event),
        requires_network_key_data: true,
        requires_next_active_committee: true,
    }
}

fn extract_encryption_keys_from_committee(
    committee: &Committee,
) -> DwalletMPCResult<HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>> {
    committee
        .class_groups_public_keys_and_proofs
        .iter()
        .map(|(name, key)| {
            let party_id = authority_name_to_party_id_from_committee(committee, name)?;
            let key = key.clone();
            Ok((party_id, key))
        })
        .collect::<DwalletMPCResult<HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>>>()
}
