// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use arc_swap::ArcSwapOption;
use enum_dispatch::enum_dispatch;
use fastcrypto::groups::bls12381;
use fastcrypto_tbls::dkg_v1;
use fastcrypto_tbls::nodes::PartyId;
use fastcrypto_zkp::bn254::zk_login::{JwkId, OIDCProvider, JWK};
use fastcrypto_zkp::bn254::zk_login_api::ZkLoginEnv;
use futures::future::{join_all, select, Either};
use futures::FutureExt;
use ika_types::committee::Committee;
use ika_types::committee::CommitteeTrait;
use ika_types::crypto::{
    AuthorityName, AuthorityPublicKeyBytes, AuthoritySignInfo, AuthorityStrongQuorumSignInfo,
};
use ika_types::digests::ChainIdentifier;
use ika_types::error::{IkaError, IkaResult};
use itertools::{izip, Itertools};
use parking_lot::RwLock;
use parking_lot::{Mutex, RwLockReadGuard, RwLockWriteGuard};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use sui_macros::fail_point_arg;
use sui_types::accumulator::Accumulator;
use sui_types::authenticator_state::{get_authenticator_state, ActiveJwk};
use sui_types::base_types::{ConciseableName, ObjectRef};
use sui_types::base_types::{EpochId, ObjectID, SequenceNumber};
use sui_types::crypto::RandomnessRound;
use sui_types::signature::GenericSignature;
use sui_types::storage::{BackingPackageStore, InputKey, ObjectStore};
use sui_types::transaction::{
    AuthenticatorStateUpdate, CertifiedTransaction, InputObjectKind, SenderSignedData, Transaction,
    TransactionDataAPI, TransactionKey, TransactionKind, VerifiedCertificate,
    VerifiedSignedTransaction, VerifiedTransaction,
};
use tokio::sync::OnceCell;
use tracing::{debug, error, info, instrument, trace, warn};
use typed_store::rocks::{read_size_from_env, ReadWriteOptions};
use typed_store::rocksdb::Options;
use typed_store::{
    rocks::{default_db_options, DBBatch, DBMap, DBOptions, MetricConf},
    traits::{TableSummary, TypedStoreDebug},
    TypedStoreError,
};

use super::epoch_start_configuration::EpochStartConfigTrait;

use crate::authority::epoch_start_configuration::EpochStartConfiguration;
use crate::authority::AuthorityMetrics;
use crate::checkpoints::{
    BuilderCheckpointMessage, CheckpointHeight, CheckpointServiceNotify, EpochStats,
    PendingCheckpoint, PendingCheckpointInfo, PendingCheckpointV1,
};

use crate::authority::authority_perpetual_tables::AuthorityPerpetualTables;
use crate::consensus_handler::{
    ConsensusCommitInfo, SequencedConsensusTransaction, SequencedConsensusTransactionKey,
    SequencedConsensusTransactionKind, VerifiedSequencedConsensusTransaction,
};
use crate::dwallet_mpc::batches_manager::DWalletMPCBatchesManager;
use crate::dwallet_mpc::mpc_manager::{DWalletMPCDBMessage, DWalletMPCManager};
use crate::dwallet_mpc::mpc_outputs_verifier::{
    DWalletMPCOutputsVerifier, OutputResult, OutputVerificationResult,
};
use crate::dwallet_mpc::network_dkg::DwalletMPCNetworkKeyVersions;
use crate::dwallet_mpc::{
    authority_name_to_party_id, presign_first_public_input, session_info_from_event,
};
use crate::epoch::epoch_metrics::EpochMetrics;
use crate::epoch::reconfiguration::ReconfigState;
use crate::stake_aggregator::{GenericMultiStakeAggregator, StakeAggregator};
use dwallet_classgroups_types::ClassGroupsDecryptionKey;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCPublicOutput, NetworkDecryptionKeyShares,
};
use group::PartyID;
use ika_protocol_config::{Chain, ProtocolConfig, ProtocolVersion};
use ika_types::digests::MessageDigest;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::message::MessageKind;
use ika_types::message_envelope::TrustedEnvelope;
use ika_types::messages_checkpoint::{
    CheckpointMessage, CheckpointSequenceNumber, CheckpointSignatureMessage,
};
use ika_types::messages_consensus::{
    AuthorityCapabilitiesV1, ConsensusTransaction, ConsensusTransactionKey,
    ConsensusTransactionKind,
};
use ika_types::messages_consensus::{Round, TimestampMs};
use ika_types::messages_dwallet_mpc::{
    DWalletMPCEvent, DWalletMPCOutputMessage, MPCProtocolInitData, SessionInfo,
    StartPresignFirstRoundEvent,
};
use ika_types::sui::epoch_start_system::{EpochStartSystem, EpochStartSystemTrait};
use move_bytecode_utils::module_cache::SyncModuleCache;
use mpc::{Weight, WeightedThresholdAccessStructure};
use mysten_common::sync::notify_once::NotifyOnce;
use mysten_common::sync::notify_read::NotifyRead;
use mysten_metrics::monitored_scope;
use prometheus::IntCounter;
use std::str::FromStr;
use std::time::Duration;
use sui_macros::fail_point;
use sui_storage::mutex_table::{MutexGuard, MutexTable};
use sui_types::digests::TransactionDigest;
use sui_types::effects::TransactionEffects;
use sui_types::executable_transaction::{
    TrustedExecutableTransaction, VerifiedExecutableTransaction,
};
use sui_types::id::ID;
use sui_types::sui_system_state::epoch_start_sui_system_state::EpochStartSystemState;
use tap::TapOptional;
use tokio::time::Instant;
use typed_store::DBMapUtils;
use typed_store::{retry_transaction_forever, Map};

/// The key where the latest consensus index is stored in the database.
// TODO: Make a single table (e.g., called `variables`) storing all our lonely variables in one place.
const LAST_CONSENSUS_STATS_ADDR: u64 = 0;
const RECONFIG_STATE_INDEX: u64 = 0;
const OVERRIDE_PROTOCOL_UPGRADE_BUFFER_STAKE_INDEX: u64 = 0;
pub const EPOCH_DB_PREFIX: &str = "epoch_";

// Types for randomness DKG.
pub(crate) type PkG = bls12381::G2Element;
pub(crate) type EncG = bls12381::G2Element;

