// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::*;
use serde::{Deserialize, Serialize};
use std::path::Path;
use sui_json_rpc_types::SuiEvent;
use sui_types::Identifier;
use typed_store::metrics::SamplingInterval;
use typed_store::rocks::util::{empty_compaction_filter, reference_count_merge_operator};
use typed_store::rocks::{
    default_db_options, read_size_from_env, DBBatch, DBMap, DBMapTableConfigMap, DBOptions,
    MetricConf,
};
use typed_store::traits::{Map, TableSummary, TypedStoreDebug};

use crate::authority::epoch_start_configuration::EpochStartConfiguration;
use typed_store::rocksdb::Options;
use typed_store::DBMapUtils;

/// AuthorityPerpetualTables contains data that must be preserved from one epoch to the next.
#[derive(DBMapUtils)]
pub struct AuthorityPerpetualTables {
    /// Parameters of the system fixed at the epoch start
    pub(crate) epoch_start_configuration: DBMap<(), EpochStartConfiguration>,

    /// A singleton table that stores latest pruned checkpoint. Used to keep objects pruner progress
    pub(crate) pruned_checkpoint: DBMap<(), CheckpointSequenceNumber>,

    /// pending events from sui received but not yet executed
    pub(crate) pending_events: DBMap<EventID, SuiEvent>,

    /// module identifier to the last processed EventID
    pub(crate) sui_syncer_cursors: DBMap<Identifier, EventID>,
}

impl AuthorityPerpetualTables {
    pub fn path(parent_path: &Path) -> PathBuf {
        parent_path.join("perpetual")
    }

    pub fn open(parent_path: &Path, db_options: Option<Options>) -> Self {
        Self::open_tables_read_write(
            Self::path(parent_path),
            MetricConf::new("perpetual")
                .with_sampling(SamplingInterval::new(Duration::from_secs(60), 0)),
            db_options,
            None,
        )
    }

    pub fn get_recovery_epoch_at_restart(&self) -> IkaResult<EpochId> {
        Ok(self
            .epoch_start_configuration
            .get(&())?
            .expect("Must have current epoch.")
            .epoch_start_state()
            .epoch())
    }

    pub fn set_epoch_start_configuration(
        &self,
        epoch_start_configuration: &EpochStartConfiguration,
    ) -> IkaResult {
        let mut wb = self.epoch_start_configuration.batch();
        wb.insert_batch(
            &self.epoch_start_configuration,
            std::iter::once(((), epoch_start_configuration)),
        )?;
        wb.write()?;
        Ok(())
    }

    pub fn get_highest_pruned_checkpoint(&self) -> IkaResult<CheckpointSequenceNumber> {
        Ok(self.pruned_checkpoint.get(&())?.unwrap_or_default())
    }

    pub fn set_highest_pruned_checkpoint(
        &self,
        wb: &mut DBBatch,
        checkpoint_number: CheckpointSequenceNumber,
    ) -> IkaResult {
        wb.insert_batch(&self.pruned_checkpoint, [((), checkpoint_number)])?;
        Ok(())
    }

    pub fn set_highest_pruned_checkpoint_without_wb(
        &self,
        checkpoint_number: CheckpointSequenceNumber,
    ) -> IkaResult {
        let mut wb = self.pruned_checkpoint.batch();
        self.set_highest_pruned_checkpoint(&mut wb, checkpoint_number)?;
        wb.write()?;
        Ok(())
    }

    pub fn insert_pending_events(&self, module: Identifier, events: &[SuiEvent]) -> IkaResult {
        let cursor = events.last().map(|e| e.id);
        if let Some(cursor) = cursor {
            let mut batch = self.pending_events.batch();
            batch.insert_batch(&self.sui_syncer_cursors, [(module, cursor)])?;
            batch.insert_batch(&self.pending_events, events.iter().map(|e| (e.id, e)))?;
            batch.write()?;
        }
        Ok(())
    }

    pub(crate) fn remove_pending_events(&self, events: &[EventID]) -> IkaResult {
        let mut batch = self.pending_events.batch();
        batch.delete_batch(&self.pending_events, events)?;
        batch.write()?;
        Ok(())
    }

    pub fn get_all_pending_events(&self) -> HashMap<EventID, SuiEvent> {
        self.pending_events.unbounded_iter().collect()
    }

    pub fn get_sui_event_cursors(
        &self,
        identifiers: &[Identifier],
    ) -> IkaResult<Vec<Option<EventID>>> {
        Ok(self.sui_syncer_cursors.multi_get(identifiers)?)
    }
}
