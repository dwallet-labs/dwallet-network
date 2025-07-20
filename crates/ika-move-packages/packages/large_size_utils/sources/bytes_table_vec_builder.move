// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// This module provides a builder pattern to construct a `TableVec` object.
///
/// Due to Sui's limitations on object size (≤ 250KB) and transaction size (≤ 128KB),
/// the full data must be split into parts and stored dynamically using `table_vec`
/// in different transactions - buffer module can be used to store the parts of the data
/// in a single transaction.
module large_size_utils::bytes_table_vec_builder;

use sui::table_vec;

// === Structs ===

/// `TableVecBuilder` is used to construct a `TableVec` object.
public struct TableVecBuilder has key, store {
    id: UID,
    table_vec: table_vec::TableVec<vector<u8>>,
}

// === Public Functions ===

/// Creates a new `TableVecBuilder` instance.
public fun empty(ctx: &mut TxContext): TableVecBuilder {
    TableVecBuilder {
        id: object::new(ctx),
        table_vec: table_vec::empty(ctx),
    }
}

/// Returns the length of the `TableVecBuilder`.
public fun length(self: &TableVecBuilder): u64 {
    self.table_vec.length()
}

/// Pushes a value to the `TableVecBuilder`.
public fun push_back(self: &mut TableVecBuilder, value: vector<u8>) {
    self.table_vec.push_back(value);
}

/// Finalizes the construction of a `TableVec` object.
public fun destroy(self: TableVecBuilder): table_vec::TableVec<vector<u8>> {
    let TableVecBuilder { id, table_vec } = self;
    id.delete();
    table_vec
}