// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use move_core_types::language_storage::TypeTag;
use move_vm_types::loaded_data::runtime_types::Type;
use pera_types::error::ExecutionError;

pub trait TypeTagResolver {
    fn get_type_tag(&self, type_: &Type) -> Result<TypeTag, ExecutionError>;
}