// CertLockGuard and CertTxGuard are functionally identical right now, but we retain a distinction
// anyway. If we need to support distributed object storage, having this distinction will be
// useful, as we will most likely have to re-implement a retry / write-ahead-log at that point.
pub struct CertLockGuard(#[allow(unused)] MutexGuard);
pub struct CertTxGuard(#[allow(unused)] CertLockGuard);

impl CertTxGuard {
    pub fn release(self) {}
    pub fn commit_tx(self) {}
    pub fn as_lock_guard(&self) -> &CertLockGuard {
        &self.0
    }
}

impl CertLockGuard {
    pub fn dummy_for_tests() -> Self {
        let lock = Arc::new(tokio::sync::Mutex::new(()));
        Self(lock.try_lock_owned().unwrap())
    }
}

pub enum CancelConsensusCertificateReason {
    CongestionOnObjects(Vec<ObjectID>),
    DkgFailed,
}

pub enum ConsensusCertificateResult {
    /// The consensus message was ignored (e.g. because it has already been processed).
    Ignored,
    /// An executable transaction (can be a user tx or a system tx)
    IkaTransaction(MessageKind),
    /// A message was processed which updates randomness state.
    RandomnessConsensusMessage,
    /// Everything else, e.g. AuthorityCapabilities, CheckpointSignatures, etc.
    ConsensusMessage,
    /// A system message in consensus was ignored (e.g. because of end of epoch).
    IgnoredSystem,
    // /// A will-be-cancelled transaction. It'll still go through execution engine (but not be executed),
    // /// unlock any owned objects, and return corresponding cancellation error according to
    // /// `CancelConsensusCertificateReason`.
    // Cancelled(
    //     (
    //         VerifiedExecutableTransaction,
    //         CancelConsensusCertificateReason,
    //     ),
    // ),
}

/// ConsensusStats is versioned because we may iterate on the struct, and it is
/// stored on disk.
#[enum_dispatch]
pub trait ConsensusStatsAPI {
    fn is_initialized(&self) -> bool;

    fn get_num_messages(&self, authority: usize) -> u64;
    fn inc_num_messages(&mut self, authority: usize) -> u64;

    fn get_num_user_transactions(&self, authority: usize) -> u64;
    fn inc_num_user_transactions(&mut self, authority: usize) -> u64;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[enum_dispatch(ConsensusStatsAPI)]
pub enum ConsensusStats {
    V1(ConsensusStatsV1),
}

impl ConsensusStats {
    pub fn new(size: usize) -> Self {
        Self::V1(ConsensusStatsV1 {
            num_messages: vec![0; size],
            num_user_transactions: vec![0; size],
        })
    }
}

impl Default for ConsensusStats {
    fn default() -> Self {
        Self::new(0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConsensusStatsV1 {
    pub num_messages: Vec<u64>,
    pub num_user_transactions: Vec<u64>,
}

impl ConsensusStatsAPI for ConsensusStatsV1 {
    fn is_initialized(&self) -> bool {
        !self.num_messages.is_empty()
    }

    fn get_num_messages(&self, authority: usize) -> u64 {
        self.num_messages[authority]
    }

    fn inc_num_messages(&mut self, authority: usize) -> u64 {
        self.num_messages[authority] += 1;
        self.num_messages[authority]
    }

    fn get_num_user_transactions(&self, authority: usize) -> u64 {
        self.num_user_transactions[authority]
    }

    fn inc_num_user_transactions(&mut self, authority: usize) -> u64 {
        self.num_user_transactions[authority] += 1;
        self.num_user_transactions[authority]
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq, Copy)]
pub struct ExecutionIndices {
    /// The round number of the last committed leader.
    pub last_committed_round: u64,
    /// The index of the last sub-DAG that was executed (either fully or partially).
    pub sub_dag_index: u64,
    /// The index of the last transaction was executed (used for crash-recovery).
    pub transaction_index: u64,
}

impl Ord for ExecutionIndices {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (
            self.last_committed_round,
            self.sub_dag_index,
            self.transaction_index,
        )
            .cmp(&(
                other.last_committed_round,
                other.sub_dag_index,
                other.transaction_index,
            ))
    }
}

impl PartialOrd for ExecutionIndices {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct ExecutionIndicesWithStats {
    pub index: ExecutionIndices,
    // Hash is always 0 and kept for compatibility only.
    pub hash: u64,
    pub stats: ConsensusStats,
}

pub struct AuthorityPerEpochStore {
    /// The name of this authority.
    pub(crate) name: AuthorityName,

    /// Committee of validators for the current epoch.
    committee: Arc<Committee>,

    /// Holds the underlying per-epoch typed store tables.
    /// This is an ArcSwapOption because it needs to be used concurrently,
    /// and it needs to be cleared at the end of the epoch.
    tables: ArcSwapOption<AuthorityEpochTables>,

    protocol_config: ProtocolConfig,

    // needed for re-opening epoch db.
    parent_path: PathBuf,
    db_options: Option<Options>,

    /// In-memory cache of the content from the reconfig_state db table.
    reconfig_state_mem: RwLock<ReconfigState>,
    consensus_notify_read: NotifyRead<SequencedConsensusTransactionKey, ()>,

    // Subscribers will get notified when a transaction is executed via checkpoint execution.
    executed_transactions_to_checkpoint_notify_read:
        NotifyRead<MessageDigest, CheckpointSequenceNumber>,

    executed_digests_notify_read: NotifyRead<TransactionKey, MessageDigest>,

    /// Get notified when a synced checkpoint has reached CheckpointExecutor.
    synced_checkpoint_notify_read: NotifyRead<CheckpointSequenceNumber, ()>,
    /// Caches the highest synced checkpoint sequence number as this has been notified from the CheckpointExecutor
    highest_synced_checkpoint: RwLock<CheckpointSequenceNumber>,

    /// This is used to notify all epoch specific tasks that epoch has ended.
    epoch_alive_notify: NotifyOnce,

    /// Used to notify all epoch specific tasks that user certs are closed.
    user_certs_closed_notify: NotifyOnce,

    /// This lock acts as a barrier for tasks that should not be executed in parallel with reconfiguration
    /// See comments in AuthorityPerEpochStore::epoch_terminated() on how this is used
    /// Crash recovery note: we write next epoch in the database first, and then use this lock to
    /// wait for in-memory tasks for the epoch to finish. If node crashes at this stage validator
    /// will start with the new epoch(and will open instance of per-epoch store for a new epoch).
    epoch_alive: tokio::sync::RwLock<bool>,
    initiate_process_mid_epoch: Mutex<StakeAggregator<(), true>>,
    end_of_publish: Mutex<StakeAggregator<(), true>>,

    /// MutexTable for transaction locks (prevent concurrent execution of same transaction)
    mutex_table: MutexTable<MessageDigest>,

    /// The moment when the current epoch started locally on this validator. Note that this
    /// value could be skewed if the node crashed and restarted in the middle of the epoch. That's
    /// ok because this is used for metric purposes and we could tolerate some skews occasionally.
    pub(crate) epoch_open_time: Instant,

    /// The moment when epoch is reach mid round. We don't care much about crash recovery because it's
    /// a metric that doesn't have to be available for each epoch.
    mid_epoch_time: RwLock<Option<Instant>>,

    /// The moment when epoch is closed. We don't care much about crash recovery because it's
    /// a metric that doesn't have to be available for each epoch, and it's only used during
    /// the last few seconds of an epoch.
    epoch_close_time: RwLock<Option<Instant>>,
    pub(crate) metrics: Arc<EpochMetrics>,
    epoch_start_configuration: Arc<EpochStartConfiguration>,

    executed_in_epoch_table_enabled: once_cell::sync::OnceCell<bool>,

    /// Chain identifier
    chain_identifier: ChainIdentifier,

    /// State machine managing dWallet MPC outputs.
    /// This state machine is used to store outputs and emit ones
    /// where the quorum of votes is valid.
    dwallet_mpc_outputs_verifier: OnceCell<tokio::sync::Mutex<DWalletMPCOutputsVerifier>>,
    dwallet_mpc_batches_manager: OnceCell<tokio::sync::Mutex<DWalletMPCBatchesManager>>,
    pub dwallet_mpc_network_keys: OnceCell<DwalletMPCNetworkKeyVersions>,
    dwallet_mpc_round_messages: tokio::sync::Mutex<Vec<DWalletMPCDBMessage>>,
    dwallet_mpc_round_outputs: tokio::sync::Mutex<Vec<DWalletMPCOutputMessage>>,
    dwallet_mpc_round_events: tokio::sync::Mutex<Vec<DWalletMPCEvent>>,
    dwallet_mpc_round_completed_sessions: tokio::sync::Mutex<Vec<ObjectID>>,
    dwallet_mpc_manager: OnceCell<tokio::sync::Mutex<DWalletMPCManager>>,
    perpetual_tables: Arc<AuthorityPerpetualTables>,
}

/// AuthorityEpochTables contains tables that contain data that is only valid within an epoch.
#[derive(DBMapUtils)]
pub struct AuthorityEpochTables {
    /// Transactions that were executed in the current epoch.
    executed_in_epoch: DBMap<MessageDigest, ()>,

    /// Certificates that have been received from clients or received from consensus, but not yet
    /// executed. Entries are cleared after execution.
    /// This table is critical for crash recovery, because usually the consensus output progress
    /// is updated after a certificate is committed into this table.
    ///
    /// In theory, this table may be superseded by storing consensus and checkpoint execution
    /// progress. But it is more complex, because it would be necessary to track inflight
    /// executions not ordered by indices. For now, tracking inflight certificates as a map
    /// seems easier.
    #[default_options_override_fn = "pending_execution_table_default_config"]
    pub(crate) pending_execution: DBMap<MessageDigest, TrustedExecutableTransaction>,

    /// Track which transactions have been processed in handle_consensus_transaction. We must be
    /// sure to advance next_shared_object_versions exactly once for each transaction we receive from
    /// consensus. But, we may also be processing transactions from checkpoints, so we need to
    /// track this state separately.
    ///
    /// Entries in this table can be garbage collected whenever we can prove that we won't receive
    /// another handle_consensus_transaction call for the given digest. This probably means at
    /// epoch change.
    consensus_message_processed: DBMap<SequencedConsensusTransactionKey, bool>,

    /// Map stores pending transactions that this authority submitted to consensus
    #[default_options_override_fn = "pending_consensus_transactions_table_default_config"]
    pending_consensus_transactions: DBMap<ConsensusTransactionKey, ConsensusTransaction>,

    /// The following table is used to store a single value (the corresponding key is a constant). The value
    /// represents the index of the latest consensus message this authority processed, running hash of
    /// transactions, and accumulated stats of consensus output.
    /// This field is written by a single process (consensus handler).
    last_consensus_stats: DBMap<u64, ExecutionIndicesWithStats>,

    /// This table contains current reconfiguration state for validator for current epoch
    reconfig_state: DBMap<u64, ReconfigState>,

    /// Validators that have sent InitiateProcessMidEpoch message in this epoch
    initiate_process_mid_epoch: DBMap<AuthorityName, ()>,

    /// Validators that have sent EndOfPublish message in this epoch
    end_of_publish: DBMap<AuthorityName, ()>,

    /// This table has information for the checkpoints for which we constructed all the data
    /// from consensus, but not yet constructed actual checkpoint.
    ///
    /// Key in this table is the consensus commit height and not a checkpoint sequence number.
    ///
    /// Non-empty list of transactions here might result in empty list when we are forming checkpoint.
    /// Because we don't want to create checkpoints with empty content(see CheckpointBuilder::write_checkpoint),
    /// the sequence number of checkpoint does not match height here.
    #[default_options_override_fn = "pending_checkpoints_table_default_config"]
    pending_checkpoints: DBMap<CheckpointHeight, PendingCheckpoint>,

    /// Maps non-digest TransactionKeys to the corresponding digest after execution, for use
    /// by checkpoint builder.
    transaction_key_to_digest: DBMap<TransactionKey, MessageDigest>,

    /// Stores pending signatures
    /// The key in this table is checkpoint sequence number and an arbitrary integer
    pending_checkpoint_signatures:
        DBMap<(CheckpointSequenceNumber, u64), CheckpointSignatureMessage>,

    /// Maps sequence number to checkpoint summary, used by CheckpointBuilder to build checkpoint within epoch
    builder_checkpoint_message_v1: DBMap<CheckpointSequenceNumber, BuilderCheckpointMessage>,
    /// Record of the capabilities advertised by each authority.
    authority_capabilities_v1: DBMap<AuthorityName, AuthorityCapabilitiesV1>,

    /// Contains a single key, which overrides the value of
    /// ProtocolConfig::buffer_stake_for_protocol_upgrade_bps
    override_protocol_upgrade_buffer_stake: DBMap<u64, u64>,

    /// When transaction is executed via checkpoint executor, we store association here
    pub(crate) executed_transactions_to_checkpoint: DBMap<MessageDigest, CheckpointSequenceNumber>,

    /// Holds all the DWallet MPC related messages that have been
    /// received since the beginning of the epoch.
    /// The key is the consensus round number,
    /// the value is the dWallet-mpc messages that have been received in that
    /// round.
    pub(crate) dwallet_mpc_messages: DBMap<u64, Vec<DWalletMPCDBMessage>>,
    pub(crate) dwallet_mpc_outputs: DBMap<u64, Vec<DWalletMPCOutputMessage>>,
    // TODO (#538): change type to the inner, basic type instead of using Sui's wrapper
    // pub struct SessionID([u8; AccountAddress::LENGTH]);
    pub(crate) dwallet_mpc_completed_sessions: DBMap<u64, Vec<ObjectID>>,
    pub(crate) dwallet_mpc_events: DBMap<u64, Vec<DWalletMPCEvent>>,
}

fn signed_transactions_table_default_config() -> DBOptions {
    default_db_options()
        .optimize_for_write_throughput()
        .optimize_for_large_values_no_scan(1 << 10)
}

fn pending_execution_table_default_config() -> DBOptions {
    default_db_options()
        .optimize_for_write_throughput()
        .optimize_for_large_values_no_scan(1 << 10)
}

fn pending_consensus_transactions_table_default_config() -> DBOptions {
    default_db_options()
        .optimize_for_write_throughput()
        .optimize_for_large_values_no_scan(1 << 10)
}

fn pending_checkpoints_table_default_config() -> DBOptions {
    default_db_options()
        .optimize_for_write_throughput()
        .optimize_for_large_values_no_scan(1 << 10)
}

impl AuthorityEpochTables {
    pub fn open(epoch: EpochId, parent_path: &Path, db_options: Option<Options>) -> Self {
        Self::open_tables_transactional(
            Self::path(epoch, parent_path),
            MetricConf::new("epoch"),
            db_options,
            None,
        )
    }

    pub fn open_readonly(epoch: EpochId, parent_path: &Path) -> AuthorityEpochTablesReadOnly {
        Self::get_read_only_handle(
            Self::path(epoch, parent_path),
            None,
            None,
            MetricConf::new("epoch_readonly"),
        )
    }

    pub fn path(epoch: EpochId, parent_path: &Path) -> PathBuf {
        parent_path.join(format!("{}{}", EPOCH_DB_PREFIX, epoch))
    }

    fn load_reconfig_state(&self) -> IkaResult<ReconfigState> {
        let state = self
            .reconfig_state
            .get(&RECONFIG_STATE_INDEX)?
            .unwrap_or_default();
        Ok(state)
    }

    pub fn get_all_pending_consensus_transactions(&self) -> Vec<ConsensusTransaction> {
        self.pending_consensus_transactions
            .unbounded_iter()
            .map(|(_k, v)| v)
            .collect()
    }

    pub fn reset_db_for_execution_since_genesis(&self) -> IkaResult {
        // TODO: Add new tables that get added to the db automatically
        self.executed_transactions_to_checkpoint.unsafe_clear()?;
        Ok(())
    }

    /// WARNING: This method is very subtle and can corrupt the database if used incorrectly.
    /// It should only be used in one-off cases or tests after fully understanding the risk.
    pub fn remove_executed_tx_subtle(&self, digest: &MessageDigest) -> IkaResult {
        self.executed_transactions_to_checkpoint.remove(digest)?;
        Ok(())
    }

    pub fn get_last_consensus_index(&self) -> IkaResult<Option<ExecutionIndices>> {
        Ok(self
            .last_consensus_stats
            .get(&LAST_CONSENSUS_STATS_ADDR)?
            .map(|s| s.index))
    }

    pub fn get_last_consensus_stats(&self) -> IkaResult<Option<ExecutionIndicesWithStats>> {
        Ok(self.last_consensus_stats.get(&LAST_CONSENSUS_STATS_ADDR)?)
    }

    pub fn get_pending_checkpoint_signatures_iter(
        &self,
        checkpoint_seq: CheckpointSequenceNumber,
        starting_index: u64,
    ) -> IkaResult<
        impl Iterator<Item = ((CheckpointSequenceNumber, u64), CheckpointSignatureMessage)> + '_,
    > {
        let key = (checkpoint_seq, starting_index);
        debug!("Scanning pending checkpoint signatures from {:?}", key);
        let iter = self
            .pending_checkpoint_signatures
            .unbounded_iter()
            .skip_to(&key)?;
        Ok::<_, IkaError>(iter)
    }
}

pub(crate) const MUTEX_TABLE_SIZE: usize = 1024;

impl AuthorityPerEpochStore {
    #[instrument(name = "AuthorityPerEpochStore::new", level = "error", skip_all, fields(epoch = committee.epoch))]
    pub fn new(
        name: AuthorityName,
        committee: Arc<Committee>,
        parent_path: &Path,
        db_options: Option<Options>,
        metrics: Arc<EpochMetrics>,
        epoch_start_configuration: EpochStartConfiguration,
        chain_identifier: ChainIdentifier,
        perpetual_tables: Arc<AuthorityPerpetualTables>,
    ) -> Arc<Self> {
        let current_time = Instant::now();
        let epoch_id = committee.epoch;

        let tables = AuthorityEpochTables::open(epoch_id, parent_path, db_options.clone());
        let initiate_process_mid_epoch = StakeAggregator::from_iter(
            committee.clone(),
            tables.initiate_process_mid_epoch.unbounded_iter(),
        );
        let end_of_publish =
            StakeAggregator::from_iter(committee.clone(), tables.end_of_publish.unbounded_iter());
        let reconfig_state = tables
            .load_reconfig_state()
            .expect("Load reconfig state at initialization cannot fail");

        let epoch_alive_notify = NotifyOnce::new();
        assert_eq!(
            epoch_start_configuration.epoch_start_state().epoch(),
            epoch_id
        );
        let epoch_start_configuration = Arc::new(epoch_start_configuration);
        metrics.current_epoch.set(epoch_id as i64);
        metrics
            .current_voting_right
            .set(committee.weight(&name) as i64);
        let protocol_version = epoch_start_configuration
            .epoch_start_state()
            .protocol_version();
        // let protocol_config =
        //     ProtocolConfig::get_for_version(protocol_version, chain_identifier.chain());
        let protocol_config = ProtocolConfig::get_for_version(protocol_version, Chain::Mainnet);

        let s = Arc::new(Self {
            name,
            committee,
            protocol_config,
            tables: ArcSwapOption::new(Some(Arc::new(tables))),
            parent_path: parent_path.to_path_buf(),
            db_options,
            reconfig_state_mem: RwLock::new(reconfig_state),
            epoch_alive_notify,
            user_certs_closed_notify: NotifyOnce::new(),
            epoch_alive: tokio::sync::RwLock::new(true),
            consensus_notify_read: NotifyRead::new(),
            executed_transactions_to_checkpoint_notify_read: NotifyRead::new(),
            executed_digests_notify_read: NotifyRead::new(),
            synced_checkpoint_notify_read: NotifyRead::new(),
            highest_synced_checkpoint: RwLock::new(0),
            initiate_process_mid_epoch: Mutex::new(initiate_process_mid_epoch),
            end_of_publish: Mutex::new(end_of_publish),
            mutex_table: MutexTable::new(MUTEX_TABLE_SIZE),
            epoch_open_time: current_time,
            mid_epoch_time: Default::default(),
            epoch_close_time: Default::default(),
            metrics,
            epoch_start_configuration,
            executed_in_epoch_table_enabled: once_cell::sync::OnceCell::new(),
            chain_identifier,
            dwallet_mpc_outputs_verifier: OnceCell::new(),
            dwallet_mpc_batches_manager: OnceCell::new(),
            dwallet_mpc_round_messages: tokio::sync::Mutex::new(Vec::new()),
            dwallet_mpc_round_outputs: tokio::sync::Mutex::new(Vec::new()),
            dwallet_mpc_round_events: tokio::sync::Mutex::new(Vec::new()),
            dwallet_mpc_round_completed_sessions: tokio::sync::Mutex::new(Vec::new()),
            dwallet_mpc_manager: OnceCell::new(),
            dwallet_mpc_network_keys: OnceCell::new(),
            perpetual_tables,
        });

        s.update_buffer_stake_metric();
        s
    }

    /// Saves a DWallet MPC message in the `round messages`.
    /// The `round messages` are later being stored to the on-disk DB to allow state sync.
    pub(crate) async fn save_dwallet_mpc_round_message(&self, message: DWalletMPCDBMessage) {
        let mut dwallet_mpc_round_messages = self.dwallet_mpc_round_messages.lock().await;
        dwallet_mpc_round_messages.push(message.clone());
    }

    /// Saves a DWallet MPC output in the round messages
    /// The round outputs are later being stored to the on-disk DB to allow state sync.
    pub(crate) async fn save_dwallet_mpc_output(&self, output: DWalletMPCOutputMessage) {
        let mut dwallet_mpc_round_outputs = self.dwallet_mpc_round_outputs.lock().await;
        dwallet_mpc_round_outputs.push(output.clone());
    }

    /// Loads the DWallet MPC events from the given mystecity round.
    pub(crate) async fn load_dwallet_mpc_events_from_round(
        &self,
        round: Round,
    ) -> IkaResult<Vec<DWalletMPCEvent>> {
        Ok(self
            .tables()?
            .dwallet_mpc_events
            .iter_with_bounds(Some(round), None)
            .map(|(_, events)| events)
            .flatten()
            .collect())
    }

    /// Loads the DWallet MPC completed sessions from the given mystecity round.
    pub(crate) async fn load_dwallet_mpc_completed_sessions_from_round(
        &self,
        round: Round,
    ) -> IkaResult<Vec<ObjectID>> {
        Ok(self
            .tables()?
            .dwallet_mpc_completed_sessions
            .iter_with_bounds(Some(round), None)
            .map(|(_, events)| events)
            .flatten()
            .collect())
    }

    /// Saves a DWallet MPC event in the round events
    /// The round events are later being stored to the on-disk DB to allow state sync.
    pub(crate) async fn save_dwallet_mpc_event(&self, event: DWalletMPCEvent) {
        let mut dwallet_mpc_round_outputs = self.dwallet_mpc_round_events.lock().await;
        dwallet_mpc_round_outputs.push(event);
    }

    /// Saves a DWallet MPC completed session in the round completed sessions
    /// The round completed sessions are later being stored to the on-disk DB to allow state sync.
    pub(crate) async fn save_dwallet_mpc_completed_session(&self, session_id: ObjectID) {
        let mut dwallet_mpc_round_completed_sessions =
            self.dwallet_mpc_round_completed_sessions.lock().await;
        dwallet_mpc_round_completed_sessions.push(session_id);
    }

    /// A function to initiate the [`DWalletMPCManager`] when a new epoch starts.
    pub fn set_dwallet_mpc_manager(&self, sender: DWalletMPCManager) -> IkaResult<()> {
        if self
            .dwallet_mpc_manager
            .set(tokio::sync::Mutex::new(sender))
            .is_err()
        {
            error!("AuthorityPerEpochStore: `set_dwallet_mpc_batches_manager` called more than once; this should never happen");
        }
        Ok(())
    }

    /// A function to initiate the [`DWalletMPCBatchesManager`] when a new epoch starts.
    pub fn set_dwallet_mpc_batches_manager(
        &self,
        batches_manager: DWalletMPCBatchesManager,
    ) -> IkaResult<()> {
        if self
            .dwallet_mpc_batches_manager
            .set(tokio::sync::Mutex::new(batches_manager))
            .is_err()
        {
            error!("AuthorityPerEpochStore: `set_dwallet_mpc_batches_manager` called more than once; this should never happen");
        }
        Ok(())
    }

    /// A function to initiate the [`DWalletMPCOutputsVerifier`] when a new epoch starts.
    /// This outputs verifier handles storing all the outputs of dWallet MPC session,
    /// and writes them to the chain once all the outputs are ready and verified.
    pub fn set_dwallet_mpc_outputs_verifier(
        &self,
        verifier: DWalletMPCOutputsVerifier,
    ) -> IkaResult<()> {
        if self
            .dwallet_mpc_outputs_verifier
            .set(tokio::sync::Mutex::new(verifier))
            .is_err()
        {
            error!(
                "AuthorityPerEpochStore: `set_dwallet_mpc_outputs_verifier` called more than once; this should never happen"
            );
        }
        Ok(())
    }

    pub fn get_weighted_threshold_access_structure(
        &self,
    ) -> DwalletMPCResult<WeightedThresholdAccessStructure> {
        let quorum_threshold = self.committee().quorum_threshold();
        let weighted_parties: HashMap<PartyID, Weight> = self
            .committee()
            .voting_rights
            .iter()
            .map(|(name, weight)| Ok((authority_name_to_party_id(name, &self)?, *weight as Weight)))
            .collect::<DwalletMPCResult<HashMap<PartyID, Weight>>>()?;

        WeightedThresholdAccessStructure::new(quorum_threshold as PartyID, weighted_parties)
            .map_err(|e| DwalletMPCError::TwoPCMPCError(e.to_string()))
    }

    /// A function to initiate the network keys `state` for the dWallet MPC when a new epoch starts.
    pub fn set_dwallet_mpc_network_keys(
        &self,
        class_groups_decryption_key: ClassGroupsDecryptionKey,
    ) -> IkaResult<()> {
        if self
            .dwallet_mpc_network_keys
            .set(DwalletMPCNetworkKeyVersions::new(
                self,
                &self.get_weighted_threshold_access_structure()?,
                class_groups_decryption_key,
            ))
            .is_err()
        {
            error!("AuthorityPerEpochStore: `set_dwallet_mpc_network_keys` called more than once; this should never happen");
        }
        Ok(())
    }

    /// Retrieves the decryption key shares for the current epoch if they exist in the system state.
    ///
    /// The data is sourced from the epoch's initial system state.
    /// The returned value is a map where:
    /// - The key represents the key scheme (e.g., Secp256k1, Ristretto, etc.).
    /// - The value is a vector of [`NetworkDecryptionKeyShares`],
    ///   which contains all versions of the encrypted decryption key shares.
    pub(crate) fn load_decryption_key_shares_from_system_state(
        &self,
    ) -> DwalletMPCResult<HashMap<DWalletMPCNetworkKeyScheme, Vec<NetworkDecryptionKeyShares>>>
    {
        // let decryption_key_shares = match self.epoch_start_state() {
        //     EpochStartSystem::V1(data) => data.get_decryption_key_shares(),
        // }
        //     .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
        //     .contents
        //     .into_iter()
        //     .map(|entry| {
        //         Ok((
        //             DWalletMPCNetworkKeyScheme::try_from(entry.key)?,
        //             entry.value,
        //         ))
        //     })
        //     .collect::<DwalletMPCResult<HashMap<_, _>>>()?;
        //
        // Ok(decryption_key_shares)
        Ok(HashMap::new())
    }

    /// Retrieves the *running validator's* latest decryption key shares for each key scheme
    /// if they exist in the system state.
    ///
    /// The data is sourced from the epoch's initial system state.
    /// The returned value is a map where:
    /// - The key represents the key scheme.
    /// - The value is a `Vec<u8>`, containing the decryption key shares for the validator.
    pub(crate) fn load_validator_decryption_key_shares_from_system_state(
        &self,
    ) -> DwalletMPCResult<HashMap<DWalletMPCNetworkKeyScheme, Vec<Vec<u8>>>> {
        let decryption_key_shares = self.load_decryption_key_shares_from_system_state()?;
        decryption_key_shares
            .into_iter()
            .map(|(key_type, encryption_shares)| {
                let shares = encryption_shares
                    .iter()
                    .map(|share| {
                        // TODO (#382): Decrypt the decryption key share
                        Vec::new()
                    })
                    .collect();
                Ok((key_type, shares))
            })
            .collect()
    }

    /// Return the [`DWalletMPCOutputsVerifier`].
    /// Uses a Mutex because the instance is initialized from a different thread.
    pub async fn get_dwallet_mpc_outputs_verifier(
        &self,
    ) -> tokio::sync::MutexGuard<DWalletMPCOutputsVerifier> {
        loop {
            match self.dwallet_mpc_outputs_verifier.get() {
                Some(dwallet_mpc_outputs_verifier) => {
                    return dwallet_mpc_outputs_verifier.lock().await
                }
                None => {
                    error!("failed to get the DWalletMPCOutputsVerifier, retrying...");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    /// Return the current epoch's [`DWalletMPCBatchesManager`].
    /// This manager handles storing all the valid outputs of a batched dWallet MPC sessions,
    /// and writes them to the chain at once when all the batch outputs are ready.
    pub async fn get_dwallet_mpc_batches_manager(
        &self,
    ) -> tokio::sync::MutexGuard<DWalletMPCBatchesManager> {
        loop {
            match self.dwallet_mpc_batches_manager.get() {
                Some(dwallet_mpc_batches_manager) => {
                    return dwallet_mpc_batches_manager.lock().await
                }
                None => {
                    error!("failed to get the DWallet Batches Manager, retrying...");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    /// Return the current epoch's [`DWalletMPCManager`].
    pub async fn get_dwallet_mpc_manager(&self) -> tokio::sync::MutexGuard<DWalletMPCManager> {
        loop {
            match self.dwallet_mpc_manager.get() {
                Some(dwallet_mpc_manager) => return dwallet_mpc_manager.lock().await,
                None => {
                    error!("failed to get the DWallet MPC Manager, retrying...");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    pub fn tables(&self) -> IkaResult<Arc<AuthorityEpochTables>> {
        match self.tables.load_full() {
            Some(tables) => Ok(tables),
            None => Err(IkaError::EpochEnded(self.epoch())),
        }
    }

    // Ideally the epoch tables handle should have the same lifetime as the outer AuthorityPerEpochStore,
    // and this function should be unnecessary. But unfortunately, Arc<AuthorityPerEpochStore> outlives the
    // epoch significantly right now, so we need to manually release the tables to release its memory usage.
    pub fn release_db_handles(&self) {
        // When the logic to release DB handles becomes obsolete, it may still be useful
        // to make sure AuthorityEpochTables is not used after the next epoch starts.
        self.tables.store(None);
    }

    pub fn get_parent_path(&self) -> PathBuf {
        self.parent_path.clone()
    }

    /// Returns `&Arc<EpochStartConfiguration>`
    /// User can treat this `Arc` as `&EpochStartConfiguration`, or clone the Arc to pass as owned object
    pub fn epoch_start_config(&self) -> &Arc<EpochStartConfiguration> {
        &self.epoch_start_configuration
    }

    pub fn epoch_start_state(&self) -> &EpochStartSystem {
        self.epoch_start_configuration.epoch_start_state()
    }

    pub fn get_chain_identifier(&self) -> ChainIdentifier {
        self.chain_identifier
    }

    pub fn new_at_next_epoch(
        &self,
        name: AuthorityName,
        new_committee: Committee,
        epoch_start_configuration: EpochStartConfiguration,
        chain_identifier: ChainIdentifier,
        perpetual_tables: Arc<AuthorityPerpetualTables>,
    ) -> Arc<Self> {
        assert_eq!(self.epoch() + 1, new_committee.epoch);
        self.record_reconfig_halt_duration_metric();
        self.record_epoch_total_duration_metric();
        Self::new(
            name,
            Arc::new(new_committee),
            &self.parent_path,
            self.db_options.clone(),
            self.metrics.clone(),
            epoch_start_configuration,
            chain_identifier,
            perpetual_tables,
        )
    }

    pub fn new_at_next_epoch_for_testing(&self) -> Arc<Self> {
        let perpetual_tables_options = default_db_options().optimize_db_for_write_throughput(4);
        let perpetual_tables = Arc::new(AuthorityPerpetualTables::open(
            &self.parent_path.join("store"),
            Some(perpetual_tables_options.options),
        ));
        let next_epoch = self.epoch() + 1;
        let next_committee = Committee::new(
            next_epoch,
            self.committee.voting_rights.iter().cloned().collect(),
        );
        self.new_at_next_epoch(
            self.name,
            next_committee,
            self.epoch_start_configuration
                .new_at_next_epoch_for_testing(),
            self.chain_identifier,
            perpetual_tables,
        )
    }

    pub fn committee(&self) -> &Arc<Committee> {
        &self.committee
    }

    pub fn protocol_config(&self) -> &ProtocolConfig {
        &self.protocol_config
    }

    pub fn epoch(&self) -> EpochId {
        self.committee.epoch
    }

    pub fn protocol_version(&self) -> ProtocolVersion {
        self.epoch_start_state().protocol_version()
    }

    pub fn store_reconfig_state(&self, new_state: &ReconfigState) -> IkaResult {
        self.tables()?
            .reconfig_state
            .insert(&RECONFIG_STATE_INDEX, new_state)?;
        Ok(())
    }

    pub fn get_last_consensus_stats(&self) -> IkaResult<ExecutionIndicesWithStats> {
        match self
            .tables()?
            .get_last_consensus_stats()
            .map_err(IkaError::from)?
        {
            Some(stats) => Ok(stats),
            None => {
                let indices = self
                    .tables()?
                    .get_last_consensus_index()
                    .map(|x| x.unwrap_or_default())
                    .map_err(IkaError::from)?;
                Ok(ExecutionIndicesWithStats {
                    index: indices,
                    hash: 0, // unused
                    stats: ConsensusStats::default(),
                })
            }
        }
    }

    /// `pending_certificates` table related methods. Should only be used from TransactionManager.

    /// Gets all pending certificates. Used during recovery.
    pub fn all_pending_execution(&self) -> IkaResult<Vec<VerifiedExecutableTransaction>> {
        Ok(self
            .tables()?
            .pending_execution
            .unbounded_iter()
            .map(|(_, cert)| cert.into())
            .collect())
    }

    /// Called when transaction outputs are committed to disk
    #[instrument(level = "trace", skip_all)]
    pub fn handle_committed_transactions(&self, digests: &[MessageDigest]) -> IkaResult<()> {
        let tables = match self.tables() {
            Ok(tables) => tables,
            // After Epoch ends, it is no longer necessary to remove pending transactions
            // because the table will not be used anymore and be deleted eventually.
            Err(IkaError::EpochEnded(_)) => return Ok(()),
            Err(e) => return Err(e),
        };
        let mut batch = tables.pending_execution.batch();
        // pending_execution stores transactions received from consensus which may not have
        // been executed yet. At this point, they have been committed to the db durably and
        // can be removed.
        // After end-to-end quarantining, we will not need pending_execution since the consensus
        // log itself will be used for recovery.
        batch.delete_batch(&tables.pending_execution, digests)?;

        batch.write()?;
        Ok(())
    }

    pub fn get_all_pending_consensus_transactions(&self) -> Vec<ConsensusTransaction> {
        self.tables()
            .expect("recovery should not cross epoch boundary")
            .get_all_pending_consensus_transactions()
    }

    /// Returns true if all messages with the given keys were processed by consensus.
    pub fn all_external_consensus_messages_processed(
        &self,
        keys: impl Iterator<Item = ConsensusTransactionKey>,
    ) -> IkaResult<bool> {
        let keys = keys.map(SequencedConsensusTransactionKey::External);
        Ok(self
            .check_consensus_messages_processed(keys)?
            .into_iter()
            .all(|processed| processed))
    }

    pub fn is_consensus_message_processed(
        &self,
        key: &SequencedConsensusTransactionKey,
    ) -> IkaResult<bool> {
        Ok(self
            .tables()?
            .consensus_message_processed
            .contains_key(key)?)
    }

    pub fn check_consensus_messages_processed(
        &self,
        keys: impl Iterator<Item = SequencedConsensusTransactionKey>,
    ) -> IkaResult<Vec<bool>> {
        Ok(self
            .tables()?
            .consensus_message_processed
            .multi_contains_keys(keys)?)
    }

    pub async fn consensus_messages_processed_notify(
        &self,
        keys: Vec<SequencedConsensusTransactionKey>,
    ) -> Result<(), IkaError> {
        let registrations = self.consensus_notify_read.register_all(&keys);

        let unprocessed_keys_registrations = registrations
            .into_iter()
            .zip(self.check_consensus_messages_processed(keys.into_iter())?)
            .filter(|(_, processed)| !processed)
            .map(|(registration, _)| registration);

        join_all(unprocessed_keys_registrations).await;
        Ok(())
    }

    pub fn has_sent_end_of_publish(&self, authority: &AuthorityName) -> IkaResult<bool> {
        Ok(self
            .end_of_publish
            .try_lock()
            .expect("No contention on end_of_publish lock")
            .contains_key(authority))
    }

    pub fn clear_override_protocol_upgrade_buffer_stake(&self) -> IkaResult {
        warn!(
            epoch = ?self.epoch(),
            "clearing buffer_stake_for_protocol_upgrade_bps override"
        );
        self.tables()?
            .override_protocol_upgrade_buffer_stake
            .remove(&OVERRIDE_PROTOCOL_UPGRADE_BUFFER_STAKE_INDEX)?;
        self.update_buffer_stake_metric();
        Ok(())
    }

    pub fn set_override_protocol_upgrade_buffer_stake(&self, new_stake_bps: u64) -> IkaResult {
        warn!(
            ?new_stake_bps,
            epoch = ?self.epoch(),
            "storing buffer_stake_for_protocol_upgrade_bps override"
        );
        self.tables()?
            .override_protocol_upgrade_buffer_stake
            .insert(
                &OVERRIDE_PROTOCOL_UPGRADE_BUFFER_STAKE_INDEX,
                &new_stake_bps,
            )?;
        self.update_buffer_stake_metric();
        Ok(())
    }

    fn update_buffer_stake_metric(&self) {
        self.metrics
            .effective_buffer_stake
            .set(self.get_effective_buffer_stake_bps() as i64);
    }

    pub fn get_effective_buffer_stake_bps(&self) -> u64 {
        self.tables()
            .expect("epoch initialization should have finished")
            .override_protocol_upgrade_buffer_stake
            .get(&OVERRIDE_PROTOCOL_UPGRADE_BUFFER_STAKE_INDEX)
            .expect("force_protocol_upgrade read cannot fail")
            .tap_some(|b| warn!("using overridden buffer stake value of {}", b))
            .unwrap_or_else(|| {
                self.protocol_config()
                    .buffer_stake_for_protocol_upgrade_bps()
            })
    }

    /// Record most recently advertised capabilities of all authorities
    pub fn record_capabilities_v1(&self, capabilities: &AuthorityCapabilitiesV1) -> IkaResult {
        info!("received capabilities v1 {:?}", capabilities);
        let authority = &capabilities.authority;
        let tables = self.tables()?;

        // Read-compare-write pattern assumes we are only called from the consensus handler task.
        if let Some(cap) = tables.authority_capabilities_v1.get(authority)? {
            if cap.generation >= capabilities.generation {
                debug!(
                    "ignoring new capabilities {:?} in favor of previous capabilities {:?}",
                    capabilities, cap
                );
                return Ok(());
            }
        }
        tables
            .authority_capabilities_v1
            .insert(authority, capabilities)?;
        Ok(())
    }

    pub fn get_capabilities_v1(&self) -> IkaResult<Vec<AuthorityCapabilitiesV1>> {
        let result: Result<Vec<AuthorityCapabilitiesV1>, TypedStoreError> = self
            .tables()?
            .authority_capabilities_v1
            .values()
            .map_into()
            .collect();
        Ok(result?)
    }

    // fn finish_consensus_certificate_process_with_batch(
    //     &self,
    //     output: &mut ConsensusCommitOutput,
    //     certificates: &[VerifiedExecutableTransaction],
    // ) -> IkaResult {
    //     output.insert_pending_execution(certificates);
    //
    //     if cfg!(debug_assertions) {
    //         for certificate in certificates {
    //             // User signatures are written in the same batch as consensus certificate processed flag,
    //             // which means we won't attempt to insert this twice for the same tx digest
    //             assert!(!self
    //                 .tables()?
    //                 .user_signatures_for_checkpoints
    //                 .contains_key(certificate.digest())
    //                 .unwrap());
    //         }
    //     }
    //     Ok(())
    // }

    pub fn get_reconfig_state_read_lock_guard(&self) -> RwLockReadGuard<ReconfigState> {
        self.reconfig_state_mem.read()
    }

    pub fn get_reconfig_state_write_lock_guard(&self) -> RwLockWriteGuard<ReconfigState> {
        self.reconfig_state_mem.write()
    }

    pub fn update_mid_epoch_time(&self) {
        // Set mid_epoch_time for metric purpose.
        let mut mid_epoch_time = self.mid_epoch_time.write();
        if mid_epoch_time.is_none() {
            // Only update it the first time epoch is closed.
            *mid_epoch_time = Some(Instant::now());
        }
    }

    pub fn close_user_certs(&self, mut lock_guard: RwLockWriteGuard<'_, ReconfigState>) {
        lock_guard.close_user_certs();
        self.store_reconfig_state(&lock_guard)
            .expect("Updating reconfig state cannot fail");

        // Set epoch_close_time for metric purpose.
        let mut epoch_close_time = self.epoch_close_time.write();
        if epoch_close_time.is_none() {
            // Only update it the first time epoch is closed.
            *epoch_close_time = Some(Instant::now());

            self.user_certs_closed_notify
                .notify()
                .expect("user_certs_closed_notify called twice on same epoch store");
        }
    }

    pub async fn user_certs_closed_notify(&self) {
        self.user_certs_closed_notify.wait().await
    }

    /// Notify epoch is terminated, can only be called once on epoch store
    pub async fn epoch_terminated(&self) {
        // Notify interested tasks that epoch has ended
        self.epoch_alive_notify
            .notify()
            .expect("epoch_terminated called twice on same epoch store");
        // This `write` acts as a barrier - it waits for futures executing in
        // `within_alive_epoch` to terminate before we can continue here
        debug!("Epoch terminated - waiting for pending tasks to complete");
        *self.epoch_alive.write().await = false;
        debug!("All pending epoch tasks completed");
    }

    /// Waits for the notification about epoch termination
    pub async fn wait_epoch_terminated(&self) {
        self.epoch_alive_notify.wait().await
    }

    /// This function executes given future until epoch_terminated is called
    /// If future finishes before epoch_terminated is called, future result is returned
    /// If epoch_terminated is called before future is resolved, error is returned
    ///
    /// In addition to the early termination guarantee, this function also prevents epoch_terminated()
    /// if future is being executed.
    #[allow(clippy::result_unit_err)]
    pub async fn within_alive_epoch<F: Future + Send>(&self, f: F) -> Result<F::Output, ()> {
        // This guard is kept in the future until it resolves, preventing `epoch_terminated` to
        // acquire a write lock
        let guard = self.epoch_alive.read().await;
        if !*guard {
            return Err(());
        }
        let terminated = self.wait_epoch_terminated().boxed();
        let f = f.boxed();
        match select(terminated, f).await {
            Either::Left((_, _f)) => Err(()),
            Either::Right((result, _)) => Ok(result),
        }
    }

    /// Verifies transaction signatures and other data
    /// Important: This function can potentially be called in parallel and you can not rely on order of transactions to perform verification
    /// If this function return an error, transaction is skipped and is not passed to handle_consensus_transaction
    /// This function returns unit error and is responsible for emitting log messages for internal errors
    fn verify_consensus_transaction(
        &self,
        transaction: SequencedConsensusTransaction,
        skipped_consensus_txns: &IntCounter,
    ) -> Option<VerifiedSequencedConsensusTransaction> {
        let _scope = monitored_scope("VerifyConsensusTransaction");
        if self
            .is_consensus_message_processed(&transaction.transaction.key())
            .expect("Storage error")
        {
            debug!(
                consensus_index=?transaction.consensus_index.transaction_index,
                tracking_id=?transaction.transaction.get_tracking_id(),
                "handle_consensus_transaction UserTransaction [skip]",
            );
            skipped_consensus_txns.inc();
            return None;
        }
        // Signatures are verified as part of the consensus payload verification in IkaTxValidator
        match &transaction.transaction {
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::DWalletMPCSessionFailedWithMalicious(authority, ..),
                ..
            }) => {
                // When sending a `DWalletMPCSessionFailedWithMalicious`,
                // the validator also includes its public key.
                // Here, we verify that the public key used to sign this transaction matches
                // the provided public key.
                // This public key is later used to identify the authority that sent the MPC message.
                if transaction.sender_authority() != *authority {
                    warn!(
                        "DWalletMPCSessionFailedWithMalicious: authority {} does not match its author from consensus {}",
                        authority, transaction.certificate_author_index
                    );
                    return None;
                }
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::DWalletMPCOutput(authority, _, _),
                ..
            }) => {
                // When sending an MPC output, the validator also includes its public key.
                // Here, we verify that the public key used to sign this transaction matches
                // the provided public key.
                // This public key is later used to identify the authority that sent the MPC message.
                if transaction.sender_authority() != *authority {
                    warn!(
                        "DWalletMPCOutput authority {} does not match its author from consensus {}",
                        authority, transaction.certificate_author_index
                    );
                    return None;
                }
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::DWalletMPCMessage(message),
                ..
            }) => {
                // When sending an MPC message, the validator also includes its public key.
                // Here, we verify that the public key used to sign this transaction matches
                // the provided public key.
                // This public key is later used
                // to identify the authority that sent the MPC message.
                if transaction.sender_authority() != message.authority {
                    warn!(
                        "DWalletMPCMessage authority {} does not match its author from consensus {}",
                        message.authority, transaction.certificate_author_index
                    );
                    return None;
                }
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::CheckpointSignature(data),
                ..
            }) => {
                if transaction.sender_authority() != data.checkpoint_message.auth_sig().authority {
                    warn!(
                        "CheckpointSignature authority {} does not match its author from consensus {}",
                        data.checkpoint_message.auth_sig().authority,
                        transaction.certificate_author_index
                    );
                    return None;
                }
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::InitiateProcessMidEpoch(authority),
                ..
            }) => {
                if &transaction.sender_authority() != authority {
                    warn!(
                        "InitiateProcessMidEpoch authority {} does not match its author from consensus {}",
                        authority, transaction.certificate_author_index
                    );
                    return None;
                }
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::EndOfPublish(authority),
                ..
            }) => {
                if &transaction.sender_authority() != authority {
                    warn!(
                        "EndOfPublish authority {} does not match its author from consensus {}",
                        authority, transaction.certificate_author_index
                    );
                    return None;
                }
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind:
                    ConsensusTransactionKind::CapabilityNotificationV1(AuthorityCapabilitiesV1 {
                        authority,
                        ..
                    }),
                ..
            }) => {
                if transaction.sender_authority() != *authority {
                    warn!(
                        "CapabilityNotification authority {} does not match its author from consensus {}",
                        authority, transaction.certificate_author_index
                    );
                    return None;
                }
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::TestMessage(authority, _),
                ..
            }) => {
                if transaction.sender_authority() != *authority {
                    warn!(
                        "TestMessage authority {} does not match its author from consensus {}",
                        authority, transaction.certificate_author_index
                    );
                    return None;
                }
            }
            SequencedConsensusTransactionKind::System(_) => {}
        }
        Some(VerifiedSequencedConsensusTransaction(transaction))
    }

    fn db_batch(&self) -> IkaResult<DBBatch> {
        Ok(self.tables()?.last_consensus_stats.batch())
    }

    #[cfg(test)]
    pub fn db_batch_for_test(&self) -> DBBatch {
        self.db_batch()
            .expect("test should not be write past end of epoch")
    }

    #[instrument(level = "debug", skip_all)]
    pub(crate) async fn process_consensus_transactions_and_commit_boundary<
        'a,
        C: CheckpointServiceNotify,
    >(
        &self,
        transactions: Vec<SequencedConsensusTransaction>,
        consensus_stats: &ExecutionIndicesWithStats,
        checkpoint_service: &Arc<C>,
        consensus_commit_info: &ConsensusCommitInfo,
        authority_metrics: &Arc<AuthorityMetrics>,
    ) -> IkaResult<Vec<MessageKind>> {
        // Split transactions into different types for processing.
        let verified_transactions: Vec<_> = transactions
            .into_iter()
            .filter_map(|transaction| {
                self.verify_consensus_transaction(
                    transaction,
                    &authority_metrics.skipped_consensus_txns,
                )
            })
            .collect();
        let mut system_transactions = Vec::with_capacity(verified_transactions.len());
        let mut current_commit_sequenced_consensus_transactions =
            Vec::with_capacity(verified_transactions.len());

        let mut initiate_process_mid_epoch_transactions =
            Vec::with_capacity(verified_transactions.len());
        let mut end_of_publish_transactions = Vec::with_capacity(verified_transactions.len());
        for tx in verified_transactions {
            if tx.0.is_initiate_process_mid_epoch() {
                initiate_process_mid_epoch_transactions.push(tx);
            } else if tx.0.is_end_of_publish() {
                end_of_publish_transactions.push(tx);
            } else if tx.0.is_system() {
                system_transactions.push(tx);
            } else {
                current_commit_sequenced_consensus_transactions.push(tx);
            }
        }

        let mut output = ConsensusCommitOutput::new(consensus_commit_info.round);

        // Sequenced_transactions stores all transactions that will be sent to
        // process_consensus_transactions.
        let mut sequenced_transactions: Vec<VerifiedSequencedConsensusTransaction> =
            Vec::with_capacity(current_commit_sequenced_consensus_transactions.len());

        sequenced_transactions.extend(current_commit_sequenced_consensus_transactions);

        let consensus_transactions: Vec<_> = system_transactions
            .into_iter()
            .chain(sequenced_transactions)
            .collect();

        let (mut verified_messages, notifications, lock, mid_epoch_round, final_round) = self
            .process_consensus_transactions(
                &mut output,
                &consensus_transactions,
                &initiate_process_mid_epoch_transactions,
                &end_of_publish_transactions,
                checkpoint_service,
                consensus_commit_info,
                //&mut roots,
                authority_metrics,
            )
            .await?;
        //self.finish_consensus_certificate_process_with_batch(&mut output, &verified_transactions)?;
        output.record_consensus_commit_stats(consensus_stats.clone());

        // Create pending checkpoints if we are still accepting tx.
        let should_accept_tx = if let Some(lock) = &lock {
            lock.should_accept_tx()
        } else {
            // It is ok to just release lock here as functions called by this one are the
            // only place that transition reconfig state, and this function itself is always
            // executed from consensus task. At this point if the lock was not already provided
            // above, we know we won't be transitioning state for this commit.
            self.get_reconfig_state_read_lock_guard().should_accept_tx()
        };
        let make_checkpoint = should_accept_tx || final_round;
        if make_checkpoint {
            let checkpoint_height = consensus_commit_info.round;

            let pending_checkpoint = PendingCheckpoint::V1(PendingCheckpointV1 {
                messages: verified_messages.clone(),
                details: PendingCheckpointInfo {
                    timestamp_ms: consensus_commit_info.timestamp,
                    mid_of_epoch: mid_epoch_round,
                    last_of_epoch: final_round,
                    checkpoint_height,
                },
            });
            self.write_pending_checkpoint(&mut output, &pending_checkpoint)?;
        }

        let mut batch = self.db_batch()?;
        output.write_to_batch(self, &mut batch)?;
        batch.write()?;

        // Only after batch is written, notify checkpoint service to start building any new
        // pending checkpoints.
        if make_checkpoint {
            debug!(
                ?consensus_commit_info.round,
                "Notifying checkpoint service about new pending checkpoint(s)",
            );
            checkpoint_service.notify_checkpoint()?;
        }

        self.process_notifications(&notifications, &end_of_publish_transactions);

        if mid_epoch_round {
            info!(
                epoch=?self.epoch(),
                mid_epoch_round=?mid_epoch_round,
                "Notified last checkpoint"
            );
            self.record_initiate_process_mid_epoch_quorum_time_metric();
        }

        if final_round {
            info!(
                epoch=?self.epoch(),
                // Accessing lock on purpose so that the compiler ensures
                // the lock is not yet dropped.
                lock=?lock.as_ref(),
                final_round=?final_round,
                "Notified last checkpoint"
            );
            self.record_end_of_message_quorum_time_metric();
        }

        Ok(verified_messages)
    }

    // Caller is not required to set ExecutionIndices with the right semantics in
    // VerifiedSequencedConsensusTransaction.
    // Also, ConsensusStats and hash will not be updated in the db with this function, unlike in
    // process_consensus_transactions_and_commit_boundary().
    #[cfg(any(test, feature = "test-utils"))]
    pub async fn process_consensus_transactions_for_tests<C: CheckpointServiceNotify>(
        self: &Arc<Self>,
        transactions: Vec<SequencedConsensusTransaction>,
        checkpoint_service: &Arc<C>,
        authority_metrics: &Arc<AuthorityMetrics>,
        skip_consensus_commit_prologue_in_test: bool,
    ) -> IkaResult<Vec<VerifiedExecutableTransaction>> {
        self.process_consensus_transactions_and_commit_boundary(
            transactions,
            &ExecutionIndicesWithStats::default(),
            &ConsensusCommitInfo::new_for_test(
                // if self.randomness_state_enabled() {
                //     self.get_highest_pending_checkpoint_height() / 2 + 1
                // } else {
                //     self.get_highest_pending_checkpoint_height() + 1
                // },
                self.get_highest_pending_checkpoint_height() + 1,
                0,
                skip_consensus_commit_prologue_in_test,
            ),
            authority_metrics,
        )
        .await
    }

    fn process_notifications(
        &self,
        notifications: &[SequencedConsensusTransactionKey],
        end_of_publish: &[VerifiedSequencedConsensusTransaction],
    ) {
        for key in notifications
            .iter()
            .cloned()
            .chain(end_of_publish.iter().map(|tx| tx.0.transaction.key()))
        {
            self.consensus_notify_read.notify(&key, &());
        }
    }

    /// Depending on the type of the VerifiedSequencedConsensusTransaction wrappers,
    /// - Verify and initialize the state to execute the certificates.
    ///   Return VerifiedCertificates for each executable certificate
    /// - Or update the state for checkpoint or epoch change protocol.
    #[instrument(level = "debug", skip_all)]
    #[allow(clippy::type_complexity)]
    pub(crate) async fn process_consensus_transactions<C: CheckpointServiceNotify>(
        &self,
        output: &mut ConsensusCommitOutput,
        transactions: &[VerifiedSequencedConsensusTransaction],
        initiate_process_mid_epoch_transactions: &[VerifiedSequencedConsensusTransaction],
        end_of_publish_transactions: &[VerifiedSequencedConsensusTransaction],
        checkpoint_service: &Arc<C>,
        consensus_commit_info: &ConsensusCommitInfo,
        //roots: &mut BTreeSet<MessageDigest>,
        authority_metrics: &Arc<AuthorityMetrics>,
    ) -> IkaResult<(
        Vec<MessageKind>,                      // transactions to schedule
        Vec<SequencedConsensusTransactionKey>, // keys to notify as complete
        Option<RwLockWriteGuard<ReconfigState>>,
        bool, // true if mid-epoch round
        bool, // true if final round
    )> {
        let _scope = monitored_scope("ConsensusCommitHandler::process_consensus_transactions");

        let mut verified_certificates = VecDeque::with_capacity(transactions.len() + 1);
        let mut notifications = Vec::with_capacity(transactions.len());

        let mut cancelled_txns: BTreeMap<MessageDigest, CancelConsensusCertificateReason> =
            BTreeMap::new();

        for tx in transactions {
            let key = tx.0.transaction.key();
            let mut ignored = false;
            let mut filter_roots = false;
            match self
                .process_consensus_transaction(
                    output,
                    tx,
                    checkpoint_service,
                    consensus_commit_info.round,
                    authority_metrics,
                )
                .await?
            {
                ConsensusCertificateResult::IkaTransaction(cert) => {
                    notifications.push(key.clone());
                    verified_certificates.push_back(cert);
                }
                // ConsensusCertificateResult::Cancelled((cert, reason)) => {
                //     notifications.push(key.clone());
                //     assert!(cancelled_txns.insert(*cert.digest(), reason).is_none());
                //     verified_certificates.push_back(cert);
                // }
                ConsensusCertificateResult::RandomnessConsensusMessage => {
                    //randomness_state_updated = true;
                    notifications.push(key.clone());
                }
                ConsensusCertificateResult::ConsensusMessage => notifications.push(key.clone()),
                ConsensusCertificateResult::IgnoredSystem => {
                    filter_roots = true;
                }
                // Note: ignored external transactions must not be recorded as processed. Otherwise
                // they may not get reverted after restart during epoch change.
                ConsensusCertificateResult::Ignored => {
                    ignored = true;
                    filter_roots = true;
                }
            }
            if !ignored {
                output.record_consensus_message_processed(key.clone());
            }
        }

        self.save_dwallet_mpc_round_message(DWalletMPCDBMessage::EndOfDelivery)
            .await;

        // Save all the dWallet-MPC related DB data to the consensus commit output to
        // write it to the local DB. After saving the data, clear the data from the epoch store.
        let mut dwallet_mpc_round_messages = self.dwallet_mpc_round_messages.lock().await;
        output.set_dwallet_mpc_round_messages(dwallet_mpc_round_messages.clone());
        dwallet_mpc_round_messages.clear();
        let mut dwallet_mpc_round_outputs = self.dwallet_mpc_round_outputs.lock().await;
        output.set_dwallet_mpc_round_outputs(dwallet_mpc_round_outputs.clone());
        dwallet_mpc_round_outputs.clear();
        let mut dwallet_mpc_round_completed_sessions =
            self.dwallet_mpc_round_completed_sessions.lock().await;
        output
            .set_dwallet_mpc_round_completed_sessions(dwallet_mpc_round_completed_sessions.clone());
        dwallet_mpc_round_completed_sessions.clear();

        let key_version = self
            .dwallet_mpc_network_keys
            .get()
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .key_version(DWalletMPCNetworkKeyScheme::Secp256k1)
            .unwrap_or_default();
        let pending_events = self.perpetual_tables.get_all_pending_events();
        let party_id = authority_name_to_party_id(&self.name, &self)?;
        let dwallet_mpc_new_events = pending_events
            .iter()
            .map(|(_, event)| {
                let session_info =
                    session_info_from_event(event.clone(), party_id, Some(key_version))
                        .map_err(|e| DwalletMPCError::NonMPCEvent(e.to_string()))?
                        .ok_or(DwalletMPCError::NonMPCEvent(
                            "Failed to craete session info from event".to_string(),
                        ))?;
                Ok(DWalletMPCEvent {
                    event: event.clone(),
                    session_info,
                })
            })
            .collect::<DwalletMPCResult<_>>()?;
        output.set_dwallet_mpc_round_events(dwallet_mpc_new_events);
        let pending_event_ids = pending_events.keys().cloned().collect::<Vec<_>>();
        self.perpetual_tables
            .remove_pending_events(&pending_event_ids)?;

        authority_metrics
            .consensus_handler_cancelled_transactions
            .inc_by(cancelled_txns.len() as u64);

        let verified_certificates: Vec<_> = verified_certificates.into();

        let mid_epoch_round = self.process_initiate_process_mid_epoch_transactions(
            output,
            initiate_process_mid_epoch_transactions,
        )?;

        let (lock, final_round) = self.process_end_of_publish_transactions_and_reconfig(
            output,
            end_of_publish_transactions,
        )?;

        Ok((
            verified_certificates,
            notifications,
            lock,
            mid_epoch_round,
            final_round,
        ))
    }

    fn process_initiate_process_mid_epoch_transactions(
        &self,
        output: &mut ConsensusCommitOutput,
        transactions: &[VerifiedSequencedConsensusTransaction],
    ) -> IkaResult<
        bool, // true if mid-epoch round
    > {
        for transaction in transactions {
            let VerifiedSequencedConsensusTransaction(SequencedConsensusTransaction {
                transaction,
                ..
            }) = transaction;

            if let SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::InitiateProcessMidEpoch(authority),
                ..
            }) = transaction
            {
                debug!(
                    "Received InitiateProcessMidEpoch for epoch {} from {:?}",
                    self.committee.epoch,
                    authority.concise()
                );

                let collected_initiate_process_mid_epoch = {
                    let mut initiate_process_mid_epoch_stake = self.initiate_process_mid_epoch.try_lock()
                        .expect("No contention on Authority::initiate_process_mid_epoch as it is only accessed from consensus handler");
                    if !initiate_process_mid_epoch_stake.has_quorum() {
                        output.insert_initiate_process_mid_epoch(*authority);
                        initiate_process_mid_epoch_stake
                            .insert_generic(*authority, ())
                            .is_quorum_reached()
                    } else {
                        // If we past the stage where we are accepting consensus certificates we also don't record initiate process mid-epoch message messages
                        debug!("Ignoring initiate process mid-epoch message from validator {:?} as we already collected enough initiate process mid-epoch message messages", authority.concise());
                        false
                    }
                    // initiate_process_mid_epoch lock is released here.
                };

                // Important: we actually rely here on fact that ConsensusHandler panics if its
                // operation returns error. If some day we won't panic in ConsensusHandler on error
                // we need to figure out here how to revert in-memory state of .initiate_process_mid_epoch
                // when write fails.
                output.record_consensus_message_processed(transaction.key());
                if collected_initiate_process_mid_epoch {
                    debug!(
                        "Collected enough initiate_process_mid_epoch messages for epoch {} with last message from validator {:?}",
                        self.committee.epoch,
                        authority.concise(),
                    );
                    return Ok(true);
                }
            } else {
                panic!(
                    "process_initiate_process_mid_epoch called with non-initiate-process-mid-epoch transaction"
                );
            }
        }
        Ok(false)
    }

    fn process_end_of_publish_transactions_and_reconfig(
        &self,
        output: &mut ConsensusCommitOutput,
        transactions: &[VerifiedSequencedConsensusTransaction],
    ) -> IkaResult<(
        Option<RwLockWriteGuard<ReconfigState>>,
        bool, // true if final round
    )> {
        let mut lock = None;

        for transaction in transactions {
            let VerifiedSequencedConsensusTransaction(SequencedConsensusTransaction {
                transaction,
                ..
            }) = transaction;

            if let SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::EndOfPublish(authority),
                ..
            }) = transaction
            {
                debug!(
                    "Received EndOfPublish for epoch {} from {:?}",
                    self.committee.epoch,
                    authority.concise()
                );

                // It is ok to just release lock here as this function is the only place that transition into RejectAllCerts state
                // And this function itself is always executed from consensus task
                let collected_end_of_publish = if lock.is_none()
                    && self
                        .get_reconfig_state_read_lock_guard()
                        .should_accept_consensus_certs()
                {
                    output.insert_end_of_publish(*authority);
                    self.end_of_publish.try_lock()
                        .expect("No contention on Authority::end_of_publish as it is only accessed from consensus handler")
                        .insert_generic(*authority, ()).is_quorum_reached()
                    // end_of_publish lock is released here.
                } else {
                    // If we past the stage where we are accepting consensus certificates we also don't record end of publish messages
                    debug!("Ignoring end of publish message from validator {:?} as we already collected enough end of publish messages", authority.concise());
                    false
                };

                if collected_end_of_publish {
                    assert!(lock.is_none());
                    debug!(
                        "Collected enough end_of_publish messages for epoch {} with last message from validator {:?}",
                        self.committee.epoch,
                        authority.concise(),
                    );
                    let mut l = self.get_reconfig_state_write_lock_guard();
                    l.close_all_certs();
                    output.store_reconfig_state(l.clone());
                    // Holding this lock until end of process_consensus_transactions_and_commit_boundary() where we write batch to DB
                    lock = Some(l);
                };
                // Important: we actually rely here on fact that ConsensusHandler panics if its
                // operation returns error. If some day we won't panic in ConsensusHandler on error
                // we need to figure out here how to revert in-memory state of .end_of_publish
                // and .reconfig_state when write fails.
                output.record_consensus_message_processed(transaction.key());
            } else {
                panic!(
                    "process_end_of_publish_transactions_and_reconfig called with non-end-of-publish transaction"
                );
            }
        }

        // Determine if we're ready to advance reconfig state to RejectAllTx.
        let is_reject_all_certs = if let Some(lock) = &lock {
            lock.is_reject_all_certs()
        } else {
            // It is ok to just release lock here as this function is the only place that
            // transitions into RejectAllTx state, and this function itself is always
            // executed from consensus task.
            self.get_reconfig_state_read_lock_guard()
                .is_reject_all_certs()
        };

        if !is_reject_all_certs {
            return Ok((lock, false));
        }

        // Acquire lock to advance state if we don't already have it.
        let mut lock = lock.unwrap_or_else(|| self.get_reconfig_state_write_lock_guard());
        lock.close_all_tx();
        output.store_reconfig_state(lock.clone());
        Ok((Some(lock), true))
    }

    #[instrument(level = "trace", skip_all)]
    async fn process_consensus_transaction<C: CheckpointServiceNotify>(
        &self,
        output: &mut ConsensusCommitOutput,
        transaction: &VerifiedSequencedConsensusTransaction,
        checkpoint_service: &Arc<C>,
        commit_round: Round,
        authority_metrics: &Arc<AuthorityMetrics>,
    ) -> IkaResult<ConsensusCertificateResult> {
        let _scope = monitored_scope("ConsensusCommitHandler::process_consensus_transaction");

        let VerifiedSequencedConsensusTransaction(SequencedConsensusTransaction {
            certificate_author_index: _,
            certificate_author,
            consensus_index,
            transaction,
        }) = transaction;
        let tracking_id = transaction.get_tracking_id();

        match &transaction {
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::DWalletMPCOutput(_, session_info, output),
                ..
            }) => {
                self.process_dwallet_mpc_output(
                    certificate_author.clone(),
                    session_info.clone(),
                    output.clone(),
                )
                .await
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind:
                    ConsensusTransactionKind::DWalletMPCSessionFailedWithMalicious(
                        authority_name,
                        report,
                    ),
                ..
            }) => {
                self.save_dwallet_mpc_round_message(
                    DWalletMPCDBMessage::SessionFailedWithMaliciousParties(
                        authority_name.clone(),
                        report.clone(),
                    ),
                )
                .await;
                Ok(ConsensusCertificateResult::ConsensusMessage)
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::DWalletMPCMessage(message),
                ..
            }) => {
                // Filter DWalletMPCMessages from the consensus output and save them in the local
                // DB.
                // Those messages will get processed when the dWallet MPC service reads
                // them from the DB.
                self.save_dwallet_mpc_round_message(DWalletMPCDBMessage::Message(message.clone()))
                    .await;
                Ok(ConsensusCertificateResult::ConsensusMessage)
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::CheckpointSignature(info),
                ..
            }) => {
                // We usually call notify_checkpoint_signature in IkaTxValidator, but that step can
                // be skipped when a batch is already part of a certificate, so we must also
                // notify here.
                checkpoint_service.notify_checkpoint_signature(self, info)?;
                Ok(ConsensusCertificateResult::ConsensusMessage)
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::InitiateProcessMidEpoch(_),
                ..
            }) => {
                // these are partitioned earlier
                panic!("process_consensus_transaction called with initiate-process-mid-epoch transaction");
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::EndOfPublish(_),
                ..
            }) => {
                // these are partitioned earlier
                panic!("process_consensus_transaction called with end-of-publish transaction");
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::CapabilityNotificationV1(capabilities),
                ..
            }) => {
                let authority = capabilities.authority;
                if self
                    .get_reconfig_state_read_lock_guard()
                    .should_accept_consensus_certs()
                {
                    debug!(
                        "Received CapabilityNotificationV2 from {:?}",
                        authority.concise()
                    );
                    self.record_capabilities_v1(capabilities)?;
                } else {
                    debug!(
                        "Ignoring CapabilityNotificationV2 from {:?} because of end of epoch",
                        authority.concise()
                    );
                }
                Ok(ConsensusCertificateResult::ConsensusMessage)
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::TestMessage(authority, num),
                ..
            }) => Ok(
                self.process_consensus_system_transaction(&MessageKind::TestMessage(
                    self.committee.authority_index(&authority).unwrap(),
                    *num,
                )),
            ),
            SequencedConsensusTransactionKind::System(system_transaction) => {
                Ok(self.process_consensus_system_transaction(system_transaction))
            }
        }
    }

    async fn process_dwallet_mpc_output(
        &self,
        origin_authority: AuthorityName,
        session_info: SessionInfo,
        output: Vec<u8>,
    ) -> IkaResult<ConsensusCertificateResult> {
        self.save_dwallet_mpc_output(DWalletMPCOutputMessage {
            output: output.clone(),
            authority: origin_authority.clone(),
            session_info: session_info.clone(),
        })
        .await;

        let authority_index = authority_name_to_party_id(&origin_authority, &self);
        let mut dwallet_mpc_verifier = self.get_dwallet_mpc_outputs_verifier().await;
        let output_verification_result = dwallet_mpc_verifier
            .try_verify_output(&output, &session_info, origin_authority)
            .await
            .unwrap_or_else(|e| {
                error!("error verifying DWalletMPCOutput output from session {:?} and party {:?}: {:?}",session_info.session_id, authority_index, e);
                OutputVerificationResult {
                    result: OutputResult::Malicious,
                    malicious_actors: vec![origin_authority],
                }
            });

        let mut manager = self.get_dwallet_mpc_manager().await;
        manager.flag_authorities_as_malicious(&output_verification_result.malicious_actors);

        match output_verification_result.result {
            OutputResult::FirstQuorumReached => {
                self.save_dwallet_mpc_completed_session(session_info.session_id)
                    .await;
                // Output result of a single Protocol from the batch session.
                if session_info.mpc_round.is_part_of_batch() {
                    let mut batches_manager = self.get_dwallet_mpc_batches_manager().await;
                    batches_manager.store_verified_output(session_info.clone(), output.clone())?;
                    batches_manager.is_batch_completed(&session_info)?;
                    Ok(
                        self.process_consensus_system_transaction(&MessageKind::DwalletMPCOutput(
                            session_info,
                            output,
                        )),
                    )
                } else {
                    // Extract the final network DKG transaction parameters from
                    // the verified output.
                    // We can't preform this within the execution engine,
                    // as it requires the class-groups crate from crypto-private lib.
                    if let MPCProtocolInitData::NetworkDkg(key_scheme, _) = session_info.mpc_round {
                        let weighted_threshold_access_structure =
                            self.get_weighted_threshold_access_structure()?;

                        let key = crate::dwallet_mpc::network_dkg::dwallet_mpc_network_key_from_session_output(
                            self.epoch(),
                            key_scheme,
                            &weighted_threshold_access_structure,
                            &output,
                        )?;

                        Ok(self.process_consensus_system_transaction(
                            &MessageKind::DwalletMPCNetworkDKGOutput(key_scheme, key),
                        ))
                    } else {
                        Ok(self.process_consensus_system_transaction(
                            &MessageKind::DwalletMPCOutput(session_info, output),
                        ))
                    }
                }
            }
            OutputResult::NotEnoughVotes => Ok(ConsensusCertificateResult::ConsensusMessage),
            OutputResult::AlreadyCommitted | OutputResult::Malicious => {
                // Ignore this output,
                // since there is nothing to do with it,
                // at this stage.
                Ok(ConsensusCertificateResult::IgnoredSystem)
            }
        }
    }

    fn process_consensus_system_transaction(
        &self,
        system_transaction: &MessageKind,
    ) -> ConsensusCertificateResult {
        if !self.get_reconfig_state_read_lock_guard().should_accept_tx() {
            debug!(
                "Ignoring system transaction {:?} because of end of epoch",
                system_transaction.digest()
            );
            return ConsensusCertificateResult::IgnoredSystem;
        }

        ConsensusCertificateResult::IkaTransaction(system_transaction.clone())
    }

    pub(crate) fn write_pending_checkpoint(
        &self,
        output: &mut ConsensusCommitOutput,
        checkpoint: &PendingCheckpoint,
    ) -> IkaResult {
        assert!(
            self.get_pending_checkpoint(&checkpoint.height())?.is_none(),
            "Duplicate pending checkpoint notification at height {:?}",
            checkpoint.height()
        );

        debug!(
            checkpoint_commit_height = checkpoint.height(),
            "Pending checkpoint has {} messages",
            checkpoint.messages().len(),
        );
        trace!(
            checkpoint_commit_height = checkpoint.height(),
            "Messages for pending checkpoint: {:?}",
            checkpoint.messages()
        );

        output.insert_pending_checkpoint(checkpoint.clone());

        Ok(())
    }

    pub fn get_pending_checkpoints(
        &self,
        last: Option<CheckpointHeight>,
    ) -> IkaResult<Vec<(CheckpointHeight, PendingCheckpoint)>> {
        let tables = self.tables()?;
        let mut iter = tables.pending_checkpoints.unbounded_iter();
        if let Some(last_processed_height) = last {
            iter = iter.skip_to(&(last_processed_height + 1))?;
        }
        Ok(iter.collect())
    }

    pub fn get_pending_checkpoint(
        &self,
        index: &CheckpointHeight,
    ) -> IkaResult<Option<PendingCheckpoint>> {
        Ok(self.tables()?.pending_checkpoints.get(index)?)
    }

    pub fn process_pending_checkpoint(
        &self,
        commit_height: CheckpointHeight,
        checkpoint_messages: Vec<CheckpointMessage>,
    ) -> IkaResult<()> {
        let tables = self.tables()?;
        // All created checkpoints are inserted in builder_checkpoint_summary in a single batch.
        // This means that upon restart we can use BuilderCheckpointSummary::commit_height
        // from the last built summary to resume building checkpoints.
        let mut batch = tables.pending_checkpoints.batch();
        for (position_in_commit, summary) in checkpoint_messages.into_iter().enumerate() {
            let sequence_number = summary.sequence_number;
            let summary = BuilderCheckpointMessage {
                checkpoint_message: summary,
                checkpoint_height: Some(commit_height),
                position_in_commit,
            };
            batch.insert_batch(
                &tables.builder_checkpoint_message_v1,
                [(&sequence_number, summary)],
            )?;
        }

        // find all pending checkpoints <= commit_height and remove them
        let iter = tables
            .pending_checkpoints
            .safe_range_iter(0..=commit_height);
        let keys = iter
            .map(|c| c.map(|(h, _)| h))
            .collect::<Result<Vec<_>, _>>()?;

        batch.delete_batch(&tables.pending_checkpoints, &keys)?;

        Ok(batch.write()?)
    }

    pub fn last_built_checkpoint_message_builder(
        &self,
    ) -> IkaResult<Option<BuilderCheckpointMessage>> {
        Ok(self
            .tables()?
            .builder_checkpoint_message_v1
            .unbounded_iter()
            .skip_to_last()
            .next()
            .map(|(_, s)| s))
    }

    pub fn last_built_checkpoint_message(
        &self,
    ) -> IkaResult<Option<(CheckpointSequenceNumber, CheckpointMessage)>> {
        Ok(self
            .tables()?
            .builder_checkpoint_message_v1
            .unbounded_iter()
            .skip_to_last()
            .next()
            .map(|(seq, s)| (seq, s.checkpoint_message)))
    }

    pub fn get_built_checkpoint_message(
        &self,
        sequence: CheckpointSequenceNumber,
    ) -> IkaResult<Option<CheckpointMessage>> {
        Ok(self
            .tables()?
            .builder_checkpoint_message_v1
            .get(&sequence)?
            .map(|s| s.checkpoint_message))
    }

    pub fn get_last_checkpoint_signature_index(&self) -> IkaResult<u64> {
        Ok(self
            .tables()?
            .pending_checkpoint_signatures
            .unbounded_iter()
            .skip_to_last()
            .next()
            .map(|((_, index), _)| index)
            .unwrap_or_default())
    }

    pub fn insert_checkpoint_signature(
        &self,
        checkpoint_seq: CheckpointSequenceNumber,
        index: u64,
        info: &CheckpointSignatureMessage,
    ) -> IkaResult<()> {
        Ok(self
            .tables()?
            .pending_checkpoint_signatures
            .insert(&(checkpoint_seq, index), info)?)
    }

    pub fn record_initiate_process_mid_epoch_quorum_time_metric(&self) {
        if let Some(mid_epoch_time) = *self.mid_epoch_time.read() {
            self.metrics
                .epoch_initiate_process_mid_epoch_quorum_time_since_mid_epoch_reached_ms
                .set(mid_epoch_time.elapsed().as_millis() as i64);
        }
    }

    pub(crate) fn record_epoch_pending_certs_process_time_metric(&self) {
        if let Some(epoch_close_time) = *self.epoch_close_time.read() {
            self.metrics
                .epoch_pending_certs_processed_time_since_epoch_close_ms
                .set(epoch_close_time.elapsed().as_millis() as i64);
        }
    }

    pub fn record_end_of_message_quorum_time_metric(&self) {
        if let Some(epoch_close_time) = *self.epoch_close_time.read() {
            self.metrics
                .epoch_end_of_publish_quorum_time_since_epoch_close_ms
                .set(epoch_close_time.elapsed().as_millis() as i64);
        }
    }

    pub(crate) fn report_epoch_metrics_at_last_checkpoint(&self, checkpoint_count: u64) {
        if let Some(epoch_close_time) = *self.epoch_close_time.read() {
            self.metrics
                .epoch_last_checkpoint_created_time_since_epoch_close_ms
                .set(epoch_close_time.elapsed().as_millis() as i64);
        }
        info!(epoch=?self.epoch(), "Epoch statistics: checkpoint_count={:?}", checkpoint_count);
        self.metrics
            .epoch_checkpoint_count
            .set(checkpoint_count as i64);
        // self.metrics
        //     .epoch_transaction_count
        //     .set(stats.transaction_count as i64);
    }

    pub fn record_epoch_reconfig_start_time_metric(&self) {
        if let Some(epoch_close_time) = *self.epoch_close_time.read() {
            self.metrics
                .epoch_reconfig_start_time_since_epoch_close_ms
                .set(epoch_close_time.elapsed().as_millis() as i64);
        }
    }

    fn record_reconfig_halt_duration_metric(&self) {
        if let Some(epoch_close_time) = *self.epoch_close_time.read() {
            self.metrics
                .epoch_validator_halt_duration_ms
                .set(epoch_close_time.elapsed().as_millis() as i64);
        }
    }

    pub(crate) fn record_epoch_first_checkpoint_creation_time_metric(&self) {
        self.metrics
            .epoch_first_checkpoint_created_time_since_epoch_begin_ms
            .set(self.epoch_open_time.elapsed().as_millis() as i64);
    }

    fn record_epoch_total_duration_metric(&self) {
        self.metrics.current_epoch.set(self.epoch() as i64);
        self.metrics
            .epoch_total_duration
            .set(self.epoch_open_time.elapsed().as_millis() as i64);
    }
}

