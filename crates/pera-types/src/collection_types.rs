// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use serde::{Deserialize, Serialize};

use crate::base_types::ObjectID;
use crate::id::UID;

/// Rust version of the Move pera::vec_map::VecMap type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct VecMap<K, V> {
    pub contents: Vec<Entry<K, V>>,
}

/// Rust version of the Move pera::vec_map::Entry type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Entry<K, V> {
    pub key: K,
    pub value: V,
}

/// Rust version of the Move pera::vec_set::VecSet type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct VecSet<T> {
    pub contents: Vec<T>,
}

/// Rust version of the Move pera::table::Table type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TableVec {
    pub contents: Table,
}

impl Default for TableVec {
    fn default() -> Self {
        TableVec {
            contents: Table {
                id: ObjectID::ZERO,
                size: 0,
            },
        }
    }
}

/// Rust version of the Move pera::table::Table type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Table {
    pub id: ObjectID,
    pub size: u64,
}

impl Default for Table {
    fn default() -> Self {
        Table {
            id: ObjectID::ZERO,
            size: 0,
        }
    }
}

/// Rust version of the Move pera::linked_table::LinkedTable type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct LinkedTable<K> {
    pub id: ObjectID,
    pub size: u64,
    pub head: Option<K>,
    pub tail: Option<K>,
}

impl<K> Default for LinkedTable<K> {
    fn default() -> Self {
        LinkedTable {
            id: ObjectID::ZERO,
            size: 0,
            head: None,
            tail: None,
        }
    }
}

/// Rust version of the Move pera::linked_table::Node type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct LinkedTableNode<K, V> {
    pub prev: Option<K>,
    pub next: Option<K>,
    pub value: V,
}

/// Rust version of the Move pera::bag::Bag type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Bag {
    pub id: UID,
    pub size: u64,
}

/// Rust version of the Move pera::bag::Bag type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ObjectBag {
    pub id: UID,
    pub size: u64,
}

impl Default for Bag {
    fn default() -> Self {
        Self {
            id: UID::new(ObjectID::ZERO),
            size: 0,
        }
    }
}

impl Default for ObjectBag {
    fn default() -> Self {
        Self {
            id: UID::new(ObjectID::ZERO),
            size: 0,
        }
    }
}
