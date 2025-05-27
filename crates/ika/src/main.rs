// Copyright (c) dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

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
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_log_level("error")
        .with_env()
        .init();
    debug!("Ika CLI version: {VERSION}");
    exit_main!(args.command.execute().await);
}
