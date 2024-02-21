// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use parking_lot::Mutex;
use std::sync::Arc;

use sui_types::base_types::TransactionDigest;
use sui_types::committee::Committee;
use sui_types::committee::EpochId;
use sui_types::digests::{TransactionEffectsDigest, TransactionEventsDigest};
use sui_types::effects::{TransactionEffects, TransactionEvents};
use sui_types::error::SuiError;
use sui_types::messages_checkpoint::CheckpointContentsDigest;
use sui_types::messages_checkpoint::CheckpointDigest;
use sui_types::messages_checkpoint::CheckpointSequenceNumber;
use sui_types::messages_checkpoint::EndOfEpochData;
use sui_types::messages_checkpoint::FullCheckpointContents;
use sui_types::messages_checkpoint::VerifiedCheckpoint;
use sui_types::messages_checkpoint::VerifiedCheckpointContents;
use sui_types::object::Object;
use sui_types::storage::WriteStore;
use sui_types::storage::{ObjectKey, ReadStore};
use sui_types::transaction::VerifiedTransaction;
use typed_store::Map;

use crate::authority::AuthorityStore;
use crate::checkpoints::CheckpointStore;
use crate::epoch::committee_store::CommitteeStore;

#[derive(Clone)]
pub struct RocksDbStore {
    authority_store: Arc<AuthorityStore>,
    committee_store: Arc<CommitteeStore>,
    checkpoint_store: Arc<CheckpointStore>,
    // in memory checkpoint watermark sequence numbers
    highest_verified_checkpoint: Arc<Mutex<Option<u64>>>,
    highest_synced_checkpoint: Arc<Mutex<Option<u64>>>,
}

impl RocksDbStore {
    pub fn new(
        authority_store: Arc<AuthorityStore>,
        committee_store: Arc<CommitteeStore>,
        checkpoint_store: Arc<CheckpointStore>,
    ) -> Self {
        Self {
            authority_store,
            committee_store,
            checkpoint_store,
            highest_verified_checkpoint: Arc::new(Mutex::new(None)),
            highest_synced_checkpoint: Arc::new(Mutex::new(None)),
        }
    }

    pub fn get_objects(&self, object_keys: &[ObjectKey]) -> Result<Vec<Option<Object>>, SuiError> {
        self.authority_store.multi_get_object_by_key(object_keys)
    }

    pub fn get_last_executed_checkpoint(&self) -> Result<Option<VerifiedCheckpoint>, SuiError> {
        Ok(self.checkpoint_store.get_highest_executed_checkpoint()?)
    }
}

