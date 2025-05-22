// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

mod dwallet_checkpoint_metrics;
mod dwallet_checkpoint_output;

use crate::authority::AuthorityState;
pub use crate::checkpoints::dwallet_checkpoint_metrics::DWalletCheckpointMetrics;
use crate::checkpoints::dwallet_checkpoint_output::{
    CertifiedDWalletCheckpointMessageOutput, DWalletCheckpointOutput,
};
pub use crate::checkpoints::dwallet_checkpoint_output::{
    LogDWalletCheckpointOutput, SendDWalletCheckpointToStateSync,
    SubmitDWalletCheckpointToConsensus,
};
use crate::stake_aggregator::{InsertResult, MultiStakeAggregator};
use mysten_metrics::{monitored_future, monitored_scope};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use sui_types::base_types::ConciseableName;

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;

use ika_types::crypto::AuthorityStrongQuorumSignInfo;
use ika_types::digests::DWalletCheckpointMessageDigest;
use ika_types::error::{IkaError, IkaResult};
use ika_types::message::MessageKind;
use ika_types::message_envelope::Message;
use ika_types::messages_dwallet_checkpoint::SignedDWalletCheckpointMessage;
use ika_types::messages_dwallet_checkpoint::{
    CertifiedDWalletCheckpointMessage, DWalletCheckpointMessage, DWalletCheckpointSequenceNumber,
    DWalletCheckpointSignatureMessage, DWalletCheckpointTimestamp, TrustedDWalletCheckpointMessage,
    VerifiedDWalletCheckpointMessage,
};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use sui_types::base_types::EpochId;
use tokio::{sync::Notify, task::JoinSet, time::timeout};
use tracing::{debug, error, info, instrument, warn};
use typed_store::traits::{TableSummary, TypedStoreDebug};
use typed_store::DBMapUtils;
use typed_store::Map;
use typed_store::{
    rocks::{DBMap, MetricConf},
    TypedStoreError,
};

pub type DWalletCheckpointHeight = u64;

