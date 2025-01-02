// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::*;
use colored::Colorize;
use ika::ika_commands::IkaCommand;
use ika_types::exit_main;
use tracing::debug;

// Define the `GIT_REVISION` and `VERSION` consts
bin_version::bin_version!();

#[derive(Parser)]
#[clap(
    name = env!("CARGO_BIN_NAME"),
    about = "Ika decentralized MPC network",
    rename_all = "kebab-case",
    author,
    version = VERSION,
    propagate_version = true,
)]
struct Args {
    #[clap(subcommand)]
    command: IkaCommand,
}

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    let args = Args::parse();
    let _guard = match args.command {
        // IkaCommand::Console { .. } | IkaCommand::KeyTool { .. } | IkaCommand::Move { .. } => {
        //     telemetry_subscribers::TelemetryConfig::new()
        //         .with_log_level("error")
        //         .with_env()
        //         .init()
        // }
        // 
        // IkaCommand::Client {
        //     cmd: Some(ReplayBatch { .. }),
        //     ..
        // } => telemetry_subscribers::TelemetryConfig::new()
        //     .with_log_level("info")
        //     .with_env()
        //     .init(),
        // 
        // IkaCommand::Client {
        //     cmd: Some(ReplayTransaction {
        //         gas_info, ptb_info, ..
        //     }),
        //     ..
        // } => {
        //     let mut config = telemetry_subscribers::TelemetryConfig::new()
        //         .with_log_level("info")
        //         .with_env();
        //     if gas_info {
        //         config = config.with_trace_target("replay_gas_info");
        //     }
        //     if ptb_info {
        //         config = config.with_trace_target("replay_ptb_info");
        //     }
        //     config.init()
        // }
        // 
        // IkaCommand::Client {
        //     cmd: Some(ProfileTransaction { .. }),
        //     ..
        // } => {
        //     // enable full logging for ProfileTransaction and ReplayTransaction
        //     telemetry_subscribers::TelemetryConfig::new()
        //         .with_env()
        //         .init()
        // }

        _ => telemetry_subscribers::TelemetryConfig::new()
            .with_log_level("error")
            .with_env()
            .init(),
    };
    debug!("Ika CLI version: {VERSION}");
    exit_main!(args.command.execute().await);
}