impl ReadStore for RocksDbStore {
    type Error = typed_store::TypedStoreError;

    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointDigest,
    ) -> Result<Option<VerifiedCheckpoint>, Self::Error> {
        self.checkpoint_store.get_checkpoint_by_digest(digest)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedCheckpoint>, Self::Error> {
        self.checkpoint_store
            .get_checkpoint_by_sequence_number(sequence_number)
    }

    fn get_highest_verified_checkpoint(&self) -> Result<VerifiedCheckpoint, Self::Error> {
        self.checkpoint_store
            .get_highest_verified_checkpoint()
            .map(|maybe_checkpoint| {
                maybe_checkpoint
                    .expect("storage should have been initialized with genesis checkpoint")
            })
    }

    fn get_highest_synced_checkpoint(&self) -> Result<VerifiedCheckpoint, Self::Error> {
        self.checkpoint_store
            .get_highest_synced_checkpoint()
            .map(|maybe_checkpoint| {
                maybe_checkpoint
                    .expect("storage should have been initialized with genesis checkpoint")
            })
    }

    fn get_lowest_available_checkpoint(&self) -> Result<CheckpointSequenceNumber, Self::Error> {
        self.checkpoint_store
            .get_highest_pruned_checkpoint_seq_number()
            .map(|seq| seq + 1)
    }

    fn get_full_checkpoint_contents_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<FullCheckpointContents>, Self::Error> {
        self.checkpoint_store
            .get_full_checkpoint_contents_by_sequence_number(sequence_number)
    }

    fn get_full_checkpoint_contents(
        &self,
        digest: &CheckpointContentsDigest,
    ) -> Result<Option<FullCheckpointContents>, Self::Error> {
        // First look to see if we saved the complete contents already.
        if let Some(seq_num) = self
            .checkpoint_store
            .get_sequence_number_by_contents_digest(digest)?
        {
            let contents = self
                .checkpoint_store
                .get_full_checkpoint_contents_by_sequence_number(seq_num)?;
            if contents.is_some() {
                return Ok(contents);
            }
        }

        // Otherwise gather it from the individual components.
        // Note we can't insert the constructed contents into `full_checkpoint_content`,
        // because it needs to be inserted along with `checkpoint_sequence_by_contents_digest`
        // and `checkpoint_content`. However at this point it's likely we don't know the
        // corresponding sequence number yet.
        self.checkpoint_store
            .get_checkpoint_contents(digest)?
            .map(|contents| FullCheckpointContents::from_checkpoint_contents(&self, contents))
            .transpose()
            .map(|contents| contents.flatten())
    }

    fn get_committee(&self, epoch: EpochId) -> Result<Option<Arc<Committee>>, Self::Error> {
        Ok(self.committee_store.get_committee(&epoch).unwrap())
    }

    fn get_transaction_block(
        &self,
        digest: &TransactionDigest,
    ) -> Result<Option<VerifiedTransaction>, Self::Error> {
        self.authority_store.get_transaction_block(digest)
    }

    fn get_transaction_effects(
        &self,
        digest: &TransactionEffectsDigest,
    ) -> Result<Option<TransactionEffects>, Self::Error> {
        self.authority_store.perpetual_tables.effects.get(digest)
    }

    fn get_transaction_events(
        &self,
        digest: &TransactionEventsDigest,
    ) -> Result<Option<TransactionEvents>, Self::Error> {
        self.authority_store.get_events(digest)
    }
}

impl WriteStore for RocksDbStore {
    fn insert_checkpoint(&self, checkpoint: &VerifiedCheckpoint) -> Result<(), Self::Error> {
        if let Some(EndOfEpochData {
            next_epoch_committee,
            ..
        }) = checkpoint.end_of_epoch_data.as_ref()
        {
            let next_committee = next_epoch_committee.iter().cloned().collect();
            let committee = Committee::new(checkpoint.epoch().saturating_add(1), next_committee);
            self.insert_committee(committee)?;
        }

        self.checkpoint_store.insert_verified_checkpoint(checkpoint)
    }

    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpoint,
    ) -> Result<(), Self::Error> {
        let mut locked = self.highest_synced_checkpoint.lock();
        if locked.is_some() && locked.unwrap() >= checkpoint.sequence_number {
            return Ok(());
        }
        self.checkpoint_store
            .update_highest_synced_checkpoint(checkpoint)?;
        *locked = Some(checkpoint.sequence_number);
        Ok(())
    }

    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpoint,
    ) -> Result<(), Self::Error> {
        let mut locked = self.highest_verified_checkpoint.lock();
        if locked.is_some() && locked.unwrap() >= checkpoint.sequence_number {
            return Ok(());
        }
        self.checkpoint_store
            .update_highest_verified_checkpoint(checkpoint)?;
        *locked = Some(checkpoint.sequence_number);
        Ok(())
    }

    fn insert_checkpoint_contents(
        &self,
        checkpoint: &VerifiedCheckpoint,
        contents: VerifiedCheckpointContents,
    ) -> Result<(), Self::Error> {
        self.authority_store
            .multi_insert_transaction_and_effects(contents.iter())?;
        self.checkpoint_store
            .insert_verified_checkpoint_contents(checkpoint, contents)
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<(), Self::Error> {
        self.committee_store
            .insert_new_committee(&new_committee)
            .unwrap();
        Ok(())
    }
}