pub struct EpochStats {
    pub dwallet_checkpoint_count: u64,
    pub transaction_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingDWalletCheckpointInfo {
    pub timestamp_ms: DWalletCheckpointTimestamp,
    pub dwallet_checkpoint_height: DWalletCheckpointHeight,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PendingDWalletCheckpoint {
    // This is an enum for future upgradability, though at the moment there is only one variant.
    V1(PendingDWalletCheckpointV1),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingDWalletCheckpointV1 {
    pub messages: Vec<MessageKind>,
    pub details: PendingDWalletCheckpointInfo,
}

impl PendingDWalletCheckpoint {
    pub fn as_v1(&self) -> &PendingDWalletCheckpointV1 {
        match self {
            PendingDWalletCheckpoint::V1(contents) => contents,
        }
    }

    pub fn into_v1(self) -> PendingDWalletCheckpointV1 {
        match self {
            PendingDWalletCheckpoint::V1(contents) => contents,
        }
    }

    pub fn messages(&self) -> &Vec<MessageKind> {
        &self.as_v1().messages
    }

    pub fn details(&self) -> &PendingDWalletCheckpointInfo {
        &self.as_v1().details
    }

    pub fn height(&self) -> DWalletCheckpointHeight {
        self.details().dwallet_checkpoint_height
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuilderDWalletCheckpointMessage {
    pub dwallet_checkpoint_message: DWalletCheckpointMessage,
    // The Height at which this `dwallet_checkpoint` message was built.
    // None for genesis dwallet_checkpoint.
    pub dwallet_checkpoint_height: Option<DWalletCheckpointHeight>,
    pub position_in_commit: usize,
}

#[derive(DBMapUtils)]
pub struct DWalletCheckpointStore {
    /// Maps dwallet_checkpoint contents digest to dwallet_checkpoint contents
    // pub(crate) dwallet_checkpoint_content: DBMap<DWalletCheckpointContentsDigest, DWalletCheckpointContents>,
    /// Maps dwallet_checkpoint message digest to dwallet_checkpoint sequence number
    pub(crate) dwallet_checkpoint_message_sequence_by_digest:
        DBMap<DWalletCheckpointMessageDigest, DWalletCheckpointSequenceNumber>,

    // /// Stores entire dwallet_checkpoint contents from state sync, indexed by sequence number, for
    // /// efficient reads of full dwallet_checkpoints. Entries from this table are deleted after state
    // /// accumulation has completed.
    // full_dwallet_checkpoint_content: DBMap<DWalletCheckpointSequenceNumber, FullDWalletCheckpointContents>,
    /// Stores certified dwallet_checkpoints
    pub(crate) dwallet_certified_checkpoints:
        DBMap<DWalletCheckpointSequenceNumber, TrustedDWalletCheckpointMessage>,
    // /// Map from dwallet_checkpoint digest to certified dwallet_checkpoint
    // pub(crate) dwallet_checkpoint_by_digest: DBMap<DWalletCheckpointMessageDigest, TrustedDWalletCheckpointMessage>,
    /// Store locally computed dwallet_checkpoint summaries so that we can detect forks and log useful
    /// information. Can be pruned as soon as we verify that we are in agreement with the latest
    /// certified dwallet_checkpoint.
    pub(crate) locally_computed_dwallet_checkpoints:
        DBMap<DWalletCheckpointSequenceNumber, DWalletCheckpointMessage>,

    // todo(zeev): why is it not used?
    /// A map from epoch ID to the sequence number of the last dwallet-checkpoint in that epoch.
    #[allow(dead_code)]
    epoch_last_dwallet_checkpoint_map: DBMap<EpochId, DWalletCheckpointSequenceNumber>,

    /// Watermarks used to determine the highest verified, fully synced, and
    /// fully executed dwallet_checkpoints
    pub(crate) watermarks: DBMap<
        DWalletCheckpointWatermark,
        (
            DWalletCheckpointSequenceNumber,
            DWalletCheckpointMessageDigest,
        ),
    >,
}

impl DWalletCheckpointStore {
    pub fn new(path: &Path) -> Arc<Self> {
        Arc::new(Self::open_tables_read_write(
            path.to_path_buf(),
            MetricConf::new("dwallet_checkpoint"),
            None,
            None,
        ))
    }

    pub fn open_readonly(path: &Path) -> DWalletCheckpointStoreReadOnly {
        Self::get_read_only_handle(
            path.to_path_buf(),
            None,
            None,
            MetricConf::new("dwallet_checkpoint_readonly"),
        )
    }

    // #[instrument(level = "info", skip_all)]
    // pub fn insert_genesis_checkpoint(
    //     &self,
    //     checkpoint: VerifiedCheckpointMessage,
    //     contents: CheckpointContents,
    //     epoch_store: &AuthorityPerEpochStore,
    // ) {
    //     assert_eq!(
    //         checkpoint.epoch(),
    //         0,
    //         "can't call insert_genesis_checkpoint with a checkpoint not in epoch 0"
    //     );
    //     assert_eq!(
    //         *checkpoint.sequence_number(),
    //         0,
    //         "can't call insert_genesis_checkpoint with a checkpoint that doesn't have a sequence number of 0"
    //     );
    //
    //     // Only insert the genesis checkpoint if the DB is empty and doesn't have it already
    //     if self
    //         .get_dwallet_checkpoint_by_digest(checkpoint.digest())
    //         .unwrap()
    //         .is_none()
    //     {
    //         if epoch_store.epoch() == checkpoint.epoch {
    //             epoch_store
    //                 .put_genesis_checkpoint_in_builder(checkpoint.data(), &contents)
    //                 .unwrap();
    //         } else {
    //             debug!(
    //                 validator_epoch =% epoch_store.epoch(),
    //                 genesis_epoch =% checkpoint.epoch(),
    //                 "Not inserting checkpoint builder data for genesis checkpoint",
    //             );
    //         }
    //         self.insert_checkpoint_contents(contents).unwrap();
    //         self.insert_verified_checkpoint(&checkpoint).unwrap();
    //         self.update_highest_synced_checkpoint(&checkpoint).unwrap();
    //     }
    // }

    pub fn get_dwallet_checkpoint_by_digest(
        &self,
        digest: &DWalletCheckpointMessageDigest,
    ) -> Result<Option<VerifiedDWalletCheckpointMessage>, TypedStoreError> {
        let sequence = self
            .dwallet_checkpoint_message_sequence_by_digest
            .get(digest)?;
        if let Some(sequence) = sequence {
            self.dwallet_certified_checkpoints
                .get(&sequence)
                .map(|maybe_checkpoint| maybe_checkpoint.map(|c| c.into()))
        } else {
            Ok(None)
        }
    }

    pub fn get_dwallet_checkpoint_by_sequence_number(
        &self,
        sequence_number: DWalletCheckpointSequenceNumber,
    ) -> Result<Option<VerifiedDWalletCheckpointMessage>, TypedStoreError> {
        self.dwallet_certified_checkpoints
            .get(&sequence_number)
            .map(|maybe_checkpoint| maybe_checkpoint.map(|c| c.into()))
    }

    pub fn get_locally_computed_checkpoint(
        &self,
        sequence_number: DWalletCheckpointSequenceNumber,
    ) -> Result<Option<DWalletCheckpointMessage>, TypedStoreError> {
        self.locally_computed_dwallet_checkpoints
            .get(&sequence_number)
    }

    pub fn get_latest_certified_checkpoint(&self) -> Option<VerifiedDWalletCheckpointMessage> {
        self.dwallet_certified_checkpoints
            .unbounded_iter()
            .skip_to_last()
            .next()
            .map(|(_, v)| v.into())
    }

    pub fn get_latest_locally_computed_checkpoint(&self) -> Option<DWalletCheckpointMessage> {
        self.locally_computed_dwallet_checkpoints
            .unbounded_iter()
            .skip_to_last()
            .next()
            .map(|(_, v)| v)
    }

    pub fn multi_get_dwallet_checkpoint_by_sequence_number(
        &self,
        sequence_numbers: &[DWalletCheckpointSequenceNumber],
    ) -> Result<Vec<Option<VerifiedDWalletCheckpointMessage>>, TypedStoreError> {
        let checkpoints = self
            .dwallet_certified_checkpoints
            .multi_get(sequence_numbers)?
            .into_iter()
            .map(|maybe_checkpoint| maybe_checkpoint.map(|c| c.into()))
            .collect();

        Ok(checkpoints)
    }

    pub fn get_highest_verified_dwallet_checkpoint(
        &self,
    ) -> Result<Option<VerifiedDWalletCheckpointMessage>, TypedStoreError> {
        let highest_verified = if let Some(highest_verified) = self
            .watermarks
            .get(&DWalletCheckpointWatermark::HighestVerified)?
        {
            highest_verified
        } else {
            return Ok(None);
        };
        self.get_dwallet_checkpoint_by_digest(&highest_verified.1)
    }

    pub fn get_highest_synced_dwallet_checkpoint(
        &self,
    ) -> Result<Option<VerifiedDWalletCheckpointMessage>, TypedStoreError> {
        let highest_synced = if let Some(highest_synced) = self
            .watermarks
            .get(&DWalletCheckpointWatermark::HighestSynced)?
        {
            highest_synced
        } else {
            return Ok(None);
        };
        self.get_dwallet_checkpoint_by_digest(&highest_synced.1)
    }

    pub fn get_highest_executed_dwallet_checkpoint_seq_number(
        &self,
    ) -> Result<Option<DWalletCheckpointSequenceNumber>, TypedStoreError> {
        if let Some(highest_executed) = self
            .watermarks
            .get(&DWalletCheckpointWatermark::HighestExecuted)?
        {
            Ok(Some(highest_executed.0))
        } else {
            Ok(None)
        }
    }

    pub fn get_highest_executed_dwallet_checkpoint(
        &self,
    ) -> Result<Option<VerifiedDWalletCheckpointMessage>, TypedStoreError> {
        let highest_executed = if let Some(highest_executed) = self
            .watermarks
            .get(&DWalletCheckpointWatermark::HighestExecuted)?
        {
            highest_executed
        } else {
            return Ok(None);
        };
        self.get_dwallet_checkpoint_by_digest(&highest_executed.1)
    }

    pub fn get_highest_pruned_dwallet_checkpoint_seq_number(
        &self,
    ) -> Result<DWalletCheckpointSequenceNumber, TypedStoreError> {
        Ok(self
            .watermarks
            .get(&DWalletCheckpointWatermark::HighestPruned)?
            .unwrap_or_default()
            .0)
    }

    // pub fn get_checkpoint_contents(
    //     &self,
    //     digest: &CheckpointContentsDigest,
    // ) -> Result<Option<CheckpointContents>, TypedStoreError> {
    //     self.checkpoint_content.get(digest)
    // }

    // pub fn get_full_checkpoint_contents_by_sequence_number(
    //     &self,
    //     seq: CheckpointSequenceNumber,
    // ) -> Result<Option<FullCheckpointContents>, TypedStoreError> {
    //     self.full_checkpoint_content.get(&seq)
    // }

    // fn check_for_checkpoint_fork(
    //     &self,
    //     local_checkpoint: &Checkpoint,
    //     verified_checkpoint: &VerifiedCheckpoint,
    // ) {
    //     if local_checkpoint != verified_checkpoint.data() {
    //         let verified_contents = self
    //             .get_checkpoint_contents(&verified_checkpoint.messages)
    //             .map(|opt_contents| {
    //                 opt_contents
    //                     .map(|contents| format!("{:?}", contents))
    //                     .unwrap_or_else(|| {
    //                         format!(
    //                             "Verified checkpoint contents not found, digest: {:?}",
    //                             verified_checkpoint.messages,
    //                         )
    //                     })
    //             })
    //             .map_err(|e| {
    //                 format!(
    //                     "Failed to get verified checkpoint contents, digest: {:?} error: {:?}",
    //                     verified_checkpoint.messages, e
    //                 )
    //             })
    //             .unwrap_or_else(|err_msg| err_msg);
    //
    //         let local_contents = self
    //             .get_checkpoint_contents(&local_checkpoint.messages)
    //             .map(|opt_contents| {
    //                 opt_contents
    //                     .map(|contents| format!("{:?}", contents))
    //                     .unwrap_or_else(|| {
    //                         format!(
    //                             "Local checkpoint contents not found, digest: {:?}",
    //                             local_checkpoint.messages
    //                         )
    //                     })
    //             })
    //             .map_err(|e| {
    //                 format!(
    //                     "Failed to get local checkpoint contents, digest: {:?} error: {:?}",
    //                     local_checkpoint.messages, e
    //                 )
    //             })
    //             .unwrap_or_else(|err_msg| err_msg);
    //
    //         // checkpoint contents may be too large for panic message.
    //         error!(
    //             verified_checkpoint = ?verified_checkpoint.data(),
    //             ?verified_contents,
    //             ?local_checkpoint,
    //             ?local_contents,
    //             "Local checkpoint fork detected!",
    //         );
    //         panic!(
    //             "Local checkpoint fork detected for sequence number: {}",
    //             local_checkpoint.sequence_number()
    //         );
    //     }
    // }

    // Called by consensus (ConsensusAggregator).
    // Different from `insert_verified_checkpoint`, it does not touch
    // the highest_verified_checkpoint watermark such that state sync
    // will have a chance to process this checkpoint and perform some
    // state-sync only things.
    pub fn insert_certified_checkpoint(
        &self,
        checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<(), TypedStoreError> {
        debug!(
            checkpoint_seq = checkpoint.sequence_number(),
            "Inserting certified checkpoint",
        );
        let mut batch = self.dwallet_certified_checkpoints.batch();
        batch.insert_batch(
            &self.dwallet_checkpoint_message_sequence_by_digest,
            [(checkpoint.digest().clone(), checkpoint.sequence_number())],
        )?;
        batch.insert_batch(
            &self.dwallet_certified_checkpoints,
            [(checkpoint.sequence_number(), checkpoint.serializable_ref())],
        )?;
        batch.write()?;

        // if let Some(local_checkpoint) = self
        //     .locally_computed_checkpoints
        //     .get(checkpoint.sequence_number())?
        // {
        //     self.check_for_checkpoint_fork(&local_checkpoint, checkpoint);
        // }

        Ok(())
    }

    // Called by state sync, apart from inserting the checkpoint and updating
    // related tables, it also bumps the highest_verified_checkpoint watermark.
    #[instrument(level = "debug", skip_all)]
    pub fn insert_verified_checkpoint(
        &self,
        checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<(), TypedStoreError> {
        self.insert_certified_checkpoint(checkpoint)?;
        self.update_highest_verified_checkpoint(checkpoint)
    }

    pub fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<(), TypedStoreError> {
        if Some(*checkpoint.sequence_number())
            > self
                .get_highest_verified_dwallet_checkpoint()?
                .map(|x| *x.sequence_number())
        {
            debug!(
                checkpoint_seq = checkpoint.sequence_number(),
                "Updating highest verified checkpoint",
            );
            self.watermarks.insert(
                &DWalletCheckpointWatermark::HighestVerified,
                &(*checkpoint.sequence_number(), *checkpoint.digest()),
            )?;
        }

        Ok(())
    }

    pub fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<(), TypedStoreError> {
        debug!(
            checkpoint_seq = checkpoint.sequence_number(),
            "Updating highest synced checkpoint",
        );
        self.watermarks.insert(
            &DWalletCheckpointWatermark::HighestSynced,
            &(*checkpoint.sequence_number(), *checkpoint.digest()),
        )
    }

    pub fn delete_highest_executed_dwallet_checkpoint_test_only(
        &self,
    ) -> Result<(), TypedStoreError> {
        let mut wb = self.watermarks.batch();
        wb.delete_batch(
            &self.watermarks,
            std::iter::once(DWalletCheckpointWatermark::HighestExecuted),
        )?;
        wb.write()?;
        Ok(())
    }

    pub fn reset_db_for_execution_since_genesis(&self) -> IkaResult {
        self.delete_highest_executed_dwallet_checkpoint_test_only()?;
        self.watermarks.rocksdb.flush()?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DWalletCheckpointWatermark {
    HighestVerified,
    HighestSynced,
    HighestExecuted,
    HighestPruned,
}

pub struct DWalletCheckpointBuilder {
    // todo(zeev): why is it not used?
    #[allow(dead_code)]
    state: Arc<AuthorityState>,
    tables: Arc<DWalletCheckpointStore>,
    epoch_store: Arc<AuthorityPerEpochStore>,
    notify: Arc<Notify>,
    notify_aggregator: Arc<Notify>,
    output: Box<dyn DWalletCheckpointOutput>,
    metrics: Arc<DWalletCheckpointMetrics>,
    max_messages_per_dwallet_checkpoint: usize,
    max_dwallet_checkpoint_size_bytes: usize,
    previous_epoch_last_dwallet_checkpoint_sequence_number: u64,
}

pub struct DWalletCheckpointAggregator {
    tables: Arc<DWalletCheckpointStore>,
    epoch_store: Arc<AuthorityPerEpochStore>,
    notify: Arc<Notify>,
    current: Option<DWalletCheckpointSignatureAggregator>,
    output: Box<dyn CertifiedDWalletCheckpointMessageOutput>,
    state: Arc<AuthorityState>,
    metrics: Arc<DWalletCheckpointMetrics>,
}

// This holds information to aggregate signatures for one dwallet_checkpoint.
pub struct DWalletCheckpointSignatureAggregator {
    next_index: u64,
    dwallet_checkpoint_message: DWalletCheckpointMessage,
    digest: DWalletCheckpointMessageDigest,
    /// Aggregates voting stake for each signed dwallet_checkpoint proposal by authority.
    signatures_by_digest:
        MultiStakeAggregator<DWalletCheckpointMessageDigest, DWalletCheckpointMessage, true>,
    // todo(zeev): why is it not used?
    #[allow(dead_code)]
    tables: Arc<DWalletCheckpointStore>,
    #[allow(dead_code)]
    state: Arc<AuthorityState>,
    metrics: Arc<DWalletCheckpointMetrics>,
}

impl DWalletCheckpointBuilder {
    fn new(
        state: Arc<AuthorityState>,
        tables: Arc<DWalletCheckpointStore>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        notify: Arc<Notify>,
        output: Box<dyn DWalletCheckpointOutput>,
        notify_aggregator: Arc<Notify>,
        metrics: Arc<DWalletCheckpointMetrics>,
        max_messages_per_dwallet_checkpoint: usize,
        max_dwallet_checkpoint_size_bytes: usize,
        previous_epoch_last_dwallet_checkpoint_sequence_number: u64,
    ) -> Self {
        Self {
            state,
            tables,
            epoch_store,
            notify,
            output,
            notify_aggregator,
            metrics,
            max_messages_per_dwallet_checkpoint,
            max_dwallet_checkpoint_size_bytes,
            previous_epoch_last_dwallet_checkpoint_sequence_number,
        }
    }

    async fn run(mut self) {
        info!("Starting DWalletCheckpointBuilder");
        loop {
            self.maybe_build_dwallet_checkpoints().await;

            self.notify.notified().await;
        }
    }

    async fn maybe_build_dwallet_checkpoints(&mut self) {
        let _scope = monitored_scope("BuildDWalletCheckpoints");

        // Collect info about the most recently built dwallet_checkpoint.
        let checkpoint_message = self
            .epoch_store
            .last_built_dwallet_checkpoint_message_builder()
            .expect("epoch should not have ended");
        let mut last_height = checkpoint_message
            .clone()
            .and_then(|s| s.dwallet_checkpoint_height);
        let mut last_timestamp =
            checkpoint_message.map(|s| s.dwallet_checkpoint_message.timestamp_ms);

        let min_checkpoint_interval_ms = self
            .epoch_store
            .protocol_config()
            .min_dwallet_checkpoint_interval_ms_as_option()
            .unwrap_or_default();
        let mut grouped_pending_checkpoints = Vec::new();
        let mut checkpoints_iter = self
            .epoch_store
            .get_pending_checkpoints(last_height)
            .expect("unexpected epoch store error")
            .into_iter()
            .peekable();
        while let Some((height, pending)) = checkpoints_iter.next() {
            // Group PendingCheckpoints until:
            // - minimum interval has elapsed ...
            let current_timestamp = pending.details().timestamp_ms;
            let can_build = match last_timestamp {
                Some(last_timestamp) => {
                    current_timestamp >= last_timestamp + min_checkpoint_interval_ms
                }
                None => true,
            };
            grouped_pending_checkpoints.push(pending);
            if !can_build {
                debug!(
                    checkpoint_commit_height = height,
                    ?last_timestamp,
                    ?current_timestamp,
                    "waiting for more PendingCheckpoints: minimum interval not yet elapsed"
                );
                continue;
            }

            // Min interval has elapsed, we can now coalesce and build a dwallet_checkpoint.
            last_height = Some(height);
            last_timestamp = Some(current_timestamp);
            debug!(
                checkpoint_commit_height = height,
                "Making checkpoint at commit height"
            );
            if let Err(e) = self
                .make_checkpoint(std::mem::take(&mut grouped_pending_checkpoints))
                .await
            {
                error!("Error while making checkpoint, will retry in 1s: {:?}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
                self.metrics.dwallet_checkpoint_errors.inc();
                return;
            }
        }
        debug!(
            "Waiting for more checkpoints from consensus after processing {last_height:?}; {} pending checkpoints left unprocessed until next interval",
            grouped_pending_checkpoints.len(),
        );
    }

    #[instrument(level = "debug", skip_all, fields(last_height = pendings.last().unwrap().details().dwallet_checkpoint_height))]
    async fn make_checkpoint(&self, pendings: Vec<PendingDWalletCheckpoint>) -> anyhow::Result<()> {
        let last_details = pendings.last().unwrap().details().clone();

        // Keeps track of the effects that are already included in the current checkpoint.
        // This is used when there are multiple pending checkpoints to create a single checkpoint
        // because in such scenarios, dependencies of a transaction may in earlier created checkpoints,
        // or in earlier pending checkpoints.
        //let mut effects_in_current_checkpoint = BTreeSet::new();

        // Stores the transactions that should be included in the checkpoint. Transactions will be recorded in the checkpoint
        // in this order.
        let mut sorted_tx_effects_included_in_checkpoint = Vec::new();
        for pending_checkpoint in pendings.into_iter() {
            let pending = pending_checkpoint.into_v1();
            // let txn_in_checkpoint = self
            //     .resolve_checkpoint_transactions(pending.roots, &mut effects_in_current_checkpoint)
            //     .await?;
            sorted_tx_effects_included_in_checkpoint.extend(pending.messages);
        }
        let new_checkpoint = self
            .create_checkpoints(sorted_tx_effects_included_in_checkpoint, &last_details)
            .await?;
        self.write_checkpoints(last_details.dwallet_checkpoint_height, new_checkpoint)
            .await?;
        Ok(())
    }

    // Given the root transactions of a pending checkpoint, resolve the transactions should be included in
    // the checkpoint, and return them in the order they should be included in the checkpoint.
    // `effects_in_current_checkpoint` tracks the transactions that already exist in the current
    // checkpoint.
    // #[instrument(level = "debug", skip_all)]
    // async fn resolve_checkpoint_transactions(
    //     &self,
    //     roots: Vec<MessageDigest>,
    //     effects_in_current_checkpoint: &mut BTreeSet<TransactionDigest>,
    // ) -> IkaResult<Vec<MessageData>> {
    //     self.metrics
    //         .checkpoint_roots_count
    //         .inc_by(roots.len() as u64);
    //
    //     let root_digests = self
    //         .epoch_store
    //         .notify_read_executed_digests(&roots)
    //         .in_monitored_scope("DWalletCheckpointNotifyDigests")
    //         .await?;
    //
    //     let root_effects = self
    //         .effects_store
    //         .notify_read_executed_effects(&root_digests)
    //         .in_monitored_scope("DWalletCheckpointNotifyRead")
    //         .await?;
    //
    //     let _scope = monitored_scope("DWalletCheckpointBuilder");
    //
    //     let consensus_commit_prologue = if self
    //         .epoch_store
    //         .protocol_config()
    //         .prepend_prologue_tx_in_consensus_commit_in_checkpoints()
    //     {
    //         // If the roots contains consensus commit prologue transaction, we want to extract it,
    //         // and put it to the front of the checkpoint.
    //
    //         let consensus_commit_prologue = self
    //             .extract_consensus_commit_prologue(&root_digests, &root_effects)
    //             .await?;
    //
    //         // Get the unincluded depdnencies of the consensus commit prologue. We should expect no
    //         // other dependencies that haven't been included in any previous checkpoints.
    //         if let Some((ccp_digest, ccp_effects)) = &consensus_commit_prologue {
    //             let unsorted_ccp = self.complete_checkpoint_effects(
    //                 vec![ccp_effects.clone()],
    //                 effects_in_current_checkpoint,
    //             )?;
    //
    //             // No other dependencies of this consensus commit prologue that haven't been included
    //             // in any previous checkpoint.
    //             assert_eq!(unsorted_ccp.len(), 1);
    //             assert_eq!(unsorted_ccp[0].transaction_digest(), ccp_digest);
    //         }
    //         consensus_commit_prologue
    //     } else {
    //         None
    //     };
    //
    //     let unsorted =
    //         self.complete_checkpoint_effects(root_effects, effects_in_current_checkpoint)?;
    //
    //     let _scope = monitored_scope("DWalletCheckpointBuilder::causal_sort");
    //     let mut sorted: Vec<TransactionEffects> = Vec::with_capacity(unsorted.len() + 1);
    //     if let Some((ccp_digest, ccp_effects)) = consensus_commit_prologue {
    //         #[cfg(debug_assertions)]
    //         {
    //             // When consensus_commit_prologue is extracted, it should not be included in the `unsorted`.
    //             for tx in unsorted.iter() {
    //                 assert!(tx.transaction_digest() != &ccp_digest);
    //             }
    //         }
    //         sorted.push(ccp_effects);
    //     }
    //     //sorted.extend(CausalOrder::causal_sort(unsorted));
    //
    //     #[cfg(msim)]
    //     {
    //         // Check consensus commit prologue invariants in sim test.
    //         self.expensive_consensus_commit_prologue_invariants_check(&root_digests, &sorted);
    //     }
    //
    //     Ok(sorted)
    // }

    // This function is used to extract the consensus commit prologue digest and effects from the root
    // transactions.
    // This function can only be used when prepend_prologue_tx_in_consensus_commit_in_checkpoints is enabled.
    // The consensus commit prologue is expected to be the first transaction in the roots.
    // async fn extract_consensus_commit_prologue(
    //     &self,
    //     root_digests: &[TransactionDigest],
    //     root_effects: &[TransactionEffects],
    // ) -> IkaResult<Option<(TransactionDigest, TransactionEffects)>> {
    //     let _scope = monitored_scope("DWalletCheckpointBuilder::extract_consensus_commit_prologue");
    //     if root_digests.is_empty() {
    //         return Ok(None);
    //     }
    //
    //     // Reads the first transaction in the roots, and checks whether it is a consensus commit prologue
    //     // transaction.
    //     // When prepend_prologue_tx_in_consensus_commit_in_checkpoints is enabled, the consensus commit prologue
    //     // transaction should be the first transaction in the roots written by the consensus handler.
    //     let first_tx = self
    //         .state
    //         .get_transaction_cache_reader()
    //         .get_transaction_block(&root_digests[0])?
    //         .expect("Transaction block must exist");
    //
    //     Ok(match first_tx.transaction_data().kind() {
    //         TransactionKind::ConsensusCommitPrologue(_)
    //         | TransactionKind::ConsensusCommitPrologueV2(_)
    //         | TransactionKind::ConsensusCommitPrologueV3(_) => {
    //             assert_eq!(first_tx.digest(), root_effects[0].transaction_digest());
    //             Some((*first_tx.digest(), root_effects[0].clone()))
    //         }
    //         _ => None,
    //     })
    // }

    #[instrument(level = "debug", skip_all)]
    async fn write_checkpoints(
        &self,
        height: DWalletCheckpointHeight,
        new_checkpoints: Vec<DWalletCheckpointMessage>,
    ) -> IkaResult {
        let _scope = monitored_scope("DWalletCheckpointBuilder::write_checkpoints");
        //let mut batch = self.tables.checkpoint_content.batch();
        // let mut all_tx_digests =
        //     Vec::with_capacity(new_checkpoints.iter().map(|(_, c)| c.size()).sum());

        for checkpoint_message in &new_checkpoints {
            debug!(
                checkpoint_commit_height = height,
                checkpoint_seq = checkpoint_message.sequence_number,
                checkpoint_digest = ?checkpoint_message.digest(),
                "writing checkpoint",
            );
            //all_tx_digests.extend(contents.iter().map(|digest| digest));

            self.output
                .dwallet_checkpoint_created(checkpoint_message, &self.epoch_store, &self.tables)
                .await?;

            self.metrics
                .messages_included_in_dwallet_checkpoint
                .inc_by(checkpoint_message.messages.len() as u64);
            let sequence_number = checkpoint_message.sequence_number;
            self.metrics
                .last_constructed_dwallet_checkpoint
                .set(sequence_number as i64);

            // batch.insert_batch(
            //     &self.tables.checkpoint_content,
            //     [(contents.digest(), contents)],
            // )?;

            self.tables
                .locally_computed_dwallet_checkpoints
                .insert(&sequence_number, checkpoint_message)?;

            // batch.insert_batch(
            //     &self.tables.locally_computed_checkpoints,
            //     [(sequence_number, summary)],
            // )?;
        }

        // Durably commit transactions (but not their outputs) to the database.
        // Called before writing a locally built checkpoint to the CheckpointStore, so that
        // the inputs of the checkpoint cannot be lost.
        // These transactions are guaranteed to be final unless this validator
        // forks (i.e. constructs a checkpoint which will never be certified). In this case
        // some non-final transactions could be left in the database.
        //
        // This is an intermediate solution until we delay commits to the epoch db. After
        // we have done that, crash recovery will be done by re-processing consensus commits
        // and pending_consensus_transactions, and this method can be removed.
        // self.state
        //     .get_cache_commit()
        //     .persist_transactions(&all_tx_digests)
        //     .await?;

        //batch.write()?;

        // for (local_checkpoint, _) in &new_checkpoints {
        //     if let Some(certified_checkpoint) = self
        //         .tables
        //         .certified_checkpoints
        //         .get(local_checkpoint.sequence_number())?
        //     {
        //         self.tables
        //             .check_for_checkpoint_fork(local_checkpoint, &certified_checkpoint.into());
        //     }
        // }

        self.notify_aggregator.notify_one();
        self.epoch_store
            .process_pending_checkpoint(height, new_checkpoints)?;
        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn split_checkpoint_chunks(
        &self,
        messages: Vec<MessageKind>,
    ) -> anyhow::Result<Vec<Vec<MessageKind>>> {
        let _guard = monitored_scope("DWalletCheckpointBuilder::split_checkpoint_chunks");
        let mut chunks = Vec::new();
        let mut chunk = Vec::new();
        let mut chunk_size: usize = 0;
        for message in messages {
            // Roll over to a new chunk after either max count or max size is reached.
            // The size calculation here is intended to estimate the size of the
            // FullDWalletCheckpointContents struct. If this code is modified, that struct
            // should also be updated accordingly.
            let size = bcs::serialized_size(&message)?;
            if chunk.len() == self.max_messages_per_dwallet_checkpoint
                || (chunk_size + size) > self.max_dwallet_checkpoint_size_bytes
            {
                if chunk.is_empty() {
                    // Always allow at least one tx in a checkpoint.
                    warn!("Size of single transaction ({size}) exceeds max checkpoint size ({}); allowing excessively large checkpoint to go through.", self.max_dwallet_checkpoint_size_bytes);
                } else {
                    chunks.push(chunk);
                    chunk = Vec::new();
                    chunk_size = 0;
                }
            }

            chunk.push(message);
            chunk_size += size;
        }

        if !chunk.is_empty() {
            // We intentionally create an empty chunk if there is no content provided
            // and `last_of_epoch` or `mid_of_epoch` to make an end of epoch message.
            // Important: if some conditions are added here later, we need to make sure we always
            // have at least one chunk if last_pending_of_epoch is set
            chunks.push(chunk);
            // Note: empty checkpoints are ok - they shouldn't happen at all on a network with even
            // modest load.
        }
        Ok(chunks)
    }

    #[instrument(level = "debug", skip_all)]
    async fn create_checkpoints(
        &self,
        all_messages: Vec<MessageKind>,
        details: &PendingDWalletCheckpointInfo,
    ) -> anyhow::Result<Vec<DWalletCheckpointMessage>> {
        let _scope = monitored_scope("DWalletCheckpointBuilder::create_checkpoints");
        let epoch = self.epoch_store.epoch();
        let total = all_messages.len();
        let mut last_checkpoint = self.epoch_store.last_built_checkpoint_message()?;
        // if last_checkpoint.is_none() {
        //     let epoch = self.epoch_store.epoch();
        //     if epoch > 0 {
        //         let previous_epoch = epoch - 1;
        //         let last_verified = self.tables.get_epoch_last_checkpoint(previous_epoch)?;
        //         last_checkpoint = last_verified.map(VerifiedCheckpointMessage::into_summary_and_sequence);
        //         if let Some((ref seq, _)) = last_checkpoint {
        //             debug!("No checkpoints in builder DB, taking checkpoint from previous epoch with sequence {seq}");
        //         } else {
        //             // This is some serious bug with when CheckpointBuilder started so surfacing it via panic
        //             panic!("Can not find last checkpoint for previous epoch {previous_epoch}");
        //         }
        //     }
        // }
        let mut last_checkpoint_seq = last_checkpoint.as_ref().map(|(seq, _)| *seq);
        // Epoch 0 is where we create the validator set (we are not running Epoch 0).
        // Once we initialize, the active committee starts in Epoch 1.
        // So there is no previous committee in epoch 1.
        if epoch != 1 && last_checkpoint_seq.is_none() {
            last_checkpoint_seq = Some(self.previous_epoch_last_dwallet_checkpoint_sequence_number);
        }
        info!(
            next_dwallet_checkpoint_seq = last_checkpoint_seq.map(|s| s + 1).unwrap_or(0),
            checkpoint_timestamp = details.timestamp_ms,
            "Creating dwallet checkpoint(s) for {} messages",
            all_messages.len(),
        );

        // let all_digests: Vec<_> = all_messages
        //     .iter()
        //     .map(|message| message.digest())
        //     .collect();
        // let mut all_effects_and_transaction_sizes = Vec::with_capacity(all_messages.len());
        // let mut transactions = Vec::with_capacity(all_messages.len());
        // let mut transaction_keys = Vec::with_capacity(all_messages.len());
        // {
        //     let _guard = monitored_scope("DWalletCheckpointBuilder::wait_for_transactions_sequenced");
        //     debug!(
        //         ?last_checkpoint_seq,
        //         "Waiting for {:?} certificates to appear in consensus",
        //         all_messages.len()
        //     );

        //     for (effects, transaction_and_size) in all_messages
        //         .into_iter()
        //         .zip(transactions_and_sizes.into_iter())
        //     {
        //         let (transaction, size) = transaction_and_size
        //             .unwrap_or_else(|| panic!("Could not find executed transaction {:?}", effects));
        //         match transaction.inner().transaction_data().kind() {
        //             TransactionKind::ConsensusCommitPrologue(_)
        //             | TransactionKind::ConsensusCommitPrologueV2(_)
        //             | TransactionKind::ConsensusCommitPrologueV3(_)
        //             | TransactionKind::AuthenticatorStateUpdate(_) => {
        //                 // ConsensusCommitPrologue and AuthenticatorStateUpdate are guaranteed to be
        //                 // processed before we reach here.
        //             }
        //             TransactionKind::RandomnessStateUpdate(rsu) => {
        //                 randomness_rounds
        //                     .insert(*effects.transaction_digest(), rsu.randomness_round);
        //             }
        //             _ => {
        //                 // All other tx should be included in the call to
        //                 // `consensus_messages_processed_notify`.
        //                 transaction_keys.push(SequencedConsensusTransactionKey::External(
        //                     ConsensusTransactionKey::Certificate(*effects.transaction_digest()),
        //                 ));
        //             }
        //         }
        //         transactions.push(transaction);
        //         all_effects_and_transaction_sizes.push((effects, size));
        //     }
        //
        //     self.epoch_store
        //         .consensus_messages_processed_notify(transaction_keys)
        //         .await?;
        // }

        let chunks = self.split_checkpoint_chunks(all_messages)?;
        let chunks_count = chunks.len();

        let mut checkpoints = Vec::with_capacity(chunks_count);
        debug!(
            ?last_checkpoint_seq,
            "Creating {} checkpoints with {} transactions", chunks_count, total,
        );

        for (index, messages) in chunks.into_iter().enumerate() {
            let first_checkpoint_of_epoch = index == 0
                && (last_checkpoint_seq.is_none()
                    || last_checkpoint_seq.unwrap()
                        == self.previous_epoch_last_dwallet_checkpoint_sequence_number);
            if first_checkpoint_of_epoch {
                self.epoch_store
                    .record_epoch_first_checkpoint_creation_time_metric();
            }

            let sequence_number = last_checkpoint_seq.map(|s| s + 1).unwrap_or(0);
            last_checkpoint_seq = Some(sequence_number);

            let timestamp_ms = details.timestamp_ms;
            if let Some((_, last_checkpoint)) = &last_checkpoint {
                if last_checkpoint.timestamp_ms > timestamp_ms {
                    error!("Unexpected decrease of checkpoint timestamp, sequence: {}, previous: {}, current: {}",
                    sequence_number,  last_checkpoint.timestamp_ms, timestamp_ms);
                }
            }

            info!(
                "dWallet Checkpoint sequence: {}, messages count: {}",
                sequence_number,
                messages.len()
            );

            let dwallet_checkpoint_message =
                DWalletCheckpointMessage::new(epoch, sequence_number, messages, timestamp_ms);
            dwallet_checkpoint_message
                .report_dwallet_checkpoint_age(&self.metrics.last_created_dwallet_checkpoint_age);
            last_checkpoint = Some((sequence_number, dwallet_checkpoint_message.clone()));
            checkpoints.push(dwallet_checkpoint_message);
        }

        Ok(checkpoints)
    }

    // This function is used to check the invariants of the consensus commit prologue transactions in the checkpoint
    // in simtest.
    #[cfg(msim)]
    fn expensive_consensus_commit_prologue_invariants_check(
        &self,
        root_digests: &[TransactionDigest],
        sorted: &[TransactionEffects],
    ) {
        if !self
            .epoch_store
            .protocol_config()
            .prepend_prologue_tx_in_consensus_commit_in_checkpoints()
        {
            return;
        }

        // Gets all the consensus commit prologue transactions from the roots.
        let root_txs = self
            .state
            .get_transaction_cache_reader()
            .multi_get_transaction_blocks(root_digests)
            .unwrap();
        let ccps = root_txs
            .iter()
            .filter_map(|tx| {
                if let Some(tx) = tx {
                    if matches!(
                        tx.transaction_data().kind(),
                        TransactionKind::ConsensusCommitPrologue(_)
                            | TransactionKind::ConsensusCommitPrologueV2(_)
                            | TransactionKind::ConsensusCommitPrologueV3(_)
                    ) {
                        Some(tx)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // There should be at most one consensus commit prologue transaction in the roots.
        assert!(ccps.len() <= 1);

        // Get all the transactions in the checkpoint.
        let txs = self
            .state
            .get_transaction_cache_reader()
            .multi_get_transaction_blocks(
                &sorted
                    .iter()
                    .map(|tx| tx.transaction_digest().clone())
                    .collect::<Vec<_>>(),
            )
            .unwrap();

        if ccps.len() == 0 {
            // If there is no consensus commit prologue transaction in the roots, then there should be no
            // consensus commit prologue transaction in the checkpoint.
            for tx in txs.iter() {
                if let Some(tx) = tx {
                    assert!(!matches!(
                        tx.transaction_data().kind(),
                        TransactionKind::ConsensusCommitPrologue(_)
                            | TransactionKind::ConsensusCommitPrologueV2(_)
                            | TransactionKind::ConsensusCommitPrologueV3(_)
                    ));
                }
            }
        } else {
            // If there is one consensus commit prologue, it must be the first one in the checkpoint.
            assert!(matches!(
                txs[0].as_ref().unwrap().transaction_data().kind(),
                TransactionKind::ConsensusCommitPrologue(_)
                    | TransactionKind::ConsensusCommitPrologueV2(_)
                    | TransactionKind::ConsensusCommitPrologueV3(_)
            ));

            assert_eq!(ccps[0].digest(), txs[0].as_ref().unwrap().digest());

            for tx in txs.iter().skip(1) {
                if let Some(tx) = tx {
                    assert!(!matches!(
                        tx.transaction_data().kind(),
                        TransactionKind::ConsensusCommitPrologue(_)
                            | TransactionKind::ConsensusCommitPrologueV2(_)
                            | TransactionKind::ConsensusCommitPrologueV3(_)
                    ));
                }
            }
        }
    }
}

impl DWalletCheckpointAggregator {
    fn new(
        tables: Arc<DWalletCheckpointStore>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        notify: Arc<Notify>,
        output: Box<dyn CertifiedDWalletCheckpointMessageOutput>,
        state: Arc<AuthorityState>,
        metrics: Arc<DWalletCheckpointMetrics>,
    ) -> Self {
        let current = None;
        Self {
            tables,
            epoch_store,
            notify,
            current,
            output,
            state,
            metrics,
        }
    }

    async fn run(mut self) {
        info!("Starting CheckpointAggregator");
        loop {
            if let Err(e) = self.run_and_notify().await {
                error!(
                    "Error while aggregating checkpoint, will retry in 1s: {:?}",
                    e
                );
                self.metrics.dwallet_checkpoint_errors.inc();
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }

            let _ = timeout(Duration::from_secs(1), self.notify.notified()).await;
        }
    }

    async fn run_and_notify(&mut self) -> IkaResult {
        let checkpoint_messages = self.run_inner()?;
        for checkpoint_message in checkpoint_messages {
            self.output
                .certified_dwallet_checkpoint_message_created(&checkpoint_message)
                .await?;
        }
        Ok(())
    }

    fn run_inner(&mut self) -> IkaResult<Vec<CertifiedDWalletCheckpointMessage>> {
        let _scope = monitored_scope("DWalletCheckpointAggregator");
        let mut result = vec![];
        'outer: loop {
            let next_to_certify = self.next_checkpoint_to_certify();
            let current = if let Some(current) = &mut self.current {
                // It's possible that the checkpoint was already certified by
                // the rest of the network, and we've already received the
                // certified checkpoint via StateSync. In this case, we reset
                // the current signature aggregator to the next checkpoint to
                // be certified
                if current.dwallet_checkpoint_message.sequence_number < next_to_certify {
                    self.current = None;
                    continue;
                }
                current
            } else {
                let Some(checkpoint_message) = self
                    .epoch_store
                    .get_built_checkpoint_message(next_to_certify)?
                else {
                    return Ok(result);
                };
                self.current = Some(DWalletCheckpointSignatureAggregator {
                    next_index: 0,
                    digest: checkpoint_message.digest(),
                    dwallet_checkpoint_message: checkpoint_message,
                    signatures_by_digest: MultiStakeAggregator::new(
                        self.epoch_store.committee().clone(),
                    ),
                    tables: self.tables.clone(),
                    state: self.state.clone(),
                    metrics: self.metrics.clone(),
                });
                self.current.as_mut().unwrap()
            };

            let epoch_tables = self
                .epoch_store
                .tables()
                .expect("should not run past end of epoch");
            let iter = epoch_tables.get_pending_checkpoint_signatures_iter(
                current.dwallet_checkpoint_message.sequence_number,
                current.next_index,
            )?;
            for ((seq, index), data) in iter {
                if seq != current.dwallet_checkpoint_message.sequence_number {
                    debug!(
                        checkpoint_seq =? current.dwallet_checkpoint_message.sequence_number,
                        "Not enough checkpoint signatures",
                    );
                    // No more signatures (yet) for this checkpoint
                    return Ok(result);
                }
                debug!(
                    checkpoint_seq = current.dwallet_checkpoint_message.sequence_number,
                    "Processing signature for checkpoint (digest: {:?}) from {:?}",
                    current.dwallet_checkpoint_message.digest(),
                    data.dwallet_checkpoint_message
                        .auth_sig()
                        .authority
                        .concise()
                );
                self.metrics
                    .dwallet_checkpoint_participation
                    .with_label_values(&[&format!(
                        "{:?}",
                        data.dwallet_checkpoint_message
                            .auth_sig()
                            .authority
                            .concise()
                    )])
                    .inc();
                if let Ok(auth_signature) = current.try_aggregate(data) {
                    let checkpoint_message = VerifiedDWalletCheckpointMessage::new_unchecked(
                        CertifiedDWalletCheckpointMessage::new_from_data_and_sig(
                            current.dwallet_checkpoint_message.clone(),
                            auth_signature,
                        ),
                    );

                    self.tables
                        .insert_certified_checkpoint(&checkpoint_message)?;
                    self.metrics
                        .last_certified_dwallet_checkpoint
                        .set(current.dwallet_checkpoint_message.sequence_number as i64);
                    current
                        .dwallet_checkpoint_message
                        .report_dwallet_checkpoint_age(
                            &self.metrics.last_certified_dwallet_checkpoint_age,
                        );
                    result.push(checkpoint_message.into_inner());
                    self.current = None;
                    continue 'outer;
                } else {
                    current.next_index = index + 1;
                }
            }
            break;
        }
        Ok(result)
    }

    fn next_checkpoint_to_certify(&self) -> DWalletCheckpointSequenceNumber {
        self.tables
            .dwallet_certified_checkpoints
            .unbounded_iter()
            .skip_to_last()
            .next()
            .map(|(seq, _)| seq + 1)
            .unwrap_or_default()
    }
}

impl DWalletCheckpointSignatureAggregator {
    #[allow(clippy::result_unit_err)]
    pub fn try_aggregate(
        &mut self,
        data: DWalletCheckpointSignatureMessage,
    ) -> Result<AuthorityStrongQuorumSignInfo, ()> {
        let their_digest = *data.dwallet_checkpoint_message.digest();
        let (_, signature) = data.dwallet_checkpoint_message.into_data_and_sig();
        let author = signature.authority;
        let envelope = SignedDWalletCheckpointMessage::new_from_data_and_sig(
            self.dwallet_checkpoint_message.clone(),
            signature,
        );
        match self.signatures_by_digest.insert(their_digest, envelope) {
            // ignore repeated signatures
            InsertResult::Failed {
                error:
                    IkaError::StakeAggregatorRepeatedSigner {
                        conflicting_sig: false,
                        ..
                    },
            } => Err(()),
            InsertResult::Failed { error } => {
                warn!(
                    checkpoint_seq = self.dwallet_checkpoint_message.sequence_number,
                    "Failed to aggregate new signature from validator {:?}: {:?}",
                    author.concise(),
                    error
                );
                //self.check_for_split_brain();
                Err(())
            }
            InsertResult::QuorumReached(cert) => {
                // It is not guaranteed that signature.authority == narwhal_cert.author, but we do verify
                // the signature so we know that the author signed the message at some point.
                if their_digest != self.digest {
                    self.metrics.remote_dwallet_checkpoint_forks.inc();
                    warn!(
                        checkpoint_seq = self.dwallet_checkpoint_message.sequence_number,
                        "Validator {:?} has mismatching checkpoint digest {}, we have digest {}",
                        author.concise(),
                        their_digest,
                        self.digest
                    );
                    return Err(());
                }
                Ok(cert)
            }
            InsertResult::NotEnoughVotes {
                bad_votes: _,
                bad_authorities: _,
            } => {
                //self.check_for_split_brain();
                Err(())
            }
        }
    }

    // /// Check if there is a split brain condition in checkpoint signature aggregation, defined
    // /// as any state wherein it is no longer possible to achieve quorum on a checkpoint proposal,
    // /// irrespective of the outcome of any outstanding votes.
    // fn check_for_split_brain(&self) {
    //     debug!(
    //         checkpoint_seq = self.summary.sequence_number,
    //         "Checking for split brain condition"
    //     );
    //     if self.signatures_by_digest.quorum_unreachable() {
    //         // TODO: at this point we should immediately halt processing
    //         // of new transaction certificates to avoid building on top of
    //         // forked output
    //         // self.halt_all_execution();
    //
    //         let digests_by_stake_messages = self
    //             .signatures_by_digest
    //             .get_all_unique_values()
    //             .into_iter()
    //             .sorted_by_key(|(_, (_, stake))| -(*stake as i64))
    //             .map(|(digest, (_authorities, total_stake))| {
    //                 format!("{:?} (total stake: {})", digest, total_stake)
    //             })
    //             .collect::<Vec<String>>();
    //         error!(
    //             checkpoint_seq = self.summary.sequence_number,
    //             "Split brain detected in checkpoint signature aggregation! Remaining stake: {:?}, Digests by stake: {:?}",
    //             self.signatures_by_digest.uncommitted_stake(),
    //             digests_by_stake_messages,
    //         );
    //         self.metrics.split_brain_checkpoint_forks.inc();
    //
    //         let all_unique_values = self.signatures_by_digest.get_all_unique_values();
    //         let local_summary = self.summary.clone();
    //         let state = self.state.clone();
    //         let tables = self.tables.clone();
    //
    //         tokio::spawn(async move {
    //             diagnose_split_brain(all_unique_values, local_summary, state, tables).await;
    //         });
    //     }
    // }
}

// /// Create data dump containing relevant data for diagnosing cause of the
// /// split brain by querying one disagreeing validator for full checkpoint contents.
// /// To minimize peer chatter, we only query one validator at random from each
// /// disagreeing faction, as all honest validators that participated in this round may
// /// inevitably run the same process.
// async fn diagnose_split_brain(
//     all_unique_values: BTreeMap<CheckpointDigest, (Vec<AuthorityName>, StakeUnit)>,
//     local_summary: CheckpointSummary,
//     state: Arc<AuthorityState>,
//     tables: Arc<CheckpointStore>,
// ) {
//     debug!(
//         checkpoint_seq = local_summary.sequence_number,
//         "Running split brain diagnostics..."
//     );
//     let time = Utc::now();
//     // collect one random disagreeing validator per differing digest
//     let digest_to_validator = all_unique_values
//         .iter()
//         .filter_map(|(digest, (validators, _))| {
//             if *digest != local_summary.digest() {
//                 let random_validator = validators.choose(&mut OsRng).unwrap();
//                 Some((*digest, *random_validator))
//             } else {
//                 None
//             }
//         })
//         .collect::<HashMap<_, _>>();
//     if digest_to_validator.is_empty() {
//         panic!(
//             "Given split brain condition, there should be at \
//                 least one validator that disagrees with local signature"
//         );
//     }
//
//     let epoch_store = state.load_epoch_store_one_call_per_task();
//     let committee = epoch_store
//         .epoch_start_state()
//         .get_ika_committee_with_network_metadata();
//     let network_config = default_mysten_network_config();
//     let network_clients =
//         make_network_authority_clients_with_network_config(&committee, &network_config);
//
//     // Query all disagreeing validators
//     let response_futures = digest_to_validator
//         .values()
//         .cloned()
//         .map(|validator| {
//             let client = network_clients
//                 .get(&validator)
//                 .expect("Failed to get network client");
//             let request = CheckpointRequestV2 {
//                 sequence_number: Some(local_summary.sequence_number),
//                 request_content: true,
//                 certified: false,
//             };
//             client.handle_checkpoint_v2(request)
//         })
//         .collect::<Vec<_>>();
//
//     let digest_name_pair = digest_to_validator.iter();
//     let response_data = futures::future::join_all(response_futures)
//         .await
//         .into_iter()
//         .zip(digest_name_pair)
//         .filter_map(|(response, (digest, name))| match response {
//             Ok(response) => match response {
//                 CheckpointResponseV2 {
//                     checkpoint: Some(CheckpointSummaryResponse::Pending(summary)),
//                     contents: Some(contents),
//                 } => Some((*name, *digest, summary, contents)),
//                 CheckpointResponseV2 {
//                     checkpoint: Some(CheckpointSummaryResponse::Certified(_)),
//                     contents: _,
//                 } => {
//                     panic!("Expected pending checkpoint, but got certified checkpoint");
//                 }
//                 CheckpointResponseV2 {
//                     checkpoint: None,
//                     contents: _,
//                 } => {
//                     error!(
//                         "Summary for checkpoint {:?} not found on validator {:?}",
//                         local_summary.sequence_number, name
//                     );
//                     None
//                 }
//                 CheckpointResponseV2 {
//                     checkpoint: _,
//                     contents: None,
//                 } => {
//                     error!(
//                         "Contents for checkpoint {:?} not found on validator {:?}",
//                         local_summary.sequence_number, name
//                     );
//                     None
//                 }
//             },
//             Err(e) => {
//                 error!(
//                     "Failed to get checkpoint contents from validator for fork diagnostics: {:?}",
//                     e
//                 );
//                 None
//             }
//         })
//         .collect::<Vec<_>>();
//
//     let local_checkpoint_contents = tables
//         .get_checkpoint_contents(&local_summary.content_digest)
//         .unwrap_or_else(|_| {
//             panic!(
//                 "Could not find checkpoint contents for digest {:?}",
//                 local_summary.digest()
//             )
//         })
//         .unwrap_or_else(|| {
//             panic!(
//                 "Could not find local full checkpoint contents for checkpoint {:?}, digest {:?}",
//                 local_summary.sequence_number,
//                 local_summary.digest()
//             )
//         });
//     let local_contents_text = format!("{local_checkpoint_contents:?}");
//
//     let local_summary_text = format!("{local_summary:?}");
//     let local_validator = state.name.concise();
//     let diff_patches = response_data
//         .iter()
//         .map(|(name, other_digest, other_summary, contents)| {
//             let other_contents_text = format!("{contents:?}");
//             let other_summary_text = format!("{other_summary:?}");
//             let (local_transactions, local_effects): (Vec<_>, Vec<_>) = local_checkpoint_contents
//                 .enumerate_transactions(&local_summary)
//                 .map(|(_, exec_digest)| (exec_digest.transaction, exec_digest.effects))
//                 .unzip();
//             let (other_transactions, other_effects): (Vec<_>, Vec<_>) = contents
//                 .enumerate_transactions(other_summary)
//                 .map(|(_, exec_digest)| (exec_digest.transaction, exec_digest.effects))
//                 .unzip();
//             let summary_patch = create_patch(&local_summary_text, &other_summary_text);
//             let contents_patch = create_patch(&local_contents_text, &other_contents_text);
//             let local_transactions_text = format!("{local_transactions:#?}");
//             let other_transactions_text = format!("{other_transactions:#?}");
//             let transactions_patch =
//                 create_patch(&local_transactions_text, &other_transactions_text);
//             let local_effects_text = format!("{local_effects:#?}");
//             let other_effects_text = format!("{other_effects:#?}");
//             let effects_patch = create_patch(&local_effects_text, &other_effects_text);
//             let seq_number = local_summary.sequence_number;
//             let local_digest = local_summary.digest();
//             let other_validator = name.concise();
//             format!(
//                 "Checkpoint: {seq_number:?}\n\
//                 Local validator (original): {local_validator:?}, digest: {local_digest:?}\n\
//                 Other validator (modified): {other_validator:?}, digest: {other_digest:?}\n\n\
//                 Summary Diff: \n{summary_patch}\n\n\
//                 Contents Diff: \n{contents_patch}\n\n\
//                 Transactions Diff: \n{transactions_patch}\n\n\
//                 Effects Diff: \n{effects_patch}",
//             )
//         })
//         .collect::<Vec<_>>()
//         .join("\n\n\n");
//
//     let header = format!(
//         "Checkpoint Fork Dump - Authority {local_validator:?}: \n\
//         Datetime: {time}",
//     );
//     let fork_logs_text = format!("{header}\n\n{diff_patches}\n\n");
//     let path = tempfile::tempdir()
//         .expect("Failed to create tempdir")
//         .into_path()
//         .join(Path::new("checkpoint_fork_dump.txt"));
//     let mut file = File::create(path).unwrap();
//     write!(file, "{}", fork_logs_text).unwrap();
//     debug!("{}", fork_logs_text);
//
//     fail_point!("split_brain_reached");
// }

pub trait DWalletCheckpointServiceNotify {
    fn notify_checkpoint_signature(
        &self,
        epoch_store: &AuthorityPerEpochStore,
        info: &DWalletCheckpointSignatureMessage,
    ) -> IkaResult;

    fn notify_checkpoint(&self) -> IkaResult;
}

/// This is a service used to communicate with other pieces of ika(for ex. authority)
pub struct DWalletCheckpointService {
    tables: Arc<DWalletCheckpointStore>,
    notify_builder: Arc<Notify>,
    notify_aggregator: Arc<Notify>,
    last_signature_index: Mutex<u64>,
    metrics: Arc<DWalletCheckpointMetrics>,
}

impl DWalletCheckpointService {
    pub fn spawn(
        state: Arc<AuthorityState>,
        checkpoint_store: Arc<DWalletCheckpointStore>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        checkpoint_output: Box<dyn DWalletCheckpointOutput>,
        certified_checkpoint_output: Box<dyn CertifiedDWalletCheckpointMessageOutput>,
        metrics: Arc<DWalletCheckpointMetrics>,
        max_messages_per_checkpoint: usize,
        max_checkpoint_size_bytes: usize,
        previous_epoch_last_checkpoint_sequence_number: u64,
    ) -> (Arc<Self>, JoinSet<()> /* Handle to tasks */) {
        info!(
            "Starting checkpoint service with {max_messages_per_checkpoint} max_messages_per_checkpoint and {max_checkpoint_size_bytes} max_checkpoint_size_bytes"
        );
        let notify_builder = Arc::new(Notify::new());
        let notify_aggregator = Arc::new(Notify::new());

        let mut tasks = JoinSet::new();

        let builder = DWalletCheckpointBuilder::new(
            state.clone(),
            checkpoint_store.clone(),
            epoch_store.clone(),
            notify_builder.clone(),
            checkpoint_output,
            notify_aggregator.clone(),
            metrics.clone(),
            max_messages_per_checkpoint,
            max_checkpoint_size_bytes,
            previous_epoch_last_checkpoint_sequence_number,
        );
        tasks.spawn(monitored_future!(builder.run()));

        let aggregator = DWalletCheckpointAggregator::new(
            checkpoint_store.clone(),
            epoch_store.clone(),
            notify_aggregator.clone(),
            certified_checkpoint_output,
            state.clone(),
            metrics.clone(),
        );
        tasks.spawn(monitored_future!(aggregator.run()));

        let last_signature_index = epoch_store
            .get_last_checkpoint_signature_index()
            .expect("should not cross end of epoch");
        let last_signature_index = Mutex::new(last_signature_index);

        let service = Arc::new(Self {
            tables: checkpoint_store,
            notify_builder,
            notify_aggregator,
            last_signature_index,
            metrics,
        });

        (service, tasks)
    }

    #[cfg(test)]
    fn write_and_notify_checkpoint_for_testing(
        &self,
        epoch_store: &AuthorityPerEpochStore,
        checkpoint: PendingDWalletCheckpoint,
    ) -> IkaResult {
        use crate::authority::authority_per_epoch_store::ConsensusCommitOutput;

        let mut output = ConsensusCommitOutput::new(0);
        epoch_store.write_pending_checkpoint(&mut output, &checkpoint)?;
        let mut batch = epoch_store.db_batch_for_test();
        output.write_to_batch(epoch_store, &mut batch)?;
        batch.write()?;
        self.notify_checkpoint()?;
        Ok(())
    }
}

impl DWalletCheckpointServiceNotify for DWalletCheckpointService {
    fn notify_checkpoint_signature(
        &self,
        epoch_store: &AuthorityPerEpochStore,
        info: &DWalletCheckpointSignatureMessage,
    ) -> IkaResult {
        let sequence = info.dwallet_checkpoint_message.sequence_number;
        let signer = info
            .dwallet_checkpoint_message
            .auth_sig()
            .authority
            .concise();

        if let Some(highest_verified_checkpoint) = self
            .tables
            .get_highest_verified_dwallet_checkpoint()?
            .map(|x| *x.sequence_number())
        {
            if sequence <= highest_verified_checkpoint {
                debug!(
                    checkpoint_seq = sequence,
                    "Ignore checkpoint signature from {} - already certified", signer,
                );
                self.metrics
                    .last_ignored_dwallet_checkpoint_signature_received
                    .set(sequence as i64);
                return Ok(());
            }
        }
        debug!(
            checkpoint_seq = sequence,
            "Received checkpoint signature, digest {} from {}",
            info.dwallet_checkpoint_message.digest(),
            signer,
        );
        self.metrics
            .last_received_dwallet_checkpoint_signatures
            .with_label_values(&[&signer.to_string()])
            .set(sequence as i64);
        // While it can be tempting to make last_signature_index into AtomicU64, this won't work
        // We need to make sure we write to `pending_signatures` and trigger `notify_aggregator` without race conditions
        let mut index = self.last_signature_index.lock();
        *index += 1;
        epoch_store.insert_checkpoint_signature(sequence, *index, info)?;
        self.notify_aggregator.notify_one();
        Ok(())
    }

    fn notify_checkpoint(&self) -> IkaResult {
        self.notify_builder.notify_one();
        Ok(())
    }
}
