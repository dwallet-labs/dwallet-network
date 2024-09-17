// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BSD-3-Clause-Clear

mod generator;
mod options;
mod padding;
mod utils;

pub use generator::{
    generate_module, generate_modules, generate_verified_modules, ModuleGenerator,
};
pub use options::ModuleGeneratorOptions;
pub use padding::Pad;
