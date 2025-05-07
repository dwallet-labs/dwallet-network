// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

mod params_message_output;
mod metrics;

use crate::authority::AuthorityState;
use crate::params_messages::params_message_output::{CertifiedParamsMessageOutput, ParamsMessageOutput};
pub use crate::params_messages::params_message_output::{
    LogParamsMessageOutput, SendParamsMessageToStateSync, SubmitParamsMessageToConsensus,
};
pub use crate::params_messages::metrics::ParamsMessageMetrics;
use crate::stake_aggregator::{InsertResult, MultiStakeAggregator};
use diffy::create_patch;
use ika_types::sui::epoch_start_system::EpochStartSystemTrait;
use itertools::Itertools;
use mysten_metrics::{monitored_future, monitored_scope};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use sui_macros::fail_point;
use sui_types::base_types::ConciseableName;

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_handler::SequencedConsensusTransactionKey;
use chrono::Utc;
use ika_protocol_config::ProtocolVersion;
use ika_types::committee::StakeUnit;
use ika_types::crypto::AuthorityStrongQuorumSignInfo;
use ika_types::digests::{ParamsMessageContentsDigest, MessageDigest};
use ika_types::error::{IkaError, IkaResult};
use ika_types::message_envelope::Message;
use ika_types::messages_params_messages::{CertifiedParamsMessage, ParamsMessage, ParamsMessageDigest, ParamsMessageSequenceNumber, ParamsMessageSignatureMessage, ParamsMessageTimestamp, TrustedParamsMessage, VerifiedParamsMessage};
use ika_types::messages_consensus::ConsensusTransactionKey;
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
use ika_types::messages_params_messages::{ParamsMessageKind};

pub type ParamsMessageHeight = u64;

