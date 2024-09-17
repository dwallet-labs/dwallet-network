// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::programmable_transactions::context::load_type_from_struct;
use crate::programmable_transactions::linkage_view::LinkageView;
use move_core_types::account_address::AccountAddress;
use move_core_types::annotated_value as A;
use move_core_types::language_storage::StructTag;
use move_core_types::resolver::ResourceResolver;
use move_vm_runtime::move_vm::MoveVM;
use pera_types::base_types::ObjectID;
use pera_types::error::PeraResult;
use pera_types::execution::TypeLayoutStore;
use pera_types::storage::{BackingPackageStore, PackageObject};
use pera_types::{error::PeraError, layout_resolver::LayoutResolver};

/// Retrieve a `MoveStructLayout` from a `Type`.
/// Invocation into the `Session` to leverage the `LinkageView` implementation
/// common to the runtime.
pub struct TypeLayoutResolver<'state, 'vm> {
    vm: &'vm MoveVM,
    linkage_view: LinkageView<'state>,
}

/// Implements PeraResolver traits by providing null implementations for module and resource
/// resolution and delegating backing package resolution to the trait object.
struct NullPeraResolver<'state>(Box<dyn TypeLayoutStore + 'state>);

impl<'state, 'vm> TypeLayoutResolver<'state, 'vm> {
    pub fn new(vm: &'vm MoveVM, state_view: Box<dyn TypeLayoutStore + 'state>) -> Self {
        let linkage_view = LinkageView::new(Box::new(NullPeraResolver(state_view)));
        Self { vm, linkage_view }
    }
}

impl<'state, 'vm> LayoutResolver for TypeLayoutResolver<'state, 'vm> {
    fn get_annotated_layout(
        &mut self,
        struct_tag: &StructTag,
    ) -> Result<A::MoveDatatypeLayout, PeraError> {
        let Ok(ty) = load_type_from_struct(self.vm, &mut self.linkage_view, &[], struct_tag) else {
            return Err(PeraError::FailObjectLayout {
                st: format!("{}", struct_tag),
            });
        };
        let layout = self.vm.get_runtime().type_to_fully_annotated_layout(&ty);
        let Ok(A::MoveTypeLayout::Struct(layout)) = layout else {
            return Err(PeraError::FailObjectLayout {
                st: format!("{}", struct_tag),
            });
        };
        Ok(A::MoveDatatypeLayout::Struct(layout))
    }
}

impl<'state> BackingPackageStore for NullPeraResolver<'state> {
    fn get_package_object(&self, package_id: &ObjectID) -> PeraResult<Option<PackageObject>> {
        self.0.get_package_object(package_id)
    }
}

impl<'state> ResourceResolver for NullPeraResolver<'state> {
    type Error = PeraError;

    fn get_resource(
        &self,
        _address: &AccountAddress,
        _typ: &StructTag,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(None)
    }
}
