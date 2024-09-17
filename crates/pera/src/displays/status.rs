// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::displays::Pretty;
use pera_json_rpc_types::PeraExecutionStatus::{self, Failure, Success};
use std::fmt::{Display, Formatter};

impl<'a> Display for Pretty<'a, PeraExecutionStatus> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Pretty(status) = self;

        let output = match status {
            Success => "success".to_string(),
            Failure { error } => format!("failed due to {error}"),
        };

        write!(f, "{}", output)
    }
}
