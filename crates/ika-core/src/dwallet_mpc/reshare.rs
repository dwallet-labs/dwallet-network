use crate::dwallet_mpc::{
    authority_name_to_party_id_from_committee, generate_access_structure_from_committee,
};
use class_groups::reconfiguration::{PublicInput, Secp256k1Party};
use class_groups::{
    Secp256k1DecryptionKeySharePublicParameters, DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
};
use dwallet_classgroups_types::ClassGroupsEncryptionKeyAndProof;
use dwallet_mpc_types::dwallet_mpc::{MPCPublicInput, MPCPublicOutput, SecpNetworkDkgOutputVersion};
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
        decryption_key_share_public_parameters: Vec<u8>,
        network_dkg_public_output: SecpNetworkDkgOutputVersion,
    ) -> DwalletMPCResult<MPCPublicInput>;
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

impl ResharePartyPublicInputGenerator for ReshareSecp256k1Party {
    fn generate_public_input(
        current_committee: &Committee,
        upcoming_committee: Committee,
        decryption_key_share_public_parameters: Vec<u8>,
        network_dkg_public_output: MPCPublicOutput,
    ) -> DwalletMPCResult<MPCPublicInput> {
        let network_dkg_public_output = match network_dkg_public_output {
            MPCPublicOutput::V1(output) => output,
        };
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

        let public_input: <ReshareSecp256k1Party as Party>::PublicInput = PublicInput::new::<
            secp256k1::GroupElement,
        >(
            &current_access_structure,
            upcoming_access_structure,
            plaintext_space_public_parameters.clone(),
            current_encryption_keys_per_crt_prime_and_proofs.clone(),
            upcoming_encryption_keys_per_crt_prime_and_proofs.clone(),
            bcs::from_bytes::<Secp256k1DecryptionKeySharePublicParameters>(
                &decryption_key_share_public_parameters,
            )?,
            DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
            current_tangible_party_id_to_upcoming(current_committee, upcoming_committee).clone(),
            bcs::from_bytes(&network_dkg_public_output)?,
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
        session_type: deserialized_event.session_type.clone(),
        session_id: deserialized_event.session_id,
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::DecryptionKeyReshare(deserialized_event),
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
            let key = bcs::from_bytes(key)?;
            Ok((party_id, key))
        })
        .collect::<DwalletMPCResult<HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>>>()
}
