// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use pera_json_rpc_types::{
    PeraTransactionBlockEffects, PeraTransactionBlockEvents, PeraTransactionBlockResponse,
};
use pera_types::digests::TransactionDigest;

#[derive(Clone)]
pub struct RetrievedTransaction {
    pub tx_digest: TransactionDigest,
    pub events: PeraTransactionBlockEvents,
    pub checkpoint: u64,
    pub timestamp_ms: u64,
    pub effects: PeraTransactionBlockEffects,
}

impl TryFrom<PeraTransactionBlockResponse> for RetrievedTransaction {
    type Error = anyhow::Error;
    fn try_from(response: PeraTransactionBlockResponse) -> Result<Self, Self::Error> {
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
