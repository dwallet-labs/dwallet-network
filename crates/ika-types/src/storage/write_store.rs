// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::sync::Arc;

use super::error::Result;
use crate::committee::Committee;
use crate::messages_checkpoint::VerifiedCheckpointMessage;
use crate::messages_ika_system_checkpoints::VerifiedIkaSystemCheckpoint;
use crate::storage::ReadStore;

pub trait WriteStore: ReadStore {
    fn insert_checkpoint(&self, checkpoint: &VerifiedCheckpointMessage) -> Result<()>;
    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<()>;
    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<()>;

    fn insert_ika_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedIkaSystemCheckpoint,
    ) -> Result<()>;
    fn update_highest_synced_ika_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedIkaSystemCheckpoint,
    ) -> Result<()>;
    fn update_highest_verified_ika_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedIkaSystemCheckpoint,
    ) -> Result<()>;

    fn insert_committee(&self, new_committee: Committee) -> Result<()>;
}

impl<T: WriteStore + ?Sized> WriteStore for &T {
    fn insert_checkpoint(&self, checkpoint: &VerifiedCheckpointMessage) -> Result<()> {
        (*self).insert_checkpoint(checkpoint)
    }

    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<()> {
        (*self).update_highest_synced_checkpoint(checkpoint)
    }

    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<()> {
        (*self).update_highest_verified_checkpoint(checkpoint)
    }

    fn insert_ika_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedIkaSystemCheckpoint,
    ) -> Result<()> {
        (*self).insert_ika_system_checkpoint(ika_system_checkpoint)
    }

    fn update_highest_synced_ika_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedIkaSystemCheckpoint,
    ) -> Result<()> {
        (*self).update_highest_synced_ika_system_checkpoint(ika_system_checkpoint)
    }

    fn update_highest_verified_ika_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedIkaSystemCheckpoint,
    ) -> Result<()> {
        (*self).update_highest_verified_ika_system_checkpoint(ika_system_checkpoint)
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        (*self).insert_committee(new_committee)
    }
}

impl<T: WriteStore + ?Sized> WriteStore for Box<T> {
    fn insert_checkpoint(&self, checkpoint: &VerifiedCheckpointMessage) -> Result<()> {
        (**self).insert_checkpoint(checkpoint)
    }

    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<()> {
        (**self).update_highest_synced_checkpoint(checkpoint)
    }

    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<()> {
        (**self).update_highest_verified_checkpoint(checkpoint)
    }

    fn insert_ika_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedIkaSystemCheckpoint,
    ) -> Result<()> {
        (**self).insert_ika_system_checkpoint(ika_system_checkpoint)
    }

    fn update_highest_synced_ika_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedIkaSystemCheckpoint,
    ) -> Result<()> {
        (**self).update_highest_synced_ika_system_checkpoint(ika_system_checkpoint)
    }

    fn update_highest_verified_ika_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedIkaSystemCheckpoint,
    ) -> Result<()> {
        (**self).update_highest_verified_ika_system_checkpoint(ika_system_checkpoint)
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        (**self).insert_committee(new_committee)
    }
}

impl<T: WriteStore + ?Sized> WriteStore for Arc<T> {
    fn insert_checkpoint(&self, checkpoint: &VerifiedCheckpointMessage) -> Result<()> {
        (**self).insert_checkpoint(checkpoint)
    }

    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<()> {
        (**self).update_highest_synced_checkpoint(checkpoint)
    }

    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpointMessage,
    ) -> Result<()> {
        (**self).update_highest_verified_checkpoint(checkpoint)
    }

    fn insert_ika_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedIkaSystemCheckpoint,
    ) -> Result<()> {
        (**self).insert_ika_system_checkpoint(ika_system_checkpoint)
    }

    fn update_highest_synced_ika_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedIkaSystemCheckpoint,
    ) -> Result<()> {
        (**self).update_highest_synced_ika_system_checkpoint(ika_system_checkpoint)
    }

    fn update_highest_verified_ika_system_checkpoint(
        &self,
        ika_system_checkpoint: &VerifiedIkaSystemCheckpoint,
    ) -> Result<()> {
        (**self).update_highest_verified_ika_system_checkpoint(ika_system_checkpoint)
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        (**self).insert_committee(new_committee)
    }
}
