// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;

use pera_json_rpc::PeraRpcModule;
use pera_json_rpc_api::{WriteApiClient, WriteApiServer};
use pera_json_rpc_types::{
    DevInspectArgs, DevInspectResults, DryRunTransactionBlockResponse,
    PeraTransactionBlockResponse, PeraTransactionBlockResponseOptions,
};
use pera_open_rpc::Module;
use pera_types::base_types::PeraAddress;
use pera_types::pera_serde::BigInt;
use pera_types::quorum_driver_types::ExecuteTransactionRequestType;

use crate::types::PeraTransactionBlockResponseWithOptions;

pub(crate) struct WriteApi {
    fullnode: HttpClient,
}

impl WriteApi {
    pub fn new(fullnode_client: HttpClient) -> Self {
        Self {
            fullnode: fullnode_client,
        }
    }
}

#[async_trait]
impl WriteApiServer for WriteApi {
    async fn execute_transaction_block(
        &self,
        tx_bytes: Base64,
        signatures: Vec<Base64>,
        options: Option<PeraTransactionBlockResponseOptions>,
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> RpcResult<PeraTransactionBlockResponse> {
        let pera_transaction_response = self
            .fullnode
            .execute_transaction_block(tx_bytes, signatures, options.clone(), request_type)
            .await?;
        Ok(PeraTransactionBlockResponseWithOptions {
            response: pera_transaction_response,
            options: options.unwrap_or_default(),
        }
        .into())
    }

    async fn dev_inspect_transaction_block(
        &self,
        sender_address: PeraAddress,
        tx_bytes: Base64,
        gas_price: Option<BigInt<u64>>,
        epoch: Option<BigInt<u64>>,
        additional_args: Option<DevInspectArgs>,
    ) -> RpcResult<DevInspectResults> {
        self.fullnode
            .dev_inspect_transaction_block(
                sender_address,
                tx_bytes,
                gas_price,
                epoch,
                additional_args,
            )
            .await
    }

    async fn dry_run_transaction_block(
        &self,
        tx_bytes: Base64,
    ) -> RpcResult<DryRunTransactionBlockResponse> {
        self.fullnode.dry_run_transaction_block(tx_bytes).await
    }
}

impl PeraRpcModule for WriteApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        pera_json_rpc_api::WriteApiOpenRpc::module_doc()
    }
}
