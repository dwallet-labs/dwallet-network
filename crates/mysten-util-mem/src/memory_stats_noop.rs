// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// Copyright 2021 Parity Technologies

#[derive(Clone, Debug)]
pub struct Unimplemented;
pub use Unimplemented as Error;

#[cfg(feature = "std")]
impl std::fmt::Display for Unimplemented {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("unimplemented")
    }
}

#[derive(Clone)]
pub struct MemoryAllocationTracker {}

impl MemoryAllocationTracker {
    pub fn new() -> Result<Self, Error> {
        Err(Error)
    }

    pub fn snapshot(&self) -> Result<crate::MemoryAllocationSnapshot, Error> {
        unimplemented!();
    }
}
