// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::*;
use std::path::Path;
use sui_json_rpc_types::SuiEvent;
use sui_types::Identifier;
use typed_store::rocks::{DBBatch, DBMap, MetricConf};
use typed_store::traits::Map;

use crate::authority::epoch_start_configuration::EpochStartConfiguration;
use ika_types::messages_dwallet_mpc::DBSuiEvent;
use typed_store::rocksdb::Options;
use typed_store::DBMapUtils;

/// AuthorityPerpetualTables contains data that must be preserved from one epoch to the next.
#[derive(DBMapUtils)]
pub struct AuthorityPerpetualTables {
    /// Parameters of the system fixed at the epoch start
    pub(crate) epoch_start_configuration: DBMap<(), EpochStartConfiguration>,

    /// A singleton table that stores latest pruned checkpoint. Used to keep objects pruner progress
    pub(crate) pruned_checkpoint: DBMap<(), DWalletCheckpointSequenceNumber>,

    /// pending events from sui received but not yet executed
    pending_events: DBMap<EventID, Vec<u8>>,

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
            MetricConf::new("perpetual"),
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

    pub fn get_highest_pruned_checkpoint(&self) -> IkaResult<DWalletCheckpointSequenceNumber> {
        Ok(self.pruned_checkpoint.get(&())?.unwrap_or_default())
    }

    pub fn set_highest_pruned_checkpoint(
        &self,
        wb: &mut DBBatch,
        checkpoint_number: DWalletCheckpointSequenceNumber,
    ) -> IkaResult {
        wb.insert_batch(&self.pruned_checkpoint, [((), checkpoint_number)])?;
        Ok(())
    }

    pub fn set_highest_pruned_checkpoint_without_wb(
        &self,
        checkpoint_number: DWalletCheckpointSequenceNumber,
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
            let serialized_events: IkaResult<Vec<(EventID, Vec<u8>)>> = events
                .iter()
                .map(|e| {
                    let serialized_event = bcs::to_bytes(&DBSuiEvent {
                        type_: e.type_.clone(),
                        contents: e.bcs.clone().into_bytes(),
                    })
                    .map_err(|e| IkaError::BCSError(e.to_string()))?;
                    Ok((e.id, serialized_event))
                })
                .collect();
            batch.insert_batch(&self.pending_events, serialized_events?)?;
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

    pub fn get_all_pending_events(&self) -> IkaResult<HashMap<EventID, Vec<u8>>> {
        Ok(self
            .pending_events
            .safe_iter_with_bounds(None, None)
            .collect::<Result<HashMap<_, _>, _>>()?)
    }

    pub fn get_sui_event_cursors(
        &self,
        identifiers: &[Identifier],
    ) -> IkaResult<Vec<Option<EventID>>> {
        Ok(self.sui_syncer_cursors.multi_get(identifiers)?)
    }
}