#[derive(Default)]
pub(crate) struct ConsensusCommitOutput {
    // Consensus and reconfig state
    consensus_round: Round,
    consensus_messages_processed: BTreeSet<SequencedConsensusTransactionKey>,
    initiate_process_mid_epoch: BTreeSet<AuthorityName>,
    end_of_publish: BTreeSet<AuthorityName>,
    reconfig_state: Option<ReconfigState>,
    consensus_commit_stats: Option<ExecutionIndicesWithStats>,

    pending_checkpoints: Vec<PendingCheckpoint>,

    /// All the dWallet-MPC related TXs that have been received in this round.
    dwallet_mpc_round_messages: Vec<DWalletMPCDBMessage>,
    dwallet_mpc_round_outputs: Vec<DWalletMPCOutputMessage>,
    dwallet_mpc_round_events: Vec<DWalletMPCEvent>,
    dwallet_mpc_completed_sessions: Vec<ObjectID>,
}

impl ConsensusCommitOutput {
    pub fn new(consensus_round: Round) -> Self {
        Self {
            consensus_round,
            ..Default::default()
        }
    }

    pub(crate) fn set_dwallet_mpc_round_messages(&mut self, new_value: Vec<DWalletMPCDBMessage>) {
        self.dwallet_mpc_round_messages = new_value;
    }

