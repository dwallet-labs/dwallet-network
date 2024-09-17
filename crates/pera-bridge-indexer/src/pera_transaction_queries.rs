// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::sync::Arc;
use std::time::Duration;
use pera_json_rpc_types::PeraTransactionBlockResponseOptions;
use pera_json_rpc_types::PeraTransactionBlockResponseQuery;
use pera_json_rpc_types::TransactionFilter;
use pera_sdk::PeraClient;
use pera_types::digests::TransactionDigest;
use pera_types::PERA_BRIDGE_OBJECT_ID;

use pera_bridge::{metrics::BridgeMetrics, retry_with_max_elapsed_time};
use tracing::{error, info};

use crate::types::RetrievedTransaction;

const QUERY_DURATION: Duration = Duration::from_secs(1);
const SLEEP_DURATION: Duration = Duration::from_secs(5);

pub async fn start_pera_tx_polling_task(
    pera_client: PeraClient,
    mut cursor: Option<TransactionDigest>,
    tx: mysten_metrics::metered_channel::Sender<(
        Vec<RetrievedTransaction>,
        Option<TransactionDigest>,
    )>,
    metrics: Arc<BridgeMetrics>,
) {
    info!("Starting PERA transaction polling task from {:?}", cursor);
    loop {
        let Ok(Ok(results)) = retry_with_max_elapsed_time!(
            pera_client.read_api().query_transaction_blocks(
                PeraTransactionBlockResponseQuery {
                    filter: Some(TransactionFilter::InputObject(PERA_BRIDGE_OBJECT_ID)),
                    options: Some(PeraTransactionBlockResponseOptions::full_content()),
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
        // Unwrap: txes is not empty
        let ckp = txes.last().unwrap().checkpoint;
        tx.send((txes, results.next_cursor))
            .await
            .expect("Failed to send transaction block to process");
        metrics.last_synced_pera_checkpoint.set(ckp as i64);
        cursor = results.next_cursor;
    }
}
