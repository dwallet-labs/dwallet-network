// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::BTreeMap;

use async_trait::async_trait;
use diesel::r2d2::R2D2Connection;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use move_binary_format::normalized::Module as NormalizedModule;

use pera_json_rpc::error::PeraRpcInputError;
use pera_json_rpc::PeraRpcModule;
use pera_json_rpc_api::MoveUtilsServer;
use pera_json_rpc_types::ObjectValueKind;
use pera_json_rpc_types::PeraMoveNormalizedType;
use pera_json_rpc_types::{
    MoveFunctionArgType, PeraMoveNormalizedFunction, PeraMoveNormalizedModule,
    PeraMoveNormalizedStruct,
};
use pera_open_rpc::Module;
use pera_types::base_types::ObjectID;

use crate::indexer_reader::IndexerReader;

pub struct MoveUtilsApi<T: R2D2Connection + 'static> {
    inner: IndexerReader<T>,
}

impl<T: R2D2Connection> MoveUtilsApi<T> {
    pub fn new(inner: IndexerReader<T>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl<T: R2D2Connection + 'static> MoveUtilsServer for MoveUtilsApi<T> {
    async fn get_normalized_move_modules_by_package(
        &self,
        package_id: ObjectID,
    ) -> RpcResult<BTreeMap<String, PeraMoveNormalizedModule>> {
        let resolver_modules = self.inner.get_package(package_id).await?.modules().clone();
        let pera_normalized_modules = resolver_modules
            .into_iter()
            .map(|(k, v)| (k, NormalizedModule::new(v.bytecode()).into()))
            .collect::<BTreeMap<String, PeraMoveNormalizedModule>>();
        Ok(pera_normalized_modules)
    }

    async fn get_normalized_move_module(
        &self,
        package: ObjectID,
        module_name: String,
    ) -> RpcResult<PeraMoveNormalizedModule> {
        let mut modules = self.get_normalized_move_modules_by_package(package).await?;
        let module = modules.remove(&module_name).ok_or_else(|| {
            PeraRpcInputError::GenericNotFound(format!(
                "No module was found with name {module_name}",
            ))
        })?;
        Ok(module)
    }

    async fn get_normalized_move_struct(
        &self,
        package: ObjectID,
        module_name: String,
        struct_name: String,
    ) -> RpcResult<PeraMoveNormalizedStruct> {
        let mut module = self
            .get_normalized_move_module(package, module_name)
            .await?;
        module
            .structs
            .remove(&struct_name)
            .ok_or_else(|| {
                PeraRpcInputError::GenericNotFound(format!(
                    "No struct was found with struct name {struct_name}"
                ))
            })
            .map_err(Into::into)
    }

    async fn get_normalized_move_function(
        &self,
        package: ObjectID,
        module_name: String,
        function_name: String,
    ) -> RpcResult<PeraMoveNormalizedFunction> {
        let mut module = self
            .get_normalized_move_module(package, module_name)
            .await?;
        module
            .exposed_functions
            .remove(&function_name)
            .ok_or_else(|| {
                PeraRpcInputError::GenericNotFound(format!(
                    "No function was found with function name {function_name}",
                ))
            })
            .map_err(Into::into)
    }

    async fn get_move_function_arg_types(
        &self,
        package: ObjectID,
        module: String,
        function: String,
    ) -> RpcResult<Vec<MoveFunctionArgType>> {
        let function = self
            .get_normalized_move_function(package, module, function)
            .await?;
        let args = function
            .parameters
            .iter()
            .map(|p| match p {
                PeraMoveNormalizedType::Struct { .. } => {
                    MoveFunctionArgType::Object(ObjectValueKind::ByValue)
                }
                PeraMoveNormalizedType::Vector(_) => {
                    MoveFunctionArgType::Object(ObjectValueKind::ByValue)
                }
                PeraMoveNormalizedType::Reference(_) => {
                    MoveFunctionArgType::Object(ObjectValueKind::ByImmutableReference)
                }
                PeraMoveNormalizedType::MutableReference(_) => {
                    MoveFunctionArgType::Object(ObjectValueKind::ByMutableReference)
                }
                _ => MoveFunctionArgType::Pure,
            })
            .collect::<Vec<MoveFunctionArgType>>();
        Ok(args)
    }
}

impl<T: R2D2Connection> PeraRpcModule for MoveUtilsApi<T> {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        pera_json_rpc_api::MoveUtilsOpenRpc::module_doc()
    }
}