    pub(crate) fn set_dwallet_mpc_round_outputs(
        &mut self,
        new_value: Vec<DWalletMPCOutputMessage>,
    ) {
        self.dwallet_mpc_round_outputs = new_value;
    }

    pub(crate) fn set_dwallet_mpc_round_completed_sessions(&mut self, new_value: Vec<ObjectID>) {
        self.dwallet_mpc_completed_sessions = new_value;
    }

    pub(crate) fn set_dwallet_mpc_round_events(&mut self, new_value: Vec<DWalletMPCEvent>) {
        self.dwallet_mpc_round_events = new_value;
    }

    fn insert_initiate_process_mid_epoch(&mut self, authority: AuthorityName) {
        self.initiate_process_mid_epoch.insert(authority);
    }

    fn insert_end_of_publish(&mut self, authority: AuthorityName) {
        self.end_of_publish.insert(authority);
    }

    fn record_consensus_commit_stats(&mut self, stats: ExecutionIndicesWithStats) {
        self.consensus_commit_stats = Some(stats);
    }

    fn store_reconfig_state(&mut self, state: ReconfigState) {
        self.reconfig_state = Some(state);
    }

    fn record_consensus_message_processed(&mut self, key: SequencedConsensusTransactionKey) {
        self.consensus_messages_processed.insert(key);
    }

