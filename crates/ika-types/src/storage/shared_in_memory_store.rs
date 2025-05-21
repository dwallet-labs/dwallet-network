// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::error::Result;
use crate::committee::{Committee, EpochId};
use crate::digests::{
    CheckpointContentsDigest, CheckpointMessageDigest, SystemCheckpointContentsDigest,
    SystemCheckpointDigest,
};
use crate::messages_dwallet_checkpoint::{CheckpointSequenceNumber, VerifiedCheckpointMessage};
use crate::messages_system_checkpoints::{
    SystemCheckpointSequenceNumber, VerifiedSystemCheckpoint,
};
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
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        self.inner()
            .get_checkpoint_by_digest(digest)
            .cloned()
            .pipe(Ok)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        self.inner()
            .get_checkpoint_by_sequence_number(sequence_number)
            .cloned()
            .pipe(Ok)
    }

    fn get_highest_verified_checkpoint(&self) -> Result<Option<VerifiedCheckpointMessage>> {
        self.inner()
            .get_highest_verified_checkpoint()
            .cloned()
            .pipe(Ok)
    }

    fn get_highest_synced_checkpoint(&self) -> Result<Option<VerifiedCheckpointMessage>> {
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

    fn get_latest_checkpoint(&self) -> Result<VerifiedCheckpointMessage> {
        todo!()
    }

    fn get_system_checkpoint_by_digest(
        &self,
        digest: &SystemCheckpointDigest,
    ) -> Result<Option<VerifiedSystemCheckpoint>> {
        self.inner()
            .get_system_checkpoint_by_digest(digest)
            .cloned()
            .pipe(Ok)
    }

    fn get_highest_verified_system_checkpoint(&self) -> Result<Option<VerifiedSystemCheckpoint>> {
        self.inner()
            .get_highest_verified_ika_system_checkpoint()
            .cloned()
            .pipe(Ok)
    }

    fn get_highest_synced_system_checkpoint(&self) -> Result<Option<VerifiedSystemCheckpoint>> {
        self.inner()
            .get_highest_synced_system_checkpoint()
            .cloned()
            .pipe(Ok)
    }

    fn get_lowest_available_system_checkpoint(&self) -> Result<SystemCheckpointSequenceNumber> {
        Ok(self.inner().get_lowest_available_system_checkpoint())
    }

    fn get_system_checkpoint_by_sequence_number(
        &self,
        sequence_number: SystemCheckpointSequenceNumber,
    ) -> Result<Option<VerifiedSystemCheckpoint>> {
        self.inner()
            .get_system_checkpoint_by_sequence_number(sequence_number)
            .cloned()
            .pipe(Ok)
    }

    fn get_latest_system_checkpoint(&self) -> Result<VerifiedSystemCheckpoint> {
        todo!()
    }
}

impl WriteStore for SharedInMemoryStore {
    fn insert_checkpoint(&self, checkpoint: &VerifiedCheckpointMessage) -> Result<()> {
        self.inner_mut().insert_checkpoint(checkpoint);
        Ok(())
    }

    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<()> {
        self.inner_mut()
            .update_highest_synced_checkpoint(checkpoint);
        Ok(())
    }

    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<()> {
        self.inner_mut()
            .update_highest_verified_checkpoint(checkpoint);
        Ok(())
    }

    fn insert_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()> {
        self.inner_mut()
            .insert_ika_system_checkpoint(ika_system_checkpoint);
        Ok(())
    }

    fn update_highest_synced_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()> {
        self.inner_mut()
            .update_highest_synced_system_checkpoint(ika_system_checkpoint);
        Ok(())
    }

    fn update_highest_verified_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()> {
        self.inner_mut()
            .update_highest_verified_system_checkpoint(ika_system_checkpoint);
        Ok(())
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        self.inner_mut().insert_committee(new_committee);
        Ok(())
    }
}

impl SharedInMemoryStore {
    pub fn insert_certified_checkpoint(&self, checkpoint: &VerifiedCheckpointMessage) {
        self.inner_mut().insert_certified_checkpoint(checkpoint);
    }

    pub fn insert_certified_ika_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedSystemCheckpoint,
    ) {
        self.inner_mut()
            .insert_certified_ika_system_checkpoint(ika_system_checkpoint);
    }
}

