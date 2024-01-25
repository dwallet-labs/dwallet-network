// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::{big_int::BigInt, move_type::MoveType};
use async_graphql::*;

#[derive(Clone, Debug, SimpleObject)]
pub(crate) struct Balance {
    /// Coin type for the balance, such as 0x2::dwlt::DWLT
    pub(crate) coin_type: Option<MoveType>,
    /// How many coins of this type constitute the balance
    pub(crate) coin_object_count: Option<u64>,
    /// Total balance across all coin objects of the coin type
    pub(crate) total_balance: Option<BigInt>,
}
