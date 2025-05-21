// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use prometheus::{
    register_int_counter_vec_with_registry, IntCounterVec, Registry,
};
use std::sync::Arc;

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