#[derive(Debug, Default)]
pub struct InMemoryStore {
    highest_verified_checkpoint: Option<(CheckpointSequenceNumber, CheckpointMessageDigest)>,
    highest_synced_checkpoint: Option<(CheckpointSequenceNumber, CheckpointMessageDigest)>,
    checkpoints: HashMap<CheckpointMessageDigest, VerifiedCheckpointMessage>,
    contents_digest_to_sequence_number: HashMap<CheckpointContentsDigest, CheckpointSequenceNumber>,
    sequence_number_to_digest: HashMap<CheckpointSequenceNumber, CheckpointMessageDigest>,

    epoch_to_committee: Vec<Committee>,

    lowest_checkpoint_number: CheckpointSequenceNumber,

    highest_verified_ika_system_checkpoint:
        Option<(SystemCheckpointSequenceNumber, SystemCheckpointDigest)>,
    highest_synced_ika_system_checkpoint:
        Option<(SystemCheckpointSequenceNumber, SystemCheckpointDigest)>,
    system_checkpoints: HashMap<SystemCheckpointDigest, VerifiedSystemCheckpoint>,
    ika_system_checkpoint_contents_digest_to_sequence_number:
        HashMap<SystemCheckpointContentsDigest, SystemCheckpointSequenceNumber>,
    ika_system_checkpoint_sequence_number_to_digest:
        HashMap<SystemCheckpointSequenceNumber, SystemCheckpointDigest>,
    lowest_ika_system_checkpoint_number: SystemCheckpointSequenceNumber,
}

impl InMemoryStore {
    pub fn insert_genesis_state(
        &mut self,
        checkpoint: VerifiedCheckpointMessage,
        ika_system_checkpoint: VerifiedSystemCheckpoint,
        committee: Committee,
    ) {
        self.insert_committee(committee);
        self.insert_checkpoint(&checkpoint);
        self.insert_ika_system_checkpoint(&ika_system_checkpoint);
        self.update_highest_synced_checkpoint(&checkpoint);
        self.update_highest_verified_system_checkpoint(&ika_system_checkpoint);
    }

    pub fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Option<&VerifiedCheckpointMessage> {
        self.checkpoints.get(digest)
    }

