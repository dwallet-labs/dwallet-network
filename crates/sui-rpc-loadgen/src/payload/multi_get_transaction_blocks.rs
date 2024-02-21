// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Result;

use crate::payload::{MultiGetTransactionBlocks, ProcessPayload, RpcCommandProcessor, SignerInfo};
use async_trait::async_trait;
use futures::future::join_all;

use super::validation::{check_transactions, chunk_entities};

#[async_trait]
impl<'a> ProcessPayload<'a, &'a MultiGetTransactionBlocks> for RpcCommandProcessor {
    async fn process(
        &'a self,
        op: &'a MultiGetTransactionBlocks,
        _signer_info: &Option<SignerInfo>,
    ) -> Result<()> {
        let clients = self.get_clients().await?;
        let digests = &op.digests;

        if op.digests.is_empty() {
            panic!("No digests provided, skipping query");
        }

        let chunks = chunk_entities(digests, None);
        let chunk_futures = chunks.into_iter().map(|chunk| {
            let clients = clients.clone();
            async move {
                check_transactions(&clients, &chunk, false, false).await;
            }
        });
        join_all(chunk_futures).await;

        Ok(())
    }
}
