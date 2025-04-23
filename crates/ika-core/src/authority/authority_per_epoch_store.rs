// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use arc_swap::ArcSwapOption;
use enum_dispatch::enum_dispatch;
use fastcrypto::groups::bls12381;
use fastcrypto_tbls::dkg_v1;
use futures::future::{join_all, select, Either};
use futures::FutureExt;
use ika_types::committee::Committee;
use ika_types::committee::CommitteeTrait;
use ika_types::crypto::AuthorityName;
use ika_types::digests::ChainIdentifier;
use ika_types::error::{IkaError, IkaResult};
use itertools::Itertools;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use sui_macros::fail_point_arg;
use sui_types::accumulator::Accumulator;
use sui_types::authenticator_state::{get_authenticator_state, ActiveJwk};
use sui_types::base_types::{ConciseableName, ObjectRef, SuiAddress};
use sui_types::base_types::{EpochId, ObjectID, SequenceNumber};
use sui_types::crypto::RandomnessRound;
use sui_types::signature::GenericSignature;
use sui_types::transaction::TransactionKey;
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
use crate::dwallet_mpc::mpc_manager::{DWalletMPCDBMessage, DWalletMPCManager};
use crate::dwallet_mpc::mpc_outputs_verifier::{
    DWalletMPCOutputsVerifier, OutputVerificationResult, OutputVerificationStatus,
};
use crate::dwallet_mpc::mpc_session::FAILED_SESSION_OUTPUT;
use crate::dwallet_mpc::network_dkg::DwalletMPCNetworkKeys;
use crate::epoch::epoch_metrics::EpochMetrics;
use crate::stake_aggregator::{GenericMultiStakeAggregator, StakeAggregator};
use dwallet_classgroups_types::{ClassGroupsDecryptionKey, ClassGroupsEncryptionKeyAndProof};
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCPublicOutput, NetworkDecryptionKeyPublicData,
};
use group::PartyID;
use ika_protocol_config::{Chain, ProtocolConfig, ProtocolVersion};
use ika_types::digests::MessageDigest;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::message::{
    DKGFirstRoundOutput, DKGSecondRoundOutput, EncryptedUserShareOutput, MessageKind,
    PartialSignatureVerificationOutput, PresignOutput, Secp256K1NetworkKeyPublicOutputSlice,
    SignOutput,
};
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
use ika_types::messages_dwallet_mpc::{DWalletMPCMessage, IkaPackagesConfig};
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
use sui_types::error::ExecutionError;
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
    /// An executable transaction used for large output (e.g., network DKG).
    IkaBulkTransaction(Vec<MessageKind>),
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

    /// MutexTable for transaction locks (prevent concurrent execution of same transaction)
    mutex_table: MutexTable<MessageDigest>,

    /// The moment when the current epoch started locally on this validator. Note that this
    /// value could be skewed if the node crashed and restarted in the middle of the epoch. That's
    /// ok because this is used for metric purposes and we could tolerate some skews occasionally.
    pub(crate) epoch_open_time: Instant,

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
    pub dwallet_mpc_network_keys: OnceCell<Arc<DwalletMPCNetworkKeys>>,
    pub(crate) dwallet_mpc_round_events: tokio::sync::Mutex<Vec<DWalletMPCEvent>>,
    pub(crate) perpetual_tables: Arc<AuthorityPerpetualTables>,
    pub(crate) packages_config: IkaPackagesConfig,
    pub next_epoch_committee: Arc<tokio::sync::RwLock<Option<Committee>>>,
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
        packages_config: IkaPackagesConfig,
        next_epoch_committee: Arc<tokio::sync::RwLock<Option<Committee>>>,
    ) -> Arc<Self> {
        let current_time = Instant::now();
        let epoch_id = committee.epoch;

        let tables = AuthorityEpochTables::open(epoch_id, parent_path, db_options.clone());

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
            epoch_alive_notify,
            user_certs_closed_notify: NotifyOnce::new(),
            epoch_alive: tokio::sync::RwLock::new(true),
            consensus_notify_read: NotifyRead::new(),
            executed_transactions_to_checkpoint_notify_read: NotifyRead::new(),
            executed_digests_notify_read: NotifyRead::new(),
            synced_checkpoint_notify_read: NotifyRead::new(),
            highest_synced_checkpoint: RwLock::new(0),
            mutex_table: MutexTable::new(MUTEX_TABLE_SIZE),
            epoch_open_time: current_time,
            epoch_close_time: Default::default(),
            metrics,
            epoch_start_configuration,
            executed_in_epoch_table_enabled: once_cell::sync::OnceCell::new(),
            chain_identifier,
            dwallet_mpc_outputs_verifier: OnceCell::new(),
            dwallet_mpc_round_events: tokio::sync::Mutex::new(Vec::new()),
            dwallet_mpc_network_keys: OnceCell::new(),
            perpetual_tables,
            packages_config,
            next_epoch_committee,
        });

        s.update_buffer_stake_metric();
        s
    }

    /// Convert a given authority name (address) to it's corresponding [`PartyID`].
    /// The [`PartyID`] is the index of the authority in the committee.
    pub fn authority_name_to_party_id(
        &self,
        authority_name: &AuthorityName,
    ) -> DwalletMPCResult<PartyID> {
        self.committee()
            .authority_index(authority_name)
            // Need to add 1 because the authority index is 0-based,
            // and the twopc_mpc library uses 1-based party IDs.
            .map(|index| (index + 1) as PartyID)
            .ok_or_else(|| DwalletMPCError::AuthorityNameNotFound(*authority_name))
    }

    pub(crate) fn get_validators_class_groups_public_keys_and_proofs(
        &self,
    ) -> IkaResult<HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>> {
        let mut validators_class_groups_public_keys_and_proofs = HashMap::new();
        for (name, _) in self.committee().voting_rights.iter() {
            let party_id = self.authority_name_to_party_id(name)?;
            let public_key =
                bcs::from_bytes(&self.committee().class_groups_public_key_and_proof(name)?)
                    .map_err(|e| DwalletMPCError::BcsError(e))?;
            validators_class_groups_public_keys_and_proofs.insert(party_id, public_key);
        }
        Ok(validators_class_groups_public_keys_and_proofs)
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
            .map(|(name, weight)| Ok((self.authority_name_to_party_id(name)?, *weight as Weight)))
            .collect::<DwalletMPCResult<HashMap<PartyID, Weight>>>()?;

        WeightedThresholdAccessStructure::new(quorum_threshold as PartyID, weighted_parties)
            .map_err(|e| DwalletMPCError::TwoPCMPCError(e.to_string()))
    }

    /// A function to initiate the network keys `state` for the dWallet MPC when a new epoch starts.
    pub fn set_dwallet_mpc_network_keys(
        &self,
        dwallet_network_keys: Arc<DwalletMPCNetworkKeys>,
    ) -> IkaResult<()> {
        if self
            .dwallet_mpc_network_keys
            .set(dwallet_network_keys)
            .is_err()
        {
            error!("AuthorityPerEpochStore: `set_dwallet_mpc_network_keys` called more than once; this should never happen");
        }
        Ok(())
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
            self.packages_config.clone(),
            self.next_epoch_committee.clone(),
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
            self.committee.class_groups_public_keys_and_proofs.clone(),
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

        for tx in verified_transactions {
            if tx.0.is_system() {
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

        let (mut verified_messages, notifications) = self
            .process_consensus_transactions(
                &mut output,
                &consensus_transactions,
                checkpoint_service,
                consensus_commit_info,
                //&mut roots,
                authority_metrics,
            )
            .await?;
        //self.finish_consensus_certificate_process_with_batch(&mut output, &verified_transactions)?;
        output.record_consensus_commit_stats(consensus_stats.clone());

        let checkpoint_height = consensus_commit_info.round;

        let pending_checkpoint = PendingCheckpoint::V1(PendingCheckpointV1 {
            messages: verified_messages.clone(),
            details: PendingCheckpointInfo {
                timestamp_ms: consensus_commit_info.timestamp,
                checkpoint_height,
            },
        });
        self.write_pending_checkpoint(&mut output, &pending_checkpoint)?;

        let mut batch = self.db_batch()?;
        output.write_to_batch(self, &mut batch)?;
        batch.write()?;

        // Only after batch is written, notify checkpoint service to start building any new
        // pending checkpoints.
        debug!(
            ?consensus_commit_info.round,
            "Notifying checkpoint service about new pending checkpoint(s)",
        );
        checkpoint_service.notify_checkpoint()?;

        self.process_notifications(&notifications);

        Ok(verified_messages)
    }

    fn process_notifications(&self, notifications: &[SequencedConsensusTransactionKey]) {
        for key in notifications.iter().cloned() {
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
        checkpoint_service: &Arc<C>,
        consensus_commit_info: &ConsensusCommitInfo,
        //roots: &mut BTreeSet<MessageDigest>,
        authority_metrics: &Arc<AuthorityMetrics>,
    ) -> IkaResult<(
        Vec<MessageKind>,                      // transactions to schedule
        Vec<SequencedConsensusTransactionKey>, // keys to notify as complete
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
                // This is a special transaction needed for NetworkDKG to bypass TX
                // size limits.
                ConsensusCertificateResult::IkaBulkTransaction(certs) => {
                    notifications.push(key.clone());
                    certs
                        .into_iter()
                        .for_each(|cert| verified_certificates.push_back(cert));
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

        // Save all the dWallet-MPC related DB data to the consensus commit output to
        // write it to the local DB. After saving the data, clear the data from the epoch store.
        let mut new_dwallet_mpc_round_messages = Self::filter_dwallet_mpc_messages(transactions);
        new_dwallet_mpc_round_messages.push(DWalletMPCDBMessage::EndOfDelivery);
        output.set_dwallet_mpc_round_messages(new_dwallet_mpc_round_messages);
        output.set_dwallet_mpc_round_outputs(Self::filter_dwallet_mpc_outputs(transactions));
        let mut dwallet_mpc_round_events = self.dwallet_mpc_round_events.lock().await;
        output.set_dwallet_mpc_round_events(dwallet_mpc_round_events.clone());

        dwallet_mpc_round_events.clear();
        let mut outputs_verifier = self.get_dwallet_mpc_outputs_verifier().await;
        output.set_dwallet_mpc_round_completed_sessions(
            outputs_verifier
                .consensus_round_completed_sessions
                .clone()
                .into_iter()
                .collect(),
        );

        outputs_verifier
            .consensus_round_completed_sessions
            .clear();

        authority_metrics
            .consensus_handler_cancelled_transactions
            .inc_by(cancelled_txns.len() as u64);

        let verified_certificates: Vec<_> = verified_certificates.into();

        Ok((verified_certificates, notifications))
    }

    /// Filter DWalletMPCMessages from the consensus output.
    /// Those messages will get processed when the dWallet MPC service reads
    /// them from the DB.
    fn filter_dwallet_mpc_messages(
        transactions: &[VerifiedSequencedConsensusTransaction],
    ) -> Vec<DWalletMPCDBMessage> {
        transactions
            .iter()
            .filter_map(|transaction| {
                let VerifiedSequencedConsensusTransaction(SequencedConsensusTransaction {
                    transaction,
                    ..
                }) = transaction;
                match transaction {
                    SequencedConsensusTransactionKind::External(ConsensusTransaction {
                        kind: ConsensusTransactionKind::DWalletMPCMessage(message),
                        ..
                    }) => Some(DWalletMPCDBMessage::Message(message.clone())),
                    SequencedConsensusTransactionKind::External(ConsensusTransaction {
                        kind:
                            ConsensusTransactionKind::DWalletMPCSessionFailedWithMalicious(
                                authority_name,
                                report,
                            ),
                        ..
                    }) => Some(DWalletMPCDBMessage::SessionFailedWithMaliciousParties(
                        authority_name.clone(),
                        report.clone(),
                    )),
                    _ => None,
                }
            })
            .collect()
    }

    /// Filter DWalletMPCMessages from the consensus output.
    /// Those messages will get processed when the dWallet MPC service reads
    /// them from the DB.
    fn filter_dwallet_mpc_outputs(
        transactions: &[VerifiedSequencedConsensusTransaction],
    ) -> Vec<DWalletMPCOutputMessage> {
        transactions
            .iter()
            .filter_map(|transaction| {
                let VerifiedSequencedConsensusTransaction(SequencedConsensusTransaction {
                    transaction,
                    ..
                }) = transaction;
                match transaction {
                    SequencedConsensusTransactionKind::External(ConsensusTransaction {
                        kind:
                            ConsensusTransactionKind::DWalletMPCOutput(
                                origin_authority,
                                session_info,
                                output,
                            ),
                        ..
                    }) => Some(DWalletMPCOutputMessage {
                        authority: origin_authority.clone(),
                        session_info: session_info.clone(),
                        output: output.clone(),
                    }),
                    _ => None,
                }
            })
            .collect()
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
                kind: ConsensusTransactionKind::DWalletMPCSessionFailedWithMalicious(..),
                ..
            }) => Ok(ConsensusCertificateResult::ConsensusMessage),
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::DWalletMPCMessage(..),
                ..
            }) => Ok(ConsensusCertificateResult::ConsensusMessage),
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
                kind: ConsensusTransactionKind::CapabilityNotificationV1(capabilities),
                ..
            }) => {
                let authority = capabilities.authority;
                debug!(
                    "Received CapabilityNotificationV2 from {:?}",
                    authority.concise()
                );
                self.record_capabilities_v1(capabilities)?;
                Ok(ConsensusCertificateResult::ConsensusMessage)
            }
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
        let authority_index = self.authority_name_to_party_id(&origin_authority);
        let mut dwallet_mpc_verifier = self.get_dwallet_mpc_outputs_verifier().await;
        let output_verification_result = dwallet_mpc_verifier
                .try_verify_output(&output, &session_info, origin_authority)
                .await
                .unwrap_or_else(|e| {
                    error!("error verifying DWalletMPCOutput output from session {:?} and party {:?}: {:?}",session_info.session_id, authority_index, e);
                    OutputVerificationResult {
                        result: OutputVerificationStatus::Malicious,
                        malicious_actors: vec![origin_authority],
                    }
                });

        match output_verification_result.result {
            OutputVerificationStatus::FirstQuorumReached(output) => {
                self.process_dwallet_transaction(output, session_info)
                    .map_err(|e| IkaError::from(e))
            }
            OutputVerificationStatus::NotEnoughVotes => {
                Ok(ConsensusCertificateResult::ConsensusMessage)
            }
            OutputVerificationStatus::AlreadyCommitted | OutputVerificationStatus::Malicious => {
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
        ConsensusCertificateResult::IkaTransaction(system_transaction.clone())
    }

    fn process_consensus_system_bulk_transaction(
        &self,
        system_transaction: &Vec<MessageKind>,
    ) -> ConsensusCertificateResult {
        ConsensusCertificateResult::IkaBulkTransaction(system_transaction.clone())
    }

    fn process_dwallet_transaction(
        &self,
        output: Vec<u8>,
        session_info: SessionInfo,
    ) -> DwalletMPCResult<ConsensusCertificateResult> {
        info!(
            validator=?self.name,
            mpc_protocol=?session_info.mpc_round,
            session_id=?session_info.session_id,
            "creating session output checkpoint transaction"
        );
        let rejected = output == FAILED_SESSION_OUTPUT.to_vec();
        match &session_info.mpc_round {
            MPCProtocolInitData::DKGFirst(event_data) => {
                let tx = MessageKind::DwalletDKGFirstRoundOutput(DKGFirstRoundOutput {
                    dwallet_id: event_data.event_data.dwallet_id.to_vec(),
                    output,
                    session_sequence_number: event_data.session_sequence_number,
                });
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
            MPCProtocolInitData::DKGSecond(init_event_data) => {
                let tx = MessageKind::DwalletDKGSecondRoundOutput(DKGSecondRoundOutput {
                    output,
                    dwallet_id: init_event_data.event_data.dwallet_id.to_vec(),
                    session_id: session_info.session_id.to_vec(),
                    encrypted_centralized_secret_share_and_proof: bcs::to_bytes(
                        &init_event_data
                            .event_data
                            .encrypted_centralized_secret_share_and_proof,
                    )?,
                    encryption_key_address: init_event_data
                        .event_data
                        .encryption_key_address
                        .to_vec(),
                    rejected,
                    session_sequence_number: init_event_data.session_sequence_number,
                });
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
            MPCProtocolInitData::Presign(init_event_data) => {
                let tx = MessageKind::DwalletPresign(PresignOutput {
                    presign: output,
                    session_id: bcs::to_bytes(&session_info.session_id)?,
                    dwallet_id: init_event_data.event_data.dwallet_id.to_vec(),
                    presign_id: init_event_data.event_data.presign_id.to_vec(),
                    rejected,
                    session_sequence_number: init_event_data.session_sequence_number,
                });
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
            MPCProtocolInitData::Sign(init_event) => {
                let tx = MessageKind::DwalletSign(SignOutput {
                    session_id: session_info.session_id.to_vec(),
                    signature: output,
                    dwallet_id: init_event.event_data.dwallet_id.to_vec(),
                    is_future_sign: init_event.event_data.is_future_sign,
                    sign_id: init_event.event_data.sign_id.to_vec(),
                    rejected,
                    session_sequence_number: init_event.session_sequence_number,
                });
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
            MPCProtocolInitData::EncryptedShareVerification(init_event_data) => {
                let tx = MessageKind::DwalletEncryptedUserShare(EncryptedUserShareOutput {
                    dwallet_id: init_event_data.event_data.dwallet_id.to_vec(),
                    encrypted_user_secret_key_share_id: init_event_data
                        .event_data
                        .encrypted_user_secret_key_share_id
                        .to_vec(),
                    rejected,
                    session_sequence_number: init_event_data.session_sequence_number,
                });
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
            MPCProtocolInitData::PartialSignatureVerification(init_event_data) => {
                let tx = MessageKind::DwalletPartialSignatureVerificationOutput(
                    PartialSignatureVerificationOutput {
                        dwallet_id: init_event_data.event_data.dwallet_id.to_vec(),
                        session_id: session_info.session_id.to_vec(),
                        partial_centralized_signed_message_id: init_event_data
                            .event_data
                            .partial_centralized_signed_message_id
                            .to_vec(),
                        rejected,
                        session_sequence_number: init_event_data.session_sequence_number,
                    },
                );
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
            MPCProtocolInitData::NetworkDkg(key_scheme, init_event) => match key_scheme {
                DWalletMPCNetworkKeyScheme::Secp256k1 => {
                    let slices = Self::slice_network_decryption_key_public_output_into_messages(
                        &init_event.event_data.dwallet_network_decryption_key_id,
                        output,
                    );

                    let messages: Vec<_> = slices
                        .into_iter()
                        .map(|slice| MessageKind::DwalletMPCNetworkDKGOutput(slice))
                        .collect();
                    Ok(self.process_consensus_system_bulk_transaction(&messages))
                }
                DWalletMPCNetworkKeyScheme::Ristretto => {
                    Err(DwalletMPCError::UnsupportedNetworkDKGKeyScheme)
                }
            },
            MPCProtocolInitData::DecryptionKeyReshare(init_event) => {
                let slices = Self::slice_network_decryption_key_public_output_into_messages(
                    &init_event.event_data.dwallet_network_decryption_key_id,
                    output,
                );

                let messages: Vec<_> = slices
                    .into_iter()
                    .map(|slice| MessageKind::DwalletMPCNetworkReshareOutput(slice))
                    .collect();
                Ok(self.process_consensus_system_bulk_transaction(&messages))
            }
        }
    }

    /// Break down the key to slices because of chain transaction size limits.
    /// Limit 16 KB per Tx `pure` argument.
    fn slice_network_decryption_key_public_output_into_messages(
        dwallet_network_decryption_key_id: &ObjectID,
        public_output: Vec<u8>,
    ) -> Vec<Secp256K1NetworkKeyPublicOutputSlice> {
        let mut slices = Vec::new();
        // We set a total of 5 KB since we need 6 KB buffer for other params.
        let five_kbytes = 5 * 1024;
        let public_chunks = public_output.chunks(five_kbytes).collect_vec();
        let empty: &[u8] = &[];
        // Take the max of the two lengths to ensure we have enough slices.
        for i in 0..public_chunks.len() {
            // If the chunk is missing, use an empty slice, as the size of the slices can be different.
            let public_chunk = public_chunks.get(i).unwrap_or(&empty);
            slices.push(Secp256K1NetworkKeyPublicOutputSlice {
                dwallet_network_decryption_key_id: dwallet_network_decryption_key_id
                    .clone()
                    .to_vec(),
                public_output: (*public_chunk).to_vec(),
                is_last: i == public_chunks.len() - 1,
            });
        }
        slices
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

    pub(crate) fn record_epoch_pending_certs_process_time_metric(&self) {
        if let Some(epoch_close_time) = *self.epoch_close_time.read() {
            self.metrics
                .epoch_pending_certs_processed_time_since_epoch_close_ms
                .set(epoch_close_time.elapsed().as_millis() as i64);
        }
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

    fn record_consensus_commit_stats(&mut self, stats: ExecutionIndicesWithStats) {
        self.consensus_commit_stats = Some(stats);
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
