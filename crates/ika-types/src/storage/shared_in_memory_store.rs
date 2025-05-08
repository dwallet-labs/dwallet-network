// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::error::Result;
use crate::committee::{Committee, EpochId};
use crate::digests::{CheckpointContentsDigest, CheckpointMessageDigest};
use crate::message::MessageKind;
use crate::messages_checkpoint::{CheckpointSequenceNumber, VerifiedCheckpointMessage};
use crate::storage::{ReadStore, WriteStore};
use std::collections::HashMap;
use std::sync::Arc;
use tap::Pipe;
use tracing::error;

#[derive(Clone, Debug, Default)]
pub struct SharedInMemoryStore(Arc<std::sync::RwLock<InMemoryStore>>);

impl SharedInMemoryStore {
    pub fn inner(&self) -> std::sync::RwLockReadGuard<'_, InMemoryStore> {
        self.0.read().unwrap()
    }

    pub fn inner_mut(&self) -> std::sync::RwLockWriteGuard<'_, InMemoryStore> {
        self.0.write().unwrap()
    }
}

impl ReadStore for SharedInMemoryStore {
    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Result<Option<VerifiedCheckpointMessage<MessageKind>>> {
        self.inner()
            .get_checkpoint_by_digest(digest)
            .cloned()
            .pipe(Ok)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedCheckpointMessage<MessageKind>>> {
        self.inner()
            .get_checkpoint_by_sequence_number(sequence_number)
            .cloned()
            .pipe(Ok)
    }

    fn get_highest_verified_checkpoint(
        &self,
    ) -> Result<Option<VerifiedCheckpointMessage<MessageKind>>> {
        self.inner()
            .get_highest_verified_checkpoint()
            .cloned()
            .pipe(Ok)
    }

    fn get_highest_synced_checkpoint(
        &self,
    ) -> Result<Option<VerifiedCheckpointMessage<MessageKind>>> {
        self.inner()
            .get_highest_synced_checkpoint()
            .cloned()
            .pipe(Ok)
    }

    fn get_lowest_available_checkpoint(&self) -> Result<CheckpointSequenceNumber> {
        Ok(self.inner().get_lowest_available_checkpoint())
    }

    fn get_committee(&self, epoch: EpochId) -> Result<Option<Arc<Committee>>> {
        self.inner()
            .get_committee_by_epoch(epoch)
            .cloned()
            .map(Arc::new)
            .pipe(Ok)
    }

    fn get_latest_checkpoint(&self) -> Result<VerifiedCheckpointMessage<MessageKind>> {
        todo!()
    }
}

impl WriteStore for SharedInMemoryStore {
    fn insert_checkpoint(&self, checkpoint: &VerifiedCheckpointMessage<MessageKind>) -> Result<()> {
        self.inner_mut().insert_checkpoint(checkpoint);
        Ok(())
    }

    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage<MessageKind>,
    ) -> Result<()> {
        self.inner_mut()
            .update_highest_synced_checkpoint(checkpoint);
        Ok(())
    }

    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage<MessageKind>,
    ) -> Result<()> {
        self.inner_mut()
            .update_highest_verified_checkpoint(checkpoint);
        Ok(())
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        self.inner_mut().insert_committee(new_committee);
        Ok(())
    }
}

impl SharedInMemoryStore {
    pub fn insert_certified_checkpoint(&self, checkpoint: &VerifiedCheckpointMessage<MessageKind>) {
        self.inner_mut().insert_certified_checkpoint(checkpoint);
    }
}

#[derive(Debug, Default)]
pub struct InMemoryStore {
    highest_verified_checkpoint: Option<(CheckpointSequenceNumber, CheckpointMessageDigest)>,
    highest_synced_checkpoint: Option<(CheckpointSequenceNumber, CheckpointMessageDigest)>,
    checkpoints: HashMap<CheckpointMessageDigest, VerifiedCheckpointMessage<MessageKind>>,
    contents_digest_to_sequence_number: HashMap<CheckpointContentsDigest, CheckpointSequenceNumber>,
    sequence_number_to_digest: HashMap<CheckpointSequenceNumber, CheckpointMessageDigest>,

