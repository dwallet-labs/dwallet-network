//! # DWallet MPC Metrics
//!
//! This module provides Prometheus metrics for monitoring DWallet Multi-Party Computation
//! (MPC) operations.
//! It tracks various aspects of MPC protocol execution, including event processing,
//! round advancement, and completion times.
//!
//! ## Metrics Overview
//!
//! The metrics are organized around MPC protocol sessions and rounds, with labels that provide
//! detailed context about the cryptographic parameters being used:
//!
//! - **protocol_name**: The type of MPC protocol (e.g., "Sign", "Presign", "dWalletDKGFirstRound")
//! - **curve**: The elliptic curve being used (e.g., "Secp256k1")
//! - **hash_scheme**: The hash algorithm for signing operations (e.g., "SHA256", "KECCAK256")
//! - **signature_algorithm**: The signature algorithm (e.g., "ECDSA")
//! - **mpc_round**: The specific round number within a protocol session

use ika_types::messages_dwallet_mpc::MPCRequestInput;
use prometheus::{
    IntGauge, IntGaugeVec, Registry, register_int_gauge_vec_with_registry,
    register_int_gauge_with_registry,
};
use std::sync::Arc;

/// Prometheus metrics for DWallet MPC operations.
///
/// This struct contains all the metrics used to monitor MPC protocol execution,
/// including event processing, round advancement, and timing information.
pub struct DWalletMPCMetrics {
    /// Tracks the number of MPC protocol sessions that have been initiated.
    ///
    /// Labels: protocol_name, curve, hash_scheme, signature_algorithm
    ///
    /// This metric increments when a new MPC event is received and processing begins.
    /// It helps monitor the overall activity level and can be used to detect
    /// when new protocols are being initiated.
    received_events_start_count: IntGaugeVec,

    /// Tracks the number of advance calls made during MPC protocol execution.
    ///
    /// Labels: protocol_name, curve, mpc_round, hash_scheme, signature_algorithm
    ///
    /// This metric increments each time the MPC protocol attempts to advance to
    /// the next step.
    /// It includes the round number to provide granular visibility
    /// into which specific rounds are being processed.
    advance_calls: IntGaugeVec,

    /// Tracks the number of successful advance completions during MPC protocol execution.
    ///
    /// Labels: protocol_name, curve, mpc_round, hash_scheme, signature_algorithm
    ///
    /// This metric increments when an advance call successfully completes.
    /// Comparing this with `advance_calls` can help identify failure rates
    /// and problematic rounds.
    advance_completions: IntGaugeVec,

    /// Tracks the total number of completed MPC protocol sessions.
    ///
    /// Labels: protocol_name, curve, hash_scheme, signature_algorithm
    ///
    /// This metric increments when an entire MPC protocol session completes
    /// successfully.
    /// It provides insight into overall protocol success rates and throughput.
    completions_count: IntGaugeVec,

    /// Records the duration of the most recent completion for each protocol/round combination.
    ///
    /// Labels: protocol_name, curve, mpc_round, hash_scheme, signature_algorithm
    /// Value: Duration in milliseconds.
    ///
    /// This metric stores the execution time of the last completed round,
    /// allowing monitoring of performance trends and identification of
    /// slow-performing protocol rounds.
    last_completion_duration: IntGaugeVec,

    /// The number of sign sessions in which a quorum of the expected decrypters has participated.
    pub number_of_expected_sign_sessions: IntGauge,
    /// The number of sign sessions in which less than a quorum of the expected decrypters has participated.
    pub number_of_unexpected_sign_sessions: IntGauge,
}

impl DWalletMPCMetrics {
    /// Creates a new instance of DWalletMPCMetrics and registers all metrics with the provided registry.
    ///
    /// # Arguments
    /// * `registry` — The Prometheus registry to register metrics with.
    ///
    /// # Returns
    /// An Arc-wrapped instance of DWalletMPCMetrics for shared access across threads.
    pub fn new(registry: &Registry) -> Arc<Self> {
        // Label sets for different metric types
        // Protocol-level metrics use these labels
        let protocol_metric_labels = [
            "protocol_name",
            "curve",
            "hash_scheme",
            "signature_algorithm",
        ];
        // Round-level metrics include the round number
        let round_metric_labels = [
            "protocol_name",
            "curve",
            "mpc_round",
            "hash_scheme",
            "signature_algorithm",
        ];

        Arc::new(Self {
            received_events_start_count: register_int_gauge_vec_with_registry!(
                "dwallet_mpc_received_events_start_count",
                "Number of received start events",
                &protocol_metric_labels,
                registry
            )
            .unwrap(),
            advance_calls: register_int_gauge_vec_with_registry!(
                "dwallet_mpc_advance_calls",
                "Number of advance calls",
                &round_metric_labels,
                registry
            )
            .unwrap(),
            advance_completions: register_int_gauge_vec_with_registry!(
                "dwallet_mpc_advance_completions",
                "Number of advance completions",
                &round_metric_labels,
                registry
            )
            .unwrap(),
            completions_count: register_int_gauge_vec_with_registry!(
                "dwallet_mpc_completions_count",
                "Number of completions",
                &protocol_metric_labels,
                registry
            )
            .unwrap(),
            last_completion_duration: register_int_gauge_vec_with_registry!(
                "dwallet_mpc_last_completion_duration",
                "Duration of the last completion in milliseconds",
                &round_metric_labels,
                registry
            )
            .unwrap(),
            number_of_unexpected_sign_sessions: register_int_gauge_with_registry!(
                "dwallet_mpc_number_of_unexpected_sign_sessions",
                "Number of unexpected sign sessions",
                registry
            )
            .unwrap(),
            number_of_expected_sign_sessions: register_int_gauge_with_registry!(
                "dwallet_mpc_number_of_expected_sign_sessions",
                "Number of expected sign sessions",
                registry
            )
            .unwrap(),
        })
    }
}

