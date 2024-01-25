// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Result;

use crate::payload::validation::check_objects;
use crate::payload::{MultiGetObjects, ProcessPayload, RpcCommandProcessor, SignerInfo};
use async_trait::async_trait;

#[async_trait]
impl<'a> ProcessPayload<'a, &'a MultiGetObjects> for RpcCommandProcessor {
    async fn process(
        &'a self,
        op: &'a MultiGetObjects,
        _signer_info: &Option<SignerInfo>,
    ) -> Result<()> {
        let clients = self.get_clients().await?;
        check_objects(&clients, &op.object_ids, false).await;
        Ok(())
    }
}
