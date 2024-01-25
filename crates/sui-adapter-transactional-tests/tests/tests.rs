// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

pub const TEST_DIR: &str = "tests";
use sui_transactional_test_runner::run_test;

datatest_stable::harness!(run_test, TEST_DIR, r".*\.(mvir|move)$");
