// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
use anyhow::Result;
use clap::Parser;
use ika_proxy::config::ProxyConfig;
use ika_proxy::{
    admin::{
        app, create_server_cert_default_allow, create_server_cert_enforce_peer,
        make_reqwest_client, server, Labels,
    },
    config::load,
    histogram_relay, metrics,
};
use ika_sui_client::metrics::SuiClientMetrics;
use ika_sui_client::SuiClient;
use mysten_metrics::RegistryService;
use prometheus::Registry;
use std::env;
use std::sync::Arc;
use sui_tls::TlsAcceptor;
use telemetry_subscribers::TelemetryConfig;
use tracing::info;

// Define the `GIT_REVISION` and `VERSION` consts
bin_version::bin_version!();

/// User agent we use when posting to mimir.
static APP_USER_AGENT: &str = const_str::concat!(
env!("CARGO_PKG_NAME"),
"/",
env!("CARGO_PKG_VERSION"),
"/",
VERSION
);

#[derive(Parser, Debug)]
#[clap(rename_all = "kebab-case")]
#[clap(name = env!("CARGO_BIN_NAME"))]
#[clap(version = VERSION)]
struct Args {
    #[clap(
        long,
        short,
        default_value = "./sui-proxy.yaml",
        help = "Specify the config file path to use"
    )]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let (_guard, _handle) = TelemetryConfig::new().init();

    let args = Args::parse();

    let config: ProxyConfig = load(args.config)?;

    info!(
        "listen on {:?} send to {:?}",
        config.listen_address, config.remote_write.url
    );

    let listener = std::net::TcpListener::bind(config.listen_address)?;

    let registry = Registry::new();
    let registry_service = RegistryService::new(registry);

    let sui_client_metrics = SuiClientMetrics::new(&registry_service.default_registry());
    let sui_client = SuiClient::new(
        &config.dynamic_peers.url,
        sui_client_metrics,
        config.dynamic_peers.ika_package_id,
        config.dynamic_peers.ika_system_package_id,
        config.dynamic_peers.ika_system_object_id,
    )
        .await?;

    let (tls_config, allower) = create_server_cert_enforce_peer(config.dynamic_peers, config.static_peers, sui_client)
        .expect("unable to create tls server config");

    // let (tls_config, allower) =
    //     // we'll only use the dynamic peers in some cases — it makes little sense to run with the static's
    //     // since this first mode allows all.
    //     if config.dynamic_peers.certificate_file.is_none() || config.dynamic_peers.private_key.is_none() {
    //         (
    //             create_server_cert_default_allow(config.dynamic_peers.hostname.unwrap())
    //                 .expect("unable to create self-signed server cert"),
    //             None,
    //         )
    //     } else {
    //         create_server_cert_enforce_peer(config.dynamic_peers, config.static_peers, sui_client)
    //             .expect("unable to create tls server config")
    //     };
    let histogram_listener = std::net::TcpListener::bind(config.histogram_address)?;
    let metrics_listener = std::net::TcpListener::bind(config.metrics_address)?;
    let acceptor = TlsAcceptor::new(tls_config);
    let client = make_reqwest_client(config.remote_write, APP_USER_AGENT);
    let histogram_relay = histogram_relay::start_prometheus_server(histogram_listener);
    let registry_service = metrics::start_prometheus_server(metrics_listener, registry_service);
    let prometheus_registry = registry_service.default_registry();
    prometheus_registry.register(mysten_metrics::uptime_metric(
        "ika-proxy",
        VERSION,
        "unavailable",
    ))?;

    let timeout_secs = match env::var("NODE_CLIENT_TIMEOUT") {
        Ok(val) => val.parse::<u64>().ok(),
        Err(_) => None,
    };

    let app = app(
        Labels {
            network: config.network,
            inventory_hostname: env::var("INVENTORY_HOSTNAME")
                .unwrap_or_else(|_| "unknown".to_string()),
        },
        client,
        histogram_relay,
        allower,
        timeout_secs,
    );

    server(listener, app, Some(acceptor)).await.unwrap();

    Ok(())
}
