// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::sync::Arc;

use super::error::Result;
use crate::committee::Committee;
use crate::messages_dwallet_checkpoint::VerifiedDWalletCheckpointMessage;
use crate::messages_system_checkpoints::VerifiedSystemCheckpoint;
use crate::storage::ReadStore;

/// A trait for writing to a store
pub trait WriteStore: ReadStore {
    fn insert_dwallet_checkpoint(
        &self,
        dwallet_checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()>;

    fn update_highest_synced_dwallet_checkpoint(
        &self,
        dwallet_checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()>;
    fn update_highest_verified_dwallet_checkpoint(
        &self,
        dwallet_checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()>;

    fn insert_system_checkpoint(&self, system_checkpoint: &VerifiedSystemCheckpoint) -> Result<()>;
    fn update_highest_synced_system_checkpoint(
        &self,
        system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()>;
    fn update_highest_verified_system_checkpoint(
        &self,
        system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()>;

    fn insert_committee(&self, new_committee: Committee) -> Result<()>;
}

impl<T: WriteStore + ?Sized> WriteStore for &T {
    fn insert_dwallet_checkpoint(
        &self,
        dwallet_checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (*self).insert_dwallet_checkpoint(dwallet_checkpoint)
    }

    fn update_highest_synced_dwallet_checkpoint(
        &self,
        dwallet_checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (*self).update_highest_synced_dwallet_checkpoint(dwallet_checkpoint)
    }

    fn update_highest_verified_dwallet_checkpoint(
        &self,
        dwallet_checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (*self).update_highest_verified_dwallet_checkpoint(dwallet_checkpoint)
    }

    fn insert_system_checkpoint(&self, system_checkpoint: &VerifiedSystemCheckpoint) -> Result<()> {
        (*self).insert_system_checkpoint(system_checkpoint)
    }

    fn update_highest_synced_system_checkpoint(
        &self,
        system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()> {
        (*self).update_highest_synced_system_checkpoint(system_checkpoint)
    }

    fn update_highest_verified_system_checkpoint(
        &self,
        system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()> {
        (*self).update_highest_verified_system_checkpoint(system_checkpoint)
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        (*self).insert_committee(new_committee)
    }
}

impl<T: WriteStore + ?Sized> WriteStore for Box<T> {
    fn insert_dwallet_checkpoint(
        &self,
        dwallet_checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (**self).insert_dwallet_checkpoint(dwallet_checkpoint)
    }

    fn update_highest_synced_dwallet_checkpoint(
        &self,
        dwallet_checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (**self).update_highest_synced_dwallet_checkpoint(dwallet_checkpoint)
    }

    fn update_highest_verified_dwallet_checkpoint(
        &self,
        dwallet_checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (**self).update_highest_verified_dwallet_checkpoint(dwallet_checkpoint)
    }

    fn insert_system_checkpoint(&self, system_checkpoint: &VerifiedSystemCheckpoint) -> Result<()> {
        (**self).insert_system_checkpoint(system_checkpoint)
    }

    fn update_highest_synced_system_checkpoint(
        &self,
        system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()> {
        (**self).update_highest_synced_system_checkpoint(system_checkpoint)
    }

    fn update_highest_verified_system_checkpoint(
        &self,
        system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()> {
        (**self).update_highest_verified_system_checkpoint(system_checkpoint)
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        (**self).insert_committee(new_committee)
    }
}

impl<T: WriteStore + ?Sized> WriteStore for Arc<T> {
    fn insert_dwallet_checkpoint(
        &self,
        dwallet_checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (**self).insert_dwallet_checkpoint(dwallet_checkpoint)
    }

    fn update_highest_synced_dwallet_checkpoint(
        &self,
        dwallet_checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (**self).update_highest_synced_dwallet_checkpoint(dwallet_checkpoint)
    }

    fn update_highest_verified_dwallet_checkpoint(
        &self,
        dwallet_checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (**self).update_highest_verified_dwallet_checkpoint(dwallet_checkpoint)
    }

    fn insert_system_checkpoint(&self, system_checkpoint: &VerifiedSystemCheckpoint) -> Result<()> {
        (**self).insert_system_checkpoint(system_checkpoint)
    }

    fn update_highest_synced_system_checkpoint(
        &self,
        system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()> {
        (**self).update_highest_synced_system_checkpoint(system_checkpoint)
    }

    fn update_highest_verified_system_checkpoint(
        &self,
        system_checkpoint: &VerifiedSystemCheckpoint,
    ) -> Result<()> {
        (**self).update_highest_verified_system_checkpoint(system_checkpoint)
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        (**self).insert_committee(new_committee)
    }
}
