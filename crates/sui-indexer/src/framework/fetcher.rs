// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Result;
use sui_rest_api::{CheckpointData, Client};
use sui_types::messages_checkpoint::CheckpointSequenceNumber;
use tracing::{info, warn};

pub struct CheckpointFetcher {
    client: Client,
    last_downloaded_checkpoint: Option<CheckpointSequenceNumber>,
    highest_known_checkpoint: CheckpointSequenceNumber,
    sender: mysten_metrics::metered_channel::Sender<CheckpointData>,
}

impl CheckpointFetcher {
    const INTERVAL_PERIOD: std::time::Duration = std::time::Duration::from_secs(5);
    const CHECKPOINT_DOWNLOAD_CONCURRENCY: usize = 100;

    pub fn new(
        client: Client,
        last_downloaded_checkpoint: Option<CheckpointSequenceNumber>,
        sender: mysten_metrics::metered_channel::Sender<CheckpointData>,
    ) -> Self {
        Self {
            client,
            last_downloaded_checkpoint,
            highest_known_checkpoint: 0,
            sender,
        }
    }

    pub async fn run(mut self) {
        let mut interval = tokio::time::interval(Self::INTERVAL_PERIOD);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        info!("CheckpointFetcher started");

        loop {
            interval.tick().await;

            if let Err(e) = self.update_highest_known_checkpoint().await {
                warn!("error updating highest known checkpoint: {e}");
                continue;
            }

            if let Err(e) = self.download_checkpoints().await {
                warn!("error downloading checkpoints: {e}");
                continue;
            }
        }
    }

    async fn update_highest_known_checkpoint(&mut self) -> Result<()> {
        let checkpoint = self.client.get_latest_checkpoint().await?;
        self.highest_known_checkpoint =
            std::cmp::max(self.highest_known_checkpoint, *checkpoint.sequence_number());
        Ok(())
    }

    async fn download_checkpoints(&mut self) -> Result<()> {
        use futures::StreamExt;
        use tap::Pipe;

        let checkpoint_range = self
            .last_downloaded_checkpoint
            .map(|i| i.saturating_add(1))
            .unwrap_or(0)..=self.highest_known_checkpoint;

        if !checkpoint_range.is_empty() {
            info!("Starting download of checkpoints {checkpoint_range:?}");
        }

        let mut checkpoint_stream = checkpoint_range
            .map(|next| self.client.get_full_checkpoint(next))
            .pipe(futures::stream::iter)
            .buffered(Self::CHECKPOINT_DOWNLOAD_CONCURRENCY);

        while let Some(maybe_checkpoint) = checkpoint_stream.next().await {
            let checkpoint = maybe_checkpoint?;
            self.last_downloaded_checkpoint =
                Some(*checkpoint.checkpoint_summary.sequence_number());

            info!(
                checkpoint = checkpoint.checkpoint_summary.sequence_number(),
                "successfully downloaded checkpoint"
            );

            self.sender
                .send(checkpoint)
                .await
                .expect("channel shouldn't be closed");
        }

        Ok(())
    }
}
