// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use clap::Parser;
use std::{path::PathBuf, time::Duration};
use tracing::info;

use sui_surfer::default_surf_strategy::DefaultSurfStrategy;

#[derive(Parser)]
#[command(rename_all = "kebab-case")]
struct Args {
    #[arg(long, help = "Number of seconds to surf, default to 30")]
    pub run_duration: Option<u64>,

    #[arg(long, help = "List of package paths to surf")]
    packages: Vec<PathBuf>,
}

const DEFAULT_RUN_DURATION: u64 = 30;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.packages.is_empty() {
        eprintln!("At least one package is required");
        return;
    }

    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_log_level("off,sui_surfer=info")
        .with_env()
        .init();

    let results = sui_surfer::run::<DefaultSurfStrategy>(
        Duration::from_secs(args.run_duration.unwrap_or(DEFAULT_RUN_DURATION)),
        args.packages,
    )
    .await;
    results.print_stats();
    info!("Finished surfing");
}
