// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::error::Result;
use crate::committee::{Committee, EpochId};
use crate::digests::CheckpointMessageDigest;
use crate::messages_checkpoint::{CheckpointSequenceNumber, VerifiedCheckpointMessage};
use std::sync::Arc;

pub trait ReadStore {
    //
    // Committee Getters
    //

    fn get_committee(&self, epoch: EpochId) -> Result<Option<Arc<Committee>>>;

    //
    // Checkpoint Getters
    //

    /// Get the latest available checkpoint. This is the latest executed checkpoint.
    ///
    /// All transactions, effects, objects and events are guaranteed to be available for the
    /// returned checkpoint.
    fn get_latest_checkpoint(&self, epoch: EpochId) -> Result<VerifiedCheckpointMessage>;

    /// Get the latest available checkpoint sequence number. This is the sequence number of the latest executed checkpoint.
    fn get_latest_checkpoint_sequence_number(
        &self,
        epoch: EpochId,
    ) -> Result<CheckpointSequenceNumber> {
        let latest_checkpoint = self.get_latest_checkpoint(epoch)?;
        Ok(*latest_checkpoint.sequence_number())
    }

    /// Get the highest verified checkpint. This is the highest checkpoint summary that has been
    /// verified, generally by state-sync. Only the checkpoint header is guaranteed to be present in
    /// the store.
    fn get_highest_verified_checkpoint(
        &self,
        epoch: EpochId,
    ) -> Result<Option<VerifiedCheckpointMessage>>;

    /// Get the highest synced checkpint. This is the highest checkpoint that has been synced from
    /// state-synce. The checkpoint header, contents, transactions, and effects of this checkpoint
    /// are guaranteed to be present in the store
    fn get_highest_synced_checkpoint(
        &self,
        epoch: EpochId,
    ) -> Result<Option<VerifiedCheckpointMessage>>;

    /// Lowest available checkpoint for which transaction and checkpoint data can be requested.
    ///
    /// Specifically this is the lowest checkpoint for which the following data can be requested:
    ///  - checkpoints
    ///  - transactions
    ///  - effects
    ///  - events
    ///
    /// For object availability see `get_lowest_available_checkpoint_objects`.
    fn get_lowest_available_checkpoint(&self, epoch: EpochId) -> Result<CheckpointSequenceNumber>;

    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Result<Option<VerifiedCheckpointMessage>>;

    fn get_checkpoint_by_sequence_number(
        &self,
        epoch: EpochId,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedCheckpointMessage>>;
}

impl<T: ReadStore + ?Sized> ReadStore for &T {
    fn get_committee(&self, epoch: EpochId) -> Result<Option<Arc<Committee>>> {
        (*self).get_committee(epoch)
    }

    fn get_latest_checkpoint(&self, epoch: EpochId) -> Result<VerifiedCheckpointMessage> {
        (*self).get_latest_checkpoint(epoch)
    }

    fn get_latest_checkpoint_sequence_number(
        &self,
        epoch: EpochId,
    ) -> Result<CheckpointSequenceNumber> {
        (*self).get_latest_checkpoint_sequence_number(epoch)
    }

    fn get_highest_verified_checkpoint(
        &self,
        epoch: EpochId,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        (*self).get_highest_verified_checkpoint(epoch)
    }

    fn get_highest_synced_checkpoint(
        &self,
        epoch: EpochId,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        (*self).get_highest_synced_checkpoint(epoch)
    }

    fn get_lowest_available_checkpoint(&self, epoch: EpochId) -> Result<CheckpointSequenceNumber> {
        (*self).get_lowest_available_checkpoint(epoch)
    }

    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        (*self).get_checkpoint_by_digest(digest)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        epoch: EpochId,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        (*self).get_checkpoint_by_sequence_number(epoch, sequence_number)
    }
}

impl<T: ReadStore + ?Sized> ReadStore for Box<T> {
    fn get_committee(&self, epoch: EpochId) -> Result<Option<Arc<Committee>>> {
        (**self).get_committee(epoch)
    }

    fn get_latest_checkpoint(&self, epoch: EpochId) -> Result<VerifiedCheckpointMessage> {
        (**self).get_latest_checkpoint(epoch)
    }

    fn get_latest_checkpoint_sequence_number(
        &self,
        epoch: EpochId,
    ) -> Result<CheckpointSequenceNumber> {
        (**self).get_latest_checkpoint_sequence_number(epoch)
    }

    fn get_highest_verified_checkpoint(
        &self,
        epoch: EpochId,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        (**self).get_highest_verified_checkpoint(epoch)
    }

    fn get_highest_synced_checkpoint(
        &self,
        epoch: EpochId,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        (**self).get_highest_synced_checkpoint(epoch)
    }

    fn get_lowest_available_checkpoint(&self, epoch: EpochId) -> Result<CheckpointSequenceNumber> {
        (**self).get_lowest_available_checkpoint(epoch)
    }

    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        (**self).get_checkpoint_by_digest(digest)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        epoch: EpochId,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        (**self).get_checkpoint_by_sequence_number(epoch, sequence_number)
    }
}

impl<T: ReadStore + ?Sized> ReadStore for Arc<T> {
    fn get_committee(&self, epoch: EpochId) -> Result<Option<Arc<Committee>>> {
        (**self).get_committee(epoch)
    }

    fn get_latest_checkpoint(&self, epoch: EpochId) -> Result<VerifiedCheckpointMessage> {
        (**self).get_latest_checkpoint(epoch)
    }

    fn get_latest_checkpoint_sequence_number(
        &self,
        epoch: EpochId,
    ) -> Result<CheckpointSequenceNumber> {
        (**self).get_latest_checkpoint_sequence_number(epoch)
    }

    fn get_highest_verified_checkpoint(
        &self,
        epoch: EpochId,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        (**self).get_highest_verified_checkpoint(epoch)
    }

    fn get_highest_synced_checkpoint(
        &self,
        epoch: EpochId,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        (**self).get_highest_synced_checkpoint(epoch)
    }

    fn get_lowest_available_checkpoint(&self, epoch: EpochId) -> Result<CheckpointSequenceNumber> {
        (**self).get_lowest_available_checkpoint(epoch)
    }

    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        (**self).get_checkpoint_by_digest(digest)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        epoch: EpochId,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedCheckpointMessage>> {
        (**self).get_checkpoint_by_sequence_number(epoch, sequence_number)
    }
}
