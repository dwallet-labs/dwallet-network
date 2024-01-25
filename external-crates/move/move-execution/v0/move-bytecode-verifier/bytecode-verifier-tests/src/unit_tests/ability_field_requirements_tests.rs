// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BSD-3-Clause-Clear

use move_binary_format::file_format::CompiledModule;
use move_bytecode_verifier::ability_field_requirements;
use proptest::prelude::*;

proptest! {
    #[test]
    fn valid_ability_transitivity(module in CompiledModule::valid_strategy(20)) {
        prop_assert!(ability_field_requirements::verify_module(&module).is_ok());
    }
}
