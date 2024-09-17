// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use pera_json_rpc_types::PeraTransactionBlockEffects;
use pera_json_rpc_types::{
    DynamicFieldPage, EventFilter, EventPage, ObjectsPage, Page, PeraEvent, PeraObjectResponse,
    PeraObjectResponseQuery, PeraTransactionBlockResponseQuery, TransactionBlocksPage,
    TransactionFilter,
};
use pera_open_rpc_macros::open_rpc;
use pera_types::base_types::{ObjectID, PeraAddress};
use pera_types::digests::TransactionDigest;
use pera_types::dynamic_field::DynamicFieldName;
use pera_types::event::EventID;

#[open_rpc(namespace = "perax", tag = "Extended API")]
#[rpc(server, client, namespace = "perax")]
pub trait IndexerApi {
    /// Return the list of objects owned by an address.
    /// Note that if the address owns more than `QUERY_MAX_RESULT_LIMIT` objects,
    /// the pagination is not accurate, because previous page may have been updated when
    /// the next page is fetched.
    /// Please use perax_queryObjects if this is a concern.
    #[method(name = "getOwnedObjects")]
    async fn get_owned_objects(
        &self,
        /// the owner's Pera address
        address: PeraAddress,
        /// the objects query criteria.
        query: Option<PeraObjectResponseQuery>,
        /// An optional paging cursor. If provided, the query will start from the next item after the specified cursor. Default to start from the first item if not specified.
        cursor: Option<ObjectID>,
        /// Max number of items returned per page, default to [QUERY_MAX_RESULT_LIMIT] if not specified.
        limit: Option<usize>,
    ) -> RpcResult<ObjectsPage>;

    /// Return list of transactions for a specified query criteria.
    #[method(name = "queryTransactionBlocks")]
    async fn query_transaction_blocks(
        &self,
        /// the transaction query criteria.
        query: PeraTransactionBlockResponseQuery,
        /// An optional paging cursor. If provided, the query will start from the next item after the specified cursor. Default to start from the first item if not specified.
        cursor: Option<TransactionDigest>,
        /// Maximum item returned per page, default to QUERY_MAX_RESULT_LIMIT if not specified.
        limit: Option<usize>,
        /// query result ordering, default to false (ascending order), oldest record first.
        descending_order: Option<bool>,
    ) -> RpcResult<TransactionBlocksPage>;

    /// Return list of events for a specified query criteria.
    #[method(name = "queryEvents")]
    async fn query_events(
        &self,
        /// The event query criteria. See [Event filter](https://docs.pera.io/build/event_api#event-filters) documentation for examples.
        query: EventFilter,
        /// optional paging cursor
        cursor: Option<EventID>,
        /// maximum number of items per page, default to [QUERY_MAX_RESULT_LIMIT] if not specified.
        limit: Option<usize>,
        /// query result ordering, default to false (ascending order), oldest record first.
        descending_order: Option<bool>,
    ) -> RpcResult<EventPage>;

    /// Subscribe to a stream of Pera event
    #[subscription(name = "subscribeEvent", item = PeraEvent)]
    fn subscribe_event(
        &self,
        /// The filter criteria of the event stream. See [Event filter](https://docs.pera.io/build/event_api#event-filters) documentation for examples.
        filter: EventFilter,
    );

    /// Subscribe to a stream of Pera transaction effects
    #[subscription(name = "subscribeTransaction", item = PeraTransactionBlockEffects)]
    fn subscribe_transaction(&self, filter: TransactionFilter);

    /// Return the list of dynamic field objects owned by an object.
    #[method(name = "getDynamicFields")]
    async fn get_dynamic_fields(
        &self,
        /// The ID of the parent object
        parent_object_id: ObjectID,
        /// An optional paging cursor. If provided, the query will start from the next item after the specified cursor. Default to start from the first item if not specified.
        cursor: Option<ObjectID>,
        /// Maximum item returned per page, default to [QUERY_MAX_RESULT_LIMIT] if not specified.
        limit: Option<usize>,
    ) -> RpcResult<DynamicFieldPage>;

    /// Return the dynamic field object information for a specified object
    #[method(name = "getDynamicFieldObject")]
    async fn get_dynamic_field_object(
        &self,
        /// The ID of the queried parent object
        parent_object_id: ObjectID,
        /// The Name of the dynamic field
        name: DynamicFieldName,
    ) -> RpcResult<PeraObjectResponse>;

    /// Return the resolved address given resolver and name
    #[method(name = "resolveNameServiceAddress")]
    async fn resolve_name_service_address(
        &self,
        /// The name to resolve
        name: String,
    ) -> RpcResult<Option<PeraAddress>>;

    /// Return the resolved names given address,
    /// if multiple names are resolved, the first one is the primary name.
    #[method(name = "resolveNameServiceNames")]
    async fn resolve_name_service_names(
        &self,
        /// The address to resolve
        address: PeraAddress,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<Page<String, ObjectID>>;
}
