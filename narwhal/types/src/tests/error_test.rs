// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use futures::{stream::FuturesUnordered, StreamExt};
use prometheus::IntGauge;

use super::DagError;
use mysten_metrics::metered_channel::{channel, Receiver, Sender, WithPermit};
use std::{future, time::Duration};

pub struct Processor {
    input: Receiver<usize>,
    output: Sender<usize>,
}

impl Processor {
    pub fn new(input: Receiver<usize>, output: Sender<usize>) -> Self {
        Self { input, output }
    }

    pub fn spawn(input: Receiver<usize>, output: Sender<usize>) {
        tokio::spawn(async move {
            let mut processor = Processor::new(input, output);
            processor.run().await;
        });
    }

    pub async fn run(&mut self) {
        let mut waiting = FuturesUnordered::new();

        loop {
            tokio::select! {
                Some(input) = self.input.recv() => {
                    let deliver: future::Ready<Result<usize, DagError>> = future::ready(
                        Ok(input)
                    );
                    waiting.push(deliver)
                }


                Some((permit, Some(res_value))) = self.output.with_permit(waiting.next())  => {
                    permit.send(res_value.unwrap());
                }
            }
        }
    }
}

#[tokio::test]
async fn with_permit_unhappy_case() {
    let counter = IntGauge::new("TEST_COUNTER", "test").unwrap();

    let (tx_inbound, rx_inbound) = channel(100, &counter); // we'll make sure we always have stuff inbound
    let (tx_outbound, mut rx_outbound) = channel(1, &counter); // we'll constrain the output

    Processor::spawn(rx_inbound, tx_outbound);
    // we fill the inbound channel with stuff
    (0..100).for_each(|i| {
        tx_inbound
            .try_send(i)
            .expect("failed to send to inbound channel");
    });

    tokio::time::sleep(Duration::from_secs(1)).await;
    // by now, the outbound channel should fail to deliver permits on each loop pass,
    // whereas the inbound channel is full

    // we now try to receive all the things we can from the outbound channel
    let mut recvd = vec![];
    while let Ok(Some(val)) = tokio::time::timeout(Duration::from_secs(1), rx_outbound.recv()).await
    {
        recvd.push(val);
    }

    assert_eq!(recvd, (0..100).collect::<Vec<usize>>());
}
