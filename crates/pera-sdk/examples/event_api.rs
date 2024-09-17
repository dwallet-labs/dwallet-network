// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

mod utils;
use futures::stream::StreamExt;
use pera_sdk::rpc_types::EventFilter;
use pera_sdk::PeraClientBuilder;
use utils::{setup_for_write, split_coin_digest};

// This example showcases how to use the Event API.
// At the end of the program it subscribes to the events
// on the Pera testnet and prints every incoming event to
// the console. The program will loop until it is force
// stopped.

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (pera, active_address, _second_address) = setup_for_write().await?;

    println!(" *** Get events *** ");
    // for demonstration purposes, we set to make a transaction
    let digest = split_coin_digest(&pera, &active_address).await?;
    let events = pera.event_api().get_events(digest).await?;
    println!("{:?}", events);
    println!(" *** Get events ***\n ");

    let descending = true;
    let query_events = pera
        .event_api()
        .query_events(EventFilter::All(vec![]), None, Some(5), descending) // query first 5 events in descending order
        .await?;
    println!(" *** Query events *** ");
    println!("{:?}", query_events);
    println!(" *** Query events ***\n ");

    let ws = PeraClientBuilder::default()
        .ws_url("wss://rpc.testnet.pera.io:443")
        .build("https://fullnode.testnet.pera.io:443")
        .await?;
    println!("WS version {:?}", ws.api_version());

    let mut subscribe = ws
        .event_api()
        .subscribe_event(EventFilter::All(vec![]))
        .await?;

    loop {
        println!("{:?}", subscribe.next().await);
    }
}
