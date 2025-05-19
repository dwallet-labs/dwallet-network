// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use clap::*;
use std::path::PathBuf;

#[derive(Parser)]
#[arg(
    name = "sui-graphql-rpc",
    about = "Sui GraphQL RPC",
    rename_all = "kebab-case",
    author,
    version
)]
pub enum Command {
    GenerateConfig {
        /// Path to output the YAML config, otherwise stdout.
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    GenerateSchema {
        /// Path to output GraphQL schema to, in SDL format.
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    GenerateExamples {
        /// Path to output examples docs.
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    FromConfig {
        /// Path to TOML file containing configuration for server.
        #[arg(short, long)]
        path: PathBuf,
    },
    StartServer {
        /// The title to display at the top of the page
        #[arg(short, long)]
        ide_title: Option<String>,
        /// DB URL for data fetching
        #[arg(short, long)]
        db_url: Option<String>,
        /// Port to bind the server to
        #[arg(short, long)]
        port: Option<u16>,
        /// Host to bind the server to
        #[arg(long)]
        host: Option<String>,
        /// Port to bind the prom server to
        #[arg(long)]
        prom_port: Option<u16>,
        /// Host to bind the prom server to
        #[arg(long)]
        prom_host: Option<String>,

        /// Path to TOML file containing configuration for service.
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// RPC url to the Node for tx execution
        #[arg(long)]
        node_rpc_url: Option<String>,
    },
}
