use prometheus::{register_int_gauge_with_registry, IntGauge, Registry};
use std::sync::Arc;

pub struct DWalletMPCMetrics {
    // DKG first Round.
    pub(crate) advance_calls_for_dwallet_dkg_first_round: IntGauge,
    pub(crate) advance_completions_for_dwallet_dkg_first_round: IntGauge,
    pub(crate) received_events_start_dwallet_dkg_first_round_count: IntGauge,
    pub(crate) dwallet_dkg_first_round_completions_count: IntGauge,
    pub(crate) dwallet_dkg_last_first_round_completion_duration: IntGauge,

    // DKG Second Round.
    pub(crate) advance_calls_for_dwallet_dkg_second_round: IntGauge,
    pub(crate) advance_completions_for_dwallet_dkg_second_round: IntGauge,
    pub(crate) received_events_start_dwallet_dkg_second_round_count: IntGauge,
    pub(crate) dwallet_dkg_second_round_completions_count: IntGauge,
    pub(crate) dwallet_dkg_last_second_round_completion_duration: IntGauge,

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
    pub(crate) network_dkg_last_completion_duration: IntGauge,

    // Encrypted Share Verification.
    pub(crate) advance_calls_for_encrypted_share_verification: IntGauge,
    pub(crate) advance_completions_for_encrypted_share_verification: IntGauge,
    pub(crate) received_events_start_encrypted_share_verification_count: IntGauge,
    pub(crate) encrypted_share_verification_round_completions_count: IntGauge,
    pub(crate) encrypted_share_verification_last_completion_duration: IntGauge,

    // Partial Signature Verification.
    pub(crate) advance_calls_for_partial_signature_verification: IntGauge,
    pub(crate) advance_completions_for_partial_signature_verification: IntGauge,
    pub(crate) received_events_start_partial_signature_verification_count: IntGauge,
    pub(crate) partial_signature_verification_round_completions_count: IntGauge,
    pub(crate) partial_signature_verification_last_completion_duration: IntGauge,

    // Decryption Key Reshare.
    pub(crate) advance_calls_for_decryption_key_reshare: IntGauge,
    pub(crate) advance_completions_for_decryption_key_reshare: IntGauge,
    pub(crate) received_events_start_decryption_key_reshare_count: IntGauge,
    pub(crate) decryption_key_reshare_round_completions_count: IntGauge,
    pub(crate) decryption_key_reshare_last_completion_duration: IntGauge,
}

impl DWalletMPCMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let this = Self {
            // DKG first Round.
            advance_completions_for_dwallet_dkg_first_round: register_int_gauge_with_registry!(
                "advance_completions_for_dwallet_dkg_first_round",
                "Number of advance completions for dWallet MPC DKG first round",
                registry
            )
            .unwrap(),
            advance_calls_for_dwallet_dkg_first_round: register_int_gauge_with_registry!(
                "advance_calls_for_dwallet_dkg_first_round",
                "Number of advance calls for dWallet MPC DKG first round",
                registry
            )
            .unwrap(),
            received_events_start_dwallet_dkg_first_round_count: register_int_gauge_with_registry!(
                "received_start_dwallet_dkg_events_count",
                "Number of start events received for dWallet MPC DKG first round",
                registry
            )
            .unwrap(),
            dwallet_dkg_first_round_completions_count: register_int_gauge_with_registry!(
                "dwallet_dkg_first_round_completions_count",
                "Number of completions for dWallet MPC DKG first round",
                registry
            )
            .unwrap(),
            dwallet_dkg_last_first_round_completion_duration: register_int_gauge_with_registry!(
                "dwallet_dkg_last_first_round_completion_duration",
                "Duration of last completion for dWallet MPC DKG first round",
                registry
            )
            .unwrap(),

            // DKG Second Round.
            advance_calls_for_dwallet_dkg_second_round: register_int_gauge_with_registry!(
                "advance_calls_for_dwallet_dkg_second_round",
                "Number of advance calls for dWallet MPC DKG second round",
                registry
            )
            .unwrap(),
            advance_completions_for_dwallet_dkg_second_round: register_int_gauge_with_registry!(
                "advance_completions_for_dwallet_dkg_second_round",
                "Number of advance completions for dWallet MPC DKG second round",
                registry
            )
            .unwrap(),
            received_events_start_dwallet_dkg_second_round_count:
                register_int_gauge_with_registry!(
                    "received_start_dwallet_dkg_second_round_events_count",
                    "Number of start events received for dWallet MPC DKG second round",
                    registry
                )
                .unwrap(),
            dwallet_dkg_second_round_completions_count: register_int_gauge_with_registry!(
                "dwallet_dkg_second_round_completions_count",
                "Number of completions for dWallet MPC DKG second round",
                registry
            )
            .unwrap(),
            dwallet_dkg_last_second_round_completion_duration: register_int_gauge_with_registry!(
                "dwallet_dkg_last_second_round_completion_duration",
                "Duration of last completion for dWallet MPC DKG second round",
                registry
            )
            .unwrap(),

            // Presign.
            advance_calls_for_presign: register_int_gauge_with_registry!(
                "advance_calls_for_presign",
                "Number of advance calls for presign phase",
                registry
            )
            .unwrap(),
            advance_completions_for_presign: register_int_gauge_with_registry!(
                "advance_completions_for_presign",
                "Number of advance completions for presign phase",
                registry
            )
            .unwrap(),
            received_events_start_presign_count: register_int_gauge_with_registry!(
                "received_start_presign_events_count",
                "Number of start events received for presign phase",
                registry
            )
            .unwrap(),
            presign_round_completions_count: register_int_gauge_with_registry!(
                "presign_round_completions_count",
                "Number of completions for presign phase",
                registry
            )
            .unwrap(),
            presign_last_completion_duration: register_int_gauge_with_registry!(
                "presign_last_completion_duration",
                "Duration of last completion for presign phase",
                registry
            )
            .unwrap(),

