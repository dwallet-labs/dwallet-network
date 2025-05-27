use prometheus::{
    register_int_gauge_vec_with_registry, register_int_gauge_with_registry, IntGauge, IntGaugeVec,
    Registry,
};
use std::sync::Arc;

pub struct DWalletMPCMetrics {
    received_events_start_count: IntGaugeVec,
    advance_calls: IntGaugeVec,
    advance_completions: IntGaugeVec,
    completions_count: IntGaugeVec,
    last_completion_duration: IntGaugeVec,
}

impl DWalletMPCMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let metric_labels = [
            "protocol_name",
            "curve",
            "mpc_round",
            "hash_scheme",
            "signature_algorithm",
        ];
        Arc::new(Self {
            received_events_start_count: register_int_gauge_vec_with_registry!(
                "dwallet_mpc_received_events_start_count",
                "Number of received events start",
                &metric_labels,
                registry
            )
            .unwrap(),
            advance_calls: register_int_gauge_vec_with_registry!(
                "dwallet_mpc_advance_calls",
                "Number of advance calls",
                &metric_labels,
                registry
            )
            .unwrap(),
            advance_completions: register_int_gauge_vec_with_registry!(
                "dwallet_mpc_advance_completions",
                "Number of advance completions",
                &metric_labels,
                registry
            )
            .unwrap(),
            completions_count: register_int_gauge_vec_with_registry!(
                "dwallet_mpc_completions_count",
                "Number of completions",
                &metric_labels,
                registry
            )
            .unwrap(),
            last_completion_duration: register_int_gauge_vec_with_registry!(
                "dwallet_mpc_last_completion_duration",
                "Duration of the last completion in milliseconds",
                &metric_labels,
                registry
            )
            .unwrap(),
        })
    }
}

impl DWalletMPCMetrics {
    pub fn add_completion(
        &self,
        protocol_name: &str,
        curve: &str,
        mpc_round: &str,
        hash_scheme: &str,
        signature_algorithm: &str,
    ) {
        self.completions_count
            .with_label_values(&[
                protocol_name,
                curve,
                mpc_round,
                hash_scheme,
                signature_algorithm,
            ])
            .inc();
    }
    
    pub fn add_received_event_start(
        &self,
        protocol_name: &str,
        curve: &str,
        mpc_round: &str,
        hash_scheme: &str,
        signature_algorithm: &str,
    ) {
        self.received_events_start_count
            .with_label_values(&[
                protocol_name,
                curve,
                mpc_round,
                hash_scheme,
                signature_algorithm,
            ])
            .inc();
    }
    
    pub fn add_advance_call(
        &self,
        protocol_name: &str,
        curve: &str,
        mpc_round: &str,
        hash_scheme: &str,
        signature_algorithm: &str,
    ) {
        self.advance_calls
            .with_label_values(&[
                protocol_name,
                curve,
                mpc_round,
                hash_scheme,
                signature_algorithm,
            ])
            .inc();
    }
    
    pub fn add_advance_completion(
        &self,
        protocol_name: &str,
        curve: &str,
        mpc_round: &str,
        hash_scheme: &str,
        signature_algorithm: &str,
    ) {
        self.advance_completions
            .with_label_values(&[
                protocol_name,
                curve,
                mpc_round,
                hash_scheme,
                signature_algorithm,
            ])
            .inc();
    }
    
    pub fn set_last_completion_duration(
        &self,
        protocol_name: &str,
        curve: &str,
        mpc_round: &str,
        hash_scheme: &str,
        signature_algorithm: &str,
        duration_ms: i64,
    ) {
        self.last_completion_duration
            .with_label_values(&[
                protocol_name,
                curve,
                mpc_round,
                hash_scheme,
                signature_algorithm,
            ])
            .set(duration_ms);
    }
}
