use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::deserialize_event_or_dynamic_field;
use crate::dwallet_mpc::dwallet_dkg::{
    dwallet_dkg_first_public_input, dwallet_dkg_second_public_input,
};
use crate::dwallet_mpc::mpc_session::PublicInput;
use crate::dwallet_mpc::network_dkg::{network_dkg_public_input, DwalletMPCNetworkKeys};
use crate::dwallet_mpc::presign::presign_public_input;
use crate::dwallet_mpc::reconfiguration::{
    ReconfigurationPartyPublicInputGenerator, ReconfigurationSecp256k1Party,
};
use crate::dwallet_mpc::sign::sign_session_public_input;
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCPrivateInput, VersionedImportedDWalletPublicOutput,
};
use group::PartyID;
use ika_types::committee::{ClassGroupsEncryptionKeyAndProof, Committee};
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{
    DBSuiEvent, DWalletDKGFirstRoundRequestEvent, DWalletDKGSecondRoundRequestEvent,
    DWalletEncryptionKeyReconfigurationRequestEvent, DWalletImportedKeyVerificationRequestEvent,
    DWalletNetworkDKGEncryptionKeyRequestEvent, DWalletSessionEvent, DWalletSessionEventTrait,
    EncryptedShareVerificationRequestEvent, FutureSignRequestEvent,
    MakeDWalletUserSecretKeySharesPublicRequestEvent, PresignRequestEvent, SignRequestEvent,
};
use std::collections::HashMap;
use std::sync::Arc;

// TODO (#542): move this logic to run before writing the event to the DB, maybe include within the session info
/// Parses an [`Event`] to extract the corresponding [`MPCParty`],
/// public input, private input and session information.
///
/// Returns an error if the event type does not correspond to any known MPC rounds
/// or if deserialization fails.
pub(crate) fn session_input_from_event(
    event: DBSuiEvent,
    epoch_store: Arc<AuthorityPerEpochStore>,
    network_keys: &Box<DwalletMPCNetworkKeys>,
    next_active_committee: Committee,
    validators_class_groups_public_keys_and_proofs: HashMap<
        PartyID,
        ClassGroupsEncryptionKeyAndProof,
    >,
) -> DwalletMPCResult<(PublicInput, MPCPrivateInput)> {
    let packages_config = &epoch_store.packages_config;
    match &event.type_ {
        t if t
            == &DWalletSessionEvent::<DWalletImportedKeyVerificationRequestEvent>::type_(
                packages_config,
            ) =>
        {
            let deserialized_event: DWalletSessionEvent<
                DWalletImportedKeyVerificationRequestEvent,
            > = deserialize_event_or_dynamic_field(&event.contents)?;

            let protocol_public_parameters = network_keys.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                &deserialized_event
                    .event_data
                    .dwallet_network_encryption_key_id,
            )?;

            let session_id = CommitmentSizedNumber::from_le_slice(
                deserialized_event
                    .session_identifier_digest()
                    .to_vec()
                    .as_slice(),
            );

            let VersionedImportedDWalletPublicOutput::V1(centralized_party_message) =
                bcs::from_bytes(&deserialized_event.event_data.centralized_party_message)?;

            let public_input = (
                protocol_public_parameters,
                session_id,
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

            let protocol_public_parameters = network_keys.get_protocol_public_parameters(
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
            let class_groups_decryption_key = network_keys
                .validator_private_dec_key_data
                .class_groups_decryption_key;

            Ok((
                PublicInput::NetworkEncryptionKeyDkg(network_dkg_public_input(
                    &epoch_store.get_weighted_threshold_access_structure()?,
                    validators_class_groups_public_keys_and_proofs,
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                )?),
                Some(bcs::to_bytes(&class_groups_decryption_key)?),
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

            let class_groups_decryption_key = network_keys
                .validator_private_dec_key_data
                .class_groups_decryption_key;

            Ok((
                    PublicInput::NetworkEncryptionKeyReconfiguration(<ReconfigurationSecp256k1Party as ReconfigurationPartyPublicInputGenerator>::generate_public_input(
                        epoch_store.committee().as_ref(),
                        next_active_committee,
                        network_keys.get_decryption_key_share_public_parameters(
                            &deserialized_event
                                .event_data
                                .dwallet_network_decryption_key_id,
                        )?,
                        network_keys
                            .get_network_dkg_public_output(
                                &deserialized_event
                                    .event_data
                                    .dwallet_network_decryption_key_id,
                            )?,
                    )?),
                    Some(bcs::to_bytes(
                        &class_groups_decryption_key
                    )?),
                ))
        }
        t if t
            == &DWalletSessionEvent::<DWalletDKGFirstRoundRequestEvent>::type_(packages_config) =>
        {
            let deserialized_event: DWalletSessionEvent<DWalletDKGFirstRoundRequestEvent> =
                deserialize_event_or_dynamic_field(&event.contents)?;

            let protocol_public_parameters = network_keys.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme - take curve from event
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

            let protocol_public_parameters = network_keys.get_protocol_public_parameters(
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

            let protocol_public_parameters = network_keys.get_protocol_public_parameters(
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

            let protocol_public_parameters = network_keys.get_protocol_public_parameters(
                // The event is assign with a Secp256k1 dwallet.
                // Todo (#473): Support generic network key scheme
                &deserialized_event
                    .event_data
                    .dwallet_network_decryption_key_id,
            )?;

            Ok((
                PublicInput::Sign(sign_session_public_input(
                    &deserialized_event,
                    epoch_store,
                    network_keys,
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

            let protocol_public_parameters = network_keys.get_protocol_public_parameters(
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

            let protocol_public_parameters = network_keys.get_protocol_public_parameters(
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
