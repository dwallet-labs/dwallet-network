// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use prometheus::{register_int_gauge_with_registry, IntGauge, Registry};
use std::sync::Arc;

pub struct SuiNotifierMetrics {
    pub(crate) last_checkpoint_to_write_to_sui: IntGauge,
    pub(crate) last_checkpoint_written_to_sui: IntGauge,
    pub(crate) new_checkpoint_to_write_to_sui_count: IntGauge,
    pub(crate) successful_checkpoints_written_to_sui_count: IntGauge,
    pub(crate) failed_checkpoints_written_to_sui_count: IntGauge,
}

impl SuiNotifierMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let this = Self {
            last_checkpoint_to_write_to_sui: register_int_gauge_with_registry!(
                "last_checkpoint_to_write_to_sui",
                "The last checkpoint to write to Sui",
                registry
            )
            .unwrap(),
            last_checkpoint_written_to_sui: register_int_gauge_with_registry!(
                "last_checkpoint_written_to_sui",
                "The last checkpoint written to Sui",
                registry
            )
            .unwrap(),
            new_checkpoint_to_write_to_sui_count: register_int_gauge_with_registry!(
                "new_checkpoint_to_write_to_sui_count",
                "The number of new checkpoints to write to Sui",
                registry
            )
            .unwrap(),
            successful_checkpoints_written_to_sui_count: register_int_gauge_with_registry!(
                "successful_checkpoints_written_to_sui_count",
                "The number of successful checkpoints written to Sui",
                registry
            )
            .unwrap(),
            failed_checkpoints_written_to_sui_count: register_int_gauge_with_registry!(
                "failed_checkpoints_written_to_sui_count",
                "The number of failed checkpoints written to Sui",
                registry
            )
            .unwrap(),
        };
        Arc::new(this)
    }
}
