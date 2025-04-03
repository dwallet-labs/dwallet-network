// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use ika_config::NodeConfig;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::VALIDATOR_TOKIO_ALLOCATED_THREADS;
use tokio::runtime::Runtime;

pub struct IkaRuntimes {
    // Order in this struct is the order in which runtimes are stopped
    pub ika_node: Runtime,
    pub metrics: Runtime,
}

pub(crate) const IKA_NODE_TOKIO_ALLOCATED_THREADS: usize = 1;
pub(crate) const METRICS_TOKIO_ALLOCATED_THREADS: usize = 1;

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

pub fn get_rayon_thread_pool_size() -> DwalletMPCResult<usize> {
    let available_cores_for_computations: usize = std::thread::available_parallelism()
        .map_err(|e| DwalletMPCError::FailedToGetAvailableParallelism(e.to_string()))?
        .into();
    if !(available_cores_for_computations > 0) {
        return Err(DwalletMPCError::InsufficientCPUCores);
    }
    Ok(available_cores_for_computations
        - IKA_NODE_TOKIO_ALLOCATED_THREADS
        - METRICS_TOKIO_ALLOCATED_THREADS
        - 1)
}
