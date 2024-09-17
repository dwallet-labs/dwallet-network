// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use serde::{Deserialize, Serialize};
use std::env;

/// config as loaded from `config.yaml`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexerConfig {
    pub remote_store_url: String,
    pub eth_rpc_url: String,
    #[serde(default = "default_db_url")]
    pub db_url: String,
    pub eth_ws_url: String,
    pub checkpoints_path: String,
    pub concurrency: u64,
    pub bridge_genesis_checkpoint: u64,
    pub eth_pera_bridge_contract_address: String,
    pub start_block: u64,
    pub metric_url: String,
    pub metric_port: u16,
    pub pera_rpc_url: Option<String>,
    pub back_fill_lot_size: u64,
    pub resume_from_checkpoint: Option<u64>,
}

impl pera_config::Config for IndexerConfig {}

pub fn default_db_url() -> String {
    env::var("DB_URL").expect("db_url must be set in config or via the $DB_URL env var")
}
