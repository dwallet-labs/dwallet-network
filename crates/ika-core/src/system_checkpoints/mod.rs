// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

mod metrics;
mod system_checkpoint_output;

use crate::authority::AuthorityState;
use crate::stake_aggregator::{InsertResult, MultiStakeAggregator};
pub use crate::system_checkpoints::metrics::SystemCheckpointMetrics;
use crate::system_checkpoints::system_checkpoint_output::{
    CertifiedSystemCheckpointOutput, SystemCheckpointOutput,
};
pub use crate::system_checkpoints::system_checkpoint_output::{
    LogSystemCheckpointOutput, SendSystemCheckpointToStateSync, SubmitSystemCheckpointToConsensus,
};
use diffy::create_patch;
use ika_types::sui::epoch_start_system::EpochStartSystemTrait;
use itertools::Itertools;
use mysten_metrics::{monitored_future, monitored_scope};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use sui_types::base_types::ConciseableName;

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_handler::SequencedConsensusTransactionKey;
use chrono::Utc;
use ika_protocol_config::ProtocolVersion;
use ika_types::committee::StakeUnit;
use ika_types::crypto::AuthorityStrongQuorumSignInfo;
use ika_types::digests::{MessageDigest, SystemCheckpointContentsDigest};
use ika_types::error::{IkaError, IkaResult};
use ika_types::message_envelope::Message;
use ika_types::messages_consensus::ConsensusTransactionKey;
use ika_types::messages_system_checkpoints::SystemCheckpointKind;
use ika_types::messages_system_checkpoints::{
    CertifiedSystemCheckpoint, SignedSystemCheckpoint, SystemCheckpoint, SystemCheckpointDigest,
    SystemCheckpointSequenceNumber, SystemCheckpointSignatureMessage, SystemCheckpointTimestamp,
    TrustedSystemCheckpoint, VerifiedSystemCheckpoint,
};
use ika_types::sui::{SystemInner, SystemInnerTrait};
use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io::Write;
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

pub type SystemCheckpointHeight = u64;

