use crate::dwallet_mpc::deserialize_event_or_dynamic_field;
use crate::dwallet_mpc::dwallet_dkg::{
    dwallet_dkg_first_party_session_info, dwallet_dkg_second_party_session_info,
    dwallet_imported_key_verification_request_event_session_info,
};
use crate::dwallet_mpc::encrypt_user_share::start_encrypted_share_verification_session_info;
use crate::dwallet_mpc::make_dwallet_user_secret_key_shares_public::make_dwallet_user_secret_key_shares_public_request_event_session_info;
use crate::dwallet_mpc::network_dkg::network_dkg_session_info;
use crate::dwallet_mpc::presign::presign_party_session_info;
use crate::dwallet_mpc::reconfiguration::network_decryption_key_reconfiguration_session_info_from_event;
use crate::dwallet_mpc::sign::{
    get_verify_partial_signatures_session_info, sign_party_session_info,
};
use dwallet_mpc_types::dwallet_mpc::DWalletMPCNetworkKeyScheme;
use ika_types::messages_dwallet_mpc::{
    DBSuiEvent, DWalletDKGFirstRoundRequestEvent, DWalletDKGSecondRoundRequestEvent,
    DWalletEncryptionKeyReconfigurationRequestEvent, DWalletImportedKeyVerificationRequestEvent,
    DWalletNetworkDKGEncryptionKeyRequestEvent, DWalletSessionEvent, DWalletSessionEventTrait,
    EncryptedShareVerificationRequestEvent, FutureSignRequestEvent, IkaPackagesConfig,
    MakeDWalletUserSecretKeySharesPublicRequestEvent, PresignRequestEvent, SessionInfo,
    SignRequestEvent,
};

// TODO(Scaly): move to event handling
// TODO(Scaly): Rename `mpc_session_request_from_event()`

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
