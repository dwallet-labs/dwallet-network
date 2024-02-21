// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::anyhow;
use async_trait::async_trait;
use futures::future::join_all;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::types::SubscriptionResult;
use jsonrpsee::{RpcModule, SubscriptionSink};

use move_core_types::identifier::Identifier;
use sui_json_rpc::SuiRpcModule;
use sui_json_rpc_api::{cap_page_limit, IndexerApiClient, IndexerApiServer};
use sui_json_rpc_types::{
    DynamicFieldPage, EventFilter, EventPage, ObjectsPage, Page, SuiObjectDataFilter,
    SuiObjectResponse, SuiObjectResponseQuery, SuiTransactionBlockResponseQuery,
    TransactionBlocksPage, TransactionFilter,
};
use sui_open_rpc::Module;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::digests::TransactionDigest;
use sui_types::dynamic_field::DynamicFieldName;
use sui_types::event::EventID;

use crate::errors::IndexerError;
use crate::store::IndexerStore;

pub(crate) struct IndexerApi<S> {
    state: S,
    fullnode: HttpClient,
    migrated_methods: Vec<String>,
}

impl<S: IndexerStore> IndexerApi<S> {
    pub fn new(state: S, fullnode_client: HttpClient, migrated_methods: Vec<String>) -> Self {
        Self {
            state,
            fullnode: fullnode_client,
            migrated_methods,
        }
    }

    async fn query_events_internal(
        &self,
        query: EventFilter,
        cursor: Option<EventID>,
        limit: Option<usize>,
        descending_order: Option<bool>,
    ) -> Result<EventPage, IndexerError> {
        self.state
            .get_events(query, cursor, limit, descending_order.unwrap_or_default())
            .await
    }

