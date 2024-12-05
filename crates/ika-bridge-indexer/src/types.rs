// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use ika_json_rpc_types::{
    IkaTransactionBlockEffects, IkaTransactionBlockEvents, IkaTransactionBlockResponse,
};
use ika_types::digests::TransactionDigest;

#[derive(Clone)]
pub struct RetrievedTransaction {
    pub tx_digest: TransactionDigest,
    pub events: IkaTransactionBlockEvents,
    pub checkpoint: u64,
    pub timestamp_ms: u64,
    pub effects: IkaTransactionBlockEffects,
}

impl TryFrom<IkaTransactionBlockResponse> for RetrievedTransaction {
    type Error = anyhow::Error;
    fn try_from(response: IkaTransactionBlockResponse) -> Result<Self, Self::Error> {
        Ok(RetrievedTransaction {
            tx_digest: response.digest,
            events: response
                .events
                .ok_or(anyhow::anyhow!("missing events in responses"))?,
            checkpoint: response
                .checkpoint
                .ok_or(anyhow::anyhow!("missing checkpoint in responses"))?,
            timestamp_ms: response
                .timestamp_ms
                .ok_or(anyhow::anyhow!("missing timestamp_ms in responses"))?,
            effects: response
                .effects
                .ok_or(anyhow::anyhow!("missing effects in responses"))?,
        })
    }
}
