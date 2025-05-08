// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::error::Result;
use crate::committee::{Committee, EpochId};
use crate::digests::CheckpointMessageDigest;
use crate::message::DwalletCheckpointMessageKind;
use crate::messages_checkpoint::{
    CheckpointSequenceNumber, VerifiedCheckpointMessage, VerifiedDWalletCheckpointMessage,
};
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
    fn get_latest_checkpoint(&self) -> Result<VerifiedDWalletCheckpointMessage>;

    /// Get the latest available checkpoint sequence number. This is the sequence number of the latest executed checkpoint.
    fn get_latest_checkpoint_sequence_number(&self) -> Result<CheckpointSequenceNumber> {
        let latest_checkpoint = self.get_latest_checkpoint()?;
        Ok(*latest_checkpoint.sequence_number())
    }

    /// Get the epoch of the latest checkpoint
    fn get_latest_epoch_id(&self) -> Result<EpochId> {
        let latest_checkpoint = self.get_latest_checkpoint()?;
        Ok(latest_checkpoint.epoch())
    }

    /// Get the highest verified checkpint. This is the highest checkpoint summary that has been
    /// verified, generally by state-sync. Only the checkpoint header is guaranteed to be present in
    /// the store.
    fn get_highest_verified_checkpoint(&self) -> Result<Option<VerifiedDWalletCheckpointMessage>>;

    /// Get the highest synced checkpint. This is the highest checkpoint that has been synced from
    /// state-synce. The checkpoint header, contents, transactions, and effects of this checkpoint
    /// are guaranteed to be present in the store
    fn get_highest_synced_checkpoint(&self) -> Result<Option<VerifiedDWalletCheckpointMessage>>;

    /// Lowest available checkpoint for which transaction and checkpoint data can be requested.
    ///
    /// Specifically this is the lowest checkpoint for which the following data can be requested:
    ///  - checkpoints
    ///  - transactions
    ///  - effects
    ///  - events
    ///
    /// For object availability see `get_lowest_available_checkpoint_objects`.
    fn get_lowest_available_checkpoint(&self) -> Result<CheckpointSequenceNumber>;

    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Result<Option<VerifiedDWalletCheckpointMessage>>;

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedDWalletCheckpointMessage>>;
}

impl<T: ReadStore + ?Sized> ReadStore for &T {
    fn get_committee(&self, epoch: EpochId) -> Result<Option<Arc<Committee>>> {
        (*self).get_committee(epoch)
    }

    fn get_latest_checkpoint(&self) -> Result<VerifiedDWalletCheckpointMessage> {
        (*self).get_latest_checkpoint()
    }

    fn get_latest_checkpoint_sequence_number(&self) -> Result<CheckpointSequenceNumber> {
        (*self).get_latest_checkpoint_sequence_number()
    }

    fn get_latest_epoch_id(&self) -> Result<EpochId> {
        (*self).get_latest_epoch_id()
    }

    fn get_highest_verified_checkpoint(&self) -> Result<Option<VerifiedDWalletCheckpointMessage>> {
        (*self).get_highest_verified_checkpoint()
    }

    fn get_highest_synced_checkpoint(&self) -> Result<Option<VerifiedDWalletCheckpointMessage>> {
        (*self).get_highest_synced_checkpoint()
    }

    fn get_lowest_available_checkpoint(&self) -> Result<CheckpointSequenceNumber> {
        (*self).get_lowest_available_checkpoint()
    }

    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Result<Option<VerifiedDWalletCheckpointMessage>> {
        (*self).get_checkpoint_by_digest(digest)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedDWalletCheckpointMessage>> {
        (*self).get_checkpoint_by_sequence_number(sequence_number)
    }
}

impl<T: ReadStore + ?Sized> ReadStore for Box<T> {
    fn get_committee(&self, epoch: EpochId) -> Result<Option<Arc<Committee>>> {
        (**self).get_committee(epoch)
    }

    fn get_latest_checkpoint(&self) -> Result<VerifiedDWalletCheckpointMessage> {
        (**self).get_latest_checkpoint()
    }

    fn get_latest_checkpoint_sequence_number(&self) -> Result<CheckpointSequenceNumber> {
        (**self).get_latest_checkpoint_sequence_number()
    }

    fn get_latest_epoch_id(&self) -> Result<EpochId> {
        (**self).get_latest_epoch_id()
    }

    fn get_highest_verified_checkpoint(&self) -> Result<Option<VerifiedDWalletCheckpointMessage>> {
        (**self).get_highest_verified_checkpoint()
    }

    fn get_highest_synced_checkpoint(&self) -> Result<Option<VerifiedDWalletCheckpointMessage>> {
        (**self).get_highest_synced_checkpoint()
    }

    fn get_lowest_available_checkpoint(&self) -> Result<CheckpointSequenceNumber> {
        (**self).get_lowest_available_checkpoint()
    }

    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Result<Option<VerifiedDWalletCheckpointMessage>> {
        (**self).get_checkpoint_by_digest(digest)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedDWalletCheckpointMessage>> {
        (**self).get_checkpoint_by_sequence_number(sequence_number)
    }
}

impl<T: ReadStore + ?Sized> ReadStore for Arc<T> {
    fn get_committee(&self, epoch: EpochId) -> Result<Option<Arc<Committee>>> {
        (**self).get_committee(epoch)
    }

    fn get_latest_checkpoint(&self) -> Result<VerifiedDWalletCheckpointMessage> {
        (**self).get_latest_checkpoint()
    }

    fn get_latest_checkpoint_sequence_number(&self) -> Result<CheckpointSequenceNumber> {
        (**self).get_latest_checkpoint_sequence_number()
    }

    fn get_latest_epoch_id(&self) -> Result<EpochId> {
        (**self).get_latest_epoch_id()
    }

    fn get_highest_verified_checkpoint(&self) -> Result<Option<VerifiedDWalletCheckpointMessage>> {
        (**self).get_highest_verified_checkpoint()
    }

    fn get_highest_synced_checkpoint(&self) -> Result<Option<VerifiedDWalletCheckpointMessage>> {
        (**self).get_highest_synced_checkpoint()
    }

    fn get_lowest_available_checkpoint(&self) -> Result<CheckpointSequenceNumber> {
        (**self).get_lowest_available_checkpoint()
    }

    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointMessageDigest,
    ) -> Result<Option<VerifiedDWalletCheckpointMessage>> {
        (**self).get_checkpoint_by_digest(digest)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedDWalletCheckpointMessage>> {
        (**self).get_checkpoint_by_sequence_number(sequence_number)
    }
}