    async fn query_transaction_blocks_internal(
        &self,
        query: SuiTransactionBlockResponseQuery,
        cursor: Option<TransactionDigest>,
        limit: Option<usize>,
        descending_order: Option<bool>,
    ) -> Result<TransactionBlocksPage, IndexerError> {
        let limit = cap_page_limit(limit);
        let is_descending = descending_order.unwrap_or_default();
        let cursor_str = cursor.map(|digest| digest.to_string());
        let mut tx_vec_from_db = match query.filter {
            None => {
                let indexer_seq_number = self
                    .state
                    .get_transaction_sequence_by_digest(cursor_str, is_descending)
                    .await?;
                self.state
                    .get_all_transaction_page(indexer_seq_number, limit + 1, is_descending)
                    .await
            }
            Some(TransactionFilter::Checkpoint(checkpoint_id)) => {
                let indexer_seq_number = self
                    .state
                    .get_transaction_sequence_by_digest(cursor_str, is_descending)
                    .await?;
                self.state
                    .get_transaction_page_by_checkpoint(
                        checkpoint_id as i64,
                        indexer_seq_number,
                        limit + 1,
                        is_descending,
                    )
                    .await
            }
            Some(TransactionFilter::MoveFunction {
                package,
                module,
                function,
            }) => {
                let module = if let Some(m) = module {
                    Some(
                        Identifier::new(m)
                            .map_err(|e| IndexerError::InvalidArgumentError(e.to_string()))?,
                    )
                } else {
                    None
                };
                let function = if let Some(f) = function {
                    Some(
                        Identifier::new(f)
                            .map_err(|e| IndexerError::InvalidArgumentError(e.to_string()))?,
                    )
                } else {
                    None
                };
                let move_call_seq_number = self
                    .state
                    .get_move_call_sequence_by_digest(cursor_str, is_descending)
                    .await?;
                self.state
                    .get_transaction_page_by_move_call(
                        package,
                        module,
                        function,
                        move_call_seq_number,
                        limit + 1,
                        is_descending,
                    )
                    .await
            }
            Some(TransactionFilter::InputObject(input_obj_id)) => {
                let input_obj_seq = self
                    .state
                    .get_input_object_sequence_by_digest(cursor_str, is_descending)
                    .await?;
                self.state
                    .get_transaction_page_by_input_object(
                        input_obj_id,
                        /* version */ None,
                        input_obj_seq,
                        limit + 1,
                        is_descending,
                    )
                    .await
            }
            Some(TransactionFilter::ChangedObject(mutated_obj_id)) => {
                let indexer_seq_number = self
                    .state
                    .get_transaction_sequence_by_digest(cursor_str, is_descending)
                    .await?;
                self.state
                    .get_transaction_page_by_changed_object(
                        mutated_obj_id,
                        None,
                        indexer_seq_number,
                        limit + 1,
                        is_descending,
                    )
                    .await
            }
            // NOTE: more efficient to run this query over transactions table
            Some(TransactionFilter::FromAddress(sender_address)) => {
                let indexer_seq_number = self
                    .state
                    .get_transaction_sequence_by_digest(cursor_str, is_descending)
                    .await?;
                self.state
                    .get_transaction_page_by_sender_address(
                        sender_address.to_string(),
                        indexer_seq_number,
                        limit + 1,
                        is_descending,
                    )
                    .await
            }
            Some(TransactionFilter::ToAddress(recipient_address)) => {
                let recipient_seq_number = self
                    .state
                    .get_recipient_sequence_by_digest(cursor_str, is_descending)
                    .await?;
                self.state
                    .get_transaction_page_by_recipient_address(
                        /* from */ None,
                        recipient_address,
                        recipient_seq_number,
                        limit + 1,
                        is_descending,
                    )
                    .await
            }
            Some(TransactionFilter::FromAndToAddress { from, to }) => {
                let recipient_seq_number = self
                    .state
                    .get_recipient_sequence_by_digest(cursor_str, is_descending)
                    .await?;
                self.state
                    .get_transaction_page_by_recipient_address(
                        Some(from),
                        to,
                        recipient_seq_number,
                        limit + 1,
                        is_descending,
                    )
                    .await
            }
            Some(TransactionFilter::FromOrToAddress { addr }) => {
                let start_sequence = self
                    .state
                    .get_recipient_sequence_by_digest(cursor_str, is_descending)
                    .await?;
                self.state
                    .get_transaction_page_by_address(addr, start_sequence, limit + 1, is_descending)
                    .await
            }
            Some(TransactionFilter::TransactionKind(tx_kind_name)) => {
                let indexer_seq_number = self
                    .state
                    .get_transaction_sequence_by_digest(cursor_str, is_descending)
                    .await?;
                self.state
                    .get_transaction_page_by_transaction_kinds(
                        vec![tx_kind_name],
                        indexer_seq_number,
                        limit + 1,
                        is_descending,
                    )
                    .await
            }
            Some(TransactionFilter::TransactionKindIn(tx_kind_names)) => {
                let indexer_seq_number = self
                    .state
                    .get_transaction_sequence_by_digest(cursor_str, is_descending)
                    .await?;
                self.state
                    .get_transaction_page_by_transaction_kinds(
                        tx_kind_names,
                        indexer_seq_number,
                        limit + 1,
                        is_descending,
                    )
                    .await
            }
        }?;

        let has_next_page = tx_vec_from_db.len() > limit;
        tx_vec_from_db.truncate(limit);
        let next_cursor = tx_vec_from_db
            .last()
            .cloned()
            .map(|tx| {
                let digest = tx.transaction_digest;
                let tx_digest: Result<TransactionDigest, _> = digest.parse();
                tx_digest.map_err(|e| {
                    IndexerError::SerdeError(format!(
                        "Failed to deserialize transaction digest: {:?} with error {:?}",
                        digest, e
                    ))
                })
            })
            .transpose()?
            .map_or(cursor, Some);

        let tx_resp_futures = tx_vec_from_db.into_iter().map(|tx| {
            self.state
                .compose_sui_transaction_block_response(tx, query.options.as_ref())
        });
        let sui_tx_resp_vec = join_all(tx_resp_futures)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Page {
            data: sui_tx_resp_vec,
            next_cursor,
            has_next_page,
        })
    }

    async fn get_owned_objects_internal(
        &self,
        address: SuiAddress,
        query: Option<SuiObjectResponseQuery>,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<ObjectsPage> {
        let address = SuiObjectDataFilter::AddressOwner(address);
        // MUSTFIX(gegaowp): implement other filters beside address owner filter.
        let (filter, options) = match query {
            Some(SuiObjectResponseQuery {
                filter: Some(filter),
                options,
            }) => match filter {
                SuiObjectDataFilter::AddressOwner(_) => Ok((address, options)),
                _ => Err(anyhow!(
                    "Only address filter is supported on indexer for now."
                )),
            },
            Some(SuiObjectResponseQuery { filter: _, options }) => Ok((address, options)),
            None => Ok((address, None)),
        }?;
        let options = options.unwrap_or_default();
        let limit = cap_page_limit(limit);

        // NOTE: fetch one more object to check if there is next page
        let mut objects = self
            .state
            .query_latest_objects(filter, cursor, limit + 1)
            .await?;

        let has_next_page = objects.len() > limit;
        objects.truncate(limit);
        let next_cursor = objects
            .last()
            .map_or(cursor, |o_read| Some(o_read.object_id()));

        let data: Vec<SuiObjectResponse> = objects
            .into_iter()
            .map(|o| (o, options.clone()).try_into())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Page {
            data,
            next_cursor,
            has_next_page,
        })
    }
}

