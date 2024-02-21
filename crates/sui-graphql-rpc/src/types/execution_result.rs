// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use async_graphql::*;

#[derive(SimpleObject, Clone)]
pub(crate) struct ExecutionResult {
    /// The errors field captures any errors that occurred during execution
    pub errors: Option<Vec<String>>,

    /// The digest field captures the digest of the transaction block
    pub digest: String,
    // TODO: add support for `TransactionBlockEffects` field
}
