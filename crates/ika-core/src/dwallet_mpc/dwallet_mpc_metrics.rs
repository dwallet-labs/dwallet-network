use prometheus::{register_int_gauge_with_registry, IntGauge, Registry};
use std::sync::Arc;

pub struct DWalletMPCMetrics {
    // DKG first Round.
    pub(crate) advance_calls_for_dwallet_dkg_first_round: IntGauge,
    pub(crate) advance_completions_for_dwallet_dkg_first_round: IntGauge,
    pub(crate) received_events_start_dwallet_dkg_first_round_count: IntGauge,
    pub(crate) dwallet_dkg_first_round_completions_count: IntGauge,
    pub(crate) dwallet_dkg_first_round_completion_duration: IntGauge,

    // DKG Second Round.
    pub(crate) advance_calls_for_dwallet_dkg_second_round: IntGauge,
    pub(crate) advance_completions_for_dwallet_dkg_second_round: IntGauge,
    pub(crate) received_events_start_dwallet_dkg_second_round_count: IntGauge,
    pub(crate) dwallet_dkg_second_round_completions_count: IntGauge,
    pub(crate) dwallet_dkg_second_round_completion_duration: IntGauge,

    // Presign.
    pub(crate) advance_calls_for_presign: IntGauge,
    pub(crate) advance_completions_for_presign: IntGauge,
    pub(crate) received_events_start_presign_count: IntGauge,
    pub(crate) presign_round_completions_count: IntGauge,
    pub(crate) presign_last_completion_duration: IntGauge,

    // Sign.
    pub(crate) advance_calls_for_sign: IntGauge,
    pub(crate) advance_completions_for_sign: IntGauge,
    pub(crate) received_events_start_sign_count: IntGauge,
    pub(crate) sign_round_completions_count: IntGauge,
    pub(crate) sign_last_completion_duration: IntGauge,

    // Network DKG.
    pub(crate) advance_calls_for_network_dkg: IntGauge,
    pub(crate) advance_completions_for_network_dkg: IntGauge,
    pub(crate) received_events_start_network_dkg_count: IntGauge,
    pub(crate) network_dkg_round_completions_count: IntGauge,
    pub(crate) network_dkg_completion_duration: IntGauge,

    // Encrypted Share Verification.
    pub(crate) advance_calls_for_encrypted_share_verification: IntGauge,
    pub(crate) advance_completions_for_encrypted_share_verification: IntGauge,
    pub(crate) received_events_start_encrypted_share_verification_count: IntGauge,
    pub(crate) encrypted_share_verification_round_completions_count: IntGauge,
    pub(crate) encrypted_share_verification_completion_duration: IntGauge,

    // Partial Signature Verification.
    pub(crate) advance_calls_for_partial_signature_verification: IntGauge,
    pub(crate) advance_completions_for_partial_signature_verification: IntGauge,
    pub(crate) received_events_start_partial_signature_verification_count: IntGauge,
    pub(crate) partial_signature_verification_round_completions_count: IntGauge,
    pub(crate) partial_signature_verification_completion_duration: IntGauge,

    // Decryption Key Reshare.
    pub(crate) advance_calls_for_decryption_key_reshare: IntGauge,
    pub(crate) advance_completions_for_decryption_key_reshare: IntGauge,
    pub(crate) received_events_start_decryption_key_reshare_count: IntGauge,
    pub(crate) decryption_key_reshare_round_completions_count: IntGauge,
    pub(crate) decryption_key_reshare_completion_duration: IntGauge,

    // todo(zeev): fix unused metrics.
    // MakeDWalletUserSecretKeySharesPublic.
    pub(crate) advance_calls_for_make_dwallet_user_secret_key_shares_public: IntGauge,
    pub(crate) advance_completions_for_make_dwallet_user_secret_key_shares_public: IntGauge,
    pub(crate) received_events_start_make_dwallet_user_secret_key_shares_public_count: IntGauge,
    pub(crate) make_dwallet_user_secret_key_shares_public_round_completions_count: IntGauge,
    pub(crate) make_dwallet_user_secret_key_shares_public_completion_duration: IntGauge,

