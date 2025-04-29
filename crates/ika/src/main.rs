// Copyright (c) dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use clap::*;
use colored::Colorize;
use ika::ika_commands::IkaCommand;
use ika_types::exit_main;
use tracing::debug;
use ika::validator_commands::read_or_generate_seed_and_class_groups_key;

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
    //current dir
    let dir = std::env::current_dir().unwrap();
    read_or_generate_seed_and_class_groups_key(
        dir.join("class-groups.key"),
        dir.join("class-groups.seed"),
    );
}
