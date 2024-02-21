// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use clap::*;
use sui_cluster_test::{config::ClusterTestOpt, ClusterTest};

#[tokio::main]
async fn main() {
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let options = ClusterTestOpt::parse();

    ClusterTest::run(options).await;
}
