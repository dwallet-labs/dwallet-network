// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use async_graphql::*;
use sui_types::effects::InputSharedObject as NativeInputSharedObject;

use super::{object_read::ObjectRead, sui_address::SuiAddress};

/// Details pertaining to shared objects that are referenced by but not changed by a transaction.
/// This information is considered part of the effects, because although the transaction specifies
/// the shared object as input, consensus must schedule it and pick the version that is actually
/// used.
#[derive(Union)]
pub(crate) enum UnchangedSharedObject {
    Read(SharedObjectRead),
    Delete(SharedObjectDelete),
}

/// The transaction accepted a shared object as input, but only to read it.
#[derive(SimpleObject)]
pub(crate) struct SharedObjectRead {
    #[graphql(flatten)]
    read: ObjectRead,
}

/// The transaction accepted a shared object as input, but it was deleted before the transaction
/// executed.
#[derive(SimpleObject)]
pub(crate) struct SharedObjectDelete {
    /// ID of the shared object.
    address: SuiAddress,

    /// The version of the shared object that was assigned to this transaction during by consensus,
    /// during sequencing.
    version: u64,

    /// Whether this transaction intended to use this shared object mutably or not. See
    /// `SharedInput.mutable` for further details.
    mutable: bool,
}

/// Error for converting from an `InputSharedObject`.
pub(crate) struct SharedObjectChanged;

impl TryFrom<NativeInputSharedObject> for UnchangedSharedObject {
    type Error = SharedObjectChanged;

    fn try_from(input: NativeInputSharedObject) -> Result<Self, Self::Error> {
        use NativeInputSharedObject as I;
        use UnchangedSharedObject as U;

        match input {
            I::Mutate(_) => Err(SharedObjectChanged),

            I::ReadOnly(oref) => Ok(U::Read(SharedObjectRead {
                read: ObjectRead(oref),
            })),

            I::ReadDeleted(id, v) => Ok(U::Delete(SharedObjectDelete {
                address: id.into(),
                version: v.value(),
                mutable: false,
            })),

            I::MutateDeleted(id, v) => Ok(U::Delete(SharedObjectDelete {
                address: id.into(),
                version: v.value(),
                mutable: true,
            })),
        }
    }
}
