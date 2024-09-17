// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use async_trait::async_trait;
use diesel::r2d2::R2D2Connection;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use pera_json_rpc::error::PeraRpcInputError;
use pera_types::error::PeraObjectResponseError;
use pera_types::object::ObjectRead;

use crate::errors::IndexerError;
use crate::indexer_reader::IndexerReader;
use pera_json_rpc::PeraRpcModule;
use pera_json_rpc_api::{ReadApiServer, QUERY_MAX_RESULT_LIMIT};
use pera_json_rpc_types::{
    Checkpoint, CheckpointId, CheckpointPage, ProtocolConfigResponse, PeraEvent,
    PeraGetPastObjectRequest, PeraObjectDataOptions, PeraObjectResponse, PeraPastObjectResponse,
    PeraTransactionBlockResponse, PeraTransactionBlockResponseOptions,
};
use pera_open_rpc::Module;
use pera_protocol_config::{ProtocolConfig, ProtocolVersion};
use pera_types::base_types::{ObjectID, SequenceNumber};
use pera_types::digests::{ChainIdentifier, TransactionDigest};
use pera_types::pera_serde::BigInt;

#[derive(Clone)]
pub(crate) struct ReadApi<T: R2D2Connection + 'static> {
    inner: IndexerReader<T>,
}

impl<T: R2D2Connection + 'static> ReadApi<T> {
    pub fn new(inner: IndexerReader<T>) -> Self {
        Self { inner }
    }

    async fn get_checkpoint(&self, id: CheckpointId) -> Result<Checkpoint, IndexerError> {
        match self
            .inner
            .spawn_blocking(move |this| this.get_checkpoint(id))
            .await
        {
            Ok(Some(epoch_info)) => Ok(epoch_info),
            Ok(None) => Err(IndexerError::InvalidArgumentError(format!(
                "Checkpoint {id:?} not found"
            ))),
            Err(e) => Err(e),
        }
    }

    async fn get_latest_checkpoint(&self) -> Result<Checkpoint, IndexerError> {
        self.inner
            .spawn_blocking(|this| this.get_latest_checkpoint())
            .await
    }

    async fn get_chain_identifier(&self) -> RpcResult<ChainIdentifier> {
        let genesis_checkpoint = self.get_checkpoint(CheckpointId::SequenceNumber(0)).await?;
        Ok(ChainIdentifier::from(genesis_checkpoint.digest))
    }
}

