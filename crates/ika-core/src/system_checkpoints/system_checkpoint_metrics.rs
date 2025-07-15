// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use prometheus::{
    Histogram, IntCounter, IntCounterVec, IntGauge, IntGaugeVec, Registry,
    register_histogram_with_registry, register_int_counter_vec_with_registry,
    register_int_counter_with_registry, register_int_gauge_vec_with_registry,
    register_int_gauge_with_registry,
};
use std::sync::Arc;

pub struct SystemCheckpointMetrics {
    pub last_certified_system_checkpoint: IntGauge,
    pub last_constructed_system_checkpoint: IntGauge,
    pub system_checkpoint_errors: IntCounter,
    pub messages_included_in_system_checkpoint: IntCounter,
    pub system_checkpoint_roots_count: IntCounter,
    pub system_checkpoint_participation: IntCounterVec,
    pub last_received_system_checkpoint_signatures: IntGaugeVec,
    pub last_sent_system_checkpoint_signature: IntGauge,
    pub last_skipped_system_checkpoint_signature_submission: IntGauge,
    pub last_ignored_system_checkpoint_signature_received: IntGauge,
    pub highest_accumulated_epoch: IntGauge,
    pub system_checkpoint_creation_latency: Histogram,
    pub remote_system_checkpoint_forks: IntCounter,
    pub split_brain_system_checkpoint_forks: IntCounter,
    pub last_created_system_checkpoint_age: Histogram,
    pub last_certified_system_checkpoint_age: Histogram,
}

impl SystemCheckpointMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let this = Self {
            last_certified_system_checkpoint: register_int_gauge_with_registry!(
                "last_certified_system_checkpoint",
                "Last certified system_checkpoint",
                registry
            )
            .unwrap(),
            last_constructed_system_checkpoint: register_int_gauge_with_registry!(
                "last_constructed_system_checkpoint",
                "Last constructed system_checkpoint",
                registry
            )
            .unwrap(),
            last_created_system_checkpoint_age: register_histogram_with_registry!(
                "last_created_system_checkpoint_age",
                "Age of the last created system_checkpoint",
                mysten_metrics::LATENCY_SEC_BUCKETS.to_vec(),
                registry
            ).unwrap(),
            last_certified_system_checkpoint_age: register_histogram_with_registry!(
                "last_certified_system_checkpoint_age",
                "Age of the last certified system_checkpoint",
                mysten_metrics::LATENCY_SEC_BUCKETS.to_vec(),
                registry
            ).unwrap(),
            system_checkpoint_errors: register_int_counter_with_registry!(
                "system_checkpoint_errors",
                "SystemCheckpoints errors count",
                registry
            )
            .unwrap(),
            messages_included_in_system_checkpoint: register_int_counter_with_registry!(
                "messages_included_in_system_checkpoint",
                "Messages included in a system_checkpoint",
                registry
            )
            .unwrap(),
            system_checkpoint_roots_count: register_int_counter_with_registry!(
                "system_checkpoint_roots_count",
                "Number of system_checkpoint roots received from consensus",
                registry
            )
            .unwrap(),
            system_checkpoint_participation: register_int_counter_vec_with_registry!(
                "system_checkpoint_participation",
                "Participation in system_checkpoint certification by validator",
                &["signer"],
                registry
            )
            .unwrap(),
            last_received_system_checkpoint_signatures: register_int_gauge_vec_with_registry!(
                "last_received_system_checkpoint_signatures",
                "Last received system_checkpoint signatures by validator",
                &["signer"],
                registry
            )
            .unwrap(),
            last_sent_system_checkpoint_signature: register_int_gauge_with_registry!(
                "last_sent_system_checkpoint_signature",
                "Last system_checkpoint signature sent by myself",
                registry
            )
            .unwrap(),
            last_skipped_system_checkpoint_signature_submission: register_int_gauge_with_registry!(
                "last_skipped_system_checkpoint_signature_submission",
                "Last system_checkpoint signature that this validator skipped submitting because it was already certfied.",
                registry
            )
            .unwrap(),
            last_ignored_system_checkpoint_signature_received: register_int_gauge_with_registry!(
                "last_ignored_system_checkpoint_signature_received",
                "Last received system_checkpoint signature that this validator ignored because it was already certfied.",
                registry
            )
            .unwrap(),
            highest_accumulated_epoch: register_int_gauge_with_registry!(
                "highest_accumulated_system_checkpoint_epoch",
                "Highest accumulated system_checkpoint epoch",
                registry
            )
            .unwrap(),
            system_checkpoint_creation_latency: register_histogram_with_registry!(
                "system_checkpoint_creation_latency",
                "Latency from consensus commit timestamp to local system_checkpoint creation in milliseconds",
                mysten_metrics::LATENCY_SEC_BUCKETS.to_vec(),
                registry,
            ).unwrap(),
            remote_system_checkpoint_forks: register_int_counter_with_registry!(
                "remote_system_checkpoint_forks",
                "Number of remote system_checkpoints that forked from local system_checkpoints",
                registry
            )
            .unwrap(),
            split_brain_system_checkpoint_forks: register_int_counter_with_registry!(
                "split_brain_system_checkpoint_forks",
                "Number of system_checkpoints that have resulted in a split brain",
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
