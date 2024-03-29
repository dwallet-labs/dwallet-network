// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BSD-3-Clause-Clear

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// An access to local or global state
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Access {
    /// Read via RHS * or exists
    Read,
    /// Written via LHS *, move_to, or move_from
    Write,
    /// Could be read or written
    ReadWrite,
}

impl Access {
    pub fn is_read(&self) -> bool {
        match self {
            Access::Read | Access::ReadWrite => true,
            Access::Write => false,
        }
    }

    pub fn is_write(&self) -> bool {
        match self {
            Access::Write | Access::ReadWrite => true,
            Access::Read => false,
        }
    }
}

impl PartialOrd for Access {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        match (self, other) {
            (Access::ReadWrite, _) => Some(Ordering::Greater),
            (_, Access::ReadWrite) => Some(Ordering::Less),
            _ => None,
        }
    }
}
