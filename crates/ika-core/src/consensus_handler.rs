// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    num::NonZeroUsize,
    sync::Arc,
};

use arc_swap::ArcSwap;
use consensus_config::Committee as ConsensusCommittee;
use consensus_core::{CommitConsumerMonitor, TransactionIndex, VerifiedBlock};
use lru::LruCache;
use mysten_metrics::{
    monitored_future,
    monitored_mpsc::{self, UnboundedReceiver},
    monitored_scope, spawn_monitored_task,
};
use serde::{Deserialize, Serialize};
use sui_macros::{fail_point_async, fail_point_if};
use ika_protocol_config::ProtocolConfig;
use sui_types::{
    authenticator_state::ActiveJwk,
    base_types::{EpochId, ObjectID, SequenceNumber, TransactionDigest},
    executable_transaction::{TrustedExecutableTransaction, VerifiedExecutableTransaction},
    transaction::{SenderSignedData, VerifiedTransaction},
};
use ika_types::crypto::{AuthorityName};
use ika_types::action::{ActionData};
use ika_types::messages_consensus::{
    AuthorityIndex, ConsensusTransaction, ConsensusTransactionKey, ConsensusTransactionKind,
};
use ika_types::digests::{ConsensusCommitDigest, ActionDigest};
use ika_types::sui::ika_system_state::epoch_start_ika_system_state::EpochStartSystemStateTrait;

use tokio::task::JoinSet;
use tracing::{debug, error, info, instrument, trace_span, warn};

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
}

/// Manages the lifetime of tasks handling the commits and transactions output by consensus.
pub(crate) struct MysticetiConsensusHandler {
    tasks: JoinSet<()>,
}

impl MysticetiConsensusHandler {
    pub(crate) fn new(
        mut consensus_handler: ConsensusHandler<CheckpointService>,
        consensus_transaction_handler: ConsensusTransactionHandler,
        mut commit_receiver: UnboundedReceiver<consensus_core::CommittedSubDag>,
        mut transaction_receiver: UnboundedReceiver<Vec<(VerifiedBlock, Vec<TransactionIndex>)>>,
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
        if consensus_transaction_handler.enabled() {
            tasks.spawn(monitored_future!(async move {
                while let Some(blocks_and_rejected_transactions) = transaction_receiver.recv().await
                {
                    consensus_transaction_handler
                        .handle_consensus_transactions(blocks_and_rejected_transactions)
                        .await;
                }
            }));
        }
        Self { tasks }
    }

    pub(crate) async fn abort(&mut self) {
        self.tasks.shutdown().await;
    }
}

impl<C> ConsensusHandler<C> {
    fn authenticator_state_update_transaction(
        &self,
        round: u64,
        mut new_active_jwks: Vec<ActiveJwk>,
    ) -> VerifiedExecutableTransaction {
        new_active_jwks.sort();

        info!("creating authenticator state update transaction");
        //assert!(self.epoch_store.authenticator_state_enabled());
        let transaction = VerifiedTransaction::new_authenticator_state_update(
            self.epoch(),
            round,
            new_active_jwks,
            self.epoch_store
                .epoch_start_config()
                .authenticator_obj_initial_shared_version()
                .expect("authenticator state obj must exist"),
        );
        VerifiedExecutableTransaction::new_system(transaction, self.epoch())
    }

    fn epoch(&self) -> EpochId {
        self.epoch_store.epoch()
    }
}

