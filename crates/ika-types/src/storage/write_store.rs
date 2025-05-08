// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::sync::Arc;

use super::error::Result;
use crate::committee::Committee;
use crate::message::DwalletCheckpointMessageKind;
use crate::messages_checkpoint::{VerifiedCheckpointMessage, VerifiedDWalletCheckpointMessage};
use crate::storage::ReadStore;

pub trait WriteStore: ReadStore {
    fn insert_checkpoint(&self, checkpoint: &VerifiedDWalletCheckpointMessage) -> Result<()>;
    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()>;
    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()>;

    fn insert_committee(&self, new_committee: Committee) -> Result<()>;
}

impl<T: WriteStore + ?Sized> WriteStore for &T {
    fn insert_checkpoint(&self, checkpoint: &VerifiedDWalletCheckpointMessage) -> Result<()> {
        (*self).insert_checkpoint(checkpoint)
    }

    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (*self).update_highest_synced_checkpoint(checkpoint)
    }

    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (*self).update_highest_verified_checkpoint(checkpoint)
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        (*self).insert_committee(new_committee)
    }
}

impl<T: WriteStore + ?Sized> WriteStore for Box<T> {
    fn insert_checkpoint(&self, checkpoint: &VerifiedDWalletCheckpointMessage) -> Result<()> {
        (**self).insert_checkpoint(checkpoint)
    }

    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (**self).update_highest_synced_checkpoint(checkpoint)
    }

    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (**self).update_highest_verified_checkpoint(checkpoint)
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        (**self).insert_committee(new_committee)
    }
}

impl<T: WriteStore + ?Sized> WriteStore for Arc<T> {
    fn insert_checkpoint(&self, checkpoint: &VerifiedDWalletCheckpointMessage) -> Result<()> {
        (**self).insert_checkpoint(checkpoint)
    }

    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (**self).update_highest_synced_checkpoint(checkpoint)
    }

    fn update_highest_verified_checkpoint(
        &self,
        checkpoint: &VerifiedDWalletCheckpointMessage,
    ) -> Result<()> {
        (**self).update_highest_verified_checkpoint(checkpoint)
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        (**self).insert_committee(new_committee)
    }
}
