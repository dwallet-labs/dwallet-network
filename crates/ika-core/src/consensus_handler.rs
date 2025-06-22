// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    num::NonZeroUsize,
    sync::Arc,
};

use arc_swap::ArcSwap;
use commitment::CommitmentSizedNumber;
use consensus_config::Committee as ConsensusCommittee;
use consensus_core::CommitConsumerMonitor;
use ika_protocol_config::ProtocolConfig;
use ika_types::crypto::AuthorityName;
use ika_types::digests::{ConsensusCommitDigest, MessageDigest};
use ika_types::message::DWalletMessageKind;
use ika_types::messages_consensus::{
    AuthorityIndex, ConsensusTransaction, ConsensusTransactionKey, ConsensusTransactionKind,
};
use ika_types::sui::epoch_start_system::EpochStartSystemTrait;
use lru::LruCache;
use mysten_metrics::{monitored_future, monitored_mpsc::UnboundedReceiver, monitored_scope};
use serde::{Deserialize, Serialize};
use sui_macros::{fail_point_async, fail_point_if};
use sui_types::base_types::EpochId;

use crate::dwallet_mpc::MPCSessionLogger;
use crate::system_checkpoints::SystemCheckpointService;
use crate::{
    authority::{
        authority_per_epoch_store::{
            AuthorityPerEpochStore, ConsensusStats, ConsensusStatsAPI, ExecutionIndices,
            ExecutionIndicesWithStats,
        },
        AuthorityMetrics, AuthorityState,
    },
    consensus_throughput_calculator::ConsensusThroughputCalculator,
    consensus_types::consensus_output_api::ConsensusCommitAPI,
    dwallet_checkpoints::{DWalletCheckpointService, DWalletCheckpointServiceNotify},
    scoring_decision::update_low_scoring_authorities,
};
use ika_types::error::IkaResult;
use tokio::task::JoinSet;
use tracing::{debug, error, info, instrument, trace_span, warn};
use typed_store::Map;

pub struct ConsensusHandlerInitializer {
    state: Arc<AuthorityState>,
    checkpoint_service: Arc<DWalletCheckpointService>,
    system_checkpoint_service: Arc<SystemCheckpointService>,
    epoch_store: Arc<AuthorityPerEpochStore>,
    low_scoring_authorities: Arc<ArcSwap<HashMap<AuthorityName, u64>>>,
    throughput_calculator: Arc<ConsensusThroughputCalculator>,
}

