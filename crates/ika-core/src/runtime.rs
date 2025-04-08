// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use ika_config::NodeConfig;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use tokio::runtime::Runtime;
use tracing::{error, warn};

pub struct IkaRuntimes {
    // Order in this struct is the order in which runtimes are stopped
    pub ika_node: Runtime,
    pub metrics: Runtime,
}

const IKA_NODE_TOKIO_ALLOCATED_THREADS: usize = 1;
const METRICS_TOKIO_ALLOCATED_THREADS: usize = 2;

impl IkaRuntimes {
    pub fn new(_confg: &NodeConfig) -> Self {
        let ika_node = tokio::runtime::Builder::new_multi_thread()
            .thread_name("ika-node-runtime")
            .worker_threads(IKA_NODE_TOKIO_ALLOCATED_THREADS)
            .enable_all()
            .build()
            .unwrap();
        let metrics = tokio::runtime::Builder::new_multi_thread()
            .thread_name("metrics-runtime")
            .worker_threads(METRICS_TOKIO_ALLOCATED_THREADS)
            .enable_all()
            .build()
            .unwrap();

        Self { ika_node, metrics }
    }
}

/// Calculates the appropriate size for the Rayon thread pool based on
/// available system resources.
pub fn get_rayon_thread_pool_size() -> DwalletMPCResult<usize> {
    let available_cores_for_computations: usize = std::thread::available_parallelism()
        .map_err(|e| DwalletMPCError::FailedToGetAvailableParallelism(e.to_string()))?
        .into();
    if available_cores_for_computations == 0 {
        return Err(DwalletMPCError::InsufficientCPUCores);
    }
    let rayon_thread_pool_size = available_cores_for_computations
        - IKA_NODE_TOKIO_ALLOCATED_THREADS
        - METRICS_TOKIO_ALLOCATED_THREADS
        - 1;
    if rayon_thread_pool_size <= 0 {
        warn!(
            ?available_cores_for_computations,
            "there are not enough logical cores for the Rayon thread pool; time slicing with the Tokio thread pool may cause unexpected behaviour"
        );
        return Ok(1);
    }
    Ok(rayon_thread_pool_size)
}
