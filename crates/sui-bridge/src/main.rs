// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use clap::Parser;
use mysten_metrics::start_prometheus_server;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};
use sui_bridge::{
    config::BridgeNodeConfig,
    server::{handler::BridgeRequestHandler, run_server},
};
use sui_config::Config;
use tracing::info;

// TODO consolidate this with sui-node/src/main.rs, but where to put it?
const GIT_REVISION: &str = {
    if let Some(revision) = option_env!("GIT_REVISION") {
        revision
    } else {
        let version = git_version::git_version!(
            args = ["--always", "--abbrev=12", "--dirty", "--exclude", "*"],
            fallback = ""
        );

        if version.is_empty() {
            panic!("unable to query git revision");
        }
        version
    }
};
const VERSION: &str = const_str::concat!(env!("CARGO_PKG_VERSION"), "-", GIT_REVISION);

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
#[clap(name = env!("CARGO_BIN_NAME"))]
#[clap(version = VERSION)]
struct Args {
    #[clap(long)]
    pub config_path: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = BridgeNodeConfig::load(&args.config_path).unwrap();

    // Init metrics server
    let metrics_address =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.metrics_port);
    let registry_service = start_prometheus_server(metrics_address);
    let prometheus_registry = registry_service.default_registry();
    mysten_metrics::init_metrics(&prometheus_registry);
    info!("Metrics server started at port {}", config.metrics_port);

    // Init logging
    let (_guard, _filter_handle) = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .with_prom_registry(&prometheus_registry)
        .init();

    let (server_config, _client_config) = config.validate().await?;

    // TODO Start Client

    // Start Server
    let socket_address = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        server_config.server_listen_port,
    );
    run_server(
        &socket_address,
        BridgeRequestHandler::new(
            server_config.key,
            server_config.sui_client,
            server_config.eth_client,
        ),
    )
    .await;
    Ok(())
}
