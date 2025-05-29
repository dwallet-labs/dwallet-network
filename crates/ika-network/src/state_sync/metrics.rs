// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use ika_types::messages_dwallet_checkpoint::DWalletCheckpointSequenceNumber;
use ika_types::messages_system_checkpoints::SystemCheckpointSequenceNumber;
use prometheus::{
    register_histogram_with_registry, register_int_gauge_with_registry, Histogram, IntGauge,
    Registry,
};
use std::sync::Arc;
use tap::Pipe;

#[derive(Clone)]
pub(super) struct Metrics(Option<Arc<Inner>>);

impl std::fmt::Debug for Metrics {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Metrics").finish()
    }
}

impl Metrics {
    pub fn enabled(registry: &Registry) -> Self {
        Metrics(Some(Inner::new(registry)))
    }

    pub fn disabled() -> Self {
        Metrics(None)
    }

    pub fn set_highest_known_dwallet_checkpoint(
        &self,
        sequence_number: DWalletCheckpointSequenceNumber,
    ) {
        if let Some(inner) = &self.0 {
            inner
                .highest_known_dwallet_checkpoint
                .set(sequence_number as i64);
        }
    }

    pub fn set_highest_verified_dwallet_checkpoint(
        &self,
        sequence_number: DWalletCheckpointSequenceNumber,
    ) {
        if let Some(inner) = &self.0 {
            inner
                .highest_verified_dwallet_checkpoint
                .set(sequence_number as i64);
        }
    }

    pub fn set_highest_synced_dwallet_checkpoint(
        &self,
        sequence_number: DWalletCheckpointSequenceNumber,
    ) {
        if let Some(inner) = &self.0 {
            inner
                .highest_synced_dwallet_checkpoint
                .set(sequence_number as i64);
        }
    }

    pub fn dwallet_checkpoint_summary_age_metrics(&self) -> Option<&Histogram> {
        if let Some(inner) = &self.0 {
            return Some(&inner.dwallet_checkpoint_summary_age);
        }
        None
    }

    pub fn set_highest_known_system_checkpoint(
        &self,
        sequence_number: DWalletCheckpointSequenceNumber,
    ) {
        if let Some(inner) = &self.0 {
            inner
                .highest_known_system_checkpoint
                .set(sequence_number as i64);
        }
    }

    pub fn set_highest_verified_system_checkpoint(
        &self,
        sequence_number: SystemCheckpointSequenceNumber,
    ) {
        if let Some(inner) = &self.0 {
            inner
                .highest_verified_system_checkpoint
                .set(sequence_number as i64);
        }
    }

    pub fn set_highest_synced_system_checkpoint(
        &self,
        sequence_number: SystemCheckpointSequenceNumber,
    ) {
        if let Some(inner) = &self.0 {
            inner
                .highest_synced_system_checkpoint
                .set(sequence_number as i64);
        }
    }

    pub fn system_checkpoint_summary_age_metrics(&self) -> Option<&Histogram> {
        if let Some(inner) = &self.0 {
            return Some(&inner.system_checkpoint_summary_age);
        }
        None
    }
}

struct Inner {
    highest_known_dwallet_checkpoint: IntGauge,
    highest_verified_dwallet_checkpoint: IntGauge,
    highest_synced_dwallet_checkpoint: IntGauge,
    dwallet_checkpoint_summary_age: Histogram,

    highest_known_system_checkpoint: IntGauge,
    highest_verified_system_checkpoint: IntGauge,
    highest_synced_system_checkpoint: IntGauge,
    system_checkpoint_summary_age: Histogram,
}

impl Inner {
    pub fn new(registry: &Registry) -> Arc<Self> {
        Self {
            highest_known_dwallet_checkpoint: register_int_gauge_with_registry!(
                "highest_known_checkpoint",
                "Highest known checkpoint",
                registry
            )
            .unwrap(),

            highest_verified_dwallet_checkpoint: register_int_gauge_with_registry!(
                "highest_verified_checkpoint",
                "Highest verified checkpoint",
                registry
            )
            .unwrap(),

            highest_synced_dwallet_checkpoint: register_int_gauge_with_registry!(
                "highest_synced_checkpoint",
                "Highest synced checkpoint",
                registry
            )
            .unwrap(),

            dwallet_checkpoint_summary_age: register_histogram_with_registry!(
                "checkpoint_summary_age",
                "Age of checkpoints summaries when they arrive and are verified.",
                mysten_metrics::LATENCY_SEC_BUCKETS.to_vec(),
                registry,
            )
            .unwrap(),
            highest_known_system_checkpoint: register_int_gauge_with_registry!(
                "highest_known_system_checkpoint",
                "Highest known params message",
                registry
            )
            .unwrap(),
            highest_verified_system_checkpoint: register_int_gauge_with_registry!(
                "highest_verified_system_checkpoint",
                "Highest verified params message",
                registry
            )
            .unwrap(),
            highest_synced_system_checkpoint: register_int_gauge_with_registry!(
                "highest_synced_system_checkpoint",
                "Highest synced params message",
                registry
            )
            .unwrap(),
            system_checkpoint_summary_age: register_histogram_with_registry!(
                "system_checkpoint_summary_age",
                "Age of params messages summaries when they arrive and are verified.",
                mysten_metrics::LATENCY_SEC_BUCKETS.to_vec(),
                registry,
            )
            .unwrap(),
        }
        .pipe(Arc::new)
    }
}
