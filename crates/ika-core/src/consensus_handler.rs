// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    num::NonZeroUsize,
    sync::Arc,
};

use arc_swap::ArcSwap;
use consensus_config::Committee as ConsensusCommittee;
use consensus_core::{CommitConsumerMonitor, TransactionIndex, VerifiedBlock};
use ika_protocol_config::ProtocolConfig;
use ika_types::crypto::AuthorityName;
use ika_types::digests::{ConsensusCommitDigest, MessageDigest};
use ika_types::message::MessageKind;
use ika_types::messages_consensus::{
    AuthorityIndex, ConsensusTransaction, ConsensusTransactionKey, ConsensusTransactionKind,
};
use ika_types::sui::epoch_start_system::EpochStartSystemTrait;
use lru::LruCache;
use mpc::WeightedThresholdAccessStructure;
use mysten_metrics::{
    monitored_future,
    monitored_mpsc::{self, UnboundedReceiver},
    monitored_scope, spawn_monitored_task,
};
use serde::{Deserialize, Serialize};
use sui_macros::{fail_point_async, fail_point_if};
use sui_types::{
    authenticator_state::ActiveJwk,
    base_types::{EpochId, ObjectID, SequenceNumber, TransactionDigest},
    executable_transaction::{TrustedExecutableTransaction, VerifiedExecutableTransaction},
    transaction::{SenderSignedData, VerifiedTransaction},
};

use crate::dwallet_mpc::mpc_manager::DWalletMPCDBMessage;
use crate::dwallet_mpc::mpc_outputs_verifier::OutputVerificationStatus;
use crate::{
    authority::{
        authority_per_epoch_store::{
            AuthorityPerEpochStore, ConsensusStats, ConsensusStatsAPI, ExecutionIndices,
            ExecutionIndicesWithStats,
        },
        epoch_start_configuration::EpochStartConfigTrait,
        AuthorityMetrics, AuthorityState,
    },
    checkpoints::{CheckpointService, CheckpointServiceNotify},
    consensus_throughput_calculator::ConsensusThroughputCalculator,
    consensus_types::consensus_output_api::{parse_block_transactions, ConsensusCommitAPI},
    scoring_decision::update_low_scoring_authorities,
};
use ika_types::dwallet_mpc_error::DwalletMPCResult;
use ika_types::error::IkaResult;
use ika_types::messages_dwallet_mpc::{DWalletMPCEvent, DWalletMPCOutputMessage};
use tokio::task::JoinSet;
use tracing::{debug, error, info, instrument, trace_span, warn};
use typed_store::Map;

pub struct ConsensusHandlerInitializer {
    state: Arc<AuthorityState>,
    checkpoint_service: Arc<CheckpointService>,
    epoch_store: Arc<AuthorityPerEpochStore>,
    low_scoring_authorities: Arc<ArcSwap<HashMap<AuthorityName, u64>>>,
    throughput_calculator: Arc<ConsensusThroughputCalculator>,
}