    epoch_to_committee: Vec<Committee>,

    lowest_checkpoint_number: CheckpointSequenceNumber,
}

impl InMemoryStore {
    pub fn insert_genesis_state(
        &mut self,
        checkpoint: VerifiedCheckpointMessage<MessageKind>,
        committee: Committee,
    ) {
        self.insert_committee(committee);
        self.insert_checkpoint(&checkpoint);
        self.update_highest_synced_checkpoint(&checkpoint);
    }

    pub fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Option<&VerifiedCheckpointMessage<MessageKind>> {
        self.checkpoints.get(digest)
    }

    pub fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Option<&VerifiedCheckpointMessage<MessageKind>> {
        self.sequence_number_to_digest
            .get(&sequence_number)
            .and_then(|digest| self.get_checkpoint_by_digest(digest))
    }

    pub fn get_sequence_number_by_contents_digest(
        &self,
        digest: &CheckpointContentsDigest,
    ) -> Option<CheckpointSequenceNumber> {
        self.contents_digest_to_sequence_number.get(digest).copied()
    }

    pub fn get_highest_verified_checkpoint(
        &self,
    ) -> Option<&VerifiedCheckpointMessage<MessageKind>> {
        self.highest_verified_checkpoint
            .as_ref()
            .and_then(|(_, digest)| self.get_checkpoint_by_digest(digest))
    }

    pub fn get_highest_synced_checkpoint(&self) -> Option<&VerifiedCheckpointMessage<MessageKind>> {
        self.highest_synced_checkpoint
            .as_ref()
            .and_then(|(_, digest)| self.get_checkpoint_by_digest(digest))
    }

    pub fn get_lowest_available_checkpoint(&self) -> CheckpointSequenceNumber {
        self.lowest_checkpoint_number
    }

    pub fn set_lowest_available_checkpoint(
        &mut self,
        checkpoint_seq_num: CheckpointSequenceNumber,
    ) {
        self.lowest_checkpoint_number = checkpoint_seq_num;
    }

    pub fn insert_checkpoint(&mut self, checkpoint: &VerifiedCheckpointMessage<MessageKind>) {
        self.insert_certified_checkpoint(checkpoint);
        let digest = *checkpoint.digest();
        let sequence_number = *checkpoint.sequence_number();

        if Some(sequence_number) > self.highest_verified_checkpoint.map(|x| x.0) {
            self.highest_verified_checkpoint = Some((sequence_number, digest));
        }
    }

    // This function simulates Consensus inserts certified checkpoint into the checkpoint store
    // without bumping the highest_verified_checkpoint watermark.
    pub fn insert_certified_checkpoint(
        &mut self,
        checkpoint: &VerifiedCheckpointMessage<MessageKind>,
    ) {
        let digest = *checkpoint.digest();
        let sequence_number = *checkpoint.sequence_number();

        self.checkpoints.insert(digest, checkpoint.clone());
        self.sequence_number_to_digest
            .insert(sequence_number, digest);
    }

    pub fn update_highest_synced_checkpoint(
        &mut self,
        checkpoint: &VerifiedCheckpointMessage<MessageKind>,
    ) {
        if !self.checkpoints.contains_key(checkpoint.digest()) {
            panic!("store should already contain checkpoint");
        }
        if let Some(highest_synced_checkpoint) = self.highest_synced_checkpoint {
            if highest_synced_checkpoint.0 >= checkpoint.sequence_number {
                return;
            }
        }
        self.highest_synced_checkpoint =
            Some((*checkpoint.sequence_number(), *checkpoint.digest()));
    }

