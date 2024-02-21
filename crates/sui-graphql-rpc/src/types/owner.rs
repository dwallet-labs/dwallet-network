// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::address::Address;
use super::dynamic_field::DynamicField;
use super::dynamic_field::DynamicFieldName;
use super::stake::StakedSui;
use super::suins_registration::SuinsRegistration;
use crate::context_data::db_data_provider::PgManager;
use crate::types::balance::*;
use crate::types::coin::*;
use crate::types::object::*;
use crate::types::sui_address::SuiAddress;

use async_graphql::connection::Connection;
use async_graphql::*;
use sui_json_rpc::name_service::NameServiceConfig;
use sui_types::dynamic_field::DynamicFieldType;

#[derive(Interface)]
#[graphql(
    field(name = "address", ty = "SuiAddress"),
    field(
        name = "object_connection",
        ty = "Option<Connection<String, Object>>",
        arg(name = "first", ty = "Option<u64>"),
        arg(name = "after", ty = "Option<String>"),
        arg(name = "last", ty = "Option<u64>"),
        arg(name = "before", ty = "Option<String>"),
        arg(name = "filter", ty = "Option<ObjectFilter>")
    ),
    field(
        name = "balance",
        ty = "Option<Balance>",
        arg(name = "type", ty = "Option<String>")
    ),
    field(
        name = "balance_connection",
        ty = "Option<Connection<String, Balance>>",
        arg(name = "first", ty = "Option<u64>"),
        arg(name = "after", ty = "Option<String>"),
        arg(name = "last", ty = "Option<u64>"),
        arg(name = "before", ty = "Option<String>")
    ),
    field(
        name = "coin_connection",
        ty = "Option<Connection<String, Coin>>",
        arg(name = "first", ty = "Option<u64>"),
        arg(name = "after", ty = "Option<String>"),
        arg(name = "last", ty = "Option<u64>"),
        arg(name = "before", ty = "Option<String>"),
        arg(name = "type", ty = "Option<String>")
    ),
    field(
        name = "staked_sui_connection",
        ty = "Option<Connection<String, StakedSui>>",
        arg(name = "first", ty = "Option<u64>"),
        arg(name = "after", ty = "Option<String>"),
        arg(name = "last", ty = "Option<u64>"),
        arg(name = "before", ty = "Option<String>")
    ),
    field(name = "default_name_service_name", ty = "Option<String>"),
    field(
        name = "suins_registrations",
        ty = "Option<Connection<String, SuinsRegistration>>",
        arg(name = "first", ty = "Option<u64>"),
        arg(name = "after", ty = "Option<String>"),
        arg(name = "last", ty = "Option<u64>"),
        arg(name = "before", ty = "Option<String>")
    ),
    field(
        name = "dynamic_field",
        ty = "Option<DynamicField>",
        arg(name = "name", ty = "DynamicFieldName")
    ),
    field(
        name = "dynamic_object_field",
        ty = "Option<DynamicField>",
        arg(name = "name", ty = "DynamicFieldName")
    ),
    field(
        name = "dynamic_field_connection",
        ty = "Option<Connection<String, DynamicField>>",
        arg(name = "first", ty = "Option<u64>"),
        arg(name = "after", ty = "Option<String>"),
        arg(name = "last", ty = "Option<u64>"),
        arg(name = "before", ty = "Option<String>"),
    )
)]
#[derive(Clone, Debug)]
pub(crate) enum ObjectOwner {
    Address(Address),
    Owner(Owner),
    Object(Object),
}

#[derive(Clone, Debug)]
pub(crate) struct Owner {
    pub address: SuiAddress,
}

#[Object]
impl Owner {
    async fn as_address(&self) -> Option<Address> {
        // For now only addresses can be owners
        Some(Address {
            address: self.address,
        })
    }

    async fn as_object(&self) -> Option<Object> {
        // TODO: extend when send to object imnplementation is done
        // For now only addresses can be owners
        None
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

    /// The coin objects for the given address or object.
    ///
    /// The type field is a string of the inner type of the coin by which to filter
    /// (e.g. `0x2::dwlt::DWLT`). If no type is provided, it will default to `0x2::dwlt::DWLT`.
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

    /// The `0x3::staking_pool::StakedSui` objects owned by the given object.
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

    /// Access a dynamic field on an object using its name.
    /// Names are arbitrary Move values whose type have `copy`, `drop`, and `store`, and are specified
    /// using their type, and their BCS contents, Base64 encoded.
    /// This field exists as a convenience when accessing a dynamic field on a wrapped object.
    pub async fn dynamic_field(
        &self,
        ctx: &Context<'_>,
        name: DynamicFieldName,
    ) -> Result<Option<DynamicField>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_dynamic_field(self.address, name, DynamicFieldType::DynamicField)
            .await
            .extend()
    }

    /// Access a dynamic object field on an object using its name.
    /// Names are arbitrary Move values whose type have `copy`, `drop`, and `store`, and are specified
    /// using their type, and their BCS contents, Base64 encoded.
    /// The value of a dynamic object field can also be accessed off-chain directly via its address (e.g. using `Query.object`).
    /// This field exists as a convenience when accessing a dynamic field on a wrapped object.
    pub async fn dynamic_object_field(
        &self,
        ctx: &Context<'_>,
        name: DynamicFieldName,
    ) -> Result<Option<DynamicField>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_dynamic_field(self.address, name, DynamicFieldType::DynamicObject)
            .await
            .extend()
    }

    /// The dynamic fields on an object.
    /// This field exists as a convenience when accessing a dynamic field on a wrapped object.
    pub async fn dynamic_field_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
    ) -> Result<Option<Connection<String, DynamicField>>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_dynamic_fields(first, after, last, before, self.address)
            .await
            .extend()
    }
}