impl ConsensusHandlerInitializer {
    pub fn new(
        state: Arc<AuthorityState>,
        checkpoint_service: Arc<CheckpointService>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        low_scoring_authorities: Arc<ArcSwap<HashMap<AuthorityName, u64>>>,
        throughput_calculator: Arc<ConsensusThroughputCalculator>,
    ) -> Self {
        Self {
            state,
            checkpoint_service,
            epoch_store,
            low_scoring_authorities,
            throughput_calculator,
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_testing(
        state: Arc<AuthorityState>,
        checkpoint_service: Arc<CheckpointService>,
    ) -> Self {
        Self {
            state: state.clone(),
            checkpoint_service,
            epoch_store: state.epoch_store_for_testing().clone(),
            low_scoring_authorities: Arc::new(Default::default()),
            throughput_calculator: Arc::new(ConsensusThroughputCalculator::new(
                None,
                state.metrics.clone(),
            )),
        }
    }

    pub(crate) fn new_consensus_handler(&self) -> ConsensusHandler<CheckpointService> {
        let new_epoch_start_state = self.epoch_store.epoch_start_state();
        let consensus_committee = new_epoch_start_state.get_consensus_committee();

        ConsensusHandler::new(
            self.epoch_store.clone(),
            self.checkpoint_service.clone(),
            self.low_scoring_authorities.clone(),
            consensus_committee,
            self.state.metrics.clone(),
            self.throughput_calculator.clone(),
        )
    }

    pub(crate) fn metrics(&self) -> &Arc<AuthorityMetrics> {
        &self.state.metrics
    }
}

pub struct ConsensusHandler<C> {
    /// A store created for each epoch. ConsensusHandler is recreated each epoch, with the
    /// corresponding store. This store is also used to get the current epoch ID.
    epoch_store: Arc<AuthorityPerEpochStore>,
    /// Holds the indices, hash and stats after the last consensus commit
    /// It is used for avoiding replaying already processed transactions,
    /// checking chain consistency, and accumulating per-epoch consensus output stats.
    last_consensus_stats: ExecutionIndicesWithStats,
    checkpoint_service: Arc<C>,
    /// Reputation scores used by consensus adapter that we update, forwarded from consensus
    low_scoring_authorities: Arc<ArcSwap<HashMap<AuthorityName, u64>>>,
    /// The consensus committee used to do stake computations for deciding set of low scoring authorities
    committee: ConsensusCommittee,
    // TODO: ConsensusHandler doesn't really share metrics with AuthorityState. We could define
    // a new metrics type here if we want to.
    metrics: Arc<AuthorityMetrics>,
    /// Lru cache to quickly discard transactions processed by consensus
    processed_cache: LruCache<SequencedConsensusTransactionKey, ()>,
    /// Using the throughput calculator to record the current consensus throughput
    throughput_calculator: Arc<ConsensusThroughputCalculator>,
}

const PROCESSED_CACHE_CAP: usize = 1024 * 1024;

impl<C> ConsensusHandler<C> {
    pub fn new(
        epoch_store: Arc<AuthorityPerEpochStore>,
        checkpoint_service: Arc<C>,
        low_scoring_authorities: Arc<ArcSwap<HashMap<AuthorityName, u64>>>,
        committee: ConsensusCommittee,
        metrics: Arc<AuthorityMetrics>,
        throughput_calculator: Arc<ConsensusThroughputCalculator>,
    ) -> Self {
        // Recover last_consensus_stats so it is consistent across validators.
        let mut last_consensus_stats = epoch_store
            .get_last_consensus_stats()
            .expect("Should be able to read last consensus index");
        // stats is empty at the beginning of epoch.
        if !last_consensus_stats.stats.is_initialized() {
            last_consensus_stats.stats = ConsensusStats::new(committee.size());
        }

        Self {
            epoch_store,
            last_consensus_stats,
            checkpoint_service,
            low_scoring_authorities,
            committee,
            metrics,
            processed_cache: LruCache::new(NonZeroUsize::new(PROCESSED_CACHE_CAP).unwrap()),
            throughput_calculator,
        }
    }

    /// Returns the last subdag index processed by the handler.
    pub(crate) fn last_processed_subdag_index(&self) -> u64 {
        self.last_consensus_stats.index.sub_dag_index
    }
}

impl<C: CheckpointServiceNotify + Send + Sync> ConsensusHandler<C> {
    #[instrument(level = "debug", skip_all)]
    async fn handle_consensus_commit(&mut self, consensus_commit: impl ConsensusCommitAPI) {
        let _scope = monitored_scope("ConsensusCommitHandler::handle_consensus_commit");

        let last_committed_round = self.last_consensus_stats.index.sub_dag_index;

        if self.should_perform_dwallet_mpc_state_sync().await {
            if let Err(err) = self.perform_dwallet_mpc_state_sync().await {
                error!(
                    "epoch switched while performing dwallet mpc state sync: {:?}",
                    err
                );
                return;
            }
        }
        let mut dwallet_mpc_verifier = self.epoch_store.get_dwallet_mpc_outputs_verifier().await;
        dwallet_mpc_verifier.last_processed_consensus_round = last_committed_round;
        // Need to drop the verifier, as `self` is being used mutably later in this function.
        drop(dwallet_mpc_verifier);

        let last_committed_round = self.last_consensus_stats.index.last_committed_round;
        let round = consensus_commit.leader_round();

        // TODO: Remove this once narwhal is deprecated. For now mysticeti will not return
        // more than one leader per round so we are not in danger of ignoring any commits.
        assert!(round >= last_committed_round);
        if last_committed_round == round {
            // we can receive the same commit twice after restart
            // It is critical that the writes done by this function are atomic - otherwise we can
            // lose the later parts of a commit if we restart midway through processing it.
            warn!(
                "Ignoring consensus output for round {} as it is already committed. NOTE: This is only expected if consensus is running.",
                round
            );
            return;
        }

        /* (transaction, serialized length) */
        let mut transactions = vec![];
        let timestamp = consensus_commit.commit_timestamp_ms();
        let leader_author = consensus_commit.leader_author_index();
        let commit_sub_dag_index = consensus_commit.commit_sub_dag_index();

        let epoch_start = self
            .epoch_store
            .epoch_start_config()
            .epoch_start_timestamp_ms();
        let timestamp = if timestamp < epoch_start {
            error!(
                "Unexpected commit timestamp {timestamp} less then epoch start time {epoch_start}, author {leader_author}, round {round}",
            );
            epoch_start
        } else {
            timestamp
        };

        info!(
            %consensus_commit,
            epoch = ?self.epoch_store.epoch(),
            "Received consensus output"
        );

        let execution_index = ExecutionIndices {
            last_committed_round: round,
            sub_dag_index: commit_sub_dag_index,
            transaction_index: 0_u64,
        };
        // This function has filtered out any already processed consensus output.
        // So we can safely assume that the index is always increasing.
        assert!(self.last_consensus_stats.index < execution_index);

        // TODO: test empty commit explicitly.
        // Note that consensus commit batch may contain no transactions, but we still need to record the current
        // round and subdag index in the last_consensus_stats, so that it won't be re-executed in the future.
        self.last_consensus_stats.index = execution_index;

        update_low_scoring_authorities(
            self.low_scoring_authorities.clone(),
            self.epoch_store.committee(),
            &self.committee,
            consensus_commit.reputation_score_sorted_desc(),
            &self.metrics,
            self.epoch_store
                .protocol_config()
                .consensus_bad_nodes_stake_threshold(),
        );

        self.metrics
            .consensus_committed_subdags
            .with_label_values(&[&leader_author.to_string()])
            .inc();

        {
            let span = trace_span!("ConsensusHandler::HandleCommit::process_consensus_txns");
            let _guard = span.enter();
            for (authority_index, parsed_transactions) in consensus_commit.transactions() {
                // TODO: consider only messages within 1~3 rounds of the leader?
                self.last_consensus_stats
                    .stats
                    .inc_num_messages(authority_index as usize);
                for parsed in parsed_transactions {
                    // Skip executing rejected transactions. Unlocking is the responsibility of the
                    // consensus transaction handler.
                    if parsed.rejected {
                        continue;
                    }
                    let kind = classify(&parsed.transaction);
                    self.metrics
                        .consensus_handler_processed
                        .with_label_values(&[kind])
                        .inc();
                    self.metrics
                        .consensus_handler_transaction_sizes
                        .with_label_values(&[kind])
                        .observe(parsed.serialized_len as f64);
                    let transaction =
                        SequencedConsensusTransactionKind::External(parsed.transaction);
                    transactions.push((transaction, authority_index));
                }
            }
        }
        info!(?transactions, "Parsed transactions");

        for (i, authority) in self.committee.authorities() {
            let hostname = &authority.hostname;
            self.metrics
                .consensus_committed_messages
                .with_label_values(&[hostname])
                .set(self.last_consensus_stats.stats.get_num_messages(i.value()) as i64);
            self.metrics
                .consensus_committed_user_transactions
                .with_label_values(&[hostname])
                .set(
                    self.last_consensus_stats
                        .stats
                        .get_num_user_transactions(i.value()) as i64,
                );
        }

        let mut all_transactions = Vec::new();
        {
            // We need a set here as well, since the processed_cache is a LRU cache and can drop
            // entries while we're iterating over the sequenced transactions.
            let mut processed_set = HashSet::new();

            for (seq, (transaction, cert_origin)) in transactions.into_iter().enumerate() {
                // In process_consensus_transactions_and_commit_boundary(), we will add a system consensus commit
                // prologue transaction, which will be the first transaction in this consensus commit batch.
                // Therefore, the transaction sequence number starts from 1 here.
                let current_tx_index = ExecutionIndices {
                    last_committed_round: round,
                    sub_dag_index: commit_sub_dag_index,
                    transaction_index: (seq + 1) as u64,
                };

                self.last_consensus_stats.index = current_tx_index;

                let certificate_author = *self
                    .epoch_store
                    .committee()
                    .authority_by_index(cert_origin)
                    .unwrap();

                let sequenced_transaction = SequencedConsensusTransaction {
                    certificate_author_index: cert_origin,
                    certificate_author,
                    consensus_index: current_tx_index,
                    transaction,
                };

                let key = sequenced_transaction.key();
                let in_set = !processed_set.insert(key);
                let in_cache = self
                    .processed_cache
                    .put(sequenced_transaction.key(), ())
                    .is_some();

                if in_set || in_cache {
                    self.metrics.skipped_consensus_txns_cache_hit.inc();
                    continue;
                }

                all_transactions.push(sequenced_transaction);
            }
        }

        let executable_transactions = self
            .epoch_store
            .process_consensus_transactions_and_commit_boundary(
                all_transactions,
                &self.last_consensus_stats,
                &self.checkpoint_service,
                &ConsensusCommitInfo::new(self.epoch_store.protocol_config(), &consensus_commit),
                &self.metrics,
            )
            .await
            .expect("Unrecoverable error in consensus handler");

        // update the calculated throughput
        self.throughput_calculator
            .add_transactions(timestamp, executable_transactions.len() as u64);

        fail_point_if!("correlated-crash-after-consensus-commit-boundary", || {
            let key = [commit_sub_dag_index, self.epoch_store.epoch()];
            if ika_simulator::random::deterministic_probability(&key, 0.01) {
                ika_simulator::task::kill_current_node(None);
            }
        });

        fail_point_async!("crash"); // for tests that produce random crashes
                                    //
                                    // self.transaction_manager_sender
                                    //     .send(executable_transactions);
    }

    /// Loads all dWallet MPC messages from the epoch start from the epoch tables.
    /// Needs to be a separate function because the DB table does not implement the `Send` trait,
    /// hence async code involving it can cause compilation errors.
    async fn load_dwallet_mpc_messages_from_epoch_start(
        &self,
    ) -> IkaResult<Vec<DWalletMPCDBMessage>> {
        Ok(self
            .epoch_store
            .tables()?
            .dwallet_mpc_messages
            .unbounded_iter()
            .map(|(_, messages)| messages)
            .flatten()
            .collect())
    }

    /// Loads all dWallet MPC outputs from the epoch start from the epoch tables.
    /// Needs to be a separate function because the DB table does not implement the `Send` trait,
    /// hence async code involving it can cause compilation errors.
    async fn load_dwallet_mpc_outputs_from_epoch_start(
        &self,
    ) -> IkaResult<Vec<DWalletMPCOutputMessage>> {
        Ok(self
            .epoch_store
            .tables()?
            .dwallet_mpc_outputs
            .unbounded_iter()
            .map(|(_, messages)| messages)
            .flatten()
            .collect())
    }

    /// Loads all dWallet MPC events from the epoch start from the epoch tables.
    /// Needed to be a separate function because the DB table does not implement the `Send` trait,
    /// hence async code involving it can cause compilation errors.
    async fn load_dwallet_mpc_events_from_epoch_start(&self) -> IkaResult<Vec<DWalletMPCEvent>> {
        Ok(self
            .epoch_store
            .tables()?
            .dwallet_mpc_events
            .unbounded_iter()
            .map(|(_, messages)| messages)
            .flatten()
            .collect())
    }

    /// Check if the dWallet MPC manager should perform a state sync.
    /// If so, block consensus and load all messages.
    /// This condition is only true if we process a round
    /// before we processed the previous round,
    /// which can only happen if we restart the node.
    async fn should_perform_dwallet_mpc_state_sync(&self) -> bool {
        let mut dwallet_mpc_verifier = self.epoch_store.get_dwallet_mpc_outputs_verifier().await;
        // Check if the dwallet mpc manager should perform a state sync, and if so block consensus and load all messages
        // This condition is only true if we process a round before we processed the previous round,
        // which can only happen if we restart the node.
        self.last_consensus_stats.index.sub_dag_index
            > dwallet_mpc_verifier.last_processed_consensus_round + 1
    }

    /// Syncs the [`DWalletMPCOutputsVerifier`] from the epoch start.
    /// Needs to be performed here,
    /// so system transactions will get created when they should, and a fork in the
    /// chain will be prevented.
    /// Fails only if the epoch switched in the middle of the state sync.
    async fn perform_dwallet_mpc_state_sync(&self) -> IkaResult {
        info!("Performing a state sync for the dWallet MPC node");
        let mut dwallet_mpc_verifier = self.epoch_store.get_dwallet_mpc_outputs_verifier().await;
        for event in self.load_dwallet_mpc_events_from_epoch_start().await? {
            dwallet_mpc_verifier.monitor_new_session_outputs(&event.session_info);
        }
        for output in self.load_dwallet_mpc_outputs_from_epoch_start().await? {
            if let Err(err) = dwallet_mpc_verifier
                .try_verify_output(&output.output, &output.session_info, output.authority)
                .await
            {
                error!(
                    "failed to verify output from session {:?} and party {:?}: {:?}",
                    output.session_info.session_id, output.authority, err
                );
            }
        }
        for message in self.load_dwallet_mpc_messages_from_epoch_start().await? {
            match message {
                DWalletMPCDBMessage::Message(_)
                | DWalletMPCDBMessage::EndOfDelivery
                | DWalletMPCDBMessage::MPCSessionFailed(_)
                | DWalletMPCDBMessage::SessionFailedWithMaliciousParties(..)
                | DWalletMPCDBMessage::PerformCryptographicComputations => {}
            }
        }
        Ok(())
    }
}

/// Manages the lifetime of tasks handling the commits and transactions output by consensus.
pub(crate) struct MysticetiConsensusHandler {
    tasks: JoinSet<()>,
}

impl MysticetiConsensusHandler {
    pub(crate) fn new(
        mut consensus_handler: ConsensusHandler<CheckpointService>,
        mut commit_receiver: UnboundedReceiver<consensus_core::CommittedSubDag>,
        commit_consumer_monitor: Arc<CommitConsumerMonitor>,
    ) -> Self {
        let mut tasks = JoinSet::new();
        tasks.spawn(monitored_future!(async move {
            // TODO: pause when execution is overloaded, so consensus can detect the backpressure.
            while let Some(consensus_commit) = commit_receiver.recv().await {
                let commit_index = consensus_commit.commit_ref.index;
                consensus_handler
                    .handle_consensus_commit(consensus_commit)
                    .await;
                commit_consumer_monitor.set_highest_handled_commit(commit_index);
            }
        }));
        Self { tasks }
    }

    pub(crate) async fn abort(&mut self) {
        self.tasks.shutdown().await;
    }
}

impl<C> ConsensusHandler<C> {
    fn epoch(&self) -> EpochId {
        self.epoch_store.epoch()
    }
}

pub(crate) fn classify(transaction: &ConsensusTransaction) -> &'static str {
    match &transaction.kind {
        ConsensusTransactionKind::CheckpointSignature(_) => "checkpoint_signature",
        ConsensusTransactionKind::DWalletMPCMessage(..) => "dwallet_mpc_message",
        ConsensusTransactionKind::DWalletMPCOutput(..) => "dwallet_mpc_output",
        ConsensusTransactionKind::CapabilityNotificationV1(_) => "capability_notification_v1",
        ConsensusTransactionKind::DWalletMPCSessionFailedWithMalicious(..) => {
            "dwallet_mpc_session_failed_with_malicious"
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencedConsensusTransaction {
    pub certificate_author_index: AuthorityIndex,
    pub certificate_author: AuthorityName,
    pub consensus_index: ExecutionIndices,
    pub transaction: SequencedConsensusTransactionKind,
}

#[derive(Debug, Clone)]
pub enum SequencedConsensusTransactionKind {
    External(ConsensusTransaction),
    System(MessageKind),
}

impl Serialize for SequencedConsensusTransactionKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let serializable = SerializableSequencedConsensusTransactionKind::from(self);
        serializable.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SequencedConsensusTransactionKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let serializable =
            SerializableSequencedConsensusTransactionKind::deserialize(deserializer)?;
        Ok(serializable.into())
    }
}

// We can't serialize SequencedConsensusTransactionKind directly because it contains a
// VerifiedExecutableTransaction, which is not serializable (by design). This wrapper allows us to
// convert to a serializable format easily.
#[derive(Debug, Clone, Serialize, Deserialize)]
enum SerializableSequencedConsensusTransactionKind {
    External(ConsensusTransaction),
    System(MessageKind),
}

impl From<&SequencedConsensusTransactionKind> for SerializableSequencedConsensusTransactionKind {
    fn from(kind: &SequencedConsensusTransactionKind) -> Self {
        match kind {
            SequencedConsensusTransactionKind::External(ext) => {
                SerializableSequencedConsensusTransactionKind::External(ext.clone())
            }
            SequencedConsensusTransactionKind::System(txn) => {
                SerializableSequencedConsensusTransactionKind::System(txn.clone())
            }
        }
    }
}

impl From<SerializableSequencedConsensusTransactionKind> for SequencedConsensusTransactionKind {
    fn from(kind: SerializableSequencedConsensusTransactionKind) -> Self {
        match kind {
            SerializableSequencedConsensusTransactionKind::External(ext) => {
                SequencedConsensusTransactionKind::External(ext)
            }
            SerializableSequencedConsensusTransactionKind::System(txn) => {
                SequencedConsensusTransactionKind::System(txn)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub enum SequencedConsensusTransactionKey {
    External(ConsensusTransactionKey),
    System(MessageDigest),
}

impl SequencedConsensusTransactionKind {
    pub fn key(&self) -> SequencedConsensusTransactionKey {
        match self {
            SequencedConsensusTransactionKind::External(ext) => {
                SequencedConsensusTransactionKey::External(ext.key())
            }
            SequencedConsensusTransactionKind::System(txn) => {
                SequencedConsensusTransactionKey::System(txn.digest())
            }
        }
    }

    pub fn get_tracking_id(&self) -> u64 {
        match self {
            SequencedConsensusTransactionKind::External(ext) => ext.get_tracking_id(),
            SequencedConsensusTransactionKind::System(_txn) => 0,
        }
    }
}

impl SequencedConsensusTransaction {
    pub fn sender_authority(&self) -> AuthorityName {
        self.certificate_author
    }

    pub fn key(&self) -> SequencedConsensusTransactionKey {
        self.transaction.key()
    }

    pub fn is_system(&self) -> bool {
        matches!(
            self.transaction,
            SequencedConsensusTransactionKind::System(_)
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedSequencedConsensusTransaction(pub SequencedConsensusTransaction);

#[cfg(test)]
impl VerifiedSequencedConsensusTransaction {
    pub fn new_test(transaction: ConsensusTransaction) -> Self {
        Self(SequencedConsensusTransaction::new_test(transaction))
    }
}

impl SequencedConsensusTransaction {
    pub fn new_test(transaction: ConsensusTransaction) -> Self {
        Self {
            certificate_author_index: 0,
            certificate_author: AuthorityName::ZERO,
            consensus_index: Default::default(),
            transaction: SequencedConsensusTransactionKind::External(transaction),
        }
    }
}

/// Represents the information from the current consensus commit.
pub struct ConsensusCommitInfo {
    pub round: u64,
    pub timestamp: u64,
    pub consensus_commit_digest: ConsensusCommitDigest,

    #[cfg(any(test, feature = "test-utils"))]
    skip_consensus_commit_prologue_in_test: bool,
}

impl ConsensusCommitInfo {
    fn new(protocol_config: &ProtocolConfig, consensus_commit: &impl ConsensusCommitAPI) -> Self {
        Self {
            round: consensus_commit.leader_round(),
            timestamp: consensus_commit.commit_timestamp_ms(),
            consensus_commit_digest: consensus_commit.consensus_digest(),

            #[cfg(any(test, feature = "test-utils"))]
            skip_consensus_commit_prologue_in_test: false,
        }
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub fn new_for_test(
        commit_round: u64,
        commit_timestamp: u64,
        skip_consensus_commit_prologue_in_test: bool,
    ) -> Self {
        Self {
            round: commit_round,
            timestamp: commit_timestamp,
            consensus_commit_digest: ConsensusCommitDigest::default(),
            skip_consensus_commit_prologue_in_test,
        }
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub fn skip_consensus_commit_prologue_in_test(&self) -> bool {
        self.skip_consensus_commit_prologue_in_test
    }
}
