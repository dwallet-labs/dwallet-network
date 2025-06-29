// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use arc_swap::ArcSwapOption;
use enum_dispatch::enum_dispatch;
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
use sui_types::base_types::ConciseableName;
use sui_types::base_types::{EpochId, ObjectID};
use tracing::{debug, error, info, instrument, trace, warn};
use typed_store::rocks::{default_db_options, DBBatch, DBMap, DBOptions, MetricConf};
use typed_store::rocksdb::Options;

use super::epoch_start_configuration::EpochStartConfigTrait;

use crate::authority::epoch_start_configuration::EpochStartConfiguration;
use crate::authority::{AuthorityMetrics, AuthorityState};
use crate::dwallet_checkpoints::{
    BuilderDWalletCheckpointMessage, DWalletCheckpointHeight, DWalletCheckpointServiceNotify,
    PendingDWalletCheckpoint, PendingDWalletCheckpointInfo, PendingDWalletCheckpointV1,
};

use crate::consensus_handler::{
    ConsensusCommitInfo, SequencedConsensusTransaction, SequencedConsensusTransactionKey,
    SequencedConsensusTransactionKind, VerifiedSequencedConsensusTransaction,
};
use crate::dwallet_mpc::mpc_manager::DWalletMPCDBMessage;
use crate::dwallet_mpc::mpc_outputs_verifier::{
    DWalletMPCOutputsVerifier, OutputVerificationResult, OutputVerificationStatus,
};
use crate::dwallet_mpc::{
    authority_name_to_party_id_from_committee, generate_access_structure_from_committee,
};
use crate::epoch::epoch_metrics::EpochMetrics;
use crate::system_checkpoints::{
    BuilderSystemCheckpoint, PendingSystemCheckpoint, PendingSystemCheckpointInfo,
    PendingSystemCheckpointV1, SystemCheckpointHeight, SystemCheckpointService,
    SystemCheckpointServiceNotify,
};
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, MPCSessionPublicOutput};
use group::PartyID;
use ika_protocol_config::{ProtocolConfig, ProtocolVersion};
use ika_types::committee::ClassGroupsEncryptionKeyAndProof;
use ika_types::digests::MessageDigest;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::message::{
    DKGFirstRoundOutput, DKGSecondRoundOutput, DWalletImportedKeyVerificationOutput,
    DWalletMessageKind, EncryptedUserShareOutput, MakeDWalletUserSecretKeySharesPublicOutput,
    NetworkKeyPublicOutputSlice, PartialSignatureVerificationOutput, PresignOutput, SignOutput,
};
use ika_types::messages_consensus::Round;
use ika_types::messages_consensus::{
    AuthorityCapabilitiesV1, ConsensusTransaction, ConsensusTransactionKey,
    ConsensusTransactionKind,
};
use ika_types::messages_dwallet_checkpoint::{
    DWalletCheckpointMessage, DWalletCheckpointSequenceNumber, DWalletCheckpointSignatureMessage,
};
use ika_types::messages_dwallet_mpc::IkaPackagesConfig;
use ika_types::messages_dwallet_mpc::{
    DWalletMPCOutputMessage, MPCProtocolInitData, SessionInfo, SessionType,
};
use ika_types::messages_system_checkpoints::{
    SystemCheckpointMessage, SystemCheckpointMessageKind, SystemCheckpointSequenceNumber,
    SystemCheckpointSignatureMessage,
};
use ika_types::sui::epoch_start_system::{EpochStartSystem, EpochStartSystemTrait};
use mpc::WeightedThresholdAccessStructure;
use mysten_common::sync::notify_once::NotifyOnce;
use mysten_common::sync::notify_read::NotifyRead;
use mysten_metrics::monitored_scope;
use prometheus::IntCounter;
use sui_types::executable_transaction::TrustedExecutableTransaction;
use tap::TapOptional;
use tokio::time::Instant;
use typed_store::DBMapUtils;
use typed_store::Map;

/// The key where the latest consensus index is stored in the database.
// TODO: Make a single table (e.g., called `variables`) storing all our lonely variables in one place.
const LAST_CONSENSUS_STATS_ADDR: u64 = 0;
const OVERRIDE_PROTOCOL_UPGRADE_BUFFER_STAKE_INDEX: u64 = 0;
pub const EPOCH_DB_PREFIX: &str = "epoch_";

pub enum CancelConsensusCertificateReason {
    CongestionOnObjects(Vec<ObjectID>),
    DkgFailed,
}

pub enum ConsensusCertificateResult {
    /// The consensus message was ignored (e.g. because it has already been processed).
    Ignored,
    /// An executable transaction (can be a user tx or a system tx)
    IkaTransaction(DWalletMessageKind),
    /// An executable transaction used for large output (e.g., network DKG).
    IkaBulkTransaction(Vec<DWalletMessageKind>),
    /// Everything else, e.g. AuthorityCapabilities, CheckpointSignatures, etc.
    ConsensusMessage,
    /// A system message in consensus was ignored (e.g. because of end of epoch).
    IgnoredSystem,

    SystemTransaction(SystemCheckpointMessageKind),
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

    // todo(zeev): why is it not used?
    #[allow(dead_code)]
    executed_in_epoch_table_enabled: once_cell::sync::OnceCell<bool>,

    /// Chain identifier
    chain_identifier: ChainIdentifier,

    pub(crate) packages_config: IkaPackagesConfig,
}

/// AuthorityEpochTables contains tables that contain data that is only valid within an epoch.
#[derive(DBMapUtils)]
pub struct AuthorityEpochTables {
    // todo(zeev): why is it not used?
    #[allow(dead_code)]
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
    pending_dwallet_checkpoints: DBMap<DWalletCheckpointHeight, PendingDWalletCheckpoint>,

    /// Stores pending signatures
    /// The key in this table is checkpoint sequence number and an arbitrary integer
    pub(crate) pending_dwallet_checkpoint_signatures:
        DBMap<(DWalletCheckpointSequenceNumber, u64), DWalletCheckpointSignatureMessage>,

