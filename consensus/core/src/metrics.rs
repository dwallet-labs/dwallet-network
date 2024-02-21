// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::sync::Arc;

use prometheus::{register_histogram_with_registry, Histogram, Registry};

const LATENCY_SEC_BUCKETS: &[f64] = &[
    0.001, 0.005, 0.01, 0.05, 0.1, 0.15, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.2, 1.4,
    1.6, 1.8, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0, 6.5, 7.0, 7.5, 8.0, 8.5, 9.0, 9.5, 10.,
    12.5, 15., 17.5, 20., 25., 30., 60., 90., 120., 180., 300.,
];

pub(crate) struct Metrics {
    pub node_metrics: NodeMetrics,
}

pub(crate) fn initialise_metrics(registry: Registry) -> Arc<Metrics> {
    let node_metrics = NodeMetrics::new(&registry);

    Arc::new(Metrics { node_metrics })
}

pub(crate) struct NodeMetrics {
    pub uptime: Histogram,
}

impl NodeMetrics {
    pub fn new(registry: &Registry) -> Self {
        Self {
            uptime: register_histogram_with_registry!(
                "uptime",
                "Total node uptime",
                LATENCY_SEC_BUCKETS.to_vec(),
                registry,
            )
            .unwrap(),
        }
    }
}
