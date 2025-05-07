// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use ika_config::NodeConfig;
use tokio::runtime::Runtime;

pub struct IkaRuntimes {
    // Order in this struct is the order in which runtimes are stopped
    pub ika_node: Runtime,
    pub metrics: Runtime,
}

impl IkaRuntimes {
    pub fn new(_confg: &NodeConfig) -> Self {
        let ika_node = tokio::runtime::Builder::new_current_thread()
            .thread_name("ika-node-runtime")
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
