// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use balance_changes::*;
pub use object_changes::*;
pub use pera_checkpoint::*;
pub use pera_coin::*;
pub use pera_event::*;
pub use pera_extended::*;
pub use pera_governance::*;
pub use pera_move::*;
pub use pera_object::*;
pub use pera_protocol::*;
pub use pera_transaction::*;
use pera_types::base_types::ObjectID;
use pera_types::dynamic_field::DynamicFieldInfo;

#[cfg(test)]
#[path = "unit_tests/rpc_types_tests.rs"]
mod rpc_types_tests;

mod balance_changes;
mod displays;
mod object_changes;
mod pera_checkpoint;
mod pera_coin;
mod pera_event;
mod pera_extended;
mod pera_governance;
mod pera_move;
mod pera_object;
mod pera_protocol;
mod pera_transaction;

pub type DynamicFieldPage = Page<DynamicFieldInfo, ObjectID>;
/// `next_cursor` points to the last item in the page;
/// Reading with `next_cursor` will start from the next item after `next_cursor` if
/// `next_cursor` is `Some`, otherwise it will start from the first item.
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Page<T, C> {
    pub data: Vec<T>,
    pub next_cursor: Option<C>,
    pub has_next_page: bool,
}

impl<T, C> Page<T, C> {
    pub fn empty() -> Self {
        Self {
            data: vec![],
            next_cursor: None,
            has_next_page: false,
        }
    }
}
