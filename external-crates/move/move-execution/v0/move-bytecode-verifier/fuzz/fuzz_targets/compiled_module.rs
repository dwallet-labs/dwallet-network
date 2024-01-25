// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BSD-3-Clause-Clear

#![no_main]
use libfuzzer_sys::fuzz_target;
use move_binary_format::file_format::CompiledModule;

fuzz_target!(|module: CompiledModule| {
    let _ = move_bytecode_verifier::verify_module_unmetered(&module);
});
