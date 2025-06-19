use crate::dwallet_mpc::deserialize_event_or_dynamic_field;
use crate::dwallet_mpc::dwallet_dkg::{
    dwallet_dkg_first_public_input, dwallet_dkg_second_public_input,
};
use crate::dwallet_mpc::mpc_manager::DWalletMPCManager;
use crate::dwallet_mpc::mpc_session::PublicInput;
use crate::dwallet_mpc::network_dkg::network_dkg_public_input;
use crate::dwallet_mpc::presign::presign_public_input;
use crate::dwallet_mpc::reconfiguration::{
    ReconfigurationPartyPublicInputGenerator, ReconfigurationSecp256k1Party,
};
use crate::dwallet_mpc::sign::sign_session_public_input;
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCPrivateInput, VersionedImportedDWalletPublicOutput,
};
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{
    DBSuiEvent, DWalletDKGFirstRoundRequestEvent, DWalletDKGSecondRoundRequestEvent,
    DWalletEncryptionKeyReconfigurationRequestEvent, DWalletImportedKeyVerificationRequestEvent,
    DWalletNetworkDKGEncryptionKeyRequestEvent, DWalletSessionEvent, DWalletSessionEventTrait,
    EncryptedShareVerificationRequestEvent, FutureSignRequestEvent,
    MakeDWalletUserSecretKeySharesPublicRequestEvent, PresignRequestEvent, SignRequestEvent,
};

// TODO (#542): move this logic to run before writing the event to the DB, maybe include within the session info
/// Parses an [`Event`] to extract the corresponding [`MPCParty`],
/// public input, private input and session information.
///
/// Returns an error if the event type does not correspond to any known MPC rounds
/// or if deserialization fails.
pub(crate) async fn public_input_from_event(
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
                PublicInput::NetworkEncryptionKeyDkg(network_dkg_public_input(
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
                    PublicInput::NetworkEncryptionKeyReconfiguration(<ReconfigurationSecp256k1Party as ReconfigurationPartyPublicInputGenerator>::generate_public_input(
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
                PublicInput::DKGFirst(dwallet_dkg_first_public_input(&protocol_public_parameters)?),
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
                PublicInput::DKGSecond(dwallet_dkg_second_public_input(
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
