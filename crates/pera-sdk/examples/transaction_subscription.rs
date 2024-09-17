// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use futures::stream::StreamExt;
use pera_json_rpc_types::TransactionFilter;
use pera_sdk::PeraClientBuilder;

// This example showcases how to use the Read API to listen
// for transactions. It subscribes to the transactions that
// transfer PERA on the Pera testnet and prints every incoming
// transaction to the console. The program will loop until it
// is force stopped.

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let ws = PeraClientBuilder::default()
        .ws_url("wss://rpc.testnet.pera.io:443")
        .build("https://fullnode.testnet.pera.io:443")
        .await?;
    println!("WS version {:?}", ws.api_version());

    let mut subscribe = ws
        .read_api()
        .subscribe_transaction(TransactionFilter::MoveFunction {
            package: "0x2".parse()?,
            module: Some("pera".to_owned()),
            function: Some("transfer".to_owned()),
        })
        .await?;

    loop {
        println!("{:?}", subscribe.next().await);
    }
}
