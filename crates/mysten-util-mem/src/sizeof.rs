// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// Copyright 2020 Parity Technologies

//! Estimation for heapsize calculation. Usable to replace call to allocator method (for some
//! allocators or simply because we just need a deterministic cunsumption measurement).

use crate::malloc_size::{
    MallocShallowSizeOf, MallocSizeOf, MallocSizeOfOps, MallocUnconditionalShallowSizeOf,
};
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::sync::Arc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use core::mem::{size_of, size_of_val};

#[cfg(feature = "std")]
use std::mem::{size_of, size_of_val};
#[cfg(feature = "std")]
use std::sync::Arc;

impl<T: ?Sized> MallocShallowSizeOf for Box<T> {
    fn shallow_size_of(&self, _ops: &mut MallocSizeOfOps) -> usize {
        size_of_val(&**self)
    }
}

impl MallocSizeOf for String {
    fn size_of(&self, _ops: &mut MallocSizeOfOps) -> usize {
        self.capacity() * size_of::<u8>()
    }
}

impl<T> MallocShallowSizeOf for Vec<T> {
    fn shallow_size_of(&self, _ops: &mut MallocSizeOfOps) -> usize {
        self.capacity() * size_of::<T>()
    }
}

impl<T> MallocUnconditionalShallowSizeOf for Arc<T> {
    fn unconditional_shallow_size_of(&self, _ops: &mut MallocSizeOfOps) -> usize {
        size_of::<T>()
    }
}
