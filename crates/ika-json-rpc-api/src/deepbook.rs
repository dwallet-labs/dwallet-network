// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use ika_open_rpc_macros::open_rpc;

#[open_rpc(namespace = "ikax", tag = "DeepBook Read API")]
#[rpc(server, client, namespace = "ikax")]
pub trait DeepBookApi {
    #[method(name = "ping")]
    async fn ping(&self) -> RpcResult<String>;
}