    pub fn update_highest_verified_checkpoint(
        &mut self,
        checkpoint: &VerifiedCheckpointMessage<MessageKind>,
    ) {
        if !self.checkpoints.contains_key(checkpoint.digest()) {
            panic!("store should already contain checkpoint");
        }
        if let Some(highest_verified_checkpoint) = self.highest_verified_checkpoint {
            if highest_verified_checkpoint.0 >= checkpoint.sequence_number {
                return;
            }
        }
        self.highest_verified_checkpoint =
            Some((*checkpoint.sequence_number(), *checkpoint.digest()));
    }

    pub fn checkpoints(
        &self,
    ) -> &HashMap<CheckpointMessageDigest, VerifiedCheckpointMessage<MessageKind>> {
        &self.checkpoints
    }

    pub fn checkpoint_sequence_number_to_digest(
        &self,
    ) -> &HashMap<CheckpointSequenceNumber, CheckpointMessageDigest> {
        &self.sequence_number_to_digest
    }

    pub fn get_committee_by_epoch(&self, epoch: EpochId) -> Option<&Committee> {
        self.epoch_to_committee.get(epoch as usize)
    }

    pub fn insert_committee(&mut self, committee: Committee) {
        let epoch = committee.epoch as usize;

        if self.epoch_to_committee.get(epoch).is_some() {
            return;
        }

        self.epoch_to_committee.push(committee);

        if self.epoch_to_committee.len() != epoch + 1 {
            error!("committee was inserted into EpochCommitteeMap out of order");
        }
    }
}

// This store only keeps last checkpoint in memory which is all we need
// for archive verification.
#[derive(Clone, Debug, Default)]
pub struct SingleCheckpointSharedInMemoryStore(SharedInMemoryStore);

impl SingleCheckpointSharedInMemoryStore {
    pub fn insert_genesis_state(
        &mut self,
        checkpoint: VerifiedCheckpointMessage<MessageKind>,
        committee: Committee,
    ) {
        let mut locked = self.0 .0.write().unwrap();
        locked.insert_genesis_state(checkpoint, committee);
    }
}

impl ReadStore for SingleCheckpointSharedInMemoryStore {
    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Result<Option<VerifiedCheckpointMessage<MessageKind>>> {
        self.0.get_checkpoint_by_digest(digest)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedCheckpointMessage<MessageKind>>> {
        self.0.get_checkpoint_by_sequence_number(sequence_number)
    }

    fn get_highest_verified_checkpoint(
        &self,
    ) -> Result<Option<VerifiedCheckpointMessage<MessageKind>>> {
        self.0.get_highest_verified_checkpoint()
    }

    fn get_highest_synced_checkpoint(
        &self,
    ) -> Result<Option<VerifiedCheckpointMessage<MessageKind>>> {
        self.0.get_highest_synced_checkpoint()
    }

    fn get_lowest_available_checkpoint(&self) -> Result<CheckpointSequenceNumber> {
        self.0.get_lowest_available_checkpoint()
    }

    fn get_committee(&self, epoch: EpochId) -> Result<Option<Arc<Committee>>> {
        self.0.get_committee(epoch)
    }

    fn get_latest_checkpoint(&self) -> Result<VerifiedCheckpointMessage<MessageKind>> {
        todo!()
    }
}

impl WriteStore for SingleCheckpointSharedInMemoryStore {
    fn insert_checkpoint(&self, checkpoint: &VerifiedCheckpointMessage<MessageKind>) -> Result<()> {
        {
            let mut locked = self.0 .0.write().unwrap();
            locked.checkpoints.clear();
            locked.sequence_number_to_digest.clear();
        }
        self.0.insert_checkpoint(checkpoint)?;
        Ok(())
    }

    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage<MessageKind>,
    ) -> Result<()> {
        self.0.update_highest_synced_checkpoint(checkpoint)?;
        Ok(())
    }

    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage<MessageKind>,
    ) -> Result<()> {
        self.0.update_highest_verified_checkpoint(checkpoint)?;
        Ok(())
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        self.0.insert_committee(new_committee)
    }
}
