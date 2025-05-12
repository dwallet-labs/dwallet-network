// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::sync::Arc;

use super::error::Result;
use crate::committee::Committee;
use crate::messages_checkpoint::VerifiedCheckpointMessage;
use crate::messages_params_messages::VerifiedParamsMessage;
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

    fn insert_params_message(&self, params_message: &VerifiedParamsMessage) -> Result<()>;
    fn update_highest_synced_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<()>;
    fn update_highest_verified_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
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

    fn insert_params_message(&self, params_message: &VerifiedParamsMessage) -> Result<()> {
        (*self).insert_params_message(params_message)
    }

    fn update_highest_synced_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<()> {
        (*self).update_highest_synced_params_message(params_message)
    }

    fn update_highest_verified_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<()> {
        (*self).update_highest_verified_params_message(params_message)
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

    fn insert_params_message(&self, params_message: &VerifiedParamsMessage) -> Result<()> {
        (**self).insert_params_message(params_message)
    }

    fn update_highest_synced_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<()> {
        (**self).update_highest_synced_params_message(params_message)
    }

    fn update_highest_verified_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<()> {
        (**self).update_highest_verified_params_message(params_message)
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

    fn insert_params_message(&self, params_message: &VerifiedParamsMessage) -> Result<()> {
        (**self).insert_params_message(params_message)
    }

    fn update_highest_synced_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<()> {
        (**self).update_highest_synced_params_message(params_message)
    }

    fn update_highest_verified_params_message(
        &self,
        params_message: &VerifiedParamsMessage,
    ) -> Result<()> {
        (**self).update_highest_verified_params_message(params_message)
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<()> {
        (**self).insert_committee(new_committee)
    }
}
