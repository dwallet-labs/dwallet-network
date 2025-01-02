// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use mysten_common::metrics::{push_metrics, MetricsPushClient};
use mysten_metrics::RegistryService;
use prometheus::{
    register_histogram_vec_with_registry, register_int_counter_vec_with_registry,
    register_int_counter_with_registry, register_int_gauge_vec_with_registry,
    register_int_gauge_with_registry, HistogramVec, IntCounter, IntCounterVec, IntGauge,
    IntGaugeVec, Registry,
};
use std::time::Duration;
use sui_types::crypto::NetworkKeyPair;

#[derive(Clone, Debug)]
pub struct SuiClientMetrics {
    pub sui_rpc_errors: IntCounterVec,

}

impl SuiClientMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let this = Self {
            sui_rpc_errors: register_int_counter_vec_with_registry!(
                "sui_client_sui_rpc_errors",
                "Total number of errors from sui RPC, by RPC method",
                &["method"],
                registry,
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
