// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use pera_sdk::PeraClientBuilder;

// This example shows the few basic ways to connect to a Pera network.
// There are several in-built methods for connecting to the
// Pera devnet, tesnet, and localnet (running locally),
// as well as a custom way for connecting to custom URLs.
// The example prints out the API versions of the different networks,
// and finally, it prints the list of available RPC methods
// and the list of subscriptions.
// Note that running this code will fail if there is no Pera network
// running locally on the default address: 127.0.0.1:9000

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let pera = PeraClientBuilder::default()
        .build("http://127.0.0.1:9000") // local network address
        .await?;
    println!("Pera local network version: {}", pera.api_version());

    // local Pera network, like the above one but using the dedicated function
    let pera_local = PeraClientBuilder::default().build_localnet().await?;
    println!("Pera local network version: {}", pera_local.api_version());

    // Pera devnet -- https://fullnode.devnet.pera.io:443
    let pera_devnet = PeraClientBuilder::default().build_devnet().await?;
    println!("Pera devnet version: {}", pera_devnet.api_version());

    // Pera testnet -- https://fullnode.testnet.pera.io:443
    let pera_testnet = PeraClientBuilder::default().build_testnet().await?;
    println!("Pera testnet version: {}", pera_testnet.api_version());

    println!("{:?}", pera_local.available_rpc_methods());
    println!("{:?}", pera_local.available_subscriptions());

    Ok(())
}
