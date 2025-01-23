// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    num::NonZeroUsize,
    sync::Arc,
};

use crate::dwallet_mpc::mpc_manager::DWalletMPCDBMessage;
use crate::dwallet_mpc::mpc_outputs_verifier::{
    DWalletMPCOutputsVerifier, OutputResult, OutputVerificationResult,
};
use crate::{
    authority::{
        authority_per_epoch_store::{
            AuthorityPerEpochStore, ConsensusStats, ConsensusStatsAPI, ExecutionIndicesWithStats,
        },
        epoch_start_configuration::EpochStartConfigTrait,
        AuthorityMetrics, AuthorityState,
    },
    checkpoints::{CheckpointService, CheckpointServiceNotify},
    consensus_throughput_calculator::ConsensusThroughputCalculator,
    consensus_types::{
        committee_api::CommitteeAPI, consensus_output_api::ConsensusOutputAPI, AuthorityIndex,
    },
    execution_cache::ObjectCacheRead,
    scoring_decision::update_low_scoring_authorities,
    transaction_manager::TransactionManager,
};
use arc_swap::ArcSwap;
use async_trait::async_trait;
use consensus_core::CommitConsumerMonitor;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCPublicOutput, NetworkDecryptionKeyShares,
};
use group::PartyID;
use lru::LruCache;
use mpc::WeightedThresholdAccessStructure;
use mysten_metrics::{monitored_mpsc::UnboundedReceiver, monitored_scope, spawn_monitored_task};
use narwhal_config::Committee;
use narwhal_executor::{ExecutionIndices, ExecutionState};
use narwhal_types::{ConsensusOutput, Round};
use pera_macros::{fail_point_async, fail_point_if};
use pera_protocol_config::ProtocolConfig;
use pera_types::dwallet_mpc_error::DwalletMPCResult;
use pera_types::error::PeraResult;
use pera_types::executable_transaction::CertificateProof;
use pera_types::message_envelope::VerifiedEnvelope;
use pera_types::messages_dwallet_mpc::{
    DWalletMPCEvent, DWalletMPCOutput, DWalletMPCOutputMessage, MPCRound, SessionInfo,
};
use pera_types::{
    authenticator_state::ActiveJwk,
    base_types::{AuthorityName, EpochId, ObjectID, SequenceNumber, TransactionDigest},
    digests::ConsensusCommitDigest,
    executable_transaction::{TrustedExecutableTransaction, VerifiedExecutableTransaction},
    messages_consensus::{ConsensusTransaction, ConsensusTransactionKey, ConsensusTransactionKind},
    pera_system_state::epoch_start_pera_system_state::EpochStartSystemStateTrait,
    transaction::{SenderSignedData, VerifiedTransaction},
};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, trace_span, warn};
use twopc_mpc::secp256k1;
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

    pub fn new_for_testing(
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
    pub fn new_consensus_handler(&self) -> ConsensusHandler<CheckpointService> {
        let new_epoch_start_state = self.epoch_store.epoch_start_state();
        let committee = new_epoch_start_state.get_narwhal_committee();

        ConsensusHandler::new(
            self.epoch_store.clone(),
            self.checkpoint_service.clone(),
            self.state.transaction_manager().clone(),
            self.state.get_object_cache_reader().clone(),
            self.low_scoring_authorities.clone(),
            committee,
            self.state.metrics.clone(),
            self.throughput_calculator.clone(),
        )
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
    /// cache reader is needed when determining the next version to assign for shared objects.
    cache_reader: Arc<dyn ObjectCacheRead>,
    /// Reputation scores used by consensus adapter that we update, forwarded from consensus
    low_scoring_authorities: Arc<ArcSwap<HashMap<AuthorityName, u64>>>,
    /// The narwhal committee used to do stake computations for deciding set of low scoring authorities
    committee: Committee,
    // TODO: ConsensusHandler doesn't really share metrics with AuthorityState. We could define
    // a new metrics type here if we want to.
    metrics: Arc<AuthorityMetrics>,
    /// Lru cache to quickly discard transactions processed by consensus
    processed_cache: LruCache<SequencedConsensusTransactionKey, ()>,
    transaction_scheduler: AsyncTransactionScheduler,
    /// Using the throughput calculator to record the current consensus throughput
    throughput_calculator: Arc<ConsensusThroughputCalculator>,
}

const PROCESSED_CACHE_CAP: usize = 1024 * 1024;

impl<C> ConsensusHandler<C> {
    pub fn new(
        epoch_store: Arc<AuthorityPerEpochStore>,
        checkpoint_service: Arc<C>,
        transaction_manager: Arc<TransactionManager>,
        cache_reader: Arc<dyn ObjectCacheRead>,
        low_scoring_authorities: Arc<ArcSwap<HashMap<AuthorityName, u64>>>,
        committee: Committee,
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
        let transaction_scheduler =
            AsyncTransactionScheduler::start(transaction_manager, epoch_store.clone());
        Self {
            epoch_store,
            last_consensus_stats,
            checkpoint_service,
            cache_reader,
            low_scoring_authorities,
            committee,
            metrics,
            processed_cache: LruCache::new(NonZeroUsize::new(PROCESSED_CACHE_CAP).unwrap()),
            transaction_scheduler,
            throughput_calculator,
        }
    }

    /// Updates the execution indexes based on the provided input.
    fn update_index_and_hash(&mut self, index: ExecutionIndices, v: &[u8]) {
        update_index_and_hash(&mut self.last_consensus_stats, index, v)
    }
}

fn update_index_and_hash(
    last_consensus_stats: &mut ExecutionIndicesWithStats,
    index: ExecutionIndices,
    v: &[u8],
) {
    // The entry point of handle_consensus_output_internal() has filtered out any already processed
    // consensus output. So we can safely assume that the index is always increasing.
    assert!(last_consensus_stats.index < index);

    let previous_hash = last_consensus_stats.hash;
    let mut hasher = DefaultHasher::new();
    previous_hash.hash(&mut hasher);
    v.hash(&mut hasher);
    let hash = hasher.finish();
    // Log hash every 1000th transaction of the subdag
    if index.transaction_index % 1000 == 0 {
        info!(
            "Integrity hash for consensus output at subdag {} transaction {} is {:016x}",
            index.sub_dag_index, index.transaction_index, hash
        );
    }

    last_consensus_stats.index = index;
    last_consensus_stats.hash = hash;
}

#[async_trait]
impl<C: CheckpointServiceNotify + Send + Sync> ExecutionState for ConsensusHandler<C> {
    /// This function gets called by the consensus for each consensus commit.
    #[instrument(level = "debug", skip_all)]
    async fn handle_consensus_output(&mut self, consensus_output: ConsensusOutput) {
        let _scope = monitored_scope("HandleConsensusOutput");
        self.handle_consensus_output_internal(consensus_output)
            .await;
    }

    fn last_executed_sub_dag_round(&self) -> u64 {
        self.last_consensus_stats.index.last_committed_round
    }

    fn last_executed_sub_dag_index(&self) -> u64 {
        self.last_consensus_stats.index.sub_dag_index
    }
}

impl<C: CheckpointServiceNotify + Send + Sync> ConsensusHandler<C> {
    #[instrument(level = "debug", skip_all)]
    async fn handle_consensus_output_internal(
        &mut self,
        consensus_output: impl ConsensusOutputAPI,
    ) {
        // This code no longer supports old protocol versions.
        assert!(self
            .epoch_store
            .protocol_config()
            .consensus_order_end_of_epoch_last());

        let last_committed_round = self.last_consensus_stats.index.sub_dag_index;

        // Check if the dwallet mpc manager should perform a state sync, and if so block consensus outputs processing & perform the state sync
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
        dwallet_mpc_verifier.last_processed_round = last_committed_round;
        // Need to drop the verifier, as `self` is being used mutably later in this function
        drop(dwallet_mpc_verifier);

        let round = consensus_output.leader_round();

        // TODO: Remove this once narwhal is deprecated. For now mysticeti will not return
        // more than one leader per round so we are not in danger of ignoring any commits.
        assert!(round >= last_committed_round);
        if last_committed_round == round {
            // we can receive the same commit twice after restart
            // It is critical that the writes done by this function are atomic - otherwise we can
            // lose the later parts of a commit if we restart midway through processing it.
            warn!(
                "Ignoring consensus output for round {} as it is already committed. NOTE: This is only expected if Narwhal is running.",
                round
            );
            return;
        }

        /* (serialized, transaction, output_cert) */
        let mut transactions = vec![];
        let timestamp = consensus_output.commit_timestamp_ms();
        let leader_author = consensus_output.leader_author_index();
        let commit_sub_dag_index = consensus_output.commit_sub_dag_index();

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
            %consensus_output,
            epoch = ?self.epoch_store.epoch(),
            "Received consensus output"
        );

        // TODO: testing empty commit explicitly.
        // Note that consensus commit batch may contain no transactions, but we still need to record the current
        // round and subdag index in the last_consensus_stats, so that it won't be re-executed in the future.
        let empty_bytes = vec![];
        self.update_index_and_hash(
            ExecutionIndices {
                last_committed_round: round,
                sub_dag_index: commit_sub_dag_index,
                transaction_index: 0_u64,
            },
            &empty_bytes,
        );

        // Load all jwks that became active in the previous round, and commit them in this round.
        // We want to delay one round because none of the transactions in the previous round could
        // have been authenticated with the jwks that became active in that round.
        //
        // Because of this delay, jwks that become active in the last round of the epoch will
        // never be committed. That is ok, because in the new epoch, the validators should
        // immediately re-submit these jwks, and they can become active then.
        let new_jwks = self
            .epoch_store
            .get_new_jwks(last_committed_round)
            .expect("Unrecoverable error in consensus handler");

        if !new_jwks.is_empty() {
            let authenticator_state_update_transaction =
                self.authenticator_state_update_transaction(round, new_jwks);
            debug!(
                "adding AuthenticatorStateUpdate({:?}) tx: {:?}",
                authenticator_state_update_transaction.digest(),
                authenticator_state_update_transaction,
            );

            transactions.push((
                empty_bytes.as_slice(),
                SequencedConsensusTransactionKind::System(authenticator_state_update_transaction),
                leader_author,
            ));
        }

        update_low_scoring_authorities(
            self.low_scoring_authorities.clone(),
            &self.committee,
            consensus_output.reputation_score_sorted_desc(),
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
            let span = trace_span!("process_consensus_certs");
            let _guard = span.enter();
            for (authority_index, authority_transactions) in consensus_output.transactions() {
                // TODO: consider only messages within 1~3 rounds of the leader?
                self.last_consensus_stats
                    .stats
                    .inc_num_messages(authority_index as usize);
                for (serialized_transaction, transaction) in authority_transactions {
                    let kind = classify(&transaction);
                    self.metrics
                        .consensus_handler_processed
                        .with_label_values(&[kind])
                        .inc();
                    self.metrics
                        .consensus_handler_transaction_sizes
                        .with_label_values(&[kind])
                        .observe(serialized_transaction.len() as f64);
                    if matches!(
                        &transaction.kind,
                        ConsensusTransactionKind::UserTransaction(_)
                    ) {
                        self.last_consensus_stats
                            .stats
                            .inc_num_user_transactions(authority_index as usize);
                    }

                    if let ConsensusTransactionKind::LockNextCommittee(authority, epoch_id) =
                        &transaction.kind
                    {
                        if *epoch_id != self.epoch_store.epoch() {
                            error!(
                                "received `LockNextCommittee` transaction for the epoch {:?} while processing epoch {:?}",
                                epoch_id,
                                self.epoch_store.epoch()
                            );
                            continue;
                        }
                        let mut dwallet_mpc_verifier =
                            self.epoch_store.get_dwallet_mpc_outputs_verifier().await;
                        self.epoch_store
                            .save_dwallet_mpc_message(
                                DWalletMPCDBMessage::LockNextEpochCommitteeVote(*authority),
                            )
                            .await;
                        if dwallet_mpc_verifier.should_lock_committee(*authority) {
                            let transaction =
                                VerifiedTransaction::new_lock_next_committee_system_transaction(
                                    *epoch_id,
                                );
                            let transaction = VerifiedExecutableTransaction::new_system(
                                transaction,
                                self.epoch(),
                            );
                            transactions.push((
                                empty_bytes.as_slice(),
                                SequencedConsensusTransactionKind::System(transaction),
                                consensus_output.leader_author_index(),
                            ));
                        }
                    }
                    // If we receive a `DwalletMPCOutput` transaction,
                    // verify that it's valid and create a system transaction
                    // to store its output on the blockchain,
                    // so it will be available for the initiating user.
                    else if let ConsensusTransactionKind::DWalletMPCOutput(
                        _authority,
                        session_info,
                        output,
                    ) = &transaction.kind
                    {
                        info!(
                            "Received dWallet MPC output from authority {:?} for session {:?}",
                            authority_index, session_info.session_id
                        );
                        let Some(origin_authority) =
                            self.committee.authority_pubkey_by_index(authority_index)
                        else {
                            error!(
                                "malicious output from unknown authority index: {:?}, for session: {:?}",
                                authority_index, session_info.session_id
                            );
                            continue;
                        };
                        self.epoch_store
                            .save_dwallet_mpc_output(DWalletMPCOutputMessage {
                                output: output.clone(),
                                authority: origin_authority.clone(),
                                session_info: session_info.clone(),
                            })
                            .await;
                        let mut dwallet_mpc_verifier =
                            self.epoch_store.get_dwallet_mpc_outputs_verifier().await;
                        let output_verification_result = dwallet_mpc_verifier
                            .try_verify_output(output, &session_info, origin_authority)
                            .unwrap_or_else(|e| {
                                error!("error verifying DWalletMPCOutput output from session {:?} and party {:?}: {:?}",session_info.session_id, authority_index, e);
                                OutputVerificationResult {
                                    result: OutputResult::Malicious,
                                    malicious_actors: vec![origin_authority],
                                }
                            });
                        match output_verification_result.result {
                            OutputResult::Valid => {
                                self.epoch_store
                                    .save_dwallet_mpc_completed_session(session_info.session_id)
                                    .await;
                                if session_info.mpc_round.is_part_of_batch() {
                                    let mut batches_manager =
                                        self.epoch_store.get_dwallet_mpc_batches_manager().await
                                    else {
                                        error!("failed to get dWallet MPC batches manager when processing DWalletMPCOutput transaction");
                                        continue;
                                    };
                                    if let Err(err) = batches_manager
                                        .store_verified_output(session_info.clone(), output.clone())
                                    {
                                        error!(
                                            "error storing verified output for session {:?}: {:?}",
                                            session_info.session_id, err
                                        );
                                    }
                                    match batches_manager.is_batch_completed(session_info) {
                                        Ok(Some(batch_output)) => {
                                            let transaction = self
                                                .create_dwallet_mpc_output_system_tx(
                                                    session_info,
                                                    &batch_output,
                                                );
                                            transactions.push((
                                                empty_bytes.as_slice(),
                                                SequencedConsensusTransactionKind::System(
                                                    transaction,
                                                ),
                                                consensus_output.leader_author_index(),
                                            ));
                                        }
                                        Err(err) => {
                                            error!(
                                                "session: `{:?}` error checking if batch is completed: {:?}",
                                                session_info.session_id, err
                                            );
                                            continue;
                                        }
                                        _ => {
                                            debug!(
                                                "session: '{:?}' received a batch output",
                                                session_info.session_id
                                            );
                                            // We don't want to write this output to the chain,
                                            // as we stored it in the batch,
                                            // but the batch is not yet complete.
                                            continue;
                                        }
                                    }
                                } else {
                                    // Extract the final network DKG transaction parameters from
                                    // the verified output.
                                    // We can't preform this within the execution engine,
                                    // as it requires the class-groups crate from crypto-private lib.
                                    if let MPCRound::NetworkDkg(key_scheme, _) =
                                        session_info.mpc_round
                                    {
                                        let weighted_threshold_access_structure = match self
                                            .epoch_store
                                            .get_weighted_threshold_access_structure()
                                        {
                                            Ok(value) => value,
                                            Err(e) => {
                                                error!(
                                                    "Failed to create access structure  {:?}",
                                                    e
                                                );
                                                continue;
                                            }
                                        };

                                        let transaction = match self
                                            .create_dwallet_network_output_system_tx(
                                                &session_info,
                                                output,
                                                &weighted_threshold_access_structure,
                                                key_scheme,
                                            ) {
                                            Ok(tx) => tx,
                                            Err(e) => {
                                                error!("Failed to create dwallet network output system tx`: {:?}", e);
                                                continue;
                                            }
                                        };
                                        transactions.push((
                                            empty_bytes.as_slice(),
                                            SequencedConsensusTransactionKind::System(transaction),
                                            consensus_output.leader_author_index(),
                                        ));
                                    } else {
                                        let transaction = self.create_dwallet_mpc_output_system_tx(
                                            session_info,
                                            output,
                                        );
                                        transactions.push((
                                            empty_bytes.as_slice(),
                                            SequencedConsensusTransactionKind::System(transaction),
                                            consensus_output.leader_author_index(),
                                        ));
                                    }
                                }
                            }
                            OutputResult::NotEnoughVotes
                            | OutputResult::AlreadyCommitted
                            | OutputResult::Malicious
                            | OutputResult::Duplicate => {
                                // Ignore this output,
                                continue;
                            }
                        }
                    } else if let ConsensusTransactionKind::RandomnessStateUpdate(
                        randomness_round,
                        _,
                    ) = &transaction.kind
                    {
                        // These are deprecated and we should never see them. Log an error and eat the tx if one appears.
                        error!("BUG: saw deprecated RandomnessStateUpdate tx for commit round {round:?}, randomness round {randomness_round:?}")
                    } else {
                        let transaction = SequencedConsensusTransactionKind::External(transaction);
                        transactions.push((serialized_transaction, transaction, authority_index));
                    }
                }
            }
        }

        for i in 0..self.committee.size() {
            let hostname = self
                .committee
                .authority_hostname_by_index(i as AuthorityIndex)
                .unwrap_or_default();
            self.metrics
                .consensus_committed_messages
                .with_label_values(&[hostname])
                .set(self.last_consensus_stats.stats.get_num_messages(i) as i64);
            self.metrics
                .consensus_committed_user_transactions
                .with_label_values(&[hostname])
                .set(self.last_consensus_stats.stats.get_num_user_transactions(i) as i64);
        }

        let mut all_transactions = Vec::new();
        {
            // We need a set here as well, since the processed_cache is a LRU cache and can drop
            // entries while we're iterating over the sequenced transactions.
            let mut processed_set = HashSet::new();

            for (seq, (serialized, transaction, cert_origin)) in
                transactions.into_iter().enumerate()
            {
                // In process_consensus_transactions_and_commit_boundary(), we will add a system consensus commit
                // prologue transaction, which will be the first transaction in this consensus commit batch.
                // Therefore, the transaction sequence number starts from 1 here.
                let current_tx_index = ExecutionIndices {
                    last_committed_round: round,
                    sub_dag_index: commit_sub_dag_index,
                    transaction_index: (seq + 1) as u64,
                };

                self.update_index_and_hash(current_tx_index, serialized);

                let certificate_author = self
                    .committee
                    .authority_pubkey_by_index(cert_origin)
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

        let transactions_to_schedule = self
            .epoch_store
            .process_consensus_transactions_and_commit_boundary(
                all_transactions,
                &self.last_consensus_stats,
                &self.checkpoint_service,
                self.cache_reader.as_ref(),
                &ConsensusCommitInfo::new(self.epoch_store.protocol_config(), &consensus_output),
                &self.metrics,
            )
            .await
            .expect("Unrecoverable error in consensus handler");

        // update the calculated throughput
        self.throughput_calculator
            .add_transactions(timestamp, transactions_to_schedule.len() as u64);

        fail_point_if!("correlated-crash-after-consensus-commit-boundary", || {
            let key = [commit_sub_dag_index, self.epoch_store.epoch()];
            if pera_simulator::random::deterministic_probability(&key, 0.01) {
                pera_simulator::task::kill_current_node(None);
            }
        });

        fail_point_async!("crash"); // for tests that produce random crashes

        self.transaction_scheduler
            .schedule(transactions_to_schedule)
            .await;
    }

    fn create_dwallet_network_output_system_tx(
        &self,
        session_info: &SessionInfo,
        verified_output: &[u8],
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
        key_scheme: DWalletMPCNetworkKeyScheme,
    ) -> DwalletMPCResult<VerifiedEnvelope<SenderSignedData, CertificateProof>> {
        let key = crate::dwallet_mpc::network_dkg::dwallet_mpc_network_key_from_session_output(
            self.epoch(),
            key_scheme,
            &weighted_threshold_access_structure,
            verified_output,
        )?;

        let mut new_session_info = session_info.clone();
        new_session_info.mpc_round = MPCRound::NetworkDkg(key_scheme, Some(key));

        Ok(self.create_dwallet_mpc_output_system_tx(&new_session_info, verified_output))
    }

    fn create_dwallet_mpc_output_system_tx(
        &self,
        session_info: &SessionInfo,
        output: &[u8],
    ) -> VerifiedEnvelope<SenderSignedData, CertificateProof> {
        let transaction =
            VerifiedTransaction::new_dwallet_mpc_output_system_transaction(DWalletMPCOutput {
                session_info: session_info.clone(),
                output: Vec::from(output),
            });
        VerifiedExecutableTransaction::new_system(transaction, self.epoch())
    }

    /// Loads all DWallet MPC messages from the epoch start from the epoch tables.
    /// Needed to be a separate function because the DB table does not implement the `Send` trait,
    /// hence async code involving it can cause compilation errors.
    async fn load_dwallet_mpc_messages_from_epoch_start(
        &self,
    ) -> PeraResult<Vec<DWalletMPCDBMessage>> {
        Ok(self
            .epoch_store
            .tables()?
            .dwallet_mpc_messages
            .unbounded_iter()
            .map(|(_, messages)| messages)
            .flatten()
            .collect())
    }

    /// Loads all DWallet MPC outputs from the epoch start from the epoch tables.
    /// Needed to be a separate function because the DB table does not implement the `Send` trait,
    /// hence async code involving it can cause compilation errors.
    async fn load_dwallet_mpc_outputs_from_epoch_start(
        &self,
    ) -> PeraResult<Vec<DWalletMPCOutputMessage>> {
        Ok(self
            .epoch_store
            .tables()?
            .dwallet_mpc_outputs
            .unbounded_iter()
            .map(|(_, messages)| messages)
            .flatten()
            .collect())
    }

    /// Loads all DWallet MPC events from the epoch start from the epoch tables.
    /// Needed to be a separate function because the DB table does not implement the `Send` trait,
    /// hence async code involving it can cause compilation errors.
    async fn load_dwallet_mpc_events_from_epoch_start(&self) -> PeraResult<Vec<DWalletMPCEvent>> {
        Ok(self
            .epoch_store
            .tables()?
            .dwallet_mpc_events
            .unbounded_iter()
            .map(|(_, messages)| messages)
            .flatten()
            .collect())
    }

    async fn should_perform_dwallet_mpc_state_sync(&self) -> bool {
        let mut dwallet_mpc_verifier = self.epoch_store.get_dwallet_mpc_outputs_verifier().await;
        // Check if the dwallet mpc manager should perform a state sync, and if so block consensus and load all messages
        // This condition is only true if we process a round before we processed the previous round,
        // which can only happen if we restart the node.
        self.last_consensus_stats.index.sub_dag_index
            > dwallet_mpc_verifier.last_processed_round + 1
    }

    /// Syncs the [`DWalletMPCOutputsVerifier`] from the epoch start.
    /// Needed to be performed here so system transactions will get created when they should & a fork in the
    /// chain will be prevented.
    /// Fails only if the epoch switched in the middle of the state sync.
    async fn perform_dwallet_mpc_state_sync(&self) -> PeraResult {
        info!("performs a state sync for the DWallet MPC node");
        let mut dwallet_mpc_verifier = self.epoch_store.get_dwallet_mpc_outputs_verifier().await;
        let mut dwallet_mpc_batches_manager =
            self.epoch_store.get_dwallet_mpc_batches_manager().await;
        for event in self.load_dwallet_mpc_events_from_epoch_start().await? {
            dwallet_mpc_batches_manager.handle_new_event(&event.session_info);
            dwallet_mpc_verifier.handle_new_event(&event.session_info);
        }
        for output in self.load_dwallet_mpc_outputs_from_epoch_start().await? {
            match dwallet_mpc_verifier.try_verify_output(
                &output.output,
                &output.session_info,
                output.authority,
            ) {
                Ok(result) => {
                    // TODO (#524): Handle malicious behavior.
                    if result.result == OutputResult::Valid {
                        if output.session_info.mpc_round.is_part_of_batch() {
                            if let Err(err) = dwallet_mpc_batches_manager.store_verified_output(
                                output.session_info.clone(),
                                output.output.clone(),
                            ) {
                                error!(
                                    "error storing verified output in batch for session {:?}: {:?}",
                                    output.session_info.session_id, err
                                );
                            }
                        }
                    }
                }
                Err(err) => {
                    error!(
                        "failed to verify output from session {:?} and party {:?}: {:?}",
                        output.session_info.session_id, output.authority, err
                    );
                }
            }
        }
        for message in self.load_dwallet_mpc_messages_from_epoch_start().await? {
            match message {
                DWalletMPCDBMessage::LockNextEpochCommitteeVote(authority) => {
                    dwallet_mpc_verifier.should_lock_committee(authority);
                }
                DWalletMPCDBMessage::Message(_)
                | DWalletMPCDBMessage::EndOfDelivery
                | DWalletMPCDBMessage::StartLockNextEpochCommittee
                | DWalletMPCDBMessage::ValidatorDataForDKG(_)
                | DWalletMPCDBMessage::MPCSessionFailed(_)
                | DWalletMPCDBMessage::SessionFailedWithMaliciousParties(..)
                | DWalletMPCDBMessage::PerformCryptographicComputations => {}
            }
        }
        Ok(())
    }
}

struct AsyncTransactionScheduler {
    sender: tokio::sync::mpsc::Sender<Vec<VerifiedExecutableTransaction>>,
}

impl AsyncTransactionScheduler {
    pub fn start(
        transaction_manager: Arc<TransactionManager>,
        epoch_store: Arc<AuthorityPerEpochStore>,
    ) -> Self {
        let (sender, recv) = tokio::sync::mpsc::channel(16);
        spawn_monitored_task!(Self::run(recv, transaction_manager, epoch_store));
        Self { sender }
    }

    pub async fn schedule(&self, transactions: Vec<VerifiedExecutableTransaction>) {
        self.sender.send(transactions).await.ok();
    }

    pub async fn run(
        mut recv: tokio::sync::mpsc::Receiver<Vec<VerifiedExecutableTransaction>>,
        transaction_manager: Arc<TransactionManager>,
        epoch_store: Arc<AuthorityPerEpochStore>,
    ) {
        while let Some(transactions) = recv.recv().await {
            let _guard = monitored_scope("ConsensusHandler::enqueue");
            transaction_manager.enqueue(transactions, &epoch_store);
        }
    }
}

/// Consensus handler used by Mysticeti. Since Mysticeti repo is not yet integrated, we use a
/// channel to receive the consensus output from Mysticeti.
/// During initialization, the sender is passed into Mysticeti which can send consensus output
/// to the channel.
pub struct MysticetiConsensusHandler {
    handle: Option<tokio::task::JoinHandle<()>>,
}

impl MysticetiConsensusHandler {
    pub fn new(
        mut consensus_handler: ConsensusHandler<CheckpointService>,
        mut receiver: UnboundedReceiver<consensus_core::CommittedSubDag>,
        commit_consumer_monitor: Arc<CommitConsumerMonitor>,
    ) -> Self {
        let handle = spawn_monitored_task!(async move {
            // TODO: pause when execution is overloaded, so consensus can detect the backpressure.
            while let Some(consensus_output) = receiver.recv().await {
                let commit_index = consensus_output.commit_ref.index;
                consensus_handler
                    .handle_consensus_output_internal(consensus_output)
                    .await;
                commit_consumer_monitor.set_highest_handled_commit(commit_index);
            }
        });
        Self {
            handle: Some(handle),
        }
    }

    pub async fn abort(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
            let _ = handle.await;
        }
    }
}

impl Drop for MysticetiConsensusHandler {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
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
        assert!(self.epoch_store.authenticator_state_enabled());
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
        ConsensusTransactionKind::UserTransaction(certificate) => {
            if certificate.contains_shared_object() {
                "shared_certificate"
            } else {
                "owned_certificate"
            }
        }
        ConsensusTransactionKind::CheckpointSignature(_) => "checkpoint_signature",
        ConsensusTransactionKind::EndOfPublish(_) => "end_of_publish",
        ConsensusTransactionKind::CapabilityNotification(_) => "capability_notification",
        ConsensusTransactionKind::CapabilityNotificationV2(_) => "capability_notification_v2",
        ConsensusTransactionKind::NewJWKFetched(_, _, _) => "new_jwk_fetched",
        ConsensusTransactionKind::RandomnessStateUpdate(_, _) => "randomness_state_update",
        ConsensusTransactionKind::RandomnessDkgMessage(_, _) => "randomness_dkg_message",
        ConsensusTransactionKind::RandomnessDkgConfirmation(_, _) => "randomness_dkg_confirmation",
        ConsensusTransactionKind::DWalletMPCMessage(..) => "dwallet_mpc_message",
        ConsensusTransactionKind::DWalletMPCOutput(..) => "dwallet_mpc_output",
        ConsensusTransactionKind::LockNextCommittee(..) => "lock_next_committee",
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
    System(VerifiedExecutableTransaction),
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
    System(TrustedExecutableTransaction),
}

impl From<&SequencedConsensusTransactionKind> for SerializableSequencedConsensusTransactionKind {
    fn from(kind: &SequencedConsensusTransactionKind) -> Self {
        match kind {
            SequencedConsensusTransactionKind::External(ext) => {
                SerializableSequencedConsensusTransactionKind::External(ext.clone())
            }
            SequencedConsensusTransactionKind::System(txn) => {
                SerializableSequencedConsensusTransactionKind::System(txn.clone().serializable())
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
                SequencedConsensusTransactionKind::System(txn.into())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub enum SequencedConsensusTransactionKey {
    External(ConsensusTransactionKey),
    System(TransactionDigest),
}

impl SequencedConsensusTransactionKind {
    pub fn key(&self) -> SequencedConsensusTransactionKey {
        match self {
            SequencedConsensusTransactionKind::External(ext) => {
                SequencedConsensusTransactionKey::External(ext.key())
            }
            SequencedConsensusTransactionKind::System(txn) => {
                SequencedConsensusTransactionKey::System(*txn.digest())
            }
        }
    }

    pub fn get_tracking_id(&self) -> u64 {
        match self {
            SequencedConsensusTransactionKind::External(ext) => ext.get_tracking_id(),
            SequencedConsensusTransactionKind::System(_txn) => 0,
        }
    }

    pub fn is_executable_transaction(&self) -> bool {
        match self {
            SequencedConsensusTransactionKind::External(ext) => ext.is_user_certificate(),
            SequencedConsensusTransactionKind::System(_) => true,
        }
    }

    pub fn executable_transaction_digest(&self) -> Option<TransactionDigest> {
        match self {
            SequencedConsensusTransactionKind::External(ext) => {
                if let ConsensusTransactionKind::UserTransaction(txn) = &ext.kind {
                    Some(*txn.digest())
                } else {
                    None
                }
            }
            SequencedConsensusTransactionKind::System(txn) => Some(*txn.digest()),
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

    pub fn is_user_tx_with_randomness(&self, randomness_state_enabled: bool) -> bool {
        if !randomness_state_enabled {
            // If randomness is disabled, these should be processed same as a tx without randomness,
            // which will eventually fail when the randomness state object is not found.
            return false;
        }
        let SequencedConsensusTransactionKind::External(ConsensusTransaction {
            kind: ConsensusTransactionKind::UserTransaction(certificate),
            ..
        }) = &self.transaction
        else {
            return false;
        };
        certificate.transaction_data().uses_randomness()
    }

    pub fn as_shared_object_txn(&self) -> Option<&SenderSignedData> {
        match &self.transaction {
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::UserTransaction(certificate),
                ..
            }) if certificate.contains_shared_object() => Some(certificate.data()),
            SequencedConsensusTransactionKind::System(txn) if txn.contains_shared_object() => {
                Some(txn.data())
            }
            _ => None,
        }
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
    fn new(protocol_config: &ProtocolConfig, consensus_output: &impl ConsensusOutputAPI) -> Self {
        Self {
            round: consensus_output.leader_round(),
            timestamp: consensus_output.commit_timestamp_ms(),
            consensus_commit_digest: consensus_output.consensus_digest(protocol_config),

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

    fn consensus_commit_prologue_transaction(&self, epoch: u64) -> VerifiedExecutableTransaction {
        let transaction =
            VerifiedTransaction::new_consensus_commit_prologue(epoch, self.round, self.timestamp);
        VerifiedExecutableTransaction::new_system(transaction, epoch)
    }

    fn consensus_commit_prologue_v2_transaction(
        &self,
        epoch: u64,
    ) -> VerifiedExecutableTransaction {
        let transaction = VerifiedTransaction::new_consensus_commit_prologue_v2(
            epoch,
            self.round,
            self.timestamp,
            self.consensus_commit_digest,
        );
        VerifiedExecutableTransaction::new_system(transaction, epoch)
    }

    fn consensus_commit_prologue_v3_transaction(
        &self,
        epoch: u64,
        cancelled_txn_version_assignment: Vec<(TransactionDigest, Vec<(ObjectID, SequenceNumber)>)>,
    ) -> VerifiedExecutableTransaction {
        let transaction = VerifiedTransaction::new_consensus_commit_prologue_v3(
            epoch,
            self.round,
            self.timestamp,
            self.consensus_commit_digest,
            cancelled_txn_version_assignment,
        );
        VerifiedExecutableTransaction::new_system(transaction, epoch)
    }

    pub fn create_consensus_commit_prologue_transaction(
        &self,
        epoch: u64,
        protocol_config: &ProtocolConfig,
        cancelled_txn_version_assignment: Vec<(TransactionDigest, Vec<(ObjectID, SequenceNumber)>)>,
    ) -> VerifiedExecutableTransaction {
        if protocol_config.record_consensus_determined_version_assignments_in_prologue() {
            self.consensus_commit_prologue_v3_transaction(epoch, cancelled_txn_version_assignment)
        } else if protocol_config.include_consensus_digest_in_prologue() {
            self.consensus_commit_prologue_v2_transaction(epoch)
        } else {
            self.consensus_commit_prologue_transaction(epoch)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use prometheus::Registry;

    use narwhal_config::AuthorityIdentifier;
    use narwhal_test_utils::latest_protocol_version;
    use narwhal_types::{Batch, Certificate, CommittedSubDag, HeaderV1Builder, ReputationScores};
    use pera_protocol_config::ConsensusTransactionOrdering;
    use pera_types::{
        base_types::{random_object_ref, AuthorityName, PeraAddress},
        committee::Committee,
        messages_consensus::{
            AuthorityCapabilitiesV1, ConsensusTransaction, ConsensusTransactionKind,
        },
        object::Object,
        pera_system_state::epoch_start_pera_system_state::EpochStartSystemStateTrait,
        supported_protocol_versions::SupportedProtocolVersions,
        transaction::{
            CertifiedTransaction, SenderSignedData, TransactionData, TransactionDataAPI,
        },
    };

    use crate::{
        authority::{
            authority_per_epoch_store::ConsensusStatsAPI,
            test_authority_builder::TestAuthorityBuilder,
        },
        checkpoints::CheckpointServiceNoop,
        consensus_adapter::consensus_tests::{test_certificates, test_gas_objects},
        post_consensus_tx_reorder::PostConsensusTxReorder,
    };

    use super::*;

    #[tokio::test]
    pub async fn test_consensus_handler() {
        // GIVEN
        let mut objects = test_gas_objects();
        let shared_object = Object::shared_for_testing();
        objects.push(shared_object.clone());

        let latest_protocol_config = &latest_protocol_version();

        let network_config =
            pera_swarm_config::network_config_builder::ConfigBuilder::new_with_temp_dir()
                .with_objects(objects.clone())
                .build();

        let state = TestAuthorityBuilder::new()
            .with_network_config(&network_config, 0)
            .build()
            .await;

        let epoch_store = state.epoch_store_for_testing().clone();
        let new_epoch_start_state = epoch_store.epoch_start_state();
        let committee = new_epoch_start_state.get_narwhal_committee();

        let metrics = Arc::new(AuthorityMetrics::new(&Registry::new()));

        let throughput_calculator = ConsensusThroughputCalculator::new(None, metrics.clone());

        let mut consensus_handler = ConsensusHandler::new(
            epoch_store,
            Arc::new(CheckpointServiceNoop {}),
            state.transaction_manager().clone(),
            state.get_object_cache_reader().clone(),
            Arc::new(ArcSwap::default()),
            committee.clone(),
            metrics,
            Arc::new(throughput_calculator),
        );

        // AND
        // Create test transactions
        let transactions = test_certificates(&state, shared_object).await;
        let mut certificates = Vec::new();
        let mut batches = Vec::new();

        for transaction in transactions.iter() {
            let transaction_bytes: Vec<u8> = bcs::to_bytes(
                &ConsensusTransaction::new_certificate_message(&state.name, transaction.clone()),
            )
            .unwrap();

            let batch = Batch::new(vec![transaction_bytes], latest_protocol_config);

            batches.push(vec![batch.clone()]);

            // AND make batch as part of a commit
            let header = HeaderV1Builder::default()
                .author(AuthorityIdentifier(0))
                .round(5)
                .epoch(0)
                .parents(BTreeSet::new())
                .with_payload_batch(batch.clone(), 0, 0)
                .build()
                .unwrap();

            let certificate = Certificate::new_unsigned(
                latest_protocol_config,
                &committee,
                header.into(),
                vec![],
            )
            .unwrap();

            certificates.push(certificate);
        }

        // AND create the consensus output
        let consensus_output = ConsensusOutput {
            sub_dag: Arc::new(CommittedSubDag::new(
                certificates.clone(),
                certificates[0].clone(),
                10,
                ReputationScores::default(),
                None,
            )),
            batches,
        };

        // AND processing the consensus output once
        consensus_handler
            .handle_consensus_output(consensus_output.clone())
            .await;

        // AND capturing the consensus stats
        let num_certificates = certificates.len();
        let num_transactions = transactions.len();
        let last_consensus_stats_1 = consensus_handler.last_consensus_stats.clone();
        assert_eq!(
            last_consensus_stats_1.index.transaction_index,
            num_transactions as u64
        );
        assert_eq!(last_consensus_stats_1.index.sub_dag_index, 10_u64);
        assert_eq!(last_consensus_stats_1.index.last_committed_round, 5_u64);
        assert_ne!(last_consensus_stats_1.hash, 0);
        assert_eq!(
            last_consensus_stats_1.stats.get_num_messages(0),
            num_certificates as u64
        );
        assert_eq!(
            last_consensus_stats_1.stats.get_num_user_transactions(0),
            num_transactions as u64
        );

        // WHEN processing the same output multiple times
        // THEN the consensus stats do not update
        for _ in 0..2 {
            consensus_handler
                .handle_consensus_output(consensus_output.clone())
                .await;
            let last_consensus_stats_2 = consensus_handler.last_consensus_stats.clone();
            assert_eq!(last_consensus_stats_1, last_consensus_stats_2);
        }
    }

    #[test]
    pub fn test_update_index_and_hash() {
        let index0 = ExecutionIndices {
            sub_dag_index: 0,
            transaction_index: 5,
            last_committed_round: 0,
        };
        let index1 = ExecutionIndices {
            sub_dag_index: 1,
            transaction_index: 2,
            last_committed_round: 3,
        };

        let mut last_seen = ExecutionIndicesWithStats {
            index: index0,
            hash: 1000,
            stats: ConsensusStats::default(),
        };

        let tx = &[0];
        update_index_and_hash(&mut last_seen, index1, tx);
        assert_eq!(last_seen.index, index1);
        assert_ne!(last_seen.hash, 1000);
    }

    #[test]
    fn test_order_by_gas_price() {
        let mut v = vec![cap_txn(10), user_txn(42), user_txn(100), cap_txn(1)];
        PostConsensusTxReorder::reorder(&mut v, ConsensusTransactionOrdering::ByGasPrice);
        assert_eq!(
            extract(v),
            vec![
                "cap(10)".to_string(),
                "cap(1)".to_string(),
                "user(100)".to_string(),
                "user(42)".to_string(),
            ]
        );

        let mut v = vec![
            user_txn(1200),
            cap_txn(10),
            user_txn(12),
            user_txn(1000),
            user_txn(42),
            user_txn(100),
            cap_txn(1),
            user_txn(1000),
        ];
        PostConsensusTxReorder::reorder(&mut v, ConsensusTransactionOrdering::ByGasPrice);
        assert_eq!(
            extract(v),
            vec![
                "cap(10)".to_string(),
                "cap(1)".to_string(),
                "user(1200)".to_string(),
                "user(1000)".to_string(),
                "user(1000)".to_string(),
                "user(100)".to_string(),
                "user(42)".to_string(),
                "user(12)".to_string(),
            ]
        );

        // If there are no user transactions, the order should be preserved.
        let mut v = vec![
            cap_txn(10),
            eop_txn(12),
            eop_txn(10),
            cap_txn(1),
            eop_txn(11),
        ];
        PostConsensusTxReorder::reorder(&mut v, ConsensusTransactionOrdering::ByGasPrice);
        assert_eq!(
            extract(v),
            vec![
                "cap(10)".to_string(),
                "eop(12)".to_string(),
                "eop(10)".to_string(),
                "cap(1)".to_string(),
                "eop(11)".to_string(),
            ]
        );
    }

    fn extract(v: Vec<VerifiedSequencedConsensusTransaction>) -> Vec<String> {
        v.into_iter().map(extract_one).collect()
    }

    fn extract_one(t: VerifiedSequencedConsensusTransaction) -> String {
        match t.0.transaction {
            SequencedConsensusTransactionKind::External(ext) => match ext.kind {
                ConsensusTransactionKind::EndOfPublish(authority) => {
                    format!("eop({})", authority.0[0])
                }
                ConsensusTransactionKind::CapabilityNotification(cap) => {
                    format!("cap({})", cap.generation)
                }
                ConsensusTransactionKind::UserTransaction(txn) => {
                    format!("user({})", txn.transaction_data().gas_price())
                }
                _ => unreachable!(),
            },
            SequencedConsensusTransactionKind::System(_) => unreachable!(),
        }
    }

    fn eop_txn(a: u8) -> VerifiedSequencedConsensusTransaction {
        let mut authority = AuthorityName::default();
        authority.0[0] = a;
        txn(ConsensusTransactionKind::EndOfPublish(authority))
    }

    fn cap_txn(generation: u64) -> VerifiedSequencedConsensusTransaction {
        txn(ConsensusTransactionKind::CapabilityNotification(
            AuthorityCapabilitiesV1 {
                authority: Default::default(),
                generation,
                supported_protocol_versions: SupportedProtocolVersions::SYSTEM_DEFAULT,
                available_system_packages: vec![],
            },
        ))
    }

    fn user_txn(gas_price: u64) -> VerifiedSequencedConsensusTransaction {
        let (committee, keypairs) = Committee::new_simple_test_committee();
        let data = SenderSignedData::new(
            TransactionData::new_transfer(
                PeraAddress::default(),
                random_object_ref(),
                PeraAddress::default(),
                random_object_ref(),
                1000 * gas_price,
                gas_price,
            ),
            vec![],
        );
        txn(ConsensusTransactionKind::UserTransaction(Box::new(
            CertifiedTransaction::new_from_keypairs_for_testing(data, &keypairs, &committee),
        )))
    }

    fn txn(kind: ConsensusTransactionKind) -> VerifiedSequencedConsensusTransaction {
        VerifiedSequencedConsensusTransaction::new_test(ConsensusTransaction {
            kind,
            tracking_id: Default::default(),
        })
    }
}
