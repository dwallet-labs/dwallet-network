// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BSD-3-Clause-Clear

use move_binary_format::file_format::CompiledModule;
use move_bytecode_verifier::RecursiveStructDefChecker;
use proptest::prelude::*;

proptest! {
    #[test]
    fn valid_recursive_struct_defs(module in CompiledModule::valid_strategy(20)) {
        prop_assert!(RecursiveStructDefChecker::verify_module(&module).is_ok());
    }
}
