// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[macro_use(sp)]
extern crate move_ir_types;

pub mod analysis;
pub mod analyzer;
pub mod compiler_info;
pub mod completion;
pub mod context;
pub mod diagnostics;
pub mod inlay_hints;
pub mod symbols;
pub mod utils;
pub mod vfs;