#[async_trait]
impl<S> IndexerApiServer for IndexerApi<S>
where
    S: IndexerStore + Sync + Send + 'static,
{
    async fn get_owned_objects(
        &self,
        address: SuiAddress,
        query: Option<SuiObjectResponseQuery>,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<ObjectsPage> {
        if !self
            .migrated_methods
            .contains(&"get_owned_objects".to_string())
        {
            let owned_obj_guard = self
                .state
                .indexer_metrics()
                .get_owned_objects_latency
                .start_timer();
            let owned_obj_resp = self
                .fullnode
                .get_owned_objects(address, query, cursor, limit)
                .await;
            owned_obj_guard.stop_and_record();
            return owned_obj_resp;
        }
        self.get_owned_objects_internal(address, query, cursor, limit)
            .await
    }
    async fn query_transaction_blocks(
        &self,
        query: SuiTransactionBlockResponseQuery,
        cursor: Option<TransactionDigest>,
        limit: Option<usize>,
        descending_order: Option<bool>,
    ) -> RpcResult<TransactionBlocksPage> {
        if !self
            .migrated_methods
            .contains(&"query_transaction_blocks".to_string())
        {
            let query_tx_guard = self
                .state
                .indexer_metrics()
                .query_transaction_blocks_latency
                .start_timer();
            let query_tx_resp = self
                .fullnode
                .query_transaction_blocks(query, cursor, limit, descending_order)
                .await;
            query_tx_guard.stop_and_record();
            return query_tx_resp;
        }
        Ok(self
            .query_transaction_blocks_internal(query, cursor, limit, descending_order)
            .await?)
    }

    async fn query_events(
        &self,
        query: EventFilter,
        // exclusive cursor if `Some`, otherwise start from the beginning
        cursor: Option<EventID>,
        limit: Option<usize>,
        descending_order: Option<bool>,
    ) -> RpcResult<EventPage> {
        if !self.migrated_methods.contains(&"query_events".to_string()) {
            let query_events_guard = self
                .state
                .indexer_metrics()
                .query_events_latency
                .start_timer();
            let query_events_resp = self
                .fullnode
                .query_events(query, cursor, limit, descending_order)
                .await;
            query_events_guard.stop_and_record();
            return query_events_resp;
        }
        Ok(self
            .query_events_internal(query, cursor, limit, descending_order)
            .await?)
    }

    async fn get_dynamic_fields(
        &self,
        parent_object_id: ObjectID,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<DynamicFieldPage> {
        let df_guard = self
            .state
            .indexer_metrics()
            .get_dynamic_fields_latency
            .start_timer();
        let df_resp = self
            .fullnode
            .get_dynamic_fields(parent_object_id, cursor, limit)
            .await;
        df_guard.stop_and_record();
        df_resp
    }

    async fn get_dynamic_field_object(
        &self,
        parent_object_id: ObjectID,
        name: DynamicFieldName,
    ) -> RpcResult<SuiObjectResponse> {
        let df_obj_guard = self
            .state
            .indexer_metrics()
            .get_dynamic_field_object_latency
            .start_timer();
        let df_obj_resp = self
            .fullnode
            .get_dynamic_field_object(parent_object_id, name)
            .await;
        df_obj_guard.stop_and_record();
        df_obj_resp
    }

    fn subscribe_event(&self, _sink: SubscriptionSink, _filter: EventFilter) -> SubscriptionResult {
        Ok(())
    }

    fn subscribe_transaction(
        &self,
        _sink: SubscriptionSink,
        _filter: TransactionFilter,
    ) -> SubscriptionResult {
        Ok(())
    }

    async fn resolve_name_service_address(&self, name: String) -> RpcResult<Option<SuiAddress>> {
        self.fullnode.resolve_name_service_address(name).await
    }

    async fn resolve_name_service_names(
        &self,
        address: SuiAddress,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<Page<String, ObjectID>> {
        self.fullnode
            .resolve_name_service_names(address, cursor, limit)
            .await
    }
}

impl<S> SuiRpcModule for IndexerApi<S>
where
    S: IndexerStore + Sync + Send + 'static,
{
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        sui_json_rpc_api::IndexerApiOpenRpc::module_doc()
    }
}