    /// Maps sequence number to checkpoint summary, used by CheckpointBuilder to build checkpoint within epoch
    pub(crate) builder_dwallet_checkpoint_message_v1:
        DBMap<DWalletCheckpointSequenceNumber, BuilderDWalletCheckpointMessage>,

    #[default_options_override_fn = "pending_checkpoints_table_default_config"]
    pending_system_checkpoints: DBMap<SystemCheckpointHeight, PendingSystemCheckpoint>,

    /// Stores pending signatures
    /// The key in this table is ika system checkpoint sequence number and an arbitrary integer
    pub(crate) pending_system_checkpoint_signatures:
        DBMap<(DWalletCheckpointSequenceNumber, u64), SystemCheckpointSignatureMessage>,

    /// Maps sequence number to ika system checkpoint summary, used by SystemCheckpointBuilder to build checkpoint within epoch
    builder_system_checkpoint_v1: DBMap<DWalletCheckpointSequenceNumber, BuilderSystemCheckpoint>,

    /// Record of the capabilities advertised by each authority.
    authority_capabilities_v1: DBMap<AuthorityName, AuthorityCapabilitiesV1>,

    /// Record the every protocol config version sent to the authority at the current epoch.
    /// This is used to check if the authority has already sent the protocol config version,
    /// so it not to be sent again.
    protocol_config_version_sent: DBMap<ProtocolVersion, ()>,

    /// Contains a single key, which overrides the value of
    /// ProtocolConfig::buffer_stake_for_protocol_upgrade_bps
    override_protocol_upgrade_buffer_stake: DBMap<u64, u64>,

    // todo(zeev): why is it not used in system checkpoint?
    /// When transaction is executed via checkpoint executor, we store association here
    pub(crate) executed_transactions_to_checkpoint:
        DBMap<MessageDigest, DWalletCheckpointSequenceNumber>,

    /// Holds all the DWallet MPC related messages that have been
    /// received since the beginning of the epoch.
    /// The key is the consensus round number,
    /// the value is the dWallet-mpc messages that have been received in that
    /// round.
    pub(crate) dwallet_mpc_messages: DBMap<u64, Vec<DWalletMPCDBMessage>>,
    pub(crate) dwallet_mpc_outputs: DBMap<u64, Vec<DWalletMPCOutputMessage>>,
    // TODO (#538): change type to the inner, basic type instead of using Sui's wrapper
    // pub struct SessionID([u8; AccountAddress::LENGTH]);
}