impl DWalletMPCMetrics {
    /// Records the completion of an MPC protocol session.
    ///
    /// This increments the `completions_count` metric with labels derived from the
    /// provided MPC event data.
    ///
    /// # Arguments
    /// * `mpc_event_data` - The MPC protocol initialization data containing context.
    pub fn add_completion(&self, mpc_event_data: &MPCRequestInput) {
        self.completions_count
            .with_label_values(&[
                &mpc_event_data.to_string(),
                &mpc_event_data.get_curve(),
                &mpc_event_data.get_hash_scheme(),
                &mpc_event_data.get_signature_algorithm(),
            ])
            .inc();
    }

    /// Records the start of processing for a received MPC event.
    ///
    /// This increments the received_events_start_count metric with labels derived
    /// from the provided MPC event data.
    ///
    /// # Arguments
    /// * `mpc_event_data` - The MPC protocol initialization data containing context.
    pub fn add_received_event_start(&self, mpc_event_data: &MPCRequestInput) {
        self.received_events_start_count
            .with_label_values(&[
                &mpc_event_data.to_string(),
                &mpc_event_data.get_curve(),
                &mpc_event_data.get_hash_scheme(),
                &mpc_event_data.get_signature_algorithm(),
            ])
            .inc();
    }

    /// Records an advance call for a specific MPC round.
    ///
    /// This increments the `advance_calls` metric with labels derived from the
    /// provided MPC event data and round information.
    ///
    /// # Arguments
    /// * `mpc_event_data` - The MPC protocol initialization data containing context
    /// * `mpc_round` — String identifier for the specific MPC round.
    pub fn add_advance_call(&self, request_input: &MPCRequestInput, mpc_round: &str) {
        self.advance_calls
            .with_label_values(&[
                &request_input.to_string(),
                &request_input.get_curve(),
                mpc_round,
                &request_input.get_hash_scheme(),
                &request_input.get_signature_algorithm(),
            ])
            .inc();
    }

    /// Records the successful completion of an advance call for a specific MPC round.
    ///
    /// This increments the `advance_completions` metric with labels derived from the
    /// provided MPC event data and round information.
    ///
    /// # Arguments
    /// * `mpc_event_data` - The MPC protocol initialization data containing context
    /// * `mpc_round` — String identifier for the specific MPC round.
    pub fn add_advance_completion(&self, mpc_event_data: &MPCRequestInput, mpc_round: &str) {
        self.advance_completions
            .with_label_values(&[
                &mpc_event_data.to_string(),
                &mpc_event_data.get_curve(),
                mpc_round,
                &mpc_event_data.get_hash_scheme(),
                &mpc_event_data.get_signature_algorithm(),
            ])
            .inc();
    }

    /// Sets the duration of the last completion for a specific MPC round.
    ///
    /// This updates the last_completion_duration metric with the provided duration
    /// and labels derived from the MPC event data and round information.
    ///
    /// # Arguments
    /// * `mpc_event_data` - The MPC protocol initialization data containing context
    /// * `mpc_round` — String identifier for the specific MPC round
    /// * `duration_ms` — Duration of the completion in milliseconds.
    pub fn set_last_completion_duration(
        &self,
        mpc_event_data: &MPCRequestInput,
        mpc_round: &str,
        duration_ms: i64,
    ) {
        self.last_completion_duration
            .with_label_values(&[
                &mpc_event_data.to_string(),
                &mpc_event_data.get_curve(),
                mpc_round,
                &mpc_event_data.get_hash_scheme(),
                &mpc_event_data.get_signature_algorithm(),
            ])
            .set(duration_ms);
    }
}
