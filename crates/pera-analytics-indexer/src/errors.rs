// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalyticsIndexerError {
    #[error("Generic error: `{0}`")]
    GenericError(String),
    #[error("Failed to retrieve the current directory.")]
    CurrentDirError,
}