            // Sign.
            advance_calls_for_sign: register_int_gauge_with_registry!(
                "advance_calls_for_sign",
                "Number of advance calls for sign phase",
                registry
            )
            .unwrap(),
            advance_completions_for_sign: register_int_gauge_with_registry!(
                "advance_completions_for_sign",
                "Number of advance completions for sign phase",
                registry
            )
            .unwrap(),
            received_events_start_sign_count: register_int_gauge_with_registry!(
                "received_start_sign_events_count",
                "Number of start events received for sign phase",
                registry
            )
            .unwrap(),
            sign_round_completions_count: register_int_gauge_with_registry!(
                "sign_round_completions_count",
                "Number of completions for sign phase",
                registry
            )
            .unwrap(),
            sign_last_completion_duration: register_int_gauge_with_registry!(
                "sign_last_completion_duration",
                "Duration of last completion for sign phase",
                registry
            )
            .unwrap(),

            // Network DKG.
            advance_calls_for_network_dkg: register_int_gauge_with_registry!(
                "advance_calls_for_network_dkg",
                "Number of advance calls for network DKG",
                registry
            )
            .unwrap(),
            advance_completions_for_network_dkg: register_int_gauge_with_registry!(
                "advance_completions_for_network_dkg",
                "Number of advance completions for network DKG",
                registry
            )
            .unwrap(),
            received_events_start_network_dkg_count: register_int_gauge_with_registry!(
                "received_start_network_dkg_events_count",
                "Number of start events received for network DKG",
                registry
            )
            .unwrap(),
            network_dkg_round_completions_count: register_int_gauge_with_registry!(
                "network_dkg_round_completions_count",
                "Number of completions for network DKG",
                registry
            )
            .unwrap(),
            network_dkg_last_completion_duration: register_int_gauge_with_registry!(
                "network_dkg_last_completion_duration",
                "Duration of last completion for network DKG",
                registry
            )
            .unwrap(),

            // Encrypted Share Verification.
            advance_calls_for_encrypted_share_verification: register_int_gauge_with_registry!(
                "advance_calls_for_encrypted_share_verification",
                "Number of advance calls for encrypted share verification",
                registry
            )
            .unwrap(),
            advance_completions_for_encrypted_share_verification:
                register_int_gauge_with_registry!(
                    "advance_completions_for_encrypted_share_verification",
                    "Number of advance completions for encrypted share verification",
                    registry
                )
                .unwrap(),
            received_events_start_encrypted_share_verification_count:
                register_int_gauge_with_registry!(
                    "received_start_encrypted_share_verification_events_count",
                    "Number of start events received for encrypted share verification",
                    registry
                )
                .unwrap(),
            encrypted_share_verification_round_completions_count:
                register_int_gauge_with_registry!(
                    "encrypted_share_verification_round_completions_count",
                    "Number of completions for encrypted share verification",
                    registry
                )
                .unwrap(),
            encrypted_share_verification_last_completion_duration:
                register_int_gauge_with_registry!(
                    "encrypted_share_verification_last_completion_duration",
                    "Duration of last completion for encrypted share verification",
                    registry
                )
                .unwrap(),

            // Partial Signature Verification.
            advance_calls_for_partial_signature_verification: register_int_gauge_with_registry!(
                "advance_calls_for_partial_signature_verification",
                "Number of advance calls for partial signature verification",
                registry
            )
            .unwrap(),
            advance_completions_for_partial_signature_verification:
                register_int_gauge_with_registry!(
                    "advance_completions_for_partial_signature_verification",
                    "Number of advance completions for partial signature verification",
                    registry
                )
                .unwrap(),
            received_events_start_partial_signature_verification_count:
                register_int_gauge_with_registry!(
                    "received_start_partial_signature_verification_events_count",
                    "Number of start events received for partial signature verification",
                    registry
                )
                .unwrap(),
            partial_signature_verification_round_completions_count:
                register_int_gauge_with_registry!(
                    "partial_signature_verification_round_completions_count",
                    "Number of completions for partial signature verification",
                    registry
                )
                .unwrap(),
            partial_signature_verification_last_completion_duration:
                register_int_gauge_with_registry!(
                    "partial_signature_verification_last_completion_duration",
                    "Duration of last completion for partial signature verification",
                    registry
                )
                .unwrap(),

            // Decryption Key Reshare.
            advance_calls_for_decryption_key_reshare: register_int_gauge_with_registry!(
                "advance_calls_for_decryption_key_reshare",
                "Number of advance calls for decryption key reshare",
                registry
            )
            .unwrap(),
            advance_completions_for_decryption_key_reshare: register_int_gauge_with_registry!(
                "advance_completions_for_decryption_key_reshare",
                "Number of advance completions for decryption key reshare",
                registry
            )
            .unwrap(),
            received_events_start_decryption_key_reshare_count: register_int_gauge_with_registry!(
                "received_start_decryption_key_reshare_events_count",
                "Number of start events received for decryption key reshare",
                registry
            )
            .unwrap(),
            decryption_key_reshare_round_completions_count: register_int_gauge_with_registry!(
                "decryption_key_reshare_round_completions_count",
                "Number of completions for decryption key reshare",
                registry
            )
            .unwrap(),
            decryption_key_reshare_last_completion_duration: register_int_gauge_with_registry!(
                "decryption_key_reshare_last_completion_duration",
                "Duration of last completion for decryption key reshare",
                registry
            )
            .unwrap(),
        };
        Arc::new(this)
    }
}
