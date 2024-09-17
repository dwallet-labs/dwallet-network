// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#![allow(unused_imports)]
#![allow(unused_variables)]
use pera_transactional_test_runner::{
    run_test_impl,
    test_adapter::{PeraTestAdapter, PRE_COMPILED},
};
use std::{path::Path, sync::Arc};
pub const TEST_DIR: &str = "tests";

datatest_stable::harness!(run_test, TEST_DIR, r".*\.(mvir|move)$");

#[cfg_attr(not(msim), tokio::main)]
#[cfg_attr(msim, msim::main)]
async fn run_test(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(feature = "pg_integration") {
        run_test_impl::<PeraTestAdapter>(path, Some(Arc::new(PRE_COMPILED.clone()))).await?;
    }
    Ok(())
}