    // ImportDWalletVerification.
    pub(crate) advance_calls_for_import_dwallet_verification: IntGauge,
    pub(crate) advance_completions_for_import_dwallet_verification: IntGauge,
    pub(crate) received_events_start_import_dwallet_verification_count: IntGauge,
    pub(crate) import_dwallet_verification_round_completions_count: IntGauge,
    pub(crate) import_dwallet_verification_completion_duration: IntGauge,
}

impl DWalletMPCMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let this = Self {
            // ImportDWalletVerification.
            advance_calls_for_import_dwallet_verification: register_int_gauge_with_registry!(
                "advance_calls_for_import_dwallet_verification",
                "Number of advance calls made during the import dWallet verification phase",
                registry
            )
                .unwrap(),
            advance_completions_for_import_dwallet_verification: register_int_gauge_with_registry!(
                "advance_completions_for_import_dwallet_verification",
                "Number of advance completions made during the import dWallet verification phase",
                registry
            )
                .unwrap(),
            received_events_start_import_dwallet_verification_count: register_int_gauge_with_registry!(
                "received_events_start_import_dwallet_verification_count",
                "Number of start events received for the import dWallet verification phase",
                registry
            )
                .unwrap(),
            import_dwallet_verification_round_completions_count: register_int_gauge_with_registry!(
                "import_dwallet_verification_round_completions_count",
                "Total number of completions for the import dWallet verification phase",
                registry
            )
                .unwrap(),
            import_dwallet_verification_completion_duration: register_int_gauge_with_registry!(
                "import_dwallet_verification_completion_duration",
                "Duration of the last completion for the import dWallet verification phase",
                registry
            )
                .unwrap(),

            // MakeDWalletUserSecretKeySharesPublic.
            advance_calls_for_make_dwallet_user_secret_key_shares_public: register_int_gauge_with_registry!(
                "advance_calls_for_make_dwallet_user_secret_key_shares_public",
                "Number of advance calls made during the make dWallet user secret key shares public phase",
                registry
            )
                .unwrap(),
            advance_completions_for_make_dwallet_user_secret_key_shares_public: register_int_gauge_with_registry!(
                "advance_completions_for_make_dwallet_user_secret_key_shares_public",
                "Number of advance completions made during the make dWallet user secret key shares public phase",
                registry
            )
                .unwrap(),
            received_events_start_make_dwallet_user_secret_key_shares_public_count: register_int_gauge_with_registry!(
                "received_events_start_make_dwallet_user_secret_key_shares_public_count",
                "Number of start events received for the make dWallet user secret key shares public phase",
                registry
            )
                .unwrap(),
            make_dwallet_user_secret_key_shares_public_round_completions_count: register_int_gauge_with_registry!(
                "make_dwallet_user_secret_key_shares_public_round_completions_count",
                "Total number of completions for the make dWallet user secret key shares public phase",
                registry
            )
                .unwrap(),
            make_dwallet_user_secret_key_shares_public_completion_duration: register_int_gauge_with_registry!(
                "make_dwallet_user_secret_key_shares_public_completion_duration",
                "Duration of the last completion for the make dWallet user secret key shares public phase",
                registry
            )
                .unwrap(),

            // DKG first Round.
            advance_calls_for_dwallet_dkg_first_round: register_int_gauge_with_registry!(
                "advance_calls_for_dwallet_dkg_first_round",
                "Number of advance calls made during the first round of dWallet MPC DKG",
                registry
            )
                .unwrap(),
            advance_completions_for_dwallet_dkg_first_round: register_int_gauge_with_registry!(
                "advance_completions_for_dwallet_dkg_first_round",
                "Number of advance completions made during the first round of dWallet MPC DKG",
                registry
            )
                .unwrap(),
            received_events_start_dwallet_dkg_first_round_count: register_int_gauge_with_registry!(
                "received_events_start_dwallet_dkg_first_round_count",
                "Number of start events received for the first round of dWallet MPC DKG",
                registry
            )
                .unwrap(),
            dwallet_dkg_first_round_completions_count: register_int_gauge_with_registry!(
                "dwallet_dkg_first_round_completions_count",
                "Total number of completions for the first round of dWallet MPC DKG",
                registry
            )
                .unwrap(),
            dwallet_dkg_first_round_completion_duration: register_int_gauge_with_registry!(
                "dwallet_dkg_first_round_completion_duration",
                "Duration of the last completion for the first round of dWallet MPC DKG",
                registry
            )
                .unwrap(),

