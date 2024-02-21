// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use async_trait::async_trait;

use crate::models_v2::checkpoints::StoredCheckpoint;
use crate::models_v2::move_call_metrics::StoredMoveCallMetrics;
use crate::models_v2::network_metrics::StoredEpochPeakTps;
use crate::models_v2::transactions::{
    StoredTransaction, StoredTransactionCheckpoint, StoredTransactionSuccessCommandCount,
    StoredTransactionTimestamp, TxSeq,
};
use crate::models_v2::tx_count_metrics::StoredTxCountMetrics;
use crate::types_v2::IndexerResult;

#[async_trait]
pub trait IndexerAnalyticalStore {
    async fn get_latest_stored_transaction(&self) -> IndexerResult<Option<StoredTransaction>>;
    async fn get_latest_stored_checkpoint(&self) -> IndexerResult<Option<StoredCheckpoint>>;
    async fn get_checkpoints_in_range(
        &self,
        start_checkpoint: i64,
        end_checkpoint: i64,
    ) -> IndexerResult<Vec<StoredCheckpoint>>;
    async fn get_tx_timestamps_in_checkpoint_range(
        &self,
        start_checkpoint: i64,
        end_checkpoint: i64,
    ) -> IndexerResult<Vec<StoredTransactionTimestamp>>;
    async fn get_tx_checkpoints_in_checkpoint_range(
        &self,
        start_checkpoint: i64,
        end_checkpoint: i64,
    ) -> IndexerResult<Vec<StoredTransactionCheckpoint>>;
    async fn get_tx_success_cmd_counts_in_checkpoint_range(
        &self,
        start_checkpoint: i64,
        end_checkpoint: i64,
    ) -> IndexerResult<Vec<StoredTransactionSuccessCommandCount>>;
    async fn get_tx(&self, tx_sequence_number: i64) -> IndexerResult<Option<StoredTransaction>>;
    async fn get_cp(&self, sequence_number: i64) -> IndexerResult<Option<StoredCheckpoint>>;

    // for network metrics including TPS and counts of objects etc.
    async fn get_latest_tx_count_metrics(&self) -> IndexerResult<Option<StoredTxCountMetrics>>;
    async fn get_latest_epoch_peak_tps(&self) -> IndexerResult<Option<StoredEpochPeakTps>>;
    fn persist_tx_count_metrics(
        &self,
        start_checkpoint: i64,
        end_checkpoint: i64,
    ) -> IndexerResult<()>;
    async fn persist_epoch_peak_tps(&self, epoch: i64) -> IndexerResult<()>;

    // for address metrics
    async fn get_address_metrics_last_processed_tx_seq(&self) -> IndexerResult<Option<TxSeq>>;
    fn persist_addresses_in_tx_range(
        &self,
        start_tx_seq: i64,
        end_tx_seq: i64,
    ) -> IndexerResult<()>;
    fn persist_active_addresses_in_tx_range(
        &self,
        start_tx_seq: i64,
        end_tx_seq: i64,
    ) -> IndexerResult<()>;
    async fn calculate_and_persist_address_metrics(&self, checkpoint: i64) -> IndexerResult<()>;

    // for move call metrics
    async fn get_latest_move_call_metrics(&self) -> IndexerResult<Option<StoredMoveCallMetrics>>;
    async fn get_latest_move_call_tx_seq(&self) -> IndexerResult<Option<TxSeq>>;
    fn persist_move_calls_in_tx_range(
        &self,
        start_tx_seq: i64,
        end_tx_seq: i64,
    ) -> IndexerResult<()>;
    async fn calculate_and_persist_move_call_metrics(&self, epoch: i64) -> IndexerResult<()>;
}
