// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[cfg(test)]
#[path = "unit_tests/transaction_deny_tests.rs"]
mod transaction_deny_tests;

pub use sui_transaction_checks::deny::check_transaction_for_signing;