    fn insert_pending_checkpoint(&mut self, checkpoint: PendingCheckpoint) {
        self.pending_checkpoints.push(checkpoint);
    }

    pub fn write_to_batch(
        self,
        epoch_store: &AuthorityPerEpochStore,
        batch: &mut DBBatch,
    ) -> IkaResult {
        let tables = epoch_store.tables()?;

        // Write all the dWallet MPC related messages from this consensus round to the local DB.
        // The [`DWalletMPCService`] constantly reads and process those messages.
        if let Some(consensus_commit_stats) = &self.consensus_commit_stats {
            batch.insert_batch(
                &tables.dwallet_mpc_messages,
                [(
                    consensus_commit_stats.index.sub_dag_index,
                    self.dwallet_mpc_round_messages,
                )],
            )?;
            batch.insert_batch(
                &tables.dwallet_mpc_completed_sessions,
                [(
                    consensus_commit_stats.index.sub_dag_index,
                    self.dwallet_mpc_completed_sessions,
                )],
            )?;
            batch.insert_batch(
                &tables.dwallet_mpc_outputs,
                [(
                    consensus_commit_stats.index.sub_dag_index,
                    self.dwallet_mpc_round_outputs,
                )],
            )?;
            batch.insert_batch(
                &tables.dwallet_mpc_events,
                [(
                    consensus_commit_stats.index.sub_dag_index,
                    self.dwallet_mpc_round_events,
                )],
            )?;
        } else {
            error!("failed to retrieve consensus commit statistics when trying to write DWallet MPC messages to local DB");
        }

        batch.insert_batch(
            &tables.consensus_message_processed,
            self.consensus_messages_processed
                .iter()
                .map(|key| (key, true)),
        )?;

        batch.insert_batch(
            &tables.initiate_process_mid_epoch,
            self.initiate_process_mid_epoch
                .iter()
                .map(|authority| (authority, ())),
        )?;

        batch.insert_batch(
            &tables.end_of_publish,
            self.end_of_publish.iter().map(|authority| (authority, ())),
        )?;

        if let Some(reconfig_state) = &self.reconfig_state {
            batch.insert_batch(
                &tables.reconfig_state,
                [(RECONFIG_STATE_INDEX, reconfig_state)],
            )?;
        }

        if let Some(consensus_commit_stats) = &self.consensus_commit_stats {
            batch.insert_batch(
                &tables.last_consensus_stats,
                [(LAST_CONSENSUS_STATS_ADDR, consensus_commit_stats)],
            )?;
        }

        batch.insert_batch(
            &tables.pending_checkpoints,
            self.pending_checkpoints
                .into_iter()
                .map(|cp| (cp.height(), cp)),
        )?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LockDetailsWrapper {
    V1(MessageDigest),
}

impl LockDetailsWrapper {
    pub fn migrate(self) -> Self {
        // TODO: when there are multiple versions, we must iteratively migrate from version N to
        // N+1 until we arrive at the latest version
        self
    }

    // Always returns the most recent version. Older versions are migrated to the latest version at
    // read time, so there is never a need to access older versions.
    pub fn inner(&self) -> &LockDetails {
        match self {
            Self::V1(v1) => v1,

            // can remove #[allow] when there are multiple versions
            #[allow(unreachable_patterns)]
            _ => panic!("lock details should have been migrated to latest version at read time"),
        }
    }
    pub fn into_inner(self) -> LockDetails {
        match self {
            Self::V1(v1) => v1,

            // can remove #[allow] when there are multiple versions
            #[allow(unreachable_patterns)]
            _ => panic!("lock details should have been migrated to latest version at read time"),
        }
    }
}

pub type LockDetails = MessageDigest;

impl From<LockDetails> for LockDetailsWrapper {
    fn from(details: LockDetails) -> Self {
        // always use latest version.
        LockDetailsWrapper::V1(details)
    }
}
