// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use balance_changes::*;
pub use object_changes::*;
pub use ika_checkpoint::*;
pub use ika_coin::*;
pub use ika_event::*;
pub use ika_extended::*;
pub use ika_governance::*;
pub use ika_move::*;
pub use ika_object::*;
pub use ika_protocol::*;
pub use ika_transaction::*;
use ika_types::base_types::ObjectID;
use ika_types::dynamic_field::DynamicFieldInfo;

#[cfg(test)]
#[path = "unit_tests/rpc_types_tests.rs"]
mod rpc_types_tests;

mod balance_changes;
mod displays;
mod object_changes;
mod ika_checkpoint;
mod ika_coin;
mod ika_event;
mod ika_extended;
mod ika_governance;
mod ika_move;
mod ika_object;
mod ika_protocol;
mod ika_transaction;

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
