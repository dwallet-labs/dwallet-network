// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::checkpoint::{Checkpoint, CheckpointId};
use async_graphql::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AvailableRange {
    pub first: u64,
    pub last: u64,
}

// TODO: do both in one query?
#[Object]
impl AvailableRange {
    async fn first(&self, ctx: &Context<'_>) -> Result<Option<Checkpoint>> {
        Checkpoint::query(ctx.data_unchecked(), CheckpointId::by_seq_num(self.first))
            .await
            .extend()
    }

    async fn last(&self, ctx: &Context<'_>) -> Result<Option<Checkpoint>> {
        Checkpoint::query(ctx.data_unchecked(), CheckpointId::by_seq_num(self.last))
            .await
            .extend()
    }
}
