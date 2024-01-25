// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// Sui types helpers and utilities
module dwallet::types {
    // === one-time witness ===

    /// Tests if the argument type is a one-time witness, that is a type with only one instantiation
    /// across the entire code base.
    public native fun is_one_time_witness<T: drop>(_: &T): bool;
}