pub(crate) fn classify(transaction: &ConsensusTransaction) -> &'static str {
    match &transaction.kind {
        ConsensusTransactionKind::CheckpointSignature(_) => "checkpoint_signature",
        ConsensusTransactionKind::EndOfPublish(_) => "end_of_publish",
        ConsensusTransactionKind::CapabilityNotificationV1(_) => "capability_notification_v1",
        ConsensusTransactionKind::TestMessage(_, _) => "test_message",
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
    System(ActionData),
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
    System(ActionData),
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
    System(ActionDigest),
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

    pub fn is_end_of_publish(&self) -> bool {
        match self {
            SequencedConsensusTransactionKind::External(ext) => {
                matches!(ext.kind, ConsensusTransactionKind::EndOfPublish(..))
            }
            SequencedConsensusTransactionKind::System(_) => false,
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

    pub fn is_end_of_publish(&self) -> bool {
        if let SequencedConsensusTransactionKind::External(ref transaction) = self.transaction {
            matches!(transaction.kind, ConsensusTransactionKind::EndOfPublish(..))
        } else {
            false
        }
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
            consensus_commit_digest: consensus_commit.consensus_digest(protocol_config),

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

/// Handles certified and rejected transactions output by consensus.
pub(crate) struct ConsensusTransactionHandler {
    /// Whether to enable handling certified transactions.
    enabled: bool,
    /// Per-epoch store.
    epoch_store: Arc<AuthorityPerEpochStore>,
    /// Metrics for consensus transaction handling.
    metrics: Arc<AuthorityMetrics>,
}

impl ConsensusTransactionHandler {
    pub fn new(
        epoch_store: Arc<AuthorityPerEpochStore>,
        metrics: Arc<AuthorityMetrics>,
    ) -> Self {
        Self {
            enabled: epoch_store.protocol_config().mysticeti_fastpath(),
            epoch_store,
            metrics,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub async fn handle_consensus_transactions(
        &self,
        blocks_and_rejected_transactions: Vec<(VerifiedBlock, Vec<TransactionIndex>)>,
    ) {
        let _scope = monitored_scope("ConsensusTransactionHandler::handle_consensus_transactions");

        let parsed_transactions = blocks_and_rejected_transactions
            .into_iter()
            .flat_map(|(block, rejected_transactions)| {
                parse_block_transactions(&block, &rejected_transactions)
            })
            .collect::<Vec<_>>();
        //let mut pending_consensus_transactions = vec![];
        // let executable_transactions: Vec<_> = parsed_transactions
        //     .into_iter()
        //     .filter_map(|parsed| {
        //         // TODO(fastpath): unlock rejected transactions.
        //         // TODO(fastpath): maybe avoid parsing blocks twice between commit and transaction handling?
        //         if parsed.rejected {
        //             self.metrics
        //                 .consensus_transaction_handler_processed
        //                 .with_label_values(&["rejected"])
        //                 .inc();
        //             return None;
        //         }
        //         self.metrics
        //             .consensus_transaction_handler_processed
        //             .with_label_values(&["certified"])
        //             .inc();
        //         match &parsed.transaction.kind {
        //             ConsensusTransactionKind::UserTransaction(tx) => {
        //                 // TODO(fastpath): use a separate function to check if a transaction should be executed in fastpath.
        //                 if tx.contains_shared_object() {
        //                     return None;
        //                 }
        //                 pending_consensus_transactions.push(parsed.transaction.clone());
        //                 let tx = VerifiedTransaction::new_unchecked(*tx.clone());
        //                 Some(VerifiedExecutableTransaction::new_from_consensus(
        //                     tx,
        //                     self.epoch_store.epoch(),
        //                 ))
        //             }
        //             _ => None,
        //         }
        //     })
        //     .collect();

        // if pending_consensus_transactions.is_empty() {
        //     return;
        // }
        // {
        //     let reconfig_state = self.epoch_store.get_reconfig_state_read_lock_guard();
        //     // Stop executing fastpath transactions when epoch change starts.
        //     if !reconfig_state.should_accept_user_certs() {
        //         return;
        //     }
        //     // Otherwise, try to ensure the certified transactions get into consensus before epoch change.
        //     // TODO(fastpath): avoid race with removals in consensus adapter, by waiting to handle commit after
        //     // all blocks in the commit are processed via the transaction handler. Other kinds of races need to be
        //     // avoided as well. Or we can track pending consensus transactions inside consensus instead.
        //     self.epoch_store
        //         .insert_pending_consensus_transactions(
        //             &pending_consensus_transactions,
        //             Some(&reconfig_state),
        //         )
        //         .unwrap_or_else(|e| {
        //             panic!("Failed to insert pending consensus transactions: {}", e)
        //         });
        // }
        // self.metrics
        //     .consensus_transaction_handler_fastpath_executions
        //     .inc_by(executable_transactions.len() as u64);
        // self.transaction_manager_sender
        //     .send(executable_transactions);
    }
}
