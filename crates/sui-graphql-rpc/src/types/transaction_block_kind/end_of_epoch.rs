// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use async_graphql::connection::{Connection, CursorType, Edge};
use async_graphql::*;
use move_binary_format::errors::PartialVMResult;
use move_binary_format::CompiledModule;
use sui_types::{
    digests::TransactionDigest,
    object::Object as NativeObject,
    transaction::{
        AuthenticatorStateExpire as NativeAuthenticatorStateExpireTransaction,
        ChangeEpoch as NativeChangeEpochTransaction,
        EndOfEpochTransactionKind as NativeEndOfEpochTransactionKind,
    },
};

use crate::types::cursor::{Cursor, Page};
use crate::types::sui_address::SuiAddress;
use crate::{
    error::Error,
    types::{
        big_int::BigInt, date_time::DateTime, epoch::Epoch, move_package::MovePackage,
        object::Object,
    },
};

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct EndOfEpochTransaction(pub Vec<NativeEndOfEpochTransactionKind>);

#[derive(Union, Clone, PartialEq, Eq)]
pub(crate) enum EndOfEpochTransactionKind {
    ChangeEpoch(ChangeEpochTransaction),
    AuthenticatorStateCreate(AuthenticatorStateCreateTransaction),
    AuthenticatorStateExpire(AuthenticatorStateExpireTransaction),
    RandomnessStateCreate(RandomnessStateCreateTransaction),
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct ChangeEpochTransaction(pub NativeChangeEpochTransaction);

#[derive(SimpleObject, Clone, PartialEq, Eq)]
pub(crate) struct AuthenticatorStateCreateTransaction {
    /// A workaround to define an empty variant of a GraphQL union.
    #[graphql(name = "_")]
    dummy: Option<bool>,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct AuthenticatorStateExpireTransaction(
    pub NativeAuthenticatorStateExpireTransaction,
);

#[derive(SimpleObject, Clone, PartialEq, Eq)]
pub(crate) struct RandomnessStateCreateTransaction {
    /// A workaround to define an empty variant of a GraphQL union.
    #[graphql(name = "_")]
    dummy: Option<bool>,
}

pub(crate) type CTxn = Cursor<usize>;
pub(crate) type CPackage = Cursor<usize>;

/// System transaction that supersedes `ChangeEpochTransaction` as the new way to run transactions
/// at the end of an epoch. Behaves similarly to `ChangeEpochTransaction` but can accommodate other
/// optional transactions to run at the end of the epoch.
#[Object]
impl EndOfEpochTransaction {
    /// The list of system transactions that are allowed to run at the end of the epoch.
    async fn transactions(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        before: Option<CTxn>,
        last: Option<u64>,
        after: Option<CTxn>,
    ) -> Result<Connection<String, EndOfEpochTransactionKind>> {
        let page = Page::from_params(ctx.data_unchecked(), first, after, last, before)?;

        let mut connection = Connection::new(false, false);
        let Some((prev, next, cs)) = page.paginate_indices(self.0.len()) else {
            return Ok(connection);
        };

        connection.has_previous_page = prev;
        connection.has_next_page = next;

        for c in cs {
            let tx = EndOfEpochTransactionKind::from(self.0[*c].clone());
            connection.edges.push(Edge::new(c.encode_cursor(), tx));
        }

        Ok(connection)
    }
}

/// A system transaction that updates epoch information on-chain (increments the current epoch).
/// Executed by the system once per epoch, without using gas. Epoch change transactions cannot be
/// submitted by users, because validators will refuse to sign them.
///
/// This transaction kind is deprecated in favour of `EndOfEpochTransaction`.
#[Object]
impl ChangeEpochTransaction {
    /// The next (to become) epoch.
    async fn epoch(&self, ctx: &Context<'_>) -> Result<Option<Epoch>> {
        Epoch::query(ctx.data_unchecked(), Some(self.0.epoch))
            .await
            .extend()
    }

    /// The protocol version in effect in the new epoch.
    async fn protocol_version(&self) -> u64 {
        self.0.protocol_version.as_u64()
    }

