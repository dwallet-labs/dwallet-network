// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use object_store::path::Path;
use object_store::ObjectStore;
use pera_data_ingestion_core::{create_remote_store_client, Worker};
use pera_storage::blob::{Blob, BlobEncoding};
use pera_types::full_checkpoint_content::CheckpointData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlobTaskConfig {
    pub url: String,
    pub remote_store_options: Vec<(String, String)>,
}

pub struct BlobWorker {
    remote_store: Box<dyn ObjectStore>,
}

impl BlobWorker {
    pub fn new(config: BlobTaskConfig) -> Self {
        Self {
            remote_store: create_remote_store_client(config.url, config.remote_store_options, 10)
                .expect("failed to create remote store client"),
        }
    }
}

#[async_trait]
impl Worker for BlobWorker {
    async fn process_checkpoint(&self, checkpoint: CheckpointData) -> Result<()> {
        let bytes = Blob::encode(&checkpoint, BlobEncoding::Bcs)?.to_bytes();
        let location = Path::from(format!(
            "{}.chk",
            checkpoint.checkpoint_summary.sequence_number
        ));
        self.remote_store
            .put(&location, Bytes::from(bytes).into())
            .await?;
        Ok(())
    }
}
