// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use clap::*;
use colored::Colorize;
use pera_tool::commands::ToolCommand;
use pera_types::exit_main;

// Define the `GIT_REVISION` and `VERSION` consts
bin_version::bin_version!();

#[derive(Parser)]
#[command(
    name = "pera-tool",
    about = "Debugging utilities for pera",
    rename_all = "kebab-case",
    author,
    version = VERSION,
)]
struct App {
    #[command(subcommand)]
    command: ToolCommand,
}

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    let app = App::parse();
    let (_guards, handle) = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    exit_main!(app.command.execute(handle).await);
}
