// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BSD-3-Clause-Clear

pub mod ast;
pub mod core;
mod dependency_ordering;
mod expand;
mod infinite_instantiations;
mod recursive_structs;
pub(crate) mod translate;
pub mod visitor;
