// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::checkpoints::CheckpointStore;
use crate::epoch::committee_store::CommitteeStore;
use crate::params_messages::ParamsMessageStore;
use ika_types::committee::Committee;
use ika_types::committee::EpochId;
use ika_types::digests::ParamsMessageDigest;
use ika_types::error::IkaError;
use ika_types::messages_checkpoint::CheckpointContentsDigest;
use ika_types::messages_checkpoint::CheckpointMessageDigest;
use ika_types::messages_checkpoint::CheckpointSequenceNumber;
use ika_types::messages_checkpoint::VerifiedCheckpointMessage;
use ika_types::messages_params_messages::{ParamsMessageSequenceNumber, VerifiedParamsMessage};
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

    params_message_store: Arc<ParamsMessageStore>,
    // in memory params_message watermark sequence numbers
    highest_verified_params_message: Arc<Mutex<Option<ParamsMessageSequenceNumber>>>,
    highest_synced_params_message: Arc<Mutex<Option<ParamsMessageSequenceNumber>>>,
}

impl RocksDbStore {
    pub fn new(
        committee_store: Arc<CommitteeStore>,
        checkpoint_store: Arc<CheckpointStore>,
        params_message_store: Arc<ParamsMessageStore>,
    ) -> Self {
        Self {
            committee_store,
            checkpoint_store,
            highest_verified_checkpoint: Arc::new(Mutex::new(None)),
            highest_synced_checkpoint: Arc::new(Mutex::new(None)),
            params_message_store,
            highest_verified_params_message: Arc::new(Mutex::new(None)),
            highest_synced_params_message: Arc::new(Mutex::new(None)),
        }
    }

    pub fn get_last_executed_checkpoint(
        &self,
    ) -> Result<Option<VerifiedCheckpointMessage>, IkaError> {
        Ok(self.checkpoint_store.get_highest_executed_checkpoint()?)
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
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedCheckpointMessage>, StorageError> {
        self.checkpoint_store
            .get_checkpoint_by_sequence_number(sequence_number)
            .map_err(Into::into)
    }

    fn get_highest_verified_checkpoint(
        &self,
    ) -> Result<Option<VerifiedCheckpointMessage>, StorageError> {
        self.checkpoint_store
            .get_highest_verified_checkpoint()
            .map_err(Into::into)
    }

    fn get_highest_synced_checkpoint(
        &self,
    ) -> Result<Option<VerifiedCheckpointMessage>, StorageError> {
        self.checkpoint_store
            .get_highest_synced_checkpoint()
            .map_err(Into::into)
    }

    fn get_lowest_available_checkpoint(&self) -> Result<CheckpointSequenceNumber, StorageError> {
        let highest_pruned_cp = self
            .checkpoint_store
            .get_highest_pruned_checkpoint_seq_number()
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
    ) -> ika_types::storage::error::Result<VerifiedCheckpointMessage> {
        self.checkpoint_store
            .get_highest_executed_checkpoint()
            .map_err(ika_types::storage::error::Error::custom)?
            .ok_or_else(|| {
                ika_types::storage::error::Error::missing("unable to get latest checkpoint")
            })
    }

    fn get_latest_params_message(&self) -> Result<VerifiedParamsMessage> {
        self.params_message_store
            .get_highest_executed_params_message()
            .map_err(ika_types::storage::error::Error::custom)?
            .ok_or_else(|| {
                ika_types::storage::error::Error::missing("unable to get latest params message")
            })
    }

    fn get_highest_verified_params_message(&self) -> Result<Option<VerifiedParamsMessage>> {
        self.params_message_store
            .get_highest_verified_params_message()
            .map_err(ika_types::storage::error::Error::custom)
    }

    fn get_highest_synced_params_message(&self) -> Result<Option<VerifiedParamsMessage>> {
        self.params_message_store
            .get_highest_synced_params_message()
            .map_err(ika_types::storage::error::Error::custom)
    }

    fn get_lowest_available_params_message(&self) -> Result<ParamsMessageSequenceNumber> {
        let highest_pruned_cp = self
            .params_message_store
            .get_highest_pruned_params_message_seq_number()
            .map_err(ika_types::storage::error::Error::custom)?;

        if highest_pruned_cp == 0 {
            Ok(0)
        } else {
            Ok(highest_pruned_cp + 1)
        }
    }

    fn get_params_message_by_digest(
        &self,
        digest: &ParamsMessageDigest,
    ) -> Result<Option<VerifiedParamsMessage>> {
        self.params_message_store
            .get_params_message_by_digest(digest)
            .map_err(ika_types::storage::error::Error::custom)
    }

    fn get_params_message_by_sequence_number(
        &self,
        sequence_number: ParamsMessageSequenceNumber,
    ) -> Result<Option<VerifiedParamsMessage>> {
        self.params_message_store
            .get_params_message_by_sequence_number(sequence_number)
            .map_err(ika_types::storage::error::Error::custom)
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

    fn insert_params_message(&self, params_message: &VerifiedParamsMessage) -> Result<()> {
        self.params_message_store
            .insert_verified_params_message(params_message)
            .map_err(ika_types::storage::error::Error::custom)
    }

    fn update_highest_synced_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<()> {
        let mut locked = self.highest_synced_params_message.lock();
        if locked.is_some() && locked.unwrap() >= params_message.sequence_number {
            return Ok(());
        }
        self.params_message_store
            .update_highest_synced_params_message(params_message)
            .map_err(ika_types::storage::error::Error::custom)?;
        *locked = Some(params_message.sequence_number);
        Ok(())
    }

    fn update_highest_verified_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<()> {
        let mut locked = self.highest_verified_params_message.lock();
        if locked.is_some() && locked.unwrap() >= params_message.sequence_number {
            return Ok(());
        }
        self.params_message_store
            .update_highest_verified_params_message(params_message)
            .map_err(ika_types::storage::error::Error::custom)?;
        *locked = Some(params_message.sequence_number);
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
