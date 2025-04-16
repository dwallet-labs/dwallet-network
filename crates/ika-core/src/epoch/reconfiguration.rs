// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ReconfigCertStatus {
    AcceptAllCerts,

    // User certs rejected, but we still accept certs received through consensus.
    RejectUserCerts,

    // All certs rejected, including ones received through consensus.
    // But we still accept other transactions from consensus (e.g. randomness DKG)
    // and process previously-deferred transactions.
    RejectAllCerts,

    // All tx rejected, including system tx.
    RejectAllTx,
}