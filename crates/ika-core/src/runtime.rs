// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use ika_config::NodeConfig;
use tokio::runtime::Runtime;
use tracing::error;

pub struct IkaRuntimes {
    // Order in this struct is the order in which runtimes are stopped.
    pub ika_node: Runtime,
    pub metrics: Runtime,
}

const TWENTY_MEGA_BYTES: usize = 20 * 1024 * 1024;

impl IkaRuntimes {
    pub fn new(_config: &NodeConfig) -> Self {
        if let Err(err) = rayon::ThreadPoolBuilder::new()
            .panic_handler(|err| error!("Rayon thread pool task panicked: {:?}", err))
            .stack_size(TWENTY_MEGA_BYTES)
            .build_global()
        {
            error!(?err, "failed to create rayon thread pool");
            panic!("Failed to create rayon thread pool");
        }
        let ika_node = tokio::runtime::Builder::new_multi_thread()
            .thread_name("ika-node-runtime")
            .worker_threads(5)
            .thread_stack_size(TWENTY_MEGA_BYTES)
            .enable_all()
            .build()
            .expect("Failed to create ika-node runtime");
        let metrics = tokio::runtime::Builder::new_multi_thread()
            .thread_name("metrics-runtime")
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("Failed to create metrics runtime");

        Self { ika_node, metrics }
    }
}
