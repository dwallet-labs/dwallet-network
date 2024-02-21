// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use serde::{Deserialize, Serialize};

use crate::id::UID;

/// Rust version of the Move sui::versioned::Versioned type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Versioned {
    pub id: UID,
    pub version: u64,
}