pub struct EpochStats {
    pub params_message_count: u64,
    pub transaction_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingParamsMessageInfo {
    pub timestamp_ms: ParamsMessageTimestamp,
    pub params_message_height: ParamsMessageHeight,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PendingParamsMessage {
    // This is an enum for future updatability, though at the moment there is only one variant.
    V1(PendingParamsMessageV1),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingParamsMessageV1 {
    pub messages: Vec<ParamsMessageKind>,
    pub details: PendingParamsMessageInfo,
}

impl PendingParamsMessage {
    pub fn as_v1(&self) -> &PendingParamsMessageV1 {
        match self {
            PendingParamsMessage::V1(contents) => contents,
        }
    }

    pub fn into_v1(self) -> PendingParamsMessageV1 {
        match self {
            PendingParamsMessage::V1(contents) => contents,
        }
    }

    pub fn messages(&self) -> &Vec<ParamsMessageKind> {
        &self.as_v1().messages
    }

    pub fn details(&self) -> &PendingParamsMessageInfo {
        &self.as_v1().details
    }

    pub fn height(&self) -> ParamsMessageHeight {
        self.details().params_message_height
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuilderParamsMessage {
    pub params_message: ParamsMessage,
    // Height at which this params_message message was built. None for genesis params_message
    pub params_message_height: Option<ParamsMessageHeight>,
    pub position_in_commit: usize,
}

#[derive(DBMapUtils)]
pub struct ParamsMessageStore {
    // /// Maps params_message contents digest to params_message contents
    // pub(crate) params_message_content: DBMap<ParamsMessageContentsDigest, ParamsMessageContents>,
    /// Maps params_message params_message message digest to params_message sequence number
    pub(crate) params_message_sequence_by_digest:
        DBMap<ParamsMessageDigest, ParamsMessageSequenceNumber>,

    // /// Stores entire params_message contents from state sync, indexed by sequence number, for
    // /// efficient reads of full params_messages. Entries from this table are deleted after state
    // /// accumulation has completed.
    // full_params_message_content: DBMap<ParamsMessageSequenceNumber, FullParamsMessageContents>,
    /// Stores certified params_messages
    pub(crate) certified_params_messages: DBMap<ParamsMessageSequenceNumber, TrustedParamsMessage>,
    // /// Map from params_message digest to certified params_message
    // pub(crate) params_message_by_digest: DBMap<ParamsMessageDigest, TrustedParamsMessage>,
    /// Store locally computed params_message summaries so that we can detect forks and log useful
    /// information. Can be pruned as soon as we verify that we are in agreement with the latest
    /// certified params_message.
    pub(crate) locally_computed_params_messages: DBMap<ParamsMessageSequenceNumber, ParamsMessage>,

    /// A map from epoch ID to the sequence number of the last params_message in that epoch.
    epoch_last_params_message_map: DBMap<EpochId, ParamsMessageSequenceNumber>,

    /// Watermarks used to determine the highest verified, fully synced, and
    /// fully executed params_messages
    pub(crate) watermarks:
        DBMap<ParamsMessageWatermark, (ParamsMessageSequenceNumber, ParamsMessageDigest)>,
}

impl ParamsMessageStore {
    pub fn new(path: &Path) -> Arc<Self> {
        Arc::new(Self::open_tables_read_write(
            path.to_path_buf(),
            MetricConf::new("params_message"),
            None,
            None,
        ))
    }

    pub fn open_readonly(path: &Path) -> ParamsMessageStoreReadOnly {
        Self::get_read_only_handle(
            path.to_path_buf(),
            None,
            None,
            MetricConf::new("params_message_readonly"),
        )
    }

    pub fn get_params_message_by_digest(
        &self,
        digest: &ParamsMessageDigest,
    ) -> Result<Option<VerifiedParamsMessage>, TypedStoreError> {
        let sequence = self.params_message_sequence_by_digest.get(digest)?;
        if let Some(sequence) = sequence {
            self.certified_params_messages
                .get(&sequence)
                .map(|maybe_params_message| maybe_params_message.map(|c| c.into()))
        } else {
            Ok(None)
        }
    }

    pub fn get_params_message_by_sequence_number(
        &self,
        sequence_number: ParamsMessageSequenceNumber,
    ) -> Result<Option<VerifiedParamsMessage>, TypedStoreError> {
        self.certified_params_messages
            .get(&sequence_number)
            .map(|maybe_params_message| maybe_params_message.map(|c| c.into()))
    }

    pub fn get_locally_computed_params_message(
        &self,
        sequence_number: ParamsMessageSequenceNumber,
    ) -> Result<Option<ParamsMessage>, TypedStoreError> {
        self.locally_computed_params_messages.get(&sequence_number)
    }

    pub fn get_latest_certified_params_message(&self) -> Option<VerifiedParamsMessage> {
        self.certified_params_messages
            .unbounded_iter()
            .skip_to_last()
            .next()
            .map(|(_, v)| v.into())
    }

    pub fn get_latest_locally_computed_params_message(&self) -> Option<ParamsMessage> {
        self.locally_computed_params_messages
            .unbounded_iter()
            .skip_to_last()
            .next()
            .map(|(_, v)| v)
    }

    pub fn multi_get_params_message_by_sequence_number(
        &self,
        sequence_numbers: &[ParamsMessageSequenceNumber],
    ) -> Result<Vec<Option<VerifiedParamsMessage>>, TypedStoreError> {
        let params_messages = self
            .certified_params_messages
            .multi_get(sequence_numbers)?
            .into_iter()
            .map(|maybe_params_message| maybe_params_message.map(|c| c.into()))
            .collect();

        Ok(params_messages)
    }

    pub fn get_highest_verified_params_message(
        &self,
    ) -> Result<Option<VerifiedParamsMessage>, TypedStoreError> {
        let highest_verified = if let Some(highest_verified) =
            self.watermarks.get(&ParamsMessageWatermark::HighestVerified)?
        {
            highest_verified
        } else {
            return Ok(None);
        };
        self.get_params_message_by_digest(&highest_verified.1)
    }

    pub fn get_highest_synced_params_message(
        &self,
    ) -> Result<Option<VerifiedParamsMessage>, TypedStoreError> {
        let highest_synced = if let Some(highest_synced) =
            self.watermarks.get(&ParamsMessageWatermark::HighestSynced)?
        {
            highest_synced
        } else {
            return Ok(None);
        };
        self.get_params_message_by_digest(&highest_synced.1)
    }

    pub fn get_highest_executed_params_message_seq_number(
        &self,
    ) -> Result<Option<ParamsMessageSequenceNumber>, TypedStoreError> {
        if let Some(highest_executed) =
            self.watermarks.get(&ParamsMessageWatermark::HighestExecuted)?
        {
            Ok(Some(highest_executed.0))
        } else {
            Ok(None)
        }
    }

    pub fn get_highest_executed_params_message(
        &self,
    ) -> Result<Option<VerifiedParamsMessage>, TypedStoreError> {
        let highest_executed = if let Some(highest_executed) =
            self.watermarks.get(&ParamsMessageWatermark::HighestExecuted)?
        {
            highest_executed
        } else {
            return Ok(None);
        };
        self.get_params_message_by_digest(&highest_executed.1)
    }

    pub fn get_highest_pruned_params_message_seq_number(
        &self,
    ) -> Result<ParamsMessageSequenceNumber, TypedStoreError> {
        Ok(self
            .watermarks
            .get(&ParamsMessageWatermark::HighestPruned)?
            .unwrap_or_default()
            .0)
    }

    // Called by consensus (ConsensusAggregator).
    // Different from `insert_verified_params_message`, it does not touch
    // the highest_verified_params_message watermark such that state sync
    // will have a chance to process this params_message and perform some
    // state-sync only things.
    pub fn insert_certified_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<(), TypedStoreError> {
        debug!(
            params_message_seq = params_message.sequence_number(),
            "Inserting certified params_message",
        );
        let mut batch = self.certified_params_messages.batch();
        batch.insert_batch(
            &self.params_message_sequence_by_digest,
            [(params_message.digest().clone(), params_message.sequence_number())],
        )?;
        batch.insert_batch(
            &self.certified_params_messages,
            [(params_message.sequence_number(), params_message.serializable_ref())],
        )?;
        batch.write()?;

        Ok(())
    }

    // Called by state sync, apart from inserting the params_message and updating
    // related tables, it also bumps the highest_verified_params_message watermark.
    #[instrument(level = "debug", skip_all)]
    pub fn insert_verified_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<(), TypedStoreError> {
        self.insert_certified_params_message(params_message)?;
        self.update_highest_verified_params_message(params_message)
    }

    pub fn update_highest_verified_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<(), TypedStoreError> {
        if Some(*params_message.sequence_number())
            > self
                .get_highest_verified_params_message()?
                .map(|x| *x.sequence_number())
        {
            debug!(
                params_message_seq = params_message.sequence_number(),
                "Updating highest verified params_message",
            );
            self.watermarks.insert(
                &ParamsMessageWatermark::HighestVerified,
                &(*params_message.sequence_number(), *params_message.digest()),
            )?;
        }

        Ok(())
    }

    pub fn update_highest_synced_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<(), TypedStoreError> {
        debug!(
            params_message_seq = params_message.sequence_number(),
            "Updating highest synced params_message",
        );
        self.watermarks.insert(
            &ParamsMessageWatermark::HighestSynced,
            &(*params_message.sequence_number(), *params_message.digest()),
        )
    }

    pub fn delete_highest_executed_params_message_test_only(&self) -> Result<(), TypedStoreError> {
        let mut wb = self.watermarks.batch();
        wb.delete_batch(
            &self.watermarks,
            std::iter::once(ParamsMessageWatermark::HighestExecuted),
        )?;
        wb.write()?;
        Ok(())
    }

    pub fn reset_db_for_execution_since_genesis(&self) -> IkaResult {
        self.delete_highest_executed_params_message_test_only()?;
        self.watermarks.rocksdb.flush()?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ParamsMessageWatermark {
    HighestVerified,
    HighestSynced,
    HighestExecuted,
    HighestPruned,
}

pub struct ParamsMessageBuilder {
    state: Arc<AuthorityState>,
    tables: Arc<ParamsMessageStore>,
    epoch_store: Arc<AuthorityPerEpochStore>,
    notify: Arc<Notify>,
    notify_aggregator: Arc<Notify>,
    output: Box<dyn ParamsMessageOutput>,
    metrics: Arc<ParamsMessageMetrics>,
    max_messages_per_params_message: usize,
    max_params_message_size_bytes: usize,
    previous_epoch_last_params_message_sequence_number: u64,
}

pub struct ParamsMessageAggregator {
    tables: Arc<ParamsMessageStore>,
    epoch_store: Arc<AuthorityPerEpochStore>,
    notify: Arc<Notify>,
    current: Option<ParamsMessageSignatureAggregator>,
    output: Box<dyn CertifiedParamsMessageOutput>,
    state: Arc<AuthorityState>,
    metrics: Arc<ParamsMessageMetrics>,
}

// This holds information to aggregate signatures for one params_message
pub struct ParamsMessageSignatureAggregator {
    next_index: u64,
    params_message: ParamsMessage,
    digest: ParamsMessageDigest,
    /// Aggregates voting stake for each signed params_message proposal by authority
    signatures_by_digest: MultiStakeAggregator<ParamsMessageDigest, ParamsMessage, true>,
    tables: Arc<ParamsMessageStore>,
    state: Arc<AuthorityState>,
    metrics: Arc<ParamsMessageMetrics>,
}

impl ParamsMessageBuilder {
    fn new(
        state: Arc<AuthorityState>,
        tables: Arc<ParamsMessageStore>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        notify: Arc<Notify>,
        output: Box<dyn ParamsMessageOutput>,
        notify_aggregator: Arc<Notify>,
        metrics: Arc<ParamsMessageMetrics>,
        max_messages_per_params_message: usize,
        max_params_message_size_bytes: usize,
        previous_epoch_last_params_message_sequence_number: u64,
    ) -> Self {
        Self {
            state,
            tables,
            epoch_store,
            notify,
            output,
            notify_aggregator,
            metrics,
            max_messages_per_params_message,
            max_params_message_size_bytes,
            previous_epoch_last_params_message_sequence_number,
        }
    }

    // overkill
    async fn run(mut self) {
        info!("Starting ParamsMessageBuilder");
        loop {
            self.maybe_build_params_messages().await;

            self.notify.notified().await;
        }
    }

    async fn maybe_build_params_messages(&mut self) {
        let _scope = monitored_scope("BuildParamsMessages");

        // Collect info about the most recently built params_message.
        let params_message = self
            .epoch_store
            .last_built_params_message_builder()
            .expect("epoch should not have ended");
        let mut last_height = params_message.clone().and_then(|s| s.params_message_height);
        let mut last_timestamp = params_message.map(|s| s.params_message.timestamp_ms);

        let min_params_message_interval_ms = self
            .epoch_store
            .protocol_config()
            .min_params_message_interval_ms_as_option()
            .unwrap_or_default();
        let mut grouped_pending_params_messages = Vec::new();
        let mut params_messages_iter = self
            .epoch_store
            .get_pending_params_messages(last_height)
            .expect("unexpected epoch store error")
            .into_iter()
            .peekable();
        while let Some((height, pending)) = params_messages_iter.next() {
            // Group PendingParamsMessages until:
            // - minimum interval has elapsed ...
            let current_timestamp = pending.details().timestamp_ms;
            let can_build = match last_timestamp {
                Some(last_timestamp) => {
                    current_timestamp >= last_timestamp + min_params_message_interval_ms
                }
                None => true,
            };
            grouped_pending_params_messages.push(pending);
            if !can_build {
                debug!(
                    params_message_commit_height = height,
                    ?last_timestamp,
                    ?current_timestamp,
                    "waiting for more PendingParamsMessages: minimum interval not yet elapsed"
                );
                continue;
            }

            // Min interval has elapsed, we can now coalesce and build a params_message.
            last_height = Some(height);
            last_timestamp = Some(current_timestamp);
            debug!(
                params_message_commit_height = height,
                "Making params_message at commit height"
            );
            if let Err(e) = self
                .make_params_message(std::mem::take(&mut grouped_pending_params_messages))
                .await
            {
                error!("Error while making params_message, will retry in 1s: {:?}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
                self.metrics.params_message_errors.inc();
                return;
            }
        }
        debug!(
            "Waiting for more params_messages from consensus after processing {last_height:?}; {} pending params_messages left unprocessed until next interval",
            grouped_pending_params_messages.len(),
        );
    }

    #[instrument(level = "debug", skip_all, fields(last_height = pendings.last().unwrap().details().params_message_height))]
    async fn make_params_message(&self, pendings: Vec<PendingParamsMessage>) -> anyhow::Result<()> {
        let last_details = pendings.last().unwrap().details().clone();

        // Keeps track of the effects that are already included in the current params_message.
        // This is used when there are multiple pending params_messages to create a single params_message
        // because in such scenarios, dependencies of a transaction may in earlier created params_messages,
        // or in earlier pending params_messages.
        //let mut effects_in_current_params_message = BTreeSet::new();

        // Stores the transactions that should be included in the params_message. Transactions will be recorded in the params_message
        // in this order.
        let mut sorted_tx_effects_included_in_params_message = Vec::new();
        for pending_params_message in pendings.into_iter() {
            let pending = pending_params_message.into_v1();
            // let txn_in_params_message = self
            //     .resolve_params_message_transactions(pending.roots, &mut effects_in_current_params_message)
            //     .await?;
            sorted_tx_effects_included_in_params_message.extend(pending.messages);
        }
        let new_params_message = self
            .create_params_messages(sorted_tx_effects_included_in_params_message, &last_details)
            .await?;
        self.write_params_messages(last_details.params_message_height, new_params_message)
            .await?;
        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    async fn write_params_messages(
        &self,
        height: ParamsMessageHeight,
        new_params_messages: Vec<ParamsMessage>,
    ) -> IkaResult {
        let _scope = monitored_scope("ParamsMessageBuilder::write_params_messages");
        //let mut batch = self.tables.params_message_content.batch();
        // let mut all_tx_digests =
        //     Vec::with_capacity(new_params_messages.iter().map(|(_, c)| c.size()).sum());

        for params_message in &new_params_messages {
            debug!(
                params_message_commit_height = height,
                params_message_seq = params_message.sequence_number,
                params_message_digest = ?params_message.digest(),
                "writing params_message",
            );
            //all_tx_digests.extend(contents.iter().map(|digest| digest));

            self.output
                .params_message_created(params_message, &self.epoch_store, &self.tables)
                .await?;

            self.metrics
                .messages_included_in_params_message
                .inc_by(params_message.messages.len() as u64);
            let sequence_number = params_message.sequence_number;
            self.metrics
                .last_constructed_params_message
                .set(sequence_number as i64);

            // batch.insert_batch(
            //     &self.tables.params_message_content,
            //     [(contents.digest(), contents)],
            // )?;

            self.tables
                .locally_computed_params_messages
                .insert(&sequence_number, params_message)?;

            // batch.insert_batch(
            //     &self.tables.locally_computed_params_messages,
            //     [(sequence_number, summary)],
            // )?;
        }

        self.notify_aggregator.notify_one();
        self.epoch_store
            .process_pending_params_message(height, new_params_messages)?;
        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn split_params_message_chunks(
        &self,
        messages: Vec<ParamsMessageKind>,
    ) -> anyhow::Result<Vec<Vec<ParamsMessageKind>>> {
        let _guard = monitored_scope("ParamsMessageBuilder::split_params_message_chunks");
        let mut chunks = Vec::new();
        let mut chunk = Vec::new();
        let mut chunk_size: usize = 0;
        for message in messages {
            // Roll over to a new chunk after either max count or max size is reached.
            // The size calculation here is intended to estimate the size of the
            // FullParamsMessageContents struct. If this code is modified, that struct
            // should also be updated accordingly.
            let size = bcs::serialized_size(&message)?;
            if chunk.len() == self.max_messages_per_params_message
                || (chunk_size + size) > self.max_params_message_size_bytes
            {
                if chunk.is_empty() {
                    // Always allow at least one tx in a params_message.
                    warn!("Size of single transaction ({size}) exceeds max params_message size ({}); allowing excessively large params_message to go through.", self.max_params_message_size_bytes);
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
            // Note: empty params_messages are ok - they shouldn't happen at all on a network with even
            // modest load.
        }
        Ok(chunks)
    }

    #[instrument(level = "debug", skip_all)]
    async fn create_params_messages(
        &self,
        all_messages: Vec<ParamsMessageKind>,
        details: &PendingParamsMessageInfo,
    ) -> anyhow::Result<Vec<ParamsMessage>> {
        let _scope = monitored_scope("ParamsMessageBuilder::create_params_messages");
        let epoch = self.epoch_store.epoch();
        let total = all_messages.len();
        let mut last_params_message = self.epoch_store.last_built_params_message()?;
        // if last_params_message.is_none() {
        //     let epoch = self.epoch_store.epoch();
        //     if epoch > 0 {
        //         let previous_epoch = epoch - 1;
        //         let last_verified = self.tables.get_epoch_last_params_message(previous_epoch)?;
        //         last_params_message = last_verified.map(VerifiedParamsMessage::into_summary_and_sequence);
        //         if let Some((ref seq, _)) = last_params_message {
        //             debug!("No params_messages in builder DB, taking params_message from previous epoch with sequence {seq}");
        //         } else {
        //             // This is some serious bug with when ParamsMessageBuilder started so surfacing it via panic
        //             panic!("Can not find last params_message for previous epoch {previous_epoch}");
        //         }
        //     }
        // }
        let mut last_params_message_seq = last_params_message.as_ref().map(|(seq, _)| *seq);
        // Epoch 0 is where we create the validator set (we are not running Epoch 0).
        // Once we initialize, the active committee starts in Epoch 1.
        // So there is no previous committee in epoch 1.
        if epoch != 1 && last_params_message_seq.is_none() {
            last_params_message_seq = Some(self.previous_epoch_last_params_message_sequence_number);
        }
        info!(
            next_params_message_seq = last_params_message_seq.map(|s| s + 1).unwrap_or(0),
            params_message_timestamp = details.timestamp_ms,
            "Creating params_message(s) for {} messages",
            all_messages.len(),
        );

        let chunks = self.split_params_message_chunks(all_messages)?;
        let chunks_count = chunks.len();

        let mut params_messages = Vec::with_capacity(chunks_count);
        debug!(
            ?last_params_message_seq,
            "Creating {} params_messages with {} transactions", chunks_count, total,
        );

        for (index, mut messages) in chunks.into_iter().enumerate() {
            let first_params_message_of_epoch = index == 0
                && (last_params_message_seq.is_none()
                    || last_params_message_seq.unwrap()
                        == self.previous_epoch_last_params_message_sequence_number);
            if first_params_message_of_epoch {
                self.epoch_store
                    .record_epoch_first_params_message_creation_time_metric();
            }

            let sequence_number = last_params_message_seq.map(|s| s + 1).unwrap_or(0);
            last_params_message_seq = Some(sequence_number);

            let timestamp_ms = details.timestamp_ms;
            if let Some((_, last_params_message)) = &last_params_message {
                if last_params_message.timestamp_ms > timestamp_ms {
                    error!("Unexpected decrease of params_message timestamp, sequence: {}, previous: {}, current: {}",
                    sequence_number,  last_params_message.timestamp_ms, timestamp_ms);
                }
            }

            info!(
                "ParamsMessage sequence: {}, messages count: {}",
                sequence_number,
                messages.len()
            );

            let params_message =
                ParamsMessage::new(epoch, sequence_number, messages, timestamp_ms);
            params_message.report_params_message_age(&self.metrics.last_created_params_message_age);
            last_params_message = Some((sequence_number, params_message.clone()));
            params_messages.push(params_message);
        }

        Ok(params_messages)
    }

    // This function is used to check the invariants of the consensus commit prologue transactions in the params_message
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
            .prepend_prologue_tx_in_consensus_commit_in_params_messages()
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

        // Get all the transactions in the params_message.
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
            // consensus commit prologue transaction in the params_message.
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
            // If there is one consensus commit prologue, it must be the first one in the params_message.
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

impl ParamsMessageAggregator {
    fn new(
        tables: Arc<ParamsMessageStore>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        notify: Arc<Notify>,
        output: Box<dyn CertifiedParamsMessageOutput>,
        state: Arc<AuthorityState>,
        metrics: Arc<ParamsMessageMetrics>,
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
        info!("Starting ParamsMessageAggregator");
        loop {
            if let Err(e) = self.run_and_notify().await {
                error!(
                    "Error while aggregating params_message, will retry in 1s: {:?}",
                    e
                );
                self.metrics.params_message_errors.inc();
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }

            let _ = timeout(Duration::from_secs(1), self.notify.notified()).await;
        }
    }

    async fn run_and_notify(&mut self) -> IkaResult {
        let params_messages = self.run_inner()?;
        for params_message in params_messages {
            self.output
                .certified_params_message_created(&params_message)
                .await?;
        }
        Ok(())
    }

    fn run_inner(&mut self) -> IkaResult<Vec<CertifiedParamsMessage>> {
        let _scope = monitored_scope("ParamsMessageAggregator");
        let mut result = vec![];
        'outer: loop {
            let next_to_certify = self.next_params_message_to_certify();
            let current = if let Some(current) = &mut self.current {
                // It's possible that the params_message was already certified by
                // the rest of the network, and we've already received the
                // certified params_message via StateSync. In this case, we reset
                // the current signature aggregator to the next params_message to
                // be certified
                if current.params_message.sequence_number < next_to_certify {
                    self.current = None;
                    continue;
                }
                current
            } else {
                let Some(params_message) = self
                    .epoch_store
                    .get_built_params_message(next_to_certify)?
                else {
                    return Ok(result);
                };
                self.current = Some(ParamsMessageSignatureAggregator {
                    next_index: 0,
                    digest: params_message.digest(),
                    params_message,
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
            let iter = epoch_tables.get_pending_params_message_signatures_iter(
                current.params_message.sequence_number,
                current.next_index,
            )?;
            for ((seq, index), data) in iter {
                if seq != current.params_message.sequence_number {
                    debug!(
                        params_message_seq =? current.params_message.sequence_number,
                        "Not enough params_message signatures",
                    );
                    // No more signatures (yet) for this params_message
                    return Ok(result);
                }
                debug!(
                    params_message_seq = current.params_message.sequence_number,
                    "Processing signature for params_message (digest: {:?}) from {:?}",
                    current.params_message.digest(),
                    data.params_message.auth_sig().authority.concise()
                );
                self.metrics
                    .params_message_participation
                    .with_label_values(&[&format!(
                        "{:?}",
                        data.params_message.auth_sig().authority.concise()
                    )])
                    .inc();
                if let Ok(auth_signature) = current.try_aggregate(data) {
                    let params_message = VerifiedParamsMessage::new_unchecked(
                        CertifiedParamsMessage::new_from_data_and_sig(
                            current.params_message.clone(),
                            auth_signature,
                        ),
                    );

                    self.tables
                        .insert_certified_params_message(&params_message)?;
                    self.metrics
                        .last_certified_params_message
                        .set(current.params_message.sequence_number as i64);
                    current
                        .params_message
                        .report_params_message_age(&self.metrics.last_certified_params_message_age);
                    result.push(params_message.into_inner());
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

    fn next_params_message_to_certify(&self) -> ParamsMessageSequenceNumber {
        self.tables
            .certified_params_messages
            .unbounded_iter()
            .skip_to_last()
            .next()
            .map(|(seq, _)| seq + 1)
            .unwrap_or_default()
    }
}

impl ParamsMessageSignatureAggregator {
    #[allow(clippy::result_unit_err)]
    pub fn try_aggregate(
        &mut self,
        data: ParamsMessageSignatureMessage,
    ) -> Result<AuthorityStrongQuorumSignInfo, ()> {
        let their_digest = *data.params_message.digest();
        let (_, signature) = data.params_message.into_data_and_sig();
        let author = signature.authority;
        let envelope = SignedParamsMessage::new_from_data_and_sig(
            self.params_message.clone(),
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
                    params_message_seq = self.params_message.sequence_number,
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
                    self.metrics.remote_params_message_forks.inc();
                    warn!(
                        params_message_seq = self.params_message.sequence_number,
                        "Validator {:?} has mismatching params_message digest {}, we have digest {}",
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

pub trait ParamsMessageServiceNotify {
    fn notify_params_message_signature(
        &self,
        epoch_store: &AuthorityPerEpochStore,
        info: &ParamsMessageSignatureMessage,
    ) -> IkaResult;

    fn notify_params_message(&self) -> IkaResult;
}

/// This is a service used to communicate with other pieces of ika(for ex. authority)
pub struct ParamsMessageService {
    tables: Arc<ParamsMessageStore>,
    notify_builder: Arc<Notify>,
    notify_aggregator: Arc<Notify>,
    last_signature_index: Mutex<u64>,
    metrics: Arc<ParamsMessageMetrics>,
}

impl ParamsMessageService {
    pub fn spawn(
        state: Arc<AuthorityState>,
        params_message_store: Arc<ParamsMessageStore>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        params_message_output: Box<dyn ParamsMessageOutput>,
        certified_params_message_output: Box<dyn CertifiedParamsMessageOutput>,
        metrics: Arc<ParamsMessageMetrics>,
        max_messages_per_params_message: usize,
        max_params_message_size_bytes: usize,
        previous_epoch_last_params_message_sequence_number: u64,
    ) -> (Arc<Self>, JoinSet<()> /* Handle to tasks */) {
        info!(
            "Starting params_message service with {max_messages_per_params_message} max_messages_per_params_message and {max_params_message_size_bytes} max_params_message_size_bytes"
        );
        let notify_builder = Arc::new(Notify::new());
        let notify_aggregator = Arc::new(Notify::new());

        let mut tasks = JoinSet::new();

        let builder = ParamsMessageBuilder::new(
            state.clone(),
            params_message_store.clone(),
            epoch_store.clone(),
            notify_builder.clone(),
            params_message_output,
            notify_aggregator.clone(),
            metrics.clone(),
            max_messages_per_params_message,
            max_params_message_size_bytes,
            previous_epoch_last_params_message_sequence_number,
        );
        tasks.spawn(monitored_future!(builder.run()));

        let aggregator = ParamsMessageAggregator::new(
            params_message_store.clone(),
            epoch_store.clone(),
            notify_aggregator.clone(),
            certified_params_message_output,
            state.clone(),
            metrics.clone(),
        );
        tasks.spawn(monitored_future!(aggregator.run()));

        let last_signature_index = epoch_store
            .get_last_params_message_signature_index()
            .expect("should not cross end of epoch");
        let last_signature_index = Mutex::new(last_signature_index);

        let service = Arc::new(Self {
            tables: params_message_store,
            notify_builder,
            notify_aggregator,
            last_signature_index,
            metrics,
        });

        (service, tasks)
    }

    #[cfg(test)]
    fn write_and_notify_params_message_for_testing(
        &self,
        epoch_store: &AuthorityPerEpochStore,
        params_message: PendingParamsMessage,
    ) -> IkaResult {
        use crate::authority::authority_per_epoch_store::ConsensusCommitOutput;

        let mut output = ConsensusCommitOutput::new(0);
        epoch_store.write_pending_params_message(&mut output, &params_message)?;
        let mut batch = epoch_store.db_batch_for_test();
        output.write_to_batch(epoch_store, &mut batch)?;
        batch.write()?;
        self.notify_params_message()?;
        Ok(())
    }
}

impl ParamsMessageServiceNotify for ParamsMessageService {
    fn notify_params_message_signature(
        &self,
        epoch_store: &AuthorityPerEpochStore,
        info: &ParamsMessageSignatureMessage,
    ) -> IkaResult {
        let sequence = info.params_message.sequence_number;
        let signer = info.params_message.auth_sig().authority.concise();

        if let Some(highest_verified_params_message) = self
            .tables
            .get_highest_verified_params_message()?
            .map(|x| *x.sequence_number())
        {
            if sequence <= highest_verified_params_message {
                debug!(
                    params_message_seq = sequence,
                    "Ignore params_message signature from {} - already certified", signer,
                );
                self.metrics
                    .last_ignored_params_message_signature_received
                    .set(sequence as i64);
                return Ok(());
            }
        }
        debug!(
            params_message_seq = sequence,
            "Received params_message signature, digest {} from {}",
            info.params_message.digest(),
            signer,
        );
        self.metrics
            .last_received_params_message_signatures
            .with_label_values(&[&signer.to_string()])
            .set(sequence as i64);
        // While it can be tempting to make last_signature_index into AtomicU64, this won't work
        // We need to make sure we write to `pending_signatures` and trigger `notify_aggregator` without race conditions
        let mut index = self.last_signature_index.lock();
        *index += 1;
        epoch_store.insert_params_message_signature(sequence, *index, info)?;
        self.notify_aggregator.notify_one();
        Ok(())
    }

    fn notify_params_message(&self) -> IkaResult {
        self.notify_builder.notify_one();
        Ok(())
    }
}