#[async_trait]
impl<T: R2D2Connection + 'static> ReadApiServer for ReadApi<T> {
    async fn get_object(
        &self,
        object_id: ObjectID,
        options: Option<PeraObjectDataOptions>,
    ) -> RpcResult<PeraObjectResponse> {
        let options = options.unwrap_or_default();
        let object_read = self
            .inner
            .get_object_read_in_blocking_task(object_id)
            .await?;

        match object_read {
            ObjectRead::NotExists(id) => Ok(PeraObjectResponse::new_with_error(
                PeraObjectResponseError::NotExists { object_id: id },
            )),
            ObjectRead::Exists(object_ref, o, layout) => {
                let mut display_fields = None;
                if options.show_display {
                    match self.inner.get_display_fields(&o, &layout).await {
                        Ok(rendered_fields) => display_fields = Some(rendered_fields),
                        Err(e) => {
                            return Ok(PeraObjectResponse::new(
                                Some((object_ref, o, layout, options, None).try_into()?),
                                Some(PeraObjectResponseError::DisplayError {
                                    error: e.to_string(),
                                }),
                            ));
                        }
                    }
                }
                Ok(PeraObjectResponse::new_with_data(
                    (object_ref, o, layout, options, display_fields).try_into()?,
                ))
            }
            ObjectRead::Deleted((object_id, version, digest)) => Ok(
                PeraObjectResponse::new_with_error(PeraObjectResponseError::Deleted {
                    object_id,
                    version,
                    digest,
                }),
            ),
        }
    }

    // For ease of implementation we just forward to the single object query, although in the
    // future we may want to improve the performance by having a more naitive multi_get
    // functionality
    async fn multi_get_objects(
        &self,
        object_ids: Vec<ObjectID>,
        options: Option<PeraObjectDataOptions>,
    ) -> RpcResult<Vec<PeraObjectResponse>> {
        if object_ids.len() > *QUERY_MAX_RESULT_LIMIT {
            return Err(
                PeraRpcInputError::SizeLimitExceeded(QUERY_MAX_RESULT_LIMIT.to_string()).into(),
            );
        }

        let mut futures = vec![];
        for object_id in object_ids {
            futures.push(self.get_object(object_id, options.clone()));
        }

        futures::future::join_all(futures)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
    }

    async fn get_total_transaction_blocks(&self) -> RpcResult<BigInt<u64>> {
        let checkpoint = self.get_latest_checkpoint().await?;
        Ok(BigInt::from(checkpoint.network_total_transactions))
    }

    async fn get_transaction_block(
        &self,
        digest: TransactionDigest,
        options: Option<PeraTransactionBlockResponseOptions>,
    ) -> RpcResult<PeraTransactionBlockResponse> {
        let mut txn = self
            .multi_get_transaction_blocks(vec![digest], options)
            .await?;

        let txn = txn.pop().ok_or_else(|| {
            IndexerError::InvalidArgumentError(format!("Transaction {digest} not found"))
        })?;

        Ok(txn)
    }

    async fn multi_get_transaction_blocks(
        &self,
        digests: Vec<TransactionDigest>,
        options: Option<PeraTransactionBlockResponseOptions>,
    ) -> RpcResult<Vec<PeraTransactionBlockResponse>> {
        let num_digests = digests.len();
        if num_digests > *QUERY_MAX_RESULT_LIMIT {
            Err(PeraRpcInputError::SizeLimitExceeded(
                QUERY_MAX_RESULT_LIMIT.to_string(),
            ))?
        }

        let options = options.unwrap_or_default();
        let txns = self
            .inner
            .multi_get_transaction_block_response_in_blocking_task(digests, options)
            .await?;

        Ok(txns)
    }

    async fn try_get_past_object(
        &self,
        _object_id: ObjectID,
        _version: SequenceNumber,
        _options: Option<PeraObjectDataOptions>,
    ) -> RpcResult<PeraPastObjectResponse> {
        Err(jsonrpsee::types::error::CallError::Custom(
            jsonrpsee::types::error::ErrorCode::MethodNotFound.into(),
        )
        .into())
    }

    async fn try_get_object_before_version(
        &self,
        _: ObjectID,
        _: SequenceNumber,
    ) -> RpcResult<PeraPastObjectResponse> {
        Err(jsonrpsee::types::error::CallError::Custom(
            jsonrpsee::types::error::ErrorCode::MethodNotFound.into(),
        )
        .into())
    }

    async fn try_multi_get_past_objects(
        &self,
        _past_objects: Vec<PeraGetPastObjectRequest>,
        _options: Option<PeraObjectDataOptions>,
    ) -> RpcResult<Vec<PeraPastObjectResponse>> {
        Err(jsonrpsee::types::error::CallError::Custom(
            jsonrpsee::types::error::ErrorCode::MethodNotFound.into(),
        )
        .into())
    }

    async fn get_latest_checkpoint_sequence_number(&self) -> RpcResult<BigInt<u64>> {
        let checkpoint = self.get_latest_checkpoint().await?;
        Ok(BigInt::from(checkpoint.sequence_number))
    }

    async fn get_checkpoint(&self, id: CheckpointId) -> RpcResult<Checkpoint> {
        self.get_checkpoint(id).await.map_err(Into::into)
    }

    async fn get_checkpoints(
        &self,
        cursor: Option<BigInt<u64>>,
        limit: Option<usize>,
        descending_order: bool,
    ) -> RpcResult<CheckpointPage> {
        let cursor = cursor.map(BigInt::into_inner);
        let limit = pera_json_rpc_api::validate_limit(
            limit,
            pera_json_rpc_api::QUERY_MAX_RESULT_LIMIT_CHECKPOINTS,
        )
        .map_err(PeraRpcInputError::from)?;

        let mut checkpoints = self
            .inner
            .spawn_blocking(move |this| this.get_checkpoints(cursor, limit + 1, descending_order))
            .await?;

        let has_next_page = checkpoints.len() > limit;
        checkpoints.truncate(limit);

        let next_cursor = checkpoints.last().map(|d| d.sequence_number.into());

        Ok(CheckpointPage {
            data: checkpoints,
            next_cursor,
            has_next_page,
        })
    }

    async fn get_checkpoints_deprecated_limit(
        &self,
        cursor: Option<BigInt<u64>>,
        limit: Option<BigInt<u64>>,
        descending_order: bool,
    ) -> RpcResult<CheckpointPage> {
        self.get_checkpoints(
            cursor,
            limit.map(|l| l.into_inner() as usize),
            descending_order,
        )
        .await
    }

    async fn get_events(&self, transaction_digest: TransactionDigest) -> RpcResult<Vec<PeraEvent>> {
        self.inner
            .get_transaction_events_in_blocking_task(transaction_digest)
            .await
            .map_err(Into::into)
    }

    async fn get_protocol_config(
        &self,
        version: Option<BigInt<u64>>,
    ) -> RpcResult<ProtocolConfigResponse> {
        let chain = self.get_chain_identifier().await?.chain();
        let version = if let Some(version) = version {
            (*version).into()
        } else {
            let latest_epoch = self
                .inner
                .spawn_blocking(|this| this.get_latest_epoch_info_from_db())
                .await?;
            (latest_epoch.protocol_version as u64).into()
        };

        ProtocolConfig::get_for_version_if_supported(version, chain)
            .ok_or(PeraRpcInputError::ProtocolVersionUnsupported(
                ProtocolVersion::MIN.as_u64(),
                ProtocolVersion::MAX.as_u64(),
            ))
            .map_err(Into::into)
            .map(ProtocolConfigResponse::from)
    }

    async fn get_chain_identifier(&self) -> RpcResult<String> {
        self.get_chain_identifier().await.map(|id| id.to_string())
    }
}

impl<T: R2D2Connection> PeraRpcModule for ReadApi<T> {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        pera_json_rpc_api::ReadApiOpenRpc::module_doc()
    }
}