impl ConsensusHandlerInitializer {
    pub fn new(
        state: Arc<AuthorityState>,
        checkpoint_service: Arc<DWalletCheckpointService>,
        system_checkpoint_service: Arc<SystemCheckpointService>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        low_scoring_authorities: Arc<ArcSwap<HashMap<AuthorityName, u64>>>,
        throughput_calculator: Arc<ConsensusThroughputCalculator>,
    ) -> Self {
        Self {
            state,
            checkpoint_service,
            system_checkpoint_service,
            epoch_store,
            low_scoring_authorities,
            throughput_calculator,
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_testing(
        state: Arc<AuthorityState>,
        checkpoint_service: Arc<DWalletCheckpointService>,
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

    pub(crate) fn new_consensus_handler(&self) -> ConsensusHandler<DWalletCheckpointService> {
        let new_epoch_start_state = self.epoch_store.epoch_start_state();
        let consensus_committee = new_epoch_start_state.get_consensus_committee();

        ConsensusHandler::new(
            self.epoch_store.clone(),
            self.checkpoint_service.clone(),
            self.system_checkpoint_service.clone(),
            self.low_scoring_authorities.clone(),
            consensus_committee,
            self.state.metrics.clone(),
            self.throughput_calculator.clone(),
        )
    }

    // todo(zeev): fix
    #[allow(dead_code)]
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
    system_checkpoint_service: Arc<SystemCheckpointService>,
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
        system_checkpoint_service: Arc<SystemCheckpointService>,
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
            system_checkpoint_service,
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

impl<C: DWalletCheckpointServiceNotify + Send + Sync> ConsensusHandler<C> {
    #[instrument(level = "debug", skip_all)]
    async fn handle_consensus_commit(&mut self, consensus_commit: impl ConsensusCommitAPI) {
        let _scope = monitored_scope("ConsensusCommitHandler::handle_consensus_commit");
        let round = consensus_commit.leader_round();
        let dwallet_mpc_verifier = self
            .epoch_store
            .get_dwallet_mpc_outputs_verifier_write()
            .await;
        if !dwallet_mpc_verifier.has_performed_state_sync {
            drop(dwallet_mpc_verifier);
            if let Err(err) = self.perform_dwallet_mpc_state_sync().await {
                error!(
                    "epoch switched while performing dwallet mpc state sync: {:?}",
                    err
                );
                return;
            }
            let mut dwallet_mpc_verifier = self
                .epoch_store
                .get_dwallet_mpc_outputs_verifier_write()
                .await;
            dwallet_mpc_verifier.has_performed_state_sync = true;
            drop(dwallet_mpc_verifier);
        } else {
            drop(dwallet_mpc_verifier);
        }

        let last_committed_round = self.last_consensus_stats.index.last_committed_round;

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

        debug!(
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
        debug!(num_txs = transactions.len(), "Parsed transactions");
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

        let (executable_transactions, system_checkpoint_executable_transactions) = self
            .epoch_store
            .process_consensus_transactions_and_commit_boundary(
                all_transactions,
                &self.last_consensus_stats,
                &self.checkpoint_service,
                &self.system_checkpoint_service,
                &ConsensusCommitInfo::new(self.epoch_store.protocol_config(), &consensus_commit),
                &self.metrics,
            )
            .await
            .expect("Unrecoverable error in consensus handler");

        // update the calculated throughput
        self.throughput_calculator.add_transactions(
            timestamp,
            (executable_transactions.len() + system_checkpoint_executable_transactions.len())
                as u64,
        );

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

    /// Syncs the [`DWalletMPCOutputsVerifier`] from the epoch start.
    /// Needs to be performed here,
    /// so system transactions will get created when they should, and a fork in the
    /// chain will be prevented.
    /// Fails only if the epoch switched in the middle of the state sync.
    async fn perform_dwallet_mpc_state_sync(&self) -> IkaResult {
        for item in self
            .epoch_store
            .tables()?
            .builder_dwallet_checkpoint_message_v1
            .safe_iter()
        {
            let item = item?;
            info!(
                sequence_number=?item.0,
                batch_sequence_number=?item.1.checkpoint_height,
                validator=?self.epoch_store.name,
                "Checkpoint sequence number"
            )
        }

        info!("Performing a state sync for the dWallet MPC node");
        let mut dwallet_mpc_verifier = self
            .epoch_store
            .get_dwallet_mpc_outputs_verifier_write()
            .await;

        for output in self.epoch_store.tables()?.get_all_dwallet_mpc_outputs()? {
            let party_to_authority_map = self.epoch_store.committee().party_to_authority_map();
            let mpc_protocol_name = output.session_info.mpc_round.to_string();

            // Create a base logger with common parameters.
            let base_logger = MPCSessionLogger::new()
                .with_protocol_name(mpc_protocol_name.clone())
                .with_party_to_authority_map(party_to_authority_map.clone());
            let session_identifier =
                CommitmentSizedNumber::from_le_slice(&output.session_info.session_identifier);
            base_logger.write_output_to_disk(
                session_identifier,
                self.epoch_store
                    .authority_name_to_party_id(&self.epoch_store.name)?,
                self.epoch_store
                    .authority_name_to_party_id(&output.authority)?,
                &output.output,
                &output.session_info,
            );
            if let Err(err) = dwallet_mpc_verifier
                .try_verify_output(&output.output, &output.session_info, output.authority)
                .await
            {
                error!(
                    "failed to verify output from session {:?} and party {:?}: {:?}",
                    output.session_info.session_identifier, output.authority, err
                );
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
        mut consensus_handler: ConsensusHandler<DWalletCheckpointService>,
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
    #[allow(dead_code)]
    fn epoch(&self) -> EpochId {
        self.epoch_store.epoch()
    }
}

pub(crate) fn classify(transaction: &ConsensusTransaction) -> &'static str {
    match &transaction.kind {
        ConsensusTransactionKind::DWalletCheckpointSignature(_) => "dwallet_checkpoint_signature",
        ConsensusTransactionKind::DWalletMPCMessage(..) => "dwallet_mpc_message",
        ConsensusTransactionKind::DWalletMPCOutput(..) => "dwallet_mpc_output",
        ConsensusTransactionKind::CapabilityNotificationV1(_) => "capability_notification_v1",
        ConsensusTransactionKind::DWalletMPCMaliciousReport(..) => "dwallet_mpc_malicious_report",
        ConsensusTransactionKind::DWalletMPCThresholdNotReached(..) => {
            "dwallet_mpc_threshold_not_reached"
        }
        ConsensusTransactionKind::SystemCheckpointSignature(_) => "system_checkpoint_signature",
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
    System(DWalletMessageKind),
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
    System(DWalletMessageKind),
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
    fn new(_protocol_config: &ProtocolConfig, consensus_commit: &impl ConsensusCommitAPI) -> Self {
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
