// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;
use ika_json_rpc_types::IkaTransactionBlockResponseOptions;
use ika_json_rpc_types::IkaTransactionBlockResponseQuery;
use ika_json_rpc_types::TransactionFilter;
use ika_sdk::IkaClient;
use ika_types::digests::TransactionDigest;
use ika_types::IKA_BRIDGE_OBJECT_ID;

use ika_bridge::retry_with_max_elapsed_time;
use tracing::{error, info};

use crate::types::RetrievedTransaction;

const QUERY_DURATION: Duration = Duration::from_secs(1);
const SLEEP_DURATION: Duration = Duration::from_secs(5);

pub async fn start_ika_tx_polling_task(
    ika_client: IkaClient,
    mut cursor: Option<TransactionDigest>,
    tx: mysten_metrics::metered_channel::Sender<(
        Vec<RetrievedTransaction>,
        Option<TransactionDigest>,
    )>,
) {
    info!("Starting IKA transaction polling task from {:?}", cursor);
    loop {
        let Ok(Ok(results)) = retry_with_max_elapsed_time!(
            ika_client.read_api().query_transaction_blocks(
                IkaTransactionBlockResponseQuery {
                    filter: Some(TransactionFilter::InputObject(IKA_BRIDGE_OBJECT_ID)),
                    options: Some(IkaTransactionBlockResponseOptions::full_content()),
                },
                cursor,
                None,
                false,
            ),
            Duration::from_secs(600)
        ) else {
            error!("Failed to query bridge transactions after retry");
            continue;
        };
        info!("Retrieved {} bridge transactions", results.data.len());
        let txes = match results
            .data
            .into_iter()
            .map(RetrievedTransaction::try_from)
            .collect::<anyhow::Result<Vec<_>>>()
        {
            Ok(data) => data,
            Err(e) => {
                // TOOD: Sometimes fullnode does not return checkpoint strangely. We retry instead of
                // panicking.
                error!(
                    "Failed to convert retrieved transactions to sanitized format: {}",
                    e
                );
                tokio::time::sleep(SLEEP_DURATION).await;
                continue;
            }
        };
        if txes.is_empty() {
            // When there is no more new data, we are caught up, no need to stress the fullnode
            tokio::time::sleep(QUERY_DURATION).await;
            continue;
        }
        tx.send((txes, results.next_cursor))
            .await
            .expect("Failed to send transaction block to process");
        cursor = results.next_cursor;
    }
}
