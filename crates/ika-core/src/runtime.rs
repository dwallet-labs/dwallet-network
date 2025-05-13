// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use ika_config::NodeConfig;
use tokio::runtime::Runtime;
use tracing::error;

pub struct IkaRuntimes {
    // Order in this struct is the order in which runtimes are stopped
    pub ika_node: Runtime,
    pub metrics: Runtime,
}

impl IkaRuntimes {
    pub fn new(_confg: &NodeConfig) -> Self {
        if let Err(err) = rayon::ThreadPoolBuilder::new()
            .panic_handler(|err| error!("Rayon thread pool task panicked: {:?}", err))
            .stack_size(16 * 1024 * 1024)
            .build_global()
        {
            error!("Failed to create rayon thread pool: {:?}", err);
        }
        let ika_node = tokio::runtime::Builder::new_multi_thread()
            .thread_name("ika-node-runtime")
            .thread_stack_size(16 * 1024 * 1024)
            .enable_all()
            .build()
            .unwrap();
        let metrics = tokio::runtime::Builder::new_multi_thread()
            .thread_name("metrics-runtime")
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap();

        Self { ika_node, metrics }
    }
}