            // DKG Second Round.
            advance_calls_for_dwallet_dkg_second_round: register_int_gauge_with_registry!(
                "advance_calls_for_dwallet_dkg_second_round",
                "Number of advance calls made during the second round of dWallet MPC DKG",
                registry
            )
                .unwrap(),
            advance_completions_for_dwallet_dkg_second_round: register_int_gauge_with_registry!(
                "advance_completions_for_dwallet_dkg_second_round",
                "Number of advance completions made during the second round of dWallet MPC DKG",
                registry
            )
                .unwrap(),
            received_events_start_dwallet_dkg_second_round_count:
            register_int_gauge_with_registry!(
                    "received_events_start_dwallet_dkg_second_round_count",
                    "Number of start events received for the second round of dWallet MPC DKG",
                    registry
                )
                .unwrap(),
            dwallet_dkg_second_round_completions_count: register_int_gauge_with_registry!(
                "dwallet_dkg_second_round_completions_count",
                "Total number of completions for the second round of dWallet MPC DKG",
                registry
            )
                .unwrap(),
            dwallet_dkg_second_round_completion_duration: register_int_gauge_with_registry!(
                "dwallet_dkg_second_round_completion_duration",
                "Duration of the last completion for the second round of dWallet MPC DKG",
                registry
            )
                .unwrap(),

            // Presign.
            advance_calls_for_presign: register_int_gauge_with_registry!(
                "advance_calls_for_presign",
                "Number of advance calls made during the presign phase",
                registry
            )
                .unwrap(),
            advance_completions_for_presign: register_int_gauge_with_registry!(
                "advance_completions_for_presign",
                "Number of advance completions made during the presign phase",
                registry
            )
                .unwrap(),
            received_events_start_presign_count: register_int_gauge_with_registry!(
                "received_events_start_presign_count",
                "Number of start events received for the presign phase",
                registry
            )
                .unwrap(),
            presign_round_completions_count: register_int_gauge_with_registry!(
                "presign_round_completions_count",
                "Total number of completions for the presign phase",
                registry
            )
                .unwrap(),
            presign_last_completion_duration: register_int_gauge_with_registry!(
                "presign_last_completion_duration",
                "Duration of the last completion for the presign phase",
                registry
            )
                .unwrap(),

            // Sign.
            advance_calls_for_sign: register_int_gauge_with_registry!(
                "advance_calls_for_sign",
                "Number of advance calls made during the sign phase",
                registry
            )
                .unwrap(),
            advance_completions_for_sign: register_int_gauge_with_registry!(
                "advance_completions_for_sign",
                "Number of advance completions made during the sign phase",
                registry
            )
                .unwrap(),
            received_events_start_sign_count: register_int_gauge_with_registry!(
                "received_events_start_sign_count",
                "Number of start events received for the sign phase",
                registry
            )
                .unwrap(),
            sign_round_completions_count: register_int_gauge_with_registry!(
                "sign_round_completions_count",
                "Total number of completions for the sign phase",
                registry
            )
                .unwrap(),
            sign_last_completion_duration: register_int_gauge_with_registry!(
                "sign_last_completion_duration",
                "Duration of the last completion for the sign phase",
                registry
            )
                .unwrap(),

            // Network DKG.
            advance_calls_for_network_dkg: register_int_gauge_with_registry!(
                "advance_calls_for_network_dkg",
                "Number of advance calls made during the network DKG phase",
                registry
            )
                .unwrap(),
            advance_completions_for_network_dkg: register_int_gauge_with_registry!(
                "advance_completions_for_network_dkg",
                "Number of advance completions made during the network DKG phase",
                registry
            )
                .unwrap(),
            received_events_start_network_dkg_count: register_int_gauge_with_registry!(
                "received_events_start_network_dkg_count",
                "Number of start events received for the network DKG phase",
                registry
            )
                .unwrap(),
            network_dkg_round_completions_count: register_int_gauge_with_registry!(
                "network_dkg_round_completions_count",
                "Total number of completions for the network DKG phase",
                registry
            )
                .unwrap(),
            network_dkg_completion_duration: register_int_gauge_with_registry!(
                "network_dkg_completion_duration",
                "Duration of the last completion for the network DKG phase",
                registry
            )
                .unwrap(),

