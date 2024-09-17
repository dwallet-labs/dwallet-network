// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use serde::{Deserialize, Serialize};
use pera_types::base_types::PeraAddress;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FaucetRequest {
    FixedAmountRequest(FixedAmountRequest),
    GetBatchSendStatusRequest(GetBatchSendStatusRequest),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FixedAmountRequest {
    pub recipient: PeraAddress,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBatchSendStatusRequest {
    pub task_id: String,
}

impl FaucetRequest {
    pub fn new_fixed_amount_request(recipient: impl Into<PeraAddress>) -> Self {
        Self::FixedAmountRequest(FixedAmountRequest {
            recipient: recipient.into(),
        })
    }

    pub fn new_get_batch_send_status_request(task_id: impl Into<String>) -> Self {
        Self::GetBatchSendStatusRequest(GetBatchSendStatusRequest {
            task_id: task_id.into(),
        })
    }
}
