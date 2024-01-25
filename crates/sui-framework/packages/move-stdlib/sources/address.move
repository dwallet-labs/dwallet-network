// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// Provides a way to get address length since it's a
/// platform-specific parameter.
module std::address {
    /// Should be converted to a native function.
    /// Current implementation only works for Sui.
    public fun length(): u64 {
        32
    }
}
