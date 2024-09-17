// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

mod dry_run_tx_block;
mod gas_cost_summary;
mod ptb_preview;
mod status;
mod summary;

pub struct Pretty<'a, T>(pub &'a T);
