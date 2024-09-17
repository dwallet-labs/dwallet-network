// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module oracle::decimal_value {
    public struct DecimalValue has store, drop, copy {
        value: u64,
        decimal: u8,
    }

    public fun new(value: u64, decimal: u8): DecimalValue {
        DecimalValue { value, decimal }
    }

    public fun value(self: &DecimalValue): u64 {
        self.value
    }

    public fun decimal(self: &DecimalValue): u8 {
        self.decimal
    }
}
