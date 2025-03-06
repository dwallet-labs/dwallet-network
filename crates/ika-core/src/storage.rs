// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::checkpoints::CheckpointStore;
use crate::epoch::committee_store::CommitteeStore;
use ika_types::committee::Committee;
use ika_types::committee::EpochId;
use ika_types::error::IkaError;
use ika_types::messages_checkpoint::CheckpointContentsDigest;
use ika_types::messages_checkpoint::CheckpointMessageDigest;
use ika_types::messages_checkpoint::CheckpointSequenceNumber;
use ika_types::messages_checkpoint::VerifiedCheckpointMessage;
use ika_types::storage::error::Error as StorageError;
use ika_types::storage::error::Result;
use ika_types::storage::ReadStore;
use ika_types::storage::WriteStore;
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct RocksDbStore {
    committee_store: Arc<CommitteeStore>,
    checkpoint_store: Arc<CheckpointStore>,
    // in memory checkpoint watermark sequence numbers
    highest_verified_checkpoint: Arc<Mutex<Option<CheckpointSequenceNumber>>>,
    highest_synced_checkpoint: Arc<Mutex<Option<CheckpointSequenceNumber>>>,
}

impl RocksDbStore {
    pub fn new(
        committee_store: Arc<CommitteeStore>,
        checkpoint_store: Arc<CheckpointStore>,
    ) -> Self {
        Self {
            committee_store,
            checkpoint_store,
            highest_verified_checkpoint: Arc::new(Mutex::new(None)),
            highest_synced_checkpoint: Arc::new(Mutex::new(None)),
        }
    }

    pub fn get_last_executed_checkpoint(
        &self,
        epoch: EpochId,
    ) -> Result<Option<VerifiedCheckpointMessage>, IkaError> {
        Ok(self
            .checkpoint_store
            .get_highest_executed_checkpoint(epoch)?)
    }
}

impl ReadStore for RocksDbStore {
    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Result<Option<VerifiedCheckpointMessage>, StorageError> {
        self.checkpoint_store
            .get_checkpoint_by_digest(digest)
            .map_err(Into::into)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        epoch: EpochId,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedCheckpointMessage>, StorageError> {
        self.checkpoint_store
            .get_checkpoint_by_sequence_number(epoch, sequence_number)
            .map_err(Into::into)
    }

    fn get_highest_verified_checkpoint(
        &self,
        epoch: EpochId,
    ) -> Result<Option<VerifiedCheckpointMessage>, StorageError> {
        self.checkpoint_store
            .get_highest_verified_checkpoint(epoch)
            .map_err(Into::into)
    }

    fn get_highest_synced_checkpoint(
        &self,
        epoch: EpochId,
    ) -> Result<Option<VerifiedCheckpointMessage>, StorageError> {
        self.checkpoint_store
            .get_highest_synced_checkpoint(epoch)
            .map_err(Into::into)
    }

    fn get_lowest_available_checkpoint(
        &self,
        epoch: EpochId,
    ) -> Result<CheckpointSequenceNumber, StorageError> {
        let highest_pruned_cp = self
            .checkpoint_store
            .get_highest_pruned_checkpoint_seq_number(epoch)
            .map_err(Into::<StorageError>::into)?;

        if highest_pruned_cp == 0 {
            Ok(0)
        } else {
            Ok(highest_pruned_cp + 1)
        }
    }

    fn get_committee(
        &self,
        epoch: EpochId,
    ) -> Result<Option<Arc<Committee>>, ika_types::storage::error::Error> {
        Ok(self.committee_store.get_committee(&epoch).unwrap())
    }

    fn get_latest_checkpoint(
        &self,
        epoch: EpochId,
    ) -> ika_types::storage::error::Result<VerifiedCheckpointMessage> {
        self.checkpoint_store
            .get_highest_executed_checkpoint(epoch)
            .map_err(ika_types::storage::error::Error::custom)?
            .ok_or_else(|| {
                ika_types::storage::error::Error::missing("unable to get latest checkpoint")
            })
    }
}

impl WriteStore for RocksDbStore {
    fn insert_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<(), ika_types::storage::error::Error> {
        self.checkpoint_store
            .insert_verified_checkpoint(checkpoint)
            .map_err(Into::into)
    }

    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<(), ika_types::storage::error::Error> {
        let mut locked = self.highest_synced_checkpoint.lock();
        if locked.is_some() && locked.unwrap() >= checkpoint.sequence_number {
            return Ok(());
        }
        self.checkpoint_store
            .update_highest_synced_checkpoint(checkpoint)
            .map_err(ika_types::storage::error::Error::custom)?;
        *locked = Some(checkpoint.sequence_number);
        Ok(())
    }

    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<(), ika_types::storage::error::Error> {
        let mut locked = self.highest_verified_checkpoint.lock();
        if locked.is_some() && locked.unwrap() >= checkpoint.sequence_number {
            return Ok(());
        }
        self.checkpoint_store
            .update_highest_verified_checkpoint(checkpoint)
            .map_err(ika_types::storage::error::Error::custom)?;
        *locked = Some(checkpoint.sequence_number);
        Ok(())
    }

    fn insert_committee(
        &self,
        new_committee: Committee,
    ) -> Result<(), ika_types::storage::error::Error> {
        self.committee_store
            .insert_new_committee(&new_committee)
            .unwrap();
        Ok(())
    }
}
