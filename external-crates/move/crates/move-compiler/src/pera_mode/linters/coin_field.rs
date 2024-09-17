// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! This analysis flags uses of the pera::coin::Coin struct in fields of other structs. In most cases
//! it's preferable to use pera::balance::Balance instead to save space.

use crate::{
    diag,
    diagnostics::codes::{custom, DiagnosticInfo, Severity},
    naming::ast as N,
    shared::CompilationEnv,
    typing::{ast as T, visitor::TypingVisitor},
};
use move_ir_types::location::Loc;
use move_symbol_pool::Symbol;

use super::{
    LinterDiagnosticCategory, LinterDiagnosticCode, COIN_MOD_NAME, COIN_STRUCT_NAME,
    LINT_WARNING_PREFIX, PERA_PKG_NAME,
};

const COIN_FIELD_DIAG: DiagnosticInfo = custom(
    LINT_WARNING_PREFIX,
    Severity::Warning,
    LinterDiagnosticCategory::Pera as u8,
    LinterDiagnosticCode::CoinField as u8,
    "sub-optimal 'pera::coin::Coin' field type",
);

pub struct CoinFieldVisitor;

impl TypingVisitor for CoinFieldVisitor {
    fn visit(&mut self, env: &mut CompilationEnv, program: &mut T::Program) {
        for (_, _, mdef) in program.modules.iter() {
            if mdef.attributes.is_test_or_test_only() {
                continue;
            }
            env.add_warning_filter_scope(mdef.warning_filter.clone());
            mdef.structs
                .iter()
                .filter(|(_, _, sdef)| !sdef.attributes.is_test_or_test_only())
                .for_each(|(sloc, sname, sdef)| struct_def(env, *sname, sdef, sloc));
            env.pop_warning_filter_scope();
        }
    }
}

fn struct_def(env: &mut CompilationEnv, sname: Symbol, sdef: &N::StructDefinition, sloc: Loc) {
    env.add_warning_filter_scope(sdef.warning_filter.clone());

    if let N::StructFields::Defined(_, sfields) = &sdef.fields {
        for (floc, fname, (_, ftype)) in sfields.iter() {
            if is_field_coin_type(ftype) {
                let msg = format!("The field '{fname}' of '{sname}' has type 'pera::coin::Coin'");
                let uid_msg = "Storing 'pera::balance::Balance' in this field will typically be more space-efficient";
                let d = diag!(COIN_FIELD_DIAG, (sloc, msg), (floc, uid_msg));
                env.add_diag(d);
            }
        }
    }

    env.pop_warning_filter_scope();
}

fn is_field_coin_type(sp!(_, t): &N::Type) -> bool {
    use N::Type_ as T;
    match t {
        T::Ref(_, inner_t) => is_field_coin_type(inner_t),
        T::Apply(_, tname, _) => {
            let sp!(_, tname) = tname;
            tname.is(PERA_PKG_NAME, COIN_MOD_NAME, COIN_STRUCT_NAME)
        }
        T::Unit | T::Param(_) | T::Var(_) | T::Anything | T::UnresolvedError | T::Fun(_, _) => {
            false
        }
    }
}
