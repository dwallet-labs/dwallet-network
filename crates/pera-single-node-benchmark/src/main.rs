// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use clap::Parser;
use pera_single_node_benchmark::command::Command;
use pera_single_node_benchmark::run_benchmark;
use pera_single_node_benchmark::workload::Workload;

#[tokio::main]
async fn main() {
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_log_level("off,pera_single_node_benchmark=info")
        .with_env()
        .init();

    let args = Command::parse();
    run_benchmark(
        Workload::new(args.tx_count, args.workload),
        args.component,
        args.checkpoint_size,
        args.print_sample_tx,
        args.skip_signing,
    )
    .await;

    if std::env::var("TRACE_FILTER").is_ok() {
        println!("Sleeping for 60 seconds to allow tracing to flush.");
        println!("You can ctrl-c to exit once you see trace data appearing in grafana");
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
