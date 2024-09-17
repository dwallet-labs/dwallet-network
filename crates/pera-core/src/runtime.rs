// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::env;
use std::str::FromStr;
use pera_config::NodeConfig;
use tap::TapFallible;
use tokio::runtime::Runtime;
use tracing::warn;

pub struct PeraRuntimes {
    // Order in this struct is the order in which runtimes are stopped
    pub json_rpc: Runtime,
    pub pera_node: Runtime,
    pub metrics: Runtime,
}

impl PeraRuntimes {
    pub fn new(_confg: &NodeConfig) -> Self {
        let pera_node = tokio::runtime::Builder::new_multi_thread()
            .thread_name("pera-node-runtime")
            .enable_all()
            .build()
            .unwrap();
        let metrics = tokio::runtime::Builder::new_multi_thread()
            .thread_name("metrics-runtime")
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap();

        let worker_thread = env::var("RPC_WORKER_THREAD")
            .ok()
            .and_then(|o| {
                usize::from_str(&o)
                    .tap_err(|e| warn!("Cannot parse RPC_WORKER_THREAD to usize: {e}"))
                    .ok()
            })
            .unwrap_or(num_cpus::get() / 2);

        let json_rpc = tokio::runtime::Builder::new_multi_thread()
            .thread_name("jsonrpc-runtime")
            .worker_threads(worker_thread)
            .enable_all()
            .build()
            .unwrap();
        Self {
            pera_node,
            metrics,
            json_rpc,
        }
    }
}
