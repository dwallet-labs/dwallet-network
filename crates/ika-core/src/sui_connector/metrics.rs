// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use mysten_common::metrics::{push_metrics, MetricsPushClient};
use mysten_metrics::RegistryService;
use prometheus::{
    register_histogram_vec_with_registry, register_int_counter_vec_with_registry,
    register_int_counter_with_registry, register_int_gauge_vec_with_registry,
    register_int_gauge_with_registry, HistogramVec, IntCounter, IntCounterVec, IntGauge,
    IntGaugeVec, Registry,
};
use std::sync::Arc;
use std::time::Duration;
use sui_types::crypto::NetworkKeyPair;

const FINE_GRAINED_LATENCY_SEC_BUCKETS: &[f64] = &[
    0.001, 0.005, 0.01, 0.05, 0.1, 0.15, 0.2, 0.25, 0.3, 0.35, 0.4, 0.45, 0.5, 0.6, 0.7, 0.8, 0.9,
    1.0, 1.2, 1.4, 1.6, 1.8, 2.0, 2.5, 3.0, 3.5, 4.0, 5.0, 6.0, 6.5, 7.0, 7.5, 8.0, 8.5, 9.0, 9.5,
    10., 15., 20., 25., 30., 35., 40., 45., 50., 60., 70., 80., 90., 100., 120., 140., 160., 180.,
    200., 250., 300., 350., 400.,
];

#[derive(Clone, Debug)]
pub struct SuiConnectorMetrics {
    pub last_synced_sui_checkpoints: IntGaugeVec,

    pub gas_coin_balance: IntGauge,
    pub(crate) last_checkpoint_to_write_to_sui: IntGauge,
    pub(crate) last_checkpoint_written_to_sui: IntGauge,
    pub(crate) new_checkpoint_to_write_to_sui_count: IntGauge,
    pub(crate) successful_checkpoints_written_to_sui_count: IntGauge,
    pub(crate) failed_checkpoints_written_to_sui_count: IntGauge,
}

impl SuiConnectorMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let this = Self {
            last_synced_sui_checkpoints: register_int_gauge_vec_with_registry!(
                "sui_connector_last_synced_sui_checkpoints",
                "The latest sui checkpoints synced for each module",
                &["module_name"],
                registry,
            )
            .unwrap(),
            gas_coin_balance: register_int_gauge_with_registry!(
                "sui_connector_gas_coin_balance",
                "Current balance of gas coin, in mist",
                registry,
            )
            .unwrap(),
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

    pub fn new_for_testing() -> Arc<Self> {
        let registry = Registry::new();
        Self::new(&registry)
    }
}
