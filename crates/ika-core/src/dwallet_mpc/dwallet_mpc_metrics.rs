// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use prometheus::{register_int_gauge_with_registry, IntGauge, Registry};
use std::sync::Arc;

pub struct DWalletMPCMetrics {
    // DKG first Round
    advance_calls_for_dwallet_dkg_first_round: IntGauge,
    advance_completions_for_dwallet_dkg_first_round: IntGauge,
    received_start_dwallet_dkg_events_count: IntGauge,
    dwallet_dkg_first_round_completions_count: IntGauge,

    // DKG Second Round
    advance_calls_for_dwallet_dkg_second_round: IntGauge,
    advance_completions_for_dwallet_dkg_second_round: IntGauge,
    received_start_dwallet_dkg_second_round_events_count: IntGauge,
    dwallet_dkg_second_round_completions_count: IntGauge,

    // Presign
    advance_calls_for_presign: IntGauge,
    advance_completions_for_presign: IntGauge,
    received_start_presign_events_count: IntGauge,
    presign_round_completions_count: IntGauge,

    // Sign
    advance_calls_for_sign: IntGauge,
    advance_completions_for_sign: IntGauge,
    received_start_sign_events_count: IntGauge,
    sign_round_completions_count: IntGauge,

    // Network DKG
    advance_calls_for_network_dkg: IntGauge,
    advance_completions_for_network_dkg: IntGauge,
    received_start_network_dkg_events_count: IntGauge,
    network_dkg_round_completions_count: IntGauge,

    // Encrypted Share Verification
    advance_calls_for_encrypted_share_verification: IntGauge,
    advance_completions_for_encrypted_share_verification: IntGauge,
    received_start_encrypted_share_verification_events_count: IntGauge,
    encrypted_share_verification_round_completions_count: IntGauge,

    // Partial Signature Verification
    advance_calls_for_partial_signature_verification: IntGauge,
    advance_completions_for_partial_signature_verification: IntGauge,
    received_start_partial_signature_verification_events_count: IntGauge,
    partial_signature_verification_round_completions_count: IntGauge,

    // Decryption Key Reshare
    advance_calls_for_decryption_key_reshare: IntGauge,
    advance_completions_for_decryption_key_reshare: IntGauge,
    received_start_decryption_key_reshare_events_count: IntGauge,
    decryption_key_reshare_round_completions_count: IntGauge,
}