    /// The total amount of gas charged for storage during the previous epoch (in MIST).
    async fn storage_charge(&self) -> BigInt {
        BigInt::from(self.0.storage_charge)
    }

    /// The total amount of gas charged for computation during the previous epoch (in MIST).
    async fn computation_charge(&self) -> BigInt {
        BigInt::from(self.0.computation_charge)
    }

    /// The SUI returned to transaction senders for cleaning up objects (in MIST).
    async fn storage_rebate(&self) -> BigInt {
        BigInt::from(self.0.storage_rebate)
    }

    /// The total gas retained from storage fees, that will not be returned by storage rebates when
    /// the relevant objects are cleaned up (in MIST).
    async fn non_refundable_storage_fee(&self) -> BigInt {
        BigInt::from(self.0.non_refundable_storage_fee)
    }

    /// Time at which the next epoch will start.
    async fn start_timestamp(&self) -> Result<DateTime, Error> {
        DateTime::from_ms(self.0.epoch_start_timestamp_ms as i64)
    }

    /// System packages (specifically framework and move stdlib) that are written before the new
    /// epoch starts, to upgrade them on-chain. Validators write these packages out when running the
    /// transaction.
    async fn system_packages(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<CPackage>,
        last: Option<u64>,
        before: Option<CPackage>,
    ) -> Result<Connection<String, MovePackage>> {
        let page = Page::from_params(ctx.data_unchecked(), first, after, last, before)?;

        let mut connection = Connection::new(false, false);
        let Some((prev, next, cs)) = page.paginate_indices(self.0.system_packages.len()) else {
            return Ok(connection);
        };

        connection.has_previous_page = prev;
        connection.has_next_page = next;

        for c in cs {
            let (version, modules, deps) = &self.0.system_packages[*c];
            let compiled_modules = modules
                .iter()
                .map(|bytes| CompiledModule::deserialize_with_defaults(bytes))
                .collect::<PartialVMResult<Vec<_>>>()
                .map_err(|e| Error::Internal(format!("Failed to deserialize system modules: {e}")))
                .extend()?;

            let native = NativeObject::new_system_package(
                &compiled_modules,
                *version,
                deps.clone(),
                TransactionDigest::ZERO,
            );

            let runtime_id = native.id();
            let object = Object::from_native(SuiAddress::from(runtime_id), native);
            let package = MovePackage::try_from(&object)
                .map_err(|_| Error::Internal("Failed to create system package".to_string()))
                .extend()?;

            connection.edges.push(Edge::new(c.encode_cursor(), package));
        }

        Ok(connection)
    }
}

#[Object]
impl AuthenticatorStateExpireTransaction {
    /// Expire JWKs that have a lower epoch than this.
    async fn min_epoch(&self, ctx: &Context<'_>) -> Result<Option<Epoch>> {
        Epoch::query(ctx.data_unchecked(), Some(self.0.min_epoch))
            .await
            .extend()
    }

    /// The initial version that the AuthenticatorStateUpdate was shared at.
    async fn authenticator_obj_initial_shared_version(&self) -> u64 {
        self.0.authenticator_obj_initial_shared_version.value()
    }
}

impl From<NativeEndOfEpochTransactionKind> for EndOfEpochTransactionKind {
    fn from(kind: NativeEndOfEpochTransactionKind) -> Self {
        use EndOfEpochTransactionKind as K;
        use NativeEndOfEpochTransactionKind as N;

        match kind {
            N::ChangeEpoch(ce) => K::ChangeEpoch(ChangeEpochTransaction(ce)),
            N::AuthenticatorStateCreate => {
                K::AuthenticatorStateCreate(AuthenticatorStateCreateTransaction { dummy: None })
            }
            N::AuthenticatorStateExpire(ase) => {
                K::AuthenticatorStateExpire(AuthenticatorStateExpireTransaction(ase))
            }
            N::RandomnessStateCreate => {
                K::RandomnessStateCreate(RandomnessStateCreateTransaction { dummy: None })
            }
        }
    }
}
