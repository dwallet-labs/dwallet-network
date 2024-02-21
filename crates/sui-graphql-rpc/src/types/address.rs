// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use async_graphql::{connection::Connection, *};
use sui_json_rpc::name_service::NameServiceConfig;

use crate::{context_data::db_data_provider::PgManager, error::Error};

use super::{
    balance::Balance,
    coin::Coin,
    dynamic_field::{DynamicField, DynamicFieldName},
    object::{Object, ObjectFilter},
    stake::StakedSui,
    sui_address::SuiAddress,
    suins_registration::SuinsRegistration,
    transaction_block::{TransactionBlock, TransactionBlockFilter},
};

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub(crate) struct Address {
    pub address: SuiAddress,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub(crate) enum AddressTransactionBlockRelationship {
    Sign, // Transactions this address has signed
    Sent, // Transactions that transferred objects from this address
    Recv, // Transactions that received objects into this address
    Paid, // Transactions that were paid for by this address
}

#[Object]
impl Address {
    /// Similar behavior to the `transactionBlockConnection` in Query but
    /// supports additional `AddressTransactionBlockRelationship` filter
    async fn transaction_block_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
        relation: Option<AddressTransactionBlockRelationship>,
        filter: Option<TransactionBlockFilter>,
    ) -> Result<Option<Connection<String, TransactionBlock>>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_txs_for_address(
                first,
                after,
                last,
                before,
                filter,
                (
                    self.address,
                    // Assume signer if no relationship is specified
                    relation.unwrap_or(AddressTransactionBlockRelationship::Sign),
                ),
            )
            .await
            .extend()
    }

    // =========== Owner interface methods =============

    pub async fn address(&self) -> SuiAddress {
        self.address
    }

    pub async fn object_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
        filter: Option<ObjectFilter>,
    ) -> Result<Option<Connection<String, Object>>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_owned_objs(first, after, last, before, filter, self.address)
            .await
            .extend()
    }

    pub async fn balance(
        &self,
        ctx: &Context<'_>,
        type_: Option<String>,
    ) -> Result<Option<Balance>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_balance(self.address, type_)
            .await
            .extend()
    }

    pub async fn balance_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
    ) -> Result<Option<Connection<String, Balance>>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_balances(self.address, first, after, last, before)
            .await
            .extend()
    }

    /// The coin objects for the given address.
    /// The type field is a string of the inner type of the coin
    /// by which to filter (e.g., 0x2::dwlt::DWLT).
    pub async fn coin_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
        type_: Option<String>,
    ) -> Result<Option<Connection<String, Coin>>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_coins(Some(self.address), type_, first, after, last, before)
            .await
            .extend()
    }

    /// The `0x3::staking_pool::StakedSui` objects owned by the given address.
    pub async fn staked_sui_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
    ) -> Result<Option<Connection<String, StakedSui>>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_staked_sui(self.address, first, after, last, before)
            .await
            .extend()
    }

    pub async fn default_name_service_name(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        ctx.data_unchecked::<PgManager>()
            .default_name_service_name(ctx.data_unchecked::<NameServiceConfig>(), self.address)
            .await
            .extend()
    }

    /// The SuinsRegistration NFTs owned by the given object. These grant the owner
    /// the capability to manage the associated domain.
    pub async fn suins_registrations(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
    ) -> Result<Option<Connection<String, SuinsRegistration>>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_suins_registrations(
                first,
                after,
                last,
                before,
                ctx.data_unchecked::<NameServiceConfig>(),
                self.address,
            )
            .await
            .extend()
    }

    /// This resolver is not supported on the Address type.
    pub async fn dynamic_field(&self, _name: DynamicFieldName) -> Result<Option<DynamicField>> {
        Err(Error::DynamicFieldOnAddress.extend())
    }

    /// This resolver is not supported on the Address type.
    pub async fn dynamic_object_field(
        &self,
        _name: DynamicFieldName,
    ) -> Result<Option<DynamicField>> {
        Err(Error::DynamicFieldOnAddress.extend())
    }

    pub async fn dynamic_field_connection(
        &self,
        _first: Option<u64>,
        _after: Option<String>,
        _last: Option<u64>,
        _before: Option<String>,
    ) -> Result<Option<Connection<String, DynamicField>>> {
        Err(Error::DynamicFieldOnAddress.extend())
    }
}
