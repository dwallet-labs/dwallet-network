// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::error::Result;
use crate::committee::{Committee, EpochId};
use crate::digests::{
    CheckpointContentsDigest, CheckpointMessageDigest, ParamsMessageContentsDigest,
    ParamsMessageDigest,
};
use crate::messages_checkpoint::{CheckpointSequenceNumber, VerifiedCheckpointMessage};
use crate::messages_params_messages::{ParamsMessageSequenceNumber, VerifiedParamsMessage};
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

    fn get_params_message_by_digest(
        &self,
        digest: &ParamsMessageDigest,
    ) -> Result<Option<VerifiedParamsMessage>> {
        self.inner()
            .get_params_message_by_digest(digest)
            .cloned()
            .pipe(Ok)
    }

    fn get_highest_verified_params_message(&self) -> Result<Option<VerifiedParamsMessage>> {
        self.inner()
            .get_highest_verified_params_message()
            .cloned()
            .pipe(Ok)
    }

    fn get_highest_synced_params_message(&self) -> Result<Option<VerifiedParamsMessage>> {
        self.inner()
            .get_highest_synced_params_message()
            .cloned()
            .pipe(Ok)
    }

    fn get_lowest_available_params_message(&self) -> Result<ParamsMessageSequenceNumber> {
        Ok(self.inner().get_lowest_available_params_message())
    }

    fn get_params_message_by_sequence_number(
        &self,
        sequence_number: ParamsMessageSequenceNumber,
    ) -> Result<Option<VerifiedParamsMessage>> {
        self.inner()
            .get_params_message_by_sequence_number(sequence_number)
            .cloned()
            .pipe(Ok)
    }

    fn get_latest_params_message(&self) -> Result<VerifiedParamsMessage> {
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

    fn insert_params_message(&self, params_message: &VerifiedParamsMessage) -> Result<()> {
        self.inner_mut().insert_params_message(params_message);
        Ok(())
    }

    fn update_highest_synced_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<()> {
        self.inner_mut()
            .update_highest_synced_params_message(params_message);
        Ok(())
    }

    fn update_highest_verified_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<()> {
        self.inner_mut()
            .update_highest_verified_params_message(params_message);
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

    pub fn insert_certified_params_message(&self, params_message: &VerifiedParamsMessage) {
        self.inner_mut()
            .insert_certified_params_message(params_message);
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

    highest_verified_params_message: Option<(ParamsMessageSequenceNumber, ParamsMessageDigest)>,
    highest_synced_params_message: Option<(ParamsMessageSequenceNumber, ParamsMessageDigest)>,
    params_messages: HashMap<ParamsMessageDigest, VerifiedParamsMessage>,
    params_message_contents_digest_to_sequence_number:
        HashMap<ParamsMessageContentsDigest, ParamsMessageSequenceNumber>,
    params_message_sequence_number_to_digest:
        HashMap<ParamsMessageSequenceNumber, ParamsMessageDigest>,
    lowest_params_message_number: ParamsMessageSequenceNumber,
}

impl InMemoryStore {
    pub fn insert_genesis_state(
        &mut self,
        checkpoint: VerifiedCheckpointMessage,
        params_message: VerifiedParamsMessage,
        committee: Committee,
    ) {
        self.insert_committee(committee);
        self.insert_checkpoint(&checkpoint);
        self.insert_params_message(&params_message);
        self.update_highest_synced_checkpoint(&checkpoint);
        self.update_highest_verified_params_message(&params_message);
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

    pub fn get_params_message_sequence_number_by_contents_digest(
        &self,
        digest: &ParamsMessageContentsDigest,
    ) -> Option<ParamsMessageSequenceNumber> {
        self.params_message_contents_digest_to_sequence_number
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

    pub fn get_params_message_by_digest(
        &self,
        digest: &ParamsMessageDigest,
    ) -> Option<&VerifiedParamsMessage> {
        self.params_messages.get(digest)
    }

    pub fn get_params_message_by_sequence_number(
        &self,
        sequence_number: ParamsMessageSequenceNumber,
    ) -> Option<&VerifiedParamsMessage> {
        self.params_message_sequence_number_to_digest
            .get(&sequence_number)
            .and_then(|digest| self.get_params_message_by_digest(digest))
    }

    pub fn get_highest_verified_params_message(&self) -> Option<&VerifiedParamsMessage> {
        self.highest_verified_params_message
            .as_ref()
            .and_then(|(_, digest)| self.get_params_message_by_digest(digest))
    }

    pub fn get_highest_synced_params_message(&self) -> Option<&VerifiedParamsMessage> {
        self.highest_synced_params_message
            .as_ref()
            .and_then(|(_, digest)| self.get_params_message_by_digest(digest))
    }

    pub fn get_lowest_available_params_message(&self) -> ParamsMessageSequenceNumber {
        self.lowest_params_message_number
    }

    pub fn set_lowest_available_params_message(
        &mut self,
        params_message_seq_num: ParamsMessageSequenceNumber,
    ) {
        self.lowest_params_message_number = params_message_seq_num;
    }

    pub fn insert_params_message(&mut self, params_message: &VerifiedParamsMessage) {
        self.insert_certified_params_message(params_message);
        let digest = *params_message.digest();
        let sequence_number = *params_message.sequence_number();

        if Some(sequence_number) > self.highest_verified_params_message.map(|x| x.0) {
            self.highest_verified_params_message = Some((sequence_number, digest));
        }
    }

    // This function simulates Consensus inserts certified params_message into the params_message store
    // without bumping the highest_verified_params_message watermark.
    pub fn insert_certified_params_message(&mut self, params_message: &VerifiedParamsMessage) {
        let digest = *params_message.digest();
        let sequence_number = *params_message.sequence_number();

        self.params_messages.insert(digest, params_message.clone());
        self.params_message_sequence_number_to_digest
            .insert(sequence_number, digest);
    }

    pub fn update_highest_synced_params_message(&mut self, params_message: &VerifiedParamsMessage) {
        if !self.params_messages.contains_key(params_message.digest()) {
            panic!("store should already contain params_message");
        }
        if let Some(highest_synced_params_message) = self.highest_synced_params_message {
            if highest_synced_params_message.0 >= params_message.sequence_number {
                return;
            }
        }
        self.highest_synced_params_message =
            Some((*params_message.sequence_number(), *params_message.digest()));
    }

    pub fn update_highest_verified_params_message(
        &mut self,
        params_message: &VerifiedParamsMessage,
    ) {
        if !self.params_messages.contains_key(params_message.digest()) {
            panic!("store should already contain params_message");
        }
        if let Some(highest_verified_params_message) = self.highest_verified_params_message {
            if highest_verified_params_message.0 >= params_message.sequence_number {
                return;
            }
        }
        self.highest_verified_params_message =
            Some((*params_message.sequence_number(), *params_message.digest()));
    }

    pub fn params_messages(&self) -> &HashMap<ParamsMessageDigest, VerifiedParamsMessage> {
        &self.params_messages
    }

    pub fn params_message_sequence_number_to_digest(
        &self,
    ) -> &HashMap<ParamsMessageSequenceNumber, ParamsMessageDigest> {
        &self.params_message_sequence_number_to_digest
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
        params_message: VerifiedParamsMessage,
        committee: Committee,
    ) {
        let mut locked = self.0 .0.write().unwrap();
        locked.insert_genesis_state(checkpoint, params_message, committee);
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

    fn get_latest_params_message(&self) -> Result<VerifiedParamsMessage> {
        todo!()
    }

    fn get_highest_verified_params_message(&self) -> Result<Option<VerifiedParamsMessage>> {
        self.0.get_highest_verified_params_message()
    }

    fn get_highest_synced_params_message(&self) -> Result<Option<VerifiedParamsMessage>> {
        self.0.get_highest_synced_params_message()
    }

    fn get_lowest_available_params_message(&self) -> Result<ParamsMessageSequenceNumber> {
        self.0.get_lowest_available_params_message()
    }

    fn get_params_message_by_digest(
        &self,
        digest: &ParamsMessageDigest,
    ) -> Result<Option<VerifiedParamsMessage>> {
        self.0.get_params_message_by_digest(digest)
    }

    fn get_params_message_by_sequence_number(
        &self,
        sequence_number: ParamsMessageSequenceNumber,
    ) -> Result<Option<VerifiedParamsMessage>> {
        self.0
            .get_params_message_by_sequence_number(sequence_number)
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

    fn insert_params_message(&self, params_message: &VerifiedParamsMessage) -> Result<()> {
        {
            let mut locked = self.0 .0.write().unwrap();
            locked.params_messages.clear();
            locked.params_message_sequence_number_to_digest.clear();
        }
        self.0.insert_params_message(params_message)?;
        Ok(())
    }

    fn update_highest_synced_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<()> {
        self.0
            .update_highest_synced_params_message(params_message)?;
        Ok(())
    }

    fn update_highest_verified_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<()> {
        self.0
            .update_highest_verified_params_message(params_message)?;
        Ok(())
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        self.0.insert_committee(new_committee)
    }
}
