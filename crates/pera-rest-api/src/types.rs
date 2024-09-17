// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// Chain ID of the current chain
pub const X_PERA_CHAIN_ID: &str = "x-pera-chain-id";

/// Chain name of the current chain
pub const X_PERA_CHAIN: &str = "x-pera-chain";

/// Current checkpoint height
pub const X_PERA_CHECKPOINT_HEIGHT: &str = "x-pera-checkpoint-height";

/// Lowest available checkpoint for which transaction and checkpoint data can be requested.
///
/// Specifically this is the lowest checkpoint for which the following data can be requested:
///  - checkpoints
///  - transactions
///  - effects
///  - events
pub const X_PERA_LOWEST_AVAILABLE_CHECKPOINT: &str = "x-pera-lowest-available-checkpoint";

/// Lowest available checkpoint for which object data can be requested.
///
/// Specifically this is the lowest checkpoint for which input/output object data will be
/// available.
pub const X_PERA_LOWEST_AVAILABLE_CHECKPOINT_OBJECTS: &str =
    "x-pera-lowest-available-checkpoint-objects";

/// Current epoch of the chain
pub const X_PERA_EPOCH: &str = "x-pera-epoch";

/// Cursor to be used for endpoints that support cursor-based pagination. Pass this to the start field of the endpoint on the next call to get the next page of results.
pub const X_PERA_CURSOR: &str = "x-pera-cursor";

/// Current timestamp of the chain - represented as number of milliseconds from the Unix epoch
pub const X_PERA_TIMESTAMP_MS: &str = "x-pera-timestamp-ms";