impl DWalletMPCMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let this = Self {
            advance_completions_for_dwallet_dkg_first_round: register_int_gauge_with_registry!(
                "advance_completions_for_dwallet_dkg_first_round",
                "Advance completions for dwallet dkg first round",
                registry
            )
            .unwrap(),
            advance_calls_for_dwallet_dkg_first_round: register_int_gauge_with_registry!(
                "advance_calls_for_dwallet_dkg_first_round",
                "Advance calls for dwallet dkg first round",
                registry
            )
            .unwrap(),
            received_start_dwallet_dkg_events_count: register_int_gauge_with_registry!(
                "received_start_dwallet_dkg_events_count",
                "Received start dwallet dkg events count",
                registry
            )
            .unwrap(),
            dwallet_dkg_first_round_completions_count: register_int_gauge_with_registry!(
                "dwallet_dkg_first_round_completions_count",
                "DWallet DKG first round completions count",
                registry
            )
            .unwrap(),
            advance_calls_for_dwallet_dkg_second_round: register_int_gauge_with_registry!(
                "advance_calls_for_dwallet_dkg_second_round",
                "Advance calls for dwallet dkg second round",
                registry
            )
            .unwrap(),
            advance_completions_for_dwallet_dkg_second_round: register_int_gauge_with_registry!(
                "advance_completions_for_dwallet_dkg_second_round",
                "Advance completions for dwallet dkg second round",
                registry
            )
            .unwrap(),
            received_start_dwallet_dkg_second_round_events_count:
                register_int_gauge_with_registry!(
                    "received_start_dwallet_dkg_second_round_events_count",
                    "Received start dwallet dkg second round events count",
                    registry
                )
                .unwrap(),
            dwallet_dkg_second_round_completions_count: register_int_gauge_with_registry!(
                "dwallet_dkg_second_round_completions_count",
                "DWallet DKG second round completions count",
                registry
            )
            .unwrap(),
            advance_calls_for_presign: register_int_gauge_with_registry!(
                "advance_calls_for_presign",
                "Advance calls for presign",
                registry
            )
            .unwrap(),
            advance_completions_for_presign: register_int_gauge_with_registry!(
                "advance_completions_for_presign",
                "Advance completions for presign",
                registry
            )
            .unwrap(),
            received_start_presign_events_count: register_int_gauge_with_registry!(
                "received_start_presign_events_count",
                "Received start presign events count",
                registry
            )
            .unwrap(),
            presign_round_completions_count: register_int_gauge_with_registry!(
                "presign_round_completions_count",
                "Presign round completions count",
                registry
            )
            .unwrap(),
            advance_calls_for_sign: register_int_gauge_with_registry!(
                "advance_calls_for_sign",
                "Advance calls for sign",
                registry
            )
            .unwrap(),
            advance_completions_for_sign: register_int_gauge_with_registry!(
                "advance_completions_for_sign",
                "Advance completions for sign",
                registry
            )
            .unwrap(),
            received_start_sign_events_count: register_int_gauge_with_registry!(
                "received_start_sign_events_count",
                "Received start sign events count",
                registry
            )
            .unwrap(),
            sign_round_completions_count: register_int_gauge_with_registry!(
                "sign_round_completions_count",
                "Sign round completions count",
                registry
            )
            .unwrap(),
            advance_calls_for_network_dkg: register_int_gauge_with_registry!(
                "advance_calls_for_network_dkg",
                "Advance calls for network DKG",
                registry
            )
            .unwrap(),
            advance_completions_for_network_dkg: register_int_gauge_with_registry!(
                "advance_completions_for_network_dkg",
                "Advance completions for network DKG",
                registry
            )
            .unwrap(),
            received_start_network_dkg_events_count: register_int_gauge_with_registry!(
                "received_start_network_dkg_events_count",
                "Received start network DKG events count",
                registry
            )
            .unwrap(),
            network_dkg_round_completions_count: register_int_gauge_with_registry!(
                "network_dkg_round_completions_count",
                "Network DKG round completions count",
                registry
            )
            .unwrap(),
            advance_calls_for_encrypted_share_verification: register_int_gauge_with_registry!(
                "advance_calls_for_encrypted_share_verification",
                "Advance calls for encrypted share verification",
                registry
            )
            .unwrap(),
            advance_completions_for_encrypted_share_verification:
                register_int_gauge_with_registry!(
                    "advance_completions_for_encrypted_share_verification",
                    "Advance completions for encrypted share verification",
                    registry
                )
                .unwrap(),
            received_start_encrypted_share_verification_events_count:
                register_int_gauge_with_registry!(
                    "received_start_encrypted_share_verification_events_count",
                    "Received start encrypted share verification events count",
                    registry
                )
                .unwrap(),
            encrypted_share_verification_round_completions_count:
                register_int_gauge_with_registry!(
                    "encrypted_share_verification_round_completions_count",
                    "Encrypted share verification round completions count",
                    registry
                )
                .unwrap(),
            advance_calls_for_partial_signature_verification: register_int_gauge_with_registry!(
                "advance_calls_for_partial_signature_verification",
                "Advance calls for partial signature verification",
                registry
            )
            .unwrap(),
            advance_completions_for_partial_signature_verification:
                register_int_gauge_with_registry!(
                    "advance_completions_for_partial_signature_verification",
                    "Advance completions for partial signature verification",
                    registry
                )
                .unwrap(),
            received_start_partial_signature_verification_events_count:
                register_int_gauge_with_registry!(
                    "received_start_partial_signature_verification_events_count",
                    "Received start partial signature verification events count",
                    registry
                )
                .unwrap(),
            partial_signature_verification_round_completions_count:
                register_int_gauge_with_registry!(
                    "partial_signature_verification_round_completions_count",
                    "Partial signature verification round completions count",
                    registry
                )
                .unwrap(),
            advance_calls_for_decryption_key_reshare: register_int_gauge_with_registry!(
                "advance_calls_for_decryption_key_reshare",
                "Advance calls for decryption key reshare",
                registry
            )
            .unwrap(),
            advance_completions_for_decryption_key_reshare: register_int_gauge_with_registry!(
                "advance_completions_for_decryption_key_reshare",
                "Advance completions for decryption key reshare",
                registry
            )
            .unwrap(),
            received_start_decryption_key_reshare_events_count: register_int_gauge_with_registry!(
                "received_start_decryption_key_reshare_events_count",
                "Received start decryption key reshare events count",
                registry
            )
            .unwrap(),
            decryption_key_reshare_round_completions_count: register_int_gauge_with_registry!(
                "decryption_key_reshare_round_completions_count",
                "Decryption key reshare round completions count",
                registry
            )
            .unwrap(),
        };
        Arc::new(this)
    }
}
