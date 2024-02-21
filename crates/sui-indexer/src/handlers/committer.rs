// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::BTreeMap;

use tokio::sync::watch;
use tracing::instrument;

use tap::tap::TapFallible;
use tracing::{error, info};

use sui_types::messages_checkpoint::CheckpointSequenceNumber;

use crate::metrics::IndexerMetrics;

use crate::store::IndexerStoreV2;
use crate::types_v2::IndexerResult;
use crate::IndexerConfig;

use super::{CheckpointDataToCommit, EpochToCommit};

pub async fn start_tx_checkpoint_commit_task<S>(
    state: S,
    metrics: IndexerMetrics,
    config: IndexerConfig,
    tx_indexing_receiver: mysten_metrics::metered_channel::Receiver<CheckpointDataToCommit>,
    commit_notifier: watch::Sender<Option<CheckpointSequenceNumber>>,
) where
    S: IndexerStoreV2 + Clone + Sync + Send + 'static,
{
    use futures::StreamExt;

    info!("Indexer checkpoint commit task started...");
    let checkpoint_commit_batch_size = std::env::var("CHECKPOINT_COMMIT_BATCH_SIZE")
        .unwrap_or(5.to_string())
        .parse::<usize>()
        .unwrap();
    info!("Using checkpoint commit batch size {checkpoint_commit_batch_size}");

    let mut stream = mysten_metrics::metered_channel::ReceiverStream::new(tx_indexing_receiver)
        .ready_chunks(checkpoint_commit_batch_size);

    while let Some(indexed_checkpoint_batch) = stream.next().await {
        if indexed_checkpoint_batch.is_empty() {
            continue;
        }
        if config.skip_db_commit {
            info!(
                "[Checkpoint/Tx] Downloaded and indexed checkpoint {:?} - {:?} successfully, skipping DB commit...",
                indexed_checkpoint_batch.first().map(|c| c.checkpoint.sequence_number),
                indexed_checkpoint_batch.last().map(|c| c.checkpoint.sequence_number),
            );
            continue;
        }

        // split the batch into smaller batches per epoch to handle partitioning
        let mut indexed_checkpoint_batch_per_epoch = vec![];
        for indexed_checkpoint in indexed_checkpoint_batch {
            let epoch = indexed_checkpoint.epoch.clone();
            indexed_checkpoint_batch_per_epoch.push(indexed_checkpoint);
            if epoch.is_some() {
                commit_checkpoints(
                    &state,
                    indexed_checkpoint_batch_per_epoch,
                    epoch,
                    &metrics,
                    &commit_notifier,
                )
                .await;
                indexed_checkpoint_batch_per_epoch = vec![];
            }
        }
        if !indexed_checkpoint_batch_per_epoch.is_empty() {
            commit_checkpoints(
                &state,
                indexed_checkpoint_batch_per_epoch,
                None,
                &metrics,
                &commit_notifier,
            )
            .await;
        }
    }
}

