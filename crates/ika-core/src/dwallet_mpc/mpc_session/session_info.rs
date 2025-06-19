use crate::dwallet_mpc::mpc_manager::DWalletMPCManager;
use crate::dwallet_mpc::mpc_session::PublicInput;
use crate::dwallet_mpc::{deserialize_event_or_dynamic_field, network_dkg, reconfiguration};
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, MPCPrivateInput};
use ika_types::dwallet_mpc_error::DwalletMPCResult;
use ika_types::messages_dwallet_mpc::{
    DBSuiEvent, DWalletDKGFirstRoundRequestEvent, DWalletDKGSecondRoundRequestEvent,
    DWalletEncryptionKeyReconfigurationRequestEvent, DWalletImportedKeyVerificationRequestEvent,
    DWalletNetworkDKGEncryptionKeyRequestEvent, DWalletSessionEvent, DWalletSessionEventTrait,
    EncryptedShareVerificationRequestEvent, FutureSignRequestEvent, IkaPackagesConfig,
    MPCProtocolInitData, MakeDWalletUserSecretKeySharesPublicRequestEvent, PresignRequestEvent,
    SessionInfo, SignRequestEvent,
};

/// Parses the session info from the event that was emitted in Sui from the Move code, and returns it.
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
            Ok(Some(dwallet_dkg_first_party_session_info(
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
            Ok(Some(dwallet_dkg_second_party_session_info(
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
            Ok(Some(network_dkg_session_info(
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
                network_decryption_key_reconfiguration_session_info_from_event(deserialized_event),
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

fn network_decryption_key_reconfiguration_session_info_from_event(
    deserialized_event: DWalletSessionEvent<DWalletEncryptionKeyReconfigurationRequestEvent>,
) -> SessionInfo {
    SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::NetworkEncryptionKeyReconfiguration(deserialized_event),
    }
}

fn network_dkg_session_info(
    deserialized_event: DWalletSessionEvent<DWalletNetworkDKGEncryptionKeyRequestEvent>,
    key_scheme: DWalletMPCNetworkKeyScheme,
) -> DwalletMPCResult<SessionInfo> {
    match key_scheme {
        DWalletMPCNetworkKeyScheme::Secp256k1 => {
            Ok(network_dkg_secp256k1_session_info(deserialized_event))
        }
        DWalletMPCNetworkKeyScheme::Ristretto => {
            Ok(network_dkg_ristretto_session_info(deserialized_event))
        }
    }
}

fn network_dkg_secp256k1_session_info(
    deserialized_event: DWalletSessionEvent<DWalletNetworkDKGEncryptionKeyRequestEvent>,
) -> SessionInfo {
    SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::NetworkEncryptionKeyDkg(
            DWalletMPCNetworkKeyScheme::Secp256k1,
            deserialized_event,
        ),
    }
}

fn network_dkg_ristretto_session_info(
    deserialized_event: DWalletSessionEvent<DWalletNetworkDKGEncryptionKeyRequestEvent>,
) -> SessionInfo {
    SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::NetworkEncryptionKeyDkg(
            DWalletMPCNetworkKeyScheme::Ristretto,
            deserialized_event,
        ),
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

fn dwallet_dkg_first_party_session_info(
    deserialized_event: DWalletSessionEvent<DWalletDKGFirstRoundRequestEvent>,
) -> anyhow::Result<SessionInfo> {
    Ok(SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::DKGFirst(deserialized_event),
    })
}

fn dwallet_dkg_second_party_session_info(
    deserialized_event: DWalletSessionEvent<DWalletDKGSecondRoundRequestEvent>,
) -> SessionInfo {
    SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_identifier: deserialized_event.session_identifier_digest(),
        mpc_round: MPCProtocolInitData::DKGSecond(deserialized_event.clone()),

        epoch: deserialized_event.epoch,
    }
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
