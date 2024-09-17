// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::id::UID;
use serde::{Deserialize, Serialize};

/// Rust version of the Move pera::versioned::Versioned type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Versioned {
    pub id: UID,
    pub version: u64,
}
