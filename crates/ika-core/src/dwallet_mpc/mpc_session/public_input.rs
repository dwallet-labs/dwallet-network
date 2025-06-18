use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::dwallet_dkg::{
    DWalletDKGFirstParty, DWalletDKGFirstPartyPublicInputGenerator, DWalletDKGSecondParty,
    DWalletDKGSecondPartyPublicInputGenerator,
};
use crate::dwallet_mpc::mpc_manager::DWalletMPCManager;
use crate::dwallet_mpc::mpc_session::PublicInput;
use crate::dwallet_mpc::presign::{PresignParty, PresignPartyPublicInputGenerator};
use crate::dwallet_mpc::reshare::{ResharePartyPublicInputGenerator, ReshareSecp256k1Party};
use crate::dwallet_mpc::sign::{SignFirstParty, SignPartyPublicInputGenerator};
use crate::dwallet_mpc::{deserialize_event_or_dynamic_field, network_dkg};
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCPrivateInput, VersionedImportedDWalletPublicOutput,
};
use group::PartyID;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{
    DBSuiEvent, DWalletDKGFirstRoundRequestEvent, DWalletDKGSecondRoundRequestEvent,
    DWalletEncryptionKeyReconfigurationRequestEvent, DWalletImportedKeyVerificationRequestEvent,
    DWalletNetworkDKGEncryptionKeyRequestEvent, DWalletSessionEvent, DWalletSessionEventTrait,
    EncryptedShareVerificationRequestEvent, FutureSignRequestEvent, MPCProtocolInitData,
    MakeDWalletUserSecretKeySharesPublicRequestEvent, PresignRequestEvent, SessionIdentifier,
    SessionInfo, SignRequestEvent,
};
use mpc::Weight;
use rand_core::SeedableRng;
use shared_wasm_class_groups::message_digest::{message_digest, Hash};
use std::collections::HashSet;
use std::sync::Arc;

fn dwallet_dkg_first_public_input(
    protocol_public_parameters: &twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
) -> DwalletMPCResult<<DWalletDKGFirstParty as mpc::Party>::PublicInput> {
    <DWalletDKGFirstParty as DWalletDKGFirstPartyPublicInputGenerator>::generate_public_input(
        protocol_public_parameters.clone(),
    )
}

pub(super) fn dwallet_imported_key_verification_request_event_session_info(
    deserialized_event: DWalletSessionEvent<DWalletImportedKeyVerificationRequestEvent>,
) -> SessionInfo {
    SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::DWalletImportedKeyVerificationRequest(deserialized_event),
    }
}

fn dwallet_dkg_second_public_input(
    deserialized_event: &DWalletDKGSecondRoundRequestEvent,
    protocol_public_parameters: twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
) -> DwalletMPCResult<<DWalletDKGSecondParty as mpc::Party>::PublicInput> {
    <DWalletDKGSecondParty as DWalletDKGSecondPartyPublicInputGenerator>::generate_public_input(
        protocol_public_parameters,
        deserialized_event.first_round_output.clone(),
        deserialized_event
            .centralized_public_key_share_and_proof
            .clone(),
    )
}

pub(crate) fn presign_public_input(
    session_identifier: SessionIdentifier,
    deserialized_event: PresignRequestEvent,
    protocol_public_parameters: twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
) -> DwalletMPCResult<<PresignParty as mpc::Party>::PublicInput> {
    <PresignParty as PresignPartyPublicInputGenerator>::generate_public_input(
        protocol_public_parameters,
        // TODO: IMPORTANT: for global presign for schnorr / eddsa signature where the presign is not per dWallet - change the code to support it.
        // The Presign Party Public Input would not take the `DKGOutput` as input in that case - probably the go-to would be to have it as an Option in the `Protocol` trait.
        deserialized_event.dwallet_public_output.clone().ok_or(
            DwalletMPCError::MPCSessionError {
                session_identifier,
                error: "presign public input cannot be None as we only support ECDSA".to_string(),
            },
        )?,
    )
}

/// Deterministically determine the set of expected decrypters for an optimization of the threshold decryption in the Sign protocol.
/// Pseudo-randomly samples a subset of size `t + 10% * n`, i.e. we add an extra ten-percent of validators,
/// of which at least `t` should be online (send message) during the first round of Sign, i.e. they are expected to decrypt the signature.
///
/// This is a non-stateful way to agree on a subset (that has to be the same for all validators);
/// in the future, we may consider generating this subset in a stateful manner that takes into account the validators' online/offline states, malicious activities etc.
/// This would be better, though harder to implement in practice, and will only be done if we see that the current method is ineffective; however, we expect 10% to cover for these effects successfully.
///
/// Note: this is only an optimization: if we don't have at least `t` online decrypters out of the `expected_decrypters` subset, the Sign protocol still completes successfully, just slower.
fn generate_expected_decrypters(
    epoch_store: Arc<AuthorityPerEpochStore>,
    session_identifier: SessionIdentifier,
) -> DwalletMPCResult<HashSet<PartyID>> {
    let access_structure = epoch_store.get_weighted_threshold_access_structure()?;
    let total_weight = access_structure.total_weight();
    let expected_decrypters_weight =
        access_structure.threshold + (total_weight as f64 * 0.10).floor() as Weight;

    let mut seed_rng = rand_chacha::ChaCha20Rng::from_seed(session_identifier);
    // TODO(Scaly): use direct error here
    let expected_decrypters = access_structure
        .random_subset_with_target_weight(expected_decrypters_weight, &mut seed_rng)
        .map_err(|e| DwalletMPCError::TwoPCMPCError(e.to_string()))?;

    Ok(expected_decrypters)
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

    let expected_decrypters = generate_expected_decrypters(
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

// TODO (#542): move this logic to run before writing the event to the DB, maybe include within the session info
/// Parses an [`Event`] to extract the corresponding [`MPCParty`],
/// public input, private input and session information.
///
/// Returns an error if the event type does not correspond to any known MPC rounds
/// or if deserialization fails.
pub(crate) async fn session_input_from_event(
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
