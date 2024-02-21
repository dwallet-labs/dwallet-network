// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use async_trait::async_trait;
use move_binary_format::CompiledModule;
use move_bytecode_utils::module_cache::GetModule;
use std::collections::BTreeMap;
use std::sync::Arc;

use sui_types::base_types::{ObjectID, SequenceNumber};
use sui_types::object::ObjectRead;

use crate::errors::IndexerError;
use crate::handlers::{EpochToCommit, TransactionObjectChangesToCommit};

use crate::models_v2::display::StoredDisplay;
use crate::types_v2::{
    IndexedCheckpoint, IndexedEvent, IndexedPackage, IndexedTransaction, TxIndex,
};

#[async_trait]
pub trait IndexerStoreV2 {
    type ModuleCache: GetModule<Item = Arc<CompiledModule>, Error = anyhow::Error>
        + Send
        + Sync
        + 'static;

    async fn get_latest_tx_checkpoint_sequence_number(&self) -> Result<Option<u64>, IndexerError>;

    async fn get_latest_object_snapshot_checkpoint_sequence_number(
        &self,
    ) -> Result<Option<u64>, IndexerError>;

    async fn get_object_read(
        &self,
        object_id: ObjectID,
        version: Option<SequenceNumber>,
    ) -> Result<ObjectRead, IndexerError>;

    async fn persist_objects(
        &self,
        object_changes: Vec<TransactionObjectChangesToCommit>,
    ) -> Result<(), IndexerError>;

    async fn persist_object_history(
        &self,
        object_changes: Vec<TransactionObjectChangesToCommit>,
    ) -> Result<(), IndexerError>;

    async fn persist_object_snapshot(&self) -> Result<(), IndexerError>;

    async fn persist_checkpoints(
        &self,
        checkpoints: Vec<IndexedCheckpoint>,
    ) -> Result<(), IndexerError>;

    async fn persist_transactions(
        &self,
        transactions: Vec<IndexedTransaction>,
    ) -> Result<(), IndexerError>;

    async fn persist_tx_indices(&self, indices: Vec<TxIndex>) -> Result<(), IndexerError>;

    async fn persist_events(&self, events: Vec<IndexedEvent>) -> Result<(), IndexerError>;
    async fn persist_displays(
        &self,
        display_updates: BTreeMap<String, StoredDisplay>,
    ) -> Result<(), IndexerError>;

    async fn persist_packages(&self, packages: Vec<IndexedPackage>) -> Result<(), IndexerError>;

    async fn persist_epoch(&self, epoch: EpochToCommit) -> Result<(), IndexerError>;

    async fn advance_epoch(&self, epoch: EpochToCommit) -> Result<(), IndexerError>;

    async fn get_network_total_transactions_by_end_of_epoch(
        &self,
        epoch: u64,
    ) -> Result<u64, IndexerError>;

    fn module_cache(&self) -> Arc<Self::ModuleCache>;
}
