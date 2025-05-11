// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use prometheus::{
    register_histogram_with_registry, register_int_counter_vec_with_registry,
    register_int_counter_with_registry, register_int_gauge_vec_with_registry,
    register_int_gauge_with_registry, Histogram, IntCounter, IntCounterVec, IntGauge, IntGaugeVec,
    Registry,
};
use std::sync::Arc;

pub struct ParamsMessageMetrics {
    pub last_certified_params_message: IntGauge,
    pub last_constructed_params_message: IntGauge,
    pub params_message_errors: IntCounter,
    pub messages_included_in_params_message: IntCounter,
    pub params_message_roots_count: IntCounter,
    pub params_message_participation: IntCounterVec,
    pub last_received_params_message_signatures: IntGaugeVec,
    pub last_sent_params_message_signature: IntGauge,
    pub last_skipped_params_message_signature_submission: IntGauge,
    pub last_ignored_params_message_signature_received: IntGauge,
    pub highest_accumulated_epoch: IntGauge,
    pub params_message_creation_latency: Histogram,
    pub remote_params_message_forks: IntCounter,
    pub split_brain_params_message_forks: IntCounter,
    pub last_created_params_message_age: Histogram,
    pub last_certified_params_message_age: Histogram,
}

impl ParamsMessageMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let this = Self {
            last_certified_params_message: register_int_gauge_with_registry!(
                "last_certified_params_message",
                "Last certified params_message",
                registry
            )
            .unwrap(),
            last_constructed_params_message: register_int_gauge_with_registry!(
                "last_constructed_params_message",
                "Last constructed params_message",
                registry
            )
            .unwrap(),
            last_created_params_message_age: register_histogram_with_registry!(
                "last_created_params_message_age",
                "Age of the last created params_message",
                mysten_metrics::LATENCY_SEC_BUCKETS.to_vec(),
                registry
            ).unwrap(),
            last_certified_params_message_age: register_histogram_with_registry!(
                "last_certified_params_message_age",
                "Age of the last certified params_message",
                mysten_metrics::LATENCY_SEC_BUCKETS.to_vec(),
                registry
            ).unwrap(),
            params_message_errors: register_int_counter_with_registry!(
                "params_message_errors",
                "ParamsMessages errors count",
                registry
            )
            .unwrap(),
            messages_included_in_params_message: register_int_counter_with_registry!(
                "messages_included_in_params_message",
                "Messages included in a params_message",
                registry
            )
            .unwrap(),
            params_message_roots_count: register_int_counter_with_registry!(
                "params_message_roots_count",
                "Number of params_message roots received from consensus",
                registry
            )
            .unwrap(),
            params_message_participation: register_int_counter_vec_with_registry!(
                "params_message_participation",
                "Participation in params_message certification by validator",
                &["signer"],
                registry
            )
            .unwrap(),
            last_received_params_message_signatures: register_int_gauge_vec_with_registry!(
                "last_received_params_message_signatures",
                "Last received params_message signatures by validator",
                &["signer"],
                registry
            )
            .unwrap(),
            last_sent_params_message_signature: register_int_gauge_with_registry!(
                "last_sent_params_message_signature",
                "Last params_message signature sent by myself",
                registry
            )
            .unwrap(),
            last_skipped_params_message_signature_submission: register_int_gauge_with_registry!(
                "last_skipped_params_message_signature_submission",
                "Last params_message signature that this validator skipped submitting because it was already certfied.",
                registry
            )
            .unwrap(),
            last_ignored_params_message_signature_received: register_int_gauge_with_registry!(
                "last_ignored_params_message_signature_received",
                "Last received params_message signature that this validator ignored because it was already certfied.",
                registry
            )
            .unwrap(),
            highest_accumulated_epoch: register_int_gauge_with_registry!(
                "highest_accumulated_params_message_epoch",
                "Highest accumulated params_message epoch",
                registry
            )
            .unwrap(),
            params_message_creation_latency: register_histogram_with_registry!(
                "params_message_creation_latency",
                "Latency from consensus commit timestamp to local params_message creation in milliseconds",
                mysten_metrics::LATENCY_SEC_BUCKETS.to_vec(),
                registry,
            ).unwrap(),
            remote_params_message_forks: register_int_counter_with_registry!(
                "remote_params_message_forks",
                "Number of remote params_messages that forked from local params_messages",
                registry
            )
            .unwrap(),
            split_brain_params_message_forks: register_int_counter_with_registry!(
                "split_brain_params_message_forks",
                "Number of params_messages that have resulted in a split brain",
                registry
            )
            .unwrap(),
        };
        Arc::new(this)
    }

    pub fn new_for_tests() -> Arc<Self> {
        Self::new(&Registry::new())
    }
}