// todo(zeev): why is it not used?
#[allow(dead_code)]
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
        Self::open_tables_read_write(
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

    pub fn get_all_pending_consensus_transactions(&self) -> IkaResult<Vec<ConsensusTransaction>> {
        Ok(self
            .pending_consensus_transactions
            .safe_iter()
            .map(|item| item.map(|(_k, v)| v))
            .collect::<Result<Vec<_>, _>>()?)
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

    pub fn get_all_dwallet_mpc_dwallet_mpc_messages(&self) -> IkaResult<Vec<DWalletMPCDBMessage>> {
        Ok(self
            .dwallet_mpc_messages
            .safe_iter()
            .map(|item| item.map(|(_k, v)| v))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    pub fn get_all_dwallet_mpc_outputs(&self) -> IkaResult<Vec<DWalletMPCOutputMessage>> {
        Ok(self
            .dwallet_mpc_outputs
            .safe_iter()
            .map(|item| item.map(|(_k, v)| v))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }
}

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
        packages_config: IkaPackagesConfig,
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

        let protocol_config =
            ProtocolConfig::get_for_version(protocol_version, chain_identifier.chain());

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
            epoch_open_time: current_time,
            epoch_close_time: Default::default(),
            metrics,
            epoch_start_configuration,
            executed_in_epoch_table_enabled: once_cell::sync::OnceCell::new(),
            chain_identifier,
            packages_config,
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
        authority_name_to_party_id_from_committee(self.committee().as_ref(), authority_name)
    }

    pub(crate) fn get_validators_class_groups_public_keys_and_proofs(
        &self,
    ) -> IkaResult<HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>> {
        let mut validators_class_groups_public_keys_and_proofs = HashMap::new();
        for (name, _) in self.committee().voting_rights.iter() {
            let party_id = self.authority_name_to_party_id(name)?;
            if let Ok(public_key) = self.committee().class_groups_public_key_and_proof(name) {
                validators_class_groups_public_keys_and_proofs.insert(party_id, public_key);
            }
        }
        Ok(validators_class_groups_public_keys_and_proofs)
    }

    pub fn get_weighted_threshold_access_structure(
        &self,
    ) -> DwalletMPCResult<WeightedThresholdAccessStructure> {
        generate_access_structure_from_committee(self.committee().as_ref())
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
            self.packages_config.clone(),
        )
    }

    pub fn new_at_next_epoch_for_testing(&self) -> Arc<Self> {
        let next_epoch = self.epoch() + 1;
        let next_committee = Committee::new(
            next_epoch,
            self.committee.voting_rights.to_vec(),
            self.committee.class_groups_public_keys_and_proofs.clone(),
            self.committee.quorum_threshold,
            self.committee.validity_threshold,
        );
        self.new_at_next_epoch(
            self.name,
            next_committee,
            self.epoch_start_configuration
                .new_at_next_epoch_for_testing(),
            self.chain_identifier,
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
        match self.tables()?.get_last_consensus_stats()? {
            Some(stats) => Ok(stats),
            None => {
                let indices = self
                    .tables()?
                    .get_last_consensus_index()
                    .map(|x| x.unwrap_or_default())?;
                Ok(ExecutionIndicesWithStats {
                    index: indices,
                    hash: 0, // unused
                    stats: ConsensusStats::default(),
                })
            }
        }
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
        // The except() here is on purpose, because the epoch can't run without it.
        self.tables()
            .expect("recovery should not cross epoch boundary")
            .get_all_pending_consensus_transactions()
            .expect("failed to get pending consensus transactions")
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
        Ok(self
            .tables()?
            .authority_capabilities_v1
            .safe_iter()
            .map(|item| item.map(|(_, v)| v))
            .collect::<Result<Vec<_>, _>>()?)
    }

    pub fn record_protocol_config_version_sent(
        &self,
        protocol_version: ProtocolVersion,
    ) -> IkaResult {
        self.tables()?
            .protocol_config_version_sent
            .insert(&protocol_version, &())?;
        Ok(())
    }

    pub fn last_protocol_config_version_sent(&self) -> IkaResult<Option<ProtocolVersion>> {
        Ok(self
            .tables()?
            .protocol_config_version_sent
            .reversed_safe_iter_with_bounds(None, None)?
            .next()
            .transpose()?
            .map(|(s, _)| s))
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
                kind: ConsensusTransactionKind::DWalletMPCMaliciousReport(authority, ..),
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
                kind: ConsensusTransactionKind::DWalletMPCThresholdNotReached(authority, ..),
                ..
            }) => {
                if transaction.sender_authority() != *authority {
                    warn!(
                        ?authority,
                        certificate_author_index=?transaction.certificate_author_index,
                        "DWalletMPCSessionFailedWithMalicious: authority does not match its author from consensus",
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
                kind: ConsensusTransactionKind::DWalletCheckpointSignature(data),
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
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::SystemCheckpointSignature(data),
                ..
            }) => {
                if transaction.sender_authority() != data.checkpoint_message.auth_sig().authority {
                    warn!(
                        "SystemCheckpoint authority {} does not match its author from consensus {}",
                        data.checkpoint_message.auth_sig().authority,
                        transaction.certificate_author_index
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
        C: DWalletCheckpointServiceNotify,
    >(
        &self,
        transactions: Vec<SequencedConsensusTransaction>,
        consensus_stats: &ExecutionIndicesWithStats,
        checkpoint_service: &Arc<C>,
        system_checkpoint_service: &Arc<SystemCheckpointService>,
        consensus_commit_info: &ConsensusCommitInfo,
        authority_metrics: &Arc<AuthorityMetrics>,
        dwallet_mpc_outputs_verifier: &mut DWalletMPCOutputsVerifier,
    ) -> IkaResult<(Vec<DWalletMessageKind>, Vec<SystemCheckpointMessageKind>)> {
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

        let (verified_messages, system_checkpoint_verified_messages, notifications) = self
            .process_consensus_transactions(
                &mut output,
                &consensus_transactions,
                checkpoint_service,
                system_checkpoint_service,
                consensus_commit_info,
                //&mut roots,
                authority_metrics,
                dwallet_mpc_outputs_verifier,
            )
            .await?;
        //self.finish_consensus_certificate_process_with_batch(&mut output, &verified_transactions)?;
        output.record_consensus_commit_stats(consensus_stats.clone());

        let checkpoint_height = consensus_commit_info.round;

        let pending_checkpoint = PendingDWalletCheckpoint::V1(PendingDWalletCheckpointV1 {
            messages: verified_messages.clone(),
            details: PendingDWalletCheckpointInfo {
                timestamp_ms: consensus_commit_info.timestamp,
                checkpoint_height,
            },
        });
        self.write_pending_checkpoint(&mut output, &pending_checkpoint)?;

        let system_checkpoint_height = consensus_commit_info.round;

        let pending_system_checkpoint = PendingSystemCheckpoint::V1(PendingSystemCheckpointV1 {
            messages: system_checkpoint_verified_messages.clone(),
            details: PendingSystemCheckpointInfo {
                timestamp_ms: consensus_commit_info.timestamp,
                checkpoint_height: system_checkpoint_height,
            },
        });
        self.write_pending_system_checkpoint(&mut output, &pending_system_checkpoint)?;

        system_checkpoint_verified_messages.iter().for_each(
            |system_checkpoint_kind| match system_checkpoint_kind {
                SystemCheckpointMessageKind::SetNextConfigVersion(version) => {
                    if let Ok(tables) = self.tables() {
                        if let Err(e) = tables.protocol_config_version_sent.insert(version, &()) {
                            warn!(
                                ?e,
                                "Failed to insert the next protocol config version into the table"
                            );
                        }
                    } else {
                        warn!("Failed to insert params message digest into the table");
                    }
                }
                // For now, we only handle NextConfigVersion. Other variants are ignored.
                SystemCheckpointMessageKind::SetEpochDurationMs(_)
                | SystemCheckpointMessageKind::SetStakeSubsidyStartEpoch(_)
                | SystemCheckpointMessageKind::SetStakeSubsidyRate(_)
                | SystemCheckpointMessageKind::SetStakeSubsidyPeriodLength(_)
                | SystemCheckpointMessageKind::SetMinValidatorCount(_)
                | SystemCheckpointMessageKind::SetMaxValidatorCount(_)
                | SystemCheckpointMessageKind::SetMinValidatorJoiningStake(_)
                | SystemCheckpointMessageKind::SetMaxValidatorChangeCount(_)
                | SystemCheckpointMessageKind::SetRewardSlashingRate(_)
                | SystemCheckpointMessageKind::SetApprovedUpgrade { .. } => {
                    todo!(
                        "Handle other SystemCheckpointKind variants in process_consensus_transactions_and_commit_boundary"
                    );
                }
            },
        );

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

        debug!(
            ?consensus_commit_info.round,
            "Notifying system_checkpoint service about new pending checkpoint(s)",
        );
        system_checkpoint_service.notify_checkpoint()?;

        self.process_notifications(&notifications);

        Ok((verified_messages, system_checkpoint_verified_messages))
    }

    fn process_notifications(&self, notifications: &[SequencedConsensusTransactionKey]) {
        for key in notifications {
            self.consensus_notify_read.notify(key, &());
        }
    }

    /// Depending on the type of the VerifiedSequencedConsensusTransaction wrappers,
    /// - Verify and initialize the state to execute the certificates.
    ///   Return VerifiedCertificates for each executable certificate
    /// - Or update the state for checkpoint or epoch change protocol.
    #[instrument(level = "debug", skip_all)]
    #[allow(clippy::type_complexity)]
    pub(crate) async fn process_consensus_transactions<C: DWalletCheckpointServiceNotify>(
        &self,
        output: &mut ConsensusCommitOutput,
        transactions: &[VerifiedSequencedConsensusTransaction],
        checkpoint_service: &Arc<C>,
        system_checkpoint_service: &Arc<SystemCheckpointService>,
        consensus_commit_info: &ConsensusCommitInfo,
        //roots: &mut BTreeSet<MessageDigest>,
        authority_metrics: &Arc<AuthorityMetrics>,
        dwallet_mpc_outputs_verifier: &mut DWalletMPCOutputsVerifier,
    ) -> IkaResult<(
        Vec<DWalletMessageKind>, // transactions to schedule
        Vec<SystemCheckpointMessageKind>,
        Vec<SequencedConsensusTransactionKey>, // keys to notify as complete
    )> {
        let _scope = monitored_scope("ConsensusCommitHandler::process_consensus_transactions");

        let mut verified_certificates = VecDeque::with_capacity(transactions.len() + 1);
        let mut verified_system_checkpoint_certificates =
            VecDeque::with_capacity(transactions.len() + 1);
        let mut notifications = Vec::with_capacity(transactions.len());

        let cancelled_txns: BTreeMap<MessageDigest, CancelConsensusCertificateReason> =
            BTreeMap::new();

        for tx in transactions {
            let key = tx.0.transaction.key();
            let mut ignored = false;
            // let mut filter_roots = false;
            match self
                .process_consensus_transaction(
                    output,
                    tx,
                    checkpoint_service,
                    system_checkpoint_service,
                    consensus_commit_info.round,
                    authority_metrics,
                    dwallet_mpc_outputs_verifier,
                )
                .await?
            {
                ConsensusCertificateResult::IkaTransaction(cert) => {
                    notifications.push(key.clone());
                    verified_certificates.push_back(cert);
                }
                ConsensusCertificateResult::SystemTransaction(cert) => {
                    notifications.push(key.clone());
                    verified_system_checkpoint_certificates.push_back(cert);
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
                ConsensusCertificateResult::ConsensusMessage => notifications.push(key.clone()),
                ConsensusCertificateResult::IgnoredSystem => {
                    // filter_roots = true;
                }
                // Note: ignored external transactions must not be recorded as processed. Otherwise
                // they may not get reverted after restart during epoch change.
                ConsensusCertificateResult::Ignored => {
                    ignored = true;
                    // filter_roots = true;
                }
            }
            if !ignored {
                output.record_consensus_message_processed(key.clone());
            }
        }

        // Save all the dWallet-MPC related DB data to the consensus commit output to
        // write it to the local DB. After saving the data, clear the data from the epoch store.
        let new_dwallet_mpc_round_messages = Self::filter_dwallet_mpc_messages(transactions);
        output.set_dwallet_mpc_round_messages(new_dwallet_mpc_round_messages);
        output.set_dwallet_mpc_round_outputs(Self::filter_dwallet_mpc_outputs(transactions));

        authority_metrics
            .consensus_handler_cancelled_transactions
            .inc_by(cancelled_txns.len() as u64);

        let verified_certificates: Vec<_> = verified_certificates.into();

        Ok((
            verified_certificates,
            verified_system_checkpoint_certificates.into(),
            notifications,
        ))
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
                        kind: ConsensusTransactionKind::DWalletMPCThresholdNotReached(authority, report),
                        ..
                    }) => Some(DWalletMPCDBMessage::ThresholdNotReachedReport(*authority, report.clone())),
                    SequencedConsensusTransactionKind::External(ConsensusTransaction {
                        kind:
                            ConsensusTransactionKind::DWalletMPCMaliciousReport(
                                authority_name,
                                report,
                            ),
                        ..
                    }) => Some(DWalletMPCDBMessage::MaliciousReport(
                        *authority_name,
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
                        authority: *origin_authority,
                        session_info: *session_info.clone(),
                        output: output.clone(),
                    }),
                    _ => None,
                }
            })
            .collect()
    }

    #[instrument(level = "trace", skip_all)]
    async fn process_consensus_transaction<C: DWalletCheckpointServiceNotify>(
        &self,
        _output: &mut ConsensusCommitOutput,
        transaction: &VerifiedSequencedConsensusTransaction,
        checkpoint_service: &Arc<C>,
        system_checkpoint_service: &Arc<SystemCheckpointService>, // should i do this generic as the checkpoint service?
        _commit_round: Round,
        _authority_metrics: &Arc<AuthorityMetrics>,
        dwallet_mpc_outputs_verifier: &mut DWalletMPCOutputsVerifier,
    ) -> IkaResult<ConsensusCertificateResult> {
        let _scope = monitored_scope("ConsensusCommitHandler::process_consensus_transaction");

        let VerifiedSequencedConsensusTransaction(SequencedConsensusTransaction {
            certificate_author_index: _,
            certificate_author,
            consensus_index: _consensus_index,
            transaction,
        }) = transaction;
        let _tracking_id = transaction.get_tracking_id();

        match &transaction {
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::DWalletMPCOutput(_, session_info, output),
                ..
            }) => {
                self.process_dwallet_mpc_output(
                    *certificate_author,
                    dwallet_mpc_outputs_verifier,
                    *session_info.clone(),
                    output.clone(),
                )
                .await
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::DWalletMPCMaliciousReport(..),
                ..
            }) => Ok(ConsensusCertificateResult::ConsensusMessage),
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::DWalletMPCThresholdNotReached(..),
                ..
            }) => Ok(ConsensusCertificateResult::ConsensusMessage),
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::DWalletMPCMessage(..),
                ..
            }) => Ok(ConsensusCertificateResult::ConsensusMessage),
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::DWalletCheckpointSignature(info),
                ..
            }) => {
                // We usually call notify_checkpoint_signature in IkaTxValidator, but that step can
                // be skipped when a batch is already part of a certificate, so we must also
                // notify here.
                checkpoint_service.notify_checkpoint_signature(self, info)?;
                Ok(ConsensusCertificateResult::ConsensusMessage)
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::CapabilityNotificationV1(authority_capabilities),
                ..
            }) => {
                let authority = authority_capabilities.authority;
                debug!(
                    "Received CapabilityNotificationV1 from {:?}",
                    authority.concise()
                );
                self.record_capabilities_v1(authority_capabilities)?;
                let capabilities = self.get_capabilities_v1()?;
                if let Some((new_version, _)) = AuthorityState::is_protocol_version_supported_v1(
                    self.protocol_version(),
                    authority_capabilities
                        .supported_protocol_versions
                        .versions
                        .iter()
                        .max_by(|(protocol_version_a, _), (protocol_version_b, _)| {
                            protocol_version_a.cmp(protocol_version_b)
                        })
                        .map(|(protocol_version, _)| *protocol_version)
                        .unwrap_or_else(|| {
                            warn!("No supported protocol versions found in capabilities");
                            self.protocol_version()
                        }),
                    self.protocol_config(),
                    self.committee(),
                    capabilities.clone(),
                    self.get_effective_buffer_stake_bps(),
                ) {
                    let last_version_sent = self.last_protocol_config_version_sent()?;
                    if last_version_sent.is_none() || last_version_sent != Some(new_version) {
                        info!(
                            validator=?self.name,
                            protocol_version=?new_version,
                            "Found version quorum from capabilities v1 {:?}",
                            capabilities.first()
                        );
                        return Ok(ConsensusCertificateResult::SystemTransaction(
                            SystemCheckpointMessageKind::SetNextConfigVersion(new_version),
                        ));
                    }
                    Ok(ConsensusCertificateResult::ConsensusMessage)
                } else {
                    Ok(ConsensusCertificateResult::ConsensusMessage)
                }
            }
            SequencedConsensusTransactionKind::External(ConsensusTransaction {
                kind: ConsensusTransactionKind::SystemCheckpointSignature(data),
                ..
            }) => {
                system_checkpoint_service.notify_checkpoint_signature(self, data)?;
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
        dwallet_mpc_outputs_verifier: &mut DWalletMPCOutputsVerifier,
        session_info: SessionInfo,
        output: Vec<u8>,
    ) -> IkaResult<ConsensusCertificateResult> {
        let authority_index = self.authority_name_to_party_id(&origin_authority);

        let output_verification_result = dwallet_mpc_outputs_verifier
                .try_verify_output(&output, &session_info, origin_authority, &self)
                .unwrap_or_else(|e| {
                    error!("error verifying DWalletMPCOutput output from session identifier {:?} and party {:?}: {:?}",session_info.session_identifier, authority_index, e);
                    OutputVerificationResult {
                        result: OutputVerificationStatus::Malicious,
                        malicious_actors: vec![origin_authority],
                    }
                });

        match output_verification_result.result {
            OutputVerificationStatus::FirstQuorumReached(output) => self
                .process_dwallet_transaction(output, session_info)
                .map_err(IkaError::from),
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
        system_transaction: &DWalletMessageKind,
    ) -> ConsensusCertificateResult {
        ConsensusCertificateResult::IkaTransaction(system_transaction.clone())
    }

    fn process_consensus_system_bulk_transaction(
        &self,
        system_transaction: &[DWalletMessageKind],
    ) -> ConsensusCertificateResult {
        ConsensusCertificateResult::IkaBulkTransaction(system_transaction.to_owned())
    }

    fn process_dwallet_transaction(
        &self,
        output: Vec<u8>,
        session_info: SessionInfo,
    ) -> DwalletMPCResult<ConsensusCertificateResult> {
        info!(
            validator=?self.name,
            mpc_protocol=?session_info.mpc_round,
            session_identifier=?session_info.session_identifier,
            "Creating session output checkpoint transaction"
        );
        let (is_rejected, output) = match bcs::from_bytes(&output)? {
            MPCSessionPublicOutput::CompletedSuccessfully(output) => (false, output),
            MPCSessionPublicOutput::SessionFailed => (true, vec![]),
        };
        match &session_info.mpc_round {
            MPCProtocolInitData::DKGFirst(event_data) => {
                let SessionType::User { sequence_number } = event_data.session_type else {
                    unreachable!("DKGFirst round should be a user session");
                };
                let tx =
                    DWalletMessageKind::RespondDWalletDKGFirstRoundOutput(DKGFirstRoundOutput {
                        dwallet_id: event_data.event_data.dwallet_id.to_vec(),
                        output,
                        session_sequence_number: sequence_number,
                        rejected: is_rejected,
                    });
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
            MPCProtocolInitData::DKGSecond(init_event_data) => {
                let SessionType::User { sequence_number } = init_event_data.session_type else {
                    unreachable!("DKGSecond round should be a user session");
                };
                let tx =
                    DWalletMessageKind::RespondDWalletDKGSecondRoundOutput(DKGSecondRoundOutput {
                        output,
                        dwallet_id: init_event_data.event_data.dwallet_id.to_vec(),
                        encrypted_secret_share_id: init_event_data
                            .event_data
                            .encrypted_user_secret_key_share_id
                            .to_vec(),
                        rejected: is_rejected,
                        session_sequence_number: sequence_number,
                    });
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
            MPCProtocolInitData::Presign(init_event_data) => {
                let SessionType::User { sequence_number } = init_event_data.session_type else {
                    unreachable!("Presign round should be a user session");
                };
                let tx = DWalletMessageKind::RespondDWalletPresign(PresignOutput {
                    presign: output,
                    dwallet_id: init_event_data.event_data.dwallet_id.map(|id| id.to_vec()),
                    presign_id: init_event_data.event_data.presign_id.to_vec(),
                    rejected: is_rejected,
                    session_sequence_number: sequence_number,
                });
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
            MPCProtocolInitData::Sign(init_event) => {
                let SessionType::User { sequence_number } = init_event.session_type else {
                    unreachable!("Sign round should be a user session");
                };
                let tx = DWalletMessageKind::RespondDWalletSign(SignOutput {
                    signature: output,
                    dwallet_id: init_event.event_data.dwallet_id.to_vec(),
                    is_future_sign: init_event.event_data.is_future_sign,
                    sign_id: init_event.event_data.sign_id.to_vec(),
                    rejected: is_rejected,
                    session_sequence_number: sequence_number,
                });
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
            MPCProtocolInitData::EncryptedShareVerification(init_event_data) => {
                let SessionType::User { sequence_number } = init_event_data.session_type else {
                    unreachable!("EncryptedShareVerification round should be a user session");
                };
                let tx = DWalletMessageKind::RespondDWalletEncryptedUserShare(
                    EncryptedUserShareOutput {
                        dwallet_id: init_event_data.event_data.dwallet_id.to_vec(),
                        encrypted_user_secret_key_share_id: init_event_data
                            .event_data
                            .encrypted_user_secret_key_share_id
                            .to_vec(),
                        rejected: is_rejected,
                        session_sequence_number: sequence_number,
                    },
                );
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
            MPCProtocolInitData::PartialSignatureVerification(init_event_data) => {
                let SessionType::User { sequence_number } = init_event_data.session_type else {
                    unreachable!("PartialSignatureVerification round should be a user session");
                };
                let tx = DWalletMessageKind::RespondDWalletPartialSignatureVerificationOutput(
                    PartialSignatureVerificationOutput {
                        dwallet_id: init_event_data.event_data.dwallet_id.to_vec(),
                        partial_centralized_signed_message_id: init_event_data
                            .event_data
                            .partial_centralized_signed_message_id
                            .to_vec(),
                        rejected: is_rejected,
                        session_sequence_number: sequence_number,
                    },
                );
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
            MPCProtocolInitData::NetworkEncryptionKeyDkg(key_scheme, init_event) => {
                match key_scheme {
                    DWalletMPCNetworkKeyScheme::Secp256k1 => {
                        let slices = if is_rejected {
                            vec![NetworkKeyPublicOutputSlice {
                                session_id: init_event.session_object_id.to_vec(),
                                dwallet_network_decryption_key_id: init_event
                                    .event_data
                                    .dwallet_network_decryption_key_id
                                    .clone()
                                    .to_vec(),
                                public_output: vec![],
                                supported_curves: vec![
                                    DWalletMPCNetworkKeyScheme::Secp256k1 as u32,
                                ],
                                is_last: true,
                                rejected: true,
                            }]
                        } else {
                            Self::slice_network_dkg_public_output_into_messages(
                                &init_event.event_data.dwallet_network_decryption_key_id,
                                output,
                                init_event.session_object_id.to_vec(),
                            )
                        };

                        let messages: Vec<_> = slices
                            .into_iter()
                            .map(DWalletMessageKind::RespondDWalletMPCNetworkDKGOutput)
                            .collect();
                        Ok(self.process_consensus_system_bulk_transaction(&messages))
                    }
                    DWalletMPCNetworkKeyScheme::Ristretto => {
                        Err(DwalletMPCError::UnsupportedNetworkDKGKeyScheme)
                    }
                }
            }
            MPCProtocolInitData::NetworkEncryptionKeyReconfiguration(init_event) => {
                let slices = if is_rejected {
                    vec![NetworkKeyPublicOutputSlice {
                        session_id: init_event.session_object_id.to_vec(),
                        dwallet_network_decryption_key_id: init_event
                            .event_data
                            .dwallet_network_decryption_key_id
                            .clone()
                            .to_vec(),
                        public_output: vec![],
                        supported_curves: vec![DWalletMPCNetworkKeyScheme::Secp256k1 as u32],
                        is_last: true,
                        rejected: true,
                    }]
                } else {
                    Self::slice_network_dkg_public_output_into_messages(
                        &init_event.event_data.dwallet_network_decryption_key_id,
                        output,
                        init_event.session_object_id.to_vec(),
                    )
                };

                let messages: Vec<_> = slices
                    .into_iter()
                    .map(DWalletMessageKind::RespondDWalletMPCNetworkReconfigurationOutput)
                    .collect();
                Ok(self.process_consensus_system_bulk_transaction(&messages))
            }
            MPCProtocolInitData::MakeDWalletUserSecretKeySharesPublicRequest(init_event) => {
                let SessionType::User { sequence_number } = init_event.session_type else {
                    unreachable!(
                        "MakeDWalletUserSecretKeySharesPublic round should be a user session"
                    );
                };
                let tx = DWalletMessageKind::RespondMakeDWalletUserSecretKeySharesPublic(
                    MakeDWalletUserSecretKeySharesPublicOutput {
                        dwallet_id: init_event.event_data.dwallet_id.to_vec(),
                        public_user_secret_key_shares: init_event
                            .event_data
                            .public_user_secret_key_shares
                            .clone(),
                        rejected: is_rejected,
                        session_sequence_number: sequence_number,
                    },
                );
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
            MPCProtocolInitData::DWalletImportedKeyVerificationRequest(init_event) => {
                let SessionType::User { sequence_number } = init_event.session_type else {
                    unreachable!(
                        "MakeDWalletUserSecretKeySharesPublic round should be a user session"
                    );
                };
                let tx = DWalletMessageKind::RespondDWalletImportedKeyVerificationOutput(
                    DWalletImportedKeyVerificationOutput {
                        dwallet_id: init_event.event_data.dwallet_id.to_vec().clone(),
                        public_output: output,
                        encrypted_user_secret_key_share_id: init_event
                            .event_data
                            .encrypted_user_secret_key_share_id
                            .to_vec()
                            .clone(),
                        rejected: is_rejected,
                        session_sequence_number: sequence_number,
                    },
                );
                Ok(ConsensusCertificateResult::IkaTransaction(tx))
            }
        }
    }

    /// Break down the key to slices because of chain transaction size limits.
    /// Limit 16 KB per Tx `pure` argument.
    fn slice_network_dkg_public_output_into_messages(
        dwallet_network_decryption_key_id: &ObjectID,
        public_output: Vec<u8>,
        session_id: Vec<u8>,
    ) -> Vec<NetworkKeyPublicOutputSlice> {
        let mut slices = Vec::new();
        // We set a total of 5 KB since we need 6 KB buffer for other params.
        let five_kbytes = 5 * 1024;
        let public_chunks = public_output.chunks(five_kbytes).collect_vec();
        let empty: &[u8] = &[];
        // Take the max of the two lengths to ensure we have enough slices.
        for i in 0..public_chunks.len() {
            // If the chunk is missing, use an empty slice, as the size of the slices can be different.
            let public_chunk = public_chunks.get(i).unwrap_or(&empty);
            slices.push(NetworkKeyPublicOutputSlice {
                session_id: session_id.clone(),
                dwallet_network_decryption_key_id: dwallet_network_decryption_key_id
                    .clone()
                    .to_vec(),
                public_output: (*public_chunk).to_vec(),
                supported_curves: vec![DWalletMPCNetworkKeyScheme::Secp256k1 as u32],
                is_last: i == public_chunks.len() - 1,
                rejected: false,
            });
        }
        slices
    }

    pub(crate) fn write_pending_checkpoint(
        &self,
        output: &mut ConsensusCommitOutput,
        checkpoint: &PendingDWalletCheckpoint,
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

    pub fn get_pending_dwallet_checkpoints(
        &self,
        last: Option<DWalletCheckpointHeight>,
    ) -> IkaResult<Vec<(DWalletCheckpointHeight, PendingDWalletCheckpoint)>> {
        let tables = self.tables()?;
        let db_iter = tables
            .pending_dwallet_checkpoints
            .safe_iter_with_bounds(last.map(|height| height + 1), None);
        Ok(db_iter.collect::<Result<Vec<_>, _>>()?)
    }

    pub fn get_pending_checkpoint(
        &self,
        index: &DWalletCheckpointHeight,
    ) -> IkaResult<Option<PendingDWalletCheckpoint>> {
        Ok(self.tables()?.pending_dwallet_checkpoints.get(index)?)
    }

    pub fn process_pending_dwallet_checkpoint(
        &self,
        commit_height: DWalletCheckpointHeight,
        checkpoint_messages: Vec<DWalletCheckpointMessage>,
    ) -> IkaResult<()> {
        let tables = self.tables()?;
        // All created checkpoints are inserted in builder_checkpoint_summary in a single batch.
        // This means that upon restart we can use BuilderCheckpointSummary::commit_height
        // from the last built summary to resume building checkpoints.
        let mut batch = tables.pending_dwallet_checkpoints.batch();
        for (position_in_commit, summary) in checkpoint_messages.into_iter().enumerate() {
            let sequence_number = summary.sequence_number;
            let summary = BuilderDWalletCheckpointMessage {
                checkpoint_message: summary,
                checkpoint_height: Some(commit_height),
                position_in_commit,
            };
            batch.insert_batch(
                &tables.builder_dwallet_checkpoint_message_v1,
                [(&sequence_number, summary)],
            )?;
        }

        // find all pending checkpoints <= commit_height and remove them
        let iter = tables
            .pending_dwallet_checkpoints
            .safe_range_iter(0..=commit_height);
        let keys = iter
            .map(|c| c.map(|(h, _)| h))
            .collect::<Result<Vec<_>, _>>()?;

        batch.delete_batch(&tables.pending_dwallet_checkpoints, &keys)?;

        Ok(batch.write()?)
    }

    pub fn last_built_dwallet_checkpoint_message_builder(
        &self,
    ) -> IkaResult<Option<BuilderDWalletCheckpointMessage>> {
        Ok(self
            .tables()?
            .builder_dwallet_checkpoint_message_v1
            .reversed_safe_iter_with_bounds(None, None)?
            .next()
            .transpose()?
            .map(|(_, s)| s))
    }

    pub fn last_built_dwallet_checkpoint_message(
        &self,
    ) -> IkaResult<Option<(DWalletCheckpointSequenceNumber, DWalletCheckpointMessage)>> {
        Ok(self
            .tables()?
            .builder_dwallet_checkpoint_message_v1
            .reversed_safe_iter_with_bounds(None, None)?
            .next()
            .transpose()?
            .map(|(seq, s)| (seq, s.checkpoint_message)))
    }

    pub fn get_built_dwallet_checkpoint_message(
        &self,
        sequence: DWalletCheckpointSequenceNumber,
    ) -> IkaResult<Option<DWalletCheckpointMessage>> {
        Ok(self
            .tables()?
            .builder_dwallet_checkpoint_message_v1
            .get(&sequence)?
            .map(|s| s.checkpoint_message))
    }

    pub fn get_last_dwallet_checkpoint_signature_index(&self) -> IkaResult<u64> {
        Ok(self
            .tables()?
            .pending_dwallet_checkpoint_signatures
            .reversed_safe_iter_with_bounds(None, None)?
            .next()
            .transpose()?
            .map(|((_, index), _)| index)
            .unwrap_or(1))
    }

    pub fn insert_checkpoint_signature(
        &self,
        checkpoint_seq: DWalletCheckpointSequenceNumber,
        index: u64,
        info: &DWalletCheckpointSignatureMessage,
    ) -> IkaResult<()> {
        Ok(self
            .tables()?
            .pending_dwallet_checkpoint_signatures
            .insert(&(checkpoint_seq, index), info)?)
    }

    pub(crate) fn write_pending_system_checkpoint(
        &self,
        output: &mut ConsensusCommitOutput,
        system_checkpoint: &PendingSystemCheckpoint,
    ) -> IkaResult {
        assert!(
            self.get_pending_system_checkpoint(&system_checkpoint.height())?
                .is_none(),
            "Duplicate pending system_checkpoint notification at height {:?}",
            system_checkpoint.height()
        );

        debug!(
            system_checkpoint_commit_height = system_checkpoint.height(),
            "Pending system_checkpoint has {} messages",
            system_checkpoint.messages().len(),
        );
        trace!(
            system_checkpoint_commit_height = system_checkpoint.height(),
            "Messages for pending system_checkpoint: {:?}",
            system_checkpoint.messages()
        );

        output.insert_pending_system_checkpoint(system_checkpoint.clone());

        Ok(())
    }

    pub fn get_pending_system_checkpoints(
        &self,
        last: Option<SystemCheckpointHeight>,
    ) -> IkaResult<Vec<(SystemCheckpointHeight, PendingSystemCheckpoint)>> {
        let tables = self.tables()?;
        let db_iter = tables
            .pending_system_checkpoints
            .safe_iter_with_bounds(last.map(|height| height + 1), None);
        Ok(db_iter.collect::<Result<Vec<_>, _>>()?)
    }

    pub fn get_pending_system_checkpoint(
        &self,
        index: &SystemCheckpointHeight,
    ) -> IkaResult<Option<PendingSystemCheckpoint>> {
        Ok(self.tables()?.pending_system_checkpoints.get(index)?)
    }

    pub fn process_pending_system_checkpoint(
        &self,
        commit_height: SystemCheckpointHeight,
        system_checkpoint_messages: Vec<SystemCheckpointMessage>,
    ) -> IkaResult<()> {
        let tables = self.tables()?;
        // All created system_checkpoints are inserted in builder_system_checkpoint_summary in a single batch.
        // This means that upon restart we can use BuilderSystemCheckpointSummary::commit_height
        // from the last built summary to resume building system_checkpoints.
        let mut batch = tables.pending_system_checkpoints.batch();
        for (position_in_commit, summary) in system_checkpoint_messages.into_iter().enumerate() {
            let sequence_number = summary.sequence_number;
            let summary = BuilderSystemCheckpoint {
                checkpoint_message: summary,
                checkpoint_height: Some(commit_height),
                position_in_commit,
            };
            batch.insert_batch(
                &tables.builder_system_checkpoint_v1,
                [(&sequence_number, summary)],
            )?;
        }

        // find all pending system_checkpoints <= commit_height and remove them
        let iter = tables
            .pending_system_checkpoints
            .safe_range_iter(0..=commit_height);
        let keys = iter
            .map(|c| c.map(|(h, _)| h))
            .collect::<Result<Vec<_>, _>>()?;

        batch.delete_batch(&tables.pending_system_checkpoints, &keys)?;

        Ok(batch.write()?)
    }

    pub fn last_built_system_checkpoint_message_builder(
        &self,
    ) -> IkaResult<Option<BuilderSystemCheckpoint>> {
        Ok(self
            .tables()?
            .builder_system_checkpoint_v1
            .reversed_safe_iter_with_bounds(None, None)?
            .next()
            .transpose()?
            .map(|(_, s)| s))
    }

    pub fn last_built_system_checkpoint_message(
        &self,
    ) -> IkaResult<Option<(SystemCheckpointSequenceNumber, SystemCheckpointMessage)>> {
        Ok(self
            .tables()?
            .builder_system_checkpoint_v1
            .reversed_safe_iter_with_bounds(None, None)?
            .next()
            .transpose()?
            .map(|(seq, s)| (seq, s.checkpoint_message)))
    }

    pub fn get_built_system_checkpoint_message(
        &self,
        sequence: SystemCheckpointSequenceNumber,
    ) -> IkaResult<Option<SystemCheckpointMessage>> {
        Ok(self
            .tables()?
            .builder_system_checkpoint_v1
            .get(&sequence)?
            .map(|s| s.checkpoint_message))
    }

    pub fn get_last_system_checkpoint_signature_index(&self) -> IkaResult<u64> {
        Ok(self
            .tables()?
            .pending_system_checkpoint_signatures
            .reversed_safe_iter_with_bounds(None, None)?
            .next()
            .transpose()?
            .map(|((_, index), _)| index)
            .unwrap_or(1))
    }

    pub fn insert_system_checkpoint_signature(
        &self,
        system_checkpoint_seq: SystemCheckpointSequenceNumber,
        index: u64,
        info: &SystemCheckpointSignatureMessage,
    ) -> IkaResult<()> {
        Ok(self
            .tables()?
            .pending_system_checkpoint_signatures
            .insert(&(system_checkpoint_seq, index), info)?)
    }

    // todo(zeev): why is it not used?
    #[allow(dead_code)]
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

    pub(crate) fn record_epoch_first_system_checkpoint_creation_time_metric(&self) {
        self.metrics
            .epoch_first_system_checkpoint_created_time_since_epoch_begin_ms
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
    // todo(zeev): why is it not used?
    #[allow(dead_code)]
    // Consensus and reconfig state
    consensus_round: Round,
    consensus_messages_processed: BTreeSet<SequencedConsensusTransactionKey>,
    consensus_commit_stats: Option<ExecutionIndicesWithStats>,

    pending_checkpoints: Vec<PendingDWalletCheckpoint>,
    pending_system_checkpoints: Vec<PendingSystemCheckpoint>,

    /// All the dWallet-MPC related TXs that have been received in this round.
    dwallet_mpc_round_messages: Vec<DWalletMPCDBMessage>,
    dwallet_mpc_round_outputs: Vec<DWalletMPCOutputMessage>,
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

    fn record_consensus_commit_stats(&mut self, stats: ExecutionIndicesWithStats) {
        self.consensus_commit_stats = Some(stats);
    }

    fn record_consensus_message_processed(&mut self, key: SequencedConsensusTransactionKey) {
        self.consensus_messages_processed.insert(key);
    }

    fn insert_pending_checkpoint(&mut self, checkpoint: PendingDWalletCheckpoint) {
        self.pending_checkpoints.push(checkpoint);
    }

    fn insert_pending_system_checkpoint(&mut self, checkpoint: PendingSystemCheckpoint) {
        self.pending_system_checkpoints.push(checkpoint);
    }

    pub fn write_to_batch(
        self,
        epoch_store: &AuthorityPerEpochStore,
        batch: &mut DBBatch,
    ) -> IkaResult {
        let tables = epoch_store.tables()?;

        // Write all the dWallet MPC related messages from this consensus round to the local DB.
        // The [`DWalletMPCService`] constantly reads and process those messages.
        batch.insert_batch(
            &tables.dwallet_mpc_messages,
            [(self.consensus_round, self.dwallet_mpc_round_messages)],
        )?;
        batch.insert_batch(
            &tables.dwallet_mpc_outputs,
            [(self.consensus_round, self.dwallet_mpc_round_outputs)],
        )?;

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
            &tables.pending_dwallet_checkpoints,
            self.pending_checkpoints
                .into_iter()
                .map(|cp| (cp.height(), cp)),
        )?;

        batch.insert_batch(
            &tables.pending_system_checkpoints,
            self.pending_system_checkpoints
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
