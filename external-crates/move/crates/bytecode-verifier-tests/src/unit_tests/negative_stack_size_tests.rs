// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::support::dummy_procedure_module;
use move_binary_format::file_format::Bytecode;
use move_bytecode_verifier::meter::DummyMeter;
use move_bytecode_verifier::CodeUnitVerifier;
use move_core_types::vm_status::StatusCode;

#[test]
fn one_pop_no_push() {
    let module = dummy_procedure_module(vec![Bytecode::Pop, Bytecode::Ret]);
    let result = CodeUnitVerifier::verify_module(&Default::default(), &module, &mut DummyMeter);
    assert_eq!(
        result.unwrap_err().major_status(),
        StatusCode::NEGATIVE_STACK_SIZE_WITHIN_BLOCK
    );
}

#[test]
fn one_pop_one_push() {
    // Height: 0 + (-1 + 1) = 0 would have passed original usage verifier
    let module = dummy_procedure_module(vec![Bytecode::ReadRef, Bytecode::Ret]);
    let result = CodeUnitVerifier::verify_module(&Default::default(), &module, &mut DummyMeter);
    assert_eq!(
        result.unwrap_err().major_status(),
        StatusCode::NEGATIVE_STACK_SIZE_WITHIN_BLOCK
    );
}

#[test]
fn two_pop_one_push() {
    // Height: 0 + 1 + (-2 + 1) = 0 would have passed original usage verifier
    let module = dummy_procedure_module(vec![Bytecode::LdU64(0), Bytecode::Add, Bytecode::Ret]);
    let result = CodeUnitVerifier::verify_module(&Default::default(), &module, &mut DummyMeter);
    assert_eq!(
        result.unwrap_err().major_status(),
        StatusCode::NEGATIVE_STACK_SIZE_WITHIN_BLOCK
    );
}

#[test]
fn two_pop_no_push() {
    let module = dummy_procedure_module(vec![Bytecode::WriteRef, Bytecode::Ret]);
    let result = CodeUnitVerifier::verify_module(&Default::default(), &module, &mut DummyMeter);
    assert_eq!(
        result.unwrap_err().major_status(),
        StatusCode::NEGATIVE_STACK_SIZE_WITHIN_BLOCK
    );
}