// Unwrap: Caller needs to make sure indexed_checkpoint_batch is not empty
#[instrument(skip_all, fields(
    first = indexed_checkpoint_batch.first().as_ref().unwrap().checkpoint.sequence_number,
    last = indexed_checkpoint_batch.last().as_ref().unwrap().checkpoint.sequence_number
))]
async fn commit_checkpoints<S>(
    state: &S,
    indexed_checkpoint_batch: Vec<CheckpointDataToCommit>,
    epoch: Option<EpochToCommit>,
    metrics: &IndexerMetrics,
    commit_notifier: &watch::Sender<Option<CheckpointSequenceNumber>>,
) where
    S: IndexerStoreV2 + Clone + Sync + Send + 'static,
{
    let mut checkpoint_batch = vec![];
    let mut tx_batch = vec![];
    let mut events_batch = vec![];
    let mut tx_indices_batch = vec![];
    let mut display_updates_batch = BTreeMap::new();
    let mut object_changes_batch = vec![];
    let mut object_history_changes_batch = vec![];
    let mut packages_batch = vec![];

    for indexed_checkpoint in indexed_checkpoint_batch {
        let CheckpointDataToCommit {
            checkpoint,
            transactions,
            events,
            tx_indices,
            display_updates,
            object_changes,
            object_history_changes,
            packages,
            epoch: _,
        } = indexed_checkpoint;
        checkpoint_batch.push(checkpoint);
        tx_batch.push(transactions);
        events_batch.push(events);
        tx_indices_batch.push(tx_indices);
        display_updates_batch.extend(display_updates.into_iter());
        object_changes_batch.push(object_changes);
        object_history_changes_batch.push(object_history_changes);
        packages_batch.push(packages);
    }

    let first_checkpoint_seq = checkpoint_batch.first().as_ref().unwrap().sequence_number;
    let last_checkpoint_seq = checkpoint_batch.last().as_ref().unwrap().sequence_number;

    let guard = metrics.checkpoint_db_commit_latency.start_timer();
    let tx_batch = tx_batch.into_iter().flatten().collect::<Vec<_>>();
    let tx_indices_batch = tx_indices_batch.into_iter().flatten().collect::<Vec<_>>();
    let events_batch = events_batch.into_iter().flatten().collect::<Vec<_>>();
    let packages_batch = packages_batch.into_iter().flatten().collect::<Vec<_>>();
    let checkpoint_num = checkpoint_batch.len();
    let tx_count = tx_batch.len();

    {
        let _step_1_guard = metrics.checkpoint_db_commit_latency_step_1.start_timer();
        let mut persist_tasks = vec![
            state.persist_transactions(tx_batch),
            state.persist_tx_indices(tx_indices_batch),
            state.persist_events(events_batch),
            state.persist_displays(display_updates_batch),
            state.persist_packages(packages_batch),
            state.persist_objects(object_changes_batch.clone()),
            state.persist_object_history(object_history_changes_batch.clone()),
            state.persist_object_snapshot(),
        ];
        if let Some(epoch_data) = epoch.clone() {
            persist_tasks.push(state.persist_epoch(epoch_data));
        }
        futures::future::join_all(persist_tasks)
            .await
            .into_iter()
            .map(|res| {
                if res.is_err() {
                    error!("Failed to persist data with error: {:?}", res);
                }
                res
            })
            .collect::<IndexerResult<Vec<_>>>()
            .expect("Persisting data into DB should not fail.");
    }

    // handle partitioning on epoch boundary
    if let Some(epoch_data) = epoch {
        state
            .advance_epoch(epoch_data)
            .await
            .tap_err(|e| {
                error!("Failed to advance epoch with error: {}", e.to_string());
            })
            .expect("Advancing epochs in DB should not fail.");
        metrics.total_epoch_committed.inc();
    }

    state
        .persist_checkpoints(checkpoint_batch)
        .await
        .tap_err(|e| {
            error!(
                "Failed to persist checkpoint data with error: {}",
                e.to_string()
            );
        })
        .expect("Persisting data into DB should not fail.");
    let elapsed = guard.stop_and_record();

    commit_notifier
        .send(Some(last_checkpoint_seq))
        .expect("Commit watcher should not be closed");

    metrics
        .latest_tx_checkpoint_sequence_number
        .set(last_checkpoint_seq as i64);

    metrics
        .total_tx_checkpoint_committed
        .inc_by(checkpoint_num as u64);
    metrics.total_transaction_committed.inc_by(tx_count as u64);
    info!(
        elapsed,
        "Checkpoint {}-{} committed with {} transactions.",
        first_checkpoint_seq,
        last_checkpoint_seq,
        tx_count,
    );
    metrics
        .transaction_per_checkpoint
        .observe(tx_count as f64 / (last_checkpoint_seq - first_checkpoint_seq + 1) as f64);
    // 1000.0 is not necessarily the batch size, it's to roughly map average tx commit latency to [0.1, 1] seconds,
    // which is well covered by DB_COMMIT_LATENCY_SEC_BUCKETS.
    metrics
        .thousand_transaction_avg_db_commit_latency
        .observe(elapsed * 1000.0 / tx_count as f64);
}