pub struct EpochStats {
    pub system_checkpoint_count: u64,
    pub transaction_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingSystemCheckpointInfo {
    pub timestamp_ms: SystemCheckpointTimestamp,
    pub system_checkpoint_height: SystemCheckpointHeight,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PendingSystemCheckpoint {
    // This is an enum for future updatability, though at the moment there is only one variant.
    V1(PendingSystemCheckpointV1),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingSystemCheckpointV1 {
    pub messages: Vec<SystemCheckpointKind>,
    pub details: PendingSystemCheckpointInfo,
}

impl PendingSystemCheckpoint {
    pub fn as_v1(&self) -> &PendingSystemCheckpointV1 {
        match self {
            PendingSystemCheckpoint::V1(contents) => contents,
        }
    }

    pub fn into_v1(self) -> PendingSystemCheckpointV1 {
        match self {
            PendingSystemCheckpoint::V1(contents) => contents,
        }
    }

    pub fn messages(&self) -> &Vec<SystemCheckpointKind> {
        &self.as_v1().messages
    }

    pub fn details(&self) -> &PendingSystemCheckpointInfo {
        &self.as_v1().details
    }

    pub fn height(&self) -> SystemCheckpointHeight {
        self.details().system_checkpoint_height
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuilderSystemCheckpoint {
    pub system_checkpoint: SystemCheckpoint,
    // Height at which this system_checkpoint message was built. None for genesis system_checkpoint
    pub system_checkpoint_height: Option<SystemCheckpointHeight>,
    pub position_in_commit: usize,
}

#[derive(DBMapUtils)]
pub struct SystemCheckpointStore {
    // /// Maps system_checkpoint contents digest to system_checkpoint contents
    // pub(crate) system_checkpoint_content: DBMap<SystemCheckpointContentsDigest, SystemCheckpointContents>,
    /// Maps system_checkpoint system_checkpoint message digest to system_checkpoint sequence number
    pub(crate) system_checkpoint_sequence_by_digest:
        DBMap<SystemCheckpointDigest, SystemCheckpointSequenceNumber>,

    // /// Stores entire system_checkpoint contents from state sync, indexed by sequence number, for
    // /// efficient reads of full system_checkpoints. Entries from this table are deleted after state
    // /// accumulation has completed.
    // full_system_checkpoint_content: DBMap<SystemCheckpointSequenceNumber, FullSystemCheckpointContents>,
    /// Stores certified system_checkpoints
    pub(crate) certified_system_checkpoints:
        DBMap<SystemCheckpointSequenceNumber, TrustedSystemCheckpoint>,
    // /// Map from system_checkpoint digest to certified system_checkpoint
    // pub(crate) system_checkpoint_by_digest: DBMap<SystemCheckpointDigest, TrustedSystemCheckpoint>,
    /// Store locally computed system_checkpoint summaries so that we can detect forks and log useful
    /// information. Can be pruned as soon as we verify that we are in agreement with the latest
    /// certified system_checkpoint.
    pub(crate) locally_computed_system_checkpoints:
        DBMap<SystemCheckpointSequenceNumber, SystemCheckpoint>,

    /// A map from epoch ID to the sequence number of the last system_checkpoint in that epoch.
    epoch_last_system_checkpoint_map: DBMap<EpochId, SystemCheckpointSequenceNumber>,

    /// Watermarks used to determine the highest verified, fully synced, and
    /// fully executed system_checkpoints
    pub(crate) watermarks:
        DBMap<SystemCheckpointWatermark, (SystemCheckpointSequenceNumber, SystemCheckpointDigest)>,
}

impl SystemCheckpointStore {
    pub fn new(path: &Path) -> Arc<Self> {
        Arc::new(Self::open_tables_read_write(
            path.to_path_buf(),
            MetricConf::new("system_checkpoint"),
            None,
            None,
        ))
    }

    pub fn open_readonly(path: &Path) -> SystemCheckpointStoreReadOnly {
        Self::get_read_only_handle(
            path.to_path_buf(),
            None,
            None,
            MetricConf::new("system_checkpoint_readonly"),
        )
    }

    pub fn get_system_checkpoint_by_digest(
        &self,
        digest: &SystemCheckpointDigest,
    ) -> Result<Option<VerifiedSystemCheckpoint>, TypedStoreError> {
        let sequence = self.system_checkpoint_sequence_by_digest.get(digest)?;
        if let Some(sequence) = sequence {
            self.certified_system_checkpoints
                .get(&sequence)
                .map(|maybe_system_checkpoint| maybe_system_checkpoint.map(|c| c.into()))
        } else {
            Ok(None)
        }
    }

    pub fn get_system_checkpoint_by_sequence_number(
        &self,
        sequence_number: SystemCheckpointSequenceNumber,
    ) -> Result<Option<VerifiedSystemCheckpoint>, TypedStoreError> {
        self.certified_system_checkpoints
            .get(&sequence_number)
            .map(|maybe_system_checkpoint| maybe_system_checkpoint.map(|c| c.into()))
    }

    pub fn get_locally_computed_system_checkpoint(
        &self,
        sequence_number: SystemCheckpointSequenceNumber,
    ) -> Result<Option<SystemCheckpoint>, TypedStoreError> {
        self.locally_computed_system_checkpoints
            .get(&sequence_number)
    }

    pub fn get_latest_certified_system_checkpoint(&self) -> Option<VerifiedSystemCheckpoint> {
        self.certified_system_checkpoints
            .unbounded_iter()
            .skip_to_last()
            .next()
            .map(|(_, v)| v.into())
    }

    pub fn get_latest_locally_computed_system_checkpoint(&self) -> Option<SystemCheckpoint> {
        self.locally_computed_system_checkpoints
            .unbounded_iter()
            .skip_to_last()
            .next()
            .map(|(_, v)| v)
    }

    pub fn multi_get_system_checkpoint_by_sequence_number(
        &self,
        sequence_numbers: &[SystemCheckpointSequenceNumber],
    ) -> Result<Vec<Option<VerifiedSystemCheckpoint>>, TypedStoreError> {
        let system_checkpoints = self
            .certified_system_checkpoints
            .multi_get(sequence_numbers)?
            .into_iter()
            .map(|maybe_system_checkpoint| maybe_system_checkpoint.map(|c| c.into()))
            .collect();

        Ok(system_checkpoints)
    }

    pub fn get_highest_verified_system_checkpoint(
        &self,
    ) -> Result<Option<VerifiedSystemCheckpoint>, TypedStoreError> {
        let highest_verified = if let Some(highest_verified) = self
            .watermarks
            .get(&SystemCheckpointWatermark::HighestVerified)?
        {
            highest_verified
        } else {
            return Ok(None);
        };
        self.get_system_checkpoint_by_sequence_number(highest_verified.0)
    }

    pub fn get_highest_synced_system_checkpoint(
        &self,
    ) -> Result<Option<VerifiedSystemCheckpoint>, TypedStoreError> {
        let highest_synced = if let Some(highest_synced) = self
            .watermarks
            .get(&SystemCheckpointWatermark::HighestSynced)?
        {
            highest_synced
        } else {
            return Ok(None);
        };
        self.get_system_checkpoint_by_sequence_number(highest_synced.0)
    }

    pub fn get_highest_executed_system_checkpoint_seq_number(
        &self,
    ) -> Result<Option<SystemCheckpointSequenceNumber>, TypedStoreError> {
        if let Some(highest_executed) = self
            .watermarks
            .get(&SystemCheckpointWatermark::HighestExecuted)?
        {
            Ok(Some(highest_executed.0))
        } else {
            Ok(None)
        }
    }

    pub fn get_highest_executed_system_checkpoint(
        &self,
    ) -> Result<Option<VerifiedSystemCheckpoint>, TypedStoreError> {
        let highest_executed = if let Some(highest_executed) = self
            .watermarks
            .get(&SystemCheckpointWatermark::HighestExecuted)?
        {
            highest_executed
        } else {
            return Ok(None);
        };
        self.get_system_checkpoint_by_sequence_number(highest_executed.0)
    }

    pub fn get_highest_pruned_system_checkpoint_seq_number(
        &self,
    ) -> Result<SystemCheckpointSequenceNumber, TypedStoreError> {
        Ok(self
            .watermarks
            .get(&SystemCheckpointWatermark::HighestPruned)?
            .unwrap_or_default()
            .0)
    }

    // Called by consensus (ConsensusAggregator).
    // Different from `insert_verified_system_checkpoint`, it does not touch
    // the highest_verified_system_checkpoint watermark such that state sync
    // will have a chance to process this system_checkpoint and perform some
    // state-sync only things.
    pub fn insert_certified_system_checkpoint(
        &self,
        system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<(), TypedStoreError> {
        debug!(
            system_checkpoint_seq = system_checkpoint.sequence_number(),
            "Inserting certified system_checkpoint",
        );
        let mut batch = self.certified_system_checkpoints.batch();
        batch.insert_batch(
            &self.system_checkpoint_sequence_by_digest,
            [(
                system_checkpoint.digest().clone(),
                system_checkpoint.sequence_number(),
            )],
        )?;
        batch.insert_batch(
            &self.certified_system_checkpoints,
            [(
                system_checkpoint.sequence_number(),
                system_checkpoint.serializable_ref(),
            )],
        )?;
        batch.write()?;

        Ok(())
    }

    // Called by state sync, apart from inserting the system_checkpoint and updating
    // related tables, it also bumps the highest_verified_system_checkpoint watermark.
    #[instrument(level = "debug", skip_all)]
    pub fn insert_verified_system_checkpoint(
        &self,
        system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<(), TypedStoreError> {
        self.insert_certified_system_checkpoint(system_checkpoint)?;
        self.update_highest_verified_system_checkpoint(system_checkpoint)
    }

    pub fn update_highest_verified_system_checkpoint(
        &self,
        system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<(), TypedStoreError> {
        if Some(*system_checkpoint.sequence_number())
            > self
                .get_highest_verified_system_checkpoint()?
                .map(|x| *x.sequence_number())
        {
            debug!(
                system_checkpoint_seq = system_checkpoint.sequence_number(),
                "Updating highest verified system_checkpoint",
            );
            self.watermarks.insert(
                &SystemCheckpointWatermark::HighestVerified,
                &(
                    *system_checkpoint.sequence_number(),
                    *system_checkpoint.digest(),
                ),
            )?;
        }

        Ok(())
    }

    pub fn update_highest_synced_system_checkpoint(
        &self,
        system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<(), TypedStoreError> {
        debug!(
            system_checkpoint_seq = system_checkpoint.sequence_number(),
            "Updating highest synced system_checkpoint",
        );
        self.watermarks.insert(
            &SystemCheckpointWatermark::HighestSynced,
            &(
                *system_checkpoint.sequence_number(),
                *system_checkpoint.digest(),
            ),
        )
    }

    pub fn delete_highest_executed_system_checkpoint_test_only(
        &self,
    ) -> Result<(), TypedStoreError> {
        let mut wb = self.watermarks.batch();
        wb.delete_batch(
            &self.watermarks,
            std::iter::once(SystemCheckpointWatermark::HighestExecuted),
        )?;
        wb.write()?;
        Ok(())
    }

    pub fn reset_db_for_execution_since_genesis(&self) -> IkaResult {
        self.delete_highest_executed_system_checkpoint_test_only()?;
        self.watermarks.rocksdb.flush()?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum SystemCheckpointWatermark {
    HighestVerified,
    HighestSynced,
    HighestExecuted,
    HighestPruned,
}

pub struct SystemCheckpointBuilder {
    state: Arc<AuthorityState>,
    tables: Arc<SystemCheckpointStore>,
    epoch_store: Arc<AuthorityPerEpochStore>,
    notify: Arc<Notify>,
    notify_aggregator: Arc<Notify>,
    output: Box<dyn SystemCheckpointOutput>,
    metrics: Arc<SystemCheckpointMetrics>,
    max_messages_per_system_checkpoint: usize,
    max_system_checkpoint_size_bytes: usize,
    previous_epoch_last_system_checkpoint_sequence_number: u64,
}

pub struct SystemCheckpointAggregator {
    tables: Arc<SystemCheckpointStore>,
    epoch_store: Arc<AuthorityPerEpochStore>,
    notify: Arc<Notify>,
    current: Option<SystemCheckpointSignatureAggregator>,
    output: Box<dyn CertifiedSystemCheckpointOutput>,
    state: Arc<AuthorityState>,
    metrics: Arc<SystemCheckpointMetrics>,
}

// This holds information to aggregate signatures for one system_checkpoint
pub struct SystemCheckpointSignatureAggregator {
    next_index: u64,
    system_checkpoint: SystemCheckpoint,
    digest: SystemCheckpointDigest,
    /// Aggregates voting stake for each signed system_checkpoint proposal by authority
    signatures_by_digest: MultiStakeAggregator<SystemCheckpointDigest, SystemCheckpoint, true>,
    tables: Arc<SystemCheckpointStore>,
    state: Arc<AuthorityState>,
    metrics: Arc<SystemCheckpointMetrics>,
}

impl SystemCheckpointBuilder {
    fn new(
        state: Arc<AuthorityState>,
        tables: Arc<SystemCheckpointStore>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        notify: Arc<Notify>,
        output: Box<dyn SystemCheckpointOutput>,
        notify_aggregator: Arc<Notify>,
        metrics: Arc<SystemCheckpointMetrics>,
        max_messages_per_system_checkpoint: usize,
        max_system_checkpoint_size_bytes: usize,
        previous_epoch_last_system_checkpoint_sequence_number: u64,
    ) -> Self {
        Self {
            state,
            tables,
            epoch_store,
            notify,
            output,
            notify_aggregator,
            metrics,
            max_messages_per_system_checkpoint,
            max_system_checkpoint_size_bytes,
            previous_epoch_last_system_checkpoint_sequence_number,
        }
    }

    // overkill
    async fn run(mut self) {
        info!("Starting SystemCheckpointBuilder");
        loop {
            self.maybe_build_system_checkpoints().await;

            self.notify.notified().await;
        }
    }

    async fn maybe_build_system_checkpoints(&mut self) {
        let _scope = monitored_scope("BuildSystemCheckpoints");

        // Collect info about the most recently built system_checkpoint.
        let system_checkpoint = self
            .epoch_store
            .last_built_system_checkpoint_message_builder()
            .expect("epoch should not have ended");
        let mut last_height = system_checkpoint
            .clone()
            .and_then(|s| s.system_checkpoint_height);
        let mut last_timestamp = system_checkpoint.map(|s| s.system_checkpoint.timestamp_ms);

        let min_system_checkpoint_interval_ms = self
            .epoch_store
            .protocol_config()
            .min_system_checkpoint_interval_ms_as_option()
            .unwrap_or_default();
        let mut grouped_pending_system_checkpoints = Vec::new();
        let mut system_checkpoints_iter = self
            .epoch_store
            .get_pending_system_checkpoints(last_height)
            .expect("unexpected epoch store error")
            .into_iter()
            .peekable();
        while let Some((height, pending)) = system_checkpoints_iter.next() {
            // Group PendingSystemCheckpoints until:
            // - minimum interval has elapsed ...
            let current_timestamp = pending.details().timestamp_ms;
            let can_build = match last_timestamp {
                Some(last_timestamp) => {
                    current_timestamp >= last_timestamp + min_system_checkpoint_interval_ms
                }
                None => true,
            };
            grouped_pending_system_checkpoints.push(pending);
            if !can_build {
                debug!(
                    system_checkpoint_commit_height = height,
                    ?last_timestamp,
                    ?current_timestamp,
                    "waiting for more PendingSystemCheckpoints: minimum interval not yet elapsed"
                );
                continue;
            }

            // Min interval has elapsed, we can now coalesce and build a system_checkpoint.
            last_height = Some(height);
            last_timestamp = Some(current_timestamp);
            debug!(
                system_checkpoint_commit_height = height,
                "Making system_checkpoint at commit height"
            );
            if let Err(e) = self
                .make_system_checkpoints(std::mem::take(&mut grouped_pending_system_checkpoints))
                .await
            {
                error!(
                    "Error while making system_checkpoint, will retry in 1s: {:?}",
                    e
                );
                tokio::time::sleep(Duration::from_secs(1)).await;
                self.metrics.system_checkpoint_errors.inc();
                return;
            }
        }
        debug!(
            "Waiting for more system_checkpoints from consensus after processing {last_height:?}; {} pending system_checkpoints left unprocessed until next interval",
            grouped_pending_system_checkpoints.len(),
        );
    }

    #[instrument(level = "debug", skip_all, fields(last_height = pendings.last().unwrap().details().system_checkpoint_height))]
    async fn make_system_checkpoints(
        &self,
        pendings: Vec<PendingSystemCheckpoint>,
    ) -> anyhow::Result<()> {
        let last_details = pendings.last().unwrap().details().clone();

        // Keeps track of the effects that are already included in the current system_checkpoint.
        // This is used when there are multiple pending system_checkpoints to create a single system_checkpoint
        // because in such scenarios, dependencies of a transaction may in earlier created system_checkpoints,
        // or in earlier pending system_checkpoints.
        //let mut effects_in_current_system_checkpoint = BTreeSet::new();

        // Stores the transactions that should be included in the system_checkpoint. Transactions will be recorded in the system_checkpoint
        // in this order.
        let mut sorted_tx_effects_included_in_system_checkpoint = Vec::new();
        for pending_system_checkpoint in pendings.into_iter() {
            let pending = pending_system_checkpoint.into_v1();
            // let txn_in_system_checkpoint = self
            //     .resolve_system_checkpoint_transactions(pending.roots, &mut effects_in_current_system_checkpoint)
            //     .await?;
            sorted_tx_effects_included_in_system_checkpoint.extend(pending.messages);
        }
        let new_system_checkpoint = self
            .create_system_checkpoints(
                sorted_tx_effects_included_in_system_checkpoint,
                &last_details,
            )
            .await?;
        self.write_system_checkpoints(last_details.system_checkpoint_height, new_system_checkpoint)
            .await?;
        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    async fn write_system_checkpoints(
        &self,
        height: SystemCheckpointHeight,
        new_system_checkpoints: Vec<SystemCheckpoint>,
    ) -> IkaResult {
        let _scope = monitored_scope("SystemCheckpointBuilder::write_system_checkpoints");
        //let mut batch = self.tables.system_checkpoint_content.batch();
        // let mut all_tx_digests =
        //     Vec::with_capacity(new_system_checkpoints.iter().map(|(_, c)| c.size()).sum());

        for system_checkpoint in &new_system_checkpoints {
            debug!(
                system_checkpoint_commit_height = height,
                system_checkpoint_seq = system_checkpoint.sequence_number,
                system_checkpoint_digest = ?system_checkpoint.digest(),
                "writing system_checkpoint",
            );
            //all_tx_digests.extend(contents.iter().map(|digest| digest));

            self.output
                .system_checkpoint_created(system_checkpoint, &self.epoch_store, &self.tables)
                .await?;

            self.metrics
                .messages_included_in_system_checkpoint
                .inc_by(system_checkpoint.messages.len() as u64);
            let sequence_number = system_checkpoint.sequence_number;
            self.metrics
                .last_constructed_system_checkpoint
                .set(sequence_number as i64);

            // batch.insert_batch(
            //     &self.tables.system_checkpoint_content,
            //     [(contents.digest(), contents)],
            // )?;

            self.tables
                .locally_computed_system_checkpoints
                .insert(&sequence_number, system_checkpoint)?;

            // batch.insert_batch(
            //     &self.tables.locally_computed_system_checkpoints,
            //     [(sequence_number, summary)],
            // )?;
        }

        self.notify_aggregator.notify_one();
        self.epoch_store
            .process_pending_system_checkpoint(height, new_system_checkpoints)?;
        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn split_system_checkpoint_chunks(
        &self,
        messages: Vec<SystemCheckpointKind>,
    ) -> anyhow::Result<Vec<Vec<SystemCheckpointKind>>> {
        let _guard = monitored_scope("SystemCheckpointBuilder::split_system_checkpoint_chunks");
        let mut chunks = Vec::new();
        let mut chunk = Vec::new();
        let mut chunk_size: usize = 0;
        for message in messages {
            // Roll over to a new chunk after either max count or max size is reached.
            // The size calculation here is intended to estimate the size of the
            // FullSystemCheckpointContents struct. If this code is modified, that struct
            // should also be updated accordingly.
            let size = bcs::serialized_size(&message)?;
            if chunk.len() == self.max_messages_per_system_checkpoint
                || (chunk_size + size) > self.max_system_checkpoint_size_bytes
            {
                if chunk.is_empty() {
                    // Always allow at least one tx in a system_checkpoint.
                    warn!("Size of single transaction ({size}) exceeds max system_checkpoint size ({}); allowing excessively large system_checkpoint to go through.", self.max_system_checkpoint_size_bytes);
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
            // Note: empty system_checkpoints are ok - they shouldn't happen at all on a network with even
            // modest load.
        }
        Ok(chunks)
    }

    #[instrument(level = "debug", skip_all)]
    async fn create_system_checkpoints(
        &self,
        all_messages: Vec<SystemCheckpointKind>,
        details: &PendingSystemCheckpointInfo,
    ) -> anyhow::Result<Vec<SystemCheckpoint>> {
        let _scope = monitored_scope("SystemCheckpointBuilder::create_system_checkpoints");
        let epoch = self.epoch_store.epoch();
        let total = all_messages.len();
        let mut last_system_checkpoint = self.epoch_store.last_built_system_checkpoint_message()?;
        // if last_system_checkpoint.is_none() {
        //     let epoch = self.epoch_store.epoch();
        //     if epoch > 0 {
        //         let previous_epoch = epoch - 1;
        //         let last_verified = self.tables.get_epoch_last_system_checkpoint(previous_epoch)?;
        //         last_system_checkpoint = last_verified.map(VerifiedSystemCheckpoint::into_summary_and_sequence);
        //         if let Some((ref seq, _)) = last_system_checkpoint {
        //             debug!("No system_checkpoints in builder DB, taking system_checkpoint from previous epoch with sequence {seq}");
        //         } else {
        //             // This is some serious bug with when SystemCheckpointBuilder started so surfacing it via panic
        //             panic!("Can not find last system_checkpoint for previous epoch {previous_epoch}");
        //         }
        //     }
        // }
        let mut last_system_checkpoint_seq = last_system_checkpoint.as_ref().map(|(seq, _)| *seq);
        // Epoch 0 is where we create the validator set (we are not running Epoch 0).
        // Once we initialize, the active committee starts in Epoch 1.
        // So there is no previous committee in epoch 1.
        if epoch != 1 && last_system_checkpoint_seq.is_none() {
            last_system_checkpoint_seq =
                Some(self.previous_epoch_last_system_checkpoint_sequence_number);
        }
        info!(
            next_system_checkpoint_seq = last_system_checkpoint_seq.map(|s| s + 1).unwrap_or(0),
            system_checkpoint_timestamp = details.timestamp_ms,
            "Creating system_checkpoint(s) for {} messages",
            all_messages.len(),
        );

        if (all_messages.len() != 0) {
            info!(
                "SystemCheckpointBuilder::create_system_checkpoints: {} messages to be included in system_checkpoint",
                all_messages.len()
            );

            println!("{:?}", all_messages);
        }
        let chunks = self.split_system_checkpoint_chunks(all_messages)?;
        let chunks_count = chunks.len();

        let mut system_checkpoints = Vec::with_capacity(chunks_count);
        debug!(
            ?last_system_checkpoint_seq,
            "Creating {} system_checkpoints with {} transactions", chunks_count, total,
        );

        for (index, mut messages) in chunks.into_iter().enumerate() {
            let first_system_checkpoint_of_epoch = index == 0
                && (last_system_checkpoint_seq.is_none()
                    || last_system_checkpoint_seq.unwrap()
                        == self.previous_epoch_last_system_checkpoint_sequence_number);
            if first_system_checkpoint_of_epoch {
                self.epoch_store
                    .record_epoch_first_system_checkpoint_creation_time_metric();
            }

            let sequence_number = last_system_checkpoint_seq.map(|s| s + 1).unwrap_or(0);
            last_system_checkpoint_seq = Some(sequence_number);

            let timestamp_ms = details.timestamp_ms;
            if let Some((_, last_system_checkpoint)) = &last_system_checkpoint {
                if last_system_checkpoint.timestamp_ms > timestamp_ms {
                    error!("Unexpected decrease of system_checkpoint timestamp, sequence: {}, previous: {}, current: {}",
                    sequence_number,  last_system_checkpoint.timestamp_ms, timestamp_ms);
                }
            }

            info!(
                "SystemCheckpoint sequence: {}, messages count: {}",
                sequence_number,
                messages.len()
            );

            let system_checkpoint =
                SystemCheckpoint::new(epoch, sequence_number, messages, timestamp_ms);
            system_checkpoint
                .report_system_checkpoint_age(&self.metrics.last_created_system_checkpoint_age);
            last_system_checkpoint = Some((sequence_number, system_checkpoint.clone()));
            system_checkpoints.push(system_checkpoint);
        }

        Ok(system_checkpoints)
    }

    // This function is used to check the invariants of the consensus commit prologue transactions in the system_checkpoint
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
            .prepend_prologue_tx_in_consensus_commit_in_system_checkpoints()
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

        // Get all the transactions in the system_checkpoint.
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
            // consensus commit prologue transaction in the system_checkpoint.
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
            // If there is one consensus commit prologue, it must be the first one in the system_checkpoint.
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

impl SystemCheckpointAggregator {
    fn new(
        tables: Arc<SystemCheckpointStore>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        notify: Arc<Notify>,
        output: Box<dyn CertifiedSystemCheckpointOutput>,
        state: Arc<AuthorityState>,
        metrics: Arc<SystemCheckpointMetrics>,
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
        info!("Starting SystemCheckpointAggregator");
        loop {
            if let Err(e) = self.run_and_notify().await {
                error!(
                    "Error while aggregating system_checkpoint, will retry in 1s: {:?}",
                    e
                );
                self.metrics.system_checkpoint_errors.inc();
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }

            let _ = timeout(Duration::from_secs(1), self.notify.notified()).await;
        }
    }

    async fn run_and_notify(&mut self) -> IkaResult {
        let system_checkpoints = self.run_inner()?;
        for system_checkpoint in system_checkpoints {
            self.output
                .certified_system_checkpoint_created(&system_checkpoint)
                .await?;
        }
        Ok(())
    }

    fn run_inner(&mut self) -> IkaResult<Vec<CertifiedSystemCheckpoint>> {
        let _scope = monitored_scope("SystemCheckpointAggregator");
        let mut result = vec![];
        'outer: loop {
            let next_to_certify = self.next_system_checkpoint_to_certify();
            let current = if let Some(current) = &mut self.current {
                // It's possible that the system_checkpoint was already certified by
                // the rest of the network, and we've already received the
                // certified system_checkpoint via StateSync. In this case, we reset
                // the current signature aggregator to the next system_checkpoint to
                // be certified
                if current.system_checkpoint.sequence_number < next_to_certify {
                    self.current = None;
                    continue;
                }
                current
            } else {
                let Some(system_checkpoint) = self
                    .epoch_store
                    .get_built_system_checkpoint_message(next_to_certify)?
                else {
                    return Ok(result);
                };
                self.current = Some(SystemCheckpointSignatureAggregator {
                    next_index: 0,
                    digest: system_checkpoint.digest(),
                    system_checkpoint,
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
            let iter = epoch_tables.get_pending_system_checkpoint_signatures_iter(
                current.system_checkpoint.sequence_number,
                current.next_index,
            )?;
            for ((seq, index), data) in iter {
                if seq != current.system_checkpoint.sequence_number {
                    debug!(
                        system_checkpoint_seq =? current.system_checkpoint.sequence_number,
                        "Not enough system_checkpoint signatures",
                    );
                    // No more signatures (yet) for this system_checkpoint
                    return Ok(result);
                }
                debug!(
                    system_checkpoint_seq = current.system_checkpoint.sequence_number,
                    "Processing signature for system_checkpoint (digest: {:?}) from {:?}",
                    current.system_checkpoint.digest(),
                    data.system_checkpoint.auth_sig().authority.concise()
                );
                self.metrics
                    .system_checkpoint_participation
                    .with_label_values(&[&format!(
                        "{:?}",
                        data.system_checkpoint.auth_sig().authority.concise()
                    )])
                    .inc();
                if let Ok(auth_signature) = current.try_aggregate(data) {
                    let system_checkpoint = VerifiedSystemCheckpoint::new_unchecked(
                        CertifiedSystemCheckpoint::new_from_data_and_sig(
                            current.system_checkpoint.clone(),
                            auth_signature,
                        ),
                    );

                    self.tables
                        .insert_certified_system_checkpoint(&system_checkpoint)?;
                    self.metrics
                        .last_certified_system_checkpoint
                        .set(current.system_checkpoint.sequence_number as i64);
                    current.system_checkpoint.report_system_checkpoint_age(
                        &self.metrics.last_certified_system_checkpoint_age,
                    );
                    result.push(system_checkpoint.into_inner());
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

    fn next_system_checkpoint_to_certify(&self) -> SystemCheckpointSequenceNumber {
        self.tables
            .certified_system_checkpoints
            .unbounded_iter()
            .skip_to_last()
            .next()
            .map(|(seq, _)| seq + 1)
            .unwrap_or_default()
    }
}

impl SystemCheckpointSignatureAggregator {
    #[allow(clippy::result_unit_err)]
    pub fn try_aggregate(
        &mut self,
        data: SystemCheckpointSignatureMessage,
    ) -> Result<AuthorityStrongQuorumSignInfo, ()> {
        let their_digest = *data.system_checkpoint.digest();
        let (_, signature) = data.system_checkpoint.into_data_and_sig();
        let author = signature.authority;
        let envelope = SignedSystemCheckpoint::new_from_data_and_sig(
            self.system_checkpoint.clone(),
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
                    system_checkpoint_seq = self.system_checkpoint.sequence_number,
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
                    self.metrics.remote_system_checkpoint_forks.inc();
                    warn!(
                        system_checkpoint_seq = self.system_checkpoint.sequence_number,
                        "Validator {:?} has mismatching system_checkpoint digest {}, we have digest {}",
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
}

pub trait SystemCheckpointServiceNotify {
    fn notify_system_checkpoint_signature(
        &self,
        epoch_store: &AuthorityPerEpochStore,
        info: &SystemCheckpointSignatureMessage,
    ) -> IkaResult;

    fn notify_system_checkpoint(&self) -> IkaResult;
}

/// This is a service used to communicate with other pieces of ika(for ex. authority)
pub struct SystemCheckpointService {
    tables: Arc<SystemCheckpointStore>,
    notify_builder: Arc<Notify>,
    notify_aggregator: Arc<Notify>,
    last_signature_index: Mutex<u64>,
    metrics: Arc<SystemCheckpointMetrics>,
}

impl SystemCheckpointService {
    pub fn spawn(
        state: Arc<AuthorityState>,
        system_checkpoint_store: Arc<SystemCheckpointStore>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        system_checkpoint_output: Box<dyn SystemCheckpointOutput>,
        certified_system_checkpoint_output: Box<dyn CertifiedSystemCheckpointOutput>,
        metrics: Arc<SystemCheckpointMetrics>,
        max_messages_per_system_checkpoint: usize,
        max_system_checkpoint_size_bytes: usize,
        previous_epoch_last_system_checkpoint_sequence_number: u64,
    ) -> (Arc<Self>, JoinSet<()> /* Handle to tasks */) {
        info!(
            "Starting system_checkpoint service with {max_messages_per_system_checkpoint} max_messages_per_system_checkpoint and {max_system_checkpoint_size_bytes} max_system_checkpoint_size_bytes"
        );
        let notify_builder = Arc::new(Notify::new());
        let notify_aggregator = Arc::new(Notify::new());

        let mut tasks = JoinSet::new();

        let builder = SystemCheckpointBuilder::new(
            state.clone(),
            system_checkpoint_store.clone(),
            epoch_store.clone(),
            notify_builder.clone(),
            system_checkpoint_output,
            notify_aggregator.clone(),
            metrics.clone(),
            max_messages_per_system_checkpoint,
            max_system_checkpoint_size_bytes,
            previous_epoch_last_system_checkpoint_sequence_number,
        );
        tasks.spawn(monitored_future!(builder.run()));

        let aggregator = SystemCheckpointAggregator::new(
            system_checkpoint_store.clone(),
            epoch_store.clone(),
            notify_aggregator.clone(),
            certified_system_checkpoint_output,
            state.clone(),
            metrics.clone(),
        );
        tasks.spawn(monitored_future!(aggregator.run()));

        let last_signature_index = epoch_store
            .get_last_system_checkpoint_signature_index()
            .expect("should not cross end of epoch");
        let last_signature_index = Mutex::new(last_signature_index);

        let service = Arc::new(Self {
            tables: system_checkpoint_store,
            notify_builder,
            notify_aggregator,
            last_signature_index,
            metrics,
        });

        (service, tasks)
    }

    #[cfg(test)]
    fn write_and_notify_system_checkpoint_for_testing(
        &self,
        epoch_store: &AuthorityPerEpochStore,
        system_checkpoint: PendingSystemCheckpoint,
    ) -> IkaResult {
        use crate::authority::authority_per_epoch_store::ConsensusCommitOutput;

        let mut output = ConsensusCommitOutput::new(0);
        epoch_store.write_pending_system_checkpoint(&mut output, &system_checkpoint)?;
        let mut batch = epoch_store.db_batch_for_test();
        output.write_to_batch(epoch_store, &mut batch)?;
        batch.write()?;
        self.notify_system_checkpoint()?;
        Ok(())
    }
}

impl SystemCheckpointServiceNotify for SystemCheckpointService {
    fn notify_system_checkpoint_signature(
        &self,
        epoch_store: &AuthorityPerEpochStore,
        info: &SystemCheckpointSignatureMessage,
    ) -> IkaResult {
        let sequence = info.system_checkpoint.sequence_number;
        let signer = info.system_checkpoint.auth_sig().authority.concise();

        if let Some(highest_verified_system_checkpoint) = self
            .tables
            .get_highest_verified_system_checkpoint()?
            .map(|x| *x.sequence_number())
        {
            if sequence <= highest_verified_system_checkpoint {
                debug!(
                    system_checkpoint_seq = sequence,
                    "Ignore system_checkpoint signature from {} - already certified", signer,
                );
                self.metrics
                    .last_ignored_system_checkpoint_signature_received
                    .set(sequence as i64);
                return Ok(());
            }
        }
        debug!(
            system_checkpoint_seq = sequence,
            "Received system_checkpoint signature, digest {} from {}",
            info.system_checkpoint.digest(),
            signer,
        );
        self.metrics
            .last_received_system_checkpoint_signatures
            .with_label_values(&[&signer.to_string()])
            .set(sequence as i64);
        // While it can be tempting to make last_signature_index into AtomicU64, this won't work
        // We need to make sure we write to `pending_signatures` and trigger `notify_aggregator` without race conditions
        let mut index = self.last_signature_index.lock();
        *index += 1;
        epoch_store.insert_system_checkpoint_signature(sequence, *index, info)?;
        self.notify_aggregator.notify_one();
        Ok(())
    }

    fn notify_system_checkpoint(&self) -> IkaResult {
        self.notify_builder.notify_one();
        Ok(())
    }
}
