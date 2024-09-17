// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

mod gas_status_displays;
pub mod transaction_displays;

pub struct Pretty<'a, T>(pub &'a T);
