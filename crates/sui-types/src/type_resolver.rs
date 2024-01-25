// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::error::{ExecutionError, SuiError};
use move_core_types::{
    annotated_value as A,
    language_storage::{StructTag, TypeTag},
};
use move_vm_types::loaded_data::runtime_types::Type;

pub trait LayoutResolver {
    fn get_annotated_layout(
        &mut self,
        struct_tag: &StructTag,
    ) -> Result<A::MoveStructLayout, SuiError>;
}

pub trait TypeTagResolver {
    fn get_type_tag(&self, type_: &Type) -> Result<TypeTag, ExecutionError>;
}