            // Encrypted Share Verification.
            advance_calls_for_encrypted_share_verification: register_int_gauge_with_registry!(
                "advance_calls_for_encrypted_share_verification",
                "Number of advance calls made during the encrypted share verification phase",
                registry
            )
                .unwrap(),
            advance_completions_for_encrypted_share_verification:
            register_int_gauge_with_registry!(
                    "advance_completions_for_encrypted_share_verification",
                    "Number of advance completions made during the encrypted share verification phase",
                    registry
                )
                .unwrap(),
            received_events_start_encrypted_share_verification_count:
            register_int_gauge_with_registry!(
                    "received_events_start_encrypted_share_verification_count",
                    "Number of start events received for the encrypted share verification phase",
                    registry
                )
                .unwrap(),
            encrypted_share_verification_round_completions_count:
            register_int_gauge_with_registry!(
                    "encrypted_share_verification_round_completions_count",
                    "Total number of completions for the encrypted share verification phase",
                    registry
                )
                .unwrap(),
            encrypted_share_verification_completion_duration:
            register_int_gauge_with_registry!(
                    "encrypted_share_verification_completion_duration",
                    "Duration of the last completion for the encrypted share verification phase",
                    registry
                )
                .unwrap(),

            // Partial Signature Verification.
            advance_calls_for_partial_signature_verification: register_int_gauge_with_registry!(
                "advance_calls_for_partial_signature_verification",
                "Number of advance calls made during the partial signature verification phase",
                registry
            )
                .unwrap(),
            advance_completions_for_partial_signature_verification:
            register_int_gauge_with_registry!(
                    "advance_completions_for_partial_signature_verification",
                    "Number of advance completions made during the partial signature verification phase",
                    registry
                )
                .unwrap(),
            received_events_start_partial_signature_verification_count:
            register_int_gauge_with_registry!(
                    "received_events_start_partial_signature_verification_count",
                    "Number of start events received for the partial signature verification phase",
                    registry
                )
                .unwrap(),
            partial_signature_verification_round_completions_count:
            register_int_gauge_with_registry!(
                    "partial_signature_verification_round_completions_count",
                    "Total number of completions for the partial signature verification phase",
                    registry
                )
                .unwrap(),
            partial_signature_verification_completion_duration:
            register_int_gauge_with_registry!(
                    "partial_signature_verification_completion_duration",
                    "Duration of the last completion for the partial signature verification phase",
                    registry
                )
                .unwrap(),

            // Decryption Key Reshare.
            advance_calls_for_decryption_key_reshare: register_int_gauge_with_registry!(
                "advance_calls_for_decryption_key_reshare",
                "Number of advance calls made during the decryption key reshare phase",
                registry
            )
                .unwrap(),
            advance_completions_for_decryption_key_reshare: register_int_gauge_with_registry!(
                "advance_completions_for_decryption_key_reshare",
                "Number of advance completions made during the decryption key reshare phase",
                registry
            )
                .unwrap(),
            received_events_start_decryption_key_reshare_count: register_int_gauge_with_registry!(
                "received_events_start_decryption_key_reshare_count",
                "Number of start events received for the decryption key reshare phase",
                registry
            )
                .unwrap(),
            decryption_key_reshare_round_completions_count: register_int_gauge_with_registry!(
                "decryption_key_reshare_round_completions_count",
                "Total number of completions for the decryption key reshare phase",
                registry
            )
                .unwrap(),
            decryption_key_reshare_completion_duration: register_int_gauge_with_registry!(
                "decryption_key_reshare_completion_duration",
                "Duration of the last completion for the decryption key reshare phase",
                registry
            )
                .unwrap(),
        };
        Arc::new(this)
    }
}