    pub fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Option<&VerifiedCheckpointMessage> {
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

    pub fn get_ika_system_checkpoint_sequence_number_by_contents_digest(
        &self,
        digest: &SystemCheckpointContentsDigest,
    ) -> Option<SystemCheckpointSequenceNumber> {
        self.ika_system_checkpoint_contents_digest_to_sequence_number
            .get(digest)
            .copied()
    }

    pub fn get_highest_verified_checkpoint(&self) -> Option<&VerifiedCheckpointMessage> {
        self.highest_verified_checkpoint
            .as_ref()
            .and_then(|(_, digest)| self.get_checkpoint_by_digest(digest))
    }

    pub fn get_highest_synced_checkpoint(&self) -> Option<&VerifiedCheckpointMessage> {
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

    pub fn insert_checkpoint(&mut self, checkpoint: &VerifiedCheckpointMessage) {
        self.insert_certified_checkpoint(checkpoint);
        let digest = *checkpoint.digest();
        let sequence_number = *checkpoint.sequence_number();

        if Some(sequence_number) > self.highest_verified_checkpoint.map(|x| x.0) {
            self.highest_verified_checkpoint = Some((sequence_number, digest));
        }
    }

    // This function simulates Consensus inserts certified checkpoint into the checkpoint store
    // without bumping the highest_verified_checkpoint watermark.
    pub fn insert_certified_checkpoint(&mut self, checkpoint: &VerifiedCheckpointMessage) {
        let digest = *checkpoint.digest();
        let sequence_number = *checkpoint.sequence_number();

        self.checkpoints.insert(digest, checkpoint.clone());
        self.sequence_number_to_digest
            .insert(sequence_number, digest);
    }

    pub fn update_highest_synced_checkpoint(&mut self, checkpoint: &VerifiedCheckpointMessage) {
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

    pub fn update_highest_verified_checkpoint(&mut self, checkpoint: &VerifiedCheckpointMessage) {
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

    pub fn checkpoints(&self) -> &HashMap<CheckpointMessageDigest, VerifiedCheckpointMessage> {
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

    pub fn get_system_checkpoint_by_digest(
        &self,
        digest: &SystemCheckpointDigest,
    ) -> Option<&VerifiedSystemCheckpoint> {
        self.system_checkpoints.get(digest)
    }

    pub fn get_system_checkpoint_by_sequence_number(
        &self,
        sequence_number: SystemCheckpointSequenceNumber,
    ) -> Option<&VerifiedSystemCheckpoint> {
        self.ika_system_checkpoint_sequence_number_to_digest
            .get(&sequence_number)
            .and_then(|digest| self.get_system_checkpoint_by_digest(digest))
    }

    pub fn get_highest_verified_ika_system_checkpoint(&self) -> Option<&VerifiedSystemCheckpoint> {
        self.highest_verified_ika_system_checkpoint
            .as_ref()
            .and_then(|(_, digest)| self.get_system_checkpoint_by_digest(digest))
    }

    pub fn get_highest_synced_system_checkpoint(&self) -> Option<&VerifiedSystemCheckpoint> {
        self.highest_synced_ika_system_checkpoint
            .as_ref()
            .and_then(|(_, digest)| self.get_system_checkpoint_by_digest(digest))
    }

    pub fn get_lowest_available_system_checkpoint(&self) -> SystemCheckpointSequenceNumber {
        self.lowest_ika_system_checkpoint_number
    }

    pub fn set_lowest_available_ika_system_checkpoint(
        &mut self,
        ika_system_checkpoint_seq_num: SystemCheckpointSequenceNumber,
    ) {
        self.lowest_ika_system_checkpoint_number = ika_system_checkpoint_seq_num;
    }

    pub fn insert_ika_system_checkpoint(
        &mut self,
        ika_system_checkpoint: &VerifiedSystemCheckpoint,
    ) {
        self.insert_certified_ika_system_checkpoint(ika_system_checkpoint);
        let digest = *ika_system_checkpoint.digest();
        let sequence_number = *ika_system_checkpoint.sequence_number();

        if Some(sequence_number) > self.highest_verified_ika_system_checkpoint.map(|x| x.0) {
            self.highest_verified_ika_system_checkpoint = Some((sequence_number, digest));
        }
    }

    // This function simulates Consensus inserts certified ika_system_checkpoint into the ika_system_checkpoint store
    // without bumping the highest_verified_ika_system_checkpoint watermark.
    pub fn insert_certified_ika_system_checkpoint(
        &mut self,
        ika_system_checkpoint: &VerifiedSystemCheckpoint,
    ) {
        let digest = *ika_system_checkpoint.digest();
        let sequence_number = *ika_system_checkpoint.sequence_number();

        self.system_checkpoints
            .insert(digest, ika_system_checkpoint.clone());
        self.ika_system_checkpoint_sequence_number_to_digest
            .insert(sequence_number, digest);
    }

    pub fn update_highest_synced_system_checkpoint(
        &mut self,
        ika_system_checkpoint: &VerifiedSystemCheckpoint,
    ) {
        if !self
            .system_checkpoints
            .contains_key(ika_system_checkpoint.digest())
        {
            panic!("store should already contain ika_system_checkpoint");
        }
        if let Some(highest_synced_ika_system_checkpoint) =
            self.highest_synced_ika_system_checkpoint
        {
            if highest_synced_ika_system_checkpoint.0 >= ika_system_checkpoint.sequence_number {
                return;
            }
        }
        self.highest_synced_ika_system_checkpoint = Some((
            *ika_system_checkpoint.sequence_number(),
            *ika_system_checkpoint.digest(),
        ));
    }

    pub fn update_highest_verified_system_checkpoint(
        &mut self,
        ika_system_checkpoint: &VerifiedSystemCheckpoint,
    ) {
        if !self
            .system_checkpoints
            .contains_key(ika_system_checkpoint.digest())
        {
            panic!("store should already contain ika_system_checkpoint");
        }
        if let Some(highest_verified_ika_system_checkpoint) =
            self.highest_verified_ika_system_checkpoint
        {
            if highest_verified_ika_system_checkpoint.0 >= ika_system_checkpoint.sequence_number {
                return;
            }
        }
        self.highest_verified_ika_system_checkpoint = Some((
            *ika_system_checkpoint.sequence_number(),
            *ika_system_checkpoint.digest(),
        ));
    }

    pub fn system_checkpoints(&self) -> &HashMap<SystemCheckpointDigest, VerifiedSystemCheckpoint> {
        &self.system_checkpoints
    }

    pub fn ika_system_checkpoint_sequence_number_to_digest(
        &self,
    ) -> &HashMap<SystemCheckpointSequenceNumber, SystemCheckpointDigest> {
        &self.ika_system_checkpoint_sequence_number_to_digest
    }
}

// This store only keeps last checkpoint in memory which is all we need
// for archive verification.
#[derive(Clone, Debug, Default)]
pub struct SingleCheckpointSharedInMemoryStore(SharedInMemoryStore);

impl SingleCheckpointSharedInMemoryStore {
    pub fn insert_genesis_state(
        &mut self,
        checkpoint: VerifiedCheckpointMessage,
        ika_system_checkpoint: VerifiedSystemCheckpoint,
        committee: Committee,
    ) {
        let mut locked = self.0 .0.write().unwrap();
        locked.insert_genesis_state(checkpoint, ika_system_checkpoint, committee);
    }
}

impl ReadStore for SingleCheckpointSharedInMemoryStore {
    fn get_committee(&self, epoch: EpochId) -> Result<Option<Arc<Committee>>> {
        self.0.get_committee(epoch)
    }

    fn get_latest_checkpoint(&self) -> Result<VerifiedCheckpointMessage> {
        todo!()
    }

    fn get_highest_verified_checkpoint(&self) -> Result<Option<VerifiedCheckpointMessage>> {
        self.0.get_highest_verified_checkpoint()
    }

    fn get_highest_synced_checkpoint(&self) -> Result<Option<VerifiedCheckpointMessage>> {
        self.0.get_highest_synced_checkpoint()
    }

    fn get_lowest_available_checkpoint(&self) -> Result<CheckpointSequenceNumber> {
        self.0.get_lowest_available_checkpoint()
    }

    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        self.0.get_checkpoint_by_digest(digest)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        self.0.get_checkpoint_by_sequence_number(sequence_number)
    }

    fn get_latest_system_checkpoint(&self) -> Result<VerifiedSystemCheckpoint> {
        todo!()
    }

    fn get_highest_verified_system_checkpoint(&self) -> Result<Option<VerifiedSystemCheckpoint>> {
        self.0.get_highest_verified_system_checkpoint()
    }

    fn get_highest_synced_system_checkpoint(&self) -> Result<Option<VerifiedSystemCheckpoint>> {
        self.0.get_highest_synced_system_checkpoint()
    }

    fn get_lowest_available_system_checkpoint(&self) -> Result<SystemCheckpointSequenceNumber> {
        self.0.get_lowest_available_system_checkpoint()
    }

    fn get_system_checkpoint_by_digest(
        &self,
        digest: &SystemCheckpointDigest,
    ) -> Result<Option<VerifiedSystemCheckpoint>> {
        self.0.get_system_checkpoint_by_digest(digest)
    }

    fn get_system_checkpoint_by_sequence_number(
        &self,
        sequence_number: SystemCheckpointSequenceNumber,
    ) -> Result<Option<VerifiedSystemCheckpoint>> {
        self.0
            .get_system_checkpoint_by_sequence_number(sequence_number)
    }
}

impl WriteStore for SingleCheckpointSharedInMemoryStore {
    fn insert_checkpoint(&self, checkpoint: &VerifiedCheckpointMessage) -> Result<()> {
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
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<()> {
        self.0.update_highest_synced_checkpoint(checkpoint)?;
        Ok(())
    }

    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<()> {
        self.0.update_highest_verified_checkpoint(checkpoint)?;
        Ok(())
    }

    fn insert_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()> {
        {
            let mut locked = self.0 .0.write().unwrap();
            locked.system_checkpoints.clear();
            locked
                .ika_system_checkpoint_sequence_number_to_digest
                .clear();
        }
        self.0.insert_system_checkpoint(ika_system_checkpoint)?;
        Ok(())
    }

    fn update_highest_synced_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()> {
        self.0
            .update_highest_synced_system_checkpoint(ika_system_checkpoint)?;
        Ok(())
    }

    fn update_highest_verified_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()> {
        self.0
            .update_highest_verified_system_checkpoint(ika_system_checkpoint)?;
        Ok(())
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        self.0.insert_committee(new_committee)
    }
}
