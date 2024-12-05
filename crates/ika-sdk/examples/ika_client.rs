// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use ika_sdk::IkaClientBuilder;

// This example shows the few basic ways to connect to a Ika network.
// There are several in-built methods for connecting to the
// Ika devnet, tesnet, and localnet (running locally),
// as well as a custom way for connecting to custom URLs.
// The example prints out the API versions of the different networks,
// and finally, it prints the list of available RPC methods
// and the list of subscriptions.
// Note that running this code will fail if there is no Ika network
// running locally on the default address: 127.0.0.1:9000

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let ika = IkaClientBuilder::default()
        .build("http://127.0.0.1:9000") // local network address
        .await?;
    println!("Ika local network version: {}", ika.api_version());

    // local Ika network, like the above one but using the dedicated function
    let ika_local = IkaClientBuilder::default().build_localnet().await?;
    println!("Ika local network version: {}", ika_local.api_version());

    // Ika devnet -- https://fullnode.devnet.ika.io:443
    let ika_devnet = IkaClientBuilder::default().build_devnet().await?;
    println!("Ika devnet version: {}", ika_devnet.api_version());

    // Ika testnet -- https://fullnode.testnet.ika.io:443
    let ika_testnet = IkaClientBuilder::default().build_testnet().await?;
    println!("Ika testnet version: {}", ika_testnet.api_version());

    println!("{:?}", ika_local.available_rpc_methods());
    println!("{:?}", ika_local.available_subscriptions());

    Ok(())
}
