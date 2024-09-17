// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::programmable_transactions::context::new_session_for_linkage;
use crate::programmable_transactions::{
    context::load_type,
    linkage_view::{LinkageInfo, LinkageView},
};
use move_core_types::account_address::AccountAddress;
use move_core_types::annotated_value as A;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_core_types::resolver::ResourceResolver;
use move_vm_runtime::{move_vm::MoveVM, session::Session};
use pera_types::base_types::ObjectID;
use pera_types::error::PeraResult;
use pera_types::execution::TypeLayoutStore;
use pera_types::storage::{BackingPackageStore, PackageObject};
use pera_types::{error::PeraError, layout_resolver::LayoutResolver};

/// Retrieve a `MoveStructLayout` from a `Type`.
/// Invocation into the `Session` to leverage the `LinkageView` implementation
/// common to the runtime.
pub struct TypeLayoutResolver<'state, 'vm> {
    session: Session<'state, 'vm, LinkageView<'state>>,
}

/// Implements PeraResolver traits by providing null implementations for module and resource
/// resolution and delegating backing package resolution to the trait object.
struct NullPeraResolver<'state>(Box<dyn TypeLayoutStore + 'state>);

impl<'state, 'vm> TypeLayoutResolver<'state, 'vm> {
    pub fn new(vm: &'vm MoveVM, state_view: Box<dyn TypeLayoutStore + 'state>) -> Self {
        let session = new_session_for_linkage(
            vm,
            LinkageView::new(Box::new(NullPeraResolver(state_view)), LinkageInfo::Unset),
        );
        Self { session }
    }
}

impl<'state, 'vm> LayoutResolver for TypeLayoutResolver<'state, 'vm> {
    fn get_annotated_layout(
        &mut self,
        struct_tag: &StructTag,
    ) -> Result<A::MoveDatatypeLayout, PeraError> {
        let type_tag: TypeTag = TypeTag::from(struct_tag.clone());
        let Ok(ty) = load_type(&mut self.session, &type_tag) else {
            return Err(PeraError::FailObjectLayout {
                st: format!("{}", struct_tag),
            });
        };
        let layout = self.session.type_to_fully_annotated_layout(&ty);
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
